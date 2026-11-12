@row_none n=1 rec=fct-bskn6eoi@1 type=S
@row_none n=2 rec=fct-wdvdzuew@1 type=0
what: an editing note for the plan document itself
@row_none n=3 rec=fct-6jmx2zce@1 type=S
@row_none n=4 rec=fct-6rq6vcoc@1 type=S
@row_add n=1 row=decision/a11y-tier-standard area=1-a11y-scope-summary rec=fct-loqatd4n@1
name: A11y tier Standard (1)
rule: The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
@row_none n=5 rec=fct-fp5ry76t@1 type=S
@row_add n=2 row=absence/web-or-graphical-frontend area=1-a11y-scope-summary rec=fct-tzchk23h@2
capability: A web or graphical frontend
@row_none n=6 rec=fct-maahpp3k@1 type=S
@row_add n=3 row=decision/no-a11y-tool-output-discipline-assertions area=1-a11y-scope-summary rec=fct-xmkdtzxa@3,fct-ulpihvwi@1,fct-sf2opzaw@1
name: Output-discipline assertions instead of an a11y tool
rule: cli and tui use no a11y tool: the existing process, literal-pin and portable-pty output-discipline assertions on text and streams stand in, with manual screen-reader passes as a supplement.
why: no automated a11y tool reaches a terminal
@row_add n=4 row=contract/viola-revive-json/start-refusal area=1-a11y-scope-summary rec=fct-z2htpz2e@2
boundary: viola revive --json output
item: start refusal
promise: Every refusal of the start that revive shares with run prints one exit-1 document, pinned-hash-mismatch and squatted-name among them, and a program not found as internal-error.
@row_add n=5 row=contract/viola-revive-json/pump-child-error area=1-a11y-scope-summary rec=fct-z2htpz2e@2
boundary: viola revive --json output
item: pump_child error
promise: A pump_child error prints no document: the child has held the terminal, and viola writes nothing past it.
@row_add n=6 row=contract/tui-passthrough/entry area=1-a11y-scope-summary rec=fct-rdegquaa@2
boundary: viola run TUI passthrough
item: entry
promise: The passthrough is entered by viola run and by a viola revive whose preflight passed.
@row_add n=7 row=contract/tui-passthrough/vendor-content area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-weaq66l7@1
boundary: viola run TUI passthrough
item: vendor content
promise: The rendered content belongs to the vendor claude and is never traced or parsed; no conformance claim is made about the claude TUI.
@row_add n=8 row=contract/tui-passthrough/viola-terminal-bytes area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-weaq66l7@1
boundary: viola run TUI passthrough
item: viola terminal bytes
promise: viola writes zero bytes of its own to the terminal; only the child's output reaches it.
@row_add n=9 row=contract/tui-passthrough/human-keystrokes area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-weaq66l7@1,fct-gglbhu4k@1,fct-hu3qpzu6@2,fct-67rr3koq@1
boundary: viola run TUI passthrough
item: human keystrokes
promise: Human keystrokes are never blocked, refused or delayed past the current atomic paste.
@row_add n=10 row=contract/tui-passthrough/focus-mouse-resize-sequences area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-hu3qpzu6@2
boundary: viola run TUI passthrough
item: focus, mouse and resize sequences
promise: Focus, mouse and resize sequences and terminal replies pass through and do not count as human editing for wheel purposes.
@row_add n=11 row=contract/tui-passthrough/vt100-feed-panic area=1-a11y-scope-summary rec=fct-rdegquaa@2
boundary: viola run TUI passthrough
item: vt100 feed panic
promise: The boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes still reach it.
@row_add n=12 row=decision/single-vendor-content-boundary area=1-a11y-scope-summary rec=fct-c5w2jfcj@1
name: Single vendor-content boundary
rule: The viola run TUI passthrough, which a passed viola revive also enters, is the only boundary-only vendor-content zone.
@row_add n=13 row=decision/no-compliance-regime area=1-a11y-scope-summary rec=fct-btfrs2pb@1,fct-2b735qj7@1,fct-wvygpu6t@1
name: No accessibility compliance regime
rule: No accessibility compliance regime applies, so no regime-mandated WCAG success criterion binds any surface.
@row_none n=7 rec=fct-gbsh2fhc@1 type=S
@row_add n=14 row=decision/colour-second-cue-only area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-67rr3koq@1,fct-lpulrs7q@1,fct-6ghxxgxr@1
name: Colour only a second cue
rule: Colour is never the sole cue on the CLI: the word always sits beside any SGR.
@row_add n=15 row=decision/static-linear-output area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-pgo5ntgv@1,fct-q7isztzg@1
name: Static linear output
rule: viola's own output is static and linear, with no spinner or redraw.
@row_add n=16 row=contract/cli-output/control-characters area=1-a11y-scope-summary rec=fct-sf2opzaw@1
boundary: CLI output
item: C0/C1 control characters
promise: C0/C1 control characters are escaped except \n and \t, which list rows escape too.
@row_add n=17 row=decision/tui-boundary-assertions-only area=1-a11y-scope-summary rec=fct-weaq66l7@1,fct-gglbhu4k@1
name: TUI boundary assertions only
rule: The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim.
@row_add n=18 row=contract/tui-passthrough/run-diagnostics area=1-a11y-scope-summary rec=fct-weaq66l7@1
boundary: viola run TUI passthrough
item: run diagnostics
promise: viola's diagnostics for run go to diagnostics/, never to the terminal.
@row_none n=8 rec=fct-vliofs34@1 type=S
@row_add n=19 row=decision/cli-not-keyboard-interactive area=1-a11y-scope-summary rec=fct-gglbhu4k@1
name: CLI not keyboard-interactive
rule: The CLI has no prompt, TUI mode or pager; its input comes only from stdin or --file.
@row_add n=20 row=decision/manual-keyboard-pass area=1-a11y-scope-summary rec=fct-gglbhu4k@1,fct-lvifofdb@1
name: Manual keyboard pass as supplement
rule: A manual keyboard pass over CLI and TUI supplements the automated keystroke boundary assertion.
@row_add n=21 row=decision/manual-orca-pass area=1-a11y-scope-summary rec=fct-orpg5jsb@2,fct-ol6ygl55@2
name: Manual Orca screen-reader pass
rule: A manual Orca screen-reader pass on Section 4 P4 and P6, founder-owned, supplements the automated assertions.
why: no blind or low-vision target users are named, so screen-reader priority is scoped to CLI/TUI linearity, not a full assistive-tech matrix
@row_add n=22 row=decision/no-browser-a11y-leg area=1-a11y-scope-summary rec=fct-ktqfqjb4@3
name: No browser a11y leg
rule: CI runs no browser a11y leg.
why: viola renders no page
@row_add n=23 row=decision/a11y-inside-existing-suites area=1-a11y-scope-summary rec=fct-ktqfqjb4@3
name: A11y assertions inside existing suites
rule: A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
@row_add n=24 row=decision/ci-test-gate-no-skips area=1-a11y-scope-summary rec=fct-ktqfqjb4@3
name: CI test gate allows no skips
rule: The CI test job's gate requires coverage and doctest and never allows skips.
@row_add n=25 row=decision/critical-path-send-readback area=1-a11y-scope-summary rec=fct-zvuuhh2n@1,fct-utn3b3p5@1,fct-4md23dny@1
name: Critical path: confirmed send and readback
rule: Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path.
@row_none n=9 rec=fct-ooazrtpo@1 type=0
what: a list of source citations
@row_add n=26 row=decision/critical-path-dialog-answer area=1-a11y-scope-summary rec=fct-tauqfaof@1,fct-opu4qsrg@1,fct-4md23dny@1
name: Critical path: dialog pending, then answer
rule: A pending dialog answered with viola answer (question, permission, plan), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
@row_none n=10 rec=fct-yjr6ng6y@1 type=0
what: a list of source citations
@row_add n=27 row=decision/critical-path-human-takes-wheel area=1-a11y-scope-summary rec=fct-gxtjio4p@1,fct-4wknsfov@1,fct-4md23dny@1
name: Critical path: human takes the wheel
rule: The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path.
@row_none n=11 rec=fct-gr6yt6ul@1 type=0
what: an N/A ARIA-roles line for terminal surfaces
@row_add n=28 row=contract/cli-refusal/format area=1-a11y-scope-summary rec=fct-67rr3koq@1,fct-lpulrs7q@1
boundary: CLI refusals
item: refusal format
promise: A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
@row_none n=12 rec=fct-ayv3gkd7@1 type=0
what: a list of source citations
@row_add n=29 row=decision/critical-path-cli-board-output area=1-a11y-scope-summary rec=fct-vunrzfno@2,fct-3mbiblpp@1,fct-4md23dny@1
name: Critical path: CLI board, wait/last and verify output
rule: The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
@row_add n=30 row=decision/terminal-structural-equivalents area=1-a11y-scope-summary rec=fct-w4hvnbpo@1
name: Terminal structural equivalents of ARIA roles
rule: In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator and the --help group labels.
@row_add n=31 row=contract/viola-list/unwrapped-separator area=1-a11y-scope-summary rec=fct-w4hvnbpo@1
boundary: viola list output
item: -- UNWRAPPED - READ-ONLY --
promise: The separator appears only when unwrapped rows exist.
@row_add n=32 row=contract/viola-wait/waiting-line area=1-a11y-scope-summary rec=fct-pgo5ntgv@1
boundary: viola wait output
item: waiting: <name>
promise: The waiting line is written once on stderr, on a TTY only.
@row_add n=33 row=contract/viola-verify/step-line area=1-a11y-scope-summary rec=fct-pgo5ntgv@1
boundary: viola verify output
item: step line
promise: Each verify step prints a line [NN/NN] … pass|fail.
@row_add n=34 row=decision/output-discipline-stands-in-for-sc area=1-a11y-scope-summary rec=fct-lpulrs7q@1
name: Output discipline stands in for WCAG SC
rule: Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
why: WCAG does not apply directly to terminal output
@row_none n=13 rec=fct-u3smqwc5@1 type=0
what: a list of source citations
@row_none n=14 rec=fct-xhd44rln@1 type=S
@row_none n=15 rec=fct-v4k6yuiy@1 type=S
@row_none n=16 rec=fct-tdneapld@1 type=S
@row_add n=35 row=absence/internationalization area=1-a11y-scope-summary rec=fct-hmypp33q@1
capability: Internationalization: no i18n, RTL or lang conventions
@row_add n=36 row=absence/touch-surface area=1-a11y-scope-summary rec=fct-72c2us6l@1
capability: A touch surface
@row_none n=17 rec=fct-dtnxd75i@1 type=S
@row_none n=18 rec=fct-o3rrybxc@1 type=S
@row_none n=19 rec=fct-y24ge3fw@1 type=S
@row_none n=20 rec=fct-3frlwnx5@1 type=S
@row_add n=37 row=decision/no-sgr-without-tty area=2-a11y-strategy rec=fct-fce6ttn6@1,fct-ijnl2u6y@4,fct-znmnfnxq@1
name: No SGR without a colour-capable terminal
rule: The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`.
guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
@row_add n=38 row=decision/word-beside-every-sgr area=2-a11y-strategy rec=fct-fce6ttn6@1,fct-ijnl2u6y@4,fct-znmnfnxq@1
name: Word beside every SGR span
rule: Every SGR span on the CLI sits beside its word, so each state is still carried by the word without the colour.
guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
@row_add n=39 row=contract/cli-refusal/unable-line area=2-a11y-strategy rec=fct-zj2wxt2x@2,fct-6jy2xx4r@1
boundary: CLI refusal output
item: unable line
promise: A refusal prints `unable  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
@row_add n=40 row=contract/cli-refusal/hint-line area=2-a11y-strategy rec=fct-zj2wxt2x@2,fct-6jy2xx4r@1
boundary: CLI refusal output
item: hint line
promise: A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
@row_add n=41 row=decision/no-aaa-cognitive-escalation area=2-a11y-strategy rec=fct-6py5v5ks@2,fct-lfozu4af@1
name: No AAA cognitive escalation
rule: Cognitive accessibility is not escalated to AAA; SC 2.2.6, 3.1.5 and 3.3.9 are not claimed, and only cognitive-relevant A/AA criteria with a v1 surface are covered.
why: no cognitive-accessibility trigger fired
@row_none n=21 rec=fct-4lys5tdv@1 type=S
@row_none n=22 rec=fct-sutj5ddg@1 type=S
@row_none n=23 rec=fct-gmsx3s7a@2 type=S
@row_none n=24 rec=fct-q4opus3z@1 type=S
@row_add n=42 row=decision/cli-structural-landmark-equivalents area=4-aria-patterns-roles rec=fct-m7wf3gp4@1
name: CLI structural equivalents of landmarks
rule: The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels.
@row_add n=43 row=decision/tui-boundary-only area=4-aria-patterns-roles rec=fct-m7wf3gp4@1,fct-4ktfiloi@3
name: Tui accessibility is boundary-only
rule: The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own.
@row_none n=25 rec=fct-lbor4q4u@1 type=S
@row_none n=26 rec=fct-64bntlu7@1 type=S
@row_none n=27 rec=fct-ypjst57k@1 type=S
@row_none n=28 rec=fct-cgy7rc2r@1 type=S
@row_none n=29 rec=fct-3xiq4dqd@2 type=5
@row_none n=30 rec=fct-r2rqn3md@1 type=5
@row_add n=44 row=decision/send-refused-while-human-typing area=4-aria-patterns-roles rec=fct-jkkyxmll@3
name: Send refused while the human types
rule: When the human takes the wheel, `send` is refused with `human-typing` on stderr and a typed exit code.
guard: std::process::Command asserts
@row_add n=45 row=decision/wheel-ignores-terminal-events area=4-aria-patterns-roles rec=fct-jkkyxmll@3
name: Focus, mouse and resize never move the wheel
rule: On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal.
guard: outer-PTY nextest cases on ubuntu-latest
@row_add n=46 row=decision/c0-c1-escaped area=4-aria-patterns-roles rec=fct-ijnl2u6y@4
name: C0 and C1 escaped in CLI output
rule: C0 and C1 control characters in CLI output are escaped.
guard: literal pins over std::process::Command on ubuntu-latest
@row_add n=47 row=decision/linear-static-output area=4-aria-patterns-roles rec=fct-ijnl2u6y@4
name: Linear static CLI output
rule: CLI output is linear and static.
guard: literal pins over std::process::Command on ubuntu-latest
@row_add n=48 row=decision/stale-row-dim area=4-aria-patterns-roles rec=fct-lchghj7u@3,fct-znmnfnxq@1
name: Dim only on stale rows
rule: Dim SGR is applied only to stale rows, beside the word that carries the state.
guard: unit-tier tests and outer-PTY tests on ubuntu-latest
@row_none n=31 rec=fct-wbijh7na@1 type=S
@row_add n=49 row=decision/cli-no-interactive-input area=5-keyboard-navigation rec=fct-2bakknao@1
name: No interactive CLI input
rule: The CLI has no interactive prompt, pager or redraw, and takes input only from stdin or `--file`.
@row_add n=50 row=decision/tui-terminal-belongs-to-child area=5-keyboard-navigation rec=fct-pzkrh6fa@1
name: Tui terminal belongs to the child
rule: On the tui, the terminal belongs to the wrapped child process, not to viola.
@row_none n=32 rec=fct-h4gwghks@1 type=S
@row_none n=33 rec=fct-idxet2fd@1 type=S
@row_add n=51 row=decision/dialog-amber-sgr area=6-visual-design-verification rec=fct-znmnfnxq@1
name: Amber only on the DIALOG word
rule: Amber SGR is applied only to the `DIALOG` word.
@row_add n=52 row=decision/name-bold-sgr area=6-visual-design-verification rec=fct-znmnfnxq@1
name: Bold only on NAME values
rule: Bold SGR is applied only to NAME values.
@row_add n=53 row=decision/unable-fail-never-coloured area=6-visual-design-verification rec=fct-znmnfnxq@1
name: Refusal and failure words never coloured
rule: The words `unable` and `fail` are never coloured.
@row_add n=54 row=decision/name-truncation area=6-visual-design-verification rec=fct-znmnfnxq@1
name: NAME truncation only on a narrow terminal
rule: An unwrapped NAME is cut with `...` only on a terminal too narrow and never inside a `\xHH` escape; a wrapped NAME, piped output and `--json` are never cut.
@row_none n=34 rec=fct-7gxxr43g@1 type=S
@row_none n=35 rec=fct-jbtnxl7p@1 type=S
@row_none n=36 rec=fct-4reqosrn@1 type=S
@row_none n=37 rec=fct-p4ciplii@1 type=S
@row_add n=55 row=decision/screen-reader-target-cli area=7-screen-reader-support rec=fct-2kjabyk2@2,fct-yzqlnwsx@4
name: Orca with GNOME Terminal for the cli
rule: Manual screen-reader checks of the cli use Orca with GNOME Terminal on Linux: linear output, refusals read as text, no escape noise.
@row_add n=56 row=decision/screen-reader-target-tui area=7-screen-reader-support rec=fct-wawttggc@2,fct-4ktfiloi@3
name: Orca with foot on Hyprland for the tui
rule: Manual screen-reader checks of the tui use Orca with foot on Hyprland on Linux.
@row_none n=38 rec=fct-ixta2kmh@1 type=S
@row_add n=57 row=absence/user-time-limit area=8-cognitive-accessibility rec=fct-do54jgjo@1,fct-nu6eg27k@1
capability: A time limit on the user, and any timeout-extension component
@row_add n=58 row=decision/time-bounds-never-on-human area=8-cognitive-accessibility rec=fct-mkekg7f5@1
name: Time bounds never fall on the human
rule: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read, budget pause) falls on the driver, the user's command or viola's child; no key is blocked, refused or delayed by it.
@row_add n=59 row=contract/cli-refusal/exit-code area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: exit code
promise: Every refusal ends with a typed exit code.
@row_add n=60 row=contract/cli-refusal/unknown-refusal area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: the `unknown` refusal
promise: The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing.
@row_add n=61 row=contract/cli-refusal/fault area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: fault
promise: Faults are not refusals and carry no hint.
@row_add n=62 row=contract/cli-refusal/json area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: --json
promise: Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document.
@row_none n=39 rec=fct-ync5zj2r@1 type=0
what: the WCAG success criteria the error pattern is mapped to
@row_add n=63 row=decision/refusal-layout-literal-pins area=8-cognitive-accessibility rec=fct-2jdilj7v@4
name: Refusal layout held by literal pins
rule: Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest.
guard: literal pins over std::process::Command on ubuntu-latest
@row_add n=64 row=decision/no-reading-level-gate area=8-cognitive-accessibility rec=fct-uoxd5hfd@1
name: No reading-level gate
rule: No reading-level target is gated and no readability scorer is introduced.
why: SC 3.1.5 Reading Level (AAA) is not claimed, and no readability scorer is in a11y-research
@row_add n=65 row=decision/fixed-copy-from-design area=8-cognitive-accessibility rec=fct-kgelfpj7@1
name: Fixed CLI copy comes from design
rule: The CLI's fixed copy set (`no wrapped sessions — start one with  viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
@row_add n=66 row=decision/reason-code-beside-unable area=8-cognitive-accessibility rec=fct-kgelfpj7@1
name: Reason codes always beside unable
rule: Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
@row_add n=67 row=decision/no-bare-error-text area=8-cognitive-accessibility rec=fct-kgelfpj7@1
name: No stack trace or bare status as error text
rule: The CLI never shows a stack trace or a bare status number as the only error text.
@row_add n=68 row=decision/fixed-cli-strings-pinned area=8-cognitive-accessibility rec=fct-fhiyvzbh@2
name: Fixed CLI strings pinned by literal
rule: Every fixed CLI string, hints and result lines included, is pinned by literal.
why: so any copy change fails CI until the pinned string is updated on purpose
@row_add n=69 row=contract/viola-list-caption/order area=8-cognitive-accessibility rec=fct-xifq2q6n@1,fct-cyo3rpos@2
boundary: viola list caption
item: order
promise: The caption order is `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI`, pinned by the `CAPTION` literal.
@row_none n=40 rec=fct-nl3a6agd@1 type=S
@row_none n=41 rec=fct-und5fgbp@1 type=S
@row_add n=70 row=decision/a11y-ci-github-actions area=9-ci-integration rec=fct-by4mf7d7@2
name: A11y checks in the GitHub Actions push/PR workflow
rule: A11y checks run in the one GitHub Actions push/PR workflow on an `ubuntu-latest` matrix; the scheduled nightly workflow carries no a11y step.
@row_none n=42 rec=fct-bxtkmwvm@1 type=S
@row_none n=43 rec=fct-ow4aqcke@1 type=S
@row_none n=44 rec=fct-j7gtyyjf@1 type=S
@row_add n=71 row=decision/a11y-cases-inside-coverage-suite area=9-ci-integration rec=fct-s4vibfo3@4,fct-pkoabs22@2
name: A11y cases run inside the coverage suite
rule: The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict.
why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract
@row_none n=45 rec=fct-bskmbnbb@1 type=S
@row_add n=72 row=decision/no-separate-a11y-driver area=9-ci-integration rec=fct-pkoabs22@2,fct-4sblkbrt@1
name: No separate a11y driver or suite value
rule: A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
@row_add n=73 row=decision/no-own-a11y-artifact area=9-ci-integration rec=fct-vicafivy@1
name: No a11y artifact of its own
rule: A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
@row_none n=46 rec=fct-3i5p4gas@1 type=S
@row_none n=47 rec=fct-bit55egn@1 type=S
@row_none n=48 rec=fct-jj5q2zzy@1 type=S
@row_add n=74 row=decision/cli-invariants-pass-in-ci area=10-slo-invariants-a11y-budgets rec=fct-k6kozbbz@2
name: CLI output-discipline invariants pass in CI
rule: The CLI output-discipline invariants (P4, P6) pass on the `ubuntu-latest` CI leg inside the `coverage` suite.
@row_add n=75 row=decision/mutation-audit-covers-crate-invariants area=10-slo-invariants-a11y-budgets rec=fct-k6kozbbz@2
name: Mutation audit covers invariants held in crate tests
rule: Where the CLI output-discipline invariants (P4, P6) live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them.
@row_add n=76 row=decision/no-a11y-performance-budget area=10-slo-invariants-a11y-budgets rec=fct-g5vn64oj@1,fct-2575wca3@1
name: No performance budget per a11y CI run
rule: A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
@row_add n=77 row=decision/zero-flakiness area=10-slo-invariants-a11y-budgets rec=fct-zejx5pqs@1,fct-jecbfjie@1
name: Zero flakiness, no retries
rule: Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed.
why: retry-once policies mask real signals (the agent-driven principle)
@row_add n=78 row=decision/a11y-gate-breach-fails-build area=10-slo-invariants-a11y-budgets rec=fct-tyva3sbt@1,fct-lzk7pnbc@1
name: Gate breach fails the build
rule: A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
@row_none n=49 rec=fct-x6z7pmrz@1 type=S
@row_none n=50 rec=fct-tuewsa34@1 type=S
@row_add n=79 row=decision/non-editing-sequences area=11-a11y-anti-patterns-never-do-these rec=fct-csjr2vg7@2,fct-itb2wxsm@2
name: Terminal sequences are not human editing
rule: Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4).
@row_add n=80 row=decision/no-information-by-colour-alone area=11-a11y-anti-patterns-never-do-these rec=fct-4w44zoy4@1,fct-vtox22ts@4
name: No information by colour alone
rule: Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).
@row_add n=81 row=decision/no-colour-on-failure-words area=11-a11y-anti-patterns-never-do-these rec=fct-633dmm7e@1
name: Failure words are never coloured
rule: The CLI words `unable` and `fail` are never coloured.
@row_add n=82 row=decision/no-unwarned-timeouts area=11-a11y-anti-patterns-never-do-these rec=fct-ickgtugn@1
name: No harsh timeouts without warning
rule: No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
@row_add n=83 row=decision/refusal-ends-with-hint area=11-a11y-anti-patterns-never-do-these rec=fct-c6pzu5nt@1
name: Every refusal ends with a hint
rule: Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3).
except: `unknown` (exit 14), whose detail is a code word or nothing
@row_add n=84 row=decision/codes-beside-plain-word area=11-a11y-anti-patterns-never-do-these rec=fct-lsnfzz6p@1
name: Kebab-case codes beside a plain word
rule: A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
@row_add n=85 row=decision/no-stack-trace-only-error area=11-a11y-anti-patterns-never-do-these rec=fct-retrfvu7@1
name: No stack trace as the only error text
rule: The CLI never renders a stack trace as the only error text.
@row_add n=86 row=decision/list-captions-fixed area=11-a11y-anti-patterns-never-do-these rec=fct-vt5poetp@1
name: Board captions fixed by their literal
rule: The caption set and order of `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) never drift from its `CAPTION` literal (SC 3.2.4).
@row_add n=87 row=decision/no-spinners area=11-a11y-anti-patterns-never-do-these rec=fct-dqbono2s@1,fct-vtox22ts@4
name: No spinners
rule: CLI output is static and linear, with no spinner or redraw.
why: design ban
@row_add n=88 row=decision/a11y-fail-fast-gate area=11-a11y-anti-patterns-never-do-these rec=fct-icuat5rz@1
name: A11y is a fail-fast gate
rule: A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
@row_add n=89 row=decision/machine-output-a11y-review area=11-a11y-anti-patterns-never-do-these rec=fct-ayxkclws@1
name: No human-only a11y review
rule: No a11y review is human-only without machine output.
why: the canonical agent-runnable principle
@row_add n=90 row=decision/no-soft-a11y-slo area=11-a11y-anti-patterns-never-do-these rec=fct-o743h7h2@1
name: No soft a11y SLO
rule: An a11y SLO is never soft: it triggers a build or deploy failure on its threshold.
@row_add n=91 row=decision/no-vendor-locked-a11y-tool area=11-a11y-anti-patterns-never-do-these rec=fct-kdsodzwc@1
name: No vendor-locked a11y tool as only method
rule: A vendor-locked a11y tool without machine output or API (Stark Visual Studio plugin, Tenable.io UI-only) is never the ONLY method.
@row_add n=92 row=decision/no-screen-content-parsing area=11-a11y-anti-patterns-never-do-these rec=fct-i7pow3j3@1,fct-twqx5x46@3
name: No screen-content parsing
rule: The rendered `claude` TUI screen is never parsed for content, and screenshots are never taken as a11y evidence (R7).
why: the rendered TUI is vendor content
@row_none n=51 rec=fct-uq2yxg4r@1 type=S
@row_none n=52 rec=fct-m7m7gjre@1 type=S
@row_add n=93 row=contract/revive-json-refusal/document area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: exit-1 refusal document
promise: Each exit-1 refusal of the strict-modes check, the preflight or the start shared with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr.
@row_add n=94 row=contract/revive-json-refusal/strict-modes-failed area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: strict-modes-failed
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=95 row=contract/revive-json-refusal/already-live area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: already-live
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=96 row=contract/revive-json-refusal/no-session area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: no-session
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; `viola revive <name> --list --json` over an empty chain prints this document, exit 1.
@row_add n=97 row=contract/revive-json-refusal/session-live area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: session-live
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=98 row=contract/revive-json-refusal/cwd-missing area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: cwd-missing
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=99 row=contract/revive-json-refusal/stale-heartbeat area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: stale-heartbeat
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_add n=100 row=contract/revive-json-refusal/pinned-hash-mismatch area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: pinned-hash-mismatch
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns.
@row_add n=101 row=contract/revive-json-refusal/squatted-name area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: squatted-name
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns.
@row_add n=102 row=contract/revive-json-refusal/internal-error area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: internal-error
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; a `claude` not found at a start reads as this document and has no human pair.
@row_add n=103 row=contract/revive-list/json area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --list
item: --json output
promise: `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order.
@row_add n=104 row=contract/revive-list/human-rows area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --list
item: human rows
promise: Without `--json`, `viola revive <name> --list` prints static ASCII rows `<ts>  <cause>  <id>`, with no header, no colour and no ESC byte.
@row_add n=105 row=contract/revive-human-refusal/pair area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive human refusals
item: unable/hint pair
promise: Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical.
@row_add n=106 row=contract/cli-usage-error/exit-2 area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: CLI exit codes
item: 2
promise: A clap usage error exits 2; `viola plugin` alone is such an error.
@row_add n=107 row=contract/interrupted-verify/stderr area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola verify interrupted
item: stderr lines
promise: A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII.
@row_add n=108 row=contract/interrupted-verify/exit area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola verify interrupted
item: exit 1
promise: A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1.
@row_add n=109 row=contract/plugin-install/human-output area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola plugin install
item: human output
promise: `viola plugin install` prints its four planning-form lines as one plain-ASCII write, no SGR, and no ESC or non-ASCII byte piped, under `NO_COLOR` or under `TERM=dumb`.
@row_add n=110 row=contract/plugin-install/json-output area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola plugin install
item: --json output
promise: Under `--json`, `viola plugin install` prints exactly `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`.
@row_add n=111 row=contract/plugin-install/exit-21 area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola plugin install
item: exit 21
promise: The exit-21 refusal of `viola plugin install` is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
@row_add n=112 row=contract/frame-verb-refusal/server-verify-failed area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: frame verb refusals
item: server-verify-failed
promise: send, wait, last, answer, pause, release, link and unlink refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty.
@row_add n=113 row=contract/cli-json-failure/document area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: CLI --json failures
item: failure document
promise: Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
@row_add n=114 row=contract/cli-json-failure/send-answer-exit-2 area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: CLI --json failures
item: send and answer exit 2
promise: The `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document.
@row_add n=115 row=contract/link-unlink/result-line area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: result line
promise: `viola link` and `viola unlink` each print one plain-ASCII result line in one write, with no SGR in any mode.
@row_add n=116 row=contract/link-unlink/json-ok area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: --json output
promise: Under `--json`, `viola link` and `viola unlink` each print one `{"v":1,"ok":<ok>}` document.
@row_add n=117 row=contract/link-unlink/exit-21 area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: exit 21
promise: `viola link` and `viola unlink` refuse with exit 21 a driven instance that fails the strict-modes check (before any log opens), a stopped driven instance, and a server that is not the recorded wrapper.
@row_add n=118 row=contract/link-unlink/exit-20 area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: exit 20
promise: A pair the wrapper refuses as invalid params reaches `viola link` or `viola unlink` as a fault: exit 20, no hint.
@row_add n=119 row=contract/frame-verb-refusal/strict-modes-failed area=a11y-scope-summary rec=fct-dssvg3hs@2
boundary: frame verb refusals
item: strict-modes-failed
promise: send, wait, last, answer, pause and release refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.
@row_add n=120 row=contract/frame-verb-refusal/socket-dir-refused area=a11y-scope-summary rec=fct-sjhco52d@2
boundary: frame verb refusals
item: socket-dir-refused
promise: Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found.
@row_add n=121 row=contract/start-refusal/home-strict-modes area=a11y-scope-summary rec=fct-hj6bq3jt@2
boundary: viola run, revive start and verify refusals
item: home strict-modes failure
promise: Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
@row_add n=122 row=contract/start-refusal/socket-dir area=a11y-scope-summary rec=fct-rts3hgep@2
boundary: viola run, revive start and verify refusals
item: socket directory failure
promise: Over a failing socket directory, `viola run` and a `viola revive` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.
@row_add n=123 row=contract/tui-passthrough/own-bytes area=a11y-scope-summary rec=fct-itb2wxsm@2,fct-twqx5x46@3
boundary: TUI passthrough
item: terminal bytes
promise: viola writes zero bytes to the terminal except the child's output; a passed `viola revive` writes no line of its own while the child holds the terminal.
@row_add n=124 row=contract/tui-passthrough/pump-child-error area=a11y-scope-summary rec=fct-itb2wxsm@2
boundary: TUI passthrough
item: pump_child error under revive --json
promise: Under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.
@row_add n=125 row=contract/tui-passthrough/refused-peer area=a11y-scope-summary rec=fct-itb2wxsm@2
boundary: TUI passthrough
item: refused peer
promise: A peer the wrapper refuses is dropped before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel.
@row_add n=126 row=contract/tui-passthrough/after-feed-panic area=a11y-scope-summary rec=fct-yscgzykm@2
boundary: TUI passthrough
item: after a contained vt100 feed panic
promise: The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do.
@row_add n=127 row=contract/hook-call/server-verify-failed area=a11y-scope-summary rec=fct-walt4vws@2
boundary: viola hook event and dialog calls
item: server not the recorded wrapper
promise: A hook event or dialog call that meets a server that is not the recorded wrapper sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s.
@row_add n=128 row=contract/mcp-frame-tool/server-verify-failed area=a11y-scope-summary rec=fct-walt4vws@2
boundary: MCP server frame tools
item: server-verify-failed
promise: An MCP frame tool (send, wait, last, answer) meeting a server that is not the recorded wrapper returns `isError` with code `instance-unreachable`, `detail` `server-verify-failed` (text `refused: instance-unreachable`) and sends no frame.
@row_add n=129 row=contract/hook-call/strict-modes-failed area=a11y-scope-summary rec=fct-6sgmn3kz@1
boundary: viola hook event and dialog calls
item: strict-modes failure
promise: A hook call refused by the strict-modes check returns exit 0 at once, empty stdout and stderr, under 1.0 s, with no `config.json` read, no stdin read, no frame, no line and no SessionEnd append; a `VIOLA_DIR` naming no instance directory writes nothing.
@row_add n=130 row=contract/mcp-frame-tool/strict-modes-failed area=a11y-scope-summary rec=fct-6sgmn3kz@1
boundary: MCP server frame tools
item: strict-modes-failed
promise: An MCP frame tool against an instance failing the strict-modes check returns `isError` with code `instance-unreachable`, `detail` `strict-modes-failed`, sends no frame and writes no line.
@row_add n=131 row=contract/hook-call/socket-dir-refused area=a11y-scope-summary rec=fct-w46xvuhu@1
boundary: viola hook event and dialog calls
item: socket directory failure
promise: A hook call against a failing socket directory sends no frame and fails open: exit 0, empty stdout and stderr, its one record a `hook-decision` at WARN with `detail` `socket-dir-refused`.
@row_add n=132 row=contract/mcp-frame-tool/socket-dir-refused area=a11y-scope-summary rec=fct-yupqihr2@1
boundary: MCP server frame tools
item: socket-dir-refused
promise: An MCP frame tool against a failing socket directory returns `isError` with code `instance-unreachable`, `detail` `socket-dir-refused`, sends no frame and writes no line.
@row_add n=133 row=decision/output-discipline-a11y-fallback area=a11y-scope-summary rec=fct-vtox22ts@4
name: Output-discipline assertions stand in for a11y tools
rule: CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON.
why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
@row_add n=134 row=decision/manual-a11y-pass-gates-nothing area=a11y-scope-summary rec=fct-vtox22ts@4,fct-twqx5x46@3
name: Manual screen-reader pass gates nothing
rule: The manual Orca screen-reader pass (GNOME Terminal for the cli, foot on Hyprland for the tui) is founder-owned and supplemental; nothing is gated on it.
@row_add n=135 row=contract/viola-list-board/order area=a11y-scope-summary rec=fct-vtox22ts@4,fct-p4tq3d4m@1,fct-ai3fe6uo@1
boundary: viola list board
item: structure
promise: The board is the BAY context line, the uppercase caption row and the wrapped rows, then the unwrapped read-only rows behind their separator.
@row_add n=136 row=contract/viola-list-board/empty-board area=a11y-scope-summary rec=fct-vtox22ts@4
boundary: viola list board
item: no wrapped sessions
promise: With no wrapped row, the board prints the BAY line, the caption row and `no wrapped sessions  start one: viola run <name> -- claude`.
@row_add n=137 row=contract/viola-list-board/unwrapped-separator area=a11y-scope-summary rec=fct-vtox22ts@4,fct-p4tq3d4m@1,fct-ai3fe6uo@1
boundary: viola list board
item: -- UNWRAPPED - READ-ONLY --
promise: The separator and unwrapped rows print only when the one `claude agents --json` read lists a session no wrapped row joins; unwrapped rows show the escaped `sessionId` as NAME and `n/a` in WHEEL, DIALOG and CLI; a failed, oversize or malformed read adds no separator and no unwrapped row.
@row_add n=138 row=contract/viola-list-board/colour area=a11y-scope-summary rec=fct-vtox22ts@4
boundary: viola list board
item: colour
promise: On a terminal `viola list` writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`.
@row_add n=139 row=contract/cli-help/group-labels area=a11y-scope-summary rec=fct-vtox22ts@4,fct-p4tq3d4m@1,fct-ai3fe6uo@1
boundary: viola --help
item: group labels
promise: The `--help` verb table is grouped under the labels board / traffic / wheel / handoff / setup.
@row_add n=140 row=contract/log-identity/service-name area=a11y-scope-summary rec=fct-vtox22ts@4,fct-twqx5x46@3
boundary: log identity tags
item: service_name
promise: Records carry `service_name:"viola"` and `version`.
@row_add n=141 row=contract/log-identity/process area=a11y-scope-summary rec=fct-vtox22ts@4,fct-twqx5x46@3
boundary: log identity tags
item: process
promise: `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run`.
@row_add n=142 row=decision/control-character-escaping area=a11y-scope-summary rec=fct-vtox22ts@4
name: C0/C1 control characters escaped
rule: CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two.
why: the list-row exception is ratified (T5)
except: in `viola list` table rows `\n` and `\t` are escaped too
@row_none n=53 rec=fct-mc7fijew@1 type=4
@row_add n=143 row=decision/run-diagnostics-off-terminal area=a11y-scope-summary rec=fct-twqx5x46@3
name: Run diagnostics never reach the terminal
rule: viola's diagnostics for `run` go to `diagnostics/`, never to the terminal.
@row_add n=144 row=decision/linux-only area=a11y-scope-summary rec=fct-6wf4lqpi@1
name: Linux only
rule: Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
@row_add n=145 row=contract/cli-help/usage-line area=aria-patterns-roles rec=fct-ai3fe6uo@1
boundary: viola --help
item: usage line
promise: `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the group rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, then `Options:`, plain ASCII.
@row_add n=146 row=contract/cli-help/served-verbs-only area=aria-patterns-roles rec=fct-ai3fe6uo@1
boundary: viola --help
item: served verbs only
promise: The help group rows list the served verbs only: `plugin install` is one entry in `setup:` and a bare `plugin` is unserved.
@row_add n=147 row=contract/cli-help/handoff-row area=aria-patterns-roles rec=fct-ai3fe6uo@1,fct-p4tq3d4m@1
boundary: viola --help
item: handoff row
promise: The help `handoff:` row is `handoff:  link, unlink`, between `wheel:` and `setup:`.
@row_none n=54 rec=fct-u64ggudd@1 type=0
what: a table row marking ARIA landmarks as not applicable to the vendor claude TUI
@row_add n=148 row=contract/cli-exit-codes/10 area=aria-patterns-roles rec=fct-dx4kie52@3,fct-6edxk4iz@3,fct-avr5o2ub@3
boundary: CLI exit codes
item: 10
promise: human-typing: the human holds the wheel, so the automated send is refused with nothing typed.
@row_add n=149 row=decision/linear-static-cli-output area=aria-patterns-roles rec=fct-uok334yy@6
name: Linear static CLI output
rule: CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
@row_add n=150 row=contract/viola-list-board/caption area=aria-patterns-roles rec=fct-uok334yy@6,fct-urfjsgc5@1
boundary: viola list board
item: caption order
promise: The caption order is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, reused by the terminal fleet console.
@row_add n=151 row=decision/no-sgr-off-terminal area=aria-patterns-roles rec=fct-uok334yy@6,fct-es7le4as@4
name: No SGR off a terminal
rule: viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive.
guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
@row_add n=152 row=decision/sgr-beside-its-word area=aria-patterns-roles rec=fct-uok334yy@6,fct-es7le4as@4
name: SGR only beside its word
rule: Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured.
guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal
@row_add n=153 row=decision/control-bytes-escaped area=aria-patterns-roles rec=fct-uok334yy@6
name: Control bytes printed escaped
rule: Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line.
why: security control for C0/C1 escaping, ratified for list rows
guard: escape_row_shows_every_control_character_as_hex_text
@row_none n=55 rec=fct-jgpx3odq@2 type=5
@row_add n=154 row=decision/time-bounds-fall-on-the-driver area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Time bounds fall on the driver only
rule: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, budget pause, statusline deadline, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
@row_add n=155 row=contract/viola-wait/timed-out area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola wait
item: timed_out
promise: viola wait returns timed_out true only for a timeout the caller passes in.
@row_add n=156 row=contract/mcp-wait-tool/timeout-ms area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: MCP wait tool
item: timeout_ms
promise: Clamped to 50 000 ms, which is also the default when none is passed, so a wait with no timeout is answered timed-out after 50 s.
@row_add n=157 row=contract/config-json/dialog-hold-s area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: config.json
item: dialog_hold_s
promise: Whole seconds 60 to 3600, default 60, read once at the wrapper's start; any other value gives 60 and one parse-rejected.
@row_add n=158 row=decision/dialog-hold-bounds-the-driver area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Dialog hold bounds the driver
rule: A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold.
state: provisional
@row_add n=159 row=contract/event-line/dialog-expired area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: the event line
item: dialog-expired
promise: Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait.
@row_add n=160 row=contract/cli-exit-codes/13 area=cognitive-accessibility rec=fct-6edxk4iz@3,fct-dx4kie52@3,fct-avr5o2ub@3
boundary: CLI exit codes
item: 13
promise: An answer for an expired or absent dialog is refused not-delivered / unknown-dialog.
@row_add n=161 row=contract/send-refusal/input-not-ready area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola send refusal
item: input-not-ready
promise: A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed.
@row_add n=162 row=contract/send-refusal/no-prompt-submitted area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola send refusal
item: no-prompt-submitted
promise: A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted.
@row_add n=163 row=contract/viola-hook-statusline/exit area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola hook statusline
item: exit status
promise: Runs the user's own status line command under a 5 s deadline and exits 0 with empty stderr in every case.
@row_add n=164 row=decision/claude-agents-read-fails-open area=cognitive-accessibility rec=fct-6edxk4iz@3
name: claude agents read fails open
rule: A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's preflight.
@row_add n=165 row=decision/budget-gate area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Budget gate on automated sends
rule: While a five-hour or seven-day window stands at or over its threshold with a known reset ahead, an automated viola send is refused budget-paused with nothing typed, until that reset or viola release --budget; no threshold value switches the gate off.
guard: path6_a_reading_over_the_threshold_refuses_the_send
@row_add n=166 row=decision/budget-gate-only-on-send area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Only send reads the budget gate
rule: answer, wait, last and pause do not read the budget gate and give under a closed gate what they give under an open one.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_add n=167 row=contract/cli-exit-codes/11 area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: CLI exit codes
item: 11
promise: budget-paused, with detail five-hour or seven-day.
@row_add n=168 row=contract/config-json/five-hour area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: config.json
item: five_hour
promise: The five-hour window's budget threshold, default 90.
@row_add n=169 row=contract/config-json/seven-day area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: config.json
item: seven_day
promise: The seven-day window's budget threshold, default 85.
@row_add n=170 row=decision/error-word-detail-hint area=cognitive-accessibility rec=fct-n7d5inyr@1
name: Errors as word, detail and hint
rule: Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
@row_add n=171 row=contract/cli-refusal/stderr-layout area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: CLI refusal
item: stderr layout
promise: unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
@row_add n=172 row=decision/unknown-refusal-has-no-hint area=cognitive-accessibility rec=fct-n7d5inyr@1
name: unknown refusal carries no hint
rule: The unknown refusal prints no hint.
why: its detail is a code word or nothing, opaque to a reader, so no hint is printed rather than a guess
@row_add n=173 row=contract/cli-exit-codes/14 area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-dvgtuxyi@1
boundary: CLI exit codes
item: 14
promise: unknown: a refusal whose word is outside the closed refusal list.
@row_add n=174 row=decision/faults-carry-no-hint area=cognitive-accessibility rec=fct-n7d5inyr@1
name: Faults carry no hint
rule: Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
@row_add n=175 row=decision/json-one-document-no-hint area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-wkabfqtn@1
name: One --json document with no hint
rule: Under --json a failure gives one document with no hint.
except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document
@row_add n=176 row=contract/cli-fixed-message/form area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: CLI fixed-message refusal
item: form
promise: viola revive's exit-1 refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
@row_add n=177 row=contract/cli-exit-codes/2 area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-wkabfqtn@1
boundary: CLI exit codes
item: 2
promise: A usage error, or an unusable answer response or send text written as one fixed stderr line with no document.
@row_add n=178 row=contract/start-json-refusal/document area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: run / revive start under --json
item: refusal document
promise: Every refusal of the start revive shares with run gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
@row_add n=179 row=contract/viola-verify/interrupted-pair area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: viola verify
item: interrupted pair
promise: unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.
@row_add n=180 row=contract/cli-exit-codes/21 area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-bhnorrcm@1
boundary: CLI exit codes
item: 21
promise: instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail.
@row_add n=181 row=contract/instance-unreachable-json/document area=cognitive-accessibility rec=fct-bhnorrcm@1
boundary: exit 21 under --json
item: document
promise: One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC.
@row_add n=182 row=contract/instance-unreachable-detail/strict-modes-failed area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-zlmstg36@1,fct-mxmekxqi@1
boundary: instance-unreachable detail
item: strict-modes-failed
promise: The instance failed the strict-modes check; refused before any frame or log, no log line written.
@row_add n=183 row=contract/instance-unreachable-detail/instance-dead area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-tssjslea@1,fct-wqxezfp7@1,fct-xzkz7422@1
boundary: instance-unreachable detail
item: instance-dead
promise: The name has no live endpoint but its instance directory stands, or the instance is stale.
@row_add n=184 row=contract/instance-unreachable-detail/unwrapped-name area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-wqxezfp7@1,fct-xzkz7422@1
boundary: instance-unreachable detail
item: unwrapped-name
promise: The name has no live endpoint and no instance directory (absent, a regular file or a symlink).
@row_add n=185 row=contract/instance-unreachable-detail/endpoint-lost area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-tssjslea@1
boundary: instance-unreachable detail
item: endpoint-lost
promise: The wrapper was lost with the call open.
@row_add n=186 row=contract/instance-unreachable-detail/server-verify-failed area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-n7d5inyr@1
boundary: instance-unreachable detail
item: server-verify-failed
promise: The process serving the endpoint is not the recorded wrapper and was not contacted.
@row_add n=187 row=contract/instance-unreachable-detail/socket-dir-refused area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-gh3uczd3@1
boundary: instance-unreachable detail
item: socket-dir-refused
promise: The socket directory was refused; the verb sent no frame.
@row_add n=188 row=decision/strict-modes-refusal-before-any-write area=cognitive-accessibility rec=fct-zlmstg36@1,fct-mxmekxqi@1,fct-zwbgfalb@1,fct-bjgpzcmx@1,fct-xzkz7422@1
name: Strict-modes refusal before any write
rule: A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify.
@row_add n=189 row=decision/socket-dir-refusal-before-any-frame area=cognitive-accessibility rec=fct-gh3uczd3@1,fct-cgrssu3w@1
name: Socket-directory refusal before any frame
rule: A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start.
@row_add n=190 row=contract/start-holder-refusal/stale-heartbeat area=cognitive-accessibility rec=fct-45izkccw@1,fct-tssjslea@1
boundary: start against a holder, exit 1
item: stale-heartbeat
promise: The holder is stale; stale heartbeat is exit 1's cause alone.
@row_add n=191 row=contract/start-holder-refusal/already-live area=cognitive-accessibility rec=fct-45izkccw@1
boundary: start against a holder, exit 1
item: already-live
promise: The holder is live, or a fresh replayed beat was read.
@row_add n=192 row=decision/unstattable-home-is-strict-modes-refusal area=cognitive-accessibility rec=fct-ms6n7636@2
name: Unstattable home is a strict-modes refusal
rule: A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
@row_add n=193 row=decision/printed-names-are-parsed-arguments area=cognitive-accessibility rec=fct-5kq5fzne@1,fct-adl57ilr@1
name: Printed names are the parsed arguments
rule: Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log.
@row_add n=194 row=decision/link-pair-validated-by-wrapper area=cognitive-accessibility rec=fct-uyauvo6g@1
name: Link pairs validated by the wrapper
rule: viola link and viola unlink never refuse a pair client-side; a pair the wrapper refuses as invalid params reaches the verb as a fault, exit 20, no hint.
@row_add n=195 row=contract/cli-exit-codes/20 area=cognitive-accessibility rec=fct-uyauvo6g@1
boundary: CLI exit codes
item: 20
promise: A fault, with no hint.
@row_add n=196 row=contract/code-word/format area=cognitive-accessibility rec=fct-yq4bu6pq@1
boundary: code word
item: format
promise: 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
@row_add n=197 row=decision/peer-replies-screened-to-code-words area=cognitive-accessibility rec=fct-yq4bu6pq@1
name: Peer replies screened to code words
rule: A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
@row_add n=198 row=decision/unlisted-code-words-stay-visible area=cognitive-accessibility rec=fct-dvgtuxyi@1
name: Unlisted code words stay visible
rule: A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14.
why: visible by design
@row_add n=199 row=contract/fault-body/message area=cognitive-accessibility rec=fct-lwbpjniq@1
boundary: fault body under --json and on MCP
item: message
promise: A code outside the five known reads the fixed message wrapper fault with null data.
@row_add n=200 row=contract/human-fault-line/wrapper-fault area=cognitive-accessibility rec=fct-lwbpjniq@1
boundary: human fault line
item: wrapper fault
promise: error: wrapper fault  <code>.
@row_add n=201 row=contract/json-fault/internal-error area=cognitive-accessibility rec=fct-aaun2rti@1
boundary: CLI fault under --json
item: internal-error
promise: Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there.
@row_add n=202 row=decision/client-euid-refusal-test-strategy area=cognitive-accessibility rec=fct-g3h7n6e2@4
name: Client euid refusal test strategy
rule: No foreign peer is driven: the client euid refusal's decision is tested over injected euids and its real read on Linux, and each client seam's wiring is read only by forced refusals in remove-the-guard runs.
@row_none n=56 rec=fct-cetk7x7u@2 type=5
@row_none n=57 rec=fct-52gvy64u@2 type=5
@row_none n=58 rec=fct-trymhbcp@2 type=5
@row_add n=203 row=decision/json-failure-documents-are-clean area=cognitive-accessibility rec=fct-5fnuqr36@1
name: --json failure documents are clean
rule: Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
guard: cli_error_documents_hold_no_path_or_upstream_text
@row_add n=204 row=decision/error-bodies-have-writer-and-witness area=cognitive-accessibility rec=fct-fuvobsk5@1
name: Error bodies have a writer and a witness
rule: Each error body names a writer and a witness that stand, and each product error writer is named.
guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
@row_add n=205 row=decision/thiserror-messages-interpolate-nothing area=cognitive-accessibility rec=fct-fuvobsk5@1
name: thiserror messages interpolate nothing
rule: No thiserror message interpolates a value.
guard: thiserror_messages_interpolate_nothing
@row_add n=206 row=decision/one-hint-per-code area=cognitive-accessibility rec=fct-zzwwqxzg@1
name: One hint per code
rule: No two codes share a hint.
guard: no_two_codes_share_a_hint
@row_add n=207 row=decision/no-hint-names-release area=cognitive-accessibility rec=fct-zzwwqxzg@1,fct-avr5o2ub@3
name: No hint names release
rule: No hint names the release verb.
guard: no_hint_names_release
@row_none n=59 rec=fct-dmbuxeqb@1 type=5
@row_none n=60 rec=fct-kia5lupj@1 type=5
@row_add n=208 row=decision/fixed-cli-strings-pinned-by-literal area=cognitive-accessibility rec=fct-oji2cu7x@2
name: Fixed CLI strings pinned by literal
rule: Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
@row_add n=209 row=decision/link-result-lines-one-write-no-sgr area=cognitive-accessibility rec=fct-tfdrn5u5@1
name: Link result lines in one write, no SGR
rule: The link and unlink result lines are each one write with no SGR in any mode.
guard: link_verb_lines_are_the_design_lines_in_one_write
@row_add n=210 row=contract/viola-link-line/time area=cognitive-accessibility rec=fct-adl57ilr@1
boundary: viola link line
item: time
promise: The reply's since as UTC clock time in whole seconds with Z.
@row_add n=211 row=decision/state-carried-by-printed-word area=visual-design-verification rec=fct-es7le4as@4
name: State carried by its printed word
rule: Each board state is carried by its printed word (stale, DIALOG <kind>, expired, budget-paused, unverified-cli, idle, busy, n/a), never by colour alone.
@row_add n=212 row=decision/colour-decision-inputs area=visual-design-verification rec=fct-es7le4as@4
name: Colour decision inputs
rule: The colour decision reads NO_COLOR (present, empty included), TERM (exactly dumb; 256color), COLORTERM and whether stdout is a terminal, and picks a truecolor, 256 or 16 depth.
@row_add n=213 row=decision/unwrapped-name-truncation area=visual-design-verification rec=fct-es7le4as@4
name: Unwrapped NAME truncation
rule: On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus ..., never inside an escape; a wrapped NAME is never cut, and piped output and --json keep every name whole.
guard: cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it
@row_add n=214 row=contract/viola-list-stale-row/escape-sequences area=visual-design-verification rec=fct-46bk3wap@2
boundary: viola list stale row on a terminal
item: escape sequences
promise: Exactly DIM, BOLD, RESET, DIM, RESET, with the word stale printed beside the dim.
@row_add n=215 row=decision/cli-a11y-by-literal-tests area=a11y-testing-tool-pick rec=fct-dlpkde76@3
name: CLI a11y checked by literal tests
rule: With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.
@row_add n=216 row=decision/tui-boundary-driver-pin area=a11y-testing-tool-pick rec=fct-dlpkde76@3
name: TUI boundary driver
rule: The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins.
@row_none n=61 rec=fct-mzgdpnxi@1 type=S
@row_add n=217 row=absence/rendered-page area=bootstrap-phases-derive-for-route-setup-project rec=fct-trh3q5nm@3,fct-lfnx2oea@2,fct-d5bgiald@2,fct-56elcbkj@3
capability: A rendered page, and with it any browser a11y tooling
@row_none n=62 rec=fct-agfwiito@1 type=7
@row_add n=218 row=decision/manual-screen-reader-pass area=bootstrap-phases-derive-for-route-setup-project rec=fct-t3p7hz2f@2
name: Manual screen reader pass
rule: The screen reader check is a manual Orca pass owned by the founder, with no automated test spec.
@row_add n=219 row=decision/a11y-runs-in-existing-test-suites area=bootstrap-phases-derive-for-route-setup-project rec=fct-yftct3hq@2,fct-gcqo3m4v@2
name: a11y runs in the existing test suites
rule: The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.
@row_add n=220 row=decision/a11y-failures-are-nextest-failures area=bootstrap-phases-derive-for-route-setup-project rec=fct-ycol6rsi@1,fct-s3apctwx@1
name: a11y failures are nextest failures
rule: An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
@row_none n=63 rec=fct-b5v555it@1 type=0
what: a note on how the route and setup-project skills use the bootstrap phase list
@row_none n=64 rec=fct-3siidzor@1 type=S
@row_add n=221 row=decision/gate-requires-coverage-doctest area=ci-integration rec=fct-56elcbkj@3
name: Gate requires coverage and doctest
rule: The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.
@row_add n=222 row=decision/stdin-classifier-non-editing-list area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Closed non-editing stdin list
rule: Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel.
why: ratified, the exact grammars chosen after a narrower option was shown
@row_add n=223 row=decision/human-key-takes-the-wheel area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Human key takes the wheel
rule: A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed.
guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
@row_add n=224 row=decision/zero-viola-bytes-on-the-terminal area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Zero viola bytes on the terminal
rule: The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run.
@row_add n=225 row=decision/statusline-output-passed-byte-for-byte area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Statusline output passed byte for byte
rule: viola hook statusline hands the user's own status line output to the child byte for byte and writes nothing to the terminal itself.
guard: passthrough_with_a_statusline_source_adds_no_viola_bytes
@row_add n=226 row=decision/live-readings-never-gate area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Live readings never gate
rule: Live readings of the takeover clause on a real terminal are supplemental and never gating.
@row_add n=227 row=decision/boundary-tests-sequenced-on-records area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Boundary tests sequenced on records
rule: The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
@row_add n=228 row=decision/boundary-check-never-inspects-claude-content area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Boundary check never inspects claude content
rule: The keyboard boundary check never inspects claude content.
@row_add n=229 row=decision/cli-screen-reader-test-setup area=screen-reader-test-pattern rec=fct-tocuc2rh@2
name: CLI screen-reader test setup
rule: The cli surface is screen-reader tested with Orca on GNOME Terminal.
@row_add n=230 row=decision/tui-screen-reader-test-setup area=screen-reader-test-pattern rec=fct-tocuc2rh@2
name: TUI screen-reader test setup
rule: The tui surface is screen-reader tested with Orca on foot under Hyprland.
@row_add n=231 row=contract/structured-violation-json/cli-tui-a11y-rows area=structured-violation-json-schema rec=fct-7jyl4oxv@1
boundary: Structured violation JSON
item: a11y rows for cli and tui
promise: The cli and tui surfaces emit no a11y rows.
@row_add n=232 row=contract/structured-violation-json/suites-failures area=structured-violation-json-schema rec=fct-7jyl4oxv@1
boundary: Structured violation JSON
item: suites[].failures[]
promise: Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
@row_add n=233 row=decision/no-assertions-for-plugin-and-ipc area=1-a11y-scope-summary rec=fct-lgjt6tbz@1,fct-rgudbdwz@1
name: Plugin and IPC channel not assertable
rule: The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions.
why: ipc-internal with no UI surface, machine-to-machine traffic only
@row_add n=234 row=decision/no-assertions-for-internal-crates area=1-a11y-scope-summary rec=fct-jfjbxqvy@1,fct-rgudbdwz@1
name: Internal crates not assertable
rule: The internal crates of the modular monolith are not-assertable and carry no assertions.
why: library-only, with no UI surface
@row_add n=235 row=decision/no-wcag-conformance-claim area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-lpulrs7q@1,fct-ms7djxet@3,fct-vtox22ts@4,fct-twqx5x46@3
name: No WCAG conformance claim
rule: No WCAG conformance is claimed for the cli or the `viola run` TUI, whether `viola run` or a passed `viola revive` started it; the output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 and are not a11y-tool evidence.
why: WCAG does not apply directly to terminal output, and no automated a11y tool reaches terminal output
@row_add n=236 row=decision/no-sgr-in-plain-modes area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-6ghxxgxr@1,fct-633dmm7e@1,fct-vtox22ts@4
name: No SGR in plain modes
rule: No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
@row_add n=237 row=decision/paths-not-carried area=1-a11y-scope-summary rec=fct-zggdjhnj@1
name: Paths not carried
rule: Tests Critical Path 1 (the `run` start sequence), E1 (unwrapped hooks no-op), E2 (`CLAUDE*` env strip) and security vectors 1–9 are not carried as paths; their user-visible outcomes are covered by Path 4.
why: they run on tui/ipc-internal with no viola-owned UI
@row_add n=238 row=decision/cognitive-accessibility-not-triggered area=1-a11y-scope-summary rec=fct-2ylyatq6@1
name: Cognitive accessibility not triggered
rule: The cognitive-accessibility trigger is none: no cognitive-accessibility rigor is required.
why: the brief has no cognitive, older-user or language-proficiency signal
@row_add n=239 row=decision/tier-single-source-of-rigor area=1-a11y-scope-summary rec=fct-ov6v75dw@1
name: Tier as single source of accessibility rigor
rule: The tier is the single source of truth for how much accessibility rigor downstream skills apply, in the phase loop per chunk and in cross-cutting reads from route and setup-project.
@row_add n=240 row=decision/human-keystrokes-never-blocked area=2-a11y-strategy rec=fct-7a7ijng6@1,fct-pzkrh6fa@1,fct-csjr2vg7@2,fct-itb2wxsm@2
name: Human keystrokes never blocked
rule: A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current atomic paste.
@row_add n=241 row=decision/cli-output-discipline-assertion-tools area=4-aria-patterns-roles rec=fct-yzqlnwsx@4
name: CLI assertable as output discipline
rule: The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
@row_add n=242 row=decision/no-terminal-conformance-claim area=4-aria-patterns-roles rec=fct-yzqlnwsx@4,fct-ijnl2u6y@4,fct-mkekg7f5@1,fct-uok334yy@6,fct-6edxk4iz@3,fct-es7le4as@4
name: No conformance claim for the terminal
rule: The cli checks make no conformance claim for the terminal; they only stand in for SC 1.3.2, 1.4.1 and 3.3.1.
@row_add n=243 row=decision/portable-pty-outer-pty-driver area=4-aria-patterns-roles rec=fct-4ktfiloi@3,fct-twqx5x46@3
name: portable-pty as the outer-PTY boundary driver
rule: The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
@row_add n=244 row=decision/wrapped-tui-reads-as-unwrapped area=7-screen-reader-support rec=fct-wawttggc@2,fct-tocuc2rh@2
name: Wrapped TUI reads as unwrapped
rule: The wrapped `claude` TUI must read to a screen reader the same as unwrapped.
why: Must-Work O1
@row_add n=245 row=decision/manual-checks-supplemental-only area=11-a11y-anti-patterns-never-do-these rec=fct-3vxnlnbx@1
name: Manual checks are supplemental only
rule: Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
@row_add n=246 row=contract/revive-json-refusal/socket-dir-refused area=a11y-scope-summary rec=fct-k76k2lf7@2,fct-cgrssu3w@1
boundary: viola revive --json refusals
item: socket-dir-refused
promise: One exit-1 document with `socket-dir-refused` its code on stdout, in place of the human pair, before any snapshot, bind or child spawn, holding no path, pid or uid, with stderr empty and no hint.
