use super::*;

/// A start over a log that holds a link: the first snapshot holds that pair with its first
/// `since`, and the start appends its one record and no `link`.
#[test]
fn links_set_at_a_start_over_a_log_with_a_link_is_in_the_first_snapshot() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let instance_dir = tmp.path().join("instance");
    create_private_dir(&instance_dir).expect("the instance dir");
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let linked = EventLine::new(
        &name,
        EventKind::Link,
        Source::Wrapper,
        json!({"driver": "overseer", "driven": "builder"}),
        Utc::now(),
    );
    append_event(&instance_dir, &linked).expect("append");

    let snapshot = first_snapshot(&instance_dir, tmp.path());
    assert_eq!(
        snapshot.links,
        [json!({"driver": "overseer", "driven": "builder", "since": linked.ts})]
    );
    let kinds: Vec<Value> = events_in(&instance_dir)
        .iter()
        .map(|e| e["kind"].clone())
        .collect();
    assert_eq!(kinds, ["link", "wheel", "lineage", "launch"]);
}

/// The first snapshot records where the child is spawned, and the start event stands as it
/// was: the wheel starts with the driver.
#[test]
fn snapshot_cwd_of_a_start_is_the_spawn_directory() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let instance_dir = tmp.path().join("instance");
    let spawn_dir = tmp.path().join("work dir");
    let snapshot = first_snapshot(&instance_dir, &spawn_dir);
    assert_eq!(
        snapshot.cwd,
        Some(spawn_dir.to_str().expect("utf-8").to_owned())
    );
    let events = events_in(&instance_dir);
    let read: Vec<(&Value, &Value)> = events
        .iter()
        .filter(|e| e["kind"] != "launch")
        .map(|e| (&e["kind"], &e["data"]))
        .collect();
    assert_eq!(
        read,
        [
            (
                &json!("wheel"),
                &json!({"holder": "driver", "cause": "start"})
            ),
            (&json!("lineage"), &json!({"parent": null, "role": null})),
        ]
    );
}

/// The snapshot holds the lineage before the start's lines; the `lineage` line, `source`
/// `wrapper`, follows the `wheel` start line directly.
#[test]
fn start_state_writes_the_lineage_line_after_the_wheel_line() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let instance_dir = tmp.path().join("instance");
    let lineage = Lineage {
        parent: Some("overseer".to_owned()),
        role: Some("builder".to_owned()),
    };
    let snapshot = first_snapshot_of(&instance_dir, tmp.path(), lineage.clone());
    assert_eq!(snapshot.lineage, Some(lineage));
    let events = events_in(&instance_dir);
    let read: Vec<(&Value, &Value, &Value)> = events
        .iter()
        .filter(|e| e["kind"] != "launch")
        .map(|e| (&e["kind"], &e["source"], &e["data"]))
        .collect();
    assert_eq!(
        read,
        [
            (
                &json!("wheel"),
                &json!("wrapper"),
                &json!({"holder": "driver", "cause": "start"})
            ),
            (
                &json!("lineage"),
                &json!("wrapper"),
                &json!({"parent": "overseer", "role": "builder"})
            ),
        ]
    );
}

/// The snapshot holds the pair before the start's lines; the `launch` line, `source`
/// `wrapper`, follows the `lineage` line directly and names the pair handed.
#[test]
fn start_state_writes_the_launch_line_after_the_lineage_line() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let instance_dir = tmp.path().join("instance");
    let pair = Pair {
        model: "haiku".to_owned(),
        effort: "low".to_owned(),
    };
    let snapshot = first_snapshot_with(&instance_dir, tmp.path(), Lineage::default(), pair.clone());
    assert_eq!(snapshot.launch, Some(pair));
    let events = events_in(&instance_dir);
    let read: Vec<(&Value, &Value)> = events.iter().map(|e| (&e["kind"], &e["source"])).collect();
    assert_eq!(
        read,
        [
            (&json!("wheel"), &json!("wrapper")),
            (&json!("lineage"), &json!("wrapper")),
            (&json!("launch"), &json!("wrapper")),
        ]
    );
    assert_eq!(
        events[2]["data"],
        json!({"model": "haiku", "effort": "low"})
    );
}

