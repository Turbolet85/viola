@history_item n=1 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the rendered `claude` screen may be parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char. This was ratified as a boundary widening (founder's F1/F2, card confirm delegate PROVISIONAL). The new browser-tools literal has to fit inside "modal literals", or this row has to be amended again.

@history_item n=2 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing except: the test-only standing live rig reads the trust selector's place, the box as empty, holds-text or absent, and modal dialogs, as closed words only, its raw frames local and gitignored. This chunk's live starts on the rig (the arm where the prompt must stand, and its control) rely on this exception and must stay inside it. The `browser-tools-prompt` modal name is one of the rig's closed words.

@history_item n=3 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (7. Screen Reader Support) ~ decision/wrapped-tui-reads-as-unwrapped except: in an untrusted folder a wrapped `claude` opens on its session box, because viola wrote the trust record and typed no key; an unwrapped one opens on the trust form. This is the precedent next door: the first screen a wrapped start meets, with viola typing nothing into the vendor screen. A standing browser-tools modal is another first-screen state, and it is never typed into either.

@history_item n=4 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (8. Cognitive Accessibility, 1. A11y Scope Summary) ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver: the start's Claude config lock wait is added, falling on viola's own start. ~ decision/run-diagnostics-off-terminal: the trust step reports in one `trust-record` diagnostics line, never as a refusal, exit code or hint. If the gate waits out its bound at a standing modal, or verify reads it, that bound and any record word must follow these rows: never on the human, never on the terminal.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — §8, §3 and §4 P4: a verified `viola send` waits in the readiness gate for at most `GATE_MAX_WAIT` (8.5 s), then is refused `not-delivered` / `input-not-ready` with nothing typed. A human key during the wait takes the wheel and the send is refused `human-typing`, held by `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records and never on a timer. This is the gate this chunk extends. Its refusal at a standing browser-tools modal must keep this bound on the driver and keep the human-key-wins clause (rows decision/human-key-takes-the-wheel, contract/send-refusal/input-not-ready).

@history_item n=6 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the tui passthrough boundary after a contained vt100 feed panic) This adds the boundary entity: `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a human key still reaches the child, and no viola literal reaches the outer terminal. It is the precedent for a gate refusal that writes nothing to the terminal (rows contract/tui-passthrough/after-feed-panic, vt100-feed-panic). A modal-standing refusal must keep the same zero-viola-bytes shape.

@history_item n=7 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — The non-editing set became the closed list of focus reports, mouse reports and terminal replies, with every other byte counting as editing. Case (3) of the boundary wheel was landed. This is the origin of rows decision/non-editing-sequences and decision/stdin-classifier-non-editing-list. Any byte a rig or the fake agent types at the modal counts as editing, and the recommended number is none.

@history_item n=8 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — The closed non-editing list was extended by exact grammar, and the first live reading of the takeover clause was recorded, supplemental and never gating. This is the origin of decision/live-readings-never-gate. This chunk's live starts on the rig are readings of the same kind and cannot become CI gates.

@history_item n=9 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — A second live takeover reading was recorded, on the product binary over a live `claude` 2.1.296, supplemental and never gating. It is the precedent for how a live reading on the real CLI is recorded in the keyboard harness without gating.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the tui boundary on openpty alone; the a11y cases' CI home) The TUI is hosted in openpty only. Each boundary clause is its own nextest case on `ubuntu-latest`, and the zero-viola-bytes case is compared byte for byte with an unwrapped fake-agent run. The fake-agent browser-tools screen cases land on that one leg inside the `coverage` suite.

@history_item n=11 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (cli checks on std::process::Command and literal pins; §8 verification pinned by literal; portable-pty =0.9.0) Every fixed CLI string and refusal layout is pinned by literal over `std::process::Command`, and the outer-PTY driver is portable-pty `=0.9.0`, inherited and never re-pinned. Any new fixed verify step line or refusal text this chunk adds (the 24th row's step) takes a literal pin. The tui driver stays as pinned (rows decision/fixed-cli-strings-pinned, decision/tui-boundary-driver-pin, decision/portable-pty-outer-pty-driver).

@history_item n=12 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — Verify's interrupted pair (`unable: verify was stopped before it finished`, then `hint: run viola verify again to stamp this CLI`) was added, with exit 1 and no step line or summary, and the stop path adds no time bound. This chunk adds a verify probe run and changes the row count from 23 to 24. The stop with the modal standing must keep this pair and its no-bound rule (rows contract/interrupted-verify/*, contract/viola-verify/interrupted-pair).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the CLI verbs' strict-modes refusals as scope entities; their text) `viola run`, a revive start and `viola verify` over a failing home tree refuse before any log opens: exit 1, the home-state pair, `verify` printing no step line or summary. This is the origin of contract/start-refusal/home-strict-modes. Verify's new probe must sit after this refusal and leave it unchanged.

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — A passed `viola revive` enters the same TUI passthrough boundary as `run`, and it writes no line of its own while the child holds the terminal. The modal reading at the pre-send gate binds a revived start too (rows decision/single-vendor-content-boundary, contract/tui-passthrough/own-bytes, decision/human-keystrokes-never-blocked).

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`--json`'s one document and its exit-2 exception; `unknown`'s detail; the census of error writers) A `--json` failure is one document with no hint, except the `viola answer` / `viola send` exit-2 paths. `unknown` (exit 14) has a code word or no detail and no hint. `thiserror_messages_interpolate_nothing` and the writer census hold every error body. A gate refusal at the modal travels these paths and must keep within them (rows contract/cli-refusal/json, decision/json-one-document-no-hint, decision/thiserror-messages-interpolate-nothing, decision/refusal-ends-with-hint).

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the exit-cause table and one hint per code) `no_two_codes_share_a_hint` and `tests/cross_exit_causes.rs` hold each code to its own hint. If the modal-standing refusal gets a detail code of its own (or reuses `input-not-ready`), the one-hint-per-code guard and the table bind it (row decision/one-hint-per-code).

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human dropped the budget pause, and exit 11 is retired and never reused. These are the history of the two time-bound rows this chunk rests on. A retired exit code is never reused for a new modal refusal.

@history_item n=18 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver dropped the status-line deadline. They are also the history of the time-bound rows: each bound a chunk adds or removes is written into both rows in the same wrap.

@history_item n=19 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — Every area of the plan became rows (decision, contract, absence), each verified against the code. This is the baseline the picked rows come from. Changes this chunk owes are row edits (`~` / `+` / `−`) to them.

@history_item n=20 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — The colour spans were built and checked, each beside its word, with none under `--json`, a passthrough role or non-TTY. This is the origin of decision/sgr-beside-its-word and decision/no-sgr-off-terminal. Verify's new step line and any refusal stay uncoloured (`unable` and `fail` never coloured).
