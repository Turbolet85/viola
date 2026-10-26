//! Session links over the CLI and the wrapper channel (architecture §Established Decisions,
//! [Links]; test-plan §6 Scenario E5's `link` step): an issued `send` or a taken `answer` from
//! another instance leaves one `link` line `driver -> driven` in the driven instance's own log,
//! directly before the record it rides with; a refused call and a call under the wrapper's own name
//! leave none; the snapshot's `links` mirrors the set, a restarted wrapper restores it from the log,
//! and the log replay reads the same set back. `viola link` / `viola unlink` (test-plan §6 Scenario
//! E3) record and remove a pair through the wrapper's own methods, print the design lines, refuse a
//! stopped or a widened instance with exit 21, and log no name. Every case starts from a stamped
//! home.

#[allow(dead_code)]
mod support;

use std::path::Path;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, viola};
use support::events::{events, wait_events};
use support::home::{StampedHome, Wrapper, snapshot_data, stamped_home, workspace_path};
use support::hygiene::load_schema;
use support::statusline::CANARY;
use viola_state::events::Skipped;
use viola_state::replay::replay;

/// A gated `PostToolUse`, then a gated `Stop`: the turn a confirmed send started, ended on cue.
const PATH3: &str = "fixtures/fake-scripts/path3.json";
/// Gated dialog hooks: the first step raises a question.
const PATH4: &str = "fixtures/fake-scripts/path4.json";

/// A stamped wrapper over the committed hook fixtures (and `script`), its SessionStart landed.
fn boot(stamped: StampedHome, script: Option<&str>) -> Wrapper {
    support::events::boot(stamped, script, &[])
}

/// `viola send builder --json` as `from`, with a text that carries the tests' canary.
fn send(home: &Path, from: Option<&str>) -> Ran {
    let text = format!("{CANARY} to the builder");
    viola(home, &["send", "builder", "--json"], Some(&text), from)
}

fn links(lines: &[Value]) -> Vec<&Value> {
    lines.iter().filter(|l| l["kind"] == "link").collect()
}

/// The one `link` line of `lines` and where it stands: `overseer -> builder`, the wrapper's own.
fn the_link(lines: &[Value]) -> (usize, &Value) {
    let at: Vec<usize> = (0..lines.len())
        .filter(|n| lines[*n]["kind"] == "link")
        .collect();
    assert_eq!(at.len(), 1, "{lines:?}");
    let link = &lines[at[0]];
    assert_eq!(link["source"], "wrapper");
    assert_eq!(link["instance"], "builder");
    assert_eq!(
        link["data"],
        json!({"driver": "overseer", "driven": "builder"})
    );
    (at[0], link)
}

/// The snapshot's `links`.
fn snapshot_links(instance_dir: &Path) -> Value {
    snapshot_data(instance_dir).expect("snapshot")["links"].clone()
}

/// The one item `link`'s line makes.
fn linked_since(link: &Value) -> Value {
    json!([{"driver": "overseer", "driven": "builder", "since": link["ts"]}])
}

/// The offset each complete line of the log starts at.
fn line_starts(instance_dir: &Path) -> Vec<usize> {
    let log = std::fs::read(instance_dir.join("events.ndjson")).expect("the log");
    let ends = log.iter().enumerate().filter(|(_, b)| **b == b'\n');
    std::iter::once(0)
        .chain(ends.map(|(at, _)| at + 1))
        .collect()
}

/// No home-level line carries the canary or a link's own fields, and every role line passes the
/// diag-line schema.
fn assert_logs_clean(home: &Path) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("role file");
        assert!(!text.contains(CANARY), "{path:?} holds content");
        for (n, line) in support::ndjson::read_lines(&path).iter().enumerate() {
            assert!(
                validator.is_valid(line),
                "{path:?} line {n} fails the schema"
            );
            for key in ["driver", "driven", "since"] {
                assert!(line.get(key).is_none(), "{path:?} line {n} holds {key}");
            }
        }
    }
}

