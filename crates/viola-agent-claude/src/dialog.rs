//! The dialog tier (architecture [Hook Contract]; §Standard Contracts `hook.dialog` / `answer`): a
//! PreToolUse or PermissionRequest payload classified into a dialog and its `data`, a driver's answer
//! read into a closed shape, and the decision body that answer maps to (S3 / S7 / S8). Pure: no I/O.
//! The field paths are the recorded payloads' (`fixtures/claude/<version>/`); upstream text is
//! content, copied, never interpreted.

use std::collections::BTreeMap;

use serde_json::{Map, Value, json};
use viola_core::EventKind;

use crate::AgentError;
use crate::hook::HookEvent;

/// The three dialog kinds an `answer` normalises to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogKind {
    Question,
    Permission,
    Plan,
}

impl DialogKind {
    pub const ALL: [Self; 3] = [Self::Question, Self::Permission, Self::Plan];

    pub const fn as_str(self) -> &'static str {
        self.event_kind().as_str()
    }

    pub const fn event_kind(self) -> EventKind {
        match self {
            Self::Question => EventKind::Question,
            Self::Permission => EventKind::Permission,
            Self::Plan => EventKind::Plan,
        }
    }

    pub fn from_name(kind: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.as_str() == kind)
    }
}

/// The two tools whose dialog the CLI raises through PreToolUse and then, with the same
/// `tool_input`, through PermissionRequest (measured on the 2.1.287 captures).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogTool {
    AskUserQuestion,
    ExitPlanMode,
}

impl DialogTool {
    pub const ALL: [Self; 2] = [Self::AskUserQuestion, Self::ExitPlanMode];

    pub const fn name(self) -> &'static str {
        match self {
            Self::AskUserQuestion => "AskUserQuestion",
            Self::ExitPlanMode => "ExitPlanMode",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|t| t.name() == name)
    }

    pub const fn kind(self) -> DialogKind {
        match self {
            Self::AskUserQuestion => DialogKind::Question,
            Self::ExitPlanMode => DialogKind::Plan,
        }
    }
}

/// A dialog-tier payload, classified: its kind and `data` (the dialog event's, without
/// `dialog_id`), the tool's input, and whether it is a PermissionRequest repeating a PreToolUse of a
/// [`DialogTool`] — a continuation of that dialog.
#[derive(Debug, Clone, PartialEq)]
pub struct Dialog {
    pub hook: HookEvent,
    pub kind: DialogKind,
    pub data: Value,
    pub tool: Option<DialogTool>,
    pub input: Value,
    pub continuation: bool,
}

/// `bytes` as `event`'s payload. `Ok(None)` is a PreToolUse for a tool outside the dialog matcher;
/// a payload that is not one object is `Malformed`, one missing a field its kind needs
/// `DialogMalformed`, and a spine event `NotAnEvent`.
pub fn classify(event: HookEvent, bytes: &[u8]) -> Result<Option<Dialog>, AgentError> {
    if !event.is_dialog() {
        return Err(AgentError::NotAnEvent);
    }
    let payload: Value = serde_json::from_slice(bytes).map_err(|_| AgentError::Malformed)?;
    let payload = payload.as_object().ok_or(AgentError::Malformed)?;
    let name = payload
        .get("tool_name")
        .and_then(Value::as_str)
        .ok_or(AgentError::DialogMalformed)?;
    let input = payload.get("tool_input").cloned().unwrap_or(Value::Null);
    let tool = DialogTool::from_name(name);
    let (kind, data) = match tool {
        Some(tool) => (tool.kind(), tool_data(tool, &input)?),
        None if event == HookEvent::PermissionRequest => (
            DialogKind::Permission,
            json!({"tool": name, "input": input}),
        ),
        None => return Ok(None),
    };
    Ok(Some(Dialog {
        hook: event,
        kind,
        data,
        tool,
        continuation: tool.is_some() && event == HookEvent::PermissionRequest,
        input,
    }))
}

/// A question's `{questions:[{question, header, options:[label], multi_select}]}`, a plan's
/// `{plan}`.
fn tool_data(tool: DialogTool, input: &Value) -> Result<Value, AgentError> {
    match tool {
        DialogTool::AskUserQuestion => {
            let questions = input["questions"]
                .as_array()
                .ok_or(AgentError::DialogMalformed)?;
            let questions: Vec<Value> = questions
                .iter()
                .map(question_data)
                .collect::<Result<_, _>>()?;
            Ok(json!({"questions": questions}))
        }
        DialogTool::ExitPlanMode => {
            let plan = input["plan"].as_str().ok_or(AgentError::DialogMalformed)?;
            Ok(json!({"plan": plan}))
        }
    }
}

