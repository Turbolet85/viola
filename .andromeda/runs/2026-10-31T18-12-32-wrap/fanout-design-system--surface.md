proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 83
    section: '## Surface: cli › ### Component Patterns'
    change: 'Line 83 now reads whole:   - Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose. Exit 21 prints `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of the closed five, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (the founder, his own word, `inputs#I2` Q1). Exit 1 prints `{"v":1,"error":"<code>","detail":null}` for every refusal of `viola revive` and, under `--json`, at the catch site. obs-plan D-20 names the log detail codes: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`; for `viola revive` `no-session` and `cwd-missing` (its strict-modes refusal writes no log line since chunk 2026-10-10-budget-governor, so it carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1); and for `viola verify`, under a parsed `VIOLA_NAME`, `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` and `unclean-fixture` beside `interrupted` (a refused home tree stays lineless). Of them, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document carries the same word. A stale holder reads `stale-heartbeat` (its log line, and the document of `viola revive --json`); a live holder and a fresh replayed beat read `already-live`.'
    sidecar: 'surface.md cli-send: the pending arch amendment on `--json` retired; exit 21 carries one `detail` per cause, the stale-heartbeat and unwrapped-name causes have their codes, the log code list grows.'
    rationale: 'Report Retired names the exit-21 `detail: null` and `already-live` as a stale holder refusal; Counts `causes-new` and `stale-live` each list surface.md:83; Symbols / APIs gives the five causes, the stale code and the codes of verify; Schema / config gives the six new enum words.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 152
    section: '## Surface: cli › ### Component Patterns'
    change: 'In row 21, the `--json` cell now reads: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, `strict-modes-failed` for the `list`, `link`, `unlink` and `plugin install` refusals and the strict-modes refusals of the six frame verbs, with stderr empty'
    sidecar: 'surface.md cli-exit-codes row 21: the `--json` document carries its cause; the awaited arch amendment clause removed.'
    rationale: 'Report Symbols / APIs, the exit-21 document moved; Retired names `detail: null`; Expected amendments name exit-code row 21 and surface.md:152.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 168
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace the closing document with `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`; the rest of the line stands.'
    sidecar: 'surface.md cli-exit-codes: the server-verification document reads `detail` `server-verify-failed`.'
    rationale: 'Report Symbols / APIs, the five closed causes; Retired names `detail: null`; Expected amendments name surface.md:168.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 135
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace the line from `Under --json` to its end with: Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract) every refusal of its strict-modes check, its preflight and the start it shares with `run` prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`. `internal-error` is also a program not found, which has no human pair, and an error of its start before the child, which still logs at the catch site; a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.'
    sidecar: 'surface.md cli-verify-run: the `revive --json` post-preflight gap closed; ten codes, every start refusal one document.'
    rationale: 'Report Symbols / APIs, `viola revive --json`: its sentence that a post-preflight refusal prints no document is no longer true; Retired names the gap; Counts `revive-gap`, `start-codes-rev` and `title` move.'
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 144
    section: '## Surface: cli › ### Component Patterns'
    change: 'In row 1, the `--json` cell now reads: every refusal of `viola revive --json`, its strict-modes check, its preflight and its start: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; arch §Standard Contracts); under `--json` the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr empty (`run`, the start of `revive`, `mcp` and `hook` stay silent there); every other exit 1 has no `--json` document'
    sidecar: 'surface.md cli-exit-codes row 1: the owed `revive` start clause spent, ten codes, the document of the catch site added; the human cell stands byte for byte.'
    rationale: 'Report Counts lines over 2 000 chars: surface.md:144 catch-site re-derived, title changed; Symbols / APIs, `viola revive --json` and the `cli` catch site under `--json`.'
    dependent-of: D-retired
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 84
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace the closing clause `and the --json document detail is row 20 closed object` with: and under `--json` `error: wrapper fault` reads row 20 with its closed `detail` object, while `error: internal error` reads `{"v":1,"error":"internal-error","detail":null}` on stdout in its place, stderr empty.'
    sidecar: 'surface.md cli-send: the fault line states the new `internal-error` document under `--json`.'
    rationale: 'Report Symbols / APIs, the `cli` catch site under `--json`; Counts `catch-site`: a site stating the output under `--json` takes the document; Expected amendments name surface.md:84.'
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 90
    section: '## Surface: cli › ### Component Patterns'
    change: 'Replace `as the hint cause server-verify beside not-running` with: as the hint cause `server-verify` beside `not-running`, `unwrapped-name` and `endpoint-lost`; the rest of the line stands.'
    sidecar: 'surface.md cli-send: the hint causes are four.'
    rationale: 'Report Symbols / APIs, `src/cmd/client.rs`: `hint_cause()` is `not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 65
    section: '## Surface: cli › ### Component Patterns'
    change: 'Line 65 now reads:     - the name is an unwrapped session (built, hint cause `unwrapped-name`: no instance directory at the name, whether absent, a regular file or a symlink; the reading PROVISIONAL, `inputs#I2` Q2, delegate the overseer): `<name> is not a wrapped session; only sessions started with viola run take commands`'
    sidecar: 'surface.md cli-send: the unwrapped-name hint built, its reading named; its wording did not move.'
    rationale: 'Report Counts `hints`: 0 changed in wording, and the pending status of surface.md:65 moves; Symbols / APIs, `without_endpoint`; Retired names the not-built line of the unwrapped-name hint.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 65
    section: '## Surface: cli › ### Component Patterns'
    change: 'New list item following line 65:     - the wrapper is lost with the call open (hint cause `endpoint-lost`; PROVISIONAL, `inputs#I2` Q3, delegate the overseer): the whole literal of the endpoint-lost hint, its wording "went away during the call", `<name>` the parsed name, plain ASCII, last on stderr'
    sidecar: 'surface.md cli-send: the new endpoint-lost hint, a delegate word, PROVISIONAL.'
    rationale: 'Report Counts `hints`: the new hint is new text; Decisions: Q3, the hint "went away during the call"; Coverage: the two hints, plain ASCII, last on stderr.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 152
    section: '## Surface: cli › ### Component Patterns'
    change: 'In row 21, the human cell lists the hints as (not running / unwrapped / lost during the call / server verification) in place of (not running / unwrapped / server verification).'
    sidecar: 'surface.md cli-exit-codes row 21: the endpoint-lost hint joins the list of hints.'
    rationale: 'Report Symbols / APIs: `endpoint-lost` is one of the five closed causes, `hint_cause()` carries it; Outcome: the two hints.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 90
    section: '## Surface: cli › ### Component Patterns'
    change: 'New list item following line 90: - Each cause of `instance-unreachable` has its reading, after the strict-modes check, which stays first. With no endpoint the instance directory decides (`viola_state::liveness::without_endpoint`, opening nothing and writing no line): a directory is `instance-dead`, the hint cause `not-running`, not renamed; anything else is `unwrapped-name`. A refused connect is `instance-dead`; a connection closed or failed with the call open is `endpoint-lost`. `endpoint-lost` carries `during` `call`; `instance-dead`, `unwrapped-name` and `server-verify-failed` carry `during` `connect`. A frame verb against a stale instance reads `instance-dead`: stale heartbeat is a cause of exit 1 alone.'
    sidecar: 'surface.md cli-send: how each exit-21 cause is read, new detail of this chunk.'
    rationale: 'Report Symbols / APIs: `without_endpoint`, `answer_of`, a process-exit `during`, a stale holder code; Reverted / negative API facts: `not-running` stays the key of the hint cause.'
note: The whole endpoint-lost hint literal is in no master and the report gives only "went away during the call" (inputs#I2 Q3), so apply it from src/human/hints.rs:50-55; D-design-tokens holds (every Coverage tokens flag n/a); surface.md:144 is already over the 2 000-char line limit; body lines .andromeda/design-system.md:303, :334 and :338 state the same retired claims, outside this file's proposals.
