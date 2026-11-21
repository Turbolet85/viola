@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the MCP wait cap, a driver-only bound) §8 Timeout extensions: the MCP `wait` tool clamps `timeout_ms` to `WAIT_CAP` 50 000 ms, which is also its default (`viola_mcp::wait_timeout`). The bound falls on the driver only: the plugin-run `viola mcp` serves a driving session, never a person. Why it matters: this is the precedent this chunk extends. That chunk closed `wait` "below the client's tool-call timeout" without a measured client bound. This chunk measures that bound and must keep `contract/mcp-wait-tool/timeout-ms` and the driver-only framing true for every tool it caps.

@history_item n=2 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — §8: a verified `send` waits in the readiness gate for at most `GATE_MAX_WAIT` (8.5 s), then is refused `not-delivered` / `input-not-ready` with nothing typed. A human key during the wait takes the wheel and the send is refused `human-typing`; the outer-PTY case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` is sequenced on the wrapper's lines, never a timer. Why it matters: the gate is one segment of `send`'s longest block. Any cap or split must leave `contract/send-refusal/input-not-ready`, `decision/human-key-takes-the-wheel` and "the human always wins" unchanged.

@history_item n=3 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — §8: once typed, a confirmed `send` waits within `CONFIRM_WINDOW_FALLBACK` (10 s), or within `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL) for `/compact` on a verified CLI. If it is not confirmed in time the result is `not-delivered` / `no-prompt-submitted`, a bound on the driver only. Why it matters: the 40 s compact window plus the 8.5 s gate is `send`'s longest block. The chunk must sum these against the client's tool-call timeout. Any change touches `contract/send-refusal/no-prompt-submitted` and the time-bound rows.

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — §8 Timeout extensions changed from "no limit on a human" to a dialog deadline that bounds the driver's `answer` path only; on expiry the dialog renders for the human. Why it matters: this is the origin of the rule that every viola time bound falls on the driver, never the human. A new MCP call bound must keep that rule.

@history_item n=5 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold became `dialog_hold_s`, 60–3600 s, default 60, under a `hooks.json` timeout of 3615 s and a hook bound of 3605 s. At the end of the hold the wrapper appends `dialog-expired`, which wakes `wait`; any human key hands the dialog back. Why it matters: the hold can reach 3600 s and wakes a parked `wait`, so it bears on whether the MCP `answer`/`wait` blocks stay under the client bound (`contract/config-json/dialog-hold-s`, `contract/event-line/dialog-expired`, `decision/dialog-hold-bounds-the-driver`).

@history_item n=6 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the dialog hold bounds owned by 0.2.0's first entry) `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` stay PROVISIONAL, owned by 0.2.0's first entry. Why it matters: these are bound constants this chunk may meet while listing every tool's longest block. Their PROVISIONAL marks and owner must be kept.

