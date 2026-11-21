//! `MB`'s reader of the print-mode client's `--output-format stream-json` lines: the first
//! `assistant` line holding a `tool_use` of [`WAIT_TOOL`] opens the call, the `user` line holding
//! that id's `tool_result` closes it. Each line comes with the rig's own arrival instant; the reader
//! reads no clock and keeps no byte of any line, only the closed words below and one number.

use std::time::Instant;

use serde_json::Value;

/// The one tool the client is allowed: `viola mcp`'s `wait`, under the private config's server name.
pub const WAIT_TOOL: &str = "mcp__viola__wait";

/// The client's hard per-call limit's message fragment, read in the 2.1.330 bundle.
pub const TOOL_TIMEOUT_FRAGMENT: &str = " timed out after ";

/// The client's idle limit's message fragment, read in the 2.1.330 bundle.
pub const IDLE_TIMEOUT_FRAGMENT: &str = " sent no response or progress for ";

/// Which band a start's elapsed seconds are read against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// A `wait` with no `timeout_ms`, ended by viola's own 50 s cap: 49–60 s is `at-cap`.
    Long,
    /// A call the client cuts at the private config's 5 000 ms `timeout`: 4–20 s is `at-bound`.
    Cut,
}

/// The call as read: its tool-use id and arrival; its close once read.
#[derive(Debug, Default)]
pub struct CallReader {
    opened: Option<(String, Instant)>,
    closed: Option<Closed>,
    lines: usize,
    skipped: usize,
}

#[derive(Debug)]
struct Closed {
    at: Instant,
    is_error: Option<bool>,
    timed_out: Option<bool>,
    cut: &'static str,
}

/// The closed words a start's summary carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Words {
    pub call_error: &'static str,
    pub timed_out: &'static str,
    pub elapsed_s: String,
    pub elapsed_band: &'static str,
    pub cut_word: &'static str,
}

fn bool_word(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "true",
        Some(false) => "false",
        None => "unread",
    }
}

/// A `tool_result`'s content as text: a string, or the `text` of each text block joined.
fn content_text(content: &Value) -> Option<String> {
    match content {
        Value::String(text) => Some(text.clone()),
        Value::Array(blocks) => Some(
            blocks
                .iter()
                .filter_map(|b| b["text"].as_str())
                .collect::<Vec<_>>()
                .join(""),
        ),
        _ => None,
    }
}

fn cut_word(is_error: Option<bool>, text: Option<&str>) -> &'static str {
    if is_error != Some(true) {
        return "none";
    }
    match text {
        Some(t) if t.contains(TOOL_TIMEOUT_FRAGMENT) => "tool-timeout",
        Some(t) if t.contains(IDLE_TIMEOUT_FRAGMENT) => "idle-timeout",
        _ => "other",
    }
}

fn blocks<'a>(line: &'a Value, kind: &str) -> impl Iterator<Item = &'a Value> {
    let blocks = (line["type"] == kind)
        .then(|| line["message"]["content"].as_array())
        .flatten();
    blocks.into_iter().flatten()
}

impl CallReader {
    /// Reads one line that arrived at `at`; true when it closed the call.
    pub fn feed(&mut self, line: &[u8], at: Instant) -> bool {
        self.lines += 1;
        let Ok(line) = serde_json::from_slice::<Value>(line) else {
            self.skipped += 1;
            return false;
        };
        if !line["type"].is_string() {
            self.skipped += 1;
            return false;
        }
        if self.opened.is_none() {
            let id = blocks(&line, "assistant")
                .find(|b| b["type"] == "tool_use" && b["name"] == WAIT_TOOL)
                .and_then(|b| b["id"].as_str());
            if let Some(id) = id {
                self.opened = Some((id.to_owned(), at));
            }
            return false;
        }
        if self.closed.is_some() {
            return false;
        }
        let Some((id, _)) = &self.opened else {
            return false;
        };
        let Some(result) = blocks(&line, "user")
            .find(|b| b["type"] == "tool_result" && b["tool_use_id"].as_str() == Some(id))
        else {
            return false;
        };
        let is_error = match &result["is_error"] {
            Value::Bool(b) => Some(*b),
            Value::Null => Some(false),
            _ => None,
        };
        let text = content_text(&result["content"]);
        let timed_out = (is_error == Some(false))
            .then_some(text.as_deref())
            .flatten()
            .and_then(|t| serde_json::from_str::<Value>(t).ok())
            .and_then(|doc| doc["timed_out"].as_bool());
        self.closed = Some(Closed {
            at,
            is_error,
            timed_out,
            cut: cut_word(is_error, text.as_deref()),
        });
        true
    }

    /// Every line read.
    pub fn lines(&self) -> usize {
        self.lines
    }

    /// The lines of no known shape: not one JSON object with a string `type`.
    pub fn skipped(&self) -> usize {
        self.skipped
    }