fn question_data(question: &Value) -> Result<Value, AgentError> {
    let text = question["question"]
        .as_str()
        .ok_or(AgentError::DialogMalformed)?;
    let options: Vec<&str> = question["options"]
        .as_array()
        .ok_or(AgentError::DialogMalformed)?
        .iter()
        .map(|o| o["label"].as_str().ok_or(AgentError::DialogMalformed))
        .collect::<Result<_, _>>()?;
    Ok(json!({
        "question": text,
        "header": question["header"].as_str(),
        "options": options,
        "multi_select": question["multiSelect"].as_bool().unwrap_or(false),
    }))
}

/// A permission's `behavior`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permit {
    Allow,
    Deny,
}

/// A plan's `behavior`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Approve,
    Revise,
}

/// An `answer`'s `response`, normalised per dialog kind (architecture §Standard Contracts `answer`).
/// No permission suggestion is carried: a deliberate v1 limit.
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    Question {
        answers: BTreeMap<String, String>,
        annotations: Option<Map<String, Value>>,
    },
    Permission {
        behavior: Permit,
        message: Option<String>,
    },
    Plan {
        behavior: Verdict,
        message: Option<String>,
    },
}

impl Response {
    pub const fn kind(&self) -> DialogKind {
        match self {
            Self::Question { .. } => DialogKind::Question,
            Self::Permission { .. } => DialogKind::Permission,
            Self::Plan { .. } => DialogKind::Plan,
        }
    }

    /// `response` as one of the three shapes, the shape decided by its own fields: `answers` makes a
    /// question, `allow` / `deny` a permission, `approve` / `revise` a plan. Unknown fields are
    /// skipped; a field of the wrong type, or a shape that is none of the three, is
    /// `ResponseMalformed`.
    pub fn parse(response: &Value) -> Result<Self, AgentError> {
        let object = response.as_object().ok_or(AgentError::ResponseMalformed)?;
        if let Some(answers) = object.get("answers") {
            return question_response(answers, object.get("annotations"));
        }
        let message = match object.get("message") {
            None | Some(Value::Null) => None,
            Some(Value::String(m)) => Some(m.clone()),
            Some(_) => return Err(AgentError::ResponseMalformed),
        };
        match object.get("behavior").and_then(Value::as_str) {
            Some("allow") => Ok(Self::Permission {
                behavior: Permit::Allow,
                message,
            }),
            Some("deny") => Ok(Self::Permission {
                behavior: Permit::Deny,
                message,
            }),
            Some("approve") => Ok(Self::Plan {
                behavior: Verdict::Approve,
                message,
            }),
            Some("revise") => Ok(Self::Plan {
                behavior: Verdict::Revise,
                message,
            }),
            _ => Err(AgentError::ResponseMalformed),
        }
    }

    /// Every free-text value the response carries — each answer, every string inside
    /// `annotations`, the message — for `validate_paste_text`.
    pub fn free_text(&self) -> Vec<&str> {
        match self {
            Self::Question {
                answers,
                annotations,
            } => {
                let mut texts: Vec<&str> = answers.values().map(String::as_str).collect();
                if let Some(annotations) = annotations {
                    for (key, value) in annotations {
                        texts.push(key);
                        strings(value, &mut texts);
                    }
                }
                texts
            }
            Self::Permission { message, .. } | Self::Plan { message, .. } => {
                message.iter().map(String::as_str).collect()
            }
        }
    }

    /// The response as the channel carries it.
    pub fn to_value(&self) -> Value {
        match self {
            Self::Question {
                answers,
                annotations,
            } => {
                let mut value = json!({"answers": answers});
                if let Some(annotations) = annotations {
                    value["annotations"] = Value::Object(annotations.clone());
                }
                value
            }
            Self::Permission { behavior, message } => {
                let behavior = match behavior {
                    Permit::Allow => "allow",
                    Permit::Deny => "deny",
                };
                with_message(behavior, message.as_deref())
            }
            Self::Plan { behavior, message } => {
                let behavior = match behavior {
                    Verdict::Approve => "approve",
                    Verdict::Revise => "revise",
                };
                with_message(behavior, message.as_deref())
            }
        }
    }
}

