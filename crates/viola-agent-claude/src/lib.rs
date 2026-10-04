//! What `viola run` must know about the `claude` CLI: which inherited variables carry the parent
//! session's identity (the R8 strip) and how the npm shim resolves to the real executable.

pub mod hook;
pub mod ledger;
pub mod screen;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Parent-session identity measured on a wrapped host; always stripped, whatever the persistent
/// environment says (security-plan §Secret Management → Storage).
pub const IDENTITY_FLOOR: [&str; 11] = [
    "CLAUDECODE",
    "CLAUDE_CODE_BRIDGE_SESSION_ID",
    "CLAUDE_CODE_CHILD_SESSION",
    "CLAUDE_CODE_ENTRYPOINT",
    "CLAUDE_CODE_EXECPATH",
    "CLAUDE_CODE_MESSAGING_SOCKET",
    "CLAUDE_CODE_MESSAGING_TOKEN",
    "CLAUDE_CODE_SESSION_ATTENDED",
    "CLAUDE_CODE_SESSION_ID",
    "CLAUDE_EFFORT",
    "CLAUDE_PID",
];

/// Windows environment names and paths are case-insensitive.
pub const CASE_INSENSITIVE: bool = cfg!(windows);

/// The npm package's real binary, relative to the folder holding the `claude.cmd` shim.
const SHIM_TARGET: [&str; 5] = [
    "node_modules",
    "@anthropic-ai",
    "claude-code",
    "bin",
    "claude.exe",
];

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StripPlan {
    /// Names to remove from the child's environment, as inherited.
    pub remove: Vec<OsString>,
    /// `CLAUDE*` names left in place because the persistent environment defines them.
    pub kept: Vec<String>,
}

impl StripPlan {
    pub fn count(&self) -> usize {
        self.remove.len()
    }

    /// The removed names, sorted and comma-joined; names only, never values.
    pub fn removed_names(&self) -> String {
        joined(self.remove.iter().map(|n| n.to_string_lossy().into_owned()))
    }

    pub fn kept_names(&self) -> String {
        joined(self.kept.iter().cloned())
    }
}

fn joined(names: impl Iterator<Item = String>) -> String {
    let mut names: Vec<String> = names.collect();
    names.sort();
    names.join(",")
}

fn key(name: &str, case_insensitive: bool) -> String {
    if case_insensitive {
        name.to_ascii_uppercase()
    } else {
        name.to_owned()
    }
}

/// Every inherited `CLAUDE*` name is removed unless the persistent environment defines it and it
/// is not on [`IDENTITY_FLOOR`].
pub fn plan_strip(
    inherited: impl IntoIterator<Item = OsString>,
    persistent: &[String],
) -> StripPlan {
    plan_strip_with(inherited, persistent, CASE_INSENSITIVE)
}

fn plan_strip_with(
    inherited: impl IntoIterator<Item = OsString>,
    persistent: &[String],
    case_insensitive: bool,
) -> StripPlan {
    let persistent: Vec<String> = persistent
        .iter()
        .map(|p| key(p, case_insensitive))
        .collect();
    let mut plan = StripPlan::default();
    for name in inherited {
        let text = name.to_string_lossy().into_owned();
        let k = key(&text, case_insensitive);
        if !k.starts_with("CLAUDE") {
            continue;
        }
        let floor = IDENTITY_FLOOR.contains(&k.as_str());
        if !floor && persistent.contains(&k) {
            plan.kept.push(text);
        } else {
            plan.remove.push(name);
        }
    }
    plan
}

/// The flag that hands the per-start plugin folder to the `claude` child.
pub const PLUGIN_DIR_FLAG: &str = "--plugin-dir";

const VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_TOKEN: &str = "@@VIOLA_VERSION@@";
const BIN_TOKEN: &str = "@@VIOLA_BIN@@";

/// The embedded plugin, relative path → template. `mcpServers` stays empty until the `mcp` verb
/// exists: a registered command that exits 2 blocks Claude Code.
const PLUGIN_FILES: [(&str, &str); 3] = [
    (
        ".claude-plugin/plugin.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugin/.claude-plugin/plugin.json"
        )),
    ),
    (
        "hooks/hooks.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugin/hooks/hooks.json"
        )),
    ),
    (
        ".mcp.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugin/.mcp.json"
        )),
    ),
];

