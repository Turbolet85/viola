# Apply — a11y-plan

## Verdicts
- fanout-a11y-plan--cascade.md 2 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:27+#1
- fanout-a11y-plan--cascade.md 3 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:27+#2
- fanout-a11y-plan--cascade.md 4 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:27+#3
- fanout-a11y-plan--cascade.md 5 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:27+#4
- fanout-a11y-plan--cascade.md 6 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:53+#1
- fanout-a11y-plan--cascade.md 7 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:60#1
- cascade-compare-a11y-plan.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:20+#1
- cascade-compare-a11y-plan.md 2 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:20+#2
- cascade-compare-a11y-plan.md 3 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:41+#1

## Edits
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:27
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, `send` / `wait` / `last` / `answer` / `pause` / `release`, strict-modes:** an instance that fails the strict-modes check is refused with exit 21 before any frame or log (`send` after its text read), no log line written: the same fixed instance-state `unable` / `hint:` pair as `viola link` / `viola unlink` on stderr, `hint:` last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:27
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, `viola run` / `viola revive` start, strict-modes:** a home tree that fails the strict-modes check (the home, the instance's files, `ledger/`, `bin/`, `plugin/`, the pinned copy and its plugin folders) is refused with exit 1 before any log opens, no line written and no child started: the fixed home-state `unable` / `hint:` pair for a home-level refusal, the instance-state pair for an instance-level one, `hint:` last on stderr, stdout empty; under `--json`, `revive` gives its `strict-modes-failed` document (above).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:27
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, `viola verify`, strict-modes:** a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the strict-modes check is refused with exit 1 before its log opens, with no step line, no summary and no line written: the fixed home-state `unable` / `hint:` pair on stderr, `hint:` last; a `ledger/` that `update_stamps`'s own check refuses gives the same pair.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:27
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, an unstattable home:** a home path under a regular file is a strict-modes refusal with its hint on Unix (`verify`: the home-state pair, exit 1; the six frame verbs: the instance-state pair, exit 21); on Windows such a path stats `NotFound`, the check skips it as absent, and the `cli` catch site's fault line stands, with no hint.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:53
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, strict-modes refusals:** `frame_verb_refuses_a_widened_instance_before_any_log` (`tests/security_negatives_strict_modes.rs`) reads, over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes against a live wrapper whose instance directory another user may write, exit 21 with the `INSTANCE_PAIR` literal on stderr and stdout empty, or under `--json` the exact `instance-unreachable` document with stderr empty, no line added and no frame sent, the widening left as found; `run_refuses_a_widened_home_tree_before_its_log` (`tests/run_cli.rs`) reads `run`'s exit 1 with the `HOME_PAIR` or `INSTANCE_PAIR` literal, stdout empty, no line and no child; `verify_refuses_a_widened_ledger_before_any_step` and `verify_under_a_regular_file_refuses_the_home` (`tests/cli_verify.rs`, the latter's oracle per OS: the home-state pair on Unix, the catch-site line on Windows) read `verify`'s home-state pair with no step line or summary. Unlike the server-verify cases, these run on all three OS legs.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:60
  old: «`tests/security_negatives_server_identity.rs`. Any copy change»
  new: «`tests/security_negatives_server_identity.rs`, and the instance-state and home-state pairs as the six frame verbs and `run` print them, pinned by the `INSTANCE_PAIR` / `HOME_PAIR` literals in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs` (`verify`'s home-state pair by literal in `tests/cli_verify.rs`). Any copy change»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:20
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** `viola send`, `wait`, `last`, `answer`, `pause` and `release` against an instance that fails the strict-modes check, in the human TTY, plain and `--json` modes: each refuses before any frame or log (`send` after its text read) with exit 21, stdout empty and `link`'s instance-state pair on stderr, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; no line is written and the mode or ACL is left as found, on all three OSes (the refusal's text: Cognitive Accessibility › Error recovery)»
  add: «  - **Source:** `src/cmd/client.rs` (`refuse_untrusted`), `src/cmd/{send,wait,last,answer,pause,release}.rs`; `tests/security_negatives_strict_modes.rs` (`frame_verb_refuses_a_widened_instance_before_any_log`).»
  add: «  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:20
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** `viola run`, a `viola revive` start and `viola verify` over a home tree that fails the strict-modes check: each refuses before any log opens with exit 1, stdout empty and the home-state pair on stderr, `hint:` last, and starts no child — `run` and revive's start print the instance-state pair for an instance-level refusal, and revive under `--json` gives its `strict-modes-failed` document (the CLI entity above); `verify` prints no step line and no summary, and its `update_stamps` refusal gives the same home-state pair; on Unix a home under a regular file is this refusal, on Windows (such a path stats `NotFound`) the `cli` catch-site fault line stands (the refusals' text: Cognitive Accessibility › Error recovery)»
  add: «  - **Source:** `src/cmd/run.rs` (`checked_exe`, `refuse_tree`), `src/cmd/revive.rs`, `src/cmd/verify.rs` (`refuse_home`); `tests/run_cli.rs` (`run_refuses_a_widened_home_tree_before_its_log`), `tests/cli_verify.rs` (`verify_refuses_a_widened_ledger_before_any_step`, `verify_under_a_regular_file_refuses_the_home`), `tests/security_negatives_strict_modes.rs` (`a_home_another_user_may_read_refuses_run`).»
  add: «  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:41
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** Claude Code plugin against an instance that fails the strict-modes check: the `viola hook` event and dialog calls and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`)»
  add: «  - **Source:** `src/cmd/hook.rs` (`canonical`, `trusted`) and `crates/viola-mcp/src/tools.rs` (`frame`); `tests/hook_fail_open.rs` (`hook_refused_by_the_instance_check_fails_open_and_writes_nothing`) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).»
  add: «  - **Assertability:** not-assertable»
  add: «  - **Reason:** a refused hook event or dialog call returns exit 0 at once with empty stdout and stderr, under 1.0 s — no `config.json` read, no stdin read, no frame, no line, no SessionEnd append — and a `VIOLA_DIR` naming no existing instance directory writes nothing; an MCP frame tool returns `isError` with the code `instance-unreachable` only (`detail: null`) and sends no frame and writes no line. Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
(none)