    /// The call's words, its elapsed seconds read against `shape`'s band.
    pub fn words(&self, shape: Shape) -> Words {
        let Some(closed) = &self.closed else {
            return Words {
                call_error: "unread",
                timed_out: "unread",
                elapsed_s: "unread".to_owned(),
                elapsed_band: "unread",
                cut_word: "none",
            };
        };
        let elapsed = self
            .opened
            .as_ref()
            .map(|(_, at)| closed.at.saturating_duration_since(*at).as_secs());
        let band = match (elapsed, shape) {
            (None, _) => "unread",
            (Some(49..=60), Shape::Long) => "at-cap",
            (Some(4..=20), Shape::Cut) => "at-bound",
            (Some(_), _) => "outside",
        };
        Words {
            call_error: bool_word(closed.is_error),
            timed_out: bool_word(closed.timed_out),
            elapsed_s: elapsed.map_or_else(|| "unread".to_owned(), |s| s.to_string()),
            elapsed_band: band,
            cut_word: closed.cut,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;

    use super::*;

    fn call(id: &str) -> Vec<u8> {
        json!({"type": "assistant", "message": {"content": [
            {"type": "text", "text": "calling"},
            {"type": "tool_use", "id": id, "name": "mcp__viola__wait", "input": {"target": "rig-mb"}},
        ]}})
        .to_string()
        .into_bytes()
    }

    fn result(id: &str, is_error: Option<bool>, content: Value) -> Vec<u8> {
        let mut block = json!({"type": "tool_result", "tool_use_id": id, "content": content});
        if let Some(e) = is_error {
            block["is_error"] = Value::Bool(e);
        }
        json!({"type": "user", "message": {"content": [block]}})
            .to_string()
            .into_bytes()
    }

    fn read(lines: &[(Vec<u8>, u64)], shape: Shape) -> (Words, CallReader) {
        let t0 = Instant::now();
        let mut reader = CallReader::default();
        for (line, s) in lines {
            reader.feed(line, t0 + Duration::from_secs(*s));
        }
        (reader.words(shape), reader)
    }

    #[test]
    fn mcp_call_reader_reads_each_closed_word() {
        let timed_out = json!([{"type": "text", "text": "{\"timed_out\":true}"}]);
        let cut =
            json!("MCP server \"viola\" tool \"wait\" timed out after 5s. The outcome is unknown.");
        let idle = json!("MCP server \"viola\" tool \"wait\" sent no response or progress for 5s");
        let refused = json!([{"type": "text", "text": "refused: instance-unreachable"}]);
        let system = json!({"type": "system", "subtype": "init"})
            .to_string()
            .into_bytes();
        for (label, lines, shape, want) in [
            (
                "a whole call at the cap",
                vec![
                    (system.clone(), 0),
                    (call("t1"), 1),
                    (result("t1", Some(false), timed_out.clone()), 51),
                ],
                Shape::Long,
                ("false", "true", "50", "at-cap", "none"),
            ),
            (
                "no is_error read as false, a string content",
                vec![
                    (call("t1"), 0),
                    (result("t1", None, json!("{\"timed_out\":false}")), 3),
                ],
                Shape::Long,
                ("false", "false", "3", "outside", "none"),
            ),
            (
                "the client's hard limit",
                vec![(call("t1"), 2), (result("t1", Some(true), cut), 7)],
                Shape::Cut,
                ("true", "unread", "5", "at-bound", "tool-timeout"),
            ),
            (
                "the client's idle limit",
                vec![(call("t1"), 0), (result("t1", Some(true), idle), 31)],
                Shape::Cut,
                ("true", "unread", "31", "outside", "idle-timeout"),
            ),
            (
                "viola's own refusal",
                vec![(call("t1"), 0), (result("t1", Some(true), refused), 1)],
                Shape::Cut,
                ("true", "unread", "1", "outside", "other"),
            ),
            (
                "an is_error of another type",
                vec![(call("t1"), 0), (result("t1", None, json!(7)), 1)],
                Shape::Long,
                ("false", "unread", "1", "outside", "none"),
            ),
            (
                "no call",
                vec![(system.clone(), 0)],
                Shape::Long,
                ("unread", "unread", "unread", "unread", "none"),
            ),
            (
                "a result before its call",
                vec![
                    (result("t1", Some(false), timed_out.clone()), 0),
                    (call("t1"), 1),
                ],
                Shape::Long,
                ("unread", "unread", "unread", "unread", "none"),
            ),
            (
                "two calls, the first counts",
                vec![
                    (call("t1"), 0),
                    (call("t2"), 1),
                    (result("t2", Some(true), json!("x timed out after 5s")), 6),
                    (result("t1", Some(false), timed_out.clone()), 50),
                ],
                Shape::Long,
                ("false", "true", "50", "at-cap", "none"),
            ),
            (
                "a band's edges",
                vec![
                    (call("t1"), 0),
                    (result("t1", Some(false), timed_out.clone()), 61),
                ],
                Shape::Long,
                ("false", "true", "61", "outside", "none"),
            ),
            (
                "a cut band's low edge",
                vec![(call("t1"), 0), (result("t1", Some(true), json!("x")), 4)],
                Shape::Cut,
                ("true", "unread", "4", "at-bound", "other"),
            ),
        ] {
            let (words, _) = read(&lines, shape);
            let got = (
                words.call_error,
                words.timed_out,
                words.elapsed_s.as_str(),
                words.elapsed_band,
                words.cut_word,
            );
            assert_eq!(got, want, "{label}");
        }
    }

    #[test]
    fn mcp_call_reader_skips_and_counts_an_unknown_line_shape() {
        let lines = vec![
            (b"ok".to_vec(), 0),
            (b"{\"no\":\"type\"}".to_vec(), 0),
            (b"[1]".to_vec(), 0),
            (call("t1"), 1),
            (b"{\"type\":7}".to_vec(), 2),
            (result("t1", Some(false), json!("{\"timed_out\":true}")), 52),
        ];
        let (words, reader) = read(&lines, Shape::Long);
        assert_eq!(reader.lines(), 6);
        assert_eq!(reader.skipped(), 4);
        assert_eq!(words.elapsed_band, "at-cap");
    }

    #[test]
    fn mcp_call_reader_reports_the_closing_line_once() {
        let t0 = Instant::now();
        let mut reader = CallReader::default();
        assert!(!reader.feed(&call("t1"), t0));
        assert!(reader.feed(&result("t1", Some(true), json!("x")), t0));
        assert!(!reader.feed(&result("t1", Some(true), json!("x")), t0));
    }
}