/// An opened start: the snapshot holds the window before the start's lines, and the `window`
/// line, `source` `wrapper`, follows the `launch` line directly.
#[test]
fn start_state_writes_the_window_line_after_the_launch_line() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let instance_dir = tmp.path().join("instance");
    let window = Window {
        address: Some("0x555aa16bde50".to_owned()),
        workspace: Some(1),
        placement: "placed".to_owned(),
        origin: "cli".to_owned(),
    };
    let snapshot = first_snapshot_opened(
        &instance_dir,
        tmp.path(),
        Lineage::default(),
        run::launch::compiled(),
        Some(window.clone()),
    );
    assert_eq!(snapshot.window, Some(window));
    let events = events_in(&instance_dir);
    let read: Vec<(&Value, &Value)> = events.iter().map(|e| (&e["kind"], &e["source"])).collect();
    assert_eq!(
        read,
        [
            (&json!("wheel"), &json!("wrapper")),
            (&json!("lineage"), &json!("wrapper")),
            (&json!("launch"), &json!("wrapper")),
            (&json!("window"), &json!("wrapper")),
        ]
    );
    assert_eq!(
        events[3]["data"],
        json!({"address": "0x555aa16bde50", "workspace": 1, "placement": "placed",
            "origin": "cli"})
    );
}

#[test]
fn run_args_refuse_a_place_without_an_origin() {
    assert_eq!(
        parse(&["builder", "--place", "1", "--", "claude"]).err(),
        Some(clap::error::ErrorKind::MissingRequiredArgument)
    );
    assert_eq!(
        parse(&["builder", "--origin", "human", "--", "claude"]).err(),
        Some(clap::error::ErrorKind::ValueValidation)
    );
    assert_eq!(
        parse(&["builder", "--origin", "cli", "--place", "x", "--", "claude"]).err(),
        Some(clap::error::ErrorKind::ValueValidation)
    );
}

#[test]
fn run_args_take_a_hidden_origin_and_place() {
    let args = parse(&[
        "builder", "--origin", "agent", "--place", "3", "--", "claude",
    ])
    .expect("args");
    assert_eq!(
        (args.origin.as_deref(), args.place),
        (Some("agent"), Some(3))
    );
    let args = parse(&["builder", "--origin", "cli", "--", "claude"]).expect("args");
    assert_eq!((args.origin.as_deref(), args.place), (Some("cli"), None));
    let args = parse(&["builder", "--", "claude"]).expect("args");
    assert_eq!((args.origin, args.place), (None, None));
    use clap::CommandFactory as _;
    let help = Line::command().render_help().to_string();
    assert!(
        !help.contains("--origin") && !help.contains("--place"),
        "{help}"
    );
}

#[rstest::rstest]
#[case::both(&["builder", "--parent", "overseer", "--role", "builder", "--", "claude"], Ok((Some("overseer"), Some("builder"))))]
#[case::parent_alone(&["builder", "--parent", "overseer", "--", "claude"], Ok((Some("overseer"), None)))]
#[case::neither(&["builder", "--", "claude"], Ok((None, None)))]
#[case::bad_name(&["builder", "--parent", "Over Seer", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
#[case::bad_role(&["builder", "--role", "Builder!", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
#[case::self_parent(&["builder", "--parent", "builder", "--", "claude"], Err(clap::error::ErrorKind::ArgumentConflict))]
fn run_args_take_parent_and_role(
    #[case] words: &[&str],
    #[case] expected: Result<(Option<&str>, Option<&str>), clap::error::ErrorKind>,
) {
    let parsed =
        parse(words).map(|args| (args.parent.map(|name| name.as_ref().to_owned()), args.role));
    let expected =
        expected.map(|(parent, role)| (parent.map(str::to_owned), role.map(str::to_owned)));
    assert_eq!(parsed, expected);
    if let Err(kind) = parsed {
        let error = clap::Error::new(kind);
        assert_eq!(error.exit_code(), 2, "{kind:?}");
    }
}

