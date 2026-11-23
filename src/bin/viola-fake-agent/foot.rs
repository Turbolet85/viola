//! The stand-in `foot`: when this executable's own file name is `foot`, it records its argv and
//! its pid beside itself, then runs the words after its three `--…=` options as its direct child in
//! the `--working-directory` folder, waits on it and exits with its code (no env var and no option
//! selects it).

use super::*;

/// The file name that turns the fake agent into the stand-in `foot`.
const FOOT: &str = "foot";
/// Beside the executable: the argv it was started with, as one JSON array.
const FOOT_ARGV: &str = "foot-argv";
/// Beside the executable: its own pid, read by the fake `hyprctl`'s `terminal` mode.
pub(super) const FOOT_PID: &str = "foot-pid";
const WORKING_DIRECTORY: &str = "--working-directory=";

/// Whether this process was started under the file name `foot`.
pub(super) fn is_foot() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.file_name().map(|n| n == FOOT))
        .unwrap_or(false)
}

/// The receipts, then the child; exit 1 with no child when the argv holds no program.
pub(super) fn foot(args: &[String]) -> ExitCode {
    if let Some(dir) = exe_dir() {
        let _ = fs::write(dir.join(FOOT_ARGV), json!(args).to_string());
        let staged = dir.join(format!("{FOOT_PID}.next"));
        if fs::write(&staged, std::process::id().to_string()).is_ok() {
            let _ = fs::rename(&staged, dir.join(FOOT_PID));
        }
    }
    let (working, words) = split_options(args);
    let Some((program, rest)) = words.split_first() else {
        return ExitCode::from(1);
    };
    let mut child = Command::new(program);
    child.args(rest);
    if let Some(working) = working {
        child.current_dir(working);
    }
    match child.status() {
        Ok(status) => ExitCode::from(
            status
                .code()
                .and_then(|c| u8::try_from(c).ok())
                .unwrap_or(1),
        ),
        Err(_) => ExitCode::from(1),
    }
}

/// The `--working-directory` value among the leading `--…=` options, and the words after them.
pub(super) fn split_options(args: &[String]) -> (Option<&str>, &[String]) {
    let options = args
        .iter()
        .take_while(|arg| arg.starts_with("--") && arg.contains('='))
        .count();
    let working = args[..options]
        .iter()
        .find_map(|arg| arg.strip_prefix(WORKING_DIRECTORY));
    (working, &args[options..])
}
