mod answer;
mod client;
mod hook;
mod last;
mod list;
mod mcp;
mod pause;
mod plugin;
mod release;
mod revive;
mod run;
mod send;
mod verify;
mod wait;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Context as _;
use clap::{Parser, Subcommand};
use viola_core::ViolaName;
use viola_core::obs::ObsProcess;

use crate::obs::DetailSink;

/// `viola --help`: the served verbs under their group labels, then the options (layout-templates
/// §Output structure — `viola --help`). A verb joins its row only once clap serves it.
const HELP_TEMPLATE: &str = "\
{usage-heading} {usage}

board:    list
traffic:  send, wait, last, answer
wheel:    pause, release
setup:    run, revive, verify, plugin install

Options:
{options}";

#[derive(Parser)]
#[command(
    name = "viola",
    version,
    about = "Drive one Claude Code session from another",
    override_usage = "viola [--home <dir>] <verb> [args]",
    help_template = HELP_TEMPLATE
)]
pub(crate) struct Cli {
    /// The viola home (default: <user home>/.viola)
    #[arg(long, global = true)]
    home: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Wrap a program as the named instance
    Run(run::RunArgs),
    /// Restart a dead instance in this terminal, resuming its newest logged session
    Revive(revive::ReviveArgs),
    /// Type a prompt into the named instance and read it back; the text comes from stdin or --file
    Send(send::SendArgs),
    /// Wait for the named instance's next turn end, dialog or session end at or after a cursor
    Wait(wait::WaitArgs),
    /// Print the named instance's newest turn message
    Last(last::LastArgs),
    /// Answer the named instance's pending dialog by its id; the response comes from stdin or --file
    Answer(answer::AnswerArgs),
    /// Show every wrapped session: its liveness, wheel, dialog and CLI, and the usage budget
    List(list::ListArgs),
    /// Take the named instance's wheel for the human without a keystroke
    Pause(pause::PauseArgs),
    /// Hand the named instance's wheel back to the driver: the human's verb, refused to a driver
    Release(release::ReleaseArgs),
    /// Measure the local claude CLI against the capability ledger and stamp its version
    ///
    /// Run it from a folder you trust in Claude Code: the input-box probe runs in a subfolder there,
    /// which inherits that trust. An unapproved external CLAUDE.md import blocks the probe: its
    /// dialog shows in the subfolder, and verify never answers it.
    Verify(verify::VerifyArgs),
    /// Write viola's MCP-only plugin for an unwrapped driver
    Plugin(plugin::PluginArgs),
    /// Hand a Claude Code hook's payload to the wrapper (run by the plugin, never by a person)
    #[command(hide = true)]
    Hook(hook::HookArgs),
    /// Serve the MCP tools to a Claude Code session (run by the plugin, never by a person)
    #[command(hide = true)]
    Mcp,
}

/// A dispatch error and, once the home and the instance resolved, where its chain may go.
pub(crate) struct Failure {
    pub(crate) error: anyhow::Error,
    pub(crate) sink: Option<DetailSink>,
}

