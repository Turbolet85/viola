//! `claude agents --json`: the CLI's list of its live sessions, read by `viola list` (each row's
//! status, and the sessions viola does not wrap) and by `viola revive` (an id open elsewhere).
//! Pure functions over bytes: the child is spawned by the root binary. A row's text is content,
//! never a command, a path or a `ViolaName`.

use serde_json::{Map, Value};
use viola_core::MAX_FRAME;

/// The subcommand and its flag: the bare list, never `--all`.
pub const ARGS: [&str; 2] = ["agents", "--json"];

/// The keys a row carries as measured on 2.1.287 and 2.1.296.
pub const KEYS: [&str; 7] = [
    "cwd",
    "kind",
    "name",
    "pid",
    "sessionId",
    "startedAt",
    "status",
];

/// A session's state as the CLI names it, closed: any word but the two measured is unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Idle,
    Busy,
    Unknown,
}

impl AgentStatus {
    pub fn from_word(word: Option<&str>) -> Self {
        match word {
            Some("idle") => Self::Idle,
            Some("busy") => Self::Busy,
            _ => Self::Unknown,
        }
    }
}

/// One listed session. Only `session_id` is required; a key of another type reads as absent.
#[derive(Debug, Clone, PartialEq)]
pub struct AgentsRow {
    pub pid: Option<u32>,
    pub session_id: String,
    pub status: AgentStatus,
    pub kind: Option<String>,
    pub cwd: Option<String>,
    pub name: Option<String>,
    pub started_at: Option<i64>,
    /// The row as listed, for the recorder.
    pub raw: Value,
}

impl AgentsRow {
    /// Whether this row is the wrapped session with this child pid or this session id.
    pub fn joins(&self, child_pid: Option<u32>, agent_session_id: Option<&str>) -> bool {
        let by_pid = self.pid.is_some() && self.pid == child_pid;
        by_pid || agent_session_id == Some(self.session_id.as_str())
    }
}

/// A parsed list: the rows, the elements stepped over (not an object, or no string `sessionId`)
/// and the keys outside [`KEYS`] that were kept.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AgentsList {
    pub rows: Vec<AgentsRow>,
    pub skipped: u64,
    pub tolerated: u64,
}

/// Why a read gave no list; each word is a closed `parse-rejected` `detail`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentsFailure {
    Oversize,
    Malformed,
}

impl AgentsFailure {
    pub fn detail(self) -> &'static str {
        match self {
            Self::Oversize => "oversize",
            Self::Malformed => "malformed",
        }
    }
}

/// The CLI's output: one JSON array of at most [`MAX_FRAME`] bytes, at serde's default depth.
pub fn parse(bytes: &[u8]) -> Result<AgentsList, AgentsFailure> {
    if bytes.len() as u64 > MAX_FRAME {
        return Err(AgentsFailure::Oversize);
    }
    let Ok(Value::Array(elements)) = serde_json::from_slice::<Value>(bytes) else {
        return Err(AgentsFailure::Malformed);
    };
    let mut list = AgentsList::default();
    for element in elements {
        match element {
            Value::Object(fields) => match row(fields) {
                Some((row, tolerated)) => {
                    list.tolerated += tolerated;
                    list.rows.push(row);
                }
                None => list.skipped += 1,
            },
            _ => list.skipped += 1,
        }
    }
    Ok(list)
}

