proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 27
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New `values · cognitive-errors` group after the link / unlink strict-modes group: "- **CLI, `viola send` / `wait` / `last` / `answer` / `pause` / `release`, strict-modes (chunk 2026-10-26-home-and-code-bearing-file-integrity):** an instance that fails the strict-modes check is refused with exit 21 before anything under the home is opened and before any frame, no log line written, the same fixed instance-state `unable` / `hint:` pair `viola link` / `viola unlink` print on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint."'
    sidecar: 'cognitive-accessibility Error recovery: the six frame verbs join link / unlink in refusing a strict-modes-failing instance with exit 21 and the instance pair, hint last (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes, Symbols: `client::refuse_untrusted` called first by send, wait, last, answer, pause, release (refused under `--json` gives the `instance-unreachable` document, else `human::refuse` of `human::instance_state_pair(name)`, `Some(21)`, opens nothing, writes no line); Counts 8 (pattern instance-pair, the six frame verbs join the list) and 20; Coverage of new surfaces (a11y the instance pair, hint last).'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 39
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New `values · cognitive-errors` group after the link / unlink wrapper-refusal group: "- **CLI, `viola run`, the start of `viola revive` and `viola verify`, home tree strict-modes (chunk 2026-10-26-home-and-code-bearing-file-integrity):** a home tree that fails the strict-modes check is refused with exit 1 before any log opens, no log line written and no child started (`verify`: no step line and no summary): the fixed home-state `unable` / `hint:` pair on stderr, `hint:` last, or, for `run` and the start of `revive` when the refused path is the instance, the instance-state pair; `viola revive --json` keeps its `strict-modes-failed` document."'
    sidecar: 'cognitive-accessibility Error recovery: run, the revive start and verify refuse a strict-modes-failing home tree with exit 1 and the home or instance pair, hint last, before any log (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes, Symbols: run.rs `checked_exe` / `refuse_tree` (HomeLevel to `human::home_state_pair()`, InstanceLevel to `human::instance_state_pair(name)`, through `human::refuse`), called by `run` and the revive start before `open_wrapper_log` (exit 1, no log, no line, no child; revive `--json` keeps `strict-modes-failed`); verify.rs `check_home_tree` then `refuse_home()` (exit 1, no step line, no summary, no line), and `StampsError::StrictModes` mapped to the same pair; Count 9 (pattern home-pair).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 53
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New `procedure · cognitive-errors` group after the link / unlink verification group: "- **Verification, strict-modes refusals (chunk 2026-10-26-home-and-code-bearing-file-integrity):** `tests/security_negatives_strict_modes.rs` reads the six frame verbs refusing a widened instance (exit 21, the instance pair with `hint:` last, or under `--json` the `instance-unreachable` document; no frame, no line, the mode or ACL read back as found) and `viola run` refusing a widened or read-granted home before its log (exit 1), on all three OS legs; `tests/run_cli.rs`, `tests/cli_verify.rs` and `tests/cli_controls_not_disableable.rs` read the `run` / `verify` home-tree refusals (green on ci#40403389629)."'
    sidecar: 'cognitive-accessibility Error recovery verification: the strict-modes refusals witnessed by security_negatives_strict_modes on three legs, with run_cli, cli_verify and cli_controls_not_disableable (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Coverage of new surfaces (refuse_untrusted tests integ `binary(security_negatives_strict_modes)` three OSes, a11y the instance pair hint last; run / verify / revive tree check tests `run_cli`, `cli_verify`, `cli_controls_not_disableable`, `security_negatives_strict_modes`); Outcome acceptance rows (six verbs exit 21, mode or ACL as found, three OSes; run exit 1 before any log; ci#40403389629 green 15/15); Expected amendments a11y-plan §8, the strict-modes witnesses on three legs.'
note: 'No D-a11y-surface or D-a11y-obs-schema drift: the chunk adds no interactive element and changes no a11y violation schema (the diag-line `sideload_fallback` word is obs-only); no Retired item stands in this file, and line 45 (macOS server-verify cannot occur) is a different fact, no-change per report Count 11.'
