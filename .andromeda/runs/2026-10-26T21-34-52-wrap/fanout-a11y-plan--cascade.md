proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'After line 8 (already over 2 000 chars, so the addition is its own sub-bullet), add `  - **Strict-modes refusals (chunk 2026-10-26-home-and-code-bearing-file-integrity):** `send`, `wait`, `last`, `answer`, `pause` and `release` refuse, before any frame, an instance that fails the strict-modes check with exit 21 and no line: the instance-state `unable` / `hint:` pair `link` prints, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, on all three OS legs; `viola verify` refuses a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the check with the home-state pair, exit 1, before any `[NN/NN]` step line or summary; `viola run` and `revive`''s start refuse a failing home tree before any log or child with the home-state pair (home level) or the instance-state pair (instance level), exit 1, as printed CLI text on stderr, the TUI never entered (`revive --json` keeps `strict-modes-failed`); verbs, `--json` codes and the server-verify refusal are unchanged.`'
    sidecar: 'a11y-scope-summary: the CLI-verbs entity gains the strict-modes refusals of the six frame verbs (exit 21, instance pair / instance-unreachable), verify (exit 1, home pair) and run / revive start (exit 1, home or instance pair).'
    rationale: 'Report Changes, Symbols: client.rs refuse_untrusted (20-41) called first by send, wait, last, answer, pause, release (exit 21, instance_state_pair or the instance-unreachable document); verify.rs 64-70 / refuse_home 95-99 (home pair, exit 1, no step line); run.rs checked_exe / refuse_tree (189-215) before open_wrapper_log in run (166-172) and revive start (revive.rs 125-135), exit 1, no child; Outcome: three CI legs green (ci#40403389629).'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 25
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'After line 25, add a sibling entity in its shape: `- **CLI, `send` / `wait` / `last` / `answer` / `pause` / `release`, strict-modes:** an instance that fails the strict-modes check is refused before any frame with exit 21 and no log line written: the same fixed instance-state `unable` / `hint:` pair as `viola link` / `viola unlink`, `hint:` last on stderr; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint; witnessed on all three OS legs.`'
    sidecar: 'cognitive-accessibility Error recovery: new entity for the six frame verbs'' strict-modes refusal (exit 21, the link instance pair, hint last; --json instance-unreachable, no hint), three OS legs.'
    rationale: 'Report Symbols: client.rs refuse_untrusted (20-41) runs check_instance first in send (190-192), wait (43-45), last (30-32), answer (94-96), pause (32-34), release (35-37) and prints human::instance_state_pair or refused_document("instance-unreachable"), Some(21), no line; Coverage: binary(security_negatives_strict_modes) three OSes; founder P4 Q3 (inputs#I3) gave the six verbs link''s instance pair.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'After line 19, add `  - **CLI, `viola run` / `viola revive` start, strict-modes:** a home tree that fails the strict-modes check is refused before any log opens, exit 1, no child started: the fixed home-state `unable` / `hint:` pair for a home-level refusal, the instance-state pair for an instance-level one, `hint:` last on stderr; `revive --json` keeps its `{"v":1,"error":"strict-modes-failed","detail":null}` document.`'
    sidecar: 'cognitive-accessibility Error recovery: run and revive start refuse a failing home tree before any log, exit 1, home pair or instance pair, hint last; revive --json keeps strict-modes-failed.'
    rationale: 'Report Symbols: run.rs checked_exe (189-200) and refuse_tree (202-215: HomeLevel to home_state_pair, InstanceLevel to instance_state_pair, through human::refuse), called by run (166-172) and revive start (revive.rs 125-135) before open_wrapper_log: exit 1, no log, no line, no child; revive --json keeps strict-modes-failed.'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'After line 19 (beside `viola verify`''s interrupted pair), add `  - **CLI, `viola verify`, strict-modes:** a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the strict-modes check is refused with the fixed home-state `unable` / `hint:` pair, `hint:` last, exit 1, before any `[NN/NN]` step line or summary and with no line written; a `ledger/` refused by `update_stamps`''s own check gives the same pair.`'
    sidecar: 'cognitive-accessibility Error recovery: verify refuses a failing home tree with the home pair, exit 1, before any step line or summary.'
    rationale: 'Report Symbols: verify.rs check_home_tree(&home, None, None) (64-70) before its cli log; refused, refuse_home (95-99, home_state_pair through human::refuse), exit 1, no step line, no summary, no line; measure maps StampsError::StrictModes to the same pair (288-296).'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'After line 19 (beside the faults-carry-no-hint sentence), add `  - **CLI, an unstattable home:** a home path under a regular file is a strict-modes refusal with a hint on Unix (`verify`: the home-state pair, exit 1; the frame verbs: the instance-state pair, exit 21); on Windows such a path stats `NotFound`, so the catch-site fault, with no hint, stands there; the tests read it through a per-OS oracle.`'
    sidecar: 'cognitive-accessibility Error recovery: an unstattable home is a hinted strict-modes refusal on Unix; on Windows the hint-free catch-site fault stands.'
    rationale: 'Report Counts 13 and Deviation 8: a home under a regular file is now the home pair on verify (verify_under_a_regular_file_refuses_the_home, per-OS oracle after CI: the catch-site line on Windows, where such a path stats NotFound); Decisions: measured on Windows a path under a regular file stats NotFound.'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 44
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'After line 44, add `  - **cli, strict-modes:** the six frame verbs'' exit-21 instance-state pair (hint last, stdout empty) and their `--json` `instance-unreachable` document (stderr empty) are pinned by the `INSTANCE_PAIR` literal in `frame_verb_refuses_a_widened_instance_before_any_log` (`tests/security_negatives_strict_modes.rs`, in both modes); `run`''s exit-1 home- and instance-state pairs by the `HOME_PAIR` / `INSTANCE_PAIR` literals in `run_refuses_a_widened_home_tree_before_its_log` (`tests/run_cli.rs`); `verify`''s home-state pair by `verify_refuses_a_widened_ledger_before_any_step` and, per-OS, `verify_under_a_regular_file_refuses_the_home` (`tests/cli_verify.rs`); unlike the server-verify cases, `tests/security_negatives_strict_modes.rs` runs on all three OS legs.`'
    sidecar: 'cognitive-accessibility Error recovery verification: the strict-modes pairs of the six frame verbs, run and verify, witnessed on all three OS legs, are named beside the two-leg server-verify cases.'
    rationale: 'Report Expected amendments: a11y-plan §8, the strict-modes witnesses on three legs; New text by line: tests/security_negatives_strict_modes.rs 30-37 (INSTANCE_PAIR, HOME_PAIR), 70-131 frame_verb_refuses_a_widened_instance_before_any_log (93-109 for json in [false, true]); tests/run_cli.rs 599-606, 619-674; tests/cli_verify.rs 251-280; Coverage: security_negatives_strict_modes three OSes; Outcome: ci#40403389629 green 15/15.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:44
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 60
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Extend line 60''s pinned-string list: after the server-verify hint, add `and the instance-state and home-state pairs that the six frame verbs, `run`, `revive`''s start and `verify` now print, pinned by the `INSTANCE_PAIR` / `HOME_PAIR` literals in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs`` before the closing sentence.'
    sidecar: 'cognitive-accessibility Plain language verification: the pinned-string list names the instance and home state pairs and their INSTANCE_PAIR / HOME_PAIR literals.'
    rationale: 'Report Symbols: refuse_untrusted prints human::instance_state_pair (six verbs), refuse_tree prints home_state_pair / instance_state_pair (run, revive start), refuse_home prints home_state_pair (verify); New text by line: INSTANCE_PAIR / HOME_PAIR consts in tests/security_negatives_strict_modes.rs (30-37) and tests/run_cli.rs (599-606).'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:60
note: the body's §8 Error recovery Verification parenthetical names only the server-verify refusal as two-leg and still holds; its Plain language Verification broad line lists the literal-pinned strings and may name the state pairs if the line-60 add lands (body line numbers not taken, so no body proposal written).
