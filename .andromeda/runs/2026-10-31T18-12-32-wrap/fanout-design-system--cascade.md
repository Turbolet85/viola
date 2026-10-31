proposals:
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 40
    section: '## Surface: cli › ### Component Patterns'
    change: 'At the end of the Refusal item, before its closing period, add: under `--json` the refusal, the board''s only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty; the human pairs stand as written.'
    sidecar: 'surface cli-list: the board''s refusal states its `--json` document, `detail` `strict-modes-failed`.'
    rationale: 'Report **Symbols / APIs**: `src/cmd/list.rs`''s `refused_document` builds `strict-modes-failed`, the board''s only refusal; the item stated only the human form.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 65
    section: '## Surface: cli › ### Component Patterns'
    change: 'After the unwrapped-session row add one row of the same list: `    - a wrapper lost with the call open (`endpoint-lost`): its own hint, the string at `src/human/hints.rs:50-55` byte for byte (the report quotes its words only as "went away during the call"), last on stderr, plain ASCII; the wording is the delegate''s, `inputs#I2` Q3, PROVISIONAL`.'
    sidecar: 'surface cli-send: the new `endpoint-lost` hint row, its wording provisional.'
    rationale: 'Report **Counts** `hints`: the new hint is new text; **Coverage**: last on stderr, plain ASCII; **Decisions**: Q3 is the delegate''s word, PROVISIONAL. The report gives no full literal and the codebase is not this pass''s input.'
    fact: F5
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 66
    section: '## Surface: cli › ### Component Patterns'
    change: 'At the end of the strict-modes row, replace `under `--json` the `instance-unreachable` document` with `under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed``.'
    sidecar: 'surface cli-send: the frame verbs'' strict-modes document names its cause.'
    rationale: 'Report **Counts** `doc-null`: a strict-modes site now reads `strict-modes-failed`; the instance-state pair, no frame and no line are unmoved.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 70
    section: '## Surface: cli › ### Component Patterns'
    change: 'After the stale-heartbeat row add one row of the same list: `    - The codes of these two: a start against a stale holder prints and logs `stale-heartbeat`; a live holder and a fresh replayed beat stay `already-live`; both human pairs stand as written.`'
    sidecar: 'surface cli-send: a stale holder has its own code, `stale-heartbeat`.'
    rationale: 'Report **Symbols / APIs**, a stale holder''s code; **Retired**: `already-live` as a stale holder''s refusal.'
    fact: F6
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 83
    section: '## Surface: cli › ### Component Patterns'
    change: 'The line now reads: `  - Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose. Exit 21''s document is `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one of the five closed causes `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; exit 1''s is `{"v":1,"error":"<code>","detail":null}`, printed by `viola revive` for every refusal of the start it shares with `run` and, as `internal-error`, by the `cli` catch site. obs-plan D-20 names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and for `viola revive` `no-session` and `cwd-missing` (its strict-modes refusal writes no log line since chunk 2026-10-10-budget-governor, so it carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1). Of them, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document reads the same word. The socket-directory cause and macOS''s server check stay without these codes, with the route entry "Unix endpoint and home hardening".`'
    sidecar: 'surface cli-send: the arch-amendment pending block is spent; the exit-21 document names its cause and the two standing carve-outs are named.'
    rationale: 'Report **Retired**: the exit-21 document''s `detail: null` and the `revive --json` post-preflight gap; **Schema / config**: the enum gains `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`; **Outcome** `v1-18`: the two carve-outs stay unmet.'
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 84
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace `and the `--json` document''s `detail` is row 20''s closed object` with `and the `--json` document''s `detail` is, for the wrapper fault, row 20''s closed object and, for the internal error, null (`{"v":1,"error":"internal-error","detail":null}`)`.'
    sidecar: 'surface cli-send: the internal-error fault''s `--json` document stated beside the wrapper fault''s.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`: `internal_error_document()` prints a null `detail`; the sentence gave both faults row 20''s object.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 90
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace `as the hint cause `server-verify` beside `not-running`` with `as the hint cause `server-verify` beside `not-running`, `unwrapped-name` and `endpoint-lost` (`not-running` keeps its key, not renamed to the log''s `instance-dead`)`.'
    sidecar: 'surface cli-send: the four hint causes named beside the server-verification hint.'
    rationale: 'Report **Symbols / APIs**: `hint_cause()` reads `not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify`; **Reverted / negative API facts**: `not-running` stays the key.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 135
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace from `Under `--json` (both arms;` to the end of the line with: `Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract; every start refusal, chunk 2026-10-31-exit-cause-code-catalogue) a refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty, its codes in the item below, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`.`'
    sidecar: 'surface cli-verify-run: the `revive --json` post-preflight gap is closed and its owner clause removed.'
    rationale: 'Report **Symbols / APIs**: "a refusal `run`''s start makes after a passed preflight prints no document" is no longer true; **Retired**: the post-preflight gap.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 135
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- **`revive --json`''s documents:** one exit-1 document for every refusal of the start it shares with `run`, its closed codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `run` itself prints none of these documents.`'
    sidecar: 'surface cli-verify-run: the ten closed codes of `revive --json`''s start.'
    rationale: 'Report **Symbols / APIs**, `viola revive --json` and `run`''s start under `--json` (`run` passes false).'
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 135
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- **`revive --json`''s `internal-error`:** a program not found prints `internal-error`, a cause with no human pair and none added; an anyhow error of its start prints the same document and still reaches the catch site''s log.`'
    sidecar: 'surface cli-verify-run: `internal-error` under `revive --json`, its two paths.'
    rationale: 'Report **Symbols / APIs**: `refused_with` for a program not found; `started` prints `refused_document("internal-error")` and returns the same error.'
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 135
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- **A `pump_child` error under `revive --json`:** prints no document: the child has held the terminal and viola writes nothing past it.`'
    sidecar: 'surface cli-verify-run: a `pump_child` error stays without a document.'
    rationale: 'Report **Symbols / APIs**, `viola revive --json`; **Reverted / negative API facts**: a `Started`-time document from `main` was rejected.'
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 144
    section: '## Surface: cli › ### Component Patterns'
    change: 'The `--json` cell now reads: `` `viola revive --json`''s refusals of the start it shares with `run`: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; arch §Standard Contracts); under `--json` the `cli` catch site''s `{"v":1,"error":"internal-error","detail":null}`, stderr empty, with `run`, `revive`''s start, `mcp` and `hook` silent there; a `pump_child` error and every other exit 1 have no `--json` document ``. The human cell stands byte for byte.'
    sidecar: 'surface cli-exit-codes: row 1''s `--json` cell gains the start documents and the catch site''s document; its owner clause removed.'
    rationale: 'Report **Retired**: the post-preflight gap; **Counts** `start-codes-rev`, `title`, `catch-site`: the human line stands, a site stating the output under `--json` takes the document.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 152
    section: '## Surface: cli › ### Component Patterns'
    change: 'In the human cell replace `(not running / unwrapped / server verification)` with `(not running / unwrapped / endpoint lost / server verification)`. The `--json` cell now reads: `` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; the `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs'' strict-modes refusals read `strict-modes-failed`; stderr empty ``.'
    sidecar: 'surface cli-exit-codes: row 21''s document is one per cause; its awaiting clause is spent.'
    rationale: 'Report **Symbols / APIs**, the exit-21 document moved; **Retired**: its `detail: null`; `inputs#I2` Q1, the founder''s own word.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 162
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- Under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty; without `--json` the line stands byte for byte.`'
    sidecar: 'surface cli-exit-codes: the catch site''s `--json` document.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`; **Counts** `catch-site`.'
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 162
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- The catch site counts `--json` only among the arguments before a `--`, read from argv.`'
    sidecar: 'surface cli-exit-codes: how the catch site reads `--json`.'
    rationale: 'Report **Symbols / APIs**: `json_asked(args)` in `src/main.rs` (`:154-161`), read from argv as `role_of` is.'
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 162
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- `run`, `revive`''s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.`'
    sidecar: 'surface cli-exit-codes: the catch site''s four silent paths.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`.'
    fact: F13
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 168
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace the line''s document `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`.'
    sidecar: 'surface cli-exit-codes: the server-verification document names its cause.'
    rationale: 'Report **Symbols / APIs**: the five closed causes; **Retired**: the exit-21 document''s `detail: null`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 168
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- Exit 21''s `--json` document carries its cause in `detail`, one of five closed words: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`; `v` stays 1 and `error` stays `instance-unreachable`.`'
    sidecar: 'surface cli-exit-codes: the five closed exit-21 causes.'
    rationale: 'Report **Symbols / APIs**, the exit-21 document moved.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 168
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- A refused strict-modes check at exit 21 reads `strict-modes-failed` in that document on the eight frame verbs, `list` and `plugin install`.`'
    sidecar: 'surface cli-exit-codes: where exit 21 reads `strict-modes-failed`.'
    rationale: 'Report **Symbols / APIs**: `refuse_untrusted` and `src/cmd/plugin.rs` write `StrictModes` through `unreachable_document`, and `src/cmd/list.rs` builds the same word.'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 168
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name` (`inputs#I2` Q2, the delegate''s word, PROVISIONAL).`'
    sidecar: 'surface cli-exit-codes: the no-endpoint reading of the instance directory, provisional.'
    rationale: 'Report **Symbols / APIs**: `without_endpoint`, the strict-modes check first at every caller; **Decisions**: Q2 is the delegate''s word, PROVISIONAL.'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 168
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- A failed connect is `instance-dead`, a wrapper lost with the call open is `endpoint-lost`, and a failed server verification is `server-verify-failed`.`'
    sidecar: 'surface cli-exit-codes: the connect, lost-call and server-verification causes.'
    rationale: 'Report **Symbols / APIs**: `answer_of`, `Connect` is `Dead` and `Closed | Io` is `Lost`; `of_reply` likewise; the `ServerUnverified` arm.'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 168
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add the item: `- A frame verb against a stale instance reads `instance-dead`: stale heartbeat is exit 1''s cause alone.`'
    sidecar: 'surface cli-exit-codes: a stale instance reads `instance-dead` on a frame verb.'
    rationale: 'Report **Symbols / APIs**, a stale holder''s code: `live_endpoint` gives none for a stale wrapper.'
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 263
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: 'Replace `which exit 2 with their fixed stderr line and no document.` with `which exit 2 with their fixed stderr line and no document, and a `pump_child` error under `viola revive --json`, which prints none because the child has held the terminal. At the catch site `run`, `revive`''s start, `mcp` and `hook` stay silent.`'
    sidecar: 'body Streams: the `--json` line names the paths that stay without a document.'
    rationale: 'Report **Symbols / APIs**: a `pump_child` error prints no document; `run`, `revive`''s start, `mcp` and `hook` stay silent at the catch site.'
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 292
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace the last sentence with: `A failed strict-modes check refuses the whole board (exit 21) in the fixed `unable:` / `hint:` form, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`.`'
    sidecar: 'body cli-list: the board''s refusal states its `--json` document.'
    rationale: 'Report **Symbols / APIs**: `src/cmd/list.rs`''s `refused_document` builds `strict-modes-failed`, the board''s only refusal.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system.md
    near: 303
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace `Under `--json` no hint prints; a per-cause `detail` code is pending, owner: the arch amendment naming them.` with `Under `--json` no hint prints; exit 21''s document names its cause in `detail`, one of five closed words.`'
    sidecar: 'body cli-send: the pending per-cause `detail` code is delivered.'
    rationale: 'Report **Retired**: the exit-21 document''s `detail: null`; **Expected amendments**: design-system''s two pending lines.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system.md
    near: 334
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace from `and under `--json` a refused strict-modes check` to the end of the line with: `and under `--json` every refusal of the start it shares with `run` prints one stdout document and no hint; a `pump_child` error prints none, the child having held the terminal.`'
    sidecar: 'body cli-verify-run: the `revive --json` post-preflight gap is closed and its owner clause removed.'
    rationale: 'Report **Symbols / APIs**: the sentence "a refusal `run`''s start makes after a passed preflight prints no document" is no longer true; **Retired**; **Expected amendments** names `design-system.md:334`.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system.md
    near: 338
    section: '## Surface: cli › ### Component Patterns'
    change: 'In code 1 replace `(a `--json` document only for `viola revive`''s strict-modes and preflight refusals)` with `(a `--json` document for every refusal of `viola revive`''s start, and the catch site''s `internal-error` document)`.'
    sidecar: 'body exit-code row 1: its `--json` limit widened to the start documents and the catch site''s.'
    rationale: 'Report **Retired**: the post-preflight gap; **Symbols / APIs**, the `cli` catch site under `--json`; **Expected amendments** names `design-system.md:338`.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system.md
    near: 338
    section: '## Surface: cli › ### Component Patterns'
    change: 'In code 21 replace `its `--json` document with stderr empty` with `its `--json` document, one per cause in `detail`, with stderr empty`.'
    sidecar: 'body exit-code row 21: the document is one per cause.'
    rationale: 'Report **Counts**: exit-21''s document, one form to one per cause; **Expected amendments** names `design-system.md:338`.'
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 340
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace `or under `--json` the `instance-unreachable` document, stderr empty.` with `or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty.`'
    sidecar: 'body plugin install: the strict-modes document names its cause.'
    rationale: 'Report **Symbols / APIs**: `src/cmd/plugin.rs` writes `StrictModes` through `unreachable_document`.'
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 342
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace `or under `--json` the `instance-unreachable` document, stderr empty; no frame` with `or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty; no frame`.'
    sidecar: 'body link, unlink and frame verbs: the strict-modes document names its cause.'
    rationale: 'Report **Symbols / APIs**: `refuse_untrusted` writes `StrictModes` through `unreachable_document`; **Counts** `doc-null`.'
note: 'F5''s full hint literal is not in the report, so the orchestrator takes it from `src/human/hints.rs:50-55` and checks it against layout-templates; the report''s `surface.md:65` pending status is not in the picked text, so no pending edit is proposed there and any such line outside the picked entities stays unread; `surface.md:144` already passes 2 000 chars, and `verify`''s log codes are obs-plan''s.'
