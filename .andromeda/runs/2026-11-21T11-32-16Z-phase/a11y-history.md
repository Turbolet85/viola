@history_item n=1 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (1. A11y Scope Summary, 2 rows) ~ contract/tui-passthrough/human-keystrokes and ~ contract/tui-passthrough/viola-terminal-bytes: keys are never blocked and viola writes zero bytes of its own only once the wrapped child holds the terminal. The one exception is the start's automatic verify window: lines L1 to L5 and verify's own output come before the child's bytes. Why it matters: this chunk runs the cross-session sender inside `viola verify` and the automatic verify behind `verify_new_cli`, so the new probe's spawn and its window sit inside this carve-out and must not widen it. These rows are PROVISIONAL, a delegate's confirm («Confirm all 23»), until the founder's own word.

@history_item n=2 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (2. A11y Strategy) ~ decision/human-keystrokes-never-blocked gains its except: a key typed during the start's automatic verify window, before any child exists, waits in the terminal's own buffer. Why it matters: a longer verify (a print-mode sender plus Run B and its 30 s window) lengthens that window. The except must still describe it, and no new block on the human may come in. PROVISIONAL.

@history_item n=3 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (1. A11y Scope Summary) ~ contract/viola-verify/step-line: pass|fail became pass|absent|fail, with `absent` only on the browser-tools-prompt row. Why it matters: the new `cross-session-prefix` row prints its own `[NN/NN]` step line and changes the step count. This is the precedent for a ledger row's word reaching the step-line contract; the new row prints pass|fail only unless the row's promise is amended. PROVISIONAL (delegate overseer).

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair: a `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI`, and exits 1. The stop path adds no time bound, and its kill and pump join end at once. Why it matters: rows contract/interrupted-verify/* and contract/viola-verify/interrupted-pair. The new sender child and Run B must be killed at once on a stop, with no extra wait. The wording was made the founder's at 2026-10-20-t06-58-31-wrap.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the CLI verbs' strict-modes refusals as scope entities; the strict-modes refusals' text; their witnesses and pinned pairs) `viola run`, a revive start and `viola verify` over a home tree that fails the strict-modes check refuse before any log opens: exit 1, the home-state pair, no step line or summary, no child. The `HOME_PAIR` literals pin it. Why it matters: rows contract/start-refusal/home-strict-modes and decision/strict-modes-refusal-before-any-write. The probe's new spawns (the sender, Run B) must stay after that check.

@history_item n=6 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (an unstattable home, per OS) a home path under a regular file is a strict-modes refusal with its hint (verify: home-state pair, exit 1). Why it matters: row decision/unstattable-home-is-strict-modes-refusal, which verify's probe path keeps.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (strict-modes refusals, the Windows arms retired; the a11y cases' CI home on ubuntu-latest) the Windows NotFound arm is retired. Every a11y case runs in the `coverage` suite on the one `ubuntu-latest` leg, whose `gate --require coverage,doctest` is the verdict. Why it matters: rows decision/a11y-ci-github-actions, decision/zero-flakiness and decision/a11y-fail-fast-gate. The replay case of the recorded cross-session payload lands there, with no retry and no `#[ignore]`.

@history_item n=8 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (8. Cognitive Accessibility, 2 entries) ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver added the start's Claude config lock wait, a bound that falls on viola's own start. Why it matters: the probe's 30 s UserPromptSubmit window is a new bound held by verify. Both rows enumerate every bound and whom it falls on, so this is the precedent for listing a new one.

@history_item n=9 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human dropped the budget pause, and contract/cli-exit-codes/11 was retired. Why it matters: these are earlier edits to the same two bound registers. When the chunk adds the probe window, the lists must be kept exhaustive.

@history_item n=10 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver dropped the status-line deadline. Why it matters: this is the same register's edit history, and the two rows must move together.

@history_item n=11 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (7. Screen Reader Support) ~ decision/wrapped-tui-reads-as-unwrapped gains its except: the wrapped start writes the trust record and opens on the session box. Why it matters: Run B starts with the compiled `crossSessionInbound` accept settings literal. Any start-time setting that changes what the wrapped TUI shows against an unwrapped one has to be checked against this row's sameness rule.

@history_item n=12 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the screen is parsed only for compiled signatures, the prompt row only for its opening char. Why it matters: the probe reads only UserPromptSubmit and dialog presence in Run B. Reading the prefix on screen would break this ban.

@history_item n=13 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gains its except for the test-only standing live rig (closed words only). Why it matters: this chunk's live stamp runs through the rig's `S1`-class start. A diagnostic reading (the sender's tool calls by NAME, the send tool's result class) would need its own except member if it reads the screen.

@history_item n=14 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — ~ decision/no-screen-content-parsing except: the rig reads box-row cell attributes after a turn end, as closed words and never text. Why it matters: this is a precedent for widening the rig's except one reading at a time, by the founder's answer.

@history_item n=15 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — ~ decision/no-screen-content-parsing except: «after a turn end or send». Why it matters: the same except's widening history. Each new rig reading is named, never implied. PROVISIONAL.

@history_item n=16 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — ~ decision/no-screen-content-parsing except gains «or before any send», which gives the row's current except text. Why it matters: this is the current form this chunk's rig use must fit. PROVISIONAL.

@history_item n=17 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — ~ decision/no-screen-content-parsing rule adds «the compiled shapes over them», which gives the row's current rule text. Why it matters: it bounds what Run B's screen may be read for. PROVISIONAL.

@history_item n=18 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — records the takeover clause's first live reading on Linux, supplemental and never gating. Why it matters: row decision/live-readings-never-gate. The live stamp of the new row is evidence, while the CI gate is the fake agent's replay.

@history_item n=19 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading, on the product, supplemental and never gating. Why it matters: the same row, decision/live-readings-never-gate.

@history_item n=20 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the cli output-discipline checks on std::process::Command and literal pins; the §8 verification lines pinned by literal; the cli tooling) CLI a11y is checked by literal pins over `std::process::Command` on ubuntu-latest, with every fixed CLI string pinned by literal. Why it matters: rows decision/cli-a11y-by-literal-tests and decision/refusal-layout-literal-pins. The new step line's literal text needs its pin.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the exit-cause table and one hint per code) adds `no_two_codes_share_a_hint` and `no_hint_names_release`. Why it matters: rows decision/one-hint-per-code and decision/no-hint-names-release. Any new reason code or hint (for example a probe failure detail like `no-prompt`) is held by them.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the census of error writers and the body check; `unknown`'s detail; a peer's refusal screened to a code word) adds `error_bodies_each_name_a_writer_and_a_witness_that_stand`; a code word is 1 to 64 bytes of [a-z0-9-]; unlisted code words stay visible at exit 14. Why it matters: rows decision/error-bodies-have-writer-and-witness, contract/code-word/format and decision/unlisted-code-words-stay-visible. They bind any new error body or closed word the probe adds.

@history_item n=23 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour layer: SGR only on a colour-capable terminal, none under `--json` or a passthrough role. Why it matters: rows decision/no-sgr-without-tty, decision/no-sgr-in-plain-modes and decision/unable-fail-never-coloured. Verify's new step line and the automatic verify lines stay plain.

@history_item n=24 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary, and the keystroke ban covers its start. Why it matters: rows decision/single-vendor-content-boundary and decision/human-keystrokes-never-blocked. A revive start goes through the same start as `run`, so any automatic verify there carries the new probe too.

@history_item n=25 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli and tui a11y cases run as nextest cases in the `coverage` suite, gated by `gate --require coverage,doctest`, with no separate a11y driver. Why it matters: rows decision/a11y-fail-fast-gate and decision/no-soft-a11y-slo apply to the replay case's home.

@history_item n=26 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow, and the scheduled nightly carries no a11y step. Why it matters: row decision/a11y-ci-github-actions. The new row's a11y-relevant cases belong in the push/PR workflow.

@history_item n=27 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows: decision, contract and absence, with relied rows kept or cut. Why it matters: every picked row handle dates from this conversion. Later edits name rows by handle (`~`, `+`, `−`), which is the form this chunk's amendments take.
