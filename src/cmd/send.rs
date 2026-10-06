//! `viola send <name> [--file <PATH>] [--json]`: the driver's text, from stdin or `--file`, typed
//! into the named instance as one bracketed paste and confirmed by the wrapper after the fact
//! (architecture [Delivery Confirmation]). The text is checked here first (client checks are
//! advisory; the wrapper re-runs them), the instance must be live by its snapshot pid + start time
//! and heartbeat, then one `send` request; the reply is the readback mirror or one `--json`
//! document, and a typed exit code.

use std::fs::File;
use std::io::{self, IsTerminal as _, Read, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use chrono::Utc;
use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_channel::{ChannelError, Client};
use viola_core::obs::ObsEvent;
use viola_core::{MAX_FRAME, NotDelivered, RefusalReason, ViolaName, obs_event};

use super::client::{self, Reply, clock_part, live_endpoint, own_name};
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct SendArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = parse_name)]
    pub(super) name: ViolaName,
    /// Read the text from this file instead of stdin
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
    /// Print one JSON document on stdout instead of the readback lines
    #[arg(long)]
    json: bool,
    /// Never set: the text is not an argument, so any second word is a usage error.
    #[allow(dead_code)]
    #[arg(hide = true, value_parser = refuse_text_argument)]
    text: Option<String>,
}

pub(crate) fn parse_name(raw: &str) -> Result<ViolaName, String> {
    warn_if_rewritten(raw);
    ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
}

fn refuse_text_argument(raw: &str) -> Result<String, String> {
    warn_if_rewritten(raw);
    Err("the text is read from stdin or --file, never from an argument".to_owned())
}

/// Git for Windows rewrites a leading-slash argument into a path under its install (`/clear` →
/// `C:/Program Files/Git/clear`): say so before clap's usage error (design-system Platform-Specific
/// Notes).
fn warn_if_rewritten(raw: &str) {
    if looks_rewritten(raw) {
        let _ = io::stderr()
            .lock()
            .write_all(b"warning: argument looks like a Git Bash rewritten path\n");
    }
}

fn looks_rewritten(raw: &str) -> bool {
    let bytes = raw.as_bytes();
    bytes.len() > 3
        && bytes[0].is_ascii_alphabetic()
        && &bytes[1..3] == b":/"
        && raw[3..]
            .to_ascii_lowercase()
            .starts_with("program files/git/")
}

/// Exit 2 with one fixed line: the text could not be taken.
const UNUSABLE_TEXT: &str = "error: the text must be readable UTF-8 of at most 16 MiB\n";

/// At most `MAX_FRAME` bytes of UTF-8; `None` for anything else.
fn read_text(reader: impl Read) -> Option<String> {
    let mut bytes = Vec::new();
    reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
    if bytes.len() as u64 > MAX_FRAME {
        return None;
    }
    String::from_utf8(bytes).ok()
}

/// The closed refusal → exit table (architecture §Conventions exit codes).
pub(super) fn exit_of(reason: RefusalReason) -> u8 {
    match reason {
        RefusalReason::HumanTyping => 10,
        RefusalReason::BudgetPaused => 11,
        RefusalReason::UnverifiedCli => 12,
        RefusalReason::NotDelivered => 13,
        RefusalReason::Unknown => 14,
    }
}

pub(super) fn reason_of(reason: &str) -> RefusalReason {
    serde_json::from_value(Value::from(reason)).unwrap_or(RefusalReason::Unknown)
}

/// The verb's outputs: one `--json` document, or the mirror lines.
struct Out<'a> {
    name: &'a str,
    json: bool,
}

impl Out<'_> {
    fn open(&self) {
        if !self.json && io::stdout().is_terminal() {
            let issued = Utc::now().format("%H:%M:%S%.3fZ").to_string();
            let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
        }
    }

    fn read_back(&self, ok: &Value) {
        if self.json {
            return client::document(&json!({"v": 1, "ok": ok}));
        }
        let submitted = ok["submitted_at"].as_str().unwrap_or_default();
        let cursor = ok["cursor"].as_u64().unwrap_or_default();
        let _ = human::write_read_back(
            &mut io::stdout().lock(),
            self.name,
            clock_part(submitted),
            cursor,
        );
    }

    /// An `ok` whose `confirmed` is `false`: the same document as any `ok`, or the open box that
    /// is never filled.
    fn unconfirmable(&self, ok: &Value) {
        if self.json {
            return client::document(&json!({"v": 1, "ok": ok}));
        }
        let _ = human::write_send_unconfirmable(&mut io::stdout().lock(), self.name);
    }

    fn unable(&self, reason: &str, detail: Option<&str>) {
        if self.json {
            return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
        }
        let shown = match detail {
            Some(detail) => format!("{reason}  {detail}"),
            None => reason.to_owned(),
        };
        let cause = match detail {
            Some(detail) if reason == RefusalReason::NotDelivered.as_str() => Some(detail),
            _ if reason == RefusalReason::HumanTyping.as_str() => Some(reason),
            _ => None,
        };
        let hint = cause.and_then(|cause| human::send_hint(self.name, cause));
        let _ =
            human::write_send_unable(&mut io::stderr().lock(), self.name, &shown, hint.as_deref());
    }

    /// Exit 21, its human form the bracketed mirror.
    fn unreachable(&self, during: &'static str) -> u8 {
        client::unreachable(self.json, during, || {
            let hint = human::send_hint(self.name, "not-running");
            let _ = human::write_send_unable(
                &mut io::stderr().lock(),
                self.name,
                "instance-unreachable",
                hint.as_deref(),
            );
        })
    }
}

