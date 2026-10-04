//! `viola answer <name> <dialog_id> [--file <PATH>] [--json]`: the driver's answer to the named
//! instance's pending dialog, a `response` JSON object from stdin or `--file`, carried to the
//! wrapper, which maps it to the dialog's decision body (architecture §Standard Contracts `answer`).
//! The response is read into its closed shape and its free text checked here first (client checks
//! are advisory; the wrapper re-runs them), the instance must be live by its snapshot pid + start
//! time and heartbeat (the sixth dated gap), then one `answer` request; the reply is one line or one
//! `--json` document, and a typed exit code.

use std::fs::File;
use std::io::{self, Read, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_agent_claude::dialog::Response;
use viola_channel::{ChannelError, Client};
use viola_core::{MAX_FRAME, RefusalReason, ViolaName};

use super::client::{self, Reply, live_endpoint, own_name};
use super::send::{exit_of, reason_of};
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct AnswerArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// The pending dialog's id, as `viola wait` printed it
    dialog_id: u64,
    /// Read the response JSON from this file instead of stdin
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

/// Exit 2 with one fixed line: the response could not be taken.
const UNUSABLE_RESPONSE: &str =
    "error: the response must be one JSON object of an answer's shape, at most 16 MiB of UTF-8\n";

/// At most `MAX_FRAME` bytes of UTF-8 JSON in one of the three answer shapes; `None` for anything
/// else.
fn read_response(reader: impl Read) -> Option<Response> {
    let mut bytes = Vec::new();
    reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
    if bytes.len() as u64 > MAX_FRAME {
        return None;
    }
    let text = String::from_utf8(bytes).ok()?;
    let value: Value = serde_json::from_str(&text).ok()?;
    Response::parse(&value).ok()
}

/// The verb's outputs: one `--json` document, or the result and refusal lines.
struct Out<'a> {
    name: &'a str,
    json: bool,
}

impl Out<'_> {
    fn answered(&self, dialog_id: u64, ok: &Value) {
        if self.json {
            return client::document(&json!({"v": 1, "ok": ok}));
        }
        let _ = human::write_answered(&mut io::stdout().lock(), self.name, dialog_id);
    }

    fn unable(&self, reason: &str, detail: Option<&str>) {
        if self.json {
            return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
        }
        let shown = match detail {
            Some(detail) => format!("{reason}  {detail}"),
            None => reason.to_owned(),
        };
        let hint = human::answer_hint(reason, detail);
        let _ = human::write_unable(&mut io::stderr().lock(), self.name, &shown, hint);
    }
}

pub(crate) fn answer(home: &Path, args: &AnswerArgs) -> anyhow::Result<ExitCode> {
    let response = match &args.file {
        Some(path) => File::open(path).ok().and_then(read_response),
        None => read_response(io::stdin().lock()),
    };
    let Some(response) = response else {
        let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
        return Ok(ExitCode::from(2));
    };
    let home = client::start(home, &args.name)?;
    let out = Out {
        name: args.name.as_ref(),
        json: args.json,
    };
    let code = deliver(&home, args.dialog_id, &response, &out)?;
    Ok(ExitCode::from(code))
}

fn deliver(home: &Path, dialog_id: u64, response: &Response, out: &Out<'_>) -> anyhow::Result<u8> {
    let refused = response
        .free_text()
        .into_iter()
        .find_map(|text| viola_core::validate_paste_text(text).err());
    if let Some(detail) = refused {
        out.unable(RefusalReason::NotDelivered.as_str(), Some(detail.as_str()));
        run::log_self_exit(13, None);
        return Ok(13);
    }
    let instance_dir = home.join("instances").join(out.name);
    let unreachable =
        |during| client::unreachable(out.json, during, || client::unable_unreachable(out.name));
    let Some(endpoint) = live_endpoint(&instance_dir) else {
        return Ok(unreachable("connect"));
    };
    let mut params = Map::new();
    params.insert("dialog_id".to_owned(), dialog_id.into());
    params.insert("response".to_owned(), response.to_value());
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    let reply = match client::answer_of(request(&endpoint, params))? {
        Ok(reply) => reply,
        Err(during) => return Ok(unreachable(during)),
    };
    let code = match reply {
        Reply::Ok(ok) => {
            out.answered(dialog_id, &ok);
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

#[instrument(skip_all, name = "answer.client")]
fn request(endpoint: &str, params: Map<String, Value>) -> Result<Value, ChannelError> {
    Client::connect(endpoint, "cli")?.request("answer", params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::question(br#"{"answers": {"Which color?": "red"}}"#.as_slice(), true)]
    #[case::permission(br#"{"behavior": "deny", "message": "no"}"#.as_slice(), true)]
    #[case::plan(br#"{"behavior": "approve"}"#.as_slice(), true)]
    #[case::not_json(b"approve".as_slice(), false)]
    #[case::not_utf8(b"\xff\xfe".as_slice(), false)]
    #[case::array(b"[1]".as_slice(), false)]
    #[case::wrong_shape(br#"{"behavior": "maybe"}"#.as_slice(), false)]
    fn read_response_takes_one_answer_shape(#[case] bytes: &[u8], #[case] taken: bool) {
        assert_eq!(read_response(bytes).is_some(), taken);
    }

    #[test]
    fn read_response_stops_at_max_frame() {
        let body = br#"{"behavior": "allow"}"#;
        let max = usize::try_from(MAX_FRAME).expect("fits");
        let padded = |len: usize| {
            let mut bytes = body.to_vec();
            bytes.resize(len, b' ');
            bytes
        };
        assert!(read_response(&padded(max)[..]).is_some());
        assert!(read_response(&padded(max + 1)[..]).is_none());
    }

    /// The client checks every free-text value before any frame: an ESC in an annotation is the
    /// `control-character` refusal.
    #[test]
    fn a_control_character_anywhere_in_the_response_is_found() {
        let response = read_response(
            &br#"{"answers": {"q": "a"}, "annotations": {"q": {"notes": "\u001b[2J"}}}"#[..],
        )
        .expect("a question");
        let refused = response
            .free_text()
            .into_iter()
            .find_map(|text| viola_core::validate_paste_text(text).err());
        assert_eq!(refused.map(|d| d.as_str()), Some("control-character"));
    }
}
