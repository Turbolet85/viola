//! The fake `hyprctl`: when this executable's own file name is `hyprctl`, it answers each relied-on
//! request from the recorded set beside it (no env var and no option selects it).

use super::*;

/// The file name that turns the fake agent into the fake `hyprctl`.
pub(super) const HYPRCTL: &str = "hyprctl";
/// Beside the executable: one recorded fixture per request word.
const HYPRCTL_SET: &str = "hyprctl-set";
/// Beside the executable: `recorded` (the default, also for an absent file), `oversize`,
/// `malformed` or `absent` (the recorded no-instance reply, whatever the request).
const HYPRCTL_MODE: &str = "hyprctl-mode";
/// The variant of the recorded no-instance reply.
const ABSENT_VARIANT: &str = ".absent-1.json";

/// Whether this process was started under the file name `hyprctl`.
pub(super) fn is_hyprctl() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.file_name().map(|n| n == HYPRCTL))
        .unwrap_or(false)
}

/// What [`hyprctl_output`] gives for `args`: its bytes on stdout and its exit code.
pub(super) fn hyprctl(args: &[String]) -> ExitCode {
    let dir = exe_dir();
    let mode = dir
        .as_deref()
        .and_then(|d| fs::read_to_string(d.join(HYPRCTL_MODE)).ok());
    let (out, code) = hyprctl_output(mode.as_deref().map(str::trim), dir.as_deref(), args);
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(&out).and_then(|()| stdout.flush());
    ExitCode::from(code)
}

/// `oversize`: one byte past the frame cap; `malformed`: a text that is not JSON; `absent`: the
/// set's recorded no-instance reply; anything else: the recorded fixture whose `args` are `args`,
/// its `stdout` unchanged and its `exit`. An argv no fixture holds prints nothing and exits 1.
pub(super) fn hyprctl_output(
    mode: Option<&str>,
    dir: Option<&Path>,
    args: &[String],
) -> (Vec<u8>, u8) {
    match mode {
        Some("oversize") => Some((
            vec![b' '; usize::try_from(MAX_FRAME).unwrap_or(usize::MAX - 1) + 1],
            0,
        )),
        Some("malformed") => Some((b"viola fixture text\n".to_vec(), 0)),
        Some("absent") => recorded(dir, |name, _| name.ends_with(ABSENT_VARIANT)),
        _ => recorded(dir, |name, doc| {
            !name.ends_with(ABSENT_VARIANT) && doc["args"] == json!(args)
        }),
    }
    .unwrap_or((Vec::new(), 1))
}

fn recorded(dir: Option<&Path>, matches: impl Fn(&str, &Value) -> bool) -> Option<(Vec<u8>, u8)> {
    let set = dir?.join(HYPRCTL_SET);
    let mut names: Vec<PathBuf> = fs::read_dir(set)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    names.sort();
    names.into_iter().find_map(|path| {
        let name = path.file_name()?.to_str()?.to_owned();
        let doc: Value = serde_json::from_slice(&fs::read(&path).ok()?).ok()?;
        if !matches(&name, &doc) {
            return None;
        }
        let stdout = doc["stdout"].as_str()?.as_bytes().to_vec();
        let exit = u8::try_from(doc["exit"].as_i64()?).ok()?;
        Some((stdout, exit))
    })
}