/// An issued send from `overseer` links it once: the `link` line stands directly before that
/// send's `send-issued`, whose cursor is still the offset its own line starts at, the snapshot
/// holds the pair since that line's `ts`, and a second issued send adds nothing.
#[rstest]
fn links_a_send_from_another_instance_links_once(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH3));
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    assert_eq!(snapshot_links(&dir), json!([]));

    let sent = send(&home, Some("overseer"));
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let lines = events(&dir);
    let (at, link) = the_link(&lines);
    let issued = &lines[at + 1];
    assert_eq!(issued["kind"], "send-issued");
    assert_eq!(
        issued["data"],
        json!({"cursor": line_starts(&dir)[at + 1], "from": "overseer"})
    );
    assert_eq!(snapshot_links(&dir), linked_since(link));

    wrapper.release();
    wrapper.release();
    wait_events(&dir, "the scripted Stop's turn-ended", |l| {
        l.iter().any(|e| e["kind"] == "turn-ended")
    });
    let again = send(&home, Some("overseer"));
    assert_eq!(again.code, Some(0), "stderr: {}", again.stderr);
    let lines = events(&dir);
    assert_eq!(
        lines.iter().filter(|e| e["kind"] == "send-issued").count(),
        2
    );
    let (_, still) = the_link(&lines);
    assert_eq!(snapshot_links(&dir), linked_since(still));
    assert_logs_clean(&home);
    wrapper.stop();
}

/// A taken answer from `overseer` links it; an answer the wrapper refuses links nothing.
#[rstest]
fn links_an_answer_from_another_instance_links(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH4));
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    wrapper.release();
    let lines = wait_events(&dir, "the question's line", |l| {
        l.iter().any(|e| e["kind"] == "question")
    });
    let asked = lines
        .iter()
        .position(|e| e["kind"] == "question")
        .expect("the question");
    let dialog_id = lines[asked]["data"]["dialog_id"]
        .as_u64()
        .expect("a dialog_id");
    let response = json!({"answers": {"Which shape?": format!("{CANARY} a triangle")}}).to_string();
    let answer = |id: u64| {
        let id = id.to_string();
        let args = ["answer", "builder", id.as_str(), "--json"];
        viola(&home, &args, Some(&response), Some("overseer"))
    };

    let unknown = answer(dialog_id + 7);
    assert_eq!(unknown.code, Some(13), "stderr: {}", unknown.stderr);
    assert_eq!(
        unknown.stdout,
        "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"unknown-dialog\"}\n"
    );
    assert!(links(&events(&dir)).is_empty(), "a refused answer links");
    assert_eq!(snapshot_links(&dir), json!([]));

    let taken = answer(dialog_id);
    assert_eq!(taken.code, Some(0), "stderr: {}", taken.stderr);
    assert_eq!(taken.stdout, "{\"v\":1,\"ok\":{}}\n");
    let lines = events(&dir);
    let (at, link) = the_link(&lines);
    assert!(at > asked, "the link follows the dialog it answers");
    assert_eq!(snapshot_links(&dir), linked_since(link));
    assert_logs_clean(&home);
    wrapper.stop();
}

/// A send refused under a paused wheel links nothing, and neither does an issued send under the
/// wrapper's own name.
#[rstest]
fn links_a_refused_send_and_an_own_name_send_record_none(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();

    let paused = viola(&home, &["pause", "builder", "--json"], None, None);
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    let refused = send(&home, Some("overseer"));
    assert_eq!(refused.code, Some(10), "stderr: {}", refused.stderr);
    assert_eq!(
        refused.stdout,
        "{\"v\":1,\"refusal\":\"human-typing\",\"detail\":\"manual-pause\"}\n"
    );
    assert!(links(&events(&dir)).is_empty(), "a refused send links");

    let released = viola(&home, &["release", "builder", "--json"], Some(""), None);
    assert_eq!(released.code, Some(0), "stderr: {}", released.stderr);
    let own = send(&home, Some("builder"));
    assert_eq!(own.code, Some(0), "stderr: {}", own.stderr);
    let lines = events(&dir);
    let issued: Vec<&Value> = lines
        .iter()
        .filter(|e| e["kind"] == "send-issued")
        .collect();
    assert_eq!(issued.len(), 1, "{issued:?}");
    assert_eq!(issued[0]["data"]["from"], "builder");
    assert!(links(&lines).is_empty(), "the wrapper's own name links");
    assert_eq!(snapshot_links(&dir), json!([]));
    assert_logs_clean(&home);
    wrapper.stop();
}

