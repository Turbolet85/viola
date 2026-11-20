@history_item n=1 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — §8 Timeout extensions: a raised dialog takes `viola answer` only within the deadline; on expiry the hook exits 0 with no body, the dialog renders for the human in the `claude` TUI, and a later `answer` is refused `not-delivered` / `unknown-dialog` (exit 13). This is the origin of the fail-open fall-back this chunk must keep for a PermissionRequest question whose hold expires (decision/dialog-hold-bounds-the-driver, contract/cli-exit-codes/13).

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the deadline became the configurable dialog hold (`dialog_hold_s` 60–3600, default 60, `hooks.json` timeout 3615 s, hook bound 3605 s); at the hold's end the wrapper replies `null` and appends one `dialog-expired` line; any human key hands a held dialog back at once. The new `allow` + `updatedInput` answer must sit inside this hold and leave the human's key and the expiry `null` unchanged (contract/config-json/dialog-hold-s, contract/event-line/dialog-expired).

@history_item n=3 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the dialog hold bounds owned by 0.2.0's first entry) — `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` stay PROVISIONAL with an owner; it records that an unverified CLI replies `null` to a dialog at once. That is the unverified-cli gate this chunk keeps: the question answer stays gated on a verify stamp counting every relied row, the new row included.

@history_item n=4 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/dialog-hold-bounds-the-driver moved from provisional to standing, the dialog read on screen after expiry. The row this chunk rests on for the human-always-wins boundary is ratified, so the chunk may not widen the hold or change the expiry fall-back without a new ruling.

@history_item n=5 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — ~ contract/viola-verify/step-line: `[NN/NN] … pass|absent|fail`, `absent` only on the browser-tools-prompt row (PROVISIONAL, delegate). This is the precedent for a new ledger row getting its verify probe: the new PermissionRequest-question row prints a step line, and only the browser-tools-prompt row may print `absent` unless this row is amended.