#[rstest::rstest]
#[case::both(&["builder", "--model", "haiku", "--effort", "low", "--", "claude"], Ok((Some("haiku"), Some("low"))))]
#[case::model_alone(&["builder", "--model", "claude-opus-5-5", "--", "claude"], Ok((Some("claude-opus-5-5"), None)))]
#[case::effort_alone(&["builder", "--effort", "xhigh", "--", "claude"], Ok((None, Some("xhigh"))))]
#[case::neither(&["builder", "--", "claude"], Ok((None, None)))]
#[case::bad_effort(&["builder", "--effort", "huge", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
#[case::bad_model(&["builder", "--model", "Opus", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
fn run_args_take_model_and_effort(
    #[case] words: &[&str],
    #[case] expected: Result<(Option<&str>, Option<&str>), clap::error::ErrorKind>,
) {
    let parsed = parse(words).map(|args| (args.model, args.effort));
    let expected =
        expected.map(|(model, effort)| (model.map(str::to_owned), effort.map(str::to_owned)));
    assert_eq!(parsed, expected);
    if let Err(kind) = parsed {
        assert_eq!(clap::Error::new(kind).exit_code(), 2, "{kind:?}");
    }
}

#[rstest::rstest]
#[case::spaced(&["builder", "--", "claude", "--model", "x"], false)]
#[case::effort_equals(&["builder", "--", "claude", "--effort=low"], false)]
#[case::model_equals(&["builder", "--model", "haiku", "--", "claude", "--model=x"], false)]
#[case::a_longer_word(&["builder", "--", "claude", "--modelx"], true)]
fn run_args_refuse_a_pair_in_the_callers_words(#[case] words: &[&str], #[case] passes: bool) {
    assert_eq!(parse(words).is_ok(), passes, "{words:?}");
    if !passes {
        assert_eq!(
            parse(words).err(),
            Some(clap::error::ErrorKind::ArgumentConflict)
        );
        let line = pair_after_dashes().to_string();
        assert!(line.contains("'--model <MODEL>'") && line.contains("'--effort <EFFORT>'"));
        assert!(
            !line.contains("--model=x") && !line.contains("claude"),
            "{line}"
        );
    }
}

#[test]
fn run_lineage_takes_the_flag_over_the_inherited_name() {
    let args = parse(&["builder", "--parent", "overseer", "--", "claude"]).expect("args");
    assert_eq!(
        lineage_of(&args, Some(named("tester"))),
        Lineage {
            parent: Some("overseer".to_owned()),
            role: None,
        }
    );
}

#[test]
fn run_lineage_an_inherited_name_is_the_parent_when_no_flag() {
    let args = parse(&["builder", "--role", "builder", "--", "claude"]).expect("args");
    assert_eq!(
        lineage_of(&args, Some(named("overseer"))),
        Lineage {
            parent: Some("overseer".to_owned()),
            role: Some("builder".to_owned()),
        }
    );
    assert_eq!(lineage_of(&args, None).parent, None);
}

/// A `viola run builder` typed inside `builder`'s own session: the inherited name is dropped,
/// never refused.
#[test]
fn run_lineage_an_inherited_own_name_is_no_parent() {
    let args = parse(&["builder", "--", "claude"]).expect("args");
    assert_eq!(
        lineage_of(&args, Some(named("builder"))),
        Lineage::default()
    );
}

#[cfg(unix)]
#[test]
fn snapshot_cwd_of_a_start_in_a_directory_that_is_not_utf8_is_absent() {
    use std::os::unix::ffi::OsStrExt as _;
    let tmp = tempfile::tempdir().expect("tempdir");
    let spawn_dir = tmp.path().join(std::ffi::OsStr::from_bytes(b"work-\xff"));
    let snapshot = first_snapshot(&tmp.path().join("instance"), &spawn_dir);
    assert_eq!(snapshot.cwd, None);
}