pub(crate) fn dispatch(cli: Cli) -> Result<ExitCode, Failure> {
    match cli.command {
        // The hook's home is its session's, from `VIOLA_DIR`, never `--home`.
        Command::Hook(args) => hook::hook(&args),
        Command::Run(args) => {
            let home = resolved_home(cli.home)?;
            let sink = DetailSink {
                home: home.clone(),
                instance: args.name.clone(),
                process: ObsProcess::Run,
            };
            run::run(&home, args).map_err(|error| Failure {
                error,
                sink: Some(sink),
            })
        }
        // A revived instance is a wrapper: its chain goes where `run`'s does. `--list` starts
        // nothing and is a `cli` verb: its chain goes to the caller's own, when it has one.
        Command::Revive(args) => {
            let home = resolved_home(cli.home)?;
            let sink = if args.list {
                client::own_name().map(|instance| DetailSink {
                    home: home.clone(),
                    instance,
                    process: ObsProcess::Cli,
                })
            } else {
                Some(DetailSink {
                    home: home.clone(),
                    instance: args.name.clone(),
                    process: ObsProcess::Run,
                })
            };
            revive::revive(&home, args).map_err(|error| Failure { error, sink })
        }
        Command::Send(args) => {
            let home = resolved_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            send::send(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Wait(args) => {
            let home = resolved_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            wait::wait(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Last(args) => {
            let home = resolved_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            last::last(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Answer(args) => {
            let home = resolved_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            answer::answer(&home, &args).map_err(|error| Failure { error, sink })
        }
        // A board names no instance: its chain goes to the caller's own, when it has one.
        Command::List(args) => {
            let home = resolved_home(cli.home)?;
            let sink = client::own_name().map(|instance| DetailSink {
                home: home.clone(),
                instance,
                process: ObsProcess::Cli,
            });
            list::list(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Pause(args) => {
            let home = resolved_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            pause::pause(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Release(args) => {
            let home = resolved_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            release::release(&home, &args).map_err(|error| Failure { error, sink })
        }
        // The install names no instance: its chain goes to the caller's own, when it has one.
        Command::Plugin(args) => {
            let home = resolved_home(cli.home)?;
            let sink = client::own_name().map(|instance| DetailSink {
                home: home.clone(),
                instance,
                process: ObsProcess::Cli,
            });
            plugin::plugin(&home, &args).map_err(|error| Failure { error, sink })
        }
        // The server's chain goes to its own instance's detail file, when it runs in one.
        Command::Mcp => {
            let home = resolved_home(cli.home)?;
            let sink = client::own_name().map(|instance| DetailSink {
                home: home.clone(),
                instance,
                process: ObsProcess::Mcp,
            });
            mcp::mcp(&home).map_err(|error| Failure { error, sink })
        }
        Command::Verify(args) => {
            let home = resolved_home(cli.home)?;
            let instance = std::env::var_os("VIOLA_NAME")
                .and_then(|name| name.into_string().ok())
                .and_then(|name| ViolaName::try_new(name).ok());
            let sink = instance.clone().map(|instance| DetailSink {
                home: home.clone(),
                instance,
                process: ObsProcess::Cli,
            });
            verify::verify(&home, instance.as_ref(), &args).map_err(|error| Failure { error, sink })
        }
    }
}

/// A `cli` verb's detail sink: its chain goes there, and its one `error: internal error` line is
/// printed by the catch site alone (obs-plan §7 Per-role behaviour).
fn cli_sink(home: &Path, instance: &ViolaName) -> Option<DetailSink> {
    Some(DetailSink {
        home: home.to_path_buf(),
        instance: instance.clone(),
        process: ObsProcess::Cli,
    })
}

/// The default viola home of a user home: where a start with no `--home` lives.
pub(super) fn default_home(user_home: &Path) -> PathBuf {
    user_home.join(".viola")
}

/// The wrapped session this process belongs to.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Instance {
    pub(super) name: ViolaName,
    pub(super) dir: PathBuf,
    pub(super) home: PathBuf,
}

/// `VIOLA_NAME` and `VIOLA_DIR` as `viola run` sets them in its child: a valid name, and an
/// absolute dir that is `<home>/instances/<name>`. Anything else is not a wrapped session.
pub(super) fn instance_of(name: Option<OsString>, dir: Option<OsString>) -> Option<Instance> {
    let name = ViolaName::try_new(name?.into_string().ok()?).ok()?;
    let dir = PathBuf::from(dir?);
    if !dir.is_absolute() {
        return None;
    }
    let own: &str = name.as_ref();
    let instances = dir
        .parent()
        .filter(|_| dir.file_name() == Some(OsStr::new(own)))?;
    let home = instances
        .parent()
        .filter(|_| instances.file_name() == Some(OsStr::new("instances")))?
        .to_path_buf();
    Some(Instance { name, dir, home })
}

/// The home every verb but `hook` runs on, in the documented order (architecture §Cross-cutting
/// Patterns, Config management), one spelling of it (`canonical_home`).
fn resolved_home(flag: Option<PathBuf>) -> Result<PathBuf, Failure> {
    resolve_home(
        flag,
        std::env::var_os("VIOLA_NAME"),
        std::env::var_os("VIOLA_DIR"),
        std::env::home_dir(),
    )
    .map(|home| canonical_home(&home))
    .map_err(|error| Failure { error, sink: None })
}

/// `--home` as given; else the home of the wrapped session `VIOLA_NAME` and `VIOLA_DIR` name, when
/// both pass `instance_of`; else `<user home>/.viola`. A pair that fails the check is no home and
/// refuses nothing: the order falls through.
fn resolve_home(
    flag: Option<PathBuf>,
    name: Option<OsString>,
    dir: Option<OsString>,
    user_home: Option<PathBuf>,
) -> anyhow::Result<PathBuf> {
    if let Some(home) = flag {
        return Ok(home);
    }
    if let Some(instance) = instance_of(name, dir) {
        return Ok(instance.home);
    }
    user_home
        .map(|user_home| default_home(&user_home))
        .context("no user home directory")
}

/// `home` in the one spelling `run` and every client hash the endpoint over: canonicalised once it
/// exists, with a Windows verbatim drive path in its plain drive form; else, not there yet (or not
/// readable), absolute as given.
pub(super) fn canonical_home(home: &Path) -> PathBuf {
    std::fs::canonicalize(home)
        .map(plain_drive)
        .or_else(|_| std::path::absolute(home))
        .unwrap_or_else(|_| home.to_path_buf())
}

/// `\\?\C:\…` as `C:\…`; any other spelling, another verbatim form included, as given.
fn plain_drive(path: PathBuf) -> PathBuf {
    let plain = path.to_str().and_then(|text| {
        let rest = text.strip_prefix(r"\\?\")?;
        let drive = rest.as_bytes();
        (drive.len() >= 3
            && drive[0].is_ascii_alphabetic()
            && drive[1] == b':'
            && drive[2] == b'\\')
            .then(|| PathBuf::from(rest))
    });
    plain.unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn default_home_is_dot_viola_under_the_user_home() {
        let user_home = std::env::temp_dir().join("u");
        assert_eq!(default_home(&user_home), user_home.join(".viola"));
    }

    fn os(text: &str) -> Option<OsString> {
        Some(OsString::from(text))
    }

    /// The order as a table: `--home`, then a wrapped session's home, then the default.
    #[rstest]
    #[case::the_flag_beats_a_wrapped_session(
        Some("flag"),
        "builder",
        "w/instances/builder",
        "flag"
    )]
    #[case::the_flag_alone(Some("flag"), "", "", "flag")]
    #[case::a_wrapped_session_beats_the_default(None, "builder", "w/instances/builder", "w")]
    #[case::a_name_that_is_no_name(None, "Not-A-Name", "w/instances/Not-A-Name", "u/.viola")]
    #[case::a_dir_of_another_name(None, "builder", "w/instances/other", "u/.viola")]
    #[case::a_dir_outside_instances(None, "builder", "w/sessions/builder", "u/.viola")]
    #[case::a_relative_dir(None, "builder", "relative/instances/builder", "u/.viola")]
    #[case::no_session(None, "", "", "u/.viola")]
    fn resolve_home_takes_the_flag_then_the_session_then_the_default(
        #[case] flag: Option<&str>,
        #[case] name: &str,
        #[case] dir: &str,
        #[case] want: &str,
    ) {
        let root = std::env::temp_dir();
        let at = |rel: &str| {
            if rel.starts_with("relative") {
                PathBuf::from(rel)
            } else {
                root.join(rel)
            }
        };
        let got = resolve_home(
            flag.map(&at),
            (!name.is_empty()).then(|| OsString::from(name)),
            (!dir.is_empty()).then(|| at(dir).into_os_string()),
            Some(root.join("u")),
        )
        .expect("a home");
        assert_eq!(got, at(want));
    }

    #[test]
    fn resolve_home_needs_only_one_of_the_two_variables_missing_to_fall_through() {
        let root = std::env::temp_dir();
        let dir = root.join("w").join("instances").join("builder");
        let user = Some(root.join("u"));
        let default = root.join("u").join(".viola");
        let alone = resolve_home(None, None, Some(dir.into_os_string()), user.clone());
        assert_eq!(alone.expect("a home"), default);
        let named = resolve_home(None, os("builder"), None, user);
        assert_eq!(named.expect("a home"), default);
    }

    #[test]
    fn resolve_home_with_no_user_home_and_no_session_is_an_error() {
        assert!(resolve_home(None, None, None, None).is_err());
        let flag = std::env::temp_dir().join("flag");
        assert_eq!(
            resolve_home(Some(flag.clone()), None, None, None).expect("the flag"),
            flag
        );
    }

    #[test]
    fn canonical_home_resolves_an_existing_home_and_keeps_a_new_one_absolute() {
        let tmp = tempfile::tempdir().expect("tempdir");
        // Canonical, and on Windows in the plain drive form (`plain_drive`'s own literal table).
        let real = plain_drive(std::fs::canonicalize(tmp.path()).expect("canonical"));
        assert!(!real.to_string_lossy().starts_with(r"\\?\"), "{real:?}");
        let through = tmp.path().join("x").join("..");
        std::fs::create_dir(tmp.path().join("x")).expect("x");
        assert_eq!(canonical_home(&through), real);
        let new = tmp.path().join("not-yet");
        assert_eq!(
            canonical_home(&new),
            std::path::absolute(&new).expect("abs")
        );
        assert!(!new.exists(), "a new home is never created here");
    }

    #[cfg(unix)]
    #[test]
    fn canonical_home_resolves_a_link_to_its_target() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let target = tmp.path().join("target");
        std::fs::create_dir(&target).expect("target");
        let link = tmp.path().join("link");
        std::os::unix::fs::symlink(&target, &link).expect("link");
        assert_eq!(
            canonical_home(&link),
            std::fs::canonicalize(&target).expect("canonical")
        );
    }

    #[rstest]
    #[case::a_verbatim_drive(r"\\?\C:\Users\u\.viola", r"C:\Users\u\.viola")]
    #[case::a_verbatim_drive_root(r"\\?\d:\", r"d:\")]
    #[case::a_verbatim_unc(r"\\?\UNC\server\share\h", r"\\?\UNC\server\share\h")]
    #[case::a_verbatim_volume(r"\\?\Volume{1}\h", r"\\?\Volume{1}\h")]
    #[case::a_device_path(r"\\.\C:\h", r"\\.\C:\h")]
    #[case::a_plain_drive(r"C:\h", r"C:\h")]
    #[case::a_unix_path("/home/u/.viola", "/home/u/.viola")]
    #[case::too_short(r"\\?\C:", r"\\?\C:")]
    fn plain_drive_strips_only_the_verbatim_drive_prefix(#[case] given: &str, #[case] want: &str) {
        assert_eq!(plain_drive(PathBuf::from(given)), PathBuf::from(want));
    }
}
