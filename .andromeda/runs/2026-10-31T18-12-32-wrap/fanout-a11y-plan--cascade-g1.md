proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 31
    section: '## 1. A11y Scope Summary'
    change: 'Remove the **Owed:** line that says a refusal `run`''s start makes after a passed preflight prints no document yet under `viola revive --json`: the gap is closed and its owed-to entry "Exit-cause code catalogue" is spent.'
    sidecar: 'Spent the Owed line on the `viola revive --json` post-preflight gap, closed by chunk 2026-10-31-exit-cause-code-catalogue.'
    rationale: 'The report''s Retired bullet names the `revive --json` post-preflight gap, and its Symbols / APIs says the sentence is no longer true.'
    basis: .andromeda/a11y-plan.md:31
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'In the `revive` `--json` clause, read: under it each exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`.'
    sidecar: 'Widened the `viola revive --json` refusal clause and its code list to the ten codes of chunk 2026-10-31-exit-cause-code-catalogue.'
    rationale: 'The report''s Symbols / APIs, under `viola revive --json`, lists these ten codes and one exit-1 document for every refusal of the start it shares with `run`; its Counts row `start-codes-rev` marks the code lists changed.'
    basis: src/cmd/revive.rs:119-134
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'Remove the clause "a refusal `run`''s start makes after a passed preflight prints no document as landed (owed to "Exit-cause code catalogue");", leaving the sentence''s other clauses as they stand.'
    sidecar: 'Removed the post-preflight no-document clause and its owed-to title from the CLI verbs entity.'
    rationale: 'The report''s Retired bullet names the `revive --json` post-preflight gap; its Counts row for lines over 2 000 chars marks revive-gap and title changed on this line.'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'In the `viola plugin install` clause, read: its exit-21 refusal is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.'
    sidecar: 'The `viola plugin install` exit-21 document now names its cause, `strict-modes-failed`.'
    rationale: 'The report''s Retired bullet names the exit-21 document''s `detail: null`, and its Symbols / APIs says `src/cmd/plugin.rs` writes `StrictModes` through `unreachable_document`.'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'In the server-verify clause, read: or under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty; and end the entity with: their `error` word and exit codes are unchanged.'
    sidecar: 'The frame verbs'' server-verify document now names its cause, `server-verify-failed`; the closing clause now says what stayed unchanged, the `error` word and the exit codes.'
    rationale: 'The report''s Retired bullet names the exit-21 document''s `detail: null`, and its Symbols / APIs says no exit moved and `error` stays `instance-unreachable`, while `detail` now carries a code per cause.'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 11
    section: '## 1. A11y Scope Summary'
    change: 'Add after this line:   - **The start''s refusals under `viola revive --json`:** every refusal of the start it shares with `run` prints one exit-1 document before the child spawns, `batch-script-child`, `pinned-hash-mismatch` and `squatted-name` among them; the human pairs are byte for byte as they stood.'
    sidecar: 'Added the start refusals of `viola revive --json` to the CLI verbs entity.'
    rationale: 'The report''s Symbols / APIs: `refused_with` prints `refused_document` in place of the pair for these three codes, all returning before `spawn_child`, the human pairs byte for byte as they stood.'
    basis: src/cmd/run.rs:422-431
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 11
    section: '## 1. A11y Scope Summary'
    change: 'Add after this line:   - **A `claude` not found at a start:** under `viola revive --json` it reads as the `internal-error` document, and it has no human pair, none being added.'
    sidecar: 'Added the `internal-error` reading of a `claude` not found under `viola revive --json`.'
    rationale: 'The report''s Symbols / APIs: for a program not found, `refused_with` prints `internal-error`, with no pair today and none added; the table''s `a_revive_with_no_claude_on_its_path_is_internal_error` drives it.'
    basis: src/cmd/run.rs:422-431
    fact: F8
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 25
    section: '## 1. A11y Scope Summary'
    change: 'Read: or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty; the rest of the entity stands.'
    sidecar: 'The frame verbs'' strict-modes document now names its cause, `strict-modes-failed`.'
    rationale: 'The report''s Retired bullet names the exit-21 document''s `detail: null`; its Symbols / APIs says `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 47
    section: '## 1. A11y Scope Summary'
    change: 'Add after this line:     - under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.'
    sidecar: 'Added to the TUI passthrough boundary that a `pump_child` error under `viola revive --json` prints no document.'
    rationale: 'The report''s Symbols / APIs, under `viola revive --json`: a `pump_child` error prints no document, because the child has held the terminal; its Outcome re-asserts that a passed `revive` writes nothing while its child runs.'
    basis: src/cmd/revive.rs:119-134
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 71
    section: '## 1. A11y Scope Summary'
    change: 'Read: an MCP frame tool returns `isError` with the code `instance-unreachable` and `detail` `server-verify-failed` (text `refused: instance-unreachable`) and sends no frame; the rest of the reason stands.'
    sidecar: 'The MCP server-verify `isError` body now carries its cause, `server-verify-failed`, beside the code.'
    rationale: 'The report''s Symbols / APIs under MCP: a failed `server_verified` is `ServerVerify`, the `isError` body is the CLI document less `v`, and the text stays `refused: instance-unreachable`; "the code only" no longer holds, and no human-facing byte is added.'
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 80
    section: '## 1. A11y Scope Summary'
    change: 'Read: an MCP frame tool returns `isError` with the code `instance-unreachable` and `detail` `strict-modes-failed` and sends no frame and writes no line; the rest of the reason stands.'
    sidecar: 'The MCP strict-modes `isError` body now carries its cause, `strict-modes-failed`, in place of `detail: null`.'
    rationale: 'The report''s Retired bullet names the exit-21 document''s `detail: null`; its Symbols / APIs under MCP says a refused `check_instance` is `StrictModes`, with no frame and no line.'
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:80
note: Line 8 of a11y-scope-summary.md is already over the 2 000-char limit (report, Counts) and these edits do not bring it under; D-a11y-surface did not fire, as the report adds no interactive element, and D-a11y-obs-schema found no divergence in a11y-plan's body, which holds no violation schema, though §3's contracts registry was outside this read and the `process-exit` enum moved 13 to 19.
