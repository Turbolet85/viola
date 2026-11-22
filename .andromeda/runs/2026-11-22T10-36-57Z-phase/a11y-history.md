@history_item n=1 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (rx3cmhmn, §8) — `~ decision/time-bounds-fall-on-the-driver` gained the MCP call deadline, and `+ contract/instance-unreachable-detail/call-deadline` added call-deadline as the seventh MCP-only unreachable cause. The CLI's exit-21 set stays six, and the outcome of a send is unknown. This chunk's paste bound has to fit under that 55 s deadline. If it adds its own closed word, the word extends this register and the time-bounds list the same way.

@history_item n=2 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (4urjbp6k, §8) — `~ decision/time-bounds-never-on-human` gained the MCP call deadline. This is the direct precedent: a new bound viola holds is added to this list, and the entry states whom it falls on. The paste bound must fall on the driver and must never block, refuse or delay a key.

@history_item n=3 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — Set the readiness gate's `GATE_MAX_WAIT` (8.5 s), which ends in not-delivered / input-not-ready with nothing typed. The bound falls on the driver only. A key pressed during the wait takes the wheel and the send is refused human-typing. The rule is "the human always wins" through a wait of seconds. The outer-PTY case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` is sequenced on records, never on a timer, and its stated limit is that it cannot tell send's first wheel read from its second. This is the template for what human keys do while a paste is blocked or after it is cut. It also gives the gate's bound that the paste bound's arithmetic must count.

@history_item n=4 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Added the confirm windows `CONFIRM_WINDOW_FALLBACK` (10 s) and `COMPACT_CONFIRM_WINDOW` (40 s), which end in not-delivered / no-prompt-submitted, a bound on the driver only with the wheel untouched. These windows count in send's total against the MCP call deadline. A cut paste that leaves text with no Enter meets this confirmation path.

@history_item n=5 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — MCP `wait` clamps to `WAIT_CAP` 50 s, a driver-only bound that sits below the client's tool-call timeout. It is a precedent for keeping viola's bounds below the MCP client's limit, and for a driver-only bound that touches no key and leaves the wheel alone.

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — The first entry to put a time bound on the driver and never on the human (§8 Timeout extensions). On expiry viola fails open and the vendor TUI is left to the human. It sets the pattern this chunk's bound follows.

@history_item n=7 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — Made the dialog hold configurable and kept it on the driver: any human key still hands the dialog back at once, whatever the hold. No human-facing time limit was added, which keeps `absence/user-time-limit` and `decision/no-unwarned-timeouts` intact. A paste bound must not add one either.