pub(crate) fn send(home: &Path, args: &SendArgs) -> anyhow::Result<ExitCode> {
    let text = match &args.file {
        Some(path) => File::open(path).ok().and_then(read_text),
        None => read_text(io::stdin().lock()),
    };
    let Some(text) = text else {
        let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
        return Ok(ExitCode::from(2));
    };
    let home = client::start(home, &args.name)?;
    let out = Out {
        name: args.name.as_ref(),
        json: args.json,
    };
    let code = deliver(&home, &text, &out)?;
    Ok(ExitCode::from(code))
}

fn deliver(home: &Path, text: &str, out: &Out<'_>) -> anyhow::Result<u8> {
    if let Err(detail) = viola_core::validate_paste_text(text) {
        refused_client_side(detail);
        out.unable(RefusalReason::NotDelivered.as_str(), Some(detail.as_str()));
        run::log_self_exit(13, None);
        return Ok(13);
    }
    let instance_dir = home.join("instances").join(out.name);
    let Some(endpoint) = live_endpoint(&instance_dir) else {
        return Ok(out.unreachable("connect"));
    };
    let mut params = Map::new();
    params.insert("text".to_owned(), text.into());
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    out.open();
    let reply = match client::answer_of(request(&endpoint, params))? {
        Ok(reply) => reply,
        Err(during) => return Ok(out.unreachable(during)),
    };
    let code = match reply {
        Reply::Ok(ok) => {
            if ok["confirmed"] == false {
                out.unconfirmable(&ok);
            } else {
                out.read_back(&ok);
            }
            0
        }
        Reply::Refused { reason, detail } => {
            out.unable(&reason, detail.as_deref());
            exit_of(reason_of(&reason))
        }
        Reply::Fault(detail) => return Ok(client::fault(out.json, &detail)),
    };
    run::log_self_exit(code, None);
    Ok(code)
}

#[instrument(skip_all, name = "send.client")]
fn request(endpoint: &str, params: Map<String, Value>) -> Result<Value, ChannelError> {
    Client::connect(endpoint, "cli")?.request("send", params)
}

fn refused_client_side(detail: NotDelivered) {
    obs_event!(
        INFO,
        ObsEvent::SendRefused,
        side = "client",
        refusal = RefusalReason::NotDelivered.as_str(),
        detail = detail.as_str(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::clear("C:/Program Files/Git/clear", true)]
    #[case::lower_drive("d:/program files/git/skill", true)]
    #[case::slash_clear("/clear", false)]
    #[case::name("builder", false)]
    #[case::other_drive_path("C:/Users/me/clear", false)]
    #[case::too_short("C:/", false)]
    #[case::no_colon("CX/Program Files/Git/x", false)]
    #[case::digit_drive("1:/Program Files/Git/x", false)]
    fn looks_rewritten_matches_only_the_git_bash_shape(#[case] raw: &str, #[case] want: bool) {
        assert_eq!(looks_rewritten(raw), want);
    }

    #[test]
    fn read_text_takes_utf8_up_to_max_frame() {
        assert_eq!(
            read_text(&b"line one\nline two"[..]).as_deref(),
            Some("line one\nline two")
        );
        assert_eq!(read_text(&b"\xff\xfe"[..]), None);
        let max = usize::try_from(MAX_FRAME).expect("fits");
        assert_eq!(
            read_text(io::repeat(b'a').take(MAX_FRAME)).map(|t| t.len()),
            Some(max)
        );
        assert_eq!(read_text(io::repeat(b'a').take(MAX_FRAME + 1)), None);
    }

    #[rstest]
    #[case::human_typing("human-typing", 10)]
    #[case::budget_paused("budget-paused", 11)]
    #[case::unverified_cli("unverified-cli", 12)]
    #[case::not_delivered("not-delivered", 13)]
    #[case::unknown("unknown", 14)]
    #[case::unlisted("wheel-held", 14)]
    fn exit_of_is_the_closed_table(#[case] reason: &str, #[case] exit: u8) {
        assert_eq!(exit_of(reason_of(reason)), exit);
    }

    #[test]
    fn text_argument_is_always_refused() {
        assert!(refuse_text_argument("hello").is_err());
        assert!(parse_name("builder").is_ok());
        assert!(parse_name("/clear").is_err());
    }
}
