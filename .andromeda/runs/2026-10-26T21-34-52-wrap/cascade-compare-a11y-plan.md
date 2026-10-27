# Cascade compare — a11y-plan · wrap · 2026-10-26-home-and-code-bearing-file-integrity
E items 6 · held 1 · lost at classify 2 · lost at strong 0 · beside a placed fact 2 · cascade only 4 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### a11y-scope-summary#2.1 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 20
    section: '## 1. A11y Scope Summary'
    change: 'after line 20 add one `<!-- detail: values · scope-entities -->` block holding: "- **Entity:** `viola send`, `wait`, `last`, `answer`, `pause` and `release` against an instance that fails the strict-modes check (chunk 2026-10-26-home-and-code-bearing-file-integrity), in the human TTY, plain and `--json` modes: each refuses before any frame or log (`send` after its text read) with exit 21, stdout empty and `link`''s instance pair on stderr, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; no line is written and the mode or ACL is left as found, on all three OSes (the refusal''s text: Cognitive Accessibility › Error recovery)" / "  - **Source:** `src/cmd/client.rs` (`refuse_untrusted`), `src/cmd/{send,wait,last,answer,pause,release}.rs`; `tests/security_negatives_strict_modes.rs` (`frame_verb_refuses_a_widened_instance_before_any_log`)." / "  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal."'
    sidecar: 'a11y-scope-summary: new scope entity — the six frame verbs'' strict-modes refusal (exit 21, the instance pair, hint last; the instance-unreachable document under --json), chunk 2026-10-26-home-and-code-bearing-file-integrity'
    rationale: 'report Changes › Symbols (`src/cmd/client.rs` `refuse_untrusted`, called first by the six verbs), Counts 8 (the six frame verbs print link''s instance pair) and Coverage of new surfaces (a11y the instance pair, `hint:` last ✓; tests three OSes); the file states the strict-modes refusal for link/unlink only (line 16) and the server-verify refusal for the frame verbs (line 8), so the new refusal surface has no scope record'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:16

### a11y-scope-summary#2.1 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 20
    section: '## 1. A11y Scope Summary'
    change: 'after the block above add one `<!-- detail: values · scope-entities -->` block holding: "- **Entity:** `viola run`, a `viola revive` start and `viola verify` over a home tree that fails the strict-modes check (chunk 2026-10-26-home-and-code-bearing-file-integrity): each refuses before any log opens with exit 1 and the home pair on stderr, `hint:` last, and starts no child — `run` and revive''s start print the instance pair for an instance-level refusal, revive under `--json` keeps `strict-modes-failed`; `verify` prints no step line and no summary, and its `update_stamps` refusal maps to the same home pair; on Unix a home under a regular file is this refusal, on Windows (such a path stats `NotFound`) the catch-site line stands (the refusals'' text: Cognitive Accessibility › Error recovery)" / "  - **Source:** `src/cmd/run.rs` (`checked_exe`, `refuse_tree`), `src/cmd/revive.rs`, `src/cmd/verify.rs` (`refuse_home`); `tests/run_cli.rs` (`run_refuses_a_widened_home_tree_before_its_log`), `tests/cli_verify.rs` (`verify_refuses_a_widened_ledger_before_any_step`, `verify_under_a_regular_file_refuses_the_home`), `tests/security_negatives_strict_modes.rs` (`a_home_another_user_may_read_refuses_run`)." / "  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal."'
    sidecar: 'a11y-scope-summary: new scope entity — run / revive start / verify refuse a widened home tree (exit 1, the home or instance pair, hint last, no log, no step line), chunk 2026-10-26-home-and-code-bearing-file-integrity'
    rationale: 'report Changes › Symbols (`run.rs` `checked_exe` / `refuse_tree`, revive.rs 125-135, verify.rs 64-70 / `refuse_home`, `StampsError::StrictModes` → home pair), Counts 9 and 13, Coverage of new surfaces (a11y home / instance pair ✓); the file names no refusal of these verbs on a widened home tree'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8

