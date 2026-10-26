//! The `link` / `unlink` names at the wrapper (test-plan §6 Security control negatives;
//! security-plan §Input Validation, channel frames): a frame written straight to a real wrapper's
//! endpoint, past every CLI check, whose `driver` or `driven` is no `ViolaName`, whose `driven` is
//! not the serving wrapper's name, or whose `driver` equals `driven`, is answered `-32602` with
//! `data: null`, and the wrapper appends no `link` or `unlink` record; a valid frame, the control,
//! is answered `ok` and appends one.

#[allow(dead_code)]
mod support;

use rstest::rstest;
use serde_json::{Map, Value, json};
use support::events::events;
use support::home::{Wrapper, booted_wrapper, snapshot_data};
use viola_channel::Client;

/// The link records of the instance's log.
fn link_records(wrapper: &Wrapper) -> Vec<Value> {
    events(&wrapper.instance_dir())
        .into_iter()
        .filter(|l| l["kind"] == "link" || l["kind"] == "unlink")
        .collect()
}

fn snapshot_links(wrapper: &Wrapper) -> Value {
    snapshot_data(&wrapper.instance_dir()).expect("snapshot")["links"].clone()
}

fn params(driver: &Value, driven: &Value) -> Map<String, Value> {
    let mut params = Map::new();
    for (key, value) in [("driver", driver), ("driven", driven)] {
        if !value.is_null() {
            params.insert(key.to_owned(), value.clone());
        }
    }
    params
}

#[rstest]
#[case::driver_traversal(json!("../x"), json!("builder"))]
#[case::driver_upper(json!("BUILDER"), json!("builder"))]
#[case::driver_slash(json!("a/b"), json!("builder"))]
#[case::driver_empty(json!(""), json!("builder"))]
#[case::driver_too_long(json!("a".repeat(33)), json!("builder"))]
#[case::driver_number(json!(7), json!("builder"))]
#[case::driven_traversal(json!("overseer"), json!("../x"))]
#[case::driven_upper(json!("overseer"), json!("BUILDER"))]
#[case::driven_slash(json!("overseer"), json!("a/b"))]
#[case::driven_empty(json!("overseer"), json!(""))]
#[case::driven_too_long(json!("overseer"), json!("a".repeat(33)))]
#[case::driven_number(json!("overseer"), json!(7))]
#[case::driven_foreign(json!("overseer"), json!("tester"))]
#[case::driver_is_driven(json!("builder"), json!("builder"))]
fn channel_link_names_are_validated_by_the_wrapper(
    booted_wrapper: Wrapper,
    #[case] driver: Value,
    #[case] driven: Value,
) {
    let snapshot = snapshot_data(&booted_wrapper.instance_dir()).expect("snapshot");
    let endpoint = snapshot["endpoint"].as_str().expect("endpoint").to_owned();
    let mut client = Client::connect(&endpoint, "cli").expect("connect");

    for method in ["link", "unlink"] {
        let reply = client
            .request(method, params(&driver, &driven))
            .expect("reply");
        assert_eq!(reply["error"]["code"], -32602, "{method}: {reply}");
        assert_eq!(reply["error"]["data"], Value::Null, "{method}: {reply}");
        assert!(reply.get("result").is_none(), "{method}: {reply}");
    }
    assert!(link_records(&booted_wrapper).is_empty());
    assert_eq!(snapshot_links(&booted_wrapper), json!([]));

    let control = client
        .request("link", params(&json!("overseer"), &json!("builder")))
        .expect("reply");
    assert_eq!(
        control["result"]["ok"]["already_linked"], false,
        "{control}"
    );
    let records = link_records(&booted_wrapper);
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(
        records[0]["data"],
        json!({"driver": "overseer", "driven": "builder"})
    );
    booted_wrapper.stop();
}