@history_item n=8 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver` moved to standing after a live measurement. The precedent: a bound applied ahead of ratification stays provisional until it is measured.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Added the 5 s `claude agents --json` read. It fails open and falls on viola's own child, never on a key. This is the "agents read" member of the time-bounds rows the chunk extends.

@history_item n=10 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — Added `STATUSLINE_DEADLINE` and stated whom it falls on, because the plan lists every bound viola holds and whom it falls on. The bound was later retired, but this is the origin of the register that a paste bound joins.

@history_item n=11 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — The budget pause was a driver-only refusal of send with exit 11 and nothing typed, and its keyboard case read that a human key reaches the child while no `unable`, `hint:` or code reaches the outer PTY. The exit is now retired (`contract/cli-exit-codes/11`). It is a precedent for a new send refusal word, and a warning not to reuse exit 11.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4e3yglqx, §8) — `~ contract/cli-exit-codes/11` is retired and never reused (an old budget-paused reads unknown, exit 14). `~ decision/time-bounds-fall-on-the-driver` dropped the budget pause. A new paste-bound word must not take exit 11, and an unlisted word reads unknown/14 (`decision/unlisted-code-words-stay-visible`).

@history_item n=13 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (cjoqaris, §8) — `~ decision/time-bounds-never-on-human` dropped the budget pause. This is the row the paste bound will join.

@history_item n=14 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (jywabuor, 4s4vfdjh, §8) — Both time-bounds rows (`decision/time-bounds-never-on-human`, `decision/time-bounds-fall-on-the-driver`) dropped the status-line deadline. This shows how the two parallel rows are edited together whenever a bound is added or removed.

@history_item n=15 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (sobg3z7r, avf5o2uj, §8) — Both time-bounds rows gained the start's Claude config lock wait and "viola's own start" as a place a bound may fall. If the paste bound falls on the wrapper rather than on the driver or the child, the "falls on" clause must be extended the same way.

@history_item n=16 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (5s4as3gs, §1) — `~ decision/run-diagnostics-off-terminal` added that the trust step reports one `trust-record` line in `diagnostics/`, never as a refusal, exit or hint. The paste cut's wrapper-side record must likewise go to `diagnostics/`, never to the terminal.

@history_item n=17 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (o2edema5, §1) — `~ contract/tui-passthrough/human-keystrokes` says keys are never delayed "past the current atomic paste once the wrapped child holds the terminal", and `~ contract/tui-passthrough/viola-terminal-bytes` says zero viola bytes once the child holds the terminal. A blocked paste stretches "the current atomic paste", so the bound decides how long a key may wait behind it. A cut paste must still write no viola byte.

@history_item n=18 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (okhtl4vb, §2) — `~ decision/human-keystrokes-never-blocked` names the start's automatic verify window as its one exception, PROVISIONAL. A paste bound must not add a second exception without a ratified widening.

@history_item n=19 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — Defined the closed non-editing set: focus reports, mouse reports and terminal replies, with every other byte editing. The wheel case is sequenced step by step. This is the basis of the wheel-unmoved claim at the bound and of which keys take the wheel during a blocked or cut paste (`decision/non-editing-sequences`, `decision/wheel-ignores-terminal-events`).

@history_item n=20 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — Widened the closed list to exact grammars, so one-field and one-prefix neighbours stay typing (`decision/stdin-classifier-non-editing-list`). It also recorded the first live takeover reading, which is supplemental and never gating (`decision/live-readings-never-gate`).

@history_item n=21 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — A second live takeover reading: a human key writes `wheel{human, human-input}` and the next send exits 10 human-typing. It stays supplemental, so this chunk's no-live-start boundary loses no gating evidence.

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (qihirloo) — Added the feed-panic boundary entity: send is refused not-delivered / input-not-ready (exit 13) while a typed key still reaches the child, and no `panic`, `unable` or `hint:` reaches the outer terminal. This is a precedent for a send that ends at a wrapper-side failure with a closed word, the human key unaffected and the terminal clean (`contract/tui-passthrough/after-feed-panic`, `contract/tui-passthrough/vt100-feed-panic`).

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (t4neqrve) — The TUI is hosted in openpty alone. The zero-viola-bytes case compares byte for byte with an unwrapped fake-agent run, and each of the three boundary clauses is its own nextest case on ubuntu-latest. A paste-cut keyboard case lands under these clauses on the fake agent.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (ob6ilhui, hcbqnaev) — The a11y cases run in the `coverage` suite on the one ubuntu-latest leg, and the Linux-only platform line was added (`decision/linux-only`). Any new case's CI home and reach is that leg.

@history_item n=25 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — A passed revive enters the same TUI passthrough, and the keystroke ban covers a revived start. A paste bound in the shared send/pump path binds revived starts too (`decision/single-vendor-content-boundary`, `decision/human-keystrokes-never-blocked`).

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (ruw2kqsf) — Exit 21's `--json` document names its cause in `detail` from a closed set (`contract/instance-unreachable-json/document`). The precedent: a new cause word is the founder's word and needs a widening shown. A paste-cut word added to the closed sets follows that path.

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (plofoeqd) — The MCP `isError` body is the CLI's document less `v`, so a cause crosses between CLI and MCP. The MCP `send` tool's result at the bound must carry the same closed word as CLI `send`.

@history_item n=28 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (dh4gr6sw) — A wrapper lost with the call open reads `endpoint-lost`. That word is a neighbour the paste-cut outcome must be told apart from.

@history_item n=29 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (nx4hgeqm, mmjwn3vl) — Added the exit-cause table `tests/cross_exit_causes.rs`, `no_two_codes_share_a_hint`, `no_hint_names_release`, and hints pinned by `send_hint_is_the_design_string`. A new refusal or detail word needs its own hint (no release in it), a row in the table and a literal pin (`decision/one-hint-per-code`, `decision/no-hint-names-release`).

@history_item n=30 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (r5zwekpk, muk7fxfb) — Added the catch site's `internal-error` document under `--json` (`contract/json-fault/internal-error`). A `pump_child` error prints no document once the child held the terminal, and viola writes nothing past it. This bounds how a pump-side write failure may surface.

@history_item n=31 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (wjuy56h4) — Raised exit 21's closed causes to six with `socket-dir-refused`, the founder's own word after the widening was shown (`contract/frame-verb-refusal/socket-dir-refused`). This is the latest precedent for adding a closed word to the cause set the CLI keeps at six.

@history_item n=32 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (likmmngr, uksyocnq) — MCP frame tools return `isError` with code and detail, no frame and no line. A refused peer is dropped and moves no wheel and writes nothing to the terminal. These are precedents for the MCP send result and the wheel-unmoved claim at a wrapper-side end.

@history_item n=33 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (hv7cbqdf) — `viola run`'s socket-directory refusal joined the fixed-message `unable:` form (`contract/cli-refusal/unable-line`). This is an exception to the two-field refusal line, which matters if send's new refusal takes a non-standard form.

@history_item n=34 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (ahrrqlje, xzhtjnkz) — A peer's refusal that is no code word reads `unknown`, and a detail that is no code word reads nothing. A code word outside the closed list stays visible with exit 14 (`decision/peer-replies-screened-to-code-words`, `decision/unlisted-code-words-stay-visible`, `contract/code-word/format`). The wrapper's paste-cut word must be listed and well formed, or an older CLI or driver reads it as unknown/14.

@history_item n=35 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (6kt7moqw, 5uz5ioef) — Under `--json` one document with no hint, except send's and answer's exit-2 paths. The failure-document witness `cli_error_documents_hold_no_path_or_upstream_text` requires no upstream text. The send's end at the bound must be a closed word and never the write's io error text (`decision/json-failure-documents-are-clean`, `contract/cli-refusal/json`).

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (ty5ro7y6, c3je5vt2) — Added the census of error writers (`error_bodies_each_name_a_writer_and_a_witness_that_stand`, `thiserror_messages_interpolate_nothing`) and the `wrapper fault` body. A new error variant for the cut paste must be named in the census with a witness, and must interpolate nothing (`decision/error-bodies-have-writer-and-witness`, `decision/thiserror-messages-interpolate-nothing`).

@history_item n=37 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (amaq76hw, qerkfruc) — Added the frame verbs' exit-21 server-verify refusal and the MCP tools' `isError` with no frame (`contract/frame-verb-refusal/server-verify-failed`). It is the closest refusal register on send's path that the new word will sit beside.

@history_item n=38 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (mnwtaiyh) — Widened the frame-verb server-verify refusal to eight verbs, which is the current shape of the picked frame-verb refusal row.

@history_item n=39 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (nx5xxktu) — Added the frame verbs' strict-modes refusal before any frame or log, with `send` refused after its text read (`contract/frame-verb-refusal/strict-modes-failed`). It fixes where on send's path refusals sit relative to the paste.

@history_item n=40 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — Added verify's interrupted `unable:` / `hint:` pair, one of the named exceptions in `contract/cli-refusal/unable-line`, the row any new send refusal line is checked against.

@history_item n=41 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (xfp5zvcf, dhjwvlu4, 7iu65p4v) — CLI output discipline is checked by `std::process::Command` with literal pins, which hold the `unable` / `hint:` layout, the typed exit codes and a hint-free `--json` (`decision/refusal-layout-literal-pins`, `decision/cli-a11y-by-literal-tests`, `decision/cli-output-discipline-assertion-tools`). A new send refusal takes literal pins, not a trycmd case.

@history_item n=42 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (wojncy7h) — The outer-PTY driver is portable-pty `=0.9.0`, which a11y never re-pins (`decision/portable-pty-outer-pty-driver`). A paste-blocking keyboard case uses that driver as is.

@history_item n=43 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — The output-discipline invariants held in crate tests are covered by the epoch-boundary `run --mutants` audit (`decision/mutation-audit-covers-crate-invariants`). This fits the chunk's no-mutation-run boundary, which leaves the audit to the epoch.

@history_item n=44 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — The cli / tui cases run as nextest cases in the `coverage` suite, and the zero-flakiness parallel bans `#[ignore]` (`decision/zero-flakiness`). A test that waits on a blocked paste must be sequenced on records, take no retry and run under the 20 s lint.

