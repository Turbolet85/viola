@row_add n=1 row=decision/a11y-tier-standard area=1-a11y-scope-summary rec=fct-loqatd4n@1
name: A11y tier Standard (1)
rule: The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
@row_check n=1 row=decision/a11y-tier-standard sha=507d2e04ecf6 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=2 row=absence/web-or-graphical-frontend area=1-a11y-scope-summary rec=fct-tzchk23h@2
capability: A web or graphical frontend
@row_check n=2 row=absence/web-or-graphical-frontend sha=6fce34e0d025 v=holds judge=sonnet head=8834195b0683
searched: indicatif|egui|iced|tauri|axum|actix|wasm · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs
searched: playwright|pa11y|axe-core|lighthouse|cypress|puppeteer · in .github/workflows/*.yml, scripts/*
@row_add n=3 row=decision/no-a11y-tool-output-discipline-assertions area=1-a11y-scope-summary rec=fct-xmkdtzxa@3,fct-ulpihvwi@1,fct-sf2opzaw@1
name: Output-discipline assertions instead of an a11y tool
rule: cli and tui use no a11y tool: the existing process, literal-pin and portable-pty output-discipline assertions on text and streams stand in, with manual screen-reader passes as a supplement.
why: no automated a11y tool reaches a terminal
@row_check n=3 row=decision/no-a11y-tool-output-discipline-assertions sha=550132e955d2 v=holds judge=sonnet head=8834195b0683
searched: pa11y|axe-core|lighthouse|playwright|cypress|puppeteer · in Cargo.toml, crates/*/Cargo.toml, .github/workflows/*.yml, scripts/*
@row_add n=4 row=contract/viola-revive-json/start-refusal area=1-a11y-scope-summary rec=fct-z2htpz2e@2
boundary: viola revive --json output
item: start refusal
promise: Every refusal of the start that revive shares with run prints one exit-1 document, pinned-hash-mismatch and squatted-name among them, and a program not found as internal-error.
@row_check n=4 row=contract/viola-revive-json/start-refusal sha=263612132de7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:270
quote: Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
@row_add n=5 row=contract/viola-revive-json/pump-child-error area=1-a11y-scope-summary rec=fct-z2htpz2e@2
boundary: viola revive --json output
item: pump_child error
promise: A pump_child error prints no document: the child has held the terminal, and viola writes nothing past it.
@row_check n=5 row=contract/viola-revive-json/pump-child-error sha=a77d5dab00aa v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:122-128
quote: let started = started(home, args).inspect_err(|_| { if json { client::refused_document("internal-error"); } })?; match started { Started::Launched(launched) => pump_child(*launched),
@row_add n=6 row=contract/tui-passthrough/entry area=1-a11y-scope-summary rec=fct-rdegquaa@2
boundary: viola run TUI passthrough
item: entry
promise: The passthrough is entered by viola run and by a viola revive whose preflight passed.
@row_check n=6 row=contract/tui-passthrough/entry sha=69e35416ce38 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:188
quote: Started::Launched(launched) => pump_child(*launched),
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=7 row=contract/tui-passthrough/vendor-content area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-weaq66l7@1
boundary: viola run TUI passthrough
item: vendor content
promise: The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI.
@row_check n=7 row=contract/tui-passthrough/vendor-content sha=f12b989339ba v=stale judge=sonnet head=8834195b0683
at: src/run/gate.rs:169
quote: .verdict(self.shared.sigs, waiting_since, clock.now());
at: src/run/gate.rs:81
quote: parser = "vt100-feed",
shows: The child's rendered output is fed into a vt100 screen model (gate.rs:135 offers every written byte) and read for readiness signatures, so the content is parsed.
cell: promise
value: The rendered content belongs to the vendor claude and is not traced; viola parses it through a vt100 feed, and no conformance claim is made about the claude TUI.
@row_add n=8 row=contract/tui-passthrough/viola-terminal-bytes area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-weaq66l7@1
boundary: viola run TUI passthrough
item: viola terminal bytes
promise: viola writes zero bytes of its own to the terminal; only the child's output reaches it.
@row_check n=8 row=contract/tui-passthrough/viola-terminal-bytes sha=9d1cb16a09ae v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
@row_add n=9 row=contract/tui-passthrough/human-keystrokes area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-weaq66l7@1,fct-gglbhu4k@1,fct-hu3qpzu6@2,fct-67rr3koq@1
boundary: viola run TUI passthrough
item: human keystrokes
promise: Human keystrokes are never blocked, refused or delayed past the current atomic paste.
@row_check n=9 row=contract/tui-passthrough/human-keystrokes sha=ab76e9174376 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:48
quote: if to.write_all(&buf[..n]).and_then(|()| to.flush()).is_err() {
@row_add n=10 row=contract/tui-passthrough/focus-mouse-resize-sequences area=1-a11y-scope-summary rec=fct-rdegquaa@2,fct-hu3qpzu6@2
boundary: viola run TUI passthrough
item: focus, mouse and resize sequences
promise: Focus, mouse and resize sequences and terminal replies pass through and do not count as human editing for wheel purposes.
@row_check n=10 row=contract/tui-passthrough/focus-mouse-resize-sequences sha=f54dd091ea86 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:445
quote: !reply
at: src/run/wheel.rs:491
quote: fn is_reply(params: &[u8], fin: u8) -> bool {
@row_add n=11 row=contract/tui-passthrough/vt100-feed-panic area=1-a11y-scope-summary rec=fct-rdegquaa@2
boundary: viola run TUI passthrough
item: vt100 feed panic
promise: The boundary still holds after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes still reach it.
@row_check n=11 row=contract/tui-passthrough/vt100-feed-panic sha=1de37cb7fc10 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
at: src/run/gate.rs:134
quote: let written = self.output.write(bytes)?;
@row_add n=12 row=decision/single-vendor-content-boundary area=1-a11y-scope-summary rec=fct-c5w2jfcj@1
name: Single vendor-content boundary
rule: The viola run TUI passthrough, which a passed viola revive also enters, is the only boundary-only vendor-content zone.
@row_check n=12 row=decision/single-vendor-content-boundary sha=07a47c2b9925 v=unverifiable judge=sonnet head=8834195b0683
searched: passthrough|style::read · in src/**/*.rs
why: no evidence found
@row_add n=13 row=decision/no-compliance-regime area=1-a11y-scope-summary rec=fct-btfrs2pb@1,fct-2b735qj7@1,fct-wvygpu6t@1
name: No accessibility compliance regime
rule: No accessibility compliance regime applies, so no regime-mandated WCAG success criterion binds any surface.
@row_check n=13 row=decision/no-compliance-regime sha=81fa74957e55 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=14 row=decision/colour-second-cue-only area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-67rr3koq@1,fct-lpulrs7q@1,fct-6ghxxgxr@1
name: Colour only a second cue
rule: Colour is never the sole cue on the CLI: the word always sits beside any SGR.
@row_check n=14 row=decision/colour-second-cue-only sha=68bc631f91ad v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=15 row=decision/static-linear-output area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-pgo5ntgv@1,fct-q7isztzg@1
name: Static linear output
rule: viola's own output is static and linear, with no spinner or redraw.
@row_check n=15 row=decision/static-linear-output sha=537dd082963b v=holds judge=sonnet head=8834195b0683
searched: indicatif|spinner|redraw · in src/**/*.rs, Cargo.toml, crates/*/Cargo.toml
@row_add n=16 row=contract/cli-output/control-characters area=1-a11y-scope-summary rec=fct-sf2opzaw@1
boundary: CLI output
item: C0/C1 control characters
promise: C0/C1 control characters are escaped except \n and \t, which list rows escape too.
@row_check n=16 row=contract/cli-output/control-characters sha=1f9d11723f51 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:8
quote: if c.is_control() && c != '\n' && c != '\t' {
at: src/human/escape.rs:23
quote: if c.is_control() {
@row_add n=17 row=decision/tui-boundary-assertions-only area=1-a11y-scope-summary rec=fct-weaq66l7@1,fct-gglbhu4k@1
name: TUI boundary assertions only
rule: The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim.
@row_check n=17 row=decision/tui-boundary-assertions-only sha=c0a2103463c5 v=holds judge=sonnet head=8834195b0683
at: tests/support/outer_pty.rs:78
quote: let mut pty = viola_pty::spawn(&spec).expect("outer pty spawn");
@row_add n=18 row=contract/tui-passthrough/run-diagnostics area=1-a11y-scope-summary rec=fct-weaq66l7@1
boundary: viola run TUI passthrough
item: run diagnostics
promise: viola's diagnostics for run go to diagnostics/, never to the terminal.
@row_check n=18 row=contract/tui-passthrough/run-diagnostics sha=22d4d22a5b26 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:87
quote: let dir = home.join("diagnostics");
@row_add n=19 row=decision/cli-not-keyboard-interactive area=1-a11y-scope-summary rec=fct-gglbhu4k@1
name: CLI not keyboard-interactive
rule: The CLI has no prompt, TUI mode or pager; its input comes only from stdin or --file.
@row_check n=19 row=decision/cli-not-keyboard-interactive sha=2734e9c29da3 v=holds judge=sonnet head=8834195b0683
searched: dialoguer|rustyline|inquire|pager · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs
@row_add n=20 row=decision/manual-keyboard-pass area=1-a11y-scope-summary rec=fct-gglbhu4k@1,fct-lvifofdb@1
name: Manual keyboard pass as supplement
rule: A manual keyboard pass over CLI and TUI supplements the automated keystroke boundary assertion.
@row_check n=20 row=decision/manual-keyboard-pass sha=e7f4d3fbf2d8 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=21 row=decision/manual-orca-pass area=1-a11y-scope-summary rec=fct-orpg5jsb@2,fct-ol6ygl55@2
name: Manual Orca screen-reader pass
rule: A manual Orca screen-reader pass on Section 4 P4 and P6, founder-owned, supplements the automated assertions.
why: no blind or low-vision target users are named, so screen-reader priority is scoped to CLI/TUI linearity, not a full assistive-tech matrix
@row_check n=21 row=decision/manual-orca-pass sha=2e58af5882de v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=22 row=decision/no-browser-a11y-leg area=1-a11y-scope-summary rec=fct-ktqfqjb4@3
name: No browser a11y leg
rule: CI runs no browser a11y leg.
why: viola renders no page
@row_check n=22 row=decision/no-browser-a11y-leg sha=1ba5caa1833f v=holds judge=sonnet head=8834195b0683
searched: playwright|pa11y|axe-core|lighthouse|cypress|puppeteer · in .github/workflows/*.yml, scripts/*
@row_add n=23 row=decision/a11y-inside-existing-suites area=1-a11y-scope-summary rec=fct-ktqfqjb4@3
name: A11y assertions inside existing suites
rule: A11y and CLI output-discipline assertions run inside the existing nextest-integration, nextest-e2e and coverage suites; no new suite enum value such as a11y is added without a Decisions Log entry.
@row_check n=23 row=decision/a11y-inside-existing-suites sha=c033f25386a0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:14-23
quote: pub const SUITES: [&str; 8] = [ "nextest-unit", "nextest-integration", "doctest", "nextest-e2e", "mutants", "coverage", "perf", "fuzz-replay", ];
@row_add n=24 row=decision/ci-test-gate-no-skips area=1-a11y-scope-summary rec=fct-ktqfqjb4@3
name: CI test gate allows no skips
rule: The CI test job's gate requires coverage and doctest and never allows skips.
@row_check n=24 row=decision/ci-test-gate-no-skips sha=eba4af5dff43 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
at: crates/viola-e2e/src/harness/gate.rs:89
quote: for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] {
@row_add n=25 row=decision/critical-path-send-readback area=1-a11y-scope-summary rec=fct-zvuuhh2n@1,fct-utn3b3p5@1,fct-4md23dny@1
name: Critical path: confirmed send and readback
rule: Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path.
@row_check n=25 row=decision/critical-path-send-readback sha=6edf3adc2b0f v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=26 row=decision/critical-path-dialog-answer area=1-a11y-scope-summary rec=fct-tauqfaof@1,fct-opu4qsrg@1,fct-4md23dny@1
name: Critical path: dialog pending, then answer
rule: A pending dialog answered with viola answer (question, permission, plan), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
@row_check n=26 row=decision/critical-path-dialog-answer sha=69eba6485aaa v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=27 row=decision/critical-path-human-takes-wheel area=1-a11y-scope-summary rec=fct-gxtjio4p@1,fct-4wknsfov@1,fct-4md23dny@1
name: Critical path: human takes the wheel
rule: The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path.
@row_check n=27 row=decision/critical-path-human-takes-wheel sha=139e8ce894b8 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=28 row=contract/cli-refusal/format area=1-a11y-scope-summary rec=fct-67rr3koq@1,fct-lpulrs7q@1
boundary: CLI refusals
item: refusal format
promise: A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
@row_check n=28 row=contract/cli-refusal/format sha=82f21045ae20 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
@row_add n=29 row=decision/critical-path-cli-board-output area=1-a11y-scope-summary rec=fct-vunrzfno@2,fct-3mbiblpp@1,fct-4md23dny@1
name: Critical path: CLI board, wait/last and verify output
rule: The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
@row_check n=29 row=decision/critical-path-cli-board-output sha=407a252c8c7a v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=30 row=decision/terminal-structural-equivalents area=1-a11y-scope-summary rec=fct-w4hvnbpo@1
name: Terminal structural equivalents of ARIA roles
rule: In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator and the --help group labels.
@row_check n=30 row=decision/terminal-structural-equivalents sha=c1b5f826fb79 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: src/human/board/rows.rs:80
quote: const UNWRAPPED_SEPARATOR: &str = "-- UNWRAPPED - READ-ONLY --";
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
@row_add n=31 row=contract/viola-list/unwrapped-separator area=1-a11y-scope-summary rec=fct-w4hvnbpo@1
boundary: viola list output
item: -- UNWRAPPED - READ-ONLY --
promise: The separator appears only when unwrapped rows exist.
@row_check n=31 row=contract/viola-list/unwrapped-separator sha=0e918435a1df v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:99-100
quote: if wrapped < board.items.len() { lines.push(UNWRAPPED_SEPARATOR.to_owned());
@row_add n=32 row=contract/viola-wait/waiting-line area=1-a11y-scope-summary rec=fct-pgo5ntgv@1
boundary: viola wait output
item: waiting: <name>
promise: The waiting line is written once on stderr, on a TTY only.
@row_check n=32 row=contract/viola-wait/waiting-line sha=6494ccfa831b v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:69
quote: if !args.json && io::stderr().is_terminal() {
@row_add n=33 row=contract/viola-verify/step-line area=1-a11y-scope-summary rec=fct-pgo5ntgv@1
boundary: viola verify output
item: step line
promise: Each verify step prints a line [NN/NN] … pass|fail.
@row_check n=33 row=contract/viola-verify/step-line sha=face53144e83 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
@row_add n=34 row=decision/output-discipline-stands-in-for-sc area=1-a11y-scope-summary rec=fct-lpulrs7q@1
name: Output discipline stands in for WCAG SC
rule: Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
why: WCAG does not apply directly to terminal output
@row_check n=34 row=decision/output-discipline-stands-in-for-sc sha=82180bd9569d v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=35 row=absence/internationalization area=1-a11y-scope-summary rec=fct-hmypp33q@1
capability: Internationalization: no i18n, RTL or lang conventions
@row_check n=35 row=absence/internationalization sha=67fb24e8900e v=holds judge=sonnet head=8834195b0683
searched: i18n|rtl|locale|gettext|lang= · in src/**/*.rs and crates/**/*.rs
@row_add n=36 row=absence/touch-surface area=1-a11y-scope-summary rec=fct-72c2us6l@1
capability: A touch surface
@row_check n=36 row=absence/touch-surface sha=35600d5b33ed v=holds judge=sonnet head=8834195b0683
searched: touchscreen|touch_target|pointer · in src/**/*.rs and crates/**/*.rs
@row_add n=37 row=decision/no-sgr-without-tty area=2-a11y-strategy rec=fct-fce6ttn6@1,fct-ijnl2u6y@4,fct-znmnfnxq@1
name: No SGR without a colour-capable terminal
rule: The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`.
guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
@row_check n=37 row=decision/no-sgr-without-tty sha=6b5b448dc1da v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/style.rs:62
quote: if facts.json
at: src/human/style.rs:63
quote: || facts.passthrough
at: src/human/style.rs:64
quote: || facts.no_color
at: src/human/style.rs:65
quote: || facts.term.as_deref() == Some("dumb")
at: src/human/style.rs:66
quote: || !facts.stdout_tty
guard_at: tests/cross_list_rows.rs:302
@row_add n=38 row=decision/word-beside-every-sgr area=2-a11y-strategy rec=fct-fce6ttn6@1,fct-ijnl2u6y@4,fct-znmnfnxq@1
name: Word beside every SGR span
rule: Every SGR span on the CLI sits beside its word, so each state is still carried by the word without the colour.
guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
@row_check n=38 row=decision/word-beside-every-sgr sha=f16675f736e2 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:77
quote: let span = |word: &str, token: &str| format!("{token}{word}{}{base}", tokens.reset);
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
guard_at: tests/cross_list_rows.rs:302
@row_add n=39 row=contract/cli-refusal/unable-line area=2-a11y-strategy rec=fct-zj2wxt2x@2,fct-6jy2xx4r@1
boundary: CLI refusal output
item: unable line
promise: A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
@row_check n=39 row=contract/cli-refusal/unable-line sha=335c80640cec v=stale judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
shows: The human refusal line carries the instance name first (`unable  <name>  <reason>  <detail>`), not `unable  <reason>  <detail>`; the `unable: <text>` form for revive, run socket-directory and verify interrupted does hold (src/human.rs:37, src/cmd/verify.rs:83).
cell: promise
value: A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
@row_add n=40 row=contract/cli-refusal/hint-line area=2-a11y-strategy rec=fct-zj2wxt2x@2,fct-6jy2xx4r@1
boundary: CLI refusal output
item: hint line
promise: A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
@row_check n=40 row=contract/cli-refusal/hint-line sha=e9489356d686 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=41 row=decision/no-aaa-cognitive-escalation area=2-a11y-strategy rec=fct-6py5v5ks@2,fct-lfozu4af@1
name: No AAA cognitive escalation
rule: Cognitive accessibility is not escalated to AAA; SC 2.2.6, 3.1.5 and 3.3.9 are not claimed, and only cognitive-relevant A/AA criteria with a v1 surface are covered.
why: no cognitive-accessibility trigger fired
@row_check n=41 row=decision/no-aaa-cognitive-escalation sha=2ca083c588f1 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=42 row=decision/cli-structural-landmark-equivalents area=4-aria-patterns-roles rec=fct-m7wf3gp4@1
name: CLI structural equivalents of landmarks
rule: The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels.
@row_check n=42 row=decision/cli-structural-landmark-equivalents sha=886e4040a598 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:82
quote: /// The board as ONE write: the BAY line, the caption row, the wrapped rows or the empty-board line,
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: src/human/board/rows.rs:80
quote: const UNWRAPPED_SEPARATOR: &str = "-- UNWRAPPED - READ-ONLY --";
at: src/cmd/mod.rs:34
quote: board:    list
@row_add n=43 row=decision/tui-boundary-only area=4-aria-patterns-roles rec=fct-m7wf3gp4@1,fct-4ktfiloi@3
name: Tui accessibility is boundary-only
rule: The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own.
@row_check n=43 row=decision/tui-boundary-only sha=12601f22e7da v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=44 row=decision/send-refused-while-human-typing area=4-aria-patterns-roles rec=fct-jkkyxmll@3
name: Send refused while the human types
rule: When the human takes the wheel, `send` is refused with `human-typing` on stderr and a typed exit code.
guard: std::process::Command asserts
@row_check n=44 row=decision/send-refused-while-human-typing sha=440f7cc9392e v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
guard_at: tests/cli_wheel.rs:97
@row_add n=45 row=decision/wheel-ignores-terminal-events area=4-aria-patterns-roles rec=fct-jkkyxmll@3
name: Focus, mouse and resize never move the wheel
rule: On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal.
guard: outer-PTY nextest cases on ubuntu-latest
@row_check n=45 row=decision/wheel-ignores-terminal-events sha=bbb9d7c1ff1b v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/wheel.rs:4
quote: //! Focus reports, mouse reports, terminal replies (the founder's closed list, F-W2) and host
guard_at: tests/tui_wheel.rs:226
@row_add n=46 row=decision/c0-c1-escaped area=4-aria-patterns-roles rec=fct-ijnl2u6y@4
name: C0 and C1 escaped in CLI output
rule: C0 and C1 control characters in CLI output are escaped.
guard: literal pins over std::process::Command on ubuntu-latest
@row_check n=46 row=decision/c0-c1-escaped sha=6b83254de608 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/escape.rs:23
quote: if c.is_control() {
at: src/human/escape.rs:8
quote: if c.is_control() && c != '\n' && c != '\t' {
guard_at: tests/cross_list_rows.rs:30
@row_add n=47 row=decision/linear-static-output area=4-aria-patterns-roles rec=fct-ijnl2u6y@4
name: Linear static CLI output
rule: CLI output is linear and static.
guard: literal pins over std::process::Command on ubuntu-latest
@row_check n=47 row=decision/linear-static-output sha=bfc9ed1ff700 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human.rs:59
quote: out.write_all(format!("{line}\n").as_bytes())
searched: indicatif|spinner · in src/**/*.rs (no match outside tests)
guard_at: tests/cli_output_plain.rs:56
@row_add n=48 row=decision/stale-row-dim area=4-aria-patterns-roles rec=fct-lchghj7u@3,fct-znmnfnxq@1
name: Dim only on stale rows
rule: Dim SGR is applied only to stale rows, beside the word that carries the state.
guard: unit-tier tests and outer-PTY tests on ubuntu-latest
@row_check n=48 row=decision/stale-row-dim sha=801a98181379 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:60
quote: stale: item.wrapped && item.liveness == RowLiveness::Stale,
guard_at: tests/cross_list_rows.rs:266
@row_add n=49 row=decision/cli-no-interactive-input area=5-keyboard-navigation rec=fct-2bakknao@1
name: No interactive CLI input
rule: The CLI has no interactive prompt, pager or redraw, and takes input only from stdin or `--file`.
@row_check n=49 row=decision/cli-no-interactive-input sha=e17ed3f4d9df v=holds judge=sonnet head=8834195b0683
searched: read_line( · in src/**/*.rs (none outside viola-fake-agent)
searched: rustyline|dialoguer|inquire|Pager|reedline|ratatui|crossterm · in src/**/*.rs and Cargo.toml (none)
searched: stdin().lock() · in src/cmd (send.rs:161 read_text, answer.rs:88 read_response, hook; text from stdin or --file at src/cmd/send.rs:48)
@row_add n=50 row=decision/tui-terminal-belongs-to-child area=5-keyboard-navigation rec=fct-pzkrh6fa@1
name: Tui terminal belongs to the child
rule: On the tui, the terminal belongs to the wrapped child process, not to viola.
@row_check n=50 row=decision/tui-terminal-belongs-to-child sha=0a6dbdcc380d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:647-648
quote: // Raw before the spawn: every key the human types from here on reaches the child as typed. let terminal = HostTerminal::enter();
@row_add n=51 row=decision/dialog-amber-sgr area=6-visual-design-verification rec=fct-znmnfnxq@1
name: Amber only on the DIALOG word
rule: Amber SGR is applied only to the `DIALOG` word.
@row_check n=51 row=decision/dialog-amber-sgr sha=fca5fe8da0bf v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=52 row=decision/name-bold-sgr area=6-visual-design-verification rec=fct-znmnfnxq@1
name: Bold only on NAME values
rule: Bold SGR is applied only to NAME values.
@row_check n=52 row=decision/name-bold-sgr sha=0bcafab58e25 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
@row_add n=53 row=decision/unable-fail-never-coloured area=6-visual-design-verification rec=fct-znmnfnxq@1
name: Refusal and failure words never coloured
rule: The words `unable` and `fail` are never coloured.
@row_check n=53 row=decision/unable-fail-never-coloured sha=51c840453e74 v=holds judge=sonnet head=8834195b0683
searched: tokens.attention|tokens.stale|tokens.callsign · in src/**/*.rs (only src/human/board/cells.rs:76-90, on NAME, the DIALOG word and stale rows)
searched: unable|fail with a colour token · in src/human/style.rs and src/human/board/cells.rs (none)
@row_add n=54 row=decision/name-truncation area=6-visual-design-verification rec=fct-znmnfnxq@1
name: NAME truncation only on a narrow terminal
rule: An unwrapped NAME is cut with `...` only on a terminal too narrow and never inside a `\xHH` escape; a wrapped NAME, piped output and `--json` are never cut.
@row_check n=54 row=decision/name-truncation sha=2c18b1124366 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:52-53
quote: let name = if !item.wrapped && name.chars().count() > column { cut(&name, column)
at: src/human/board/cells.rs:128-130
quote: if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) { keep = at; }
at: src/cmd/list.rs:63
quote: let width = viola_pty::host_size().map(|size| usize::from(size.cols));
@row_add n=55 row=decision/screen-reader-target-cli area=7-screen-reader-support rec=fct-2kjabyk2@2,fct-yzqlnwsx@4
name: Orca with GNOME Terminal for the cli
rule: Manual screen-reader checks of the cli use Orca with GNOME Terminal on Linux: linear output, refusals read as text, no escape noise.
@row_check n=55 row=decision/screen-reader-target-cli sha=0f5d1c2494e3 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=56 row=decision/screen-reader-target-tui area=7-screen-reader-support rec=fct-wawttggc@2,fct-4ktfiloi@3
name: Orca with foot on Hyprland for the tui
rule: Manual screen-reader checks of the tui use Orca with foot on Hyprland on Linux.
@row_check n=56 row=decision/screen-reader-target-tui sha=3cb5dace3360 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=57 row=absence/user-time-limit area=8-cognitive-accessibility rec=fct-do54jgjo@1,fct-nu6eg27k@1
capability: A time limit on the user, and any timeout-extension component
@row_check n=57 row=absence/user-time-limit sha=7b71d0c77f5e v=holds judge=sonnet head=8834195b0683
searched: session timeout|idle timeout|time limit · in src/
searched: wait.*timeout · in src/ (only the driver's `wait` timeout_ms and internal STEP waits; no user-facing limit or extension component)
@row_add n=58 row=decision/time-bounds-never-on-human area=8-cognitive-accessibility rec=fct-mkekg7f5@1
name: Time bounds never fall on the human
rule: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read, budget pause) falls on the driver, the user's command or viola's child; no key is blocked, refused or delayed by it.
@row_check n=58 row=decision/time-bounds-never-on-human sha=f0f12899448f v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:4-5
quote: //! panic does, until the host size changes. The human's passthrough never waits on either //! (security-plan §Input Validation, PTY output row; obs-plan §7).
@row_add n=59 row=contract/cli-refusal/exit-code area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: exit code
promise: Every refusal ends with a typed exit code.
@row_check n=59 row=contract/cli-refusal/exit-code sha=f3d9ef3d6365 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
at: src/cmd/send.rs:195
quote: return Ok(13);
@row_add n=60 row=contract/cli-refusal/unknown-refusal area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: the `unknown` refusal
promise: The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing.
@row_check n=60 row=contract/cli-refusal/unknown-refusal sha=44709b28794d v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
at: src/cmd/send.rs:85
quote: RefusalReason::UnverifiedCli | RefusalReason::Unknown => None,
@row_add n=61 row=contract/cli-refusal/fault area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: fault
promise: Faults are not refusals and carry no hint.
@row_check n=61 row=contract/cli-refusal/fault sha=88ad21398502 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
@row_add n=62 row=contract/cli-refusal/json area=8-cognitive-accessibility rec=fct-6jy2xx4r@1
boundary: CLI refusal output
item: --json
promise: Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document.
@row_check n=62 row=contract/cli-refusal/json sha=567e44c8b2b1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:133
quote: return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
at: src/cmd/send.rs:52
quote: const UNUSABLE_TEXT: &str = "error: the text must be readable UTF-8 of at most 16 MiB\n";
at: src/cmd/answer.rs:42-43
quote: const UNUSABLE_RESPONSE: &str = "error: the response must be one JSON object of an answer's shape, at most 16 MiB of UTF-8\n";
@row_add n=63 row=decision/refusal-layout-literal-pins area=8-cognitive-accessibility rec=fct-2jdilj7v@4
name: Refusal layout held by literal pins
rule: Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest.
guard: literal pins over std::process::Command on ubuntu-latest
@row_check n=63 row=decision/refusal-layout-literal-pins sha=14699702917e v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cli_wheel.rs:114
quote: format!("unable  builder  human-typing  manual-pause\n{HINT}\n")
guard_at: tests/cli_error_bodies.rs:72
@row_add n=64 row=decision/no-reading-level-gate area=8-cognitive-accessibility rec=fct-uoxd5hfd@1
name: No reading-level gate
rule: No reading-level target is gated and no readability scorer is introduced.
why: SC 3.1.5 Reading Level (AAA) is not claimed, and no readability scorer is in a11y-research
@row_check n=64 row=decision/no-reading-level-gate sha=711cf86a1d2d v=holds judge=sonnet head=8834195b0683
searched: readab|flesch|reading.level · in src/**, tests/**, scripts/**, crates/**, Cargo.toml (case-insensitive; only unrelated "readable only by you" hint text)
@row_add n=65 row=decision/reason-code-beside-unable area=8-cognitive-accessibility rec=fct-kgelfpj7@1
name: Reason codes always beside unable
rule: Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
@row_check n=65 row=decision/reason-code-beside-unable sha=a0594ba45506 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
@row_add n=66 row=decision/no-bare-error-text area=8-cognitive-accessibility rec=fct-kgelfpj7@1
name: No stack trace or bare status as error text
rule: The CLI never shows a stack trace or a bare status number as the only error text.
@row_check n=66 row=decision/no-bare-error-text sha=bafb50a781c6 v=holds judge=sonnet head=8834195b0683
searched: RUST_BACKTRACE|backtrace|set_hook · in src/ (panic hook writes payload and backtrace only to the instance detail file, never to stdout/stderr; CLI prints `error: internal error`)
searched: unable: \{ · in src/human.rs (errors print `unable: ...` plus `hint:` text)
@row_add n=67 row=decision/fixed-cli-strings-pinned area=8-cognitive-accessibility rec=fct-fhiyvzbh@2
name: Fixed CLI strings pinned by literal
rule: Every fixed CLI string, hints and result lines included, is pinned by literal.
why: so any copy change fails CI until the pinned string is updated on purpose
@row_check n=67 row=decision/fixed-cli-strings-pinned sha=74cba1a9863f v=holds judge=sonnet head=8834195b0683
at: tests/cli_list.rs:31
quote: const EMPTY: &str = "no wrapped sessions  start one: viola run <name> -- claude";
@row_add n=68 row=contract/viola-list-caption/order area=8-cognitive-accessibility rec=fct-xifq2q6n@1,fct-cyo3rpos@2
boundary: viola list caption
item: order
promise: The caption order is `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI`, pinned by the `CAPTION` literal.
@row_check n=68 row=contract/viola-list-caption/order sha=882e7b6b4a41 v=holds judge=sonnet head=8834195b0683
at: tests/cli_list.rs:27
quote: const CAPTION: &str = "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI";
@row_add n=69 row=decision/a11y-ci-github-actions area=9-ci-integration rec=fct-by4mf7d7@2
name: A11y checks in the GitHub Actions push/PR workflow
rule: A11y checks run in the one GitHub Actions push/PR workflow on an `ubuntu-latest` matrix; the scheduled nightly workflow carries no a11y step.
@row_check n=69 row=decision/a11y-ci-github-actions sha=1dc46e600cf8 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
searched: a11y|coverage|gate · in .github/workflows/nightly.yml (no a11y step)
@row_add n=70 row=decision/a11y-cases-inside-coverage-suite area=9-ci-integration rec=fct-s4vibfo3@4,fct-pkoabs22@2
name: A11y cases run inside the coverage suite
rule: The CLI output-discipline and TUI boundary cases run inside the test job's single instrumented `coverage` suite on the `ubuntu-latest` leg, and its `gate --require coverage,doctest` is the verdict.
why: a11y CI runs in the same pipeline as the tests, under the binding 5-command discipline of the test harness contract
@row_check n=70 row=decision/a11y-cases-inside-coverage-suite sha=074f75e3c8ea v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:38
quote: run: bash scripts/agent-run.sh run --coverage
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
@row_add n=71 row=decision/no-separate-a11y-driver area=9-ci-integration rec=fct-pkoabs22@2,fct-4sblkbrt@1
name: No separate a11y driver or suite value
rule: A11y adds no second driver outside the harness `run` command and no new `suite` value such as `a11y`.
@row_check n=71 row=decision/no-separate-a11y-driver sha=b818d8dff24d v=holds judge=sonnet head=8834195b0683
searched: a11y · in scripts/**, .github/**, crates/**, tests/**, schemas/** (only comments, and viola-e2e gate.rs:411 / tests/cli.rs:105 treating `a11y` as an unknown suite)
searched: "a11y" suite value · in crates/viola-e2e/src/harness/gate.rs (suite list has no a11y)
@row_add n=72 row=decision/no-own-a11y-artifact area=9-ci-integration rec=fct-vicafivy@1
name: No a11y artifact of its own
rule: A11y uploads no artifact of its own: a cli or tui failure is a nextest failure in the harness `run` JSON `suites[].failures[]`.
@row_check n=72 row=decision/no-own-a11y-artifact sha=7eabe649da4f v=holds judge=sonnet head=8834195b0683
searched: upload-artifact name a11y · in .github/workflows/ci.yml (uploads are diag, harness, junit, secret-scan, perf, supply-chain only)
@row_add n=73 row=decision/cli-invariants-pass-in-ci area=10-slo-invariants-a11y-budgets rec=fct-k6kozbbz@2
name: CLI output-discipline invariants pass in CI
rule: The CLI output-discipline invariants (P4, P6) pass on the `ubuntu-latest` CI leg inside the `coverage` suite.
@row_check n=73 row=decision/cli-invariants-pass-in-ci sha=d55f789c285b v=holds judge=sonnet head=8834195b0683
at: tests/cli_verify.rs:105
quote: /// No ESC, no CR, ASCII only (design-system §Surface: cli; a11y-plan §4 P6).
at: .github/workflows/ci.yml:38
quote: run: bash scripts/agent-run.sh run --coverage
@row_add n=74 row=decision/mutation-audit-covers-crate-invariants area=10-slo-invariants-a11y-budgets rec=fct-k6kozbbz@2
name: Mutation audit covers invariants held in crate tests
rule: Where the CLI output-discipline invariants (P4, P6) live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them.
@row_check n=74 row=decision/mutation-audit-covers-crate-invariants sha=246db72f1336 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:307
quote: if sel.mutants && refusal.is_none() {
@row_add n=75 row=decision/no-a11y-performance-budget area=10-slo-invariants-a11y-budgets rec=fct-g5vn64oj@1,fct-2575wca3@1
name: No performance budget per a11y CI run
rule: A11y CI has no performance budget of its own: the cli and tui cases add no stage, running inside the `coverage` suite.
@row_check n=75 row=decision/no-a11y-performance-budget sha=86885e93bb32 v=holds judge=sonnet head=8834195b0683
searched: a11y|axe|accessib (case-insensitive) · in .github/workflows/ci.yml
@row_add n=76 row=decision/zero-flakiness area=10-slo-invariants-a11y-budgets rec=fct-zejx5pqs@1,fct-jecbfjie@1
name: Zero flakiness, no retries
rule: Tests and a11y checks run with `retries: 0` and `#[ignore]` banned; a flake keeps the work red until its root cause is fixed.
why: retry-once policies mask real signals (the agent-driven principle)
@row_check n=76 row=decision/zero-flakiness sha=c73619a6e551 v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:2
quote: retries = 0
searched: #\[ignore · in tests/**/*.rs, crates/**/*.rs, src/**/*.rs
@row_add n=77 row=decision/a11y-gate-breach-fails-build area=10-slo-invariants-a11y-budgets rec=fct-tyva3sbt@1,fct-lzk7pnbc@1
name: Gate breach fails the build
rule: A `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build and deploy.
@row_check n=77 row=decision/a11y-gate-breach-fails-build sha=67d8e8ad8257 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:79
quote: breaches.push(breach("suite-missing", suite, "no-run-summary".to_owned()));
at: crates/viola-e2e/src/harness/gate.rs:89
quote: for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] {
at: crates/viola-e2e/src/harness/gate.rs:103-105
quote: breaches.push(breach( "artifact-missing", "coverage",
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
@row_add n=78 row=decision/non-editing-sequences area=11-a11y-anti-patterns-never-do-these rec=fct-csjr2vg7@2,fct-itb2wxsm@2
name: Terminal sequences are not human editing
rule: Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4).
@row_check n=78 row=decision/non-editing-sequences sha=dae9cfdb43fa v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:491
quote: fn is_reply(params: &[u8], fin: u8) -> bool {
at: src/run/wheel.rs:514
quote: (None, [], b'I' | b'O') => fields == 0,
at: src/run/wheel.rs:405
quote: true
@row_add n=79 row=decision/no-information-by-colour-alone area=11-a11y-anti-patterns-never-do-these rec=fct-4w44zoy4@1,fct-vtox22ts@4
name: No information by colour alone
rule: Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).
@row_check n=79 row=decision/no-information-by-colour-alone sha=1fdad6cb2558 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=80 row=decision/no-colour-on-failure-words area=11-a11y-anti-patterns-never-do-these rec=fct-633dmm7e@1
name: Failure words are never coloured
rule: The CLI words `unable` and `fail` are never coloured.
@row_check n=80 row=decision/no-colour-on-failure-words sha=be7528d84291 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
@row_add n=81 row=decision/no-unwarned-timeouts area=11-a11y-anti-patterns-never-do-these rec=fct-ickgtugn@1
name: No harsh timeouts without warning
rule: No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
@row_check n=81 row=decision/no-unwarned-timeouts sha=1a23aa417129 v=unverifiable judge=sonnet head=8834195b0683
searched: warn-extend · in src/ and crates/
searched: Duration::from_secs · in src/
why: no evidence found
@row_add n=82 row=decision/no-stack-trace-only-error area=11-a11y-anti-patterns-never-do-these rec=fct-retrfvu7@1
name: No stack trace as the only error text
rule: The CLI never renders a stack trace as the only error text.
@row_check n=82 row=decision/no-stack-trace-only-error sha=74672d1b3726 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=83 row=decision/list-captions-fixed area=11-a11y-anti-patterns-never-do-these rec=fct-vt5poetp@1
name: Board captions fixed by their literal
rule: The caption set and order of `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) never drift from its `CAPTION` literal (SC 3.2.4).
@row_check n=83 row=decision/list-captions-fixed sha=7df0904631d3 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:116-117
quote: const CAPTION: &str = "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI";
@row_add n=84 row=decision/no-spinners area=11-a11y-anti-patterns-never-do-these rec=fct-dqbono2s@1,fct-vtox22ts@4
name: No spinners
rule: CLI output is static and linear, with no spinner or redraw.
why: design ban
@row_check n=84 row=decision/no-spinners sha=3566836c13d9 v=holds judge=sonnet head=8834195b0683
searched: spinner · in src/ (rs)
searched: indicatif · in src/ and Cargo.toml
searched: progress · in Cargo.toml
@row_add n=85 row=decision/a11y-fail-fast-gate area=11-a11y-anti-patterns-never-do-these rec=fct-icuat5rz@1
name: A11y is a fail-fast gate
rule: A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning.
@row_check n=85 row=decision/a11y-fail-fast-gate sha=1afb8e19e197 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=86 row=decision/machine-output-a11y-review area=11-a11y-anti-patterns-never-do-these rec=fct-ayxkclws@1
name: No human-only a11y review
rule: No a11y review is human-only without machine output.
why: the canonical agent-runnable principle
@row_check n=86 row=decision/machine-output-a11y-review sha=716512c0ce1e v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=87 row=decision/no-soft-a11y-slo area=11-a11y-anti-patterns-never-do-these rec=fct-o743h7h2@1
name: No soft a11y SLO
rule: An a11y SLO is never soft: it triggers a build or deploy failure on its threshold.
@row_check n=87 row=decision/no-soft-a11y-slo sha=01f1171298d3 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=88 row=decision/no-vendor-locked-a11y-tool area=11-a11y-anti-patterns-never-do-these rec=fct-kdsodzwc@1
name: No vendor-locked a11y tool as only method
rule: A vendor-locked a11y tool without machine output or API (Stark Visual Studio plugin, Tenable.io UI-only) is never the ONLY method.
@row_check n=88 row=decision/no-vendor-locked-a11y-tool sha=6a24ffe65f0f v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=89 row=contract/revive-json-refusal/document area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: exit-1 refusal document
promise: Each exit-1 refusal of the strict-modes check, the preflight or the start shared with `run` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr.
@row_check n=89 row=contract/revive-json-refusal/document sha=69147dedcb3e v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/revive.rs:107
quote: client::refused_document(self.detail());
at: src/cmd/run.rs:420
quote: super::client::refused_document(detail);
@row_add n=90 row=contract/revive-json-refusal/strict-modes-failed area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: strict-modes-failed
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_check n=90 row=contract/revive-json-refusal/strict-modes-failed sha=e24f74a7065c v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=91 row=contract/revive-json-refusal/already-live area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: already-live
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_check n=91 row=contract/revive-json-refusal/already-live sha=d058f22f8840 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=92 row=contract/revive-json-refusal/no-session area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: no-session
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; `viola revive <name> --list --json` over an empty chain prints this document, exit 1.
@row_check n=92 row=contract/revive-json-refusal/no-session sha=1a4a00ed4ba5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
at: src/cmd/revive.rs:283
quote: Refusal::NoSession.write(name, json);
@row_add n=93 row=contract/revive-json-refusal/session-live area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: session-live
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_check n=93 row=contract/revive-json-refusal/session-live sha=908b97f8789c v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=94 row=contract/revive-json-refusal/cwd-missing area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: cwd-missing
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_check n=94 row=contract/revive-json-refusal/cwd-missing sha=76a47c58b8d7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=95 row=contract/revive-json-refusal/stale-heartbeat area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: stale-heartbeat
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries.
@row_check n=95 row=contract/revive-json-refusal/stale-heartbeat sha=500a3cf2bf57 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=96 row=contract/revive-json-refusal/pinned-hash-mismatch area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: pinned-hash-mismatch
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns.
@row_check n=96 row=contract/revive-json-refusal/pinned-hash-mismatch sha=dc8f3f48658f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=97 row=contract/revive-json-refusal/squatted-name area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: squatted-name
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; printed in place of the human pair, before the child spawns.
@row_check n=97 row=contract/revive-json-refusal/squatted-name sha=208d32962171 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=98 row=contract/revive-json-refusal/internal-error area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --json refusals
item: internal-error
promise: One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; a `claude` not found at a start reads as this document and has no human pair.
@row_check n=98 row=contract/revive-json-refusal/internal-error sha=f5cae9d6febb v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:270
quote: Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
@row_add n=99 row=contract/revive-list/json area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --list
item: --json output
promise: `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order.
@row_check n=99 row=contract/revive-list/json sha=64a7604645b3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
@row_add n=100 row=contract/revive-list/human-rows area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive --list
item: human rows
promise: Without `--json`, `viola revive <name> --list` prints static ASCII rows `<ts>  <cause>  <id>`, with no header, no colour and no ESC byte.
@row_check n=100 row=contract/revive-list/human-rows sha=59f46e01ca0a v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:313
quote: format!("{}  {}  {}", self.ts, self.cause, self.id)
@row_add n=101 row=contract/revive-human-refusal/pair area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola revive human refusals
item: unable/hint pair
promise: Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical.
@row_check n=101 row=contract/revive-human-refusal/pair sha=4efa64cbe885 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:110
quote: human::refuse(&unable, &hint);
@row_add n=102 row=contract/cli-usage-error/exit-2 area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: CLI exit codes
item: 2
promise: A clap usage error exits 2; `viola plugin` alone is such an error.
@row_check n=102 row=contract/cli-usage-error/exit-2 sha=03b8b8fa2535 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
at: src/cmd/plugin.rs:27
quote: verb: PluginVerb,
@row_add n=103 row=contract/interrupted-verify/stderr area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola verify interrupted
item: stderr lines
promise: A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, plain ASCII.
@row_check n=103 row=contract/interrupted-verify/stderr sha=4add94b61920 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:83-86
quote: human::refuse( "verify was stopped before it finished", "run viola verify again to stamp this CLI", );
@row_add n=104 row=contract/interrupted-verify/exit area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola verify interrupted
item: exit 1
promise: A `verify` stopped by Ctrl-C or TERM before its rows are checked exits 1.
@row_check n=104 row=contract/interrupted-verify/exit sha=313c6ad8579b v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
@row_add n=105 row=contract/plugin-install/human-output area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola plugin install
item: human output
promise: `viola plugin install` prints its four planning-form lines as one plain-ASCII write, no SGR, and no ESC or non-ASCII byte piped, under `NO_COLOR` or under `TERM=dumb`.
@row_check n=105 row=contract/plugin-install/human-output sha=5cfe9b1d3f0c v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:93-94
quote: pub(crate) fn write_plugin_installed(out: &mut impl Write, dir: &str) -> io::Result<()> { out.write_all(
@row_add n=106 row=contract/plugin-install/json-output area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola plugin install
item: --json output
promise: Under `--json`, `viola plugin install` prints exactly `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`.
@row_check n=106 row=contract/plugin-install/json-output sha=ffa8b6e1b699 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:67-70
quote: client::document(&json!({ "v": 1, "ok": {"marketplace": marketplace::NAME, "plugin": PLUGIN_REF}, }));
@row_add n=107 row=contract/plugin-install/exit-21 area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: viola plugin install
item: exit 21
promise: The exit-21 refusal of `viola plugin install` is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
@row_check n=107 row=contract/plugin-install/exit-21 sha=29f48be0f4c5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:55-60
quote: client::unreachable_document(UnreachableCause::StrictModes); } else { let (unable, hint) = human::home_state_pair(); human::refuse(unable, hint); } return Ok(ExitCode::from(21));
@row_add n=108 row=contract/frame-verb-refusal/server-verify-failed area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: frame verb refusals
item: server-verify-failed
promise: send, wait, last, answer, pause, release, link and unlink refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty.
@row_check n=108 row=contract/frame-verb-refusal/server-verify-failed sha=c5702356960a v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:194
quote: Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
at: src/cmd/pause.rs:77
quote: Err(cause) => return Ok(unreachable(cause)),
at: src/human/hints.rs:56-58
quote: "server-verify" => format!( "the process on {name}'s endpoint is not its recorded wrapper; it was not contacted" ),
@row_add n=109 row=contract/cli-json-failure/document area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: CLI --json failures
item: failure document
promise: Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
@row_check n=109 row=contract/cli-json-failure/document sha=b99337faea28 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:221
quote: let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: tests/cli_error_bodies.rs:104
quote: assert!(!ran.stdout.contains('\u{1b}'), "{label}: an ESC byte");
@row_add n=110 row=contract/cli-json-failure/send-answer-exit-2 area=a11y-scope-summary rec=fct-k76k2lf7@2
boundary: CLI --json failures
item: send and answer exit 2
promise: The `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document.
@row_check n=110 row=contract/cli-json-failure/send-answer-exit-2 sha=3798d55bcbfc v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:164
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
@row_add n=111 row=contract/link-unlink/result-line area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: result line
promise: `viola link` and `viola unlink` each print one plain-ASCII result line in one write, with no SGR in any mode.
@row_check n=111 row=contract/link-unlink/result-line sha=054a2bc8e77d v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/verbs.rs:79
quote: out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
@row_add n=112 row=contract/link-unlink/json-ok area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: --json output
promise: Under `--json`, `viola link` and `viola unlink` each print one `{"v":1,"ok":<ok>}` document.
@row_check n=112 row=contract/link-unlink/json-ok sha=cc8b46ab9f8a v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:121
quote: client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/unlink.rs:15
quote: let code = link_call(home, args, Verb::Unlink, request)?;
@row_add n=113 row=contract/link-unlink/exit-21 area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: exit 21
promise: `viola link` and `viola unlink` refuse with exit 21 a driven instance that fails the strict-modes check (before any log opens), a stopped driven instance, and a server that is not the recorded wrapper.
@row_check n=113 row=contract/link-unlink/exit-21 sha=27f37bc439f2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/link.rs:105
quote: return Ok(unreachable(Unreachable::without_endpoint(&instance_dir)));
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:265
quote: 21
@row_add n=114 row=contract/link-unlink/exit-20 area=a11y-scope-summary rec=fct-7hhxeiso@1
boundary: viola link and viola unlink
item: exit 20
promise: A pair the wrapper refuses as invalid params reaches `viola link` or `viola unlink` as a fault: exit 20, no hint.
@row_check n=114 row=contract/link-unlink/exit-20 sha=7f3bf6b62cba v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:116
quote: Err(detail) => return Ok(client::fault(args.json, &detail)),
at: src/cmd/client.rs:245
quote: 20
@row_add n=115 row=contract/frame-verb-refusal/strict-modes-failed area=a11y-scope-summary rec=fct-dssvg3hs@2
boundary: frame verb refusals
item: strict-modes-failed
promise: send, wait, last, answer, pause and release refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.
@row_check n=115 row=contract/frame-verb-refusal/strict-modes-failed sha=5ae0ae3e3563 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/release.rs:35
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
@row_add n=116 row=contract/frame-verb-refusal/socket-dir-refused area=a11y-scope-summary rec=fct-sjhco52d@2
boundary: frame verb refusals
item: socket-dir-refused
promise: Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found.
@row_check n=116 row=contract/frame-verb-refusal/socket-dir-refused sha=480714730957 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: src/human/hints.rs:59
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \
@row_add n=117 row=contract/start-refusal/home-strict-modes area=a11y-scope-summary rec=fct-hj6bq3jt@2
boundary: viola run, revive start and verify refusals
item: home strict-modes failure
promise: Over a home tree failing the strict-modes check, `viola run`, a `viola revive` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
@row_check n=117 row=contract/start-refusal/home-strict-modes sha=2cc03e161fed v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:172-173
quote: refuse_tree(&args.name, refused); return Ok(ExitCode::from(1));
at: src/cmd/revive.rs:148-149
quote: refuse_tree(&args.name, refused); return Ok(Started::Refused(ExitCode::from(1)));
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
@row_add n=118 row=contract/start-refusal/socket-dir area=a11y-scope-summary rec=fct-rts3hgep@2
boundary: viola run, revive start and verify refusals
item: socket directory failure
promise: Over a failing socket directory, `viola run` and a `viola revive` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.
@row_check n=118 row=contract/start-refusal/socket-dir sha=506b73d28ba1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:497
quote: if !socket_dir_ready(&endpoint) {
at: src/cmd/run.rs:300-302
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || { refuse_socket_dir(name); }));
at: src/cmd/run.rs:738
quote: &format!("the socket directory for {name} cannot be trusted"),
@row_add n=119 row=contract/tui-passthrough/own-bytes area=a11y-scope-summary rec=fct-itb2wxsm@2,fct-twqx5x46@3
boundary: TUI passthrough
item: terminal bytes
promise: viola writes zero bytes to the terminal except the child's output; a passed `viola revive` writes no line of its own while the child holds the terminal.
@row_check n=119 row=contract/tui-passthrough/own-bytes sha=124542f695b1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=120 row=contract/tui-passthrough/pump-child-error area=a11y-scope-summary rec=fct-itb2wxsm@2
boundary: TUI passthrough
item: pump_child error under revive --json
promise: Under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.
@row_check n=120 row=contract/tui-passthrough/pump-child-error sha=c0bb0eb87bbc v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:122-128
quote: let started = started(home, args).inspect_err(|_| { if json { client::refused_document("internal-error"); } })?; match started { Started::Launched(launched) => pump_child(*launched),
@row_add n=121 row=contract/tui-passthrough/refused-peer area=a11y-scope-summary rec=fct-itb2wxsm@2
boundary: TUI passthrough
item: refused peer
promise: A peer the wrapper refuses is dropped before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel.
@row_check n=121 row=contract/tui-passthrough/refused-peer sha=f04f218e8ace v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
at: crates/viola-channel/src/server.rs:150
quote: .spawn(move || serve_conn(&stream, &format!("srv-{n}"), dispatch.as_ref()));
@row_add n=122 row=contract/tui-passthrough/after-feed-panic area=a11y-scope-summary rec=fct-yscgzykm@2
boundary: TUI passthrough
item: after a contained vt100 feed panic
promise: The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do.
@row_check n=122 row=contract/tui-passthrough/after-feed-panic sha=baacd31f20bc v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46-49
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() { self.screen.poison(); self.panicked = true; parse_rejected("panicked");
at: src/run/send.rs:126
quote: return ctx.not_delivered(None, NotDelivered::InputNotReady);
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
@row_add n=123 row=contract/hook-call/server-verify-failed area=a11y-scope-summary rec=fct-walt4vws@2
boundary: viola hook event and dialog calls
item: server not the recorded wrapper
promise: A hook event or dialog call that meets a server that is not the recorded wrapper sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s.
@row_check n=123 row=contract/hook-call/server-verify-failed sha=9209ef415615 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:236-237
quote: if !peer_admitted(client.server_euid()) { return Err(NotSent::ServerUnverified);
at: src/cmd/hook.rs:56
quote: const CONNECT_DEADLINE: Duration = Duration::from_millis(750);
@row_add n=124 row=contract/mcp-frame-tool/server-verify-failed area=a11y-scope-summary rec=fct-walt4vws@2
boundary: MCP server frame tools
item: server-verify-failed
promise: An MCP frame tool (send, wait, last, answer) meeting a server that is not the recorded wrapper returns `isError` with code `instance-unreachable`, `detail` `server-verify-failed` (text `refused: instance-unreachable`) and sends no frame.
@row_check n=124 row=contract/mcp-frame-tool/server-verify-failed sha=9385ff879baf v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? { return Ok(result::unreachable(UnreachableCause::ServerVerify));
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=125 row=contract/hook-call/strict-modes-failed area=a11y-scope-summary rec=fct-6sgmn3kz@1
boundary: viola hook event and dialog calls
item: strict-modes failure
promise: A hook call refused by the strict-modes check returns exit 0 at once, empty stdout and stderr, under 1.0 s, with no `config.json` read, no stdin read, no frame, no line and no SessionEnd append; a `VIOLA_DIR` naming no instance directory writes nothing.
@row_check n=125 row=contract/hook-call/strict-modes-failed sha=4094afb2f7a3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=126 row=contract/mcp-frame-tool/strict-modes-failed area=a11y-scope-summary rec=fct-6sgmn3kz@1
boundary: MCP server frame tools
item: strict-modes-failed
promise: An MCP frame tool against an instance failing the strict-modes check returns `isError` with code `instance-unreachable`, `detail` `strict-modes-failed`, sends no frame and writes no line.
@row_check n=126 row=contract/mcp-frame-tool/strict-modes-failed sha=5a10890a4423 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=127 row=contract/hook-call/socket-dir-refused area=a11y-scope-summary rec=fct-w46xvuhu@1
boundary: viola hook event and dialog calls
item: socket directory failure
promise: A hook call against a failing socket directory sends no frame and fails open: exit 0, empty stdout and stderr, its one record a `hook-decision` at WARN with `detail` `socket-dir-refused`.
@row_check n=127 row=contract/hook-call/socket-dir-refused sha=94126858ce60 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:231-232
quote: if check_endpoint_dir(endpoint).is_err() { return Err(NotSent::SocketDir);
at: src/cmd/hook.rs:590-597
quote: Some(detail) => obs_event!( WARN, ObsEvent::HookDecision, hook_event = event.as_str(), decision_emitted = false, duration_ms = duration_ms, detail = detail, ),
@row_add n=128 row=contract/mcp-frame-tool/socket-dir-refused area=a11y-scope-summary rec=fct-yupqihr2@1
boundary: MCP server frame tools
item: socket-dir-refused
promise: An MCP frame tool against a failing socket directory returns `isError` with code `instance-unreachable`, `detail` `socket-dir-refused`, sends no frame and writes no line.
@row_check n=128 row=contract/mcp-frame-tool/socket-dir-refused sha=55ffdebea4b4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=129 row=decision/output-discipline-a11y-fallback area=a11y-scope-summary rec=fct-vtox22ts@4
name: Output-discipline assertions stand in for a11y tools
rule: CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON.
why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
@row_check n=129 row=decision/output-discipline-a11y-fallback sha=55557499e1bc v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:217
quote: portable-pty = "=0.9.0"
at: tests/support/sgr.rs:36
quote: pub fn sgr_attributes(stream: &[u8]) -> Vec<u32> {
at: crates/viola-e2e/src/harness/run.rs:132
quote: let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "live-in-ci"});
@row_add n=130 row=decision/manual-a11y-pass-gates-nothing area=a11y-scope-summary rec=fct-vtox22ts@4,fct-twqx5x46@3
name: Manual screen-reader pass gates nothing
rule: The manual Orca screen-reader pass (GNOME Terminal for the cli, foot on Hyprland for the tui) is founder-owned and supplemental; nothing is gated on it.
@row_check n=130 row=decision/manual-a11y-pass-gates-nothing sha=0c4baf5e19aa v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=131 row=contract/viola-list-board/order area=a11y-scope-summary rec=fct-vtox22ts@4,fct-p4tq3d4m@1,fct-ai3fe6uo@1
boundary: viola list board
item: structure
promise: The board is the BAY context line, the uppercase caption row and the wrapped rows, then the unwrapped read-only rows behind their separator.
@row_check n=131 row=contract/viola-list-board/order sha=bd47068dfe41 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:94-102
quote: let mut lines = vec![bay_line(budget, &board.skipped, now)]; lines.extend(rows.by_ref().take(1 + wrapped)); if wrapped == 0 { lines.push(EMPTY_BOARD.to_owned()); } if wrapped < board.items.len() { lines.push(UNWRAPPED_SEPARATOR.to_owned()); lines.extend(rows);
@row_add n=132 row=contract/viola-list-board/empty-board area=a11y-scope-summary rec=fct-vtox22ts@4
boundary: viola list board
item: no wrapped sessions
promise: With no wrapped row, the board prints the BAY line, the caption row and `no wrapped sessions  start one: viola run <name> -- claude`.
@row_check n=132 row=contract/viola-list-board/empty-board sha=33f1b0a86e63 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:78
quote: const EMPTY_BOARD: &str = "no wrapped sessions  start one: viola run <name> -- claude";
at: src/human/board/rows.rs:96-98
quote: if wrapped == 0 { lines.push(EMPTY_BOARD.to_owned()); }
@row_add n=133 row=contract/viola-list-board/unwrapped-separator area=a11y-scope-summary rec=fct-vtox22ts@4,fct-p4tq3d4m@1,fct-ai3fe6uo@1
boundary: viola list board
item: -- UNWRAPPED - READ-ONLY --
promise: The separator and unwrapped rows print only when the one `claude agents --json` read lists a session no wrapped row joins; unwrapped rows show the escaped `sessionId` as NAME and `n/a` in WHEEL, DIALOG and CLI; a failed, oversize or malformed read adds no separator and no unwrapped row.
@row_check n=133 row=contract/viola-list-board/unwrapped-separator sha=89cd6cef519d v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:99-101
quote: if wrapped < board.items.len() { lines.push(UNWRAPPED_SEPARATOR.to_owned()); lines.extend(rows);
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
at: src/human/board/rows.rs:49-50
quote: let na = || NOT_APPLICABLE.to_owned(); (na(), na(), na())
at: src/cmd/list.rs:126
quote: board::join(&mut board, observations(list));
@row_add n=134 row=contract/viola-list-board/colour area=a11y-scope-summary rec=fct-vtox22ts@4
boundary: viola list board
item: colour
promise: On a terminal `viola list` writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`.
@row_check n=134 row=contract/viola-list-board/colour sha=1ce6d5bab481 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62-68
quote: if facts.json || facts.passthrough || facts.no_color || facts.term.as_deref() == Some("dumb") || !facts.stdout_tty { return None;
at: src/human/board/rows.rs:58
quote: name: true,
at: src/human/board/rows.rs:59
quote: dialog: item.wrapped && item.dialog_pending,
at: src/human/board/rows.rs:60
quote: stale: item.wrapped && item.liveness == RowLiveness::Stale,
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
@row_add n=135 row=contract/cli-help/group-labels area=a11y-scope-summary rec=fct-vtox22ts@4,fct-p4tq3d4m@1,fct-ai3fe6uo@1
boundary: viola --help
item: group labels
promise: The `--help` verb table is grouped under the labels board / traffic / wheel / handoff / setup.
@row_check n=135 row=contract/cli-help/group-labels sha=740f83c6422a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
@row_add n=136 row=contract/log-identity/service-name area=a11y-scope-summary rec=fct-vtox22ts@4,fct-twqx5x46@3
boundary: log identity tags
item: service_name
promise: Records carry `service_name:"viola"` and `version`.
@row_check n=136 row=contract/log-identity/service-name sha=fa3b53e8fc98 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:33-34
quote: service_name = SERVICE_NAME, version = VERSION,
@row_add n=137 row=contract/log-identity/process area=a11y-scope-summary rec=fct-vtox22ts@4,fct-twqx5x46@3
boundary: log identity tags
item: process
promise: `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run`.
@row_check n=137 row=contract/log-identity/process sha=6364835da8bb v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:47
quote: obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
at: src/cmd/run.rs:225
quote: obs::viola_obs_init(home, ObsProcess::Run, Some(name.clone()), level)?;
@row_add n=138 row=decision/control-character-escaping area=a11y-scope-summary rec=fct-vtox22ts@4
name: C0/C1 control characters escaped
rule: CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two.
why: the list-row exception is ratified (T5)
except: in `viola list` table rows `\n` and `\t` are escaped too
@row_check n=138 row=decision/control-character-escaping sha=245ed5a7863b v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:8
quote: if c.is_control() && c != '\n' && c != '\t' {
at: src/human/escape.rs:23
quote: if c.is_control() {
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
at: src/human/wait.rs:5
quote: use super::escape::escape_message;
@row_add n=139 row=decision/run-diagnostics-off-terminal area=a11y-scope-summary rec=fct-twqx5x46@3
name: Run diagnostics never reach the terminal
rule: viola's diagnostics for `run` go to `diagnostics/`, never to the terminal.
@row_check n=139 row=decision/run-diagnostics-off-terminal sha=3906a829dac4 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:92
quote: Ok(Arc::new(open_private_append(&path)?))
at: Cargo.toml:257
quote: print_stderr = "deny"
searched: println!|eprintln! · in src/run/**/*.rs, src/obs.rs, src/cmd/run.rs
@row_add n=140 row=decision/linux-only area=a11y-scope-summary rec=fct-6wf4lqpi@1
name: Linux only
rule: Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
@row_check n=140 row=decision/linux-only sha=b83693842034 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: crates/viola-e2e/src/harness/pre_push.rs:21
quote: pub const PRE_PUSH_HOST_SUPPORTED: bool = cfg!(target_os = "linux");
@row_add n=141 row=contract/cli-help/usage-line area=aria-patterns-roles rec=fct-ai3fe6uo@1
boundary: viola --help
item: usage line
promise: `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the group rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, then `Options:`, plain ASCII.
@row_check n=141 row=contract/cli-help/usage-line sha=44ad7b504304 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:48
quote: override_usage = "viola [--home <dir>] <verb> [args]",
at: src/cmd/mod.rs:40
quote: Options:
@row_add n=142 row=contract/cli-help/served-verbs-only area=aria-patterns-roles rec=fct-ai3fe6uo@1
boundary: viola --help
item: served verbs only
promise: The help group rows list the served verbs only: `plugin install` is one entry in `setup:` and a bare `plugin` is unserved.
@row_check n=142 row=contract/cli-help/served-verbs-only sha=0d5ca3f047d5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:38
quote: setup:    run, revive, verify, plugin install
at: src/cmd/plugin.rs:33
quote: Install(InstallArgs),
@row_add n=143 row=contract/cli-help/handoff-row area=aria-patterns-roles rec=fct-ai3fe6uo@1,fct-p4tq3d4m@1
boundary: viola --help
item: handoff row
promise: The help `handoff:` row is `handoff:  link, unlink`, between `wheel:` and `setup:`.
@row_check n=143 row=contract/cli-help/handoff-row sha=e335ccac8c9a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:36-38
quote: wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
@row_add n=144 row=contract/cli-exit-codes/10 area=aria-patterns-roles rec=fct-dx4kie52@3,fct-6edxk4iz@3,fct-avr5o2ub@3
boundary: CLI exit codes
item: 10
promise: human-typing: the human holds the wheel, so the automated send is refused with nothing typed.
@row_check n=144 row=contract/cli-exit-codes/10 sha=ce50430078ca v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=145 row=decision/linear-static-cli-output area=aria-patterns-roles rec=fct-uok334yy@6
name: Linear static CLI output
rule: CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
@row_check n=145 row=decision/linear-static-cli-output sha=c40cf3431e48 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:69-70
quote: if !args.json && io::stderr().is_terminal() { let _ = human::write_waiting(&mut io::stderr().lock(), name);
at: src/human/wait.rs:9
quote: out.write_all(format!("waiting: {name}\n").as_bytes())
searched: spinner|indicatif|crossterm · in src/human/**/*.rs and src/cmd/**/*.rs
@row_add n=146 row=contract/viola-list-board/caption area=aria-patterns-roles rec=fct-uok334yy@6,fct-urfjsgc5@1
boundary: viola list board
item: caption order
promise: The caption order is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, reused by the terminal fleet console.
@row_check n=146 row=contract/viola-list-board/caption sha=3c40f9f1f312 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: tests/cli_list.rs:27
quote: const CAPTION: &str = "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI";
@row_add n=147 row=decision/no-sgr-off-terminal area=aria-patterns-roles rec=fct-uok334yy@6,fct-es7le4as@4
name: No SGR off a terminal
rule: viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive.
guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
@row_check n=147 row=decision/no-sgr-off-terminal sha=dae94962be83 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/style.rs:62-68
quote: if facts.json || facts.passthrough || facts.no_color || facts.term.as_deref() == Some("dumb") || !facts.stdout_tty { return None;
at: src/cmd/list.rs:64
quote: let colour = human::read_colour(false, false);
guard_at: tests/cross_list_rows.rs:302
@row_add n=148 row=decision/sgr-beside-its-word area=aria-patterns-roles rec=fct-uok334yy@6,fct-es7le4as@4
name: SGR only beside its word
rule: Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured.
guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal
@row_check n=148 row=decision/sgr-beside-its-word sha=084f0826a428 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
guard_at: tests/cross_list_rows.rs:232
@row_add n=149 row=decision/control-bytes-escaped area=aria-patterns-roles rec=fct-uok334yy@6
name: Control bytes printed escaped
rule: Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line.
why: security control for C0/C1 escaping, ratified for list rows
guard: escape_row_shows_every_control_character_as_hex_text
@row_check n=149 row=decision/control-bytes-escaped sha=e4f59b8ab70c v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/escape.rs:8
quote: if c.is_control() && c != '\n' && c != '\t' {
at: src/human/escape.rs:23
quote: if c.is_control() {
guard_at: src/human/escape.rs:61
@row_add n=150 row=decision/time-bounds-fall-on-the-driver area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Time bounds fall on the driver only
rule: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, budget pause, statusline deadline, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
@row_check n=150 row=decision/time-bounds-fall-on-the-driver sha=2696f04a4615 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:46
quote: given.map_or(WAIT_CAP, |given| given.min(WAIT_CAP))
at: src/run/dialog.rs:340
quote: let deadline = self.clock.now() + self.hold;
at: crates/viola-agent-claude/src/screen.rs:16
quote: pub const GATE_MAX_WAIT: Duration = Duration::from_millis(8500);
at: crates/viola-agent-claude/src/screen.rs:20
quote: pub const CONFIRM_WINDOW_FALLBACK: Duration = Duration::from_secs(10);
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
at: src/run/agents.rs:53
quote: VERSION_DEADLINE,
at: src/run/send.rs:108
quote: return ctx.budget_paused(&crossed);
@row_add n=151 row=contract/viola-wait/timed-out area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola wait
item: timed_out
promise: viola wait returns timed_out true only for a timeout the caller passes in.
@row_check n=151 row=contract/viola-wait/timed-out sha=1ef73cb268f4 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:149
quote: let deadline = timeout_ms.and_then(|ms| deadline(self.clock.now(), ms));
at: src/run/wait.rs:159-161
quote: if deadline.is_some_and(|d| self.clock.now() >= d) { span.record("timed_out", true); return Ok(json!({"ok": {"timed_out": true}}));
@row_add n=152 row=contract/mcp-wait-tool/timeout-ms area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: MCP wait tool
item: timeout_ms
promise: Clamped to 50 000 ms, which is also the default when none is passed, so a wait with no timeout is answered timed-out after 50 s.
@row_check n=152 row=contract/mcp-wait-tool/timeout-ms sha=4340245eea7b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:42
quote: pub const WAIT_CAP: u64 = 50_000;
at: crates/viola-mcp/src/tools.rs:46
quote: given.map_or(WAIT_CAP, |given| given.min(WAIT_CAP))
@row_add n=153 row=contract/config-json/dialog-hold-s area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: config.json
item: dialog_hold_s
promise: Whole seconds 60 to 3600, default 60, read once at the wrapper's start; any other value gives 60 and one parse-rejected.
@row_check n=153 row=contract/config-json/dialog-hold-s sha=675bc4e22217 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:277
quote: validate(greater_or_equal = 60, less_or_equal = 3600),
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
at: src/obs.rs:223
quote: ObsEvent::ParseRejected,
@row_add n=154 row=decision/dialog-hold-bounds-the-driver area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Dialog hold bounds the driver
rule: A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold.
state: provisional
@row_check n=154 row=decision/dialog-hold-bounds-the-driver sha=6f1741a73351 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:348-349
quote: if self.clock.now() >= deadline { break (None, true);
at: src/run/dialog.rs:345
quote: if state.pending.as_ref().is_some_and(|p| p.handed_back) {
at: src/run/dialog.rs:405
quote: pending.handed_back = true;
@row_add n=155 row=contract/event-line/dialog-expired area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: the event line
item: dialog-expired
promise: Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait.
@row_check n=155 row=contract/event-line/dialog-expired sha=1eebb0b6fcc4 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:378
quote: EventKind::DialogExpired,
at: src/run/dialog.rs:380
quote: json!({"dialog_id": pending.dialog_id, "dialog_kind": pending.kind.as_str()}),
at: crates/viola-core/src/lib.rs:105
quote: Self::DialogExpired,
@row_add n=156 row=contract/cli-exit-codes/13 area=cognitive-accessibility rec=fct-6edxk4iz@3,fct-dx4kie52@3,fct-avr5o2ub@3
boundary: CLI exit codes
item: 13
promise: An answer for an expired or absent dialog is refused not-delivered / unknown-dialog.
@row_check n=156 row=contract/cli-exit-codes/13 sha=ee263177fd22 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:444-447
quote: return Ok(refusal( RefusalReason::NotDelivered, Some(NotDelivered::UnknownDialog.as_str()), ));
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=157 row=contract/send-refusal/input-not-ready area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola send refusal
item: input-not-ready
promise: A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed.
@row_check n=157 row=contract/send-refusal/input-not-ready sha=f8bc7df46a3b v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:128-130
quote: let (readiness, _) = gate.wait_ready(slot.clock.as_ref(), slot.clock.now()); if readiness != Readiness::Ready { return ctx.not_delivered(None, NotDelivered::InputNotReady);
@row_add n=158 row=contract/send-refusal/no-prompt-submitted area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola send refusal
item: no-prompt-submitted
promise: A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted.
@row_check n=158 row=contract/send-refusal/no-prompt-submitted sha=4f077243388b v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/run/send/slot.rs:177-178
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW, _ => CONFIRM_WINDOW_FALLBACK,
@row_add n=159 row=contract/viola-hook-statusline/exit area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: viola hook statusline
item: exit status
promise: Runs the user's own status line command under a 5 s deadline and exits 0 with empty stderr in every case.
@row_check n=159 row=contract/viola-hook-statusline/exit sha=a106a9749d9d v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
at: src/cmd/hook/statusline.rs:59
quote: bound: STATUSLINE_DEADLINE,
at: src/main.rs:394
quote: #[case::hook_failed(Role::Hook, Outcome::Failed, ExitCode::SUCCESS)]
@row_add n=160 row=decision/claude-agents-read-fails-open area=cognitive-accessibility rec=fct-6edxk4iz@3
name: claude agents read fails open
rule: A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's preflight.
@row_check n=160 row=decision/claude-agents-read-fails-open sha=477032173385 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:124-126
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ())) { board::join(&mut board, observations(list));
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
@row_add n=161 row=decision/budget-gate area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Budget gate on automated sends
rule: While a five-hour or seven-day window stands at or over its threshold with a known reset ahead, an automated viola send is refused budget-paused with nothing typed, until that reset or viola release --budget; no threshold value switches the gate off.
guard: path6_a_reading_over_the_threshold_refuses_the_send
@row_check n=161 row=decision/budget-gate sha=55f1d27bd144 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send.rs:103-108
quote: if let Some(budget) = slot.budget.get() { let crossed = budget .evaluate(Trigger::Send, Utc::now()) .map_err(|_| ProtocolError::Internal)?; if let Some(crossed) = crossed { return ctx.budget_paused(&crossed);
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
guard_at: tests/cli_budget.rs:113
@row_add n=162 row=decision/budget-gate-only-on-send area=cognitive-accessibility rec=fct-6edxk4iz@3
name: Only send reads the budget gate
rule: answer, wait, last and pause do not read the budget gate and give under a closed gate what they give under an open one.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_check n=162 row=decision/budget-gate-only-on-send sha=e176f55b3555 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send.rs:103
quote: if let Some(budget) = slot.budget.get() {
guard_at: tests/cli_budget.rs:521
@row_add n=163 row=contract/cli-exit-codes/11 area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: CLI exit codes
item: 11
promise: budget-paused, with detail five-hour or seven-day.
@row_check n=163 row=contract/cli-exit-codes/11 sha=1f270b141899 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
at: src/run/send/budget.rs:12
quote: Some(crossed.window.as_str()),
@row_add n=164 row=contract/config-json/five-hour area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: config.json
item: five_hour
promise: The five-hour window's budget threshold, default 90.
@row_check n=164 row=contract/config-json/five-hour sha=d9d85eb170f4 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:30
quote: const FIVE_HOUR_DEFAULT: u8 = 90;
@row_add n=165 row=contract/config-json/seven-day area=cognitive-accessibility rec=fct-6edxk4iz@3
boundary: config.json
item: seven_day
promise: The seven-day window's budget threshold, default 85.
@row_check n=165 row=contract/config-json/seven-day sha=150db7a6a038 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:31
quote: const SEVEN_DAY_DEFAULT: u8 = 85;
@row_add n=166 row=decision/error-word-detail-hint area=cognitive-accessibility rec=fct-n7d5inyr@1
name: Errors as word, detail and hint
rule: Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
@row_check n=166 row=decision/error-word-detail-hint sha=93efde53d074 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=167 row=contract/cli-refusal/stderr-layout area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: CLI refusal
item: stderr layout
promise: unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
@row_check n=167 row=contract/cli-refusal/stderr-layout sha=0e59e4f8fe8a v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=168 row=decision/unknown-refusal-has-no-hint area=cognitive-accessibility rec=fct-n7d5inyr@1
name: unknown refusal carries no hint
rule: The unknown refusal prints no hint.
why: its detail is a code word or nothing, opaque to a reader, so no hint is printed rather than a guess
@row_check n=168 row=decision/unknown-refusal-has-no-hint sha=5a3eb63ba6b3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:85
quote: RefusalReason::UnverifiedCli | RefusalReason::Unknown => None,
@row_add n=169 row=contract/cli-exit-codes/14 area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-dvgtuxyi@1
boundary: CLI exit codes
item: 14
promise: unknown: a refusal whose word is outside the closed refusal list.
@row_check n=169 row=contract/cli-exit-codes/14 sha=3dd1b3d636fd v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:80
quote: RefusalReason::Unknown.as_str().to_owned()
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
@row_add n=170 row=decision/faults-carry-no-hint area=cognitive-accessibility rec=fct-n7d5inyr@1
name: Faults carry no hint
rule: Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
@row_check n=170 row=decision/faults-carry-no-hint sha=699a17497860 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=171 row=decision/json-one-document-no-hint area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-wkabfqtn@1
name: One --json document with no hint
rule: Under --json a failure gives one document with no hint.
except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document
@row_check n=171 row=decision/json-one-document-no-hint sha=c9314d7788c7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:239
quote: document(&fault_document(detail));
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/send.rs:164
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
@row_add n=172 row=contract/cli-fixed-message/form area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: CLI fixed-message refusal
item: form
promise: viola revive's exit-1 refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
@row_check n=172 row=contract/cli-fixed-message/form sha=c98bfa64915c v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/cmd/run.rs:737
quote: human::refuse(
at: src/cmd/verify.rs:83
quote: human::refuse(
@row_add n=173 row=contract/cli-exit-codes/2 area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-wkabfqtn@1
boundary: CLI exit codes
item: 2
promise: A usage error, or an unusable answer response or send text written as one fixed stderr line with no document.
@row_check n=173 row=contract/cli-exit-codes/2 sha=c8e52a6cc506 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:164
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
at: src/cmd/answer.rs:91-92
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes()); return Ok(ExitCode::from(2));
@row_add n=174 row=contract/start-json-refusal/document area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: run / revive start under --json
item: refusal document
promise: Every refusal of the start revive shares with run gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
@row_check n=174 row=contract/start-json-refusal/document sha=4113029b6967 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/run.rs:420
quote: super::client::refused_document(detail);
@row_add n=175 row=contract/viola-verify/interrupted-pair area=cognitive-accessibility rec=fct-n7d5inyr@1
boundary: viola verify
item: interrupted pair
promise: unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.
@row_check n=175 row=contract/viola-verify/interrupted-pair sha=eeff1b5466fa v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:84-85
quote: "verify was stopped before it finished", "run viola verify again to stamp this CLI",
@row_add n=176 row=contract/cli-exit-codes/21 area=cognitive-accessibility rec=fct-n7d5inyr@1,fct-bhnorrcm@1
boundary: CLI exit codes
item: 21
promise: instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail.
@row_check n=176 row=contract/cli-exit-codes/21 sha=8d8ca8994651 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=177 row=contract/instance-unreachable-json/document area=cognitive-accessibility rec=fct-bhnorrcm@1
boundary: exit 21 under --json
item: document
promise: One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC.
@row_check n=177 row=contract/instance-unreachable-json/document sha=ed43523476e8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=178 row=contract/instance-unreachable-detail/strict-modes-failed area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-zlmstg36@1,fct-mxmekxqi@1
boundary: instance-unreachable detail
item: strict-modes-failed
promise: The instance failed the strict-modes check; refused before any frame or log, no log line written.
@row_check n=178 row=contract/instance-unreachable-detail/strict-modes-failed sha=87e0f9c07a79 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31-35
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() { return Ok(None); } if json { unreachable_document(UnreachableCause::StrictModes);
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=179 row=contract/instance-unreachable-detail/instance-dead area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-tssjslea@1,fct-wqxezfp7@1,fct-xzkz7422@1
boundary: instance-unreachable detail
item: instance-dead
promise: The name has no live endpoint but its instance directory stands, or the instance is stale.
@row_check n=179 row=contract/instance-unreachable-detail/instance-dead sha=08d9b3a90ec0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:112
quote: Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead,
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
@row_add n=180 row=contract/instance-unreachable-detail/unwrapped-name area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-wqxezfp7@1,fct-xzkz7422@1
boundary: instance-unreachable detail
item: unwrapped-name
promise: The name has no live endpoint and no instance directory (absent, a regular file or a symlink).
@row_check n=180 row=contract/instance-unreachable-detail/unwrapped-name sha=aabbfd8617a8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
@row_add n=181 row=contract/instance-unreachable-detail/endpoint-lost area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-tssjslea@1
boundary: instance-unreachable detail
item: endpoint-lost
promise: The wrapper was lost with the call open.
@row_check n=181 row=contract/instance-unreachable-detail/endpoint-lost sha=88c351af6a42 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
@row_add n=182 row=contract/instance-unreachable-detail/server-verify-failed area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-n7d5inyr@1
boundary: instance-unreachable detail
item: server-verify-failed
promise: The process serving the endpoint is not the recorded wrapper and was not contacted.
@row_check n=182 row=contract/instance-unreachable-detail/server-verify-failed sha=b21dad373242 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=183 row=contract/instance-unreachable-detail/socket-dir-refused area=cognitive-accessibility rec=fct-bhnorrcm@1,fct-gh3uczd3@1
boundary: instance-unreachable detail
item: socket-dir-refused
promise: The socket directory was refused; the verb sent no frame.
@row_check n=183 row=contract/instance-unreachable-detail/socket-dir-refused sha=076d48a70312 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=184 row=decision/strict-modes-refusal-before-any-write area=cognitive-accessibility rec=fct-zlmstg36@1,fct-mxmekxqi@1,fct-zwbgfalb@1,fct-bjgpzcmx@1,fct-xzkz7422@1
name: Strict-modes refusal before any write
rule: A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify.
@row_check n=184 row=decision/strict-modes-refusal-before-any-write sha=73476b646f10 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
@row_add n=185 row=decision/socket-dir-refusal-before-any-frame area=cognitive-accessibility rec=fct-gh3uczd3@1,fct-cgrssu3w@1
name: Socket-directory refusal before any frame
rule: A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start.
@row_check n=185 row=decision/socket-dir-refusal-before-any-frame sha=8b51fea1b372 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/run.rs:735
quote: fn refuse_socket_dir(name: &ViolaName) {
@row_add n=186 row=contract/start-holder-refusal/stale-heartbeat area=cognitive-accessibility rec=fct-45izkccw@1,fct-tssjslea@1
boundary: start against a holder, exit 1
item: stale-heartbeat
promise: The holder is stale; stale heartbeat is exit 1's cause alone.
@row_check n=186 row=contract/start-holder-refusal/stale-heartbeat sha=237c0ecd9efb v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: crates/viola-state/src/liveness.rs:112
quote: Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead,
@row_add n=187 row=contract/start-holder-refusal/already-live area=cognitive-accessibility rec=fct-45izkccw@1
boundary: start against a holder, exit 1
item: already-live
promise: The holder is live, or a fresh replayed beat was read.
@row_check n=187 row=contract/start-holder-refusal/already-live sha=3b563b277076 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=188 row=decision/unstattable-home-is-strict-modes-refusal area=cognitive-accessibility rec=fct-ms6n7636@2
name: Unstattable home is a strict-modes refusal
rule: A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
@row_check n=188 row=decision/unstattable-home-is-strict-modes-refusal sha=286adfbd54f5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
@row_add n=189 row=decision/printed-names-are-parsed-arguments area=cognitive-accessibility rec=fct-5kq5fzne@1,fct-adl57ilr@1
name: Printed names are the parsed arguments
rule: Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log.
@row_check n=189 row=decision/printed-names-are-parsed-arguments sha=f5fabcaca4cf v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:93
quote: let (driver, driven): (&str, &str) = (args.driver.as_ref(), args.driven.as_ref());
at: src/cmd/link.rs:123
quote: let _ = write_line(&mut io::stdout().lock(), verb, driver, driven, &ok);
@row_add n=190 row=decision/link-pair-validated-by-wrapper area=cognitive-accessibility rec=fct-uyauvo6g@1
name: Link pairs validated by the wrapper
rule: viola link and viola unlink never refuse a pair client-side; a pair the wrapper refuses as invalid params reaches the verb as a fault, exit 20, no hint.
@row_check n=190 row=decision/link-pair-validated-by-wrapper sha=6ca698829e9d v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
at: src/cmd/link.rs:116
quote: Err(detail) => return Ok(client::fault(args.json, &detail)),
at: src/cmd/client.rs:245
quote: 20
@row_add n=191 row=contract/cli-exit-codes/20 area=cognitive-accessibility rec=fct-uyauvo6g@1
boundary: CLI exit codes
item: 20
promise: A fault, with no hint.
@row_check n=191 row=contract/cli-exit-codes/20 sha=0eadb73b710c v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:245
quote: 20
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
@row_add n=192 row=contract/code-word/format area=cognitive-accessibility rec=fct-yq4bu6pq@1
boundary: code word
item: format
promise: 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
@row_check n=192 row=contract/code-word/format sha=b277380c4929 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:377-381
quote: bytes.len() <= 64 && bytes.first().is_some_and(u8::is_ascii_lowercase) && bytes .iter() .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
@row_add n=193 row=decision/peer-replies-screened-to-code-words area=cognitive-accessibility rec=fct-yq4bu6pq@1
name: Peer replies screened to code words
rule: A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
@row_check n=193 row=decision/peer-replies-screened-to-code-words sha=c8c1b42463f1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:77-81
quote: let reason = if is_code_word(reason) { reason.to_owned() } else { RefusalReason::Unknown.as_str().to_owned() };
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
@row_add n=194 row=decision/unlisted-code-words-stay-visible area=cognitive-accessibility rec=fct-dvgtuxyi@1
name: Unlisted code words stay visible
rule: A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14.
why: visible by design
@row_check n=194 row=decision/unlisted-code-words-stay-visible sha=3d185ab00025 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:76
quote: serde_json::from_value(Value::from(reason)).unwrap_or(RefusalReason::Unknown)
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
at: src/cmd/client.rs:77
quote: let reason = if is_code_word(reason) {
@row_add n=195 row=contract/fault-body/message area=cognitive-accessibility rec=fct-lwbpjniq@1
boundary: fault body under --json and on MCP
item: message
promise: A code outside the five known reads the fixed message wrapper fault with null data.
@row_check n=195 row=contract/fault-body/message sha=162522b010d4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
@row_add n=196 row=contract/human-fault-line/wrapper-fault area=cognitive-accessibility rec=fct-lwbpjniq@1
boundary: human fault line
item: wrapper fault
promise: error: wrapper fault  <code>.
@row_check n=196 row=contract/human-fault-line/wrapper-fault sha=7c111b5846f8 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
@row_add n=197 row=contract/json-fault/internal-error area=cognitive-accessibility rec=fct-aaun2rti@1
boundary: CLI fault under --json
item: internal-error
promise: Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there.
@row_check n=197 row=contract/json-fault/internal-error sha=40947eef47d0 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67
quote: if json_asked(std::env::args_os()) {
at: src/main.rs:68
quote: cmd::internal_error_document();
at: src/main.rs:70
quote: human::internal_error();
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=198 row=decision/client-euid-refusal-test-strategy area=cognitive-accessibility rec=fct-g3h7n6e2@4
name: Client euid refusal test strategy
rule: No foreign peer is driven: the client euid refusal's decision is tested over injected euids and its real read on Linux, and each client seam's wiring is read only by forced refusals in remove-the-guard runs.
@row_check n=198 row=decision/client-euid-refusal-test-strategy sha=5feeeaf2eb66 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:534
quote: assert!(!peer_admitted(Some(own.wrapping_add(1))));
at: crates/viola-channel/src/client.rs:532
quote: assert_eq!(client.server_euid(), Some(own));
@row_add n=199 row=decision/json-failure-documents-are-clean area=cognitive-accessibility rec=fct-5fnuqr36@1
name: --json failure documents are clean
rule: Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
guard: cli_error_documents_hold_no_path_or_upstream_text
@row_check n=199 row=decision/json-failure-documents-are-clean sha=cfadc14b0161 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cli_error_bodies.rs:102-104
quote: assert!(ran.stderr.is_empty(), "{label}: stderr holds bytes"); assert_eq!(ran.stdout.lines().count(), 1, "{label}: one document"); assert!(!ran.stdout.contains('\u{1b}'), "{label}: an ESC byte");
guard_at: tests/cli_error_bodies.rs:122
@row_add n=200 row=decision/error-bodies-have-writer-and-witness area=cognitive-accessibility rec=fct-fuvobsk5@1
name: Error bodies have a writer and a witness
rule: Each error body names a writer and a witness that stand, and each product error writer is named.
guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
@row_check n=200 row=decision/error-bodies-have-writer-and-witness sha=55615c9e5ea4 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_error_bodies.rs:718
quote: if !read(&root.join(body.file)).contains(body.needle) {
at: tests/contract_error_bodies.rs:784
quote: unnamed.extend(unnamed_writers(&file, &text, &named));
guard_at: tests/contract_error_bodies.rs:703
@row_add n=201 row=decision/thiserror-messages-interpolate-nothing area=cognitive-accessibility rec=fct-fuvobsk5@1
name: thiserror messages interpolate nothing
rule: No thiserror message interpolates a value.
guard: thiserror_messages_interpolate_nothing
@row_check n=201 row=decision/thiserror-messages-interpolate-nothing sha=2617fede42f6 v=holds guard=true judge=sonnet head=8834195b0683
searched: #\[error\([^)]*\{ · in src/**/*.rs and crates/**/*.rs
guard_at: tests/contract_error_bodies.rs:794
@row_add n=202 row=decision/one-hint-per-code area=cognitive-accessibility rec=fct-zzwwqxzg@1
name: One hint per code
rule: No two codes share a hint.
guard: no_two_codes_share_a_hint
@row_check n=202 row=decision/one-hint-per-code sha=9ef5abd92620 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cross_exit_causes.rs:117
quote: assert_eq!(shared_hints(CAUSES), Vec::<&str>::new());
guard_at: tests/cross_exit_causes.rs:116
@row_add n=203 row=decision/no-hint-names-release area=cognitive-accessibility rec=fct-zzwwqxzg@1,fct-avr5o2ub@3
name: No hint names release
rule: No hint names the release verb.
guard: no_hint_names_release
@row_check n=203 row=decision/no-hint-names-release sha=9cf8539db9e1 v=holds guard=true judge=sonnet head=8834195b0683
searched: hint:.*release|hint.*"[^"]*release · in src/**/*.rs and crates/**/*.rs (only test assertions that no hint contains it)
searched: release · in src/human/hints.rs (only the doc comment and the test assertion)
guard_at: tests/cross_exit_causes.rs:129
@row_add n=204 row=decision/fixed-cli-strings-pinned-by-literal area=cognitive-accessibility rec=fct-oji2cu7x@2
name: Fixed CLI strings pinned by literal
rule: Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
@row_check n=204 row=decision/fixed-cli-strings-pinned-by-literal sha=0c3d18542b66 v=holds judge=sonnet head=8834195b0683
at: tests/cli_output_plain.rs:55
quote: const HELP: &str = concat!(
at: tests/security_negatives_server_identity.rs:25
quote: const HINT: &str =
at: tests/security_negatives_strict_modes.rs:30
quote: const INSTANCE_PAIR: &str = concat!(
@row_add n=205 row=decision/link-result-lines-one-write-no-sgr area=cognitive-accessibility rec=fct-tfdrn5u5@1
name: Link result lines in one write, no SGR
rule: The link and unlink result lines are each one write with no SGR in any mode.
guard: link_verb_lines_are_the_design_lines_in_one_write
@row_check n=205 row=decision/link-result-lines-one-write-no-sgr sha=558799750574 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/verbs.rs:74
quote: out.write_all(format!("link    {driver} -> {driven}  already linked\n").as_bytes())
at: src/human/verbs.rs:79
quote: out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
guard_at: src/human/verbs.rs:156
@row_add n=206 row=contract/viola-link-line/time area=cognitive-accessibility rec=fct-adl57ilr@1
boundary: viola link line
item: time
promise: The reply's since as UTC clock time in whole seconds with Z.
@row_check n=206 row=contract/viola-link-line/time sha=a86d399b0021 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:86
quote: Ok(at) => at.with_timezone(&Utc).format("%H:%M:%SZ").to_string(),
@row_add n=207 row=decision/state-carried-by-printed-word area=visual-design-verification rec=fct-es7le4as@4
name: State carried by its printed word
rule: Each board state is carried by its printed word (stale, DIALOG <kind>, expired, budget-paused, unverified-cli, idle, busy, n/a), never by colour alone.
@row_check n=207 row=decision/state-carried-by-printed-word sha=a420a259576e v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:43
quote: "unverified-cli"
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
at: src/human/board/cells.rs:25
quote: pub(super) const DIALOG_WORD: &str = "DIALOG";
at: src/human/board/bay.rs:26
quote: let suffix = if expired { " expired" } else { "" };
@row_add n=208 row=decision/colour-decision-inputs area=visual-design-verification rec=fct-es7le4as@4
name: Colour decision inputs
rule: The colour decision reads NO_COLOR (present, empty included), TERM (exactly dumb; 256color), COLORTERM and whether stdout is a terminal, and picks a truecolor, 256 or 16 depth.
@row_check n=208 row=decision/colour-decision-inputs sha=e87a6ac5ae73 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
at: src/human/style.rs:65
quote: || facts.term.as_deref() == Some("dumb")
at: src/human/style.rs:97
quote: colorterm: text("COLORTERM"),
at: src/human/style.rs:98
quote: stdout_tty: io::stdout().is_terminal(),
at: src/human/style.rs:70
quote: if matches!(facts.colorterm.as_deref(), Some("truecolor" | "24bit")) {
@row_add n=209 row=decision/unwrapped-name-truncation area=visual-design-verification rec=fct-es7le4as@4
name: Unwrapped NAME truncation
rule: On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus ..., never inside an escape; a wrapped NAME is never cut, and piped output and --json keep every name whole.
guard: cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it
@row_check n=209 row=decision/unwrapped-name-truncation sha=e563826f337a v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:52-53
quote: let name = if !item.wrapped && name.chars().count() > column { cut(&name, column)
at: src/human/board/cells.rs:127-128
quote: let mut keep = column.saturating_sub(ELLIPSIS.len()); if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) {
guard_at: tests/cross_list_rows.rs:346
@row_add n=210 row=contract/viola-list-stale-row/escape-sequences area=visual-design-verification rec=fct-46bk3wap@2
boundary: viola list stale row on a terminal
item: escape sequences
promise: Exactly DIM, BOLD, RESET, DIM, RESET, with the word stale printed beside the dim.
@row_check n=210 row=contract/viola-list-stale-row/escape-sequences sha=e83f7152587f v=holds judge=sonnet head=8834195b0683
at: tests/cross_list_rows.rs:292
quote: [DIM, BOLD, RESET, DIM, RESET],
at: src/human/style.rs:38
quote: stale: "\x1b[2m",
@row_add n=211 row=decision/cli-a11y-by-literal-tests area=a11y-testing-tool-pick rec=fct-dlpkde76@3
name: CLI a11y checked by literal tests
rule: With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.
@row_check n=211 row=decision/cli-a11y-by-literal-tests sha=0e4a62e11188 v=unverifiable judge=sonnet head=8834195b0683
searched: std::process::Command literal-pin approach is a test-design practice · in tests/*.rs
why: not a code claim
@row_add n=212 row=decision/tui-boundary-driver-pin area=a11y-testing-tool-pick rec=fct-dlpkde76@3
name: TUI boundary driver
rule: The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins.
@row_check n=212 row=decision/tui-boundary-driver-pin sha=3f9120d36d45 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:217
quote: portable-pty = "=0.9.0"
@row_add n=213 row=absence/rendered-page area=bootstrap-phases-derive-for-route-setup-project rec=fct-trh3q5nm@3,fct-lfnx2oea@2,fct-d5bgiald@2,fct-56elcbkj@3
capability: A rendered page, and with it any browser a11y tooling
@row_check n=213 row=absence/rendered-page sha=1ae7c9c31745 v=holds judge=sonnet head=8834195b0683
searched: axe-core|playwright|pa11y|lighthouse|aria- · in Cargo.toml, **/*.rs, .github/**, scripts/**, package.json (only hits: a refusal test of playwright as a suite value, crates/viola-e2e/src/harness/gate.rs:418, and a cookbook note)
@row_add n=214 row=decision/manual-screen-reader-pass area=bootstrap-phases-derive-for-route-setup-project rec=fct-t3p7hz2f@2
name: Manual screen reader pass
rule: The screen reader check is a manual Orca pass owned by the founder, with no automated test spec.
@row_check n=214 row=decision/manual-screen-reader-pass sha=e891819974e5 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=215 row=decision/a11y-runs-in-existing-test-suites area=bootstrap-phases-derive-for-route-setup-project rec=fct-yftct3hq@2,fct-gcqo3m4v@2
name: a11y runs in the existing test suites
rule: The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.
@row_check n=215 row=decision/a11y-runs-in-existing-test-suites sha=2c6977d984d9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:14-23
quote: pub const SUITES: [&str; 8] = [ "nextest-unit", "nextest-integration", "doctest", "nextest-e2e", "mutants", "coverage", "perf", "fuzz-replay", ];
at: .github/workflows/ci.yml:38
quote: run: bash scripts/agent-run.sh run --coverage
@row_add n=216 row=decision/a11y-failures-are-nextest-failures area=bootstrap-phases-derive-for-route-setup-project rec=fct-ycol6rsi@1,fct-s3apctwx@1
name: a11y failures are nextest failures
rule: An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
@row_check n=216 row=decision/a11y-failures-are-nextest-failures sha=6d6a5072c880 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:77
quote: pub failures: Vec<String>,
at: crates/viola-e2e/src/harness/gate.rs:411
quote: for bad in ["", "a11y", "coverage,", "unit", "coverage,bogus"] {
@row_add n=217 row=decision/gate-requires-coverage-doctest area=ci-integration rec=fct-56elcbkj@3
name: Gate requires coverage and doctest
rule: The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.
@row_check n=217 row=decision/gate-requires-coverage-doctest sha=76b40ab8e008 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
@row_add n=218 row=decision/stdin-classifier-non-editing-list area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Closed non-editing stdin list
rule: Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel.
why: ratified, the exact grammars chosen after a narrower option was shown
@row_check n=218 row=decision/stdin-classifier-non-editing-list sha=0ee908be5291 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:519-520
quote: (None, [], b'n') => numbers == b"0", (Some(b'?'), [], b'n') => numbers == b"997;1" || numbers == b"997;2",
at: src/run/wheel.rs:525
quote: _ => false,
@row_add n=219 row=decision/human-key-takes-the-wheel area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Human key takes the wheel
rule: A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed.
guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
@row_check n=219 row=decision/human-key-takes-the-wheel sha=284dfc75ad35 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send.rs:135
quote: RefusalReason::HumanTyping,
at: src/run/wheel.rs:373
quote: pub(crate) fn feed(&mut self, bytes: &[u8]) -> bool {
guard_at: tests/cli_send.rs:945
@row_add n=220 row=decision/zero-viola-bytes-on-the-terminal area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Zero viola bytes on the terminal
rule: The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run.
@row_check n=220 row=decision/zero-viola-bytes-on-the-terminal sha=e88667116eef v=holds judge=sonnet head=8834195b0683
at: tests/tui_passthrough.rs:61-65
quote: fn assert_no_viola_bytes(stream: &[u8]) { for literal in VIOLA_LITERALS { assert!(!holds(stream, literal), "viola wrote {literal:?}"); } }
at: tests/tui_passthrough.rs:250-254
quote: assert_eq!( stream, prompt_and_finish(direct, &direct_receipt, 0), "wrapped and unwrapped streams differ" );
@row_add n=221 row=decision/statusline-output-passed-byte-for-byte area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Statusline output passed byte for byte
rule: viola hook statusline hands the user's own status line output to the child byte for byte and writes nothing to the terminal itself.
guard: passthrough_with_a_statusline_source_adds_no_viola_bytes
@row_check n=221 row=decision/statusline-output-passed-byte-for-byte sha=be8ae7d11b87 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
guard_at: tests/tui_passthrough.rs:263
@row_add n=222 row=decision/live-readings-never-gate area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Live readings never gate
rule: Live readings of the takeover clause on a real terminal are supplemental and never gating.
@row_check n=222 row=decision/live-readings-never-gate sha=37afb0c625ef v=unverifiable judge=sonnet head=8834195b0683
searched: wtype · in tests/tui_wheel.rs, tests/tui_passthrough.rs, tests/cli_budget.rs, tests/support/outer_pty.rs
why: no evidence found
@row_add n=223 row=decision/boundary-tests-sequenced-on-records area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Boundary tests sequenced on records
rule: The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
@row_check n=223 row=decision/boundary-tests-sequenced-on-records sha=9047614f3af3 v=holds judge=sonnet head=8834195b0683
at: tests/cli_send.rs:967
quote: wait_send_requests(&home, 2);
at: tests/cli_send.rs:970
quote: wait_events(&dir, "the human-input wheel record", |lines| {
@row_add n=224 row=decision/boundary-check-never-inspects-claude-content area=keyboard-test-harness rec=fct-avr5o2ub@3
name: Boundary check never inspects claude content
rule: The keyboard boundary check never inspects claude content.
@row_check n=224 row=decision/boundary-check-never-inspects-claude-content sha=82a52d54934b v=holds judge=sonnet head=8834195b0683
searched: vt100|screen\(|contents\(\) · in tests/tui_wheel.rs, tests/tui_passthrough.rs, tests/cli_budget.rs, tests/support/outer_pty.rs
@row_add n=225 row=decision/cli-screen-reader-test-setup area=screen-reader-test-pattern rec=fct-tocuc2rh@2
name: CLI screen-reader test setup
rule: The cli surface is screen-reader tested with Orca on GNOME Terminal.
@row_check n=225 row=decision/cli-screen-reader-test-setup sha=7033a41ef72f v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=226 row=decision/tui-screen-reader-test-setup area=screen-reader-test-pattern rec=fct-tocuc2rh@2
name: TUI screen-reader test setup
rule: The tui surface is screen-reader tested with Orca on foot under Hyprland.
@row_check n=226 row=decision/tui-screen-reader-test-setup sha=f23bc5715225 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=227 row=contract/structured-violation-json/cli-tui-a11y-rows area=structured-violation-json-schema rec=fct-7jyl4oxv@1
boundary: Structured violation JSON
item: a11y rows for cli and tui
promise: The cli and tui surfaces emit no a11y rows.
@row_check n=227 row=contract/structured-violation-json/cli-tui-a11y-rows sha=5b024da38f67 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:105
quote: suite.failures.push(format!(
searched: a11y · in crates/viola-e2e/src (only a gate.rs test listing "a11y" as an unknown suite)
searched: a11y-violation · in crates/viola-core/src/obs.rs (asserted not a variant)
@row_add n=228 row=contract/structured-violation-json/suites-failures area=structured-violation-json-schema rec=fct-7jyl4oxv@1
boundary: Structured violation JSON
item: suites[].failures[]
promise: Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
@row_check n=228 row=contract/structured-violation-json/suites-failures sha=64a5c505e8ec v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:105
quote: suite.failures.push(format!(
at: crates/viola-e2e/src/harness/run/nextest.rs:79
quote: suite.failures.push(format!("{tool}-exit-{code}"));
@row_add n=229 row=decision/no-assertions-for-plugin-and-ipc area=1-a11y-scope-summary rec=fct-lgjt6tbz@1,fct-rgudbdwz@1
name: Plugin and IPC channel not assertable
rule: The Claude Code plugin hooks, the stdio MCP server and the wrapper IPC channel are not-assertable and carry no assertions.
why: ipc-internal with no UI surface, machine-to-machine traffic only
@row_check n=229 row=decision/no-assertions-for-plugin-and-ipc sha=8015ac063514 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=230 row=decision/no-assertions-for-internal-crates area=1-a11y-scope-summary rec=fct-jfjbxqvy@1,fct-rgudbdwz@1
name: Internal crates not assertable
rule: The internal crates of the modular monolith are not-assertable and carry no assertions.
why: library-only, with no UI surface
@row_check n=230 row=decision/no-assertions-for-internal-crates sha=db76fa2490b6 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=231 row=decision/no-wcag-conformance-claim area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-lpulrs7q@1,fct-ms7djxet@3,fct-vtox22ts@4,fct-twqx5x46@3
name: No WCAG conformance claim
rule: No WCAG conformance is claimed for the cli or the `viola run` TUI, whether `viola run` or a passed `viola revive` started it; the output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 and are not a11y-tool evidence.
why: WCAG does not apply directly to terminal output, and no automated a11y tool reaches terminal output
@row_check n=231 row=decision/no-wcag-conformance-claim sha=f8f98ed92f74 v=holds judge=sonnet head=8834195b0683
searched: WCAG · in src/**/*.rs and crates/**/*.rs
@row_add n=232 row=decision/no-sgr-in-plain-modes area=1-a11y-scope-summary rec=fct-sf2opzaw@1,fct-6ghxxgxr@1,fct-633dmm7e@1,fct-vtox22ts@4
name: No SGR in plain modes
rule: No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
@row_check n=232 row=decision/no-sgr-in-plain-modes sha=7d04a23eea79 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62
quote: if facts.json
at: src/human/style.rs:63
quote: || facts.passthrough
at: src/human/style.rs:64
quote: || facts.no_color
at: src/human/style.rs:65
quote: || facts.term.as_deref() == Some("dumb")
at: src/human/style.rs:66
quote: || !facts.stdout_tty
@row_add n=233 row=decision/paths-not-carried area=1-a11y-scope-summary rec=fct-zggdjhnj@1
name: Paths not carried
rule: Tests Critical Path 1 (the `run` start sequence), E1 (unwrapped hooks no-op), E2 (`CLAUDE*` env strip) and security vectors 1–9 are not carried as paths; their user-visible outcomes are covered by Path 4.
why: they run on tui/ipc-internal with no viola-owned UI
@row_check n=233 row=decision/paths-not-carried sha=a6696499c302 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=234 row=decision/cognitive-accessibility-not-triggered area=1-a11y-scope-summary rec=fct-2ylyatq6@1
name: Cognitive accessibility not triggered
rule: The cognitive-accessibility trigger is none: no cognitive-accessibility rigor is required.
why: the brief has no cognitive, older-user or language-proficiency signal
@row_check n=234 row=decision/cognitive-accessibility-not-triggered sha=5a752c22309b v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=235 row=decision/tier-single-source-of-rigor area=1-a11y-scope-summary rec=fct-ov6v75dw@1
name: Tier as single source of accessibility rigor
rule: The tier is the single source of truth for how much accessibility rigor downstream skills apply, in the phase loop per chunk and in cross-cutting reads from route and setup-project.
@row_check n=235 row=decision/tier-single-source-of-rigor sha=306778665b6c v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=236 row=decision/human-keystrokes-never-blocked area=2-a11y-strategy rec=fct-7a7ijng6@1,fct-pzkrh6fa@1,fct-csjr2vg7@2,fct-itb2wxsm@2
name: Human keystrokes never blocked
rule: A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current atomic paste.
@row_check n=236 row=decision/human-keystrokes-never-blocked sha=d3b11834c5b9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:48
quote: if to.write_all(&buf[..n]).and_then(|()| to.flush()).is_err() {
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=237 row=decision/cli-output-discipline-assertion-tools area=4-aria-patterns-roles rec=fct-yzqlnwsx@4
name: CLI assertable as output discipline
rule: The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
@row_check n=237 row=decision/cli-output-discipline-assertion-tools sha=2380943b81db v=holds judge=sonnet head=8834195b0683
at: tests/support/cli.rs:108
quote: let mut command = Command::new(VIOLA);
at: tests/support/outer_pty.rs:12
quote: use viola_pty::{PortablePty, Pty as _, Size, SpawnSpec};
@row_add n=238 row=decision/no-terminal-conformance-claim area=4-aria-patterns-roles rec=fct-yzqlnwsx@4,fct-ijnl2u6y@4,fct-mkekg7f5@1,fct-uok334yy@6,fct-6edxk4iz@3,fct-es7le4as@4
name: No conformance claim for the terminal
rule: The cli checks make no conformance claim for the terminal; they only stand in for SC 1.3.2, 1.4.1 and 3.3.1.
@row_check n=238 row=decision/no-terminal-conformance-claim sha=56a433cb2b4c v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=239 row=decision/portable-pty-outer-pty-driver area=4-aria-patterns-roles rec=fct-4ktfiloi@3,fct-twqx5x46@3
name: portable-pty as the outer-PTY boundary driver
rule: The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
@row_check n=239 row=decision/portable-pty-outer-pty-driver sha=a162b74ba55d v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:217
quote: portable-pty = "=0.9.0"
at: tests/support/outer_pty.rs:12
quote: use viola_pty::{PortablePty, Pty as _, Size, SpawnSpec};
@row_add n=240 row=decision/wrapped-tui-reads-as-unwrapped area=7-screen-reader-support rec=fct-wawttggc@2,fct-tocuc2rh@2
name: Wrapped TUI reads as unwrapped
rule: The wrapped `claude` TUI must read to a screen reader the same as unwrapped.
why: Must-Work O1
@row_check n=240 row=decision/wrapped-tui-reads-as-unwrapped sha=d75224d07e94 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=241 row=decision/manual-checks-supplemental-only area=11-a11y-anti-patterns-never-do-these rec=fct-3vxnlnbx@1
name: Manual checks are supplemental only
rule: Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
@row_check n=241 row=decision/manual-checks-supplemental-only sha=940c87534c92 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=242 row=contract/revive-json-refusal/socket-dir-refused area=a11y-scope-summary rec=fct-k76k2lf7@2,fct-cgrssu3w@1
boundary: viola revive --json refusals
item: socket-dir-refused
promise: One exit-1 document with `socket-dir-refused` its code on stdout, in place of the human pair, before any snapshot, bind or child spawn, holding no path, pid or uid, with stderr empty and no hint.
@row_check n=242 row=contract/revive-json-refusal/socket-dir-refused sha=b785bc89be8e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_none n=1 rec=fct-bskn6eoi@1 type=S
@row_none n=2 rec=fct-wdvdzuew@1 type=0
what: an editing note for the plan document itself
@row_none n=3 rec=fct-6jmx2zce@1 type=S
@row_none n=4 rec=fct-6rq6vcoc@1 type=S
@row_none n=5 rec=fct-fp5ry76t@1 type=S
@row_none n=6 rec=fct-maahpp3k@1 type=S
@row_none n=7 rec=fct-gbsh2fhc@1 type=S
@row_none n=8 rec=fct-vliofs34@1 type=S
@row_none n=9 rec=fct-ooazrtpo@1 type=0
what: a list of source citations
@row_none n=10 rec=fct-yjr6ng6y@1 type=0
what: a list of source citations
@row_none n=11 rec=fct-gr6yt6ul@1 type=0
what: an N/A ARIA-roles line for terminal surfaces
@row_none n=12 rec=fct-ayv3gkd7@1 type=0
what: a list of source citations
@row_none n=13 rec=fct-u3smqwc5@1 type=0
what: a list of source citations
@row_none n=14 rec=fct-xhd44rln@1 type=S
@row_none n=15 rec=fct-v4k6yuiy@1 type=S
@row_none n=16 rec=fct-tdneapld@1 type=S
@row_none n=17 rec=fct-dtnxd75i@1 type=S
@row_none n=18 rec=fct-o3rrybxc@1 type=S
@row_none n=19 rec=fct-y24ge3fw@1 type=S
@row_none n=20 rec=fct-3frlwnx5@1 type=S
@row_none n=21 rec=fct-4lys5tdv@1 type=S
@row_none n=22 rec=fct-sutj5ddg@1 type=S
@row_none n=23 rec=fct-gmsx3s7a@2 type=S
@row_none n=24 rec=fct-q4opus3z@1 type=S
@row_none n=25 rec=fct-lbor4q4u@1 type=S
@row_none n=26 rec=fct-64bntlu7@1 type=S
@row_none n=27 rec=fct-ypjst57k@1 type=S
@row_none n=28 rec=fct-cgy7rc2r@1 type=S
@row_none n=29 rec=fct-3xiq4dqd@2 type=5
@row_none n=30 rec=fct-r2rqn3md@1 type=5
@row_none n=31 rec=fct-wbijh7na@1 type=S
@row_none n=32 rec=fct-h4gwghks@1 type=S
@row_none n=33 rec=fct-idxet2fd@1 type=S
@row_none n=34 rec=fct-7gxxr43g@1 type=S
@row_none n=35 rec=fct-jbtnxl7p@1 type=S
@row_none n=36 rec=fct-4reqosrn@1 type=S
@row_none n=37 rec=fct-p4ciplii@1 type=S
@row_none n=38 rec=fct-ixta2kmh@1 type=S
@row_none n=39 rec=fct-ync5zj2r@1 type=0
what: the WCAG success criteria the error pattern is mapped to
@row_none n=40 rec=fct-nl3a6agd@1 type=S
@row_none n=41 rec=fct-und5fgbp@1 type=S
@row_none n=42 rec=fct-bxtkmwvm@1 type=S
@row_none n=43 rec=fct-ow4aqcke@1 type=S
@row_none n=44 rec=fct-j7gtyyjf@1 type=S
@row_none n=45 rec=fct-bskmbnbb@1 type=S
@row_none n=46 rec=fct-3i5p4gas@1 type=S
@row_none n=47 rec=fct-bit55egn@1 type=S
@row_none n=48 rec=fct-jj5q2zzy@1 type=S
@row_none n=49 rec=fct-x6z7pmrz@1 type=S
@row_none n=50 rec=fct-tuewsa34@1 type=S
@row_none n=51 rec=fct-uq2yxg4r@1 type=S
@row_none n=52 rec=fct-m7m7gjre@1 type=S
@row_none n=53 rec=fct-mc7fijew@1 type=4
@row_none n=54 rec=fct-u64ggudd@1 type=0
what: a table row marking ARIA landmarks as not applicable to the vendor claude TUI
@row_none n=55 rec=fct-jgpx3odq@2 type=5
@row_none n=56 rec=fct-cetk7x7u@2 type=5
@row_none n=57 rec=fct-52gvy64u@2 type=5
@row_none n=58 rec=fct-trymhbcp@2 type=5
@row_none n=59 rec=fct-dmbuxeqb@1 type=5
@row_none n=60 rec=fct-kia5lupj@1 type=5
@row_none n=61 rec=fct-mzgdpnxi@1 type=S
@row_none n=62 rec=fct-agfwiito@1 type=7
@row_none n=63 rec=fct-b5v555it@1 type=0
what: a note on how the route and setup-project skills use the bootstrap phase list
@row_none n=64 rec=fct-3siidzor@1 type=S