### a11y-scope-summary#4.1 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 41
    section: '## 1. A11y Scope Summary'
    change: 'after line 41 add one `<!-- detail: values · scope-entities -->` block holding: "- **Entity:** Claude Code plugin against an instance that fails the strict-modes check (chunk 2026-10-26-home-and-code-bearing-file-integrity): the `viola hook` event and dialog calls and the stdio MCP server''s frame tools (`send`, `wait`, `last`, `answer`)" / "  - **Source:** `src/cmd/hook.rs` (`canonical`, `trusted`) and `crates/viola-mcp/src/tools.rs` (`frame`); `tests/hook_fail_open.rs` (`hook_refused_by_the_instance_check_fails_open_and_writes_nothing`) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_refuse_a_widened_instance_before_any_frame`)." / "  - **Assertability:** not-assertable" / "  - **Reason:** a refused hook call returns exit 0 at once with empty stdout and stderr under 1.0 s — no `config.json` read, no stdin read, no frame, no line, no SessionEnd append — and a `VIOLA_DIR` naming no existing instance directory writes nothing; an MCP frame tool returns `isError` with the code `instance-unreachable` only (`detail: null`), no frame, no line. Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched."'
    sidecar: 'a11y-scope-summary: new not-assertable entity — the plugin (hook event/dialog, MCP frame tools) against an instance refused by the strict-modes check fails open / returns instance-unreachable with no human-facing byte, chunk 2026-10-26-home-and-code-bearing-file-integrity'
    rationale: 'report Changes › Symbols (`src/cmd/hook.rs` `canonical`/`trusted` before obs init; `viola-mcp` `tools.rs` `frame` 337-339 `check_instance` → `instance-unreachable`), Outcome (hook exit 0, empty streams under 1.0 s, no frame, no line, no SessionEnd), Count 23 (a `VIOLA_DIR` naming no directory writes nothing); the file records only the impostor-server form of this entity (line 36)'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:36

### cognitive-accessibility#5.1 · lost at strong · beside a placed fact F2 (near 19, cognitive-accessibility#2.1) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 39
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New `values · cognitive-errors` group after the link / unlink wrapper-refusal group: "- **CLI, `viola run`, the start of `viola revive` and `viola verify`, home tree strict-modes (chunk 2026-10-26-home-and-code-bearing-file-integrity):** a home tree that fails the strict-modes check is refused with exit 1 before any log opens, no log line written and no child started (`verify`: no step line and no summary): the fixed home-state `unable` / `hint:` pair on stderr, `hint:` last, or, for `run` and the start of `revive` when the refused path is the instance, the instance-state pair; `viola revive --json` keeps its `strict-modes-failed` document."'
    sidecar: 'cognitive-accessibility Error recovery: run, the revive start and verify refuse a strict-modes-failing home tree with exit 1 and the home or instance pair, hint last, before any log (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes, Symbols: run.rs `checked_exe` / `refuse_tree` (HomeLevel to `human::home_state_pair()`, InstanceLevel to `human::instance_state_pair(name)`, through `human::refuse`), called by `run` and the revive start before `open_wrapper_log` (exit 1, no log, no line, no child; revive `--json` keeps `strict-modes-failed`); verify.rs `check_home_tree` then `refuse_home()` (exit 1, no step line, no summary, no line), and `StampsError::StrictModes` mapped to the same pair; Count 9 (pattern home-pair).'

### cognitive-accessibility#7.1 · lost at classify · beside a placed fact F3 (near 19, cognitive-accessibility#2.1) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:53
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 53
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New `procedure · cognitive-errors` group after the link / unlink verification group: "- **Verification, strict-modes refusals (chunk 2026-10-26-home-and-code-bearing-file-integrity):** `tests/security_negatives_strict_modes.rs` reads the six frame verbs refusing a widened instance (exit 21, the instance pair with `hint:` last, or under `--json` the `instance-unreachable` document; no frame, no line, the mode or ACL read back as found) and `viola run` refusing a widened or read-granted home before its log (exit 1), on all three OS legs; `tests/run_cli.rs`, `tests/cli_verify.rs` and `tests/cli_controls_not_disableable.rs` read the `run` / `verify` home-tree refusals (green on ci#40403389629)."'
    sidecar: 'cognitive-accessibility Error recovery verification: the strict-modes refusals witnessed by security_negatives_strict_modes on three legs, with run_cli, cli_verify and cli_controls_not_disableable (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Coverage of new surfaces (refuse_untrusted tests integ `binary(security_negatives_strict_modes)` three OSes, a11y the instance pair hint last; run / verify / revive tree check tests `run_cli`, `cli_verify`, `cli_controls_not_disableable`, `security_negatives_strict_modes`); Outcome acceptance rows (six verbs exit 21, mode or ACL as found, three OSes; run exit 1 before any log; ci#40403389629 green 15/15); Expected amendments a11y-plan §8, the strict-modes witnesses on three legs.'

## Held

- cognitive-accessibility#3.1

## Cascade only

- a11y-scope-summary#1.1
- cognitive-accessibility#2.1
- cognitive-accessibility#6.1
- cognitive-accessibility#8.1