@history_item n=45 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — The tier `Standard (1)` now rests on the terminal surfaces alone, and output discipline plus the TUI boundary stand (`decision/a11y-tier-standard`, `decision/output-discipline-stands-in-for-sc`).

@history_item n=46 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — The founder kept the tier at `Standard (1)`. The tier row the chunk rests on is ratified, not provisional.

@history_item n=47 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — The colour decision applies none under `--json` or a passthrough role (`decision/no-sgr-off-terminal`, `decision/no-sgr-in-plain-modes`). A new send refusal line must carry no SGR, and `unable` is never coloured.

@history_item n=48 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — The masters became rows, each verified against the code. Every picked row's wording dates from here unless a later entry above changed it.

@history_item n=49 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — Retired the frontend stubs to N/A because viola has no web or graphical frontend. This explains the plan's absences, so the chunk has no web a11y surface to plan.

@history_item n=50 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing` covers the input box's rule and prompt rows, and the prompt row only for its opening char. Disposing of what a cut paste leaves in the input box (text with no Enter) may read the box only through these compiled signatures, never its text.

@history_item n=51 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` gained the test-only rig exception: input box empty, holds-text or absent, read as closed words. "Holds-text" is the closed reading a cut-paste check could use without reading text.

@history_item n=52 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — `~ decision/no-screen-content-parsing` except: box-row cell attributes after a turn end, as closed words and never text. It is one step of the exception's growth.

@history_item n=53 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ decision/no-screen-content-parsing` except: cell attributes after a send too. The pattern: each new rig reading must be named in the exception before it lands.

@history_item n=54 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — `~ decision/no-screen-content-parsing` except: cell attributes before any send. Any rig reading this chunk adds (none is planned, since there is no rig start) would extend it the same way.

@history_item n=55 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `~ decision/no-screen-content-parsing` rule: the compiled shapes over the literals. This is the current boundary of what send's readiness and confirmation may parse.

@history_item n=56 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** — `~ decision/no-screen-content-parsing` except: the KR resize reading. This is the row's current text from the previous chunk, whose carry (cry-mui36h42, the viola-pty test comment) rides into this one.
