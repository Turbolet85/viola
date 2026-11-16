@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — wait / last lines as landed: the `viola wait` output structure gained the `session-end` line (every non-dialog kind takes the `turn-ended` form), `dialog unknown`, the exit-21 `unable  <name>  instance-unreachable` + hint pair, `waiting:` only on a terminal stderr without `--json`, one `--json` document. Origin of contract/viola-wait-output/turn-ended, dialog-unknown, waiting-line and contract/cli-verbs/wait; this chunk's wake-condition statement (which kinds return a parked wait, a `turn-ended` carrying `background_tasks`) extends exactly this output block and must keep the turn-ended form for every non-dialog wake kind.

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `viola wait` wireframe gained the `dialog-expired` line; the `session-end` annotation became "every kind without a dialog_id takes the turn-ended form" (was "every non-dialog kind"). Precedent: a new logged kind (`dialog-expired`) was made a wake kind and its line added to the wait output — the same register (wake kinds vs log-only kinds) this chunk documents and pins; source of contract/viola-wait-output/dialog-expired and the turn-ended row's wording.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the `dialog-expired` line's wording (planning-form mark removed). Constraint: the wait output lines stand as founder-worded; any new or changed wait line this chunk would add is a wording change owed to the founder, not an implementation choice (decision/human-output-is-contract).

@history_item n=4 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` as the hidden driver's machine view: each tool success/refusal is the CLI verb's `--json` document less `v`, refusals `isError: true` with `refused: <code>`, no tool naming release/pause/link/unlink. Source of contract/mcp-tools/wait and contract/mcp-tool-results/*; the MCP `wait` tool must wake on exactly the same kinds as `viola wait --json`, so a wake-condition change lands in both views identically.

@history_item n=5 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — `wait`'s exit-21 not-running hint became one cause among several; new server-verify cause for the frame verbs (wait included): no frame written, exit 21, its own hint last on stderr. Source of contract/frame-verb-hints/instance-not-running and server-verify-failed; a re-armed or parked wait still passes through this check before any frame.

@history_item n=6 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink` join the exit-21 `unable` lines) — the not-running hint and server-check cause widened from wait/last to the eight frame verbs; the catch-site verb list grew. Shapes contract/frame-verb-hints/instance-not-running as worded today (wait, last, link, unlink, send's mirror).

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' exit-21 instance pair) — `wait` runs the instance's strict-modes check first (`client::refuse_untrusted`), before any role file or log opens, mode left as found. Source of contract/frame-verbs/strict-modes-refused; trap noted in the sibling entry: opening a role file narrows a home to 0700, so the check order is pinned by a test — any wait-path change must keep the check ahead of any log/file open.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`) — exit 21's `--json` document went from `detail: null` to one cause word (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`), so a driver can tell causes apart. Source of the contract/instance-unreachable-detail/* rows; a wait that loses its wrapper mid-park reads `endpoint-lost` — relevant to the driver's re-arm behaviour this chunk measures.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document under `--json`) — `viola wait` / `viola last`'s failure or caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout under `--json`, stderr empty. Constraint for wait's `--json` form (contract/cli-json-mode/one-document): one document in every outcome, the internal error included.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the unwrapped-name and endpoint-lost hints) — new hints for a name with no instance directory and a wrapper lost while the call is open (`went away during the call`). Source of contract/frame-verb-hints/not-wrapped and wrapper-went-away; the latter is the hint a long-parked wait prints when its wrapper dies, so the wake/park statement must not blur it with a turn end.

@history_item n=11 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's word superseded the provisional records of the 2026-10-31-exit-cause-code-catalogue entries (Q2, Q3 hints included). The not-wrapped and wrapper-went-away hint wordings are founder-ratified; changing them is a founder fork.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's sixth cause, `socket-dir-refused`, at the frame verbs) — a frame verb (wait included) meeting a refused socket directory writes no frame, exits 21 with its own hint, `detail: "socket-dir-refused"` under `--json`; distinct from the strict-modes refusal. Source of contract/frame-verb-hints/socket-dir-refused and contract/instance-unreachable-detail/socket-dir-refused.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the server-check hint without its OS scope) — the frame verbs' server-check sentence lost its Linux/Windows/macOS scope (macOS carve-out retired); the hint and `server-verify-failed` document stand. Linux-only target: wait tests and the fake agent need no per-OS arms.

@history_item n=14 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — 0.2.0 version census: no web surface (absence/web-surface), the CLI ban on interactive verbs kept, and the «on Unix» qualifiers on the socket-directory refusals dropped. Bounds this chunk to the CLI/MCP surfaces only; no spinner or interactive affordance may be added to `wait` (decision/no-spinner).

@history_item n=15 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (108 records -> 226 rows), each verified against the code; prose dropped where "the code shows". Every picked row (wait output, frame-verb hints, exit-21 details, verify output) dates from this conversion; a wait wake-condition fact the masters lack belongs as a new row through the rows-proposal path, not as prose.

@history_item n=16 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — precedent for adding a cli contract row (`+ contract/viola-answer-output/not-delivered-hints`) when a cli output behaviour the masters did not name had to be stated. The same `+ {handle}` form is how this chunk would add a documented wake-condition row under the wait output contract.

@history_item n=17 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `viola verify`'s output structure: one step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` summary, the usage line. Origin of contract/viola-verify-output/*; the CI carry's `stamped_home` fixture runs this verb unbounded (`within: None`), so any change to how the fixture stamps must keep these lines and the stamp intact.

@history_item n=18 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped before its rows are checked prints only the stopped pair, no summary, no stamp, exit 1; the summary closes only a completed run. Trap for the carry's option (b)/(a): a fixture verify bounded or killed by a timeout yields no stamp, so a stamped home must come from a completed run (contract/viola-verify/summary).

@history_item n=19 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count (`LedgerRow::relied()`), one step line per relied row, exit 0 when every relied row passes. Source of contract/viola-verify-output/exit-0 and step-line; the fixture's verify time scales with the relied row count.

@history_item n=20 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th relied row (`local-command-compact`, a 40 s PROVISIONAL window), the current `[NN/24]`. Context for the carry: the stamped-home fixture's unbounded `viola verify` runs all 24 rows, some with timing windows, which is where its loaded time goes.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS) — all 24 rows relied on every build (Windows relied set gone), stop on `SIGINT` / `SIGTERM` only. The fixture's verify cost is the full 24-row run on the Linux CI runner.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing; `unknown`, `wrapper fault` and `internal error` still take no hint. Bounds decision/hint-for-every-cause's exception: any new wait refusal cause needs its own hint unless it falls in that list.

@history_item n=23 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `--json` became the machine view of every data verb, `viola revive` included. Source of decision/json-on-every-data-verb; `wait --json` stays the driver's view this chunk's wake condition is pinned over.
