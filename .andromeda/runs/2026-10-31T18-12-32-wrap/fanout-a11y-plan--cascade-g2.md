proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'In the `viola revive` clause, replace «under `--json` its strict-modes and preflight refusals give one» with «under `--json` every refusal of the start it shares with `run` gives one», keeping the document form, no hint and empty stderr, and add `tests/cross_exit_causes.rs` beside `revive_json_*` as its witness.'
    sidecar: 'cognitive-accessibility: `viola revive --json` gives one document for every refusal of the start it shares with `run`; the post-preflight gap is closed.'
    rationale: 'Report Changes, Symbols / APIs (`viola revive --json`) and Retired (the `revive --json` post-preflight gap): the batch-script-child, pinned-hash-mismatch and squatted-name refusals now print a document too.'
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'In the frame verbs'' server-verify clause at the line''s end, replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`; the human pair and its `hint:` stand.'
    sidecar: 'cognitive-accessibility: the server-verify refusal''s `--json` document names its cause, `server-verify-failed`.'
    rationale: 'Report Changes, Symbols / APIs (the exit-21 document moved) and Retired (the exit-21 document''s `detail: null`); `Unreachable::ServerUnverified` carries the cause.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **CLI, exit 21 under `--json`:** one document per cause: `error` is `instance-unreachable` and `detail` is one of the closed words `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, with stderr empty and no ESC.`'
    sidecar: 'cognitive-accessibility: new detail, exit 21''s `--json` document per cause and its five closed words.'
    rationale: 'Report Changes, Symbols / APIs (the exit-21 document moved) and Coverage of new surfaces (one document, no hint, no ESC).'
    fact: F1
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 25
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the entity stands.'
    sidecar: 'cognitive-accessibility: the `viola link` / `viola unlink` strict-modes document names its cause, `strict-modes-failed`.'
    rationale: 'Report Counts (`doc-null`: a strict-modes site takes `strict-modes-failed`) and Retired (the exit-21 document''s `detail: null`); the report''s a11y-plan amendment names this line.'
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 31
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the entity stands.'
    sidecar: 'cognitive-accessibility: the frame verbs'' strict-modes document names its cause, `strict-modes-failed`.'
    rationale: 'Report Counts (`doc-null`: a strict-modes site takes `strict-modes-failed`) and Retired (the exit-21 document''s `detail: null`); the report''s a11y-plan amendment names this line.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 31
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **CLI, a lost or stale wrapper:** a wrapper lost with the call open is `endpoint-lost`; a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1''s cause alone.`'
    sidecar: 'cognitive-accessibility: new detail, a wrapper lost mid-call reads `endpoint-lost` and a stale instance reads `instance-dead` on a frame verb.'
    rationale: 'Report Changes, Symbols / APIs (a process-exit `during`; a stale holder''s code).'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 37
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **CLI, a start against a holder:** exit 1 tells a stale holder from a live one: `stale-heartbeat` for a `stale` holder, `already-live` for a live holder and a fresh replayed beat.`'
    sidecar: 'cognitive-accessibility: new detail, exit 1 tells a stale holder (`stale-heartbeat`) from a live one (`already-live`).'
    rationale: 'Report Changes, Symbols / APIs (a stale holder''s code) and Retired (`already-live` as a stale holder''s refusal).'
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 49
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'After «the `cli` catch site''s fault line stands, with no hint» add «; under `--json` a frame verb there prints the `internal-error` document on stdout in that line''s place, stderr empty (`verify` takes no `--json`)».'
    sidecar: 'cognitive-accessibility: the unstattable-home entity''s Windows catch-site line takes the `internal-error` document under `--json`.'
    rationale: 'Report Changes, Symbols / APIs (the `cli` catch site under `--json`; `verify` takes no `--json`) and Counts (`catch-site`: the human line stands byte for byte); New text, by line lists `a_frame_verb_that_cannot_open_its_log_is_internal_error` in `tests/cross_exit_causes.rs`.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 55
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace the sentence with: a driven instance with no live endpoint whose directory stands is refused with exit 21, the not-running `hint:` as the last stderr line; a driven name with no such directory reads `unwrapped-name`, its own `hint:` last.'
    sidecar: 'cognitive-accessibility: the link verbs'' not-running hint holds where the instance directory stands; any other reading is `unwrapped-name`.'
    rationale: 'Report Changes, Symbols / APIs: `src/cmd/link.rs:103-105` reads `Unreachable::without_endpoint(&instance_dir)`, `unlink` through `link_call`, and `hint_cause()` tells `not-running` from `unwrapped-name`; the reading is `inputs#I2` Q2, delegate the overseer, PROVISIONAL.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 55
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **CLI, a name with no endpoint:** the refusal reads the instance directory: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) is `unwrapped-name`, and the strict-modes check stays first (PROVISIONAL, `inputs#I2` Q2).`'
    sidecar: 'cognitive-accessibility: new detail, a no-endpoint refusal takes its cause from the instance directory, PROVISIONAL.'
    rationale: 'Report Changes, Symbols / APIs (`viola_state::liveness::without_endpoint`) and Decisions & corrections (the delegate''s word on `inputs#I2` Q2, PROVISIONAL).'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 55
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **CLI, the `unwrapped-name` and `endpoint-lost` human forms:** each ends with its own `hint:` as the last stderr line, in plain ASCII, the `endpoint-lost` hint being new text (PROVISIONAL, `inputs#I2` Q3) and the name in a hint the parsed `ViolaName`.`'
    sidecar: 'cognitive-accessibility: new detail, the two hints end their human forms in plain ASCII, the `endpoint-lost` text PROVISIONAL.'
    rationale: 'Report Coverage of new surfaces (the two hints), Outcome (`src/human/hints.rs:50-55`) and Decisions & corrections (the delegate''s word on `inputs#I2` Q3, PROVISIONAL).'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 79
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **CLI, a fault under `--json`:** where a CLI verb prints `error: internal error`, under `--json` it prints the `internal-error` document with a null `detail` on stdout in its place and stderr is empty; without `--json` the line is unchanged, and `run`, `revive`''s start, `mcp` and `hook` stay silent there.`'
    sidecar: 'cognitive-accessibility: new detail, the `cli` catch site prints the `internal-error` document under `--json`.'
    rationale: 'Report Changes, Symbols / APIs (the `cli` catch site under `--json`).'
    fact: F6
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 92
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'In the `viola list` clause, replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the line stands.'
    sidecar: 'cognitive-accessibility: the pinned `viola list` exit-21 document names its cause, `strict-modes-failed`.'
    rationale: 'Report Changes, Symbols / APIs (`src/cmd/list.rs` builds `strict-modes-failed`, the board''s only refusal) and Retired (the exit-21 document''s `detail: null`); the report''s a11y-plan amendment names this line.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 105
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace «the exact `instance-unreachable` document» with «the exact `instance-unreachable` document whose `detail` is `strict-modes-failed`»; the rest of the entity stands.'
    sidecar: 'cognitive-accessibility: the frame verbs'' strict-modes oracle names the document''s cause.'
    rationale: 'Report Changes, Symbols / APIs (one document per cause) and Counts (`doc-null`: a strict-modes site takes `strict-modes-failed`); this oracle, once the only exit-21 form, is now one of five, and `tests/security_negatives_strict_modes.rs` is reached.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 117
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **Verification, one hint per code:** no two codes share a hint and no hint names `release`, witnessed by `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release` in `tests/cross_exit_causes.rs`.`'
    sidecar: 'cognitive-accessibility: new detail, no two codes share a hint and no hint names `release`, with their witnesses.'
    rationale: 'Report Outcome (the table green, with `no_two_codes_share_a_hint`, its control, no hint naming `release`) and New text, by line (`tests/cross_exit_causes.rs`).'
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 117
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after this entity: `- **Verification, the exit-cause table:** `tests/cross_exit_causes.rs` is the exit-cause table: its rows check each cause''s exit, document and hint in human and `--json` modes as a `fake-agent` E2E binary, and its top comment names the causes held outside the table.`'
    sidecar: 'cognitive-accessibility: new detail, `tests/cross_exit_causes.rs` as the exit-cause table.'
    rationale: 'Report Changes (Files; Harness / gate surface, `required-features = ["fake-agent"]`, a root `cross_` binary the E2E regex takes) and Counts (`cross-table`, its comment at the file''s top).'
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 133
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Add a sub-item to this Verification: `  - The `unwrapped-name` and `endpoint-lost` hints (`src/human/hints.rs`) join the literal-pinned strings, through `send_hint_is_the_design_string` and the exit-cause table (`tests/cross_exit_causes.rs`).`'
    sidecar: 'cognitive-accessibility: new detail, the `unwrapped-name` and `endpoint-lost` hints join the literal-pinned strings.'
    rationale: 'Report Coverage of new surfaces (the two hints: tests unit + e2e, `send_hint_is_the_design_string`, the table).'
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 273
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'In the list of strings pinned by literal, replace «the server-verify hint» with «the server-verify, `unwrapped-name` and `endpoint-lost` hints»; the rest of the line stands.'
    sidecar: 'a11y-plan §8 Plain language: the broad line''s literal-pinned list gains the `unwrapped-name` and `endpoint-lost` hints.'
    rationale: 'Report Coverage of new surfaces (the two hints, pinned by `send_hint_is_the_design_string` and the table) and Expected amendments (a11y-plan §8, the literal-pinned list).'
note: 'D-a11y-surface: the report adds no interactive element, only static document and hint lines. D-a11y-obs-schema: `process-exit` `detail` grew 13 to 19 in `schemas/diag-line.v1.json:78`, but a11y-plan''s body routes a cli or tui failure to the harness `run` JSON as a nextest failure, not to the diagnostics log, so this read shows no divergence; §3''s contracts were outside it. F7, F8, F9 and `a11y-plan.md:31` belong to a11y-scope-summary, not this group.'