fn with_message(behavior: &str, message: Option<&str>) -> Value {
    let mut value = json!({"behavior": behavior});
    if let Some(message) = message {
        value["message"] = json!(message);
    }
    value
}

fn question_response(answers: &Value, annotations: Option<&Value>) -> Result<Response, AgentError> {
    let answers = answers
        .as_object()
        .ok_or(AgentError::ResponseMalformed)?
        .iter()
        .map(|(q, a)| {
            a.as_str()
                .map(|a| (q.clone(), a.to_owned()))
                .ok_or(AgentError::ResponseMalformed)
        })
        .collect::<Result<_, _>>()?;
    let annotations = match annotations {
        None | Some(Value::Null) => None,
        Some(Value::Object(map)) => Some(map.clone()),
        Some(_) => return Err(AgentError::ResponseMalformed),
    };
    Ok(Response::Question {
        answers,
        annotations,
    })
}

fn strings<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::String(s) => out.push(s),
        Value::Array(items) => items.iter().for_each(|v| strings(v, out)),
        Value::Object(map) => map.iter().for_each(|(k, v)| {
            out.push(k);
            strings(v, out);
        }),
        _ => {}
    }
}

/// The decision body `response` maps to on `hook`, given the tool's `input`; `None` is no decision
/// (the hook prints nothing):
/// - question on PreToolUse → `allow` + `updatedInput` = the input plus `answers` and any
///   `annotations` (S3, S8);
/// - plan approve on PreToolUse → `allow` alone; a PermissionRequest `allow` is ignored for
///   `ExitPlanMode`, so an approve there is no decision (S7);
/// - plan revise on PermissionRequest → `deny` + `message`; on PreToolUse no decision (S7);
/// - permission on PermissionRequest → `allow` / `deny` + `message`;
/// - a question on PermissionRequest → no decision (its body is unmeasured).
pub fn decision_body(hook: HookEvent, input: &Value, response: &Response) -> Option<String> {
    let body = match (hook, response) {
        (
            HookEvent::PreToolUse,
            Response::Question {
                answers,
                annotations,
            },
        ) => {
            let mut updated = input.as_object().cloned().unwrap_or_default();
            updated.insert("answers".to_owned(), json!(answers));
            if let Some(annotations) = annotations {
                updated.insert("annotations".to_owned(), Value::Object(annotations.clone()));
            }
            pre_tool_use_allow(Some(Value::Object(updated)))
        }
        (
            HookEvent::PreToolUse,
            Response::Plan {
                behavior: Verdict::Approve,
                ..
            },
        ) => pre_tool_use_allow(None),
        (
            HookEvent::PermissionRequest,
            Response::Plan {
                behavior: Verdict::Revise,
                message,
            },
        ) => permission_decision("deny", message.as_deref()),
        (HookEvent::PermissionRequest, Response::Permission { behavior, message }) => {
            let behavior = match behavior {
                Permit::Allow => "allow",
                Permit::Deny => "deny",
            };
            permission_decision(behavior, message.as_deref())
        }
        _ => return None,
    };
    Some(body.to_string())
}

fn pre_tool_use_allow(updated: Option<Value>) -> Value {
    let mut output = json!({"hookEventName": "PreToolUse", "permissionDecision": "allow"});
    if let Some(updated) = updated {
        output["updatedInput"] = updated;
    }
    json!({"hookSpecificOutput": output})
}