fn row(fields: Map<String, Value>) -> Option<(AgentsRow, u64)> {
    let session_id = fields.get("sessionId")?.as_str()?.to_owned();
    let text = |key: &str| fields.get(key).and_then(Value::as_str).map(str::to_owned);
    let tolerated = fields
        .keys()
        .filter(|k| !KEYS.contains(&k.as_str()))
        .count() as u64;
    let row = AgentsRow {
        pid: fields
            .get("pid")
            .and_then(Value::as_u64)
            .and_then(|pid| u32::try_from(pid).ok()),
        status: AgentStatus::from_word(fields.get("status").and_then(Value::as_str)),
        kind: text("kind"),
        cwd: text("cwd"),
        name: text("name"),
        started_at: fields.get("startedAt").and_then(Value::as_i64),
        session_id,
        raw: Value::Object(fields),
    };
    Some((row, tolerated))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;
    use rstest::rstest;
    use serde_json::json;

    const ID: &str = "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f";

    fn measured() -> Value {
        json!({
            "cwd": "/w/p",
            "kind": "interactive",
            "name": "a title",
            "pid": 4242,
            "sessionId": ID,
            "startedAt": 1_760_000_000_000_i64,
            "status": "busy",
        })
    }

    fn parsed(doc: &Value) -> AgentsList {
        parse(doc.to_string().as_bytes()).expect("a list")
    }

    #[test]
    fn agents_parse_takes_the_measured_seven_key_row() {
        let list = parsed(&json!([measured()]));
        assert_eq!(
            list.rows,
            [AgentsRow {
                pid: Some(4242),
                session_id: ID.to_owned(),
                status: AgentStatus::Busy,
                kind: Some("interactive".to_owned()),
                cwd: Some("/w/p".to_owned()),
                name: Some("a title".to_owned()),
                started_at: Some(1_760_000_000_000),
                raw: measured(),
            }]
        );
        assert_eq!((list.skipped, list.tolerated), (0, 0));
    }

    #[test]
    fn agents_parse_keeps_an_extra_key_and_counts_it_tolerated() {
        let mut row = measured();
        row["later"] = json!(1);
        row["other"] = json!("x");
        let list = parsed(&json!([row.clone()]));
        assert_eq!(list.rows.len(), 1);
        assert_eq!(list.rows[0].raw, row);
        assert_eq!((list.skipped, list.tolerated), (0, 2));
    }

    #[test]
    fn agents_parse_lists_a_background_row_with_status_unknown_joinable_only_by_id() {
        let background = json!({
            "cwd": "/w/p",
            "id": "b-1",
            "kind": "background",
            "name": "a job",
            "sessionId": ID,
            "startedAt": 1,
            "state": "running",
        });
        let list = parsed(&json!([background]));
        assert_eq!(list.rows.len(), 1);
        let row = &list.rows[0];
        assert_eq!(row.pid, None);
        assert_eq!(row.status, AgentStatus::Unknown);
        assert_eq!(row.kind.as_deref(), Some("background"));
        assert_eq!((list.skipped, list.tolerated), (0, 2));
        assert!(!row.joins(None, None));
        assert!(!row.joins(Some(4242), None));
        assert!(row.joins(None, Some(ID)));
    }

    #[rstest]
    #[case::no_session_id(json!({"pid": 1, "status": "idle"}))]
    #[case::a_numeric_session_id(json!({"pid": 1, "sessionId": 7}))]
    #[case::a_null_session_id(json!({"sessionId": null}))]
    #[case::a_string(json!("row"))]
    #[case::a_number(json!(1))]
    #[case::an_array(json!([measured()]))]
    #[case::null(json!(null))]
    fn agents_parse_skips_and_counts_an_element_without_a_string_session_id(
        #[case] element: Value,
    ) {
        let list = parsed(&json!([element, measured()]));
        assert_eq!(list.rows.len(), 1);
        assert_eq!(list.skipped, 1);
    }

    #[rstest]
    #[case::an_object(br#"{"sessionId":"x"}"#.as_slice())]
    #[case::a_string(br#""[]""#.as_slice())]
    #[case::null(b"null".as_slice())]
    #[case::empty(b"".as_slice())]
    #[case::not_json(b"No sessions".as_slice())]
    #[case::cut_short(br#"[{"sessionId":"#.as_slice())]
    #[case::two_arrays(b"[][]".as_slice())]
    #[case::not_utf8(b"[\"\xff\xfe\"]".as_slice())]
    fn agents_parse_of_a_document_that_is_not_one_array_is_malformed(#[case] bytes: &[u8]) {
        assert_eq!(parse(bytes), Err(AgentsFailure::Malformed));
    }

    #[test]
    fn agents_parse_of_an_array_past_the_default_depth_is_malformed() {
        let nested = |depth: usize| [vec![b'['; depth], vec![b']'; depth]].concat();
        assert_eq!(
            parse(&nested(100)),
            Ok(AgentsList {
                skipped: 1,
                ..AgentsList::default()
            })
        );
        assert_eq!(parse(&nested(200)), Err(AgentsFailure::Malformed));
    }

    #[test]
    fn agents_parse_takes_max_frame_bytes_and_refuses_one_more_as_oversize() {
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let mut doc = vec![b' '; cap];
        doc[0] = b'[';
        doc[cap - 1] = b']';
        assert_eq!(parse(&doc), Ok(AgentsList::default()));
        doc.push(b' ');
        assert_eq!(parse(&doc), Err(AgentsFailure::Oversize));
    }

    #[test]
    fn agents_parse_of_an_empty_array_is_no_rows() {
        assert_eq!(parse(b"[]"), Ok(AgentsList::default()));
    }

    #[rstest]
    #[case::idle(json!("idle"), AgentStatus::Idle)]
    #[case::busy(json!("busy"), AgentStatus::Busy)]
    #[case::another_word(json!("waiting"), AgentStatus::Unknown)]
    #[case::another_case(json!("Idle"), AgentStatus::Unknown)]
    #[case::empty(json!(""), AgentStatus::Unknown)]
    #[case::a_number(json!(1), AgentStatus::Unknown)]
    #[case::null(json!(null), AgentStatus::Unknown)]
    fn agents_status_maps_each_measured_word_and_nothing_else(
        #[case] status: Value,
        #[case] want: AgentStatus,
    ) {
        let mut row = measured();
        row["status"] = status;
        assert_eq!(parsed(&json!([row])).rows[0].status, want);
    }

    #[rstest]
    #[case::negative(json!(-1))]
    #[case::past_u32(json!(4_294_967_296_u64))]
    #[case::a_fraction(json!(42.5))]
    #[case::a_string(json!("42"))]
    fn agents_parse_reads_a_pid_that_is_no_u32_as_absent(#[case] pid: Value) {
        let mut row = measured();
        row["pid"] = pid;
        let list = parsed(&json!([row]));
        assert_eq!(list.rows[0].pid, None);
        assert_eq!(list.rows[0].session_id, ID);
    }

    #[rstest]
    #[case::by_pid(Some(4242), None, true)]
    #[case::by_session_id(None, Some(ID), true)]
    #[case::by_both(Some(4242), Some(ID), true)]
    #[case::pid_matches_id_does_not(Some(4242), Some("other"), true)]
    #[case::id_matches_pid_does_not(Some(1), Some(ID), true)]
    #[case::neither(Some(1), Some("other"), false)]
    #[case::no_keys(None, None, false)]
    fn agents_row_joins_by_child_pid_or_session_id(
        #[case] child_pid: Option<u32>,
        #[case] agent_session_id: Option<&str>,
        #[case] joins: bool,
    ) {
        let row = &parsed(&json!([measured()])).rows[0];
        assert_eq!(row.joins(child_pid, agent_session_id), joins);
    }

    #[test]
    fn agents_failure_details_are_the_closed_words() {
        assert_eq!(AgentsFailure::Oversize.detail(), "oversize");
        assert_eq!(AgentsFailure::Malformed.detail(), "malformed");
        assert_eq!(ARGS, ["agents", "--json"]);
    }

    fn config() -> ProptestConfig {
        ProptestConfig {
            cases: 512,
            failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel(
                "proptest-regressions",
            ))),
            ..ProptestConfig::default()
        }
    }

    /// Any JSON value, weighted towards a list of rows that mix the measured keys, wrong types and
    /// control characters.
    fn arbitrary_json() -> impl Strategy<Value = Value> {
        let leaf = prop_oneof![
            Just(Value::Null),
            any::<bool>().prop_map(Value::from),
            any::<i64>().prop_map(Value::from),
            any::<u64>().prop_map(Value::from),
            any::<f64>().prop_map(Value::from),
            ".{0,12}".prop_map(Value::from),
            "[\\x00-\\x1f\\x7f-\\x{9f}a-f0-9-]{0,12}".prop_map(Value::from),
        ];
        let key = prop_oneof![
            prop::sample::select(KEYS.to_vec()).prop_map(str::to_owned),
            "[a-zA-Z]{1,6}",
        ];
        leaf.prop_recursive(3, 24, 6, move |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..6).prop_map(Value::from),
                prop::collection::vec((key.clone(), inner), 0..8)
                    .prop_map(|pairs| Value::Object(pairs.into_iter().collect())),
            ]
        })
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn agents_prop_parse_never_panics_and_takes_ids_unchanged(doc in arbitrary_json()) {
            // The input is the text: a float need not read back as the value it was written from.
            let text = doc.to_string();
            let input: Value = serde_json::from_str(&text).expect("the text reads back");
            match parse(text.as_bytes()) {
                Ok(list) => {
                    let Value::Array(elements) = &input else {
                        return Err(TestCaseError::fail("a list from a non-array"));
                    };
                    let total = list.rows.len() as u64 + list.skipped;
                    prop_assert_eq!(total, elements.len() as u64);
                    for row in &list.rows {
                        prop_assert!(elements.contains(&row.raw));
                        prop_assert_eq!(row.raw.get("sessionId"), Some(&Value::from(row.session_id.as_str())));
                    }
                }
                Err(failure) => {
                    prop_assert_eq!(failure, AgentsFailure::Malformed);
                    prop_assert!(!input.is_array());
                }
            }
        }
    }
}
