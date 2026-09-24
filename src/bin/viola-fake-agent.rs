//! Test-only stand-in for the `claude` CLI (feature `fake-agent`, never in a release build).
//! This first slice answers `--version` and otherwise runs until Ctrl-C (`\x03`) or stdin EOF.

use std::io::Read as _;
use std::process::ExitCode;

const DEFAULT_CLI_VERSION: &str = "2.1.0";

fn cli_version(args: &[String]) -> &str {
    args.iter()
        .position(|a| a == "--cli-version")
        .and_then(|i| args.get(i + 1))
        .map_or(DEFAULT_CLI_VERSION, String::as_str)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version") {
        println!("{} (Claude Code)", cli_version(&args));
        return ExitCode::SUCCESS;
    }
    let mut stdin = std::io::stdin().lock();
    let mut byte = [0u8; 1];
    while let Ok(1) = stdin.read(&mut byte) {
        if byte[0] == 0x03 {
            break;
        }
    }
    ExitCode::SUCCESS
}
