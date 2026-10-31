proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'In the server-verify clause that ends the record, the document reads `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, so the clause ends: under `--json` the `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` document with stderr empty and no hint.'
    sidecar: 'Error recovery, server-verify refusal: the exit-21 document names its cause, `detail` `server-verify-failed` in place of null.'
    rationale: 'Report **Retired**: the exit-21 document with `detail: null`. **Symbols / APIs**: the document now reads `detail` `<cause>`, `v` still 1, five closed causes.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 25
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'The `--json` clause reads: under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout with stderr empty and no hint.'
    sidecar: 'Error recovery, `viola link` / `viola unlink` strict-modes: the exit-21 document names `strict-modes-failed`.'
    rationale: 'Report **Retired**: the exit-21 document with `detail: null`. **Counts**, `doc-null`: a strict-modes site takes `strict-modes-failed`; `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:25
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 31
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'The `--json` clause reads: under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout with stderr empty and no hint.'
    sidecar: 'Error recovery, the six frame verbs under strict-modes: the exit-21 document names `strict-modes-failed`.'
    rationale: 'Report **Retired**: the exit-21 document with `detail: null`. **Counts**, `doc-null`: a strict-modes site takes `strict-modes-failed`.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:31
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 92
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'In the `viola list` clause, the pinned document reads: under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.'
    sidecar: 'Error recovery, verification: the document the `viola list` cases pin names `strict-modes-failed`.'
    rationale: 'Report **Retired**: the exit-21 document with `detail: null`. **Symbols / APIs**: `src/cmd/list.rs` `refused_document` builds `strict-modes-failed`, the only refusal of the board.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:92
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'The `viola revive` `--json` clause reads: under `--json` every refusal of the start it shares with `run` gives one `{"v":1,"error":"<code>","detail":null}` document with no hint and empty stderr (`revive_json_*` in `tests/cli_revive.rs`, and `tests/cross_exit_causes.rs`; chunks 2026-10-17-cli-machine-contract and 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Error recovery, `viola revive --json`: the document covers every refusal of the shared start, not its strict-modes and preflight refusals alone.'
    rationale: 'Report **Retired**: the `revive --json` post-preflight gap. **Symbols / APIs**: `viola revive --json` prints one exit-1 document for every refusal of the start it shares with `run`. `detail` stays null on exit 1.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 49
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'The Windows clause ends: and the `cli` catch site''s fault line stands, with no hint; a frame verb under `--json` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of that line, stderr empty and no hint.'
    sidecar: 'Error recovery, an unstattable home on Windows: under `--json` a frame verb prints the `internal-error` document in place of the fault line.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`; **Counts**, `catch-site`: a site stating the output under `--json` takes the document; **Expected amendments** names this file for a11y-plan. `verify` takes no `--json`, so its line stands alone.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:49
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 55
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'The record reads: **CLI, `viola link` / `viola unlink`, no live endpoint:** a driven instance with no live endpoint is refused with exit 21. A stopped one, its instance directory standing (`instance-dead`), takes the not-running `hint:` as the last stderr line; a name whose instance path is absent, a regular file or a symlink (`unwrapped-name`; the reading by directory PROVISIONAL, `inputs#I2` Q2, delegate the overseer) takes the unwrapped-name `hint:` there.'
    sidecar: 'Error recovery, `viola link` / `viola unlink`: no live endpoint reads by the instance directory, the not-running hint or the unwrapped-name hint (PROVISIONAL).'
    rationale: 'Report **Symbols / APIs**: the no-endpoint arm of `src/cmd/link.rs` reads `Unreachable::without_endpoint`, a directory `InstanceDead`, anything else `UnwrappedName`; `hint_cause` gives each its own hint. **Decisions**: `inputs#I2` Q2 is carried PROVISIONAL.'
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:55
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 33
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New record, a `values · cognitive-errors` block: - **CLI, the five causes of exit 21:** each exit-21 refusal names its cause in `detail`, with `error` still `instance-unreachable`, `v` still 1 and no exit moved: `strict-modes-failed` (the check, first at every caller), `instance-dead` (the instance directory stands with no live endpoint, a stale wrapper too, or the connect fails), `unwrapped-name` (no instance directory), `endpoint-lost` (the wrapper lost with the call open), `server-verify-failed` (the serving process is not the recorded wrapper). Under `--json` that is one document with stderr empty, no hint and no ESC.'
    sidecar: 'Error recovery: new record, the five closed causes of exit 21 in the `--json` document.'
    rationale: 'Report **Symbols / APIs**: the exit-21 document moved, `UnreachableCause`, `without_endpoint`, `answer_of`, and a frame verb against a stale instance reading `instance-dead`. **Coverage**: a11y one document, no hint, no ESC. **Decisions**: `inputs#I2` Q1, the word of the founder.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 33
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New record, a `values · cognitive-errors` block: - **CLI, a recovery instruction per cause:** without `--json` the exit-21 causes read through `hint_cause` (`not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify`) beside the fixed instance-state pair for strict-modes, each `hint:` plain ASCII and last on stderr, no two codes sharing a hint and no hint naming `release` (the text: design-system §Surface: cli, `src/human/hints.rs`). The `endpoint-lost` hint, which says the wrapper went away during the call, is PROVISIONAL (`inputs#I2` Q3, delegate the overseer). `not-running` stays the key of the hint for the logged `instance-dead`.'
    sidecar: 'Error recovery: new record, the hint each exit-21 cause ends on; the `endpoint-lost` hint PROVISIONAL.'
    rationale: 'Report **Symbols / APIs** (`hint_cause`), **Coverage** (the two hints: plain ASCII, last on stderr), **Outcome** (`no_two_codes_share_a_hint`, no hint naming `release`), **Reverted** (`not-running` not renamed). **Expected amendments**: `inputs#I2` Q3, PROVISIONAL.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 39
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New record, a `values · cognitive-errors` block: - **CLI, the start of `viola revive --json`:** every refusal gives one exit-1 document, its codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `stale-heartbeat` is the code of a stale holder, `already-live` of a live one. A `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it, so a passed `revive` writes nothing while its child runs. The human pairs stand byte for byte.'
    sidecar: 'Error recovery: new record, the closed exit-1 codes of `viola revive --json` and the `pump_child` carve-out.'
    rationale: 'Report **Symbols / APIs**: `viola revive --json`, `run` start under `--json`, a stale holder code. **Retired**: `already-live` as the refusal of a stale holder. **Outcome**: a passed `revive` writes nothing while its child runs.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 81
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New record, a `values · cognitive-errors` block: - **CLI, an internal error under `--json`:** where the `cli` catch site prints `error: internal error`, `--json` among the arguments before a `--` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in its place, stderr empty and no hint. Without `--json` the line stands byte for byte; `run`, the start of `revive`, `mcp` and `hook` stay silent at the catch site.'
    sidecar: 'Error recovery: new record, the `internal-error` document of the `cli` catch site under `--json`.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`: `json_asked`, `internal_error_document`. A fault still carries no hint (**Outcome**, the a11y row).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 113
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'New record, a `procedure · cognitive-errors` block: - **Verification, the exit-cause table:** `tests/cross_exit_causes.rs` reads its exit-21 and exit-1 causes in both modes: under `--json` one document with stderr empty, without it the `hint:` last on stderr, a fault with none. Its controls are `no_two_codes_share_a_hint` (beside `shared_hints_reads_a_planted_duplicate`), `no_hint_names_release` and `every_code_is_a_code_word`; the causes its top comment names outside the table keep their own witnesses. `cli_verbs_print_the_internal_error_document_under_json` (`tests/cli_wait_last.rs`) reads the catch-site document, `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` (`tests/cli_instance_state.rs`) the stale document.'
    sidecar: 'Error recovery, verification: new record, `tests/cross_exit_causes.rs` and its controls, with the catch-site and stale witnesses.'
    rationale: 'Report **Changes** (the new file), **Counts** (`cross-table`, the top comment), **New text, by line** (the case names), **Outcome** (the table green with its control; the a11y row).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 133
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'New item under **Verification:** - The `unwrapped-name` and `endpoint-lost` hints (`send_hint`, `src/human/hints.rs`) are pinned by the unit `send_hint_is_the_design_string` and the `UNWRAPPED_NAME` / `ENDPOINT_LOST` literals in `tests/cross_exit_causes.rs`.'
    sidecar: 'Plain language, verification: the two new hints and their literal pins.'
    rationale: 'Report **Coverage**: the two hints, tests unit + e2e (`send_hint_is_the_design_string`, the table). **New text, by line** lists both literals. These two strings are literal-pinned, an exception the record at this place enumerates.'
note: 'This file only: the body line a11y-plan.md:31 (the Owed gap) and a11y-plan.md:273 (the literal-pinned list) also move by this report; D-a11y-surface and D-a11y-obs-schema fired nothing here (no interactive element; no a11y violation schema in this file).'
