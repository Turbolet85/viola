@task_prose part=session
The session was test-only. It added two unit tests to viola-hyprland, the crate that holds viola's one Hyprland seam. Together they pin the seam's two MAX_FRAME size bounds at the exact frame.
The hyprctl read test runs the crate's own planted hyprctl stub. A reply of exactly MAX_FRAME bytes is read, and one byte more is refused as oversize.
The dispatch parse test feeds exactly MAX_FRAME bytes, which pass the size check and are then refused. One byte more is refused as oversize.
No product line changed, no caller changed, and no public API, dependency, schema, config, error word or output text was added or retired.
Two remove-the-guard controls showed that each new test fails when its bound is removed. The crate's test count went from 82 to 84. The gates and CI were green.
@task_file n=1 path=crates/viola-hyprland/src/read.rs
what: the Hyprland seam's hyprctl reader, the bounded read that drains a hyprctl reply and refuses one past MAX_FRAME as oversize; only its unit tests changed
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the Hyprland seam's reply parser, the bounded parse of hyprctl clients and dispatch replies that refuses one past MAX_FRAME as oversize; only its unit tests changed
@task_item n=1 part=touches
Indirect: the rule that the mutation audit covers invariants held in crate tests. The two new crate-level tests kill the remove-the-bound mutants of the seam's two size checks. They run inside the crate's own tests, not the root package's tests.
@task_item n=2 part=touches
Indirect: the rule that internal crates are not assertable. The Hyprland crate gained unit tests of its own bounds, but no a11y assertion. A row stays true unless it says an internal crate holds no tests at all.
@task_item n=3 part=touches
Indirect: the rule that a11y assertions live inside the existing suites and that a11y failures are nextest failures. The new tests run in the crate's existing nextest suite. No new suite or driver was added.
@task_item n=4 part=touches
Indirect: the rules on zero flakiness, no retries, and a CI test gate with no skips. The read test spawns a planted hyprctl stub within its deadline, and the gates were green with 0 skipped.
@task_item n=5 part=touches
Indirect: any row that holds a bounded reader's size limit or its oversize refusal word for the hyprctl reply. The bound value and the word oversize are unchanged and are now pinned at the exact frame.
@task_item n=1 part=untouched
CLI output discipline: control-character escaping, SGR and colour cues, the word beside every SGR span, and board captions and truncation. No output text or byte sent to the terminal changed.
@task_item n=2 part=untouched
CLI refusals, exit codes, hints, --json failure documents and fault bodies. No refusal, code word, hint or exit code was added or moved.
@task_item n=3 part=untouched
The viola run TUI passthrough contracts: human keystrokes, the wheel, focus, mouse and resize sequences, vendor content, and run diagnostics.
@task_item n=4 part=untouched
Screen-reader targets and setups, the manual keyboard and Orca passes, and the cognitive-accessibility time bounds. No human-facing timing or reading path changed.
@task_item n=5 part=untouched
The absences (touch surface, web or graphical frontend, i18n, user time limit, rendered page) and the N/A harness keys. No capability was added or removed.
@task_item n=6 part=untouched
Config.json keys, the MCP tool contracts, the hook, link, verify, wait, revive and list contracts. None was reached.
