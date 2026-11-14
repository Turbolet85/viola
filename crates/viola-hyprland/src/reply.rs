//! Pure parsers over a `hyprctl` reply's bytes: the length measured against the frame cap first,
//! serde's default depth, unknown keys tolerated and a key of another type read as absent. The
//! closed types hold no text field: every string the compositor prints but an address and the
//! version tag is dropped here, so no reply text can reach a line.

use serde_json::{Map, Value};
use viola_core::MAX_FRAME;

use crate::{Address, HyprlandError, Request, WorkspaceId};

/// The text a dispatch prints when the compositor took it.
pub const DISPATCH_OK: &str = "ok";

/// One window as `clients` and `activewindow` list it. Only `address` is required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub address: Address,
    pub pid: Option<u32>,
    pub workspace: Option<WorkspaceId>,
    pub monitor: Option<i64>,
    pub at: Option<(i64, i64)>,
    pub size: Option<(i64, i64)>,
    pub floating: Option<bool>,
    pub mapped: Option<bool>,
    pub hidden: Option<bool>,
    pub focus_history_id: Option<i64>,
    pub swallowing: Option<Address>,
    pub grouped: Vec<Address>,
}

/// One workspace as `workspaces` lists it. Only `id` is required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub monitor: Option<i64>,
    pub windows: Option<u64>,
    pub last_window: Option<Address>,
}

/// The compositor's release, read from `version`'s `tag` (`v<major>.<minor>.<patch>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HyprlandVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl std::fmt::Display for HyprlandVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl HyprlandVersion {
    /// `v<major>.<minor>.<patch>`, digits only: any other text is none.
    pub fn from_tag(tag: &str) -> Option<Self> {
        let mut parts = tag.strip_prefix('v')?.split('.');
        let mut number = || {
            let part = parts.next()?;
            if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            part.parse::<u32>().ok()
        };
        let version = Self {
            major: number()?,
            minor: number()?,
            patch: number()?,
        };
        parts.next().is_none().then_some(version)
    }
}

/// A parsed reply, one arm per request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    Clients(Vec<Window>),
    ActiveWindow(Option<Window>),
    Workspaces(Vec<Workspace>),
    Version(HyprlandVersion),
    Dispatched,
}

/// The reply to `request`, parsed by its own parser.
pub fn parse(request: &Request, bytes: &[u8]) -> Result<Reply, HyprlandError> {
    match request {
        Request::Clients => parse_clients(bytes).map(Reply::Clients),
        Request::ActiveWindow => parse_active_window(bytes).map(Reply::ActiveWindow),
        Request::Workspaces => parse_workspaces(bytes).map(Reply::Workspaces),
        Request::Version => parse_version(bytes).map(Reply::Version),
        Request::FocusWindow(_) | Request::Workspace(_) => {
            parse_dispatch(bytes).map(|()| Reply::Dispatched)
        }
    }
}

fn json(bytes: &[u8]) -> Result<Value, HyprlandError> {
    if bytes.len() as u64 > MAX_FRAME {
        return Err(HyprlandError::Oversize);
    }
    serde_json::from_slice(bytes).map_err(|_| HyprlandError::Malformed)
}

/// `clients`: one JSON array; an element that is no object or holds no address is stepped over.
pub fn parse_clients(bytes: &[u8]) -> Result<Vec<Window>, HyprlandError> {
    let Value::Array(elements) = json(bytes)? else {
        return Err(HyprlandError::Malformed);
    };
    Ok(elements
        .iter()
        .filter_map(Value::as_object)
        .filter_map(window)
        .collect())
}

/// `activewindow`: one JSON object; the empty object, or one with no address, is no window.
pub fn parse_active_window(bytes: &[u8]) -> Result<Option<Window>, HyprlandError> {
    let Value::Object(fields) = json(bytes)? else {
        return Err(HyprlandError::Malformed);
    };
    Ok(window(&fields))
}

/// `workspaces`: one JSON array; an element that is no object or holds no `i32` id is stepped over.
pub fn parse_workspaces(bytes: &[u8]) -> Result<Vec<Workspace>, HyprlandError> {
    let Value::Array(elements) = json(bytes)? else {
        return Err(HyprlandError::Malformed);
    };
    Ok(elements
        .iter()
        .filter_map(Value::as_object)
        .filter_map(workspace)
        .collect())
}