/// A wrapper started over a log that holds a link restores it: its snapshot holds the same pair
/// with the same `since`, and a further send from that driver appends no second `link`.
#[rstest]
fn links_survive_a_wrapper_restart(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let sent = send(&home, Some("overseer"));
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let linked = snapshot_links(&dir);
    assert_eq!(linked, linked_since(the_link(&events(&dir)).1));
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let first_life = events(&dir).len();

    let second = boot(stamped, None);
    let lines = events(&dir);
    assert_eq!(lines[first_life]["kind"], "wheel");
    assert_eq!(lines[first_life]["data"]["cause"], "start");
    assert_eq!(lines[first_life + 1]["kind"], "budget-gate");
    assert_eq!(snapshot_links(&dir), linked);

    let again = send(&home, Some("overseer"));
    assert_eq!(again.code, Some(0), "stderr: {}", again.stderr);
    let lines = events(&dir);
    assert_eq!(
        lines.iter().filter(|e| e["kind"] == "send-issued").count(),
        2
    );
    the_link(&lines);
    assert_eq!(snapshot_links(&dir), linked);
    assert_logs_clean(&home);
    second.stop();
}

/// test-plan §6 Scenario E5, the `link` step: the replay over the real wrapper's log returns the
/// link the wrapper recorded, equal to the snapshot's, and steps over nothing.
#[rstest]
fn e5_the_replay_reads_the_link_the_wrapper_wrote(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    assert_eq!(replay(&dir).expect("the replay").links, Vec::<Value>::new());

    let sent = send(&home, Some("overseer"));
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let replayed = replay(&dir).expect("the replay");
    assert_eq!(replayed.links.len(), 1);
    assert_eq!(json!(replayed.links), snapshot_links(&dir));
    assert_eq!(
        json!(replayed.links),
        linked_since(the_link(&events(&dir)).1)
    );
    assert_eq!(
        replayed.skipped,
        Skipped {
            unknown_kinds: 0,
            unknown_fields: 0,
            torn_lines: 0,
        }
    );
    assert_logs_clean(&home);
    wrapper.stop();
}

const UNREACHABLE: &str = "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n";

/// `viola <verb> overseer builder [--json]`, with no name of its own.
fn link_verb(home: &Path, verb: &str, json: bool) -> Ran {
    let mut args = vec![verb, "overseer", "builder"];
    if json {
        args.push("--json");
    }
    viola(home, &args, None, None)
}

/// A `link --json` reply's `ok`, exit 0 and stderr empty.
fn linked(home: &Path) -> Value {
    let ran = link_verb(home, "link", true);
    assert_eq!(ran.code, Some(0), "stderr: {}", ran.stderr);
    assert_eq!(ran.stderr, "");
    let doc: Value = serde_json::from_str(&ran.stdout).expect("one document");
    assert_eq!(doc["v"], 1);
    doc["ok"].clone()
}

fn unlinks(lines: &[Value]) -> Vec<&Value> {
    lines.iter().filter(|l| l["kind"] == "unlink").collect()
}

/// `HH:MM:SSZ`: a clock time of whole seconds in UTC.
fn is_clock_time(text: &str) -> bool {
    let b = text.as_bytes();
    b.len() == 9
        && [0, 1, 3, 4, 6, 7].iter().all(|n| b[*n].is_ascii_digit())
        && b[2] == b':'
        && b[5] == b':'
        && b[8] == b'Z'
}

/// test-plan §6 Scenario E3, steps 1 to 3: two links leave one `link` record and one `since`, an
/// issued send from the driver adds none, `unlink` appends the one `unlink` record the wrapper
/// writes and empties the snapshot's `links`, a second `unlink` appends nothing, and a further
/// link is a new pair with a later `since`.
#[rstest]
fn e3_two_links_and_a_send_leave_one_link_record(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();

    let first = linked(&home);
    assert_eq!(first["already_linked"], false);
    let second = linked(&home);
    assert_eq!(second["already_linked"], true);
    assert_eq!(second["since"], first["since"]);
    let sent = send(&home, Some("overseer"));
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let lines = events(&dir);
    let (_, link) = the_link(&lines);
    assert_eq!(link["ts"], first["since"]);
    assert_eq!(snapshot_links(&dir), linked_since(link));

    let unlinked = link_verb(&home, "unlink", true);
    assert_eq!(unlinked.code, Some(0), "stderr: {}", unlinked.stderr);
    assert_eq!(unlinked.stdout, "{\"v\":1,\"ok\":{}}\n");
    assert_eq!(unlinked.stderr, "");
    let lines = events(&dir);
    let removed = unlinks(&lines);
    assert_eq!(removed.len(), 1, "{lines:?}");
    assert_eq!(removed[0]["source"], "wrapper");
    assert_eq!(removed[0]["instance"], "builder");
    assert_eq!(
        removed[0]["data"],
        json!({"driver": "overseer", "driven": "builder"})
    );
    assert_eq!(snapshot_links(&dir), json!([]));

    let again = link_verb(&home, "unlink", true);
    assert_eq!(again.code, Some(0), "stderr: {}", again.stderr);
    assert_eq!(again.stdout, "{\"v\":1,\"ok\":{}}\n");
    assert_eq!(events(&dir), lines, "an absent pair's unlink appends");

    let later = linked(&home);
    assert_eq!(later["already_linked"], false);
    let (later_since, first_since) = (
        later["since"].as_str().expect("since"),
        first["since"].as_str().expect("since"),
    );
    assert!(
        later_since > first_since,
        "{later_since} after {first_since}"
    );
    let lines = events(&dir);
    assert_eq!(links(&lines).len(), 2);
    assert_eq!(
        snapshot_links(&dir),
        json!([{"driver": "overseer", "driven": "builder", "since": later_since}])
    );
    assert_logs_clean(&home);
    wrapper.stop();
}

