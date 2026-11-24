//! The fake `hyprctl`: when this executable's own file name is `hyprctl`, it answers each relied-on
//! request from the recorded set beside it (no env var and no option selects it).

use super::*;
use std::ffi::OsStr;

use viola_hyprland::{Address, Request, WorkspaceId};

/// The file name that turns the fake agent into the fake `hyprctl`.
pub(super) const HYPRCTL: &str = "hyprctl";
/// Beside the executable: one recorded fixture per request word.
const HYPRCTL_SET: &str = "hyprctl-set";
/// Beside the executable: `recorded` (the default, also for an absent file), `oversize`,
/// `malformed`, `absent` (the recorded no-instance reply, whatever the request) or `terminal`
/// (`clients` names the stand-in `foot`'s pid, every other argv the recorded replay).
const HYPRCTL_MODE: &str = "hyprctl-mode";
/// The variant of the recorded no-instance reply.
const ABSENT_VARIANT: &str = ".absent-1.json";
/// The address `terminal` mode lists the stand-in's window at: the recorded move fixture's.
pub(super) const TERMINAL_ADDRESS: &str = "0x555aa16bde50";
/// The workspace `terminal` mode lists the stand-in's window on.
const TERMINAL_WORKSPACE: i32 = 2;
/// Beside the executable: each `dispatch` argv it was asked, one JSON array line each.
pub(super) const HYPRCTL_ASKED: &str = "hyprctl-asked";

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
    if let Some(dir) = dir.as_deref() {
        receipt(dir, args);
    }
    let (out, code) = hyprctl_output(mode.as_deref().map(str::trim), dir.as_deref(), args);
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(&out).and_then(|()| stdout.flush());
    ExitCode::from(code)
}

/// A `dispatch` argv appended to [`HYPRCTL_ASKED`] beside the executable, one line in one write;
/// any other argv leaves no line.
pub(super) fn receipt(dir: &Path, args: &[String]) {
    if args.first().map(String::as_str) != Some("dispatch") {
        return;
    }
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join(HYPRCTL_ASKED))
    {
        let _ = file.write_all(format!("{}\n", json!(args)).as_bytes());
    }
}

/// `oversize`: one byte past the frame cap; `malformed`: a text that is not JSON; `absent`: the
/// set's recorded no-instance reply; `terminal`: the stand-in's window in `clients`, and the two
/// focus dispatches on that window and its workspace answered as the recorded focus fixtures are;
/// anything else: the recorded fixture whose `args` are `args`, its `stdout` unchanged and its
/// `exit`. An argv no fixture holds prints nothing and exits 1.
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
        Some("terminal") if args == ["-j", "clients"] => Some((terminal_clients(dir), 0)),
        Some("terminal") if focus_of_terminal(args).is_some() => {
            let file = focus_of_terminal(args).unwrap_or_default();
            recorded(dir, |name, _| name == file)
        }
        _ => recorded(dir, |name, doc| {
            !name.ends_with(ABSENT_VARIANT) && doc["args"] == json!(args)
        }),
    }
    .unwrap_or((Vec::new(), 1))
}

/// The recorded fixture whose reply a focus dispatch on the stand-in's window or workspace takes.
fn focus_of_terminal(args: &[String]) -> Option<&'static str> {
    let window = Request::FocusWindow(Address::try_new(TERMINAL_ADDRESS.to_owned()).ok()?);
    let workspace = Request::Workspace(WorkspaceId::new(TERMINAL_WORKSPACE));
    [
        (window, "FocusWindow.recorded-1.json"),
        (workspace, "Workspace.recorded-1.json"),
    ]
    .into_iter()
    .find(|(request, _)| request.args().iter().eq(args.iter().map(OsStr::new)))
    .map(|(_, file)| file)
}

/// One synthetic client on the stand-in `foot`'s pid, the fields the seam's parse reads; `[]`
/// while no `foot-pid` stands beside the executable.
fn terminal_clients(dir: Option<&Path>) -> Vec<u8> {
    let pid = dir
        .and_then(|d| fs::read_to_string(d.join(super::foot::FOOT_PID)).ok())
        .and_then(|text| text.trim().parse::<u32>().ok());
    let clients = match pid {
        Some(pid) => json!([{"address": TERMINAL_ADDRESS, "pid": pid,
            "workspace": {"id": TERMINAL_WORKSPACE, "name": TERMINAL_WORKSPACE.to_string()}}]),
        None => json!([]),
    };
    format!("{clients}\n").into_bytes()
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