/// A template with the version and the pinned binary's forward-slash absolute path filled in.
fn render(template: &str, pinned_bin_fwd: &str) -> String {
    template
        .replace(VERSION_TOKEN, VERSION)
        .replace(BIN_TOKEN, pinned_bin_fwd)
}

/// The plugin folder's files, each as `(relative path, content)`, rewritten on every start.
pub fn plugin_files(pinned_bin_fwd: &str) -> [(&'static str, String); 3] {
    PLUGIN_FILES.map(|(path, template)| (path, render(template, pinned_bin_fwd)))
}

/// The crate's one error: fixed messages only, so no payload or stamp byte reaches a `Display`
/// (security-plan §Error Handling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum AgentError {
    #[error("the hook payload is not one JSON object")]
    Malformed,
    #[error("the capability stamps are malformed")]
    StampsMalformed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Refusal {
    #[error("the command is a .cmd or .bat script")]
    BatchScriptChild,
    #[error("the command was not found")]
    NotFound,
}

/// Where a bare program name is looked up.
#[derive(Debug, Clone, Copy)]
pub struct Search<'a> {
    pub path: Option<&'a OsStr>,
    pub pathext: Option<&'a OsStr>,
    pub windows: bool,
}

impl Search<'_> {
    pub const HOST_IS_WINDOWS: bool = cfg!(windows);
}

/// Resolves the program after `--` to the file to spawn. A path is used as given; a bare name is
/// searched on `PATH` (on Windows through `PATHEXT` only, so an extensionless sh shim never wins).
/// On Windows a `claude.cmd`/`claude.bat` becomes the `claude.exe` beside it, and every other
/// script is refused: portable-pty's command line does not escape arguments for `cmd.exe`.
pub fn resolve_program(
    program: &OsStr,
    cwd: &Path,
    search: Search<'_>,
    is_file: &dyn Fn(&Path) -> bool,
) -> Result<PathBuf, Refusal> {
    let given = Path::new(program);
    let found = if given.is_absolute() || given.components().count() > 1 {
        cwd.join(given)
    } else {
        find_on_path(program, search, is_file).ok_or(Refusal::NotFound)?
    };
    if !search.windows || !is_script(&found) {
        return Ok(found);
    }
    let stem_is_claude = found
        .file_stem()
        .is_some_and(|s| s.eq_ignore_ascii_case("claude"));
    let target = found
        .parent()
        .map(|dir| SHIM_TARGET.iter().fold(dir.to_path_buf(), |p, s| p.join(s)));
    match target {
        Some(target) if stem_is_claude && is_file(&target) => Ok(target),
        _ => Err(Refusal::BatchScriptChild),
    }
}

/// A `.cmd` or `.bat` file, which no child is ever spawned from.
pub fn is_script(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("cmd") || e.eq_ignore_ascii_case("bat"))
}

