@task_prose part=chunk
Closes viola-0.1.0: every gate green on Linux (fmt, clippy, check, test, cargo deny, the deny probes, the tokio-free and release checks, CI's seven jobs), the red supply-chain job fixed by moving portable-pty off =0.8.1 to a release whose graph drops pin-utils, no ignore-list widening.
Removes or keeps with a reason each Windows remnant in code (tests/cli_output_plain.rs, viola-agent-claude, the viola-e2e harness EXE_SUFFIX joins, plain_drive, the coverage-floor comments, host.rs's windows key), the masters following.
Makes the masters agree with the code and each other: decides tool or text for the trycmd / assert_cmd pinned-string claims (no manifest holds either, no tests/cmd/ exists, re-measured at HEAD), the error-enum and thiserror census, the Occupied Resources markers, the obs-plan process-start row.
Re-measures live on Linux, within two headless live sessions on the pty rig: v1-53's one viola run under a driver, a dialog hold near the 3600 s ceiling, the channel-handle no-leak property under the fake agent.
Rules each of the 17 unclaimed caps proved or deferred on the founder's word, and records claims and deferrals in the verification matrix; no new verb, flag, env var, channel method or product surface.
@task_file n=1 path=Cargo.toml mark=edits
what: the `portable-pty` dependency pin (off `=0.8.1`)
@task_file n=2 path=Cargo.lock
@task_file n=3 path=crates/viola-pty
@task_file n=4 path=deny.toml mark=re-reads
what: `cargo deny check advisories` reading RUSTSEC-2026-0441 gone, with no ignore-list widening
@task_file n=5 path=tests/cli_output_plain.rs mark=edits
what: the Windows remnant lines 104 and 116
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the Windows remnant at line 2
@task_file n=7 path=crates/viola-e2e/src/harness/session.rs mark=edits
what: the Windows remnant at line 52 and the `EXE_SUFFIX` joins
@task_file n=8 path=src/cmd/mod.rs mark=edits
what: `plain_drive`
@task_file n=9 path=tests/contract_os_gated_tests.rs mark=re-reads
what: the OS-gated census (39 OS-gated test functions, 2 gated files) that v1-53's claim reads
@task_item n=1 part=needs
The a11y plan's own claims that trycmd / assert_cmd pin the fixed CLI strings: the harness tool pick and keyboard harness (contract keys a11y-testing-tool-pick and keyboard-test-harness), the per-surface reach table's cli row naming assert_cmd 2.2.2, predicates 3.1.4 and trycmd 1.2.1, the CI stage table row, the strategy pyramid's Understandable and Cognitive rows, and the P2 / P6 path scenarios; item 1 decides tool or text for each, so their wording may change here.
@task_item n=2 part=needs
The cognitive-accessibility verification lines that name trycmd: error recovery's per-refusal layout pin, plain language's pinned fixed strings (trycmd or literal), and consistency's verification source for the CAPTION order; each is re-worded or kept by item 1's decision.
@task_item n=3 part=needs
The reach table's tui row naming the portable-pty outer-PTY boundary driver as inherited `=0.8.1`, and the cli row's portable-pty outer-PTY SGR checks: once the pin moves (item 12) the plan's version text follows the new release.
@task_item n=4 part=needs
The tui boundary assertions run on the outer-PTY driver that the portable-pty bump changes: zero viola-originated bytes, human keystrokes delivered unblocked past the current atomic paste, focus / mouse / resize sequences and terminal replies not moving the wheel (path P4, its procedure in the ARIA detail); they must stay green on ubuntu-latest after the bump.
@task_item n=5 part=needs
The CLI SGR discipline asserted under the outer PTY, which the bump also reaches: no SGR of viola's own under non-TTY, NO_COLOR, TERM=dumb, --json, viola run or a passed revive; amber only on DIALOG, bold only on NAME values, every SGR span beside its word; dim only on stale rows, proven at the unit tier and under the outer PTY (path P6 and the visual SGR detail).
@task_item n=6 part=needs
The plain-mode output discipline that tests/cli_output_plain.rs and plain_drive carry: removing their Windows lines must keep the non-TTY / NO_COLOR / TERM=dumb assertions (no SGR, C0/C1 escaped except newline and tab, linear static output) at their coverage.
@task_item n=7 part=needs
The viola run TUI passthrough the live sessions use (v1-53, the hint's driver-drives-a-builder session): viola writes zero bytes to the terminal but the child's output, its run diagnostics go to diagnostics/, human keystrokes are never blocked, refused or delayed; the live reading is a boundary reading only.
@task_item n=8 part=needs
The ban on parsing the rendered claude TUI screen or taking screenshots as evidence: the live readings this chunk makes (v1-53's run, the dialog hold, the wheel and tab-close legs) rest on viola's own outputs and the JUnit, never on screen content.
@task_item n=9 part=needs
The wheel's non-editing rule (P4 and the Keyboard anti-pattern: focus, mouse, resize sequences and terminal replies never count as human editing): item 10's hypothesis that a terminal answering CSI ? 6 n or the kitty graphics query loses the wheel at start tests exactly this rule; a shape joins the list only when measured, by its exact grammar, on the founder's word.
@task_item n=10 part=needs
The timeout rule in cognitive accessibility: the dialog hold is one of viola's time bounds that falls on the driver, never on the human, and no key is blocked by it; reading a hold near the 3600 s ceiling live (item 2) must keep that, and the plan's statement of the hold follows if the ceiling leaves PROVISIONAL.
@task_item n=11 part=needs
The CI integration the green-on-Linux clause reads: one ci.yml on the matrix [ubuntu-latest], the CLI output-discipline and TUI boundary cases inside the test job's single instrumented coverage suite, whose gate --require coverage,doctest allows no skip; failures land in the harness run JSON suites[].failures[].
@task_item n=12 part=needs
No WCAG conformance claim for the cli or the viola run TUI: the matrix claims this chunk writes for the caps it proves (v1-26's fix-pass surfaces, v1-53, v1-01 and the rest) cite assert_cmd / portable-pty results as output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, not as conformance.
@task_item n=13 part=needs
The frontend deferred to 0.2.0 with its technology not chosen: a cap whose acceptance names a surface 0.1.0 does not build (the page, feed or strip bay) is deferred only on the founder's word, and the plan's deferral statement is the reason text it cites.
@task_item n=14 part=needs
The CLI board path P6 where item 4 reaches: v1-36's link set on the board, deferred or re-worded, must leave the CAPTION literal's order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI and the board's parity with the deferred 0.2.0 page undrifted.
@task_standing n=1
convention: each new test that asserts CLI output discipline or the TUI boundary runs inside nextest-integration or nextest-e2e locally and, in CI, inside the test job's coverage suite on ubuntu-latest; no new suite enum value and no separate a11y driver outside scripts/agent-run.sh run without a Decisions Log entry
home: ## 9. CI Integration and ## 11. A11y Anti-Patterns › ### CI
@task_standing n=2
convention: each new test meets zero flakiness: retries 0, no #[ignore], no retry-once policy; a flake keeps the chunk red until its root cause is fixed
home: ## 10. SLO Invariants & A11y Budgets and ## 11. A11y Anti-Patterns › ### SLO
@task_standing n=3
convention: each new a11y-bearing check yields machine output in the harness run JSON; a manual screen-reader or keyboard pass is a supplement, never the only evidence, and no a11y artifact of its own is uploaded
home: ## 9. CI Integration and ## 11. A11y Anti-Patterns › ### Universal (agent-driven specific)
@task_item n=1 part=not_needed
The per-surface landmark inventory (BAY context line, caption line, unwrapped separator, --help group labels as structural equivalents): the chunk builds no surface and changes none of these.
@task_item n=2 part=not_needed
The screen-reader test pattern table and the manual Orca passes (Orca with GNOME Terminal, Orca with foot on Hyprland) and the screen-reader-priority trigger: founder-owned supplements; this chunk's live sessions are headless with no human key.
@task_item n=3 part=not_needed
The compliance cross-check and the not-triggered wcag-mapping and regulated-compliance entries: no regime applies and the chunk changes nothing about it.
@task_item n=4 part=not_needed
The not-triggered cognitive-accessibility, motion-sensitive, multi-language and target-size entries, and the reading-level target (SC 3.1.5 not claimed): the chunk adds no output, language or touch surface.
@task_item n=5 part=not_needed
Keyboard navigation for the cli (no prompt, pager or redraw; input by stdin or --file): the chunk adds no CLI input path.
@task_item n=6 part=not_needed
The viola revive --json refusal entity (exit-1 documents such as pinned-hash-mismatch and squatted-name, no document past a pump_child error): the chunk touches no revive start path.
@task_item n=7 part=not_needed
Path P3's unverified-cli withholding text on viola answer: the chunk changes no answer path; its live session only uses the verb.
@task_item n=8 part=not_needed
The per-verb refusal wording in error recovery (the unknown exit-14 no-hint case, the hint-free --json documents, the answer and send exit-2 stderr lines): the chunk adds and changes no refusal; only the verification wording is in reach.
@task_item n=9 part=not_needed
The not-assertable entities (plugin hooks, the stdio MCP server, the wrapper IPC channel, the internal crates): v1-13's MCP link result is a founder disposition in the matrix, and the a11y plan holds no rule over it.
