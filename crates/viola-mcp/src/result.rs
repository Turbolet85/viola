//! A channel reply as a tool result: the CLI `--json` documents less `v` (architecture §Conventions,
//! Error handling schema). An `ok` is `isError: false` with the payload as `structuredContent`; a
//! refusal, an unreachable instance, a wrapper fault and a refused input are `isError: true` with a
//! body of `refusal` / `detail` or `error` / `detail` only, and the text `refused: <code>`. Nothing
//! else of a reply, and no path or upstream text, reaches a result.

use rmcp::model::{CallToolResult, ContentBlock};
use serde_json::{Value, json};
use viola_channel::ChannelError;

use crate::McpError;

/// A wrapper's `ok` payload.
pub(crate) fn ok(payload: Value) -> CallToolResult {
    CallToolResult::structured(payload)
}

/// A refusal: `{"refusal": reason, "detail": detail}`.
pub(crate) fn refused(reason: &str, detail: Option<&str>) -> CallToolResult {
    failed(reason, json!({"refusal": reason, "detail": detail}))
}

/// An error document: `{"error": code, "detail": detail}`.
pub(crate) fn error(code: &str, detail: Value) -> CallToolResult {
    failed(code, json!({"error": code, "detail": detail}))
}

pub(crate) fn unreachable() -> CallToolResult {
    error("instance-unreachable", Value::Null)
}

/// A refused input: `detail` names the field, a closed word.
pub(crate) fn invalid(field: &'static str) -> CallToolResult {
    error("invalid-params", Value::from(field))
}

fn failed(code: &str, body: Value) -> CallToolResult {
    let mut result = CallToolResult::structured_error(body);
    result.content = vec![ContentBlock::text(format!("refused: {code}"))];
    result
}

/// One frame tool's outcome. `refusable` is false for `wait` and `last`, whose refusal is no known
/// reply: it is an internal fault, as on the CLI.
pub(crate) fn of_reply(
    sent: Result<Value, ChannelError>,
    refusable: bool,
) -> Result<CallToolResult, McpError> {
    let frame = match sent {
        Ok(frame) => frame,
        Err(ChannelError::Connect(_) | ChannelError::Closed | ChannelError::Io(_)) => {
            return Ok(unreachable());
        }
        Err(_) => return Err(McpError::Internal),
    };
    if let Some(fault) = frame.get("error") {
        return fault_detail(fault)
            .map(|detail| error("wrapper-fault", detail))
            .ok_or(McpError::Internal);
    }
    let result = frame.get("result").ok_or(McpError::Internal)?;
    if let Some(payload) = result.get("ok") {
        return Ok(ok(payload.clone()));
    }
    let reason = result
        .get("refusal")
        .and_then(Value::as_str)
        .filter(|_| refusable)
        .ok_or(McpError::Internal)?;
    let detail = match result.get("detail") {
        None | Some(Value::Null) => None,
        Some(Value::String(detail)) => Some(detail.as_str()),
        Some(_) => return Err(McpError::Internal),
    };
    Ok(refused(reason, detail))
}

/// A JSON-RPC `error` member as the wrapper-fault `detail` `{code, message, data}`, as the CLI reads
/// it; `None` without an integer `code`.
fn fault_detail(error: &Value) -> Option<Value> {
    let code = error["code"].as_i64()?;
    let message = error["message"].as_str().unwrap_or_default();
    let data = match &error["data"] {
        Value::Object(data) => Value::Object(data.clone()),
        _ => Value::Null,
    };
    Some(json!({"code": code, "message": message, "data": data}))
}