/// The three result lines of design-system §Surface: cli pattern 4, each one plain line on stdout
/// with stderr empty: a new link with its clock time, a held link, an unlink.
#[rstest]
fn link_verbs_print_the_design_lines(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();

    let new = link_verb(&home, "link", false);
    assert_eq!(new.code, Some(0), "stderr: {}", new.stderr);
    let time = new
        .stdout
        .strip_prefix("link    overseer -> builder  since ")
        .and_then(|rest| rest.strip_suffix('\n'))
        .unwrap_or_else(|| panic!("{:?}", new.stdout));
    assert!(is_clock_time(time), "{time:?}");
    let held = link_verb(&home, "link", false);
    assert_eq!(held.code, Some(0), "stderr: {}", held.stderr);
    assert_eq!(held.stdout, "link    overseer -> builder  already linked\n");
    let unlinked = link_verb(&home, "unlink", false);
    assert_eq!(unlinked.code, Some(0), "stderr: {}", unlinked.stderr);
    assert_eq!(unlinked.stdout, "unlink  overseer -> builder\n");
    for ran in [&new, &held, &unlinked] {
        assert_eq!(ran.stderr, "");
        assert!(ran.stdout.is_ascii() && !ran.stdout.contains('\u{1b}'));
    }
    wrapper.stop();
}

#[test]
fn is_clock_time_reads_the_shape_only() {
    assert!(is_clock_time("14:03:09Z"));
    assert!(!is_clock_time("14:03:09.512Z"));
    assert!(!is_clock_time("14-03-09Z"));
    assert!(!is_clock_time("14:03:0xZ"));
    assert!(!is_clock_time("14:03:09+"));
}

/// A stopped driven instance: both verbs exit 21, the not-running hint last in human mode, the
/// `instance-unreachable` document with stderr empty under `--json`.
#[rstest]
fn link_verbs_against_a_stopped_instance_exit_21(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let home = stamped.home.path();
    for verb in ["link", "unlink"] {
        let human = link_verb(home, verb, false);
        assert_eq!(human.code, Some(21), "{verb}: {}", human.stderr);
        assert_eq!(human.stdout, "", "{verb}");
        let mut lines = human.stderr.lines();
        let unable = lines.next().unwrap_or_default();
        let words: Vec<&str> = unable.split("  ").collect();
        assert_eq!(
            words,
            ["unable", "builder", "instance-unreachable"],
            "{verb}"
        );
        assert_eq!(
            human.stderr.lines().last(),
            Some("hint: builder is not running; viola list shows the live instances"),
            "{verb}: {}",
            human.stderr
        );
        let json = link_verb(home, verb, true);
        assert_eq!(json.code, Some(21), "{verb}: {}", json.stderr);
        assert_eq!(json.stdout, UNREACHABLE, "{verb}");
        assert_eq!(json.stderr, "", "{verb}");
    }
}

/// The wrapper's lines of `method`'s calls: their requests and responses.
fn channel_lines<'a>(lines: &'a [Value], event: &str, method: &str) -> Vec<&'a Value> {
    lines
        .iter()
        .filter(|l| l["event"] == event && l["method"] == method)
        .collect()
}