fn find_on_path(
    program: &OsStr,
    search: Search<'_>,
    is_file: &dyn Fn(&Path) -> bool,
) -> Option<PathBuf> {
    let dirs = std::env::split_paths(search.path?);
    let names: Vec<OsString> = if !search.windows || Path::new(program).extension().is_some() {
        vec![program.to_owned()]
    } else {
        search
            .pathext
            .unwrap_or(OsStr::new(".COM;.EXE;.BAT;.CMD"))
            .to_string_lossy()
            .split(';')
            .filter(|e| !e.is_empty())
            .map(|e| {
                let mut n = program.to_owned();
                n.push(e);
                n
            })
            .collect()
    };
    for dir in dirs {
        for name in &names {
            let candidate = dir.join(name);
            if is_file(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn names(list: &[&str]) -> Vec<OsString> {
        list.iter().map(OsString::from).collect()
    }

    #[test]
    fn plan_strip_removes_identity_and_unknown_claude_names_and_keeps_the_rest() {
        let plan = plan_strip_with(
            names(&[
                "PATH",
                "CLAUDECODE",
                "CLAUDE_CODE_SESSION_ID",
                "CLAUDE_VIOLA_TEST_NEW_X",
                "CLAUDE_CODE_DISABLE_MOUSE_CLICKS",
                "HOME",
            ]),
            &["CLAUDE_CODE_DISABLE_MOUSE_CLICKS".to_owned()],
            false,
        );
        assert_eq!(
            plan.remove,
            names(&[
                "CLAUDECODE",
                "CLAUDE_CODE_SESSION_ID",
                "CLAUDE_VIOLA_TEST_NEW_X"
            ])
        );
        assert_eq!(plan.kept, ["CLAUDE_CODE_DISABLE_MOUSE_CLICKS"]);
        assert_eq!(plan.count(), 3);
        assert_eq!(
            plan.removed_names(),
            "CLAUDECODE,CLAUDE_CODE_SESSION_ID,CLAUDE_VIOLA_TEST_NEW_X"
        );
        assert_eq!(plan.kept_names(), "CLAUDE_CODE_DISABLE_MOUSE_CLICKS");
    }

    #[test]
    fn plan_strip_strips_a_floor_name_even_when_it_is_persistent() {
        let plan = plan_strip_with(
            names(&["CLAUDE_CODE_MESSAGING_TOKEN", "CLAUDE_PID"]),
            &[
                "CLAUDE_CODE_MESSAGING_TOKEN".to_owned(),
                "CLAUDE_PID".to_owned(),
            ],
            false,
        );
        assert_eq!(
            plan.remove,
            names(&["CLAUDE_CODE_MESSAGING_TOKEN", "CLAUDE_PID"])
        );
        assert!(plan.kept.is_empty());
    }

    #[test]
    fn plan_strip_folds_case_only_when_the_host_does() {
        let insensitive = plan_strip_with(
            names(&["claude_code_session_id", "Claude_Config_Dir", "claude_new"]),
            &["CLAUDE_CONFIG_DIR".to_owned()],
            true,
        );
        assert_eq!(
            insensitive.remove,
            names(&["claude_code_session_id", "claude_new"])
        );
        assert_eq!(insensitive.kept, ["Claude_Config_Dir"]);
        let sensitive = plan_strip_with(
            names(&["claude_new", "CLAUDE_CONFIG_DIR"]),
            &["claude_config_dir".to_owned()],
            false,
        );
        assert_eq!(sensitive.remove, names(&["CLAUDE_CONFIG_DIR"]));
        assert!(sensitive.kept.is_empty());
    }

    #[test]
    fn plan_strip_is_empty_without_claude_names() {
        let plan = plan_strip(names(&["PATH", "XCLAUDE", "TEMP"]), &[]);
        assert_eq!(plan, StripPlan::default());
        assert_eq!(plan.removed_names(), "");
    }

    #[test]
    fn plan_strip_on_this_host_removes_identity_and_keeps_a_persistent_name() {
        let plan = plan_strip(
            names(&["CLAUDE_CODE_SESSION_ID", "CLAUDE_KEEP_ME", "PATH"]),
            &["CLAUDE_KEEP_ME".to_owned()],
        );
        assert_eq!(plan.remove, names(&["CLAUDE_CODE_SESSION_ID"]));
        assert_eq!(plan.kept, ["CLAUDE_KEEP_ME"]);
    }

    #[test]
    fn identity_floor_is_the_measured_list() {
        assert_eq!(IDENTITY_FLOOR.len(), 11);
        assert!(IDENTITY_FLOOR.iter().all(|n| n.starts_with("CLAUDE")));
        assert!(IDENTITY_FLOOR.contains(&"CLAUDE_CODE_MESSAGING_SOCKET"));
    }

    fn touch(path: &Path) {
        fs::create_dir_all(path.parent().expect("parent")).expect("dir");
        fs::write(path, b"x").expect("file");
    }

    fn windows<'a>(path: &'a OsStr) -> Search<'a> {
        Search {
            path: Some(path),
            pathext: Some(OsStr::new(".COM;.EXE;.BAT;.CMD")),
            windows: true,
        }
    }

    /// Windows lookup semantics on any host: a fixed file set, matched ignoring case.
    fn files(list: &[PathBuf]) -> impl Fn(&Path) -> bool + use<> {
        let list: Vec<String> = list
            .iter()
            .map(|p| p.to_string_lossy().to_ascii_lowercase())
            .collect();
        move |p: &Path| list.contains(&p.to_string_lossy().to_ascii_lowercase())
    }

    fn root() -> PathBuf {
        PathBuf::from("root")
    }

    fn shim_exe(dir: &Path) -> PathBuf {
        SHIM_TARGET.iter().fold(dir.to_path_buf(), |p, s| p.join(s))
    }

    #[test]
    fn resolve_program_turns_the_claude_shim_into_its_exe() {
        let npm = root().join("npm");
        let exe = shim_exe(&npm);
        let fs = files(&[npm.join("claude"), npm.join("claude.cmd"), exe.clone()]);
        let path = std::env::join_paths([&npm]).expect("path");
        let got = resolve_program(OsStr::new("claude"), &root(), windows(&path), &fs);
        assert_eq!(got, Ok(exe));
    }

    #[test]
    fn resolve_program_refuses_a_shim_without_its_exe_and_any_other_script() {
        let bin = root().join("bin");
        let fs = files(&[
            bin.join("claude.cmd"),
            bin.join("tool.bat"),
            bin.join("other.CMD"),
        ]);
        let path = std::env::join_paths([&bin]).expect("path");
        for program in ["claude", "tool", "other"] {
            let got = resolve_program(OsStr::new(program), &root(), windows(&path), &fs);
            assert_eq!(got, Err(Refusal::BatchScriptChild), "{program}");
        }
        let given = bin.join("tool.bat");
        let got = resolve_program(given.as_os_str(), &root(), windows(&path), &fs);
        assert_eq!(got, Err(Refusal::BatchScriptChild));
    }

    #[test]
    fn resolve_program_refuses_a_non_claude_script_even_beside_the_exe() {
        let npm = root().join("npm");
        let fs = files(&[npm.join("codex.cmd"), shim_exe(&npm)]);
        let path = std::env::join_paths([&npm]).expect("path");
        let got = resolve_program(OsStr::new("codex"), &root(), windows(&path), &fs);
        assert_eq!(got, Err(Refusal::BatchScriptChild));
    }

    #[test]
    fn resolve_program_never_takes_an_extensionless_file_on_windows() {
        let bin = root().join("bin");
        let fs = files(&[bin.join("tool")]);
        let path = std::env::join_paths([&bin]).expect("path");
        let got = resolve_program(OsStr::new("tool"), &root(), windows(&path), &fs);
        assert_eq!(got, Err(Refusal::NotFound));
    }

    #[test]
    fn resolve_program_follows_pathext_order_and_path_order() {
        let first = root().join("a");
        let second = root().join("b");
        let fs = files(&[
            first.join("tool.cmd"),
            first.join("tool.exe"),
            second.join("tool.com"),
        ]);
        let path = std::env::join_paths([&first, &second]).expect("path");
        let got = resolve_program(OsStr::new("tool"), &root(), windows(&path), &fs);
        assert_eq!(got, Ok(first.join("tool.EXE")));
        let default_ext = Search {
            pathext: None,
            ..windows(&path)
        };
        let got = resolve_program(OsStr::new("tool"), &root(), default_ext, &fs);
        assert_eq!(got, Ok(first.join("tool.EXE")));
        let later = files(&[second.join("tool.com")]);
        let got = resolve_program(OsStr::new("tool"), &root(), windows(&path), &later);
        assert_eq!(got, Ok(second.join("tool.COM")));
    }

    #[test]
    fn resolve_program_takes_a_name_with_an_extension_as_is_on_path() {
        let bin = root().join("bin");
        let fs = files(&[bin.join("tool.exe")]);
        let path = std::env::join_paths([&bin]).expect("path");
        let got = resolve_program(OsStr::new("tool.exe"), &root(), windows(&path), &fs);
        assert_eq!(got, Ok(bin.join("tool.exe")));
    }

    #[test]
    fn resolve_program_uses_a_given_path_as_given() {
        let none = files(&[]);
        let exe = std::env::temp_dir().join("x").join("agent.exe");
        let got = resolve_program(exe.as_os_str(), &root(), windows(OsStr::new("")), &none);
        assert_eq!(got, Ok(exe));
        let rel = Path::new("x").join("agent");
        let got = resolve_program(rel.as_os_str(), &root(), windows(OsStr::new("")), &none);
        assert_eq!(got, Ok(root().join("x").join("agent")));
    }

    #[test]
    fn resolve_program_on_unix_takes_the_bare_name_and_never_refuses() {
        let tmp = tempfile::tempdir().expect("tmp");
        let bin = tmp.path().join("bin");
        touch(&bin.join("claude"));
        touch(&bin.join("run.cmd"));
        let path = std::env::join_paths([&bin]).expect("path");
        let unix = Search {
            path: Some(&path),
            pathext: None,
            windows: false,
        };
        let exists = |p: &Path| p.is_file();
        let got = resolve_program(OsStr::new("claude"), tmp.path(), unix, &exists);
        assert_eq!(got, Ok(bin.join("claude")));
        let got = resolve_program(OsStr::new("run.cmd"), tmp.path(), unix, &exists);
        assert_eq!(got, Ok(bin.join("run.cmd")));
    }

    #[test]
    fn resolve_program_reports_a_missing_program() {
        let fs = files(&[]);
        let path = std::env::join_paths([root()]).expect("path");
        let got = resolve_program(OsStr::new("absent"), &root(), windows(&path), &fs);
        assert_eq!(got, Err(Refusal::NotFound));
        let none = Search {
            path: None,
            ..windows(&path)
        };
        let got = resolve_program(OsStr::new("absent"), &root(), none, &fs);
        assert_eq!(got, Err(Refusal::NotFound));
    }

    /// The rendered `hooks.json`, written out as the oracle: seven exec-form entries on the pinned
    /// path, the spine and SessionEnd sync, the activity tier async, no dialog tier.
    const HOOKS_JSON: &str = r#"{
  "hooks": {
    "SessionStart": [
      {"hooks": [{"type": "command", "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe", "args": ["hook", "session-start"], "timeout": 5}]}
    ],
    "UserPromptSubmit": [
      {"hooks": [{"type": "command", "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe", "args": ["hook", "user-prompt-submit"], "timeout": 5}]}
    ],
    "Stop": [
      {"hooks": [{"type": "command", "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe", "args": ["hook", "stop"], "timeout": 5}]}
    ],
    "SessionEnd": [
      {"hooks": [{"type": "command", "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe", "args": ["hook", "session-end"]}]}
    ],
    "Notification": [
      {"hooks": [{"type": "command", "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe", "args": ["hook", "notification"], "async": true}]}
    ],
    "PostToolUse": [
      {"hooks": [{"type": "command", "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe", "args": ["hook", "post-tool-use"], "async": true}]}
    ],
    "PostToolUseFailure": [
      {"hooks": [{"type": "command", "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe", "args": ["hook", "post-tool-use-failure"], "async": true}]}
    ]
  }
}
"#;

    #[test]
    fn plugin_files_are_the_three_layout_paths() {
        let files = plugin_files("C:/h/bin/0.1.0-0123456789abcdef/viola.exe");
        let paths: Vec<&str> = files.iter().map(|(p, _)| *p).collect();
        assert_eq!(
            paths,
            [
                ".claude-plugin/plugin.json",
                "hooks/hooks.json",
                ".mcp.json"
            ]
        );
        assert_eq!(
            files[0].1,
            "{\n  \"name\": \"viola\",\n  \"version\": \"0.1.0\",\n  \"description\": \
             \"viola bridge: hooks and MCP server for a wrapped Claude Code session\"\n}\n"
        );
        assert_eq!(files[1].1, HOOKS_JSON);
        assert!(!files[1].1.contains("\"command\": \"viola"));
        assert_eq!(files[2].1, "{\n  \"mcpServers\": {}\n}\n");
        assert_eq!(PLUGIN_DIR_FLAG, "--plugin-dir");
    }

    #[test]
    fn plugin_files_render_substitutes_the_pinned_path() {
        let template = r#"{"command":"@@VIOLA_BIN@@","args":["hook"],"v":"@@VIOLA_VERSION@@"}"#;
        assert_eq!(
            render(template, "C:/h/bin/0.1.0-0123456789abcdef/viola.exe"),
            r#"{"command":"C:/h/bin/0.1.0-0123456789abcdef/viola.exe","args":["hook"],"v":"0.1.0"}"#
        );
    }

    #[test]
    fn refusal_messages_are_fixed() {
        assert_eq!(
            Refusal::BatchScriptChild.to_string(),
            "the command is a .cmd or .bat script"
        );
        assert_eq!(Refusal::NotFound.to_string(), "the command was not found");
    }
}
