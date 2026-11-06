@carried n=1 label=a11y-plan/a11y-scope-summary fact=fct-vtox22ts
why: item 1's tool-or-text decision and the portable-pty move both reach the CLI drivers this surface names, so its tool text must follow them while the plain-mode discipline the Windows-line removals touch keeps holding.
quote: The fallback is output-discipline assertions on the tests' existing CLI drivers: `assert_cmd` 2.2.2 with `predicates` 3.1.4, `trycmd` 1.2.1, and SGR-only checks through the portable-pty outer PTY (tests excerpt, E2E driver → cli). Their pass/fail results are machine-parseable through the harness `run` JSON.
@carried n=1 run=2 label=a11y-plan/a11y-scope-summary fact=fct-vtox22ts
quote: there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; `viola revive --list` rows and the revive refusal pairs carry no ESC byte
@carried n=1 run=3 label=a11y-plan/a11y-scope-summary fact=fct-vtox22ts
quote: C0/C1 control characters are escaped except `\n` and `\t`
@carried n=1 run=4 label=a11y-plan/a11y-scope-summary fact=fct-vtox22ts
quote: This must hold on `[ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).
@carried n=1 run=5 label=a11y-plan fact=fct-xmkdtzxa
quote: **cli / tui:** no a11y tool. Use the existing `assert_cmd` / `trycmd` / portable-pty output-discipline assertions, with manual screen-reader passes as a supplement.
@carried n=1 run=6 label=a11y-plan fact=fct-yzqlnwsx
quote: | cli | assertable (output discipline, no conformance claim) | assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer-PTY SGR checks; the `ubuntu-latest` CI leg | Orca + GNOME Terminal |
@carried n=1 run=7 label=a11y-plan fact=fct-3xiq4dqd
quote: **cli:** `[RB]` mirror and `unable <reason> <detail>` + `hint:` via trycmd.
@carried n=1 run=8 label=a11y-plan fact=fct-sf2opzaw
quote: C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
@carried n=2 label=a11y-plan/a11y-scope-summary fact=fct-twqx5x46
why: the portable-pty pin moves off `=0.8.1` (item 12), so the outer-PTY driver's version text must follow the new release while its zero-bytes and unblocked-keystroke checks stay green on it.
quote: Boundary assertions run on the tests' portable-pty `=0.8.1` outer-PTY driver (tests excerpt, E2E driver → tui). They check that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
@carried n=2 run=2 label=a11y-plan/a11y-scope-summary fact=fct-twqx5x46
quote: viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
@carried n=2 run=3 label=a11y-plan fact=fct-4ktfiloi
quote: | tui | boundary-only | portable-pty outer-PTY boundary driver (inherited `=0.8.1`): zero viola bytes, keystrokes unblocked | Orca + foot on Hyprland |
@carried n=2 run=4 label=a11y-plan fact=fct-gglbhu4k
quote: **tui:** the automated boundary is that human keystrokes through the portable-pty outer PTY are never blocked.
@carried n=2 run=5 label=a11y-plan fact=fct-weaq66l7
quote: boundary assertions on the portable-pty outer-PTY driver check that viola emits zero terminal bytes of its own and never blocks or delays human keystrokes;
@carried n=3 label=a11y-plan/a11y-scope-summary fact=fct-itb2wxsm
why: the live `viola run` sessions (v1-53's run, the driver-drives-a-builder session) and the PTY host the bump reaches must hold this boundary, read as a boundary reading only.
quote: The assertion boundary is viola's wrapper:
  - viola writes zero bytes to the terminal except the child's output;
  - human keystrokes are never blocked, refused or delayed past the current atomic paste;
  - focus, mouse and resize sequences and terminal replies (F-W2's closed list) pass through and do not count as human editing; every other byte does.
@carried n=3 run=2 label=a11y-plan/a11y-scope-summary fact=fct-yscgzykm
quote: the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes (`e4 b8 ad`) reach it.
@carried n=3 run=3 label=a11y-plan fact=fct-rdegquaa
quote: viola writes zero bytes to the terminal except the child's output, human keystrokes are never blocked, refused or delayed past the current atomic paste, and focus, mouse and resize sequences and terminal replies pass through and do not count as human editing;
@carried n=4 label=a11y-plan/aria-patterns-roles fact=fct-dx4kie52
why: the portable-pty bump reaches the outer-PTY driver these three cases run on and item 10's wheel hypothesis tests clause (3), so all three must stay green on ubuntu-latest after the bump.
quote: **tui:** three portable-pty outer-PTY nextest cases on `ubuntu-latest`, one per Section 1 boundary clause (Section 3 → Keyboard test harness → Tooling): (1) zero viola-originated bytes on the outer-PTY stream, byte for byte against an unwrapped fake-agent run; (2) human keystrokes delivered unblocked past the current atomic paste; (3) `\x1b[I`/`\x1b[O`, mouse and resize sequences do not move the wheel
@carried n=4 run=2 label=a11y-plan fact=fct-jkkyxmll
quote: on the tui, outer-PTY nextest cases on `ubuntu-latest` — zero viola-originated bytes, keystrokes delivered unblocked past the current atomic paste, and focus, mouse and resize sequences not moving the wheel. SC 2.1.1 (boundary stand-in on tui).
@carried n=5 label=a11y-plan/aria-patterns-roles fact=fct-uok334yy
why: item 1 decides tool or text for the trycmd half of P6's non-TTY assertions and the portable-pty bump reaches its outer-PTY half, while its SC stand-ins stay no conformance claim.
quote: **P6 — CLI board, wait/last, verify** (cli). No ARIA. trycmd / assert_cmd assert the non-TTY modes, and the portable-pty outer PTY asserts the TTY-only behaviours, on `ubuntu-latest`:
@carried n=5 run=2 label=a11y-plan/aria-patterns-roles fact=fct-uok334yy
quote: These stand in for SC 1.3.2, 1.4.1 and 3.3.1, with no conformance claim.
@carried n=5 run=3 label=a11y-plan fact=fct-ijnl2u6y
quote: trycmd / assert_cmd assert the non-TTY modes and the portable-pty outer PTY the TTY-only behaviours, on `ubuntu-latest` — linear static output, the caption-order literal, no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`, every SGR span beside its word, C0/C1 escaped.
@carried n=5 run=4 label=a11y-plan fact=fct-lpulrs7q
quote: WCAG does not apply directly to terminal output, so there is no conformance claim. Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1
@carried n=6 label=a11y-plan/aria-patterns-roles fact=fct-uok334yy
why: v1-36's board disposition (item 4) and item 1's tool-or-text decision must leave the caption order this literal pins undrifted and pinned by literal, not by a trycmd case.
quote: **cli mirror:** a literal pins the caption order the deferred 0.2.0 page must share (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), not a trycmd case: the `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`)
@carried n=7 label=a11y-plan/aria-patterns-roles fact=fct-jgpx3odq
why: the stale-row dim witness runs under the outer PTY the portable-pty bump reaches, so it must stay green with its exact sequence on ubuntu-latest.
quote: `viola list` runs under the outer PTY with `TERM=xterm-256color`; the row is dim from its first byte to its last and `stale` is printed (the exact SGR sequence it asserts: Section 6). It runs on ubuntu-latest.
@carried n=7 run=2 label=a11y-plan/visual-design-verification fact=fct-46bk3wap
quote: carries exactly the escape sequences `[DIM, BOLD, RESET, DIM, RESET]`, the word `stale` printed beside the dim (the spans as above), as the outer-PTY stale-row witness above asserts it on ubuntu-latest.
@carried n=7 run=3 label=a11y-plan fact=fct-lchghj7u
quote: **Stale-row dim:** dim only on stale rows is proven at the unit tier and, under the outer PTY, on ubuntu-latest.
@carried n=8 label=a11y-plan/visual-design-verification fact=fct-es7le4as
why: the narrow-terminal cut reads its width from `viola_pty::host_size()` in the reached `crates/viola-pty`, and the SGR discipline is checked on assert_cmd + portable-pty, both of which the bump reaches.
quote: On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus `...`, never inside a `\xHH` escape (the width from `viola_pty::host_size()`); a wrapped NAME is never cut, and piped output and `--json` keep every name whole.
@carried n=8 run=2 label=a11y-plan/visual-design-verification fact=fct-es7le4as
quote: viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and the revive refusal pairs and `--list` rows carry no ESC byte. `unable` / `fail` are never coloured. Checked with assert_cmd + portable-pty on `[ubuntu-latest]`.
@carried n=8 run=3 label=a11y-plan fact=fct-znmnfnxq
quote: no SGR of viola's own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`; `unable` / `fail` never coloured;
@carried n=9 label=a11y-plan/a11y-scope-summary fact=fct-6wf4lqpi
why: item 17's Windows remnants are each removed or kept with a reason against this Linux-only platform, and the green-on-Linux close reads its CI on ubuntu-latest.
quote: **Platform (every surface):** Linux only — the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland; the Windows and macOS code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).
@carried n=9 run=2 label=a11y-plan fact=fct-tzchk23h
quote: The frontend is deferred to 0.2.0; its technology is not chosen.
@carried n=10 label=a11y-plan/a11y-scope-summary fact=fct-sjhco52d
why: the founder's word on the socket-directory pair's wording (item 7) must keep this refusal's hint last, plain ASCII, no SGR and naming no path, pid or uid.
quote: each refuses before any frame with exit 21 and the socket-directory `hint:` line last on stderr, plain ASCII, no SGR, naming no path, pid or uid, or under `--json` one document with `error` `instance-unreachable` and `detail` `socket-dir-refused`; the directory is left as found
@carried n=10 run=2 label=a11y-plan fact=fct-c6pzu5nt
quote: Every human-mode CLI refusal except `unknown` (exit 14, its detail a code word or nothing, design cli) ends with a `hint:` line.
@carried n=11 label=a11y-plan/a11y-scope-summary fact=fct-rts3hgep
why: the same socket-directory pair wording ruling reaches the start's refusal, which must keep its pair hint-last, plain ASCII, no SGR, naming only the parsed name.
quote: each refuses with exit 1 before any snapshot, bind or child, a failed create reading the same, with the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name
@carried n=12 label=a11y-plan fact=fct-zj2wxt2x
why: item 1 decides tool or text for the trycmd these strategy-pyramid rows name, so their wording must follow that decision, and the Operable row's portable-pty boundary must hold after the bump.
quote: | Understandable | — | [cli: `unable <reason> <detail>` then `hint:` via trycmd] |
@carried n=12 run=2 label=a11y-plan fact=fct-6py5v5ks
quote: | Cognitive | No cognitive trigger, so no AAA escalation. | [cli: `hint:` line pinned by trycmd] |
@carried n=12 run=3 label=a11y-plan fact=fct-7a7ijng6
quote: | Operable | — | [tui: portable-pty boundary — human keystrokes never blocked] |
@carried n=13 label=a11y-plan fact=fct-gmsx3s7a
why: the keyed contracts `a11y-testing-tool-pick` and `keyboard-test-harness` that item 1 names live here and must be read by key and brought to agree with item 1's decision.
quote: Contracts: ask `rec.py contracts a11y-plan` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.
@carried n=14 label=a11y-plan fact=fct-csjr2vg7
why: the live sessions and item 10's wheel hypothesis read exactly this rule, and a new non-editing shape joins only when measured, by its exact grammar, on the founder's word.
quote: NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing (P4).
@carried n=14 run=2 label=a11y-plan fact=fct-pzkrh6fa
quote: **tui:** the terminal belongs to the child. Human keystrokes are never blocked or delayed past the atomic paste.
@carried n=15 label=a11y-plan fact=fct-5lohrrsl
why: a cap whose acceptance names a 0.2.0 surface is deferred only on the founder's word, and this deferral statement is the reason text that matrix deferral cites.
quote: The frontend is deferred to 0.2.0; its technology is not chosen.
@carried n=16 label=a11y-plan fact=fct-by4mf7d7
why: the «every gate green on Linux» clause reads CI's jobs on this one workflow and matrix, which the chunk re-reads at its wrap.
quote: **Platform:** GitHub Actions, one push/PR workflow `ci.yml` (the scheduled `nightly.yml` carries no a11y step), matrix `[ubuntu-latest]` (upstream-context Architecture Excerpt → CI/CD Platform).
@carried n=17 label=a11y-plan fact=fct-s4vibfo3
why: the CI stage row names trycmd 1.2.1 and the portable-pty outer PTY, which item 1's decision and the portable-pty move each reach.
quote: | Unit / integration (`ubuntu-latest`) | assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer PTY (cli SGR, tui boundary) |
@carried n=18 label=a11y-plan fact=fct-tyva3sbt
why: the chunk's green close is read through this gate, so none of its breaches may stand at the wrap and no a11y case may be skipped to reach green.
quote: `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`).
@carried n=18 run=2 label=a11y-plan fact=fct-icuat5rz
quote: NEVER skip a11y on CI failure (a11y is FAIL-fast gate, not warning)
@carried n=19 label=a11y-plan fact=fct-ms7djxet
why: the matrix claims this chunk writes for the caps it proves cite assert_cmd / portable-pty results only as output-discipline stand-ins, never as conformance.
quote: NEVER claim WCAG conformance for the cli or the `viola run` TUI (the wrapper-hosted TUI, whether `viola run` or a passed `viola revive` started it). assert_cmd / trycmd / portable-pty results are output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, not a11y-tool evidence.
@carried n=20 label=a11y-plan fact=fct-633dmm7e
why: the SGR checks re-run on the bumped outer PTY and the plain-mode Windows-line removals must keep viola's SGR out of every listed mode.
quote: NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
@carried n=20 run=2 label=a11y-plan fact=fct-4w44zoy4
quote: NEVER convey information by color alone
@carried n=21 label=a11y-plan fact=fct-ickgtugn
why: reading the dialog hold live near its 3600 s ceiling (item 2) must keep it a bound on the driver, never a timeout set on the human.
quote: NEVER set harsh timeouts without warning (no warn-extend pattern per SC 2.2.1). v1 has none.
@carried n=22 label=a11y-plan fact=fct-i7pow3j3
why: the live readings this chunk makes (v1-53's run, the dialog hold, the wheel and tab-close legs) must rest on viola's own outputs and the JUnit, never on the rendered screen.
quote: NEVER parse the rendered `claude` TUI screen for content or take screenshots as a11y evidence (R7; tests "no screen-content parsing").
@carried n=23 label=a11y-plan fact=fct-vt5poetp
why: v1-36's link-set disposition on the board (item 4) must leave the caption set and order of `viola list` undrifted from its literal.
quote: NEVER let the caption set or order of CLI `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) drift from its `CAPTION` literal (SC 3.2.4).
@binding n=1
**a11y-scope-summary cli surface ↔ aria-patterns-roles P6 ↔ the body's strategy pyramid, CI stage row and keyed contracts**: item 1's tool-or-text decision for trycmd / assert_cmd must land the same way everywhere the plan names those tools, so the cli surface's tool line, P6's non-TTY half, the pyramid rows, the CI stage row and the two keyed contracts agree with each other and with the code (a11y-plan/a11y-scope-summary fct-vtox22ts) (a11y-plan/aria-patterns-roles fct-uok334yy) (a11y-plan fct-zj2wxt2x) (a11y-plan fct-s4vibfo3) (a11y-plan fct-gmsx3s7a)
@binding n=2
**a11y-scope-summary tui surface ↔ aria-patterns-roles P4 ↔ the body's Keyboard rules**: moving portable-pty off `=0.8.1` must carry the driver's version text to the new release, and the three outer-PTY P4 cases (zero viola bytes, keystrokes unblocked, non-editing sequences not moving the wheel) must stay green on ubuntu-latest on the new release (a11y-plan/a11y-scope-summary fct-twqx5x46) (a11y-plan/aria-patterns-roles fct-dx4kie52) (a11y-plan fct-csjr2vg7)
@binding n=3
**visual-design-verification ↔ aria-patterns-roles stale-row dim ↔ the body's Visual rules**: the SGR spans and the stale-row sequence `[DIM, BOLD, RESET, DIM, RESET]` are asserted under the same outer PTY the bump reaches, and the narrow-terminal cut reads `viola_pty::host_size()` from the reached crate, so both must hold after the move (a11y-plan/visual-design-verification fct-es7le4as) (a11y-plan/visual-design-verification fct-46bk3wap) (a11y-plan/aria-patterns-roles fct-jgpx3odq) (a11y-plan fct-633dmm7e)
@binding n=4
**a11y-scope-summary TUI passthrough ↔ the body's Universal and Keyboard bans**: the live sessions are boundary readings of the passthrough, resting on viola's own outputs and the JUnit, never on the rendered screen, and never blocking a human key (a11y-plan/a11y-scope-summary fct-itb2wxsm) (a11y-plan fct-i7pow3j3) (a11y-plan fct-csjr2vg7)
@binding n=5
**a11y-scope-summary socket-directory refusals ↔ the body's Cognitive hint rule**: whatever wording the founder rules for the socket-directory pair must keep `hint:` last, plain ASCII, no SGR and no path, pid or uid on both the frame verbs and the start (a11y-plan/a11y-scope-summary fct-sjhco52d) (a11y-plan/a11y-scope-summary fct-rts3hgep) (a11y-plan fct-c6pzu5nt)
@binding n=6
**aria-patterns-roles P6 cli mirror ↔ the body's Cognitive caption ban**: v1-36's board disposition must leave the `CAPTION` literal's order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI undrifted and pinned by literal (a11y-plan/aria-patterns-roles fct-uok334yy) (a11y-plan fct-vt5poetp)
@binding n=7
**a11y-scope-summary platform ↔ the body's CI platform and gate**: the Linux-only platform, the one `ci.yml` on `[ubuntu-latest]` and the `gate --require coverage,doctest` with no skip are what the green-on-Linux close and the Windows-remnant removals are measured against (a11y-plan/a11y-scope-summary fct-6wf4lqpi) (a11y-plan fct-by4mf7d7) (a11y-plan fct-tyva3sbt)
@binding n=8
**aria-patterns-roles P6 SC stand-ins ↔ the body's conformance ban and frontend deferral**: matrix claims cite output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1 with no conformance, and a cap naming a 0.2.0 surface is deferred only on the founder's word with the deferral statement as its reason (a11y-plan/aria-patterns-roles fct-uok334yy) (a11y-plan fct-ms7djxet) (a11y-plan fct-5lohrrsl)