/// `version`: one JSON object whose `tag` is the closed version form.
pub fn parse_version(bytes: &[u8]) -> Result<HyprlandVersion, HyprlandError> {
    let Value::Object(fields) = json(bytes)? else {
        return Err(HyprlandError::Malformed);
    };
    fields
        .get("tag")
        .and_then(Value::as_str)
        .and_then(HyprlandVersion::from_tag)
        .ok_or(HyprlandError::Malformed)
}

/// A dispatch: taken when the reply is the success text (one trailing newline allowed), else
/// refused.
pub fn parse_dispatch(bytes: &[u8]) -> Result<(), HyprlandError> {
    if bytes.len() as u64 > MAX_FRAME {
        return Err(HyprlandError::Oversize);
    }
    let text = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    if text == DISPATCH_OK.as_bytes() {
        Ok(())
    } else {
        Err(HyprlandError::Refused)
    }
}

fn address(value: Option<&Value>) -> Option<Address> {
    Address::try_new(value?.as_str()?.to_owned()).ok()
}

fn pair(value: Option<&Value>) -> Option<(i64, i64)> {
    match value?.as_array()?.as_slice() {
        [x, y] => Some((x.as_i64()?, y.as_i64()?)),
        _ => None,
    }
}

fn workspace_id(value: Option<&Value>) -> Option<WorkspaceId> {
    let id = i32::try_from(value?.as_i64()?).ok()?;
    Some(WorkspaceId::new(id))
}

fn window(fields: &Map<String, Value>) -> Option<Window> {
    let int = |key: &str| fields.get(key).and_then(Value::as_i64);
    let flag = |key: &str| fields.get(key).and_then(Value::as_bool);
    Some(Window {
        address: address(fields.get("address"))?,
        pid: fields
            .get("pid")
            .and_then(Value::as_u64)
            .and_then(|pid| u32::try_from(pid).ok()),
        workspace: workspace_id(fields.get("workspace").and_then(|w| w.get("id"))),
        monitor: int("monitor"),
        at: pair(fields.get("at")),
        size: pair(fields.get("size")),
        floating: flag("floating"),
        mapped: flag("mapped"),
        hidden: flag("hidden"),
        focus_history_id: int("focusHistoryID"),
        swallowing: address(fields.get("swallowing")),
        grouped: fields
            .get("grouped")
            .and_then(Value::as_array)
            .map(|entries| entries.iter().filter_map(|e| address(Some(e))).collect())
            .unwrap_or_default(),
    })
}