@history_item n=6 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C or TERM prints no step line or summary, only the fixed `unable:` / `hint:` pair, exit 1; the stop path adds no time bound. A new probe (possibly with a live wait) must keep that pair and stop behaviour (contract/interrupted-verify/*, contract/viola-verify/interrupted-pair).

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities; the plugin against a strict-modes-refused instance) — `verify` over a failing home tree refuses before any log with the home-state pair, no step line; the hook dialog call against a refused instance returns exit 0 at once with empty streams and writes no human-facing byte. The probe's growth and the hook's PermissionRequest question path must keep both (contract/start-refusal/home-strict-modes, contract/hook-call/strict-modes-failed).

@history_item n=8 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the plugin against an impostor wrapper writes no human-facing byte) — first entity detail for the not-assertable plugin: the `viola hook` dialog call meeting an unverified server sends no frame and fails open (exit 0, empty streams, under 1.0 s); MCP `answer` returns `isError`. The PermissionRequest question relay runs through this hook dialog call, so its new `allow` body must not alter the fail-open (contract/hook-call/server-verify-failed, decision/no-assertions-for-plugin-and-ipc).

@history_item n=9 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the eight CLI frame verbs' server-verify refusal) — `answer` is among the eight verbs refused exit 21 with the server-verify hint before any frame; if this chunk changes `answer`'s shape, its refusal surface stays as recorded (contract/frame-verb-refusal/server-verify-failed).

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (socket-directory refusals at the start, the frame verbs, hook and MCP; the tui boundary's drop of a refused peer) — `answer` and the hook dialog call refuse a failing socket directory before any frame (exit 21 / fail-open with a WARN `hook-decision`); a refused peer writes nothing to the terminal and moves no wheel. Rows this chunk's answer and hook paths must keep (contract/frame-verb-refusal/socket-dir-refused, contract/hook-call/socket-dir-refused, contract/tui-passthrough/refused-peer).

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`--json`'s one document and its exit-2 exception; `unknown`'s detail; a peer's refusal screened; a fault's body; the census of error writers) — `viola answer`'s exit-2 path writes the fixed `UNUSABLE_RESPONSE` line and no document whatever `--json` says; peer replies are screened to code words; `wrapper fault` is literal-pinned; every error body names a writer and a witness. If the answer's shape grows to carry question answers, an unusable response must keep this exit-2 form and any new error writer must join the census (contract/cli-json-failure/send-answer-exit-2, decision/error-bodies-have-writer-and-witness).

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's `--json` cause; MCP's `isError` cause; the `internal-error` document; one hint per code) — exit 21 and MCP `isError` name their closed cause in `detail`; a fault under `--json` prints the `internal-error` document; `no_two_codes_share_a_hint` and `no_hint_names_release` hold the catalogue. Any new refusal or code this chunk adds to `answer` or verify must take its own hint and a code word (decision/one-hint-per-code, decision/no-hint-names-release).

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the §8 verification lines pinned by literal; the cli output-discipline checks on std::process::Command) — every fixed CLI string, hints and result lines, is pinned by literal over `std::process::Command`, typed exit codes and a hint-free `--json` included. A new verify step line or answer string must be literal-pinned the same way (decision/fixed-cli-strings-pinned, decision/refusal-layout-literal-pins).

@history_item n=14 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — a human key during a driver wait takes the wheel at once and the waiting automation is refused `human-typing`; the case is sequenced on log records, never a timer. This is the precedent for "the human always wins" through any driver wait, which the question answer path must keep when the human answers the dialog himself (decision/human-key-takes-the-wheel, decision/boundary-tests-sequenced-on-records).

@history_item n=15 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the non-editing set became the closed list of focus/mouse reports and terminal replies, every other byte editing. A human answering the question dialog in the TUI is typing and takes the wheel; the chunk must not add any byte class to the non-editing list (decision/non-editing-sequences, decision/stdin-classifier-non-editing-list).

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the non-editing list's terminal replies extended by exact grammar, ratified; one live takeover reading recorded as supplemental, never gating. Constrains the classifier this chunk must leave alone, and frames any live reading of the question answer as non-gating (decision/stdin-classifier-non-editing-list, decision/live-readings-never-gate).

@history_item n=17 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading, on the product over a live `claude`, supplemental and never gating. Precedent for how a live start this chunk may shape at P3 records its reading (decision/live-readings-never-gate).

@history_item n=18 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the screen is parsed only for compiled signatures, modal literals among them. Any probe or rig reading of the question modal must stay within the compiled signatures.

@history_item n=19 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gained the test-only rig exception: it reads modal dialogs as closed words only. The verify probe for the new row, or a rig reading of the question dialog, must stay inside that exception.

@history_item n=20 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — ~ decision/no-screen-content-parsing except: rig reads closed words, never text, box-row attributes after a turn end. Part of the chain the question dialog's reading must respect: option and answer text is content, never read as text.

@history_item n=21 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — ~ decision/no-screen-content-parsing except widened to "after a turn end or send" (PROVISIONAL, delegate). Same chain; any new rig reading this chunk needs is a further widening of this except, needing its own word.

@history_item n=22 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — ~ decision/no-screen-content-parsing except gained "or before any send" (PROVISIONAL, delegate). The except as it stands in the picked row; a question-modal reading beyond "modal dialogs" as closed words would widen it again.

@history_item n=23 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — ~ decision/no-screen-content-parsing rule gained "the compiled shapes over them" (a modal label row's option shape), PROVISIONAL until the founder's word. The latest change to the row; the question dialog's option rows are such shapes, read for shape only, never text — the scope's "option and answer text is content" rests on it.

@history_item n=24 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (time-bound rows; run diagnostics; wrapped TUI reads as unwrapped) — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver list the start's Claude config lock wait; ~ decision/run-diagnostics-off-terminal adds the `trust-record` line; ~ decision/wrapped-tui-reads-as-unwrapped gains the trust-form exception. Any new wait in the question path joins these lists, falling on the driver; the question modal must read to the human as unwrapped.

@history_item n=25 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ the two time-bound rows dropped the budget pause; ~ contract/cli-exit-codes/11 retired; ~ decision/state-carried-by-printed-word. Shows how the time-bound rows are kept as exact lists, which a new bound in this chunk must amend the same way.

@history_item n=26 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ the two time-bound rows dropped the status-line deadline; − decision/statusline-output-passed-byte-for-byte. Same exact-list precedent for decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver.

@history_item n=27 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` cap (`WAIT_CAP` 50 s) recorded as a driver-only bound. If MCP `answer` changes shape to carry question answers, its bounds stay on the driver like this one.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone; the a11y cases' CI home; strict-modes Windows arms retired) — the TUI is hosted in openpty only; each boundary clause is its own nextest case on `ubuntu-latest` inside the `coverage` suite; the unstattable home is a strict-modes refusal with no per-OS oracle. Constrains where any new a11y case of this chunk runs.

@history_item n=29 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0) — the tui boundary driver is portable-pty `=0.9.0`, inherited, never re-pinned by a11y (decision/tui-boundary-driver-pin, decision/portable-pty-outer-pty-driver).

@history_item n=30 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — the passthrough boundary still holds after a feed panic: the driver's action refused, the human key reaching the child, no viola literal on the terminal. Precedent that a driver-side failure on the dialog path writes nothing to the human's terminal (contract/tui-passthrough/own-bytes).

@history_item n=31 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows, each verified against the code; the picked rows' handles and their relied/kept fates come from it, so this chunk amends by row handle (`+` / `~` / `−`), the new ledger row's a11y reach (if any) a row of its own.