fn permission_decision(behavior: &str, message: Option<&str>) -> Value {
    let mut decision = json!({"behavior": behavior});
    if let Some(message) = message {
        decision["message"] = json!(message);
    }
    json!({"hookSpecificOutput": {"hookEventName": "PermissionRequest", "decision": decision}})
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const CANARY: &str = "canary-chain-value-5c1e";

    fn fixture(name: &str) -> Vec<u8> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/claude/2.1.287")
            .join(name);
        std::fs::read(path).expect("relayed fixture")
    }

    fn classified(event: HookEvent, name: &str) -> Dialog {
        classify(event, &fixture(name))
            .expect("classified")
            .expect("a dialog")
    }

    #[test]
    fn classify_a_question_from_pre_tool_use() {
        let dialog = classified(HookEvent::PreToolUse, "PreToolUse.ask-user-question.json");
        assert_eq!(dialog.kind, DialogKind::Question);
        assert_eq!(dialog.tool, Some(DialogTool::AskUserQuestion));
        assert!(!dialog.continuation);
        let questions = dialog.data["questions"].as_array().expect("questions");
        let input = dialog.input["questions"].as_array().expect("input");
        assert_eq!(questions.len(), input.len());
        for (q, i) in questions.iter().zip(input) {
            assert_eq!(q["question"], i["question"]);
            assert_eq!(q["header"], i["header"]);
            assert_eq!(q["multi_select"], i["multiSelect"]);
            let labels: Vec<&Value> = i["options"]
                .as_array()
                .expect("options")
                .iter()
                .map(|o| &o["label"])
                .collect();
            let got: Vec<&Value> = q["options"].as_array().expect("labels").iter().collect();
            assert_eq!(got, labels);
        }
        assert_eq!(dialog.data.as_object().map(Map::len), Some(1));
    }

    #[test]
    fn classify_a_plan_from_pre_tool_use() {
        let dialog = classified(HookEvent::PreToolUse, "PreToolUse.exit-plan-mode.json");
        assert_eq!(dialog.kind, DialogKind::Plan);
        assert_eq!(dialog.tool, Some(DialogTool::ExitPlanMode));
        assert!(!dialog.continuation);
        assert_eq!(dialog.data, json!({"plan": dialog.input["plan"]}));
    }

    /// The captured PermissionRequest repeats its PreToolUse: a continuation of that tool, with the
    /// same `tool_input`.
    #[rstest]
    #[case::question("ask-user-question", DialogKind::Question)]
    #[case::plan("exit-plan-mode", DialogKind::Plan)]
    fn classify_a_permission_request_for_a_dialog_tool_is_a_continuation(
        #[case] tool: &str,
        #[case] kind: DialogKind,
    ) {
        let pre = classified(HookEvent::PreToolUse, &format!("PreToolUse.{tool}.json"));
        let pr = classified(
            HookEvent::PermissionRequest,
            &format!("PermissionRequest.{tool}.json"),
        );
        assert!(pr.continuation);
        assert_eq!(pr.kind, kind);
        assert_eq!(pr.tool, pre.tool);
        assert_eq!(pr.input, pre.input);
        assert_eq!(pr.data, pre.data);
    }

    #[test]
    fn classify_an_ordinary_permission_request_is_a_permission() {
        let payload = json!({"hook_event_name": "PermissionRequest", "tool_name": "Bash",
            "tool_input": {"command": CANARY}});
        let dialog = classify(HookEvent::PermissionRequest, payload.to_string().as_bytes())
            .expect("classified")
            .expect("a dialog");
        assert_eq!(dialog.kind, DialogKind::Permission);
        assert_eq!(dialog.tool, None);
        assert!(!dialog.continuation);
        assert_eq!(
            dialog.data,
            json!({"tool": "Bash", "input": {"command": CANARY}})
        );
    }

    #[test]
    fn classify_a_pre_tool_use_outside_the_matcher_is_no_dialog() {
        let payload = json!({"tool_name": "Bash", "tool_input": {"command": "ls"}});
        assert_eq!(
            classify(HookEvent::PreToolUse, payload.to_string().as_bytes()),
            Ok(None)
        );
    }

    #[rstest]
    #[case::not_json(HookEvent::PreToolUse, b"not json".as_slice(), AgentError::Malformed)]
    #[case::array(HookEvent::PermissionRequest, b"[1]".as_slice(), AgentError::Malformed)]
    #[case::no_tool(HookEvent::PreToolUse, br#"{"tool_input": {}}"#.as_slice(), AgentError::DialogMalformed)]
    #[case::tool_not_string(HookEvent::PermissionRequest, br#"{"tool_name": 1}"#.as_slice(), AgentError::DialogMalformed)]
    #[case::no_questions(HookEvent::PreToolUse, br#"{"tool_name": "AskUserQuestion", "tool_input": {}}"#.as_slice(), AgentError::DialogMalformed)]
    #[case::question_not_string(HookEvent::PreToolUse, br#"{"tool_name": "AskUserQuestion", "tool_input": {"questions": [{"question": 1, "options": []}]}}"#.as_slice(), AgentError::DialogMalformed)]
    #[case::no_options(HookEvent::PreToolUse, br#"{"tool_name": "AskUserQuestion", "tool_input": {"questions": [{"question": "q"}]}}"#.as_slice(), AgentError::DialogMalformed)]
    #[case::label_not_string(HookEvent::PreToolUse, br#"{"tool_name": "AskUserQuestion", "tool_input": {"questions": [{"question": "q", "options": [{"label": 2}]}]}}"#.as_slice(), AgentError::DialogMalformed)]
    #[case::no_plan(HookEvent::PermissionRequest, br#"{"tool_name": "ExitPlanMode", "tool_input": {"planFilePath": "p"}}"#.as_slice(), AgentError::DialogMalformed)]
    #[case::spine_event(HookEvent::Stop, br#"{"tool_name": "Bash"}"#.as_slice(), AgentError::NotAnEvent)]
    fn classify_refuses_a_payload_it_cannot_read(
        #[case] event: HookEvent,
        #[case] bytes: &[u8],
        #[case] error: AgentError,
    ) {
        assert_eq!(classify(event, bytes), Err(error));
    }

    #[test]
    fn classify_a_question_without_header_or_multi_select_reads_null_and_false() {
        let payload = br#"{"tool_name": "AskUserQuestion", "tool_input": {"questions": [{"question": "q", "options": [{"label": "a"}]}]}}"#;
        let dialog = classify(HookEvent::PreToolUse, payload)
            .expect("classified")
            .expect("a dialog");
        assert_eq!(
            dialog.data,
            json!({"questions": [{"question": "q", "header": null, "options": ["a"], "multi_select": false}]})
        );
    }

    #[test]
    fn dialog_kinds_are_the_event_kinds() {
        let names: Vec<&str> = DialogKind::ALL.iter().map(|k| k.as_str()).collect();
        assert_eq!(names, ["question", "permission", "plan"]);
        for kind in DialogKind::ALL {
            assert_eq!(DialogKind::from_name(kind.as_str()), Some(kind));
        }
        assert_eq!(DialogKind::from_name("turn-ended"), None);
        assert_eq!(DialogKind::Plan.event_kind(), EventKind::Plan);
        assert_eq!(
            DialogTool::from_name("ExitPlanMode"),
            Some(DialogTool::ExitPlanMode)
        );
        assert_eq!(DialogTool::from_name("Bash"), None);
    }

    fn question(annotations: Option<Value>) -> Response {
        let mut response = json!({"answers": {"Which color?": format!("{CANARY} red")}});
        if let Some(annotations) = annotations {
            response["annotations"] = annotations;
        }
        Response::parse(&response).expect("a question")
    }

    #[rstest]
    #[case::question(json!({"answers": {"q": "a"}}), DialogKind::Question)]
    #[case::question_annotated(json!({"answers": {}, "annotations": {"q": {"notes": "n"}}}), DialogKind::Question)]
    #[case::allow(json!({"behavior": "allow"}), DialogKind::Permission)]
    #[case::deny(json!({"behavior": "deny", "message": "no"}), DialogKind::Permission)]
    #[case::approve(json!({"behavior": "approve", "later": 1}), DialogKind::Plan)]
    #[case::revise(json!({"behavior": "revise", "message": null}), DialogKind::Plan)]
    fn response_parse_reads_each_shape(#[case] response: Value, #[case] kind: DialogKind) {
        assert_eq!(Response::parse(&response).map(|r| r.kind()), Ok(kind));
    }

    #[rstest]
    #[case::not_object(json!("allow"))]
    #[case::no_behavior(json!({"message": "m"}))]
    #[case::unknown_behavior(json!({"behavior": "maybe"}))]
    #[case::behavior_not_string(json!({"behavior": 1}))]
    #[case::message_not_string(json!({"behavior": "deny", "message": 3}))]
    #[case::answers_not_object(json!({"answers": ["a"]}))]
    #[case::answer_not_string(json!({"answers": {"q": 1}}))]
    #[case::annotations_not_object(json!({"answers": {}, "annotations": "n"}))]
    fn response_parse_refuses_a_shape_it_cannot_take(#[case] response: Value) {
        assert_eq!(
            Response::parse(&response),
            Err(AgentError::ResponseMalformed)
        );
    }

    #[test]
    fn response_round_trips_through_its_value() {
        for response in [
            json!({"answers": {"q": "a"}, "annotations": {"q": {"notes": "n"}}}),
            json!({"answers": {"q": "a"}}),
            json!({"behavior": "allow"}),
            json!({"behavior": "deny", "message": "no"}),
            json!({"behavior": "approve"}),
            json!({"behavior": "revise", "message": "rename it"}),
        ] {
            let parsed = Response::parse(&response).expect("parsed");
            assert_eq!(parsed.to_value(), response);
            assert_eq!(Response::parse(&parsed.to_value()), Ok(parsed));
        }
    }

    #[test]
    fn response_free_text_is_every_answer_annotation_and_message() {
        let annotated = question(Some(
            json!({"Which color?": {"notes": ["n1", {"k": "n2"}], "x": 1}}),
        ));
        let mut texts = annotated.free_text();
        texts.sort_unstable();
        assert_eq!(
            texts,
            [
                "Which color?",
                "canary-chain-value-5c1e red",
                "k",
                "n1",
                "n2",
                "notes",
                "x"
            ]
        );
        let revise = Response::parse(&json!({"behavior": "revise", "message": "m"})).expect("plan");
        assert_eq!(revise.free_text(), ["m"]);
        let allow = Response::parse(&json!({"behavior": "allow"})).expect("permission");
        assert!(allow.free_text().is_empty());
    }

    fn body(hook: HookEvent, input: &Value, response: &Response) -> Option<Value> {
        decision_body(hook, input, response).map(|b| serde_json::from_str(&b).expect("json"))
    }

    #[test]
    fn decision_body_question_on_pre_tool_use_is_allow_with_the_answers() {
        let input = json!({"questions": [{"question": "Which color?"}]});
        insta::assert_snapshot!(
            "question_answered",
            decision_body(HookEvent::PreToolUse, &input, &question(None)).expect("a body")
        );
        let annotations = json!({"Which color?": {"notes": format!("{CANARY} note")}});
        let annotated = question(Some(annotations.clone()));
        insta::assert_snapshot!(
            "question_answered_with_annotations",
            decision_body(HookEvent::PreToolUse, &input, &annotated).expect("a body")
        );
        let got = body(HookEvent::PreToolUse, &input, &annotated).expect("a body");
        let updated = &got["hookSpecificOutput"]["updatedInput"];
        assert_eq!(updated["questions"], input["questions"]);
        assert_eq!(updated["annotations"], annotations);
    }

    #[test]
    fn decision_body_plan_approve_is_pre_tool_use_allow_only() {
        let approve = Response::parse(&json!({"behavior": "approve"})).expect("plan");
        let input = json!({"plan": "p", "planFilePath": "f"});
        insta::assert_snapshot!(
            "plan_approved",
            decision_body(HookEvent::PreToolUse, &input, &approve).expect("a body")
        );
        assert_eq!(
            decision_body(HookEvent::PermissionRequest, &input, &approve),
            None
        );
    }

    #[test]
    fn decision_body_plan_revise_is_permission_request_deny_with_the_message() {
        let revise = Response::parse(
            &json!({"behavior": "revise", "message": format!("{CANARY} rename it")}),
        )
        .expect("plan");
        let input = json!({"plan": "p"});
        insta::assert_snapshot!(
            "plan_revised",
            decision_body(HookEvent::PermissionRequest, &input, &revise).expect("a body")
        );
        assert_eq!(decision_body(HookEvent::PreToolUse, &input, &revise), None);
        let bare = Response::parse(&json!({"behavior": "revise"})).expect("plan");
        assert_eq!(
            body(HookEvent::PermissionRequest, &input, &bare),
            Some(
                json!({"hookSpecificOutput": {"hookEventName": "PermissionRequest",
                "decision": {"behavior": "deny"}}})
            )
        );
    }

    #[test]
    fn decision_body_permission_allow_and_deny() {
        let input = json!({"command": "ls"});
        let allow = Response::parse(&json!({"behavior": "allow"})).expect("permission");
        insta::assert_snapshot!(
            "permission_allowed",
            decision_body(HookEvent::PermissionRequest, &input, &allow).expect("a body")
        );
        let deny =
            Response::parse(&json!({"behavior": "deny", "message": format!("{CANARY} not that")}))
                .expect("permission");
        insta::assert_snapshot!(
            "permission_denied",
            decision_body(HookEvent::PermissionRequest, &input, &deny).expect("a body")
        );
        assert_eq!(decision_body(HookEvent::PreToolUse, &input, &allow), None);
    }

    #[test]
    fn decision_body_question_on_permission_request_is_no_decision() {
        let input = json!({"questions": []});
        assert_eq!(
            decision_body(HookEvent::PermissionRequest, &input, &question(None)),
            None
        );
    }

    #[test]
    fn decision_body_question_with_a_non_object_input_keeps_only_the_answers() {
        let got = body(HookEvent::PreToolUse, &Value::Null, &question(None)).expect("a body");
        assert_eq!(
            got["hookSpecificOutput"]["updatedInput"],
            json!({"answers": {"Which color?": format!("{CANARY} red")}})
        );
    }
}