/// The board document's `ok` member, or its refusal as the same error document.
pub(crate) fn of_board(document: &Value) -> Result<CallToolResult, McpError> {
    if let Some(payload) = document.get("ok") {
        return Ok(ok(payload.clone()));
    }
    match document.get("error").and_then(Value::as_str) {
        Some("instance-unreachable") => Ok(unreachable()),
        _ => Err(McpError::Internal),
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use rstest::rstest;

    use super::*;

    fn text_of(result: &CallToolResult) -> String {
        result
            .content
            .iter()
            .filter_map(|block| block.as_text().map(|text| text.text.clone()))
            .collect()
    }

    /// Every reply shape a frame tool can read, and what it becomes.
    #[rstest]
    #[case::ok(
        json!({"id": 1, "result": {"ok": {"submitted_at": "t", "cursor": 4}}}),
        true,
        false,
        json!({"submitted_at": "t", "cursor": 4})
    )]
    #[case::refusal(
        json!({"id": 1, "result": {"refusal": "budget-paused", "detail": "five-hour"}}),
        true,
        true,
        json!({"refusal": "budget-paused", "detail": "five-hour"})
    )]
    #[case::refusal_null_detail(
        json!({"id": 1, "result": {"refusal": "human-typing", "detail": null}}),
        true,
        true,
        json!({"refusal": "human-typing", "detail": null})
    )]
    #[case::refusal_no_detail(
        json!({"id": 1, "result": {"refusal": "unverified-cli"}}),
        true,
        true,
        json!({"refusal": "unverified-cli", "detail": null})
    )]
    #[case::fault(
        json!({"id": 1, "error": {"code": -32603, "message": "internal error", "data": null}}),
        true,
        true,
        json!({"error": "wrapper-fault", "detail": {"code": -32603, "message": "internal error", "data": null}})
    )]
    #[case::fault_with_data(
        json!({"id": 1, "error": {"code": -32602, "message": "unsupported protocol version", "data": {"supported": 1, "wrapper": "0.1.0"}}}),
        false,
        true,
        json!({"error": "wrapper-fault", "detail": {"code": -32602, "message": "unsupported protocol version", "data": {"supported": 1, "wrapper": "0.1.0"}}})
    )]
    #[case::fault_without_message_or_object_data(
        json!({"id": 1, "error": {"code": -32603, "data": [1]}}),
        true,
        true,
        json!({"error": "wrapper-fault", "detail": {"code": -32603, "message": "", "data": null}})
    )]
    fn of_reply_maps_each_reply_shape(
        #[case] frame: Value,
        #[case] refusable: bool,
        #[case] is_error: bool,
        #[case] body: Value,
    ) {
        let result = of_reply(Ok(frame), refusable).expect("a result");
        assert_eq!(result.is_error, Some(is_error));
        assert_eq!(result.structured_content, Some(body.clone()));
        let code = body
            .get("refusal")
            .or_else(|| body.get("error"))
            .and_then(Value::as_str);
        match code {
            Some(code) => assert_eq!(text_of(&result), format!("refused: {code}")),
            None => assert_eq!(text_of(&result), body.to_string()),
        }
    }

    #[rstest]
    #[case::connect(ChannelError::Connect(io::Error::from(io::ErrorKind::NotFound)))]
    #[case::closed(ChannelError::Closed)]
    #[case::io(ChannelError::Io(io::Error::from(io::ErrorKind::ConnectionReset)))]
    fn of_reply_reads_a_vanished_instance_as_unreachable(#[case] error: ChannelError) {
        let result = of_reply(Err(error), true).expect("a result");
        assert_eq!(result.is_error, Some(true));
        assert_eq!(
            result.structured_content,
            Some(json!({"error": "instance-unreachable", "detail": null}))
        );
        assert_eq!(text_of(&result), "refused: instance-unreachable");
    }

    #[rstest]
    #[case::oversize(Err(ChannelError::Oversize), true)]
    #[case::deadline(Err(ChannelError::Deadline), true)]
    #[case::no_result(Ok(json!({"id": 1})), true)]
    #[case::fault_without_code(Ok(json!({"id": 1, "error": {}})), true)]
    #[case::refusal_not_string(Ok(json!({"id": 1, "result": {"refusal": 1}})), true)]
    #[case::detail_not_string(Ok(json!({"id": 1, "result": {"refusal": "x", "detail": 2}})), true)]
    #[case::a_wait_refusal(Ok(json!({"id": 1, "result": {"refusal": "unknown", "detail": null}})), false)]
    fn of_reply_reads_any_other_outcome_as_an_internal_fault(
        #[case] sent: Result<Value, ChannelError>,
        #[case] refusable: bool,
    ) {
        assert!(matches!(of_reply(sent, refusable), Err(McpError::Internal)));
        assert_eq!(McpError::Internal.to_string(), "internal error");
    }

    /// A refusal body holds only `refusal` and `detail`, an error body only `error` and `detail`.
    #[test]
    fn result_bodies_hold_only_their_two_keys() {
        let keys = |result: CallToolResult| -> Vec<String> {
            let body = result.structured_content.expect("a body");
            body.as_object()
                .expect("an object")
                .keys()
                .cloned()
                .collect()
        };
        assert_eq!(
            keys(refused("not-delivered", Some("empty-text"))),
            ["refusal", "detail"]
        );
        assert_eq!(keys(unreachable()), ["error", "detail"]);
        assert_eq!(keys(invalid("target")), ["error", "detail"]);
        assert_eq!(
            invalid("dialog_id").structured_content,
            Some(json!({"error": "invalid-params", "detail": "dialog_id"}))
        );
        assert_eq!(text_of(&invalid("text")), "refused: invalid-params");
    }

    #[test]
    fn of_board_takes_the_ok_member_or_the_unreachable_refusal() {
        let ok = of_board(&json!({"v": 1, "ok": {"items": [], "budget": "unknown"}})).expect("ok");
        assert_eq!(ok.is_error, Some(false));
        assert_eq!(
            ok.structured_content,
            Some(json!({"items": [], "budget": "unknown"}))
        );
        let refused = of_board(&json!({"v": 1, "error": "instance-unreachable", "detail": null}))
            .expect("refused");
        assert_eq!(
            refused.structured_content,
            Some(json!({"error": "instance-unreachable", "detail": null}))
        );
        assert!(of_board(&json!({"v": 1})).is_err());
        assert!(of_board(&json!({"v": 1, "error": "other"})).is_err());
    }
}