fn workspace(fields: &Map<String, Value>) -> Option<Workspace> {
    Some(Workspace {
        id: workspace_id(fields.get("id"))?,
        monitor: fields.get("monitorID").and_then(Value::as_i64),
        windows: fields.get("windows").and_then(Value::as_u64),
        last_window: address(fields.get("lastwindow")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;
    use rstest::rstest;
    use serde_json::json;
    use std::path::{Path, PathBuf};

    fn addr(text: &str) -> Address {
        Address::try_new(text.to_owned()).expect("an address")
    }

    /// A client as 0.56.2 lists it, its text keys holding a canary the parse must drop.
    fn client() -> Value {
        json!({
            "address": "0x55d4c1a2b3c0",
            "mapped": true,
            "hidden": false,
            "at": [10, 42],
            "size": [1900, 1030],
            "workspace": {"id": 3, "name": "CANARY-WS"},
            "floating": false,
            "monitor": 0,
            "class": "CANARY-CLASS",
            "initialClass": "CANARY-CLASS",
            "pid": 4242,
            "xwayland": false,
            "pinned": false,
            "fullscreen": 0,
            "grouped": ["0x55d4c1a2b3c0", "0x55d4c1a2b3d0", "CANARY"],
            "tags": ["CANARY-TAG"],
            "swallowing": "0x0",
            "focusHistoryID": 1,
            "xdgTag": "CANARY",
            "contentType": "none",
            "stableId": "CANARY-ID",
        })
    }

    fn parsed_client() -> Window {
        Window {
            address: addr("0x55d4c1a2b3c0"),
            pid: Some(4242),
            workspace: Some(WorkspaceId::new(3)),
            monitor: Some(0),
            at: Some((10, 42)),
            size: Some((1900, 1030)),
            floating: Some(false),
            mapped: Some(true),
            hidden: Some(false),
            focus_history_id: Some(1),
            swallowing: Some(addr("0x0")),
            grouped: vec![addr("0x55d4c1a2b3c0"), addr("0x55d4c1a2b3d0")],
        }
    }

    fn bytes(doc: &Value) -> Vec<u8> {
        doc.to_string().into_bytes()
    }

    #[test]
    fn parse_clients_takes_each_relied_field_and_drops_every_text() {
        let windows = parse_clients(&bytes(&json!([client()]))).expect("windows");
        assert_eq!(windows, [parsed_client()]);
        assert!(!format!("{windows:?}").contains("CANARY"));
    }

    #[test]
    fn parse_clients_steps_over_an_element_without_an_address() {
        let mut no_address = client();
        no_address
            .as_object_mut()
            .expect("object")
            .remove("address");
        let mut bad_address = client();
        bad_address["address"] = json!("0xZZ");
        let doc = json!([no_address, bad_address, "row", 1, null, client()]);
        assert_eq!(parse_clients(&bytes(&doc)), Ok(vec![parsed_client()]));
    }

    #[rstest]
    #[case::pid("pid", json!("4242"))]
    #[case::pid_negative("pid", json!(-1))]
    #[case::pid_past_u32("pid", json!(4_294_967_296_u64))]
    #[case::workspace("workspace", json!(3))]
    #[case::workspace_id_past_i32("workspace", json!({"id": 2_147_483_648_i64}))]
    #[case::monitor("monitor", json!("0"))]
    #[case::at("at", json!([10]))]
    #[case::size("size", json!([1900, "1030"]))]
    #[case::floating("floating", json!(0))]
    #[case::mapped("mapped", json!("true"))]
    #[case::hidden("hidden", json!(null))]
    #[case::focus_history_id("focusHistoryID", json!(1.5))]
    #[case::swallowing("swallowing", json!(0))]
    fn parse_clients_reads_a_key_of_another_type_as_absent(
        #[case] key: &str,
        #[case] value: Value,
    ) {
        let mut doc = client();
        doc[key] = value;
        let window = parse_clients(&bytes(&json!([doc]))).expect("windows")[0].clone();
        let want = Window {
            pid: if key == "pid" { None } else { Some(4242) },
            workspace: (key != "workspace").then(|| WorkspaceId::new(3)),
            monitor: (key != "monitor").then_some(0),
            at: (key != "at").then_some((10, 42)),
            size: (key != "size").then_some((1900, 1030)),
            floating: (key != "floating").then_some(false),
            mapped: (key != "mapped").then_some(true),
            hidden: (key != "hidden").then_some(false),
            focus_history_id: (key != "focusHistoryID").then_some(1),
            swallowing: (key != "swallowing").then(|| addr("0x0")),
            ..parsed_client()
        };
        assert_eq!(window, want);
    }

    #[test]
    fn parse_active_window_takes_a_window_and_reads_the_empty_object_as_none() {
        assert_eq!(
            parse_active_window(&bytes(&client())),
            Ok(Some(parsed_client()))
        );
        assert_eq!(parse_active_window(b"{}"), Ok(None));
        assert_eq!(parse_active_window(br#"{"address":7}"#), Ok(None));
    }

    #[test]
    fn parse_workspaces_takes_id_monitor_windows_and_last_window_only() {
        let doc = json!([
            {"id": 1, "name": "CANARY-NAME", "monitor": "CANARY-MON", "monitorID": 0,
             "windows": 2, "hasfullscreen": false, "lastwindow": "0x55d4c1a2b3c0",
             "ispersistent": false},
            {"id": -98, "monitorID": 1, "windows": 0, "lastwindow": "0x0"},
            {"id": "1"},
            {"name": "no id"},
            {"id": 2_147_483_648_i64},
        ]);
        let got = parse_workspaces(&bytes(&doc)).expect("workspaces");
        assert_eq!(
            got,
            [
                Workspace {
                    id: WorkspaceId::new(1),
                    monitor: Some(0),
                    windows: Some(2),
                    last_window: Some(addr("0x55d4c1a2b3c0")),
                },
                Workspace {
                    id: WorkspaceId::new(-98),
                    monitor: Some(1),
                    windows: Some(0),
                    last_window: Some(addr("0x0")),
                },
            ]
        );
        assert!(!format!("{got:?}").contains("CANARY"));
    }

    #[rstest]
    #[case::measured("v0.56.2", Some((0, 56, 2)))]
    #[case::wide("v10.200.3000", Some((10, 200, 3000)))]
    #[case::leading_zeros("v00.056.2", Some((0, 56, 2)))]
    #[case::no_v("0.56.2", None)]
    #[case::a_suffix("v0.56.2-12-gabcdef", None)]
    #[case::two_parts("v0.56", None)]
    #[case::four_parts("v0.56.2.1", None)]
    #[case::empty_part("v0..2", None)]
    #[case::a_sign("v0.+56.2", None)]
    #[case::past_u32("v4294967296.0.0", None)]
    #[case::a_newline("v0.56.2\n", None)]
    fn version_from_tag_takes_only_the_digit_form(
        #[case] tag: &str,
        #[case] want: Option<(u32, u32, u32)>,
    ) {
        let got = HyprlandVersion::from_tag(tag).map(|v| (v.major, v.minor, v.patch));
        assert_eq!(got, want);
    }

    #[test]
    fn parse_version_reads_the_tag_and_drops_every_other_key() {
        let doc = json!({"branch": "CANARY", "commit": "abc", "tag": "v0.56.2",
                         "commit_message": "CANARY", "flags": ["CANARY"]});
        let version = parse_version(&bytes(&doc)).expect("a version");
        assert_eq!(version.to_string(), "0.56.2");
        assert_eq!(
            parse_version(br#"{"tag":"0.56.2"}"#),
            Err(HyprlandError::Malformed)
        );
        assert_eq!(
            parse_version(br#"{"tag":7}"#),
            Err(HyprlandError::Malformed)
        );
        assert_eq!(parse_version(b"{}"), Err(HyprlandError::Malformed));
    }

    #[rstest]
    #[case::ok(b"ok".as_slice(), Ok(()))]
    #[case::ok_newline(b"ok\n".as_slice(), Ok(()))]
    #[case::two_newlines(b"ok\n\n".as_slice(), Err(HyprlandError::Refused))]
    #[case::an_error(b"Invalid dispatcher".as_slice(), Err(HyprlandError::Refused))]
    #[case::empty(b"".as_slice(), Err(HyprlandError::Refused))]
    #[case::upper(b"OK".as_slice(), Err(HyprlandError::Refused))]
    fn parse_dispatch_takes_only_the_success_text(
        #[case] reply: &[u8],
        #[case] want: Result<(), HyprlandError>,
    ) {
        assert_eq!(parse_dispatch(reply), want);
    }

    #[rstest]
    #[case::an_object(br#"{"address":"0x1"}"#.as_slice())]
    #[case::null(b"null".as_slice())]
    #[case::empty(b"".as_slice())]
    #[case::not_json(b"HYPRLAND_INSTANCE_SIGNATURE not set".as_slice())]
    #[case::cut_short(br#"[{"address":"#.as_slice())]
    #[case::two_arrays(b"[][]".as_slice())]
    #[case::not_utf8(b"[\"\xff\xfe\"]".as_slice())]
    fn parse_clients_of_a_document_that_is_not_one_array_is_malformed(#[case] reply: &[u8]) {
        assert_eq!(parse_clients(reply), Err(HyprlandError::Malformed));
        assert_eq!(parse_workspaces(reply), Err(HyprlandError::Malformed));
    }

    #[rstest]
    #[case::an_array(b"[]".as_slice())]
    #[case::a_string(br#""ok""#.as_slice())]
    #[case::not_json(b"ok".as_slice())]
    fn parse_active_window_and_version_of_no_object_are_malformed(#[case] reply: &[u8]) {
        assert_eq!(parse_active_window(reply), Err(HyprlandError::Malformed));
        assert_eq!(parse_version(reply), Err(HyprlandError::Malformed));
    }

    #[test]
    fn parse_of_an_array_past_the_default_depth_is_malformed() {
        let nested = |depth: usize| [vec![b'['; depth], vec![b']'; depth]].concat();
        assert_eq!(parse_clients(&nested(100)), Ok(Vec::new()));
        assert_eq!(parse_clients(&nested(200)), Err(HyprlandError::Malformed));
    }

    #[test]
    fn parse_clients_takes_max_frame_bytes_and_refuses_one_more_as_oversize() {
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let mut doc = vec![b' '; cap];
        doc[0] = b'[';
        doc[cap - 1] = b']';
        assert_eq!(parse_clients(&doc), Ok(Vec::new()));
        doc.push(b' ');
        assert_eq!(parse_clients(&doc), Err(HyprlandError::Oversize));
    }

    #[test]
    fn parse_dispatch_takes_max_frame_bytes_as_refused_and_refuses_one_more_as_oversize() {
        let mut reply = vec![b' '; usize::try_from(MAX_FRAME).expect("fits")];
        assert_eq!(parse_dispatch(&reply), Err(HyprlandError::Refused));
        reply.push(b' ');
        assert_eq!(parse_dispatch(&reply), Err(HyprlandError::Oversize));
    }

    #[test]
    fn every_parser_refuses_one_byte_past_the_cap_as_oversize() {
        let over = vec![b' '; usize::try_from(MAX_FRAME).expect("fits") + 1];
        assert_eq!(parse_active_window(&over), Err(HyprlandError::Oversize));
        assert_eq!(parse_workspaces(&over), Err(HyprlandError::Oversize));
        assert_eq!(parse_version(&over), Err(HyprlandError::Oversize));
        assert_eq!(parse_dispatch(&over), Err(HyprlandError::Oversize));
    }

    #[test]
    fn parse_routes_each_request_to_its_own_parser() {
        let id = WorkspaceId::new(1);
        assert_eq!(
            parse(&Request::Clients, b"[]"),
            Ok(Reply::Clients(Vec::new()))
        );
        assert_eq!(
            parse(&Request::ActiveWindow, b"{}"),
            Ok(Reply::ActiveWindow(None))
        );
        assert_eq!(
            parse(&Request::Workspaces, b"[]"),
            Ok(Reply::Workspaces(Vec::new()))
        );
        assert_eq!(
            parse(&Request::Version, br#"{"tag":"v0.56.2"}"#),
            Ok(Reply::Version(HyprlandVersion {
                major: 0,
                minor: 56,
                patch: 2
            }))
        );
        assert_eq!(
            parse(&Request::FocusWindow(addr("0x1")), b"ok\n"),
            Ok(Reply::Dispatched)
        );
        assert_eq!(parse(&Request::Workspace(id), b"ok"), Ok(Reply::Dispatched));
        assert_eq!(
            parse(&Request::Workspace(id), b"[]"),
            Err(HyprlandError::Refused)
        );
    }

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("the workspace root")
            .to_path_buf()
    }

    fn request_of(word: &str) -> Request {
        match word {
            "clients" => Request::Clients,
            "active-window" => Request::ActiveWindow,
            "workspaces" => Request::Workspaces,
            "version" => Request::Version,
            "focus-window" => Request::FocusWindow(addr("0x1")),
            "workspace" => Request::Workspace(WorkspaceId::new(1)),
            other => panic!("no request word {other}"),
        }
    }

    /// Each recorded reply under `fixtures/hyprland/<version>/` that exited 0 parses through its own
    /// parser; a set not yet recorded is walked as none (the replay contract's walk refuses empty).
    #[test]
    fn every_recorded_reply_parses_through_its_own_parser() {
        let root = workspace_root().join("fixtures/hyprland");
        let Ok(versions) = std::fs::read_dir(&root) else {
            return;
        };
        for version in versions {
            let dir = version.expect("an entry").path();
            for file in std::fs::read_dir(&dir).expect("a version dir") {
                let path = file.expect("an entry").path();
                let doc: Value = serde_json::from_slice(&std::fs::read(&path).expect("read"))
                    .expect("a fixture");
                let request = request_of(doc["request"].as_str().expect("a request word"));
                if doc["exit"] != json!(0) {
                    continue;
                }
                let stdout = doc["stdout"].as_str().expect("stdout text");
                assert!(
                    parse(&request, stdout.as_bytes()).is_ok(),
                    "{} does not parse",
                    path.display()
                );
            }
        }
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

    const KEYS: [&str; 16] = [
        "address",
        "pid",
        "workspace",
        "monitor",
        "monitorID",
        "at",
        "size",
        "floating",
        "mapped",
        "hidden",
        "focusHistoryID",
        "swallowing",
        "grouped",
        "id",
        "lastwindow",
        "tag",
    ];

    /// Any JSON value, weighted towards the relied keys, addresses, version tags, wrong types and
    /// control characters.
    fn arbitrary_json() -> impl Strategy<Value = Value> {
        let leaf = prop_oneof![
            Just(Value::Null),
            any::<bool>().prop_map(Value::from),
            any::<i64>().prop_map(Value::from),
            any::<u64>().prop_map(Value::from),
            any::<f64>().prop_map(Value::from),
            ".{0,12}".prop_map(Value::from),
            "0x[0-9a-fA-F]{0,18}".prop_map(Value::from),
            "v[0-9]{0,3}\\.[0-9]{0,3}\\.[0-9]{0,3}".prop_map(Value::from),
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

    fn input(doc: &Value) -> (String, Value) {
        // The input is the text: a float need not read back as the value it was written from.
        let text = doc.to_string();
        let value = serde_json::from_str(&text).expect("the text reads back");
        (text, value)
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn clients_prop_parse_never_panics_and_takes_each_address_unchanged(doc in arbitrary_json()) {
            let (text, value) = input(&doc);
            match parse_clients(text.as_bytes()) {
                Ok(windows) => {
                    let Value::Array(elements) = &value else {
                        return Err(TestCaseError::fail("windows from a non-array"));
                    };
                    prop_assert!(windows.len() <= elements.len());
                    for window in &windows {
                        let address = Value::from(window.address.as_ref());
                        prop_assert!(elements.iter().any(|e| e.get("address") == Some(&address)));
                    }
                }
                Err(error) => {
                    prop_assert_eq!(error, HyprlandError::Malformed);
                    prop_assert!(!value.is_array());
                }
            }
        }

        #[test]
        fn active_window_prop_parse_never_panics_and_reads_only_an_object(doc in arbitrary_json()) {
            let (text, value) = input(&doc);
            match parse_active_window(text.as_bytes()) {
                Ok(Some(window)) => {
                    prop_assert_eq!(value.get("address"), Some(&Value::from(window.address.as_ref())));
                }
                Ok(None) => prop_assert!(value.is_object()),
                Err(error) => {
                    prop_assert_eq!(error, HyprlandError::Malformed);
                    prop_assert!(!value.is_object());
                }
            }
        }

        #[test]
        fn workspaces_prop_parse_never_panics_and_takes_each_id_unchanged(doc in arbitrary_json()) {
            let (text, value) = input(&doc);
            match parse_workspaces(text.as_bytes()) {
                Ok(workspaces) => {
                    let Value::Array(elements) = &value else {
                        return Err(TestCaseError::fail("workspaces from a non-array"));
                    };
                    for workspace in &workspaces {
                        let id = Value::from(workspace.id.into_inner());
                        prop_assert!(elements.iter().any(|e| e.get("id") == Some(&id)));
                    }
                }
                Err(error) => {
                    prop_assert_eq!(error, HyprlandError::Malformed);
                    prop_assert!(!value.is_array());
                }
            }
        }

        #[test]
        fn version_prop_parse_never_panics_and_reads_back_its_tag(doc in arbitrary_json()) {
            let (text, value) = input(&doc);
            match parse_version(text.as_bytes()) {
                Ok(version) => {
                    // A tag with leading zeros (`v00.0.0`) reads as the same version, not the same text.
                    let tag = value.get("tag").and_then(Value::as_str);
                    prop_assert_eq!(tag.and_then(HyprlandVersion::from_tag), Some(version));
                    let printed = format!("v{version}");
                    prop_assert_eq!(HyprlandVersion::from_tag(&printed), Some(version));
                }
                Err(error) => prop_assert_eq!(error, HyprlandError::Malformed),
            }
        }

        #[test]
        fn dispatch_prop_parse_never_panics_and_takes_only_the_success_text(reply in prop::collection::vec(any::<u8>(), 0..8)) {
            let taken = parse_dispatch(&reply).is_ok();
            prop_assert_eq!(taken, reply == b"ok" || reply == b"ok\n");
        }
    }
}