/// Each call leaves one `channel-request` and one `channel-response` with its method on the
/// wrapper, joined by `(conn, corr)`, and neither holds a name the call carried, `since` or
/// `already_linked`; the verb's own `cli` log holds its `process-start` and a clean `process-exit`.
#[rstest]
fn link_verbs_log_the_channel_pair_and_no_names(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    for verb in ["link", "unlink"] {
        let ran = link_verb(&home, verb, true);
        assert_eq!(ran.code, Some(0), "{verb}: {}", ran.stderr);
    }

    let run_lines =
        support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"));
    for method in ["link", "unlink"] {
        let requests = channel_lines(&run_lines, "channel-request", method);
        let responses = channel_lines(&run_lines, "channel-response", method);
        assert_eq!(requests.len(), 1, "{method}: {run_lines:?}");
        assert_eq!(responses.len(), 1, "{method}: {run_lines:?}");
        let (request, response) = (requests[0], responses[0]);
        assert!(request["conn"].is_string(), "{request:?}");
        assert_eq!(
            (&request["conn"], &request["corr"]),
            (&response["conn"], &response["corr"])
        );
        for line in [request, response] {
            let text = line.to_string();
            for word in ["overseer", "since", "already_linked"] {
                assert!(!text.contains(word), "{method}: {word} in {text}");
            }
        }
    }

    let cli_lines =
        support::ndjson::read_lines(&home.join("diagnostics").join("cli-builder.ndjson"));
    let of =
        |event: &str| -> Vec<&Value> { cli_lines.iter().filter(|l| l["event"] == event).collect() };
    assert_eq!(of("process-start").len(), 2, "{cli_lines:?}");
    let exits = of("process-exit");
    assert_eq!(exits.len(), 2, "{cli_lines:?}");
    assert!(
        exits
            .iter()
            .all(|e| e["exit_code"] == 0 && e.get("detail").is_none())
    );
    assert_logs_clean(&home);
    wrapper.stop();
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).expect("chmod");
}

#[cfg(unix)]
fn mode_of(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::metadata(path).expect("meta").permissions().mode() & 0o777
}

/// The instance check comes before any log opens: with the driven instance's directory writable
/// by its group while its wrapper runs, both verbs in both modes exit 21 with the instance-state
/// pair or the `instance-unreachable` document, the verb's `cli` log gains no line, the wrapper
/// gets no frame, and the directory's mode stays as found.
#[cfg(unix)]
#[rstest]
fn link_verbs_refuse_a_widened_instance_before_any_log(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let cli_log = home.join("diagnostics").join("cli-builder.ndjson");
    let run_log = home.join("diagnostics").join("run-builder.ndjson");
    let requests = || {
        support::ndjson::read_lines(&run_log)
            .into_iter()
            .filter(|l| l["event"] == "channel-request")
            .count()
    };
    let (cli_before, requests_before) = (support::ndjson::read_lines(&cli_log), requests());
    set_mode(&dir, 0o770);

    for verb in ["link", "unlink"] {
        let human = link_verb(&home, verb, false);
        assert_eq!(human.code, Some(21), "{verb}: {}", human.stderr);
        assert_eq!(human.stdout, "", "{verb}");
        assert_eq!(
            human.stderr,
            "unable: builder's state files can be written by another user\n\
             hint: viola will not read them; make the viola home and its files owner-only\n",
            "{verb}"
        );
        let json = link_verb(&home, verb, true);
        assert_eq!(json.code, Some(21), "{verb}: {}", json.stderr);
        assert_eq!(json.stdout, UNREACHABLE, "{verb}");
        assert_eq!(json.stderr, "", "{verb}");
    }

    assert_eq!(mode_of(&dir), 0o770);
    assert_eq!(support::ndjson::read_lines(&cli_log), cli_before);
    assert_eq!(requests(), requests_before, "the wrapper was sent a frame");
    set_mode(&dir, 0o700);
    wrapper.stop();
}

/// A wrapper started over a log whose pair was linked and then unlinked by the methods restores an
/// empty set: its first snapshot's `links` is empty, and the restore appends no line.
#[rstest]
fn links_an_unlink_survives_a_wrapper_restart(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    assert_eq!(linked(&home)["already_linked"], false);
    let unlinked = link_verb(&home, "unlink", true);
    assert_eq!(unlinked.code, Some(0), "stderr: {}", unlinked.stderr);
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let first_life = events(&dir);

    let second = boot(stamped, None);
    assert_eq!(snapshot_links(&dir), json!([]));
    let lines = events(&dir);
    assert_eq!(links(&lines).len(), 1, "{lines:?}");
    assert_eq!(unlinks(&lines).len(), 1, "{lines:?}");
    assert_eq!(&lines[..first_life.len()], &first_life[..]);
    assert_logs_clean(&home);
    second.stop();
}