@history_item n=7 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver`: state moved from provisional to standing after the hold's ceiling was measured live. Why it matters: this picked row now stands. A bound this chunk adds over `answer` must not contradict it.

@history_item n=8 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` dropped the budget pause from the list of bounds; `~ contract/cli-exit-codes/11` is retired and never reused (an older wrapper's `budget-paused` reads `unknown`, 14). Why it matters: these are the two rows that enumerate every time bound viola holds. A new MCP call bound or cap extends that list. The retired exit 11 must not be reused for a timeout refusal.

@history_item n=9 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` dropped the 5 s status-line deadline. Why it matters: this is the second precedent for editing the enumerated time-bound rows when a bound comes or goes. This chunk follows the same form if it adds an MCP tool-call bound.

@history_item n=10 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` gained "the start's Claude config lock wait", falling on viola's own start. `~ decision/run-diagnostics-off-terminal` gained the trust step's single `trust-record` diagnostics line, never a refusal, exit code or hint. Why it matters: this is the latest form of the two time-bound rows this chunk would extend. It also sets the precedent that a new kind of bound-bearer is named in the rule itself.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — §8: `list` and `revive` each make one `claude agents --json` read, with a 5 s kill and `take(MAX_FRAME + 1)`. A failed or timed-out read fails open, and the bound falls on viola's own child. Why it matters: the MCP `list` tool's longest block includes this read (`decision/claude-agents-read-fails-open`). It must be counted against the client's tool-call timeout.

@history_item n=12 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the plugin against an impostor wrapper) a new not-assertable entity: an MCP frame tool (`send`, `wait`, `last`, `answer`) meeting a server that is not the recorded wrapper returns `isError` `instance-unreachable` with no frame (`server_verified` in `crates/viola-mcp/src/tools.rs`). Why it matters: the server check is part of `send`'s client-side checks, which this chunk must bound. Under `decision/no-assertions-for-plugin-and-ipc`, MCP results write no human-facing byte.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the plugin against a strict-modes-refused instance) a new not-assertable entity: MCP frame tools run the instance check in `frame` before `live_endpoint` and return `isError` `instance-unreachable` with no frame and no line. Why it matters: this is another client-side step on each MCP tool's path. It also sets the precedent that MCP refusals stay out of a11y assertions.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (MCP's `isError` body names its cause; a lost wrapper reads `endpoint-lost`) MCP's `isError` body is the CLI's document without `v`, with `detail` `server-verify-failed` / `strict-modes-failed`. A wrapper lost with the call open reads `endpoint-lost`. Exit 21's closed causes are listed. Why it matters: a call cut at a bound must map to an existing closed word or a new one shown at P4. The cause set (`contract/instance-unreachable-detail/*`, `contract/instance-unreachable-json/document`) and the rule against shared hints (`decision/one-hint-per-code`, `decision/no-hint-names-release`) bind any new code.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (peer refusal screened to a code word; a fault's body; the census of error writers; `--json`'s exit-2 exception) A peer's non-code-word refusal reads `unknown`, and a non-code-word detail reads nothing (null on MCP). A fault outside the five known reads `wrapper fault`. Each error body needs a writer and a witness, and `thiserror` messages interpolate nothing. The `send`/`answer` exit-2 paths write a fixed line and no document. Why it matters: a timeout result or error class this chunk adds on MCP or the CLI must pass these rows (`decision/peer-replies-screened-to-code-words`, `decision/error-bodies-have-writer-and-witness`, `decision/thiserror-messages-interpolate-nothing`, `contract/fault-body/message`, `contract/cli-json-failure/send-answer-exit-2`).

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (hook and MCP on a refused socket directory; the client euid refusal's limit of proof) MCP frame tools return `isError` `instance-unreachable` / `socket-dir-refused` past the live endpoint, with no frame and no line. The client euid decision is tested over injected euids, and seam wiring is read only in remove-the-guard runs. Why it matters: these are more client-side steps on the MCP path to bound (`contract/mcp-frame-tool/socket-dir-refused`). The euid limit of proof (`decision/client-euid-refusal-test-strategy`) sets how far a test of the shared connect can go.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the a11y cases' CI home; the server-verify and socket-directory carve-outs retired) The cli/tui cases run in the `test` job's `coverage` suite on `ubuntu-latest` alone, with `gate --require coverage,doctest` as the verdict. The server-identity and socket-dir witnesses run ungated there. Why it matters: any test this chunk adds that "can fail" runs on that one leg with zero retries (`decision/zero-flakiness`).

@history_item n=18 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the §8 verification lines pinned by literal; cli checks on std::process::Command) Every fixed CLI string, hints included, is pinned by literal. The typed exit codes and a hint-free `--json` are asserted over `std::process::Command`. Why it matters: a new or changed refusal, hint or result line from a capped argument or a split takes a literal pin (`decision/fixed-cli-strings-pinned`, `decision/refusal-layout-literal-pins`).

@history_item n=19 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify/step-line`: changed from `pass|fail` to `pass|absent|fail`, with `absent` only on the browser-tools-prompt row. Why it matters: if the bound becomes a relied ledger row with a `viola verify` probe (a 30th row), it prints a new `[NN/NN]` step line. That line must fit this contract, and `NN/NN` moves.

@history_item n=20 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/tui-passthrough/human-keystrokes`, `~ contract/tui-passthrough/viola-terminal-bytes` and `~ decision/human-keystrokes-never-blocked`: a start's automatic verify window, before the child exists, is the one exception. Keys typed then wait in the terminal's buffer, and verify's lines L1–L5 and its output precede the child's bytes. Why it matters: a 30th verify row with a probe lengthens that window and its output. This is a PROVISIONAL exception that the chunk's ledger-row fork would widen.

@history_item n=21 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C/TERM before its rows are checked prints no step line and no summary, only the `unable:` / `hint:` pair, and exits 1. The stop path adds no time bound. Why it matters: a new verify probe must keep the interrupted pair and add no bound on the stop path (`contract/interrupted-verify/*`, `contract/viola-verify/interrupted-pair`).

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the tui boundary after a contained vt100 feed panic) after the panic, the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a human key still reaches the child, and no viola literal reaches the outer terminal. Why it matters: this is one of `send`'s refusal paths on the bounded path. Any change to `send`'s waits must keep `contract/tui-passthrough/after-feed-panic`.

@history_item n=23 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `~ decision/no-screen-content-parsing`: the readiness predicate also reads the compiled shapes over the compiled literals. Why it matters: `send`'s readiness read, part of its longest block, parses the vendor screen only within this row. A bound or split must not widen what is parsed.

@history_item n=24 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ decision/no-screen-content-parsing` except: the test-only rig reads box-row cell attributes after a turn end or send, as closed words, never text (two keys: x, DEL). Why it matters: this is the rig reading behind the carry `cry-ujze4atl` (`rig_rehearsal_reads_the_box_empty_after_x_and_del`). If that red is met in this chunk, the mend must stay inside this exception, with no retry (`decision/zero-flakiness`).

@history_item n=25 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — every area of the master became rows, each verified against the code. Relied rows are kept or cut, and guards are re-pointed. Why it matters: this is the form every picked row takes. Any edit this chunk proposes is a `+`/`~`/`−` on a row with its reason, as the later entries show.
