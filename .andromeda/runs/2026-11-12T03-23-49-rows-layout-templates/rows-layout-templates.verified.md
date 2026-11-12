@row_add n=1 row=absence/web-surface area=surface-web-spa rec=fct-jdnb6ocv@2
capability: A web surface: viola renders no page
@row_check n=1 row=absence/web-surface sha=50d0039632ba v=holds judge=sonnet head=8834195b0683
searched: axum|actix|warp|rocket|hyper|tide|poem|leptos|yew|wasm-bindgen|tiny_http|<html|text/html · in **/Cargo.toml and **/*.rs (only false matches on "hyperfine" in crates/viola-e2e/src/harness)
searched: **/*.{html,tsx,jsx,vue,svelte} · in the repository (no files)
@row_add n=2 row=decision/clap-derive-cli-parsing area=surface-cli rec=fct-vnzdqtob@2
name: clap derive for CLI parsing
rule: The viola command line is parsed with clap 4 in its derive form.
@row_check n=2 row=decision/clap-derive-cli-parsing sha=749f5732d7e9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:43
quote: #[derive(Parser)]
at: Cargo.toml:197
quote: clap = { version = "=4.6.7", default-features = false, features = [
@row_add n=3 row=decision/std-isterminal-tty-detection area=surface-cli rec=fct-vnzdqtob@2
name: TTY detection through std IsTerminal
rule: Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate.
@row_check n=3 row=decision/std-isterminal-tty-detection sha=7940fa0b2484 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:6
quote: use std::io::{self, IsTerminal as _};
at: src/human/style.rs:98
quote: stdout_tty: io::stdout().is_terminal(),
@row_add n=4 row=decision/hand-written-sgr-styling area=surface-cli rec=fct-vnzdqtob@2
name: Hand-written SGR styling module
rule: Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate.
@row_check n=4 row=decision/hand-written-sgr-styling sha=81db09b9d36e v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:32-34
quote: Self::Truecolor => "\x1b[38;2;217;119;6m", Self::Ansi256 => "\x1b[38;5;172m", Self::Ansi16 => "\x1b[33m",
at: src/human/board/cells.rs:62
quote: fn padding(text: &str, width: usize) -> String {
@row_add n=5 row=contract/cli-token-rendering/color-accent area=surface-cli rec=fct-bb67thmp@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-accent
promise: The design system's attention SGR token at truecolor, 256 or 16-colour depth, set only on the DIALOG word and always followed by the uncoloured kind word.
@row_check n=5 row=contract/cli-token-rendering/color-accent sha=b14dff7edbcb v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:32
quote: Self::Truecolor => "\x1b[38;2;217;119;6m",
at: src/human/style.rs:33
quote: Self::Ansi256 => "\x1b[38;5;172m",
at: src/human/style.rs:34
quote: Self::Ansi16 => "\x1b[33m",
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=6 row=contract/cli-token-rendering/color-text-muted area=surface-cli rec=fct-xw7xe4pu@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-text-muted
promise: The stale token: SGR dim across a whole stale row, and never without the word stale in LIVE.
@row_check n=6 row=contract/cli-token-rendering/color-text-muted sha=06a78a456766 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
at: src/human/board/rows.rs:66
quote: item.liveness.as_str(),
@row_add n=7 row=contract/cli-token-rendering/callsign-role area=surface-cli rec=fct-jyy7fp52@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Callsign role
promise: The callsign token: SGR bold on NAME values only.
@row_check n=7 row=contract/cli-token-rendering/callsign-role sha=a1eb037e9530 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:39
quote: callsign: "\x1b[1m",
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
@row_add n=8 row=contract/cli-token-rendering/color-text-primary-secondary area=surface-cli rec=fct-bzzlsqum@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-text-primary / color-text-secondary
promise: The terminal's default foreground; the terminal owns ink.
@row_check n=8 row=contract/cli-token-rendering/color-text-primary-secondary sha=b47826e03434 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:18
quote: pub(crate) struct Tokens {
@row_add n=9 row=contract/cli-token-rendering/color-role-handoff area=surface-cli rec=fct-lrmne7xu@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-role-handoff
promise: Not used; link output is plain.
@row_check n=9 row=contract/cli-token-rendering/color-role-handoff sha=ee1642bbca49 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
@row_add n=10 row=contract/cli-token-rendering/color-surface area=surface-cli rec=fct-rl3k2zcm@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-surface-*
promise: Never set; the terminal background belongs to the user.
@row_check n=10 row=contract/cli-token-rendering/color-surface sha=3d154d171049 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:18
quote: pub(crate) struct Tokens {
@row_add n=11 row=contract/cli-token-rendering/label-role area=surface-cli rec=fct-42zyjbxs@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Label role
promise: Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain.
@row_check n=11 row=contract/cli-token-rendering/label-role sha=a022e79a9b7e v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=12 row=contract/cli-token-rendering/data-role area=surface-cli rec=fct-frgtrpdl@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Data role
promise: Tabular figures and timestamps.
@row_check n=12 row=contract/cli-token-rendering/data-role sha=05303c9761ad v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=13 row=contract/cli-token-rendering/spacing area=surface-cli rec=fct-lxjb27mz@2,fct-xuyt7znh@1
boundary: CLI token rendering
item: Spacing
promise: The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
@row_check n=13 row=contract/cli-token-rendering/spacing sha=7dd5510b8372 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:95-96
quote: "{open}{shown_name}{}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \ {shown_dialog}{}  {cli}{close}",
@row_add n=14 row=contract/cli-token-rendering/radius area=surface-cli rec=fct-uyipavwi@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: radius-*
promise: No terminal analogue; square brackets `[ ]` keep the ruled-rectangle reading.
@row_check n=14 row=contract/cli-token-rendering/radius sha=6ad241e9ed2d v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=15 row=contract/cli-token-rendering/motion-tokens area=surface-cli rec=fct-ndkpwvcx@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Motion tokens
promise: None; nothing redraws in place.
@row_check n=15 row=contract/cli-token-rendering/motion-tokens sha=73b2074122a8 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:105
quote: out.write_all(text.as_bytes())
@row_add n=16 row=decision/one-token-vocabulary-on-cli area=surface-cli rec=fct-ndkpwvcx@1
name: One token vocabulary resolved on the cli
rule: The one design token vocabulary (color-*, space-*, radius-*, the typography roles) resolves on the cli to the SGR tokens, the terminal default foreground or not used, through ANSI depth, the two-space gutter, ASCII `->` / ` - ` and brackets for the ruled box.
@row_check n=16 row=decision/one-token-vocabulary-on-cli sha=9a5585276268 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:30-35
quote: pub(crate) const fn tokens(self) -> Tokens { let attention = match self { Self::Truecolor => "\x1b[38;2;217;119;6m", Self::Ansi256 => "\x1b[38;5;172m", Self::Ansi16 => "\x1b[33m", };
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
@row_add n=17 row=decision/human-tty-expression-level area=surface-cli rec=fct-ekdqr4ye@1
name: Human TTY expression level
rule: On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
@row_check n=17 row=decision/human-tty-expression-level sha=6a2718a92777 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:94
quote: let mut lines = vec![bay_line(budget, &board.skipped, now)];
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
@row_add n=18 row=decision/plain-output-modes area=surface-cli rec=fct-ekdqr4ye@1
name: Plain output modes
rule: Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
@row_check n=18 row=decision/plain-output-modes sha=5fa17ccc52a1 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62-66
quote: if facts.json || facts.passthrough || facts.no_color || facts.term.as_deref() == Some("dumb") || !facts.stdout_tty
@row_add n=19 row=decision/readback-mirror-signature area=surface-cli rec=fct-w4oklt2h@1
name: Readback mirror as the send signature
rule: The readback mirror `[RB]` / `[  ]` / `[/ ]` is the first column of viola send output, in an issue line and an outcome line.
@row_check n=19 row=decision/readback-mirror-signature sha=4869cd0a4a23 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:15
quote: out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=20 row=contract/viola-send-output/issue-line area=surface-cli rec=fct-3xzidqkr@1,fct-w4oklt2h@1
boundary: viola send output
item: issue line
promise: `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
@row_check n=20 row=contract/viola-send-output/issue-line sha=b2a3b4bb5f44 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:104
quote: let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
at: src/cmd/send.rs:102
quote: if issue_line_shows(self.json, io::stdout().is_terminal()) {
at: src/cmd/send.rs:206
quote: out.open();
@row_add n=21 row=contract/viola-send-output/read-back area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: read back
promise: `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by the matching prompt-submitted.
@row_check n=21 row=contract/viola-send-output/read-back sha=5bfaa67cdef8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:216
quote: out.read_back(&ok);
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
at: src/run/send.rs:155
quote: Some(submitted_at) => ctx.confirm(cursor, &submitted_at, slot.clock.now()),
@row_add n=22 row=contract/viola-send-output/read-back-clear area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: read back for /clear
promise: On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id.
@row_check n=22 row=contract/viola-send-output/read-back-clear sha=059cb107e1c0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:192
quote: ("/clear", Some(PostCondition::NewSession)),
at: src/run/send/slot.rs:111
quote: Some("clear") if prior.as_deref() != Some(id) => Some(PostCondition::NewSession),
at: src/run/send/classify.rs:30
quote: Some((_, Some(post))) if cli_verified => Confirms::Post(*post),
@row_add n=23 row=contract/viola-send-output/read-back-compact area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: read back for /compact
promise: On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted.
@row_check n=23 row=contract/viola-send-output/read-back-compact sha=c3eaaff2eaaa v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:112
quote: Some("compact") if prior.as_deref() == Some(id) => {
at: crates/viola-agent-claude/src/screen.rs:26
quote: pub const COMPACT_CONFIRM_WINDOW: Duration = Duration::from_secs(40);
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
@row_add n=24 row=contract/viola-send-output/unable area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: unable
promise: `[/ ] unable …` prints on stderr with exit 13 (or 10/11/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
@row_check n=24 row=contract/viola-send-output/unable sha=242e85207560 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
at: src/cmd/send.rs:141
quote: human::write_send_unable(&mut io::stderr().lock(), self.name, &shown, hint.as_deref());
at: src/cmd/send.rs:67-71
quote: RefusalReason::HumanTyping => 10, RefusalReason::BudgetPaused => 11, RefusalReason::UnverifiedCli => 12, RefusalReason::NotDelivered => 13, RefusalReason::Unknown => 14,
at: src/cmd/send.rs:193-195
quote: out.unable(RefusalReason::NotDelivered.as_str(), Some(detail.as_str())); run::log_self_exit(13, None); return Ok(13);
@row_add n=25 row=contract/viola-send-output/unconfirmable area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: unconfirmable
promise: `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, and /compact or /clear on an unverified CLI version.
@row_check n=25 row=contract/viola-send-output/unconfirmable sha=86885493f097 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:33
quote: out.write_all(mirror_line("[  ]", "unconfirmable", name, note).as_bytes())
at: crates/viola-agent-claude/src/ledger.rs:193
quote: ("/remote-control", None),
at: src/run/send/classify.rs:31
quote: Some(_) => Confirms::Nothing,
at: src/cmd/send.rs:213-214
quote: if ok["confirmed"] == false { out.unconfirmable(&ok);
@row_add n=26 row=decision/shared-refusal-word-column area=surface-cli rec=fct-byp7w5z4@1
name: Shared refusal word column
rule: The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
@row_check n=26 row=decision/shared-refusal-word-column sha=809a964a0cfe v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/cmd/client.rs:272
quote: let _ = human::write_unable(
at: src/cmd/answer.rs:81
quote: let _ = human::write_unable(&mut io::stderr().lock(), self.name, &shown, hint);
at: src/cmd/wait.rs:51
quote: client::unable_unreachable(name, cause);
at: src/cmd/last.rs:38
quote: client::unable_unreachable(name, cause);
at: src/cmd/link.rs:100
quote: client::unable_unreachable(driven, cause);
at: src/cmd/pause.rs:64
quote: client::unable_unreachable(name, cause);
at: src/cmd/send.rs:141
quote: human::write_send_unable(&mut io::stderr().lock(), self.name, &shown, hint.as_deref());
@row_add n=27 row=contract/viola-send-output/json-mode area=surface-cli rec=fct-byp7w5z4@1
boundary: viola send output
item: --json
promise: Under --json the mirror is replaced by the typed ok / refusal document.
@row_check n=27 row=contract/viola-send-output/json-mode sha=4333937b7483 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:109-110
quote: if self.json { return client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/send.rs:133
quote: return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
@row_add n=28 row=decision/dialog-attention-only-on-board area=surface-cli rec=fct-byp7w5z4@1
name: DIALOG attention word only on the board
rule: The amber DIALOG attention word appears only in viola list.
@row_check n=28 row=decision/dialog-attention-only-on-board sha=17d7ba9c8b21 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=29 row=contract/cli-verbs/list area=surface-cli rec=fct-s2seqjyg@1
boundary: CLI verbs
item: viola list
promise: The board: every session as one strip row, with the BAY context header.
@row_check n=29 row=contract/cli-verbs/list sha=dd1ac0b73f4b v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:65
quote: let _ = human::write_board(
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
@row_add n=30 row=contract/cli-verbs/send area=surface-cli rec=fct-5vvtaix2@1
boundary: CLI verbs
item: viola send <target>
promise: Types at a turn boundary and reports read back, unable or unconfirmable.
@row_check n=30 row=contract/cli-verbs/send sha=5414f0531ee5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:216
quote: out.read_back(&ok);
at: src/cmd/send.rs:214
quote: out.unconfirmable(&ok);
at: src/cmd/send.rs:221
quote: out.unable(&reason, detail.as_deref());
@row_add n=31 row=contract/cli-verbs/wait area=surface-cli rec=fct-36lpgwnn@1
boundary: CLI verbs
item: viola wait <target>
promise: Returns one turn-boundary result, with no spinner.
@row_check n=31 row=contract/cli-verbs/wait sha=06a904fe7cb1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:98-100
quote: /// The result line on stdout (design-system cli Component Patterns 3). fn show(name: &str, ok: &Value, timeout_ms: Option<u64>) { let mut out = io::stdout().lock();
@row_add n=32 row=contract/cli-verbs/last area=surface-cli rec=fct-36lpgwnn@1
boundary: CLI verbs
item: viola last <target>
promise: Returns the last assistant message, with no spinner.
@row_check n=32 row=contract/cli-verbs/last sha=5a26a71cce6f v=holds judge=sonnet head=8834195b0683
at: src/cmd/last.rs:29
quote: pub(crate) fn last(home: &Path, args: &LastArgs) -> anyhow::Result<ExitCode> {
@row_add n=33 row=decision/spoken-phraseology-verb-output area=surface-cli rec=fct-xicxxmup@1
name: Spoken phraseology for wheel, handoff and dialog verbs
rule: The output of viola pause, release, link, unlink and answer is spoken phraseology.
@row_check n=33 row=decision/spoken-phraseology-verb-output sha=b5e7eeaf8ff2 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:12
quote: out.write_all(format!("answered  {name}  dialog {dialog_id}\n").as_bytes())
at: src/human/verbs.rs:17
quote: out.write_all(format!("{name}  wheel human  manual-pause  I have control\n").as_bytes())
at: src/human/verbs.rs:22
quote: out.write_all(format!("{name}  wheel {holder}  you have control\n").as_bytes())
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/verbs.rs:79
quote: out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
@row_add n=34 row=decision/single-progress-pattern area=surface-cli rec=fct-4kxdu6am@1
name: Single progress pattern
rule: The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
@row_check n=34 row=decision/single-progress-pattern sha=a96e54d12777 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
@row_add n=35 row=contract/cli-verbs/run area=surface-cli rec=fct-zbrdgwj5@1
boundary: CLI verbs
item: viola run <name> -- claude
promise: Passthrough; prints nothing while the child runs.
@row_check n=35 row=contract/cli-verbs/run sha=9be25dc2432a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:188
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=36 row=decision/mcp-server-without-human-surface area=surface-cli rec=fct-lrricjwq@1
name: MCP server with no human surface
rule: viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
@row_check n=36 row=decision/mcp-server-without-human-surface sha=fca283f1839d v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:95-96
quote: #[command(hide = true)] Mcp,
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=37 row=contract/mcp-tools/send area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: send
promise: A driver's machine view of the viola send verb.
@row_check n=37 row=contract/mcp-tools/send sha=1c1c9ecde239 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:251
quote: ToolName::Send => send(ctx, &args).await,
@row_add n=38 row=contract/mcp-tools/wait area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: wait
promise: A driver's machine view of the viola wait verb.
@row_check n=38 row=contract/mcp-tools/wait sha=46131eb7fd13 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:252
quote: ToolName::Wait => wait(ctx, &args).await,
@row_add n=39 row=contract/mcp-tools/last area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: last
promise: A driver's machine view of the viola last verb.
@row_check n=39 row=contract/mcp-tools/last sha=31f9204222f1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:253
quote: ToolName::Last => last(ctx, &args).await,
@row_add n=40 row=contract/mcp-tools/answer area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: answer
promise: A driver's machine view of the viola answer verb.
@row_check n=40 row=contract/mcp-tools/answer sha=4936c389956e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:254
quote: ToolName::Answer => answer(ctx, &args).await,
@row_add n=41 row=contract/mcp-tools/list area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: list
promise: A driver's machine view of the viola list verb.
@row_check n=41 row=contract/mcp-tools/list sha=29f9feec3628 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:255
quote: ToolName::List => list(ctx).await,
@row_add n=42 row=contract/mcp-tool-results/refusal area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tool results
item: refusal
promise: Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
@row_check n=42 row=contract/mcp-tool-results/refusal sha=6e6f5f3dbf1a v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: crates/viola-mcp/src/result.rs:40-41
quote: let mut result = CallToolResult::structured_error(body); result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=43 row=contract/mcp-tool-results/invalid-params area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tool results
item: invalid-params
promise: Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`.
@row_check n=43 row=contract/mcp-tool-results/invalid-params sha=47cb1daa9635 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
@row_add n=44 row=decision/mcp-hides-wheel-and-handoff-verbs area=surface-cli rec=fct-lrricjwq@1
name: MCP hides wheel and handoff verbs
rule: No viola mcp tool, description or result names release, pause, link or unlink.
@row_check n=44 row=decision/mcp-hides-wheel-and-handoff-verbs sha=4ac49baea998 v=holds judge=sonnet head=8834195b0683
searched: release|pause|link|unlink (case-insensitive) · in crates/viola-mcp/src (hits only comments at tools.rs:36, lib.rs:3, result.rs:23 and tests asserting absence at tools.rs:483, 652)
searched: TOOL_NAMES · in crates/viola-mcp/src/tools.rs (five names, none of the four)
@row_add n=45 row=contract/cli-verbs/revive area=surface-cli rec=fct-k7kme3xv@1
boundary: CLI verbs
item: viola revive <name>
promise: Resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like viola run.
@row_check n=45 row=contract/cli-verbs/revive sha=b8059616313b v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:250
quote: None => ids.next_back().ok_or(Refusal::NoSession),
at: src/cmd/revive.rs:224
quote: |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=46 row=contract/cli-verbs/revive-list area=surface-cli rec=fct-k7kme3xv@1
boundary: CLI verbs
item: viola revive <name> --list
promise: Prints the instance's logged sessions, one static line each.
@row_check n=46 row=contract/cli-verbs/revive-list sha=8d6a1e31d01c v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:294-296
quote: for row in &rows { human::result(&row.line()); }
@row_add n=47 row=contract/cli-verbs/plugin-install area=surface-cli rec=fct-5lv3otbb@1
boundary: CLI verbs
item: viola plugin install
promise: Writes an MCP-only local marketplace under `<home>/marketplace/` with no hooks, prints the two `claude plugin` commands that add it, and spawns no claude.
@row_check n=47 row=contract/cli-verbs/plugin-install sha=68267c68b606 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:115
quote: for (rel, content) in marketplace::marketplace_files(&fwd(&exe), &fwd(&home)) {
at: src/human/verbs.rs:97-99
quote: "viola@viola  written  {dir}\n", "run these to add it to Claude Code:\n", "  claude plugin marketplace add {dir}\n",
at: crates/viola-agent-claude/src/marketplace.rs:44
quote: "mcpServers": {
@row_add n=48 row=contract/cli-verbs/help area=surface-cli rec=fct-pgyxchhq@1
boundary: CLI verbs
item: viola --help
promise: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `handoff:` holding link, unlink and `setup:` holding plugin install.
@row_check n=48 row=contract/cli-verbs/help sha=c974037e5a6d v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
@row_add n=49 row=decision/board-ordered-by-name area=surface-cli rec=fct-mzvyqqgp@1
name: Board ordered by name
rule: viola list rows are ordered by name and nothing is re-sorted by state; NAME values are bold.
@row_check n=49 row=decision/board-ordered-by-name sha=65312f1844ad v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:95
quote: .sort_by(|(a, _), (b, _)| AsRef::<str>::as_ref(a).cmp(AsRef::<str>::as_ref(b)));
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
@row_add n=50 row=contract/viola-list-board/gate-closed area=surface-cli rec=fct-g6mmpt3p@1
boundary: viola list board
item: closed budget gate
promise: A closed budget gate prints `budget-paused` in place of `gate open`.
@row_check n=50 row=contract/viola-list-board/gate-closed sha=a5fe9517a38f v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=51 row=contract/viola-list-board/window-expired area=surface-cli rec=fct-g6mmpt3p@1
boundary: viola list board
item: passed window
promise: A passed budget window prints with `expired` after its figure, as `5H 62 % expired`.
@row_check n=51 row=contract/viola-list-board/window-expired sha=0ba204db1a45 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:26-27
quote: let suffix = if expired { " expired" } else { "" }; format!("{label} {:.0} %{suffix}", used.floor())
@row_add n=52 row=contract/viola-list-board/stale-row-with-dialog area=surface-cli rec=fct-3lloyyhg@1,fct-mdoqfj5o@2
boundary: viola list board
item: stale row with a dialog
promise: On a cocked and stale row, the DIALOG word stays amber and not dim, while the kind word and the rest of the row stay dim.
@row_check n=52 row=contract/viola-list-board/stale-row-with-dialog sha=4920f0ae8ca4 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:85-86
quote: let undim = if spans.stale { tokens.reset } else { "" }; format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=53 row=contract/viola-list-board/empty-board area=surface-cli rec=fct-lnegijti@1
boundary: viola list board
item: empty board
promise: An empty board prints `no wrapped sessions  start one: viola run <name> -- claude`.
@row_check n=53 row=contract/viola-list-board/empty-board sha=c0d010ff4f58 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:78
quote: const EMPTY_BOARD: &str = "no wrapped sessions  start one: viola run <name> -- claude";
@row_add n=54 row=decision/words-carry-every-state area=surface-cli rec=fct-i5xtjrqd@1,fct-5ius22jn@1
name: Words carry every state
rule: Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
@row_check n=54 row=decision/words-carry-every-state sha=d31510500e0b v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:66
quote: || !facts.stdout_tty
@row_add n=55 row=contract/viola-list-board/unknown-width area=surface-cli rec=fct-i5xtjrqd@1,fct-mdoqfj5o@2
boundary: viola list board
item: unknown terminal width
promise: Nothing is truncated when the terminal width is unknown.
@row_check n=55 row=contract/viola-list-board/unknown-width sha=5bd656fa7b35 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:35-38
quote: let unwrapped = match width { Some(width) => longest(false).min(width.saturating_sub(UNWRAPPED_REST)), None => longest(false), };
@row_add n=56 row=contract/viola-list-board/single-write area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: write order
promise: viola list prints as one stdout write: the BAY line, the caption row, then one row per wrapped session in name order (or the empty line), then the unwrapped rack when there is one.
@row_check n=56 row=contract/viola-list-board/single-write sha=68fbc0ad464c v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:105
quote: out.write_all(text.as_bytes())
@row_add n=57 row=contract/viola-list-board/uncoloured-modes area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: uncoloured modes
promise: The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
@row_check n=57 row=contract/viola-list-board/uncoloured-modes sha=a980311486fa v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62-69
quote: if facts.json || facts.passthrough || facts.no_color || facts.term.as_deref() == Some("dumb") || !facts.stdout_tty { return None; }
@row_add n=58 row=contract/viola-list-board/colour-depth area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: colour depth
promise: On a terminal that takes colour, the depth is truecolor, 256 or 16, read from COLORTERM / TERM.
@row_check n=58 row=contract/viola-list-board/colour-depth sha=8649e6d81837 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:70-71
quote: if matches!(facts.colorterm.as_deref(), Some("truecolor" | "24bit")) { Some(Depth::Truecolor)
at: src/human/style.rs:75-79
quote: .is_some_and(|term| term.contains("256color")) { Some(Depth::Ansi256) } else { Some(Depth::Ansi16)
@row_add n=59 row=contract/viola-list-board/stale-row-dim area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: stale row
promise: A wrapped stale row is dim from its first byte to its last, NAME dim and bold.
@row_check n=59 row=contract/viola-list-board/stale-row-dim sha=c83648f218db v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
at: src/human/board/cells.rs:90
quote: let close = if spans.stale { tokens.reset } else { "" };
@row_add n=60 row=contract/viola-list-board/plain-lines area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: plain lines
promise: The BAY line, caption, empty line and separator print plain.
@row_check n=60 row=contract/viola-list-board/plain-lines sha=c6cc178127fe v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:94
quote: let mut lines = vec![bay_line(budget, &board.skipped, now)];
at: src/human/board/rows.rs:97
quote: lines.push(EMPTY_BOARD.to_owned());
at: src/human/board/rows.rs:100
quote: lines.push(UNWRAPPED_SEPARATOR.to_owned());
at: src/human/board/rows.rs:40-44
quote: let mut rows = vec![board_line( column, [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"], None, )];
@row_add n=61 row=contract/viola-list-board/status-join area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: STATUS
promise: One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
@row_check n=61 row=contract/viola-list-board/status-join sha=e715f4ab9a3f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:333-334
quote: if let Some(at) = observations.iter().position(|o| o.joins(&item.join)) { item.status = observations[at].status;
at: crates/viola-state/src/board.rs:161
quote: by_pid || keys.agent_session_id.as_deref() == Some(self.session_id.as_str())
@row_add n=62 row=contract/viola-list-board/agents-read-failure area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: failed agents read
promise: A failed, oversize or malformed `claude agents --json` read prints the board as with no claude.
@row_check n=62 row=contract/viola-list-board/agents-read-failure sha=7e2e89388bb7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:121
quote: if let Ok(list) = std::env::current_dir()
at: src/cmd/list.rs:126
quote: board::join(&mut board, observations(list));
at: src/run/agents.rs:87-88
quote: AgentsFailure::Oversize => ReadFailure::Oversize, AgentsFailure::Malformed => ReadFailure::Malformed,
@row_add n=63 row=contract/viola-list-board/unwrapped-rack area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: unwrapped rack
promise: Unjoined agent rows print after the wrapped rows under `-- UNWRAPPED - READ-ONLY --`, sorted by name: NAME the escaped sessionId, LIVE live, its STATUS, n/a in WHEEL, DIALOG and CLI; with no unwrapped row no separator prints.
@row_check n=63 row=contract/viola-list-board/unwrapped-rack sha=f4bbf894b483 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:346
quote: unwrapped.sort_by(|a, b| a.name.cmp(&b.name));
at: src/human/board/rows.rs:49-50
quote: let na = || NOT_APPLICABLE.to_owned(); (na(), na(), na())
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
at: crates/viola-state/src/board.rs:195
quote: liveness: RowLiveness::Live,
at: src/human/board/rows.rs:99-100
quote: if wrapped < board.items.len() { lines.push(UNWRAPPED_SEPARATOR.to_owned());
@row_add n=64 row=contract/viola-list-board/column-widths area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: column widths
promise: NAME is padded to the longer of 4 and the longest name over both racks in characters, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space.
@row_check n=64 row=contract/viola-list-board/column-widths sha=d342a6f23486 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:10-13
quote: const LIVE_WIDTH: usize = 5; const STATUS_WIDTH: usize = 7; const WHEEL_WIDTH: usize = 23; const DIALOG_WIDTH: usize = 17;
at: src/human/board/cells.rs:95
quote: "{open}{shown_name}{}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \
at: src/human/board/rows.rs:39
quote: let column = longest(true).max(unwrapped).max(NAME_CAPTION.len());
@row_add n=65 row=contract/viola-list-board/name-truncation area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: NAME truncation
promise: With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.
@row_check n=65 row=contract/viola-list-board/name-truncation sha=f3296d15cb23 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:63
quote: let width = viola_pty::host_size().map(|size| usize::from(size.cols));
at: src/human/board/cells.rs:109-110
quote: pub(super) const UNWRAPPED_REST: usize = LIVE_WIDTH + STATUS_WIDTH + WHEEL_WIDTH + DIALOG_WIDTH + 5 * 2 + NOT_APPLICABLE.len();
at: src/human/board/rows.rs:52-53
quote: let name = if !item.wrapped && name.chars().count() > column { cut(&name, column)
at: src/human/board/cells.rs:127-130
quote: let mut keep = column.saturating_sub(ELLIPSIS.len()); if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) { keep = at; }
@row_add n=66 row=contract/viola-list-board/wheel-unknown area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: WHEEL unknown
promise: A replayed row with no logged holder reads WHEEL `unknown`.
@row_check n=66 row=contract/viola-list-board/wheel-unknown sha=61ef52901f1f v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:16
quote: let holder = item.wheel.map_or("unknown", Wheel::as_str);
at: crates/viola-state/src/board.rs:292
quote: wheel: state.wheel,
@row_add n=67 row=contract/viola-list-board/cli-unknown area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: CLI unknown
promise: A row with no recorded version reads CLI `unknown`.
@row_check n=67 row=contract/viola-list-board/cli-unknown sha=9c3c6a91b8bf v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:47
quote: None => "unknown".to_owned(),
@row_add n=68 row=contract/viola-list-board/dialog-unknown area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: DIALOG unknown
promise: A pending dialog of unknown kind reads `DIALOG unknown`.
@row_check n=68 row=contract/viola-list-board/dialog-unknown sha=5dcb73f0a9d7 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:33
quote: _ => format!("{DIALOG_WORD} unknown"),
@row_add n=69 row=decision/board-planning-forms-wording area=surface-cli rec=fct-mdoqfj5o@2
name: Board planning forms as the ratified wording
rule: The board's planning forms (a figure rounded down, the whole-minute age and `read unknown`, bare skip counts, `human - budget-paused`, the empty board's three lines) are its wording.
why: confirmed as built by the founder, superseding the operator's word
@row_check n=69 row=decision/board-planning-forms-wording sha=e988e755c432 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:27
quote: format!("{label} {:.0} %{suffix}", used.floor())
at: src/human/board/bay.rs:38-39
quote: Some(at) => format!("read {}m ago", (now - at.with_timezone(&Utc)).num_minutes()), None => "read unknown".to_owned(),
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
at: src/human/board/rows.rs:78
quote: const EMPTY_BOARD: &str = "no wrapped sessions  start one: viola run <name> -- claude";
@row_add n=70 row=contract/viola-list-json/document area=surface-cli rec=fct-yt73cavp@1
boundary: viola list --json
item: document
promise: One JSON document on stdout with no header line, no colour and no glyph; its item, budget and skipped shapes are the list payload contract.
@row_check n=70 row=contract/viola-list-json/document sha=83200b0dab3b v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:55
quote: client::document(&document(&board, budget.as_ref()));
@row_add n=71 row=contract/viola-list-json/strict-modes-failed area=surface-cli rec=fct-yt73cavp@1
boundary: viola list --json
item: strict-modes-failed
promise: A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
@row_check n=71 row=contract/viola-list-json/strict-modes-failed sha=ea22fcd7aa78 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:94
quote: json!({"v": 1, "error": "instance-unreachable", "detail": cause})
at: src/cmd/list.rs:47
quote: return Ok(ExitCode::from(21));
@row_add n=72 row=contract/viola-send-output/input-not-ready-hint area=surface-cli rec=fct-xzvoaizl@1
boundary: viola send output
item: input-not-ready hint
promise: An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
@row_check n=72 row=contract/viola-send-output/input-not-ready-hint sha=10aafdcb2d41 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:42
quote: "{name} was not ready for input; send again, and if it repeats a human must look at the session"
at: src/human/mirror.rs:46
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=73 row=contract/viola-send-output/json-ok area=surface-cli rec=fct-xzvoaizl@1
boundary: viola send output
item: --json ok
promise: Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
@row_check n=73 row=contract/viola-send-output/json-ok sha=deee6d3fb24d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: src/cmd/send.rs:110
quote: return client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/send.rs:97
quote: !json && terminal
@row_add n=74 row=contract/viola-send-output/word-column area=surface-cli rec=fct-resu4xix@1
boundary: viola send output
item: word column
promise: The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
@row_check n=74 row=contract/viola-send-output/word-column sha=7935ad2fbd43 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:9
quote: format!("{boxed} {word:<MIRROR_WORD$}  {name}  {rest}\n")
at: src/human/mirror.rs:6
quote: const MIRROR_WORD: usize = 13;
@row_add n=75 row=decision/send-lines-appended-never-redrawn area=surface-cli rec=fct-resu4xix@1
name: Send lines appended, never redrawn
rule: viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it.
@row_check n=75 row=decision/send-lines-appended-never-redrawn sha=18b0aa7f7c95 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:104
quote: let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
at: src/cmd/send.rs:114
quote: let _ = human::write_read_back(
@row_add n=76 row=contract/viola-send-output/piped-stdout area=surface-cli rec=fct-resu4xix@1
boundary: viola send output
item: piped stdout
promise: When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
@row_check n=76 row=contract/viola-send-output/piped-stdout sha=2da20278e6da v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:97
quote: !json && terminal
at: src/cmd/send.rs:102
quote: if issue_line_shows(self.json, io::stdout().is_terminal()) {
@row_add n=77 row=contract/viola-wait-output/turn-ended area=surface-cli rec=fct-r2nmoonb@1
boundary: viola wait output
item: turn-ended
promise: `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.
@row_check n=77 row=contract/viola-wait-output/turn-ended sha=4d5374771730 v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:20
quote: let line = format!("{kind}  {name}  {at}  cursor {cursor}\n");
at: src/cmd/wait.rs:36
quote: const DIALOG_KINDS: [&str; 4] = ["question", "permission", "plan", "dialog-expired"];
@row_add n=78 row=contract/viola-wait-output/dialog area=surface-cli rec=fct-r2nmoonb@1
boundary: viola wait output
item: dialog
promise: A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board.
@row_check n=78 row=contract/viola-wait-output/dialog sha=3419e7fdf8e8 v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:32
quote: let line = format!("{kind}  {name}  dialog {dialog}  cursor {cursor}\n");
@row_add n=79 row=contract/viola-wait-output/timed-out area=surface-cli rec=fct-r2nmoonb@1
boundary: viola wait output
item: timed out
promise: A timeout prints `timed out  <name>  <ms> ms` on stdout.
@row_check n=79 row=contract/viola-wait-output/timed-out sha=b08f2e2049fe v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:38
quote: out.write_all(format!("timed out  {name}  {ms} ms\n").as_bytes())
@row_add n=80 row=contract/viola-last-output/message area=surface-cli rec=fct-r2nmoonb@1
boundary: viola last output
item: message
promise: The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr.
@row_check n=80 row=contract/viola-last-output/message sha=0492805651b6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/last.rs:73-74
quote: if let Some(ts) = ok["ts"].as_str() && io::stderr().is_terminal()
at: src/cmd/last.rs:83
quote: let _ = human::write_no_message(&mut io::stderr().lock());
@row_add n=81 row=contract/cli-refusals/instance-unreachable area=surface-cli rec=fct-r2nmoonb@1
boundary: CLI refusals
item: instance-unreachable
promise: With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
@row_check n=81 row=contract/cli-refusals/instance-unreachable sha=cff495802a95 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/cmd/client.rs:265
quote: 21
@row_add n=82 row=decision/no-spinner area=surface-cli rec=fct-ujoiukvr@1
name: No spinner
rule: The cli shows no spinner and no elapsed-time counter.
@row_check n=82 row=decision/no-spinner sha=a0fe82aea46f v=holds judge=sonnet head=8834195b0683
searched: Sending|✓|spinner|Spinner|indicatif · in src/**/*.rs
searched: elapsed (only duration_ms log fields, no printed counter) · in src/**/*.rs
@row_add n=83 row=contract/viola-wait-output/waiting-line area=surface-cli rec=fct-ujoiukvr@1,fct-r2nmoonb@1
boundary: viola wait output
item: waiting line
promise: `waiting: <name>` is printed once, on stderr, only when stderr is a terminal and --json is not set.
@row_check n=83 row=contract/viola-wait-output/waiting-line sha=abce5f3765ce v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:69
quote: if !args.json && io::stderr().is_terminal() {
at: src/human/wait.rs:9
quote: out.write_all(format!("waiting: {name}\n").as_bytes())
@row_add n=84 row=contract/viola-wait-output/dialog-unknown area=surface-cli rec=fct-ujoiukvr@1
boundary: viola wait output
item: dialog unknown
promise: A dialog line whose event carries no dialog_id reads `dialog unknown`.
@row_check n=84 row=contract/viola-wait-output/dialog-unknown sha=bddd83c46f44 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:111
quote: .map_or_else(|| "unknown".to_owned(), |id| id.to_string());
@row_add n=85 row=contract/viola-wait-output/dialog-expired area=surface-cli rec=fct-ujoiukvr@1,fct-r2nmoonb@1
boundary: viola wait output
item: dialog-expired
promise: A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.
@row_check n=85 row=contract/viola-wait-output/dialog-expired sha=ffe1b4069c1d v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:36
quote: const DIALOG_KINDS: [&str; 4] = ["question", "permission", "plan", "dialog-expired"];
at: src/human/wait.rs:32
quote: let line = format!("{kind}  {name}  dialog {dialog}  cursor {cursor}\n");
@row_add n=86 row=contract/cli-json-mode/one-document area=surface-cli rec=fct-ujoiukvr@1
boundary: CLI --json mode
item: one document
promise: Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
@row_check n=86 row=contract/cli-json-mode/one-document sha=6513dc07b676 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:221
quote: let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
@row_add n=87 row=contract/cli-internal-error/human area=surface-cli rec=fct-ujoiukvr@1
boundary: CLI internal error
item: human output
promise: A failure or a caught panic prints `error: internal error` once, exit 1, with no hint.
@row_check n=87 row=contract/cli-internal-error/human sha=64927f0152f6 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=88 row=contract/cli-internal-error/json area=surface-cli rec=fct-ujoiukvr@1
boundary: CLI internal error
item: --json output
promise: Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
@row_check n=88 row=contract/cli-internal-error/json sha=c91545630d4b v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:101
quote: client::refused_document("internal-error");
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=89 row=contract/wheel-verb-output/pause area=surface-cli rec=fct-dryeax7x@1
boundary: wheel verb output
item: viola pause
promise: Prints `<name>  wheel human  manual-pause  I have control`.
@row_check n=89 row=contract/wheel-verb-output/pause sha=597c774c6058 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:17
quote: out.write_all(format!("{name}  wheel human  manual-pause  I have control\n").as_bytes())
@row_add n=90 row=contract/wheel-verb-output/release area=surface-cli rec=fct-dryeax7x@1
boundary: wheel verb output
item: viola release
promise: Prints `<name>  wheel driver  you have control`.
@row_check n=90 row=contract/wheel-verb-output/release sha=3a415534f8df v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:22
quote: out.write_all(format!("{name}  wheel {holder}  you have control\n").as_bytes())
@row_add n=91 row=contract/viola-answer-output/answered area=surface-cli rec=fct-dryeax7x@1
boundary: viola answer output
item: answered
promise: Prints `answered  <name>  dialog <id>`.
@row_check n=91 row=contract/viola-answer-output/answered sha=552293d0df0f v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:12
quote: out.write_all(format!("answered  {name}  dialog {dialog_id}\n").as_bytes())
@row_add n=92 row=contract/viola-answer-output/unknown-dialog area=surface-cli rec=fct-dryeax7x@1
boundary: viola answer output
item: unknown-dialog
promise: An answer to a dialog that is not pending prints `unable  <name>  not-delivered  unknown-dialog` on stderr, exit 13, then `hint: that dialog is not pending; viola list shows the current DIALOG`.
@row_check n=92 row=contract/viola-answer-output/unknown-dialog sha=c9fc3c8e2f05 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
at: src/human/hints.rs:14
quote: Some("that dialog is not pending; viola list shows the current DIALOG")
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
at: src/cmd/answer.rs:81
quote: let _ = human::write_unable(&mut io::stderr().lock(), self.name, &shown, hint);
@row_add n=93 row=contract/release-budget-output/gate-lifted area=surface-cli rec=fct-fkpva2qn@1
boundary: viola release --budget output
item: gate lifted
promise: When a pause stood, prints `<name>  budget gate lifted until <end>  wheel <holder>` on stdout, exit 0, wheel left where it is; `<end>` is `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` otherwise, a non-RFC 3339 value escaped.
@row_check n=93 row=contract/release-budget-output/gate-lifted sha=68d0fd740be3 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:49
quote: out.write_all(format!("{name}  budget gate lifted until {end}  wheel {holder}\n").as_bytes())
at: src/human/verbs.rs:34
quote: "%H:%M:%SZ"
at: src/human/verbs.rs:36
quote: "%Y-%m-%d %H:%M:%SZ"
at: src/human/verbs.rs:30
quote: return escape_message(override_until);
@row_add n=94 row=contract/release-budget-output/gate-open area=surface-cli rec=fct-fkpva2qn@1
boundary: viola release --budget output
item: gate open
promise: When no pause stood, prints `<name>  budget gate open  wheel <holder>` on stdout, exit 0, with the wheel left where it is.
@row_check n=94 row=contract/release-budget-output/gate-open sha=7d91fae5dee7 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:54
quote: out.write_all(format!("{name}  budget gate open  wheel {holder}\n").as_bytes())
@row_add n=95 row=contract/link-output/link area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: link
promise: Prints one stdout line in one write, plain in every mode, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the reply's since as UTC clock time in whole seconds, a non-RFC 3339 since printed escaped.
@row_check n=95 row=contract/link-output/link sha=4f64ceec8d09 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/verbs.rs:86
quote: Ok(at) => at.with_timezone(&Utc).format("%H:%M:%SZ").to_string(),
at: src/human/verbs.rs:87
quote: Err(_) => escape_message(since),
@row_add n=96 row=contract/link-output/already-linked area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: already linked
promise: A pair already held prints `link    <driver> -> <driven>  already linked`, exit 0.
@row_check n=96 row=contract/link-output/already-linked sha=d096fbd41c21 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:74
quote: out.write_all(format!("link    {driver} -> {driven}  already linked\n").as_bytes())
@row_add n=97 row=contract/link-output/unlink area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: unlink
promise: Prints `unlink  <driver> -> <driven>` in one write, exit 0, whether or not a pair was removed.
@row_check n=97 row=contract/link-output/unlink sha=e5c658d34d91 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:79
quote: out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
at: src/cmd/link.rs:75
quote: (Verb::Unlink, _) => human::write_unlinked(out, driver, driven),
@row_add n=98 row=contract/link-output/printed-names area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: printed names
promise: The names printed are the verb's own arguments, never names read back from the reply.
@row_check n=98 row=contract/link-output/printed-names sha=ce149f22a97a v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:93
quote: let (driver, driven): (&str, &str) = (args.driver.as_ref(), args.driven.as_ref());
at: src/cmd/link.rs:123
quote: let _ = write_line(&mut io::stdout().lock(), verb, driver, driven, &ok);
@row_add n=99 row=contract/link-output/json-link area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: link --json
promise: Prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`.
@row_check n=99 row=contract/link-output/json-link sha=d07f15f97f35 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:121
quote: client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/link.rs:48
quote: Verb::Link => ok["since"].is_string() && ok["already_linked"].is_boolean(),
@row_add n=100 row=contract/link-output/json-unlink area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: unlink --json
promise: Prints `{"v":1,"ok":{}}`.
@row_check n=100 row=contract/link-output/json-unlink sha=7a7fa13ab202 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:121
quote: client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/link.rs:49
quote: Verb::Unlink => ok.is_object(),
@row_add n=101 row=contract/link-output/wrapper-error area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: wrapper error
promise: A wrapper error, a driver equal to driven (-32602) included, is the wrapper-fault line, exit 20.
@row_check n=101 row=contract/link-output/wrapper-error sha=1bba318cafca v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:116
quote: Err(detail) => return Ok(client::fault(args.json, &detail)),
at: src/cmd/client.rs:245
quote: 20
@row_add n=102 row=contract/viola-verify-output/failing-row area=surface-cli rec=fct-hk2gtsxu@2,fct-pf5uzbca@2
boundary: viola verify output
item: failing row
promise: A run with a failing row exits 1 and still writes the stamp.
@row_check n=102 row=contract/viola-verify-output/failing-row sha=c47171831115 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
at: src/cmd/verify.rs:297-298
quote: if failed > 0 { return Ok(Ended::Exit(1));
@row_add n=103 row=contract/viola-verify-output/usage area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: usage
promise: `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude.
@row_check n=103 row=contract/viola-verify-output/usage sha=718787d4672a v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:49
quote: record: Option<PathBuf>,
at: src/cmd/verify.rs:52
quote: program: Vec<OsString>,
at: src/cmd/verify.rs:146
quote: None => (OsString::from(PROGRAM), Vec::new()),
@row_add n=104 row=contract/viola-verify-output/step-line area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: step line
promise: One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
@row_check n=104 row=contract/viola-verify-output/step-line sha=d19211ae9d3b v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
at: src/cmd/verify.rs:486
quote: human::result(&step_line(n, total, row, pass));
@row_add n=105 row=contract/viola-verify-output/summary area=surface-cli rec=fct-pf5uzbca@2,fct-hk2gtsxu@2
boundary: viola verify output
item: summary
promise: `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
@row_check n=105 row=contract/viola-verify-output/summary sha=157b07f51b20 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:293-296
quote: human::result(&format!( "stamped {version}  {} pass  {failed} fail", results.len() - failed ));
@row_add n=106 row=contract/viola-verify-output/exit-0 area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: exit 0
promise: Exit 0 when every relied row passes.
@row_check n=106 row=contract/viola-verify-output/exit-0 sha=e8262cf8418d v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:302
quote: None => Ok(Ended::Exit(0)),
@row_add n=107 row=contract/viola-verify-output/exit-1 area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: exit 1
promise: Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
@row_check n=107 row=contract/viola-verify-output/exit-1 sha=b620bbf099ce v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:298
quote: return Ok(Ended::Exit(1));
at: src/cmd/verify.rs:81
quote: Ended::Refused(detail) => (1, Some(detail)),
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
@row_add n=108 row=contract/viola-verify-output/stop area=surface-cli rec=fct-pf5uzbca@2,fct-hk2gtsxu@2
boundary: viola verify output
item: stop
promise: A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
@row_check n=108 row=contract/viola-verify-output/stop sha=89358cd91252 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:83-86
quote: human::refuse( "verify was stopped before it finished", "run viola verify again to stamp this CLI", );
at: src/cmd/verify/stop.rs:12
quote: for signal in [libc::SIGINT, libc::SIGTERM] {
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
@row_add n=109 row=contract/viola-verify-output/record area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: --record
promise: --record adds no human line.
@row_check n=109 row=contract/viola-verify-output/record sha=e5a478a19274 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:526-527
quote: write_recording(&dir.join(version), files)?; Ok(Ended::Exit(0))
@row_add n=110 row=contract/viola-verify-output/plain-ascii area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: plain ASCII
promise: All viola verify output is plain ASCII with no colour.
@row_check n=110 row=contract/viola-verify-output/plain-ascii sha=9b1fd7e87496 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:59
quote: out.write_all(format!("{line}\n").as_bytes())
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
@row_add n=111 row=contract/viola-verify-output/help-paragraph area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: --help
promise: viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
@row_check n=111 row=contract/viola-verify-output/help-paragraph sha=fa00bada53a4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:85-87
quote: /// Run it from a folder you trust in Claude Code: the input-box probe runs in a subfolder there, /// which inherits that trust. An unapproved external CLAUDE.md import blocks the probe: its /// dialog show
@row_add n=112 row=contract/viola-help/usage-line area=surface-cli rec=fct-dljdvvyu@1,fct-4rbi2th6@2
boundary: viola --help output
item: usage line
promise: `viola --help` and `viola help` print exactly `Usage: viola [--home <dir>] <verb> [args]` as the first line, then a blank line.
@row_check n=112 row=contract/viola-help/usage-line sha=e1c3c9aea6d7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:48
quote: override_usage = "viola [--home <dir>] <verb> [args]",
at: src/cmd/mod.rs:32
quote: {usage-heading} {usage}
@row_add n=113 row=contract/viola-help/verb-groups area=surface-cli rec=fct-dljdvvyu@1,fct-4rbi2th6@2,fct-xpsr3izn@1
boundary: viola --help output
item: verb group rows
promise: Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list.
@row_check n=113 row=contract/viola-help/verb-groups sha=87b007c3c0da v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
at: src/cmd/mod.rs:40-41
quote: Options: {options}";
@row_add n=114 row=decision/mcp-verb-hidden area=surface-cli rec=fct-4rbi2th6@2
name: viola mcp is a hidden verb
rule: `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
@row_check n=114 row=decision/mcp-verb-hidden sha=d729ccbceb54 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:95-96
quote: #[command(hide = true)] Mcp,
@row_add n=115 row=contract/viola-verb-help/usage area=surface-cli rec=fct-33x2qanb@2
boundary: viola <verb> --help output
item: per-verb usage
promise: `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
@row_check n=115 row=contract/viola-verb-help/usage sha=0e4e7fa1b479 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:45
quote: name = "viola",
@row_add n=116 row=decision/prompt-text-from-stdin-or-file area=surface-cli rec=fct-33x2qanb@2
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or `--file`, never from a leading-slash argument, and the help of `send` says so.
@row_check n=116 row=decision/prompt-text-from-stdin-or-file sha=e3d89134b21a v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:48
quote: Err("the text is read from stdin or --file, never from an argument".to_owned())
at: src/cmd/mod.rs:65
quote: /// Type a prompt into the named instance and read it back; the text comes from stdin or --file
@row_add n=117 row=contract/viola-run/start-output area=surface-cli rec=fct-5cewxahg@1,fct-igaudjuy@1
boundary: viola run start
item: passed start output
promise: `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits.
@row_check n=117 row=contract/viola-run/start-output sha=f8ea8709dda6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
@row_add n=118 row=contract/viola-run/already-live area=surface-cli rec=fct-5cewxahg@1
boundary: viola run start refusals
item: already live
promise: Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
@row_check n=118 row=contract/viola-run/already-live sha=5a6f131253c8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:716
quote: human::refuse(&format!("{name} is already live"), "viola list");
at: src/cmd/run.rs:413
quote: Started::Refused(ExitCode::from(1))
@row_add n=119 row=contract/viola-revive/usage area=surface-cli rec=fct-dahc6e7w@1
boundary: viola revive
item: usage
promise: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` or `viola revive <name> --list`, each with `[--json]`; `--list` conflicts with `--id`, `--fork` and the trailing arguments.
@row_check n=119 row=contract/viola-revive/usage sha=f824432dd369 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:36
quote: #[arg(long, conflicts_with_all = ["id", "fork", "extra"])]
@row_add n=120 row=contract/viola-revive/start-output area=surface-cli rec=fct-dahc6e7w@1,fct-n7x2i5oo@1
boundary: viola revive
item: passed start output
promise: A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
@row_check n=120 row=contract/viola-revive/start-output sha=01d308b8e1b5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=121 row=contract/viola-revive/list-row area=surface-cli rec=fct-dahc6e7w@1,fct-n7x2i5oo@1
boundary: viola revive
item: --list row
promise: `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
@row_check n=121 row=contract/viola-revive/list-row sha=cefbab644a2c v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:313
quote: format!("{}  {}  {}", self.ts, self.cause, self.id)
@row_add n=122 row=contract/viola-revive/list-cause area=surface-cli rec=fct-dahc6e7w@1
boundary: viola revive
item: --list cause field
promise: The cause in a `viola revive --list` row is `startup`, `clear`, `resume` or `compact` as logged, and `unknown` otherwise.
@row_check n=122 row=contract/viola-revive/list-cause sha=e72f7788f969 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:332-337
quote: match cause { "startup" => "startup", "clear" => "clear", "resume" => "resume", "compact" => "compact", _ => "unknown",
@row_add n=123 row=contract/viola-revive/refusal-shape area=surface-cli rec=fct-dahc6e7w@1
boundary: viola revive refusals
item: refusal shape
promise: Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
@row_check n=123 row=contract/viola-revive/refusal-shape sha=d67e5e86ef48 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:109-110
quote: let (unable, hint) = self.pair(name.as_ref()); human::refuse(&unable, &hint);
@row_add n=124 row=contract/viola-revive-refusals/state-files-writable area=surface-cli rec=fct-lu5j3ng5@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: state files writable by another user
promise: `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
@row_check n=124 row=contract/viola-revive-refusals/state-files-writable sha=a8a780f859f6 v=holds judge=sonnet head=8834195b0683
at: src/human/board/pairs.rs:6-7
quote: format!("{name}'s state files can be written by another user"), "viola will not read them; make the viola home and its files owner-only".to_owned(),
@row_add n=125 row=contract/viola-revive-refusals/already-live area=surface-cli rec=fct-snlrx2zh@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: already live
promise: `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
@row_check n=125 row=contract/viola-revive-refusals/already-live sha=106c43b12b2f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:716
quote: human::refuse(&format!("{name} is already live"), "viola list");
@row_add n=126 row=contract/viola-revive-refusals/no-logged-session area=surface-cli rec=fct-qz5yofop@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: no logged session
promise: `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`.
@row_check n=126 row=contract/viola-revive-refusals/no-logged-session sha=af720948cc2c v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:83-84
quote: format!("{name} has no logged session to resume"), format!("start one: viola run {name} -- {PROGRAM}"),
@row_add n=127 row=contract/viola-revive-refusals/no-session-with-id area=surface-cli rec=fct-j4jbg56f@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: no logged session with that id
promise: `unable: <name> has no logged session with that id` / `hint: viola revive <name> --list`.
@row_check n=127 row=contract/viola-revive-refusals/no-session-with-id sha=86531cd1eac4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:87-88
quote: format!("{name} has no logged session with that id"), format!("viola revive {name} --list"),
@row_add n=128 row=contract/viola-revive-refusals/session-open-elsewhere area=surface-cli rec=fct-gkytyqxv@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: session open in another claude
promise: `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`.
@row_check n=128 row=contract/viola-revive-refusals/session-open-elsewhere sha=aa9a03b6fa54 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:91-92
quote: format!("{name}'s session is open in another {PROGRAM}"), format!("close it there first, or start a copy: viola revive {name} --fork"),
@row_add n=129 row=contract/viola-revive-refusals/directory-missing area=surface-cli rec=fct-xrecgcsk@1,fct-n7x2i5oo@1
boundary: viola revive refusals
item: recorded directory missing
promise: `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
@row_check n=129 row=contract/viola-revive-refusals/directory-missing sha=35c9a3452479 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:95-98
quote: format!("{name}'s recorded directory is missing"), format!( "resume it by hand from a directory you choose: viola run {name} -- {PROGRAM} \ {RESUME_FLAG} <id>, ids from viola revive {name} --list"
@row_add n=130 row=decision/no-hint-names-release area=surface-cli rec=fct-bwpa6bmo@1,fct-xpsr3izn@1
name: No hint names viola release
rule: Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`.
why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
@row_check n=130 row=decision/no-hint-names-release sha=eab1439c63e8 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:5
quote: const HUMAN_TYPING_HINT: &str = "the human has the wheel; send again after the human hands it back";
searched: viola release · in src/human/hints.rs (hint tables; no hint string contains it)
@row_add n=131 row=contract/viola-revive/list-refusals area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive refusals
item: --list refusals
promise: `viola revive --list` has two of the refusals: the state-files pair and the no-logged-session pair.
@row_check n=131 row=contract/viola-revive/list-refusals sha=a9fd4c48ac98 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:273
quote: Refusal::StrictModes.write(name, json);
at: src/cmd/revive.rs:283
quote: Refusal::NoSession.write(name, json);
@row_add n=132 row=contract/run-revive-start/home-tree-refused area=surface-cli rec=fct-bwpa6bmo@1,fct-7zli33yx@3
boundary: viola run and viola revive starts
item: home tree refused
promise: Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
@row_check n=132 row=contract/run-revive-start/home-tree-refused sha=32e525370a74 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/human/board/pairs.rs:14-15
quote: "the viola home is not private to you", "the viola home must be readable only by you; viola does not change its permissions",
@row_add n=133 row=contract/viola-revive/preflight-order area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive start
item: preflight order
promise: The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
@row_check n=133 row=contract/viola-revive/preflight-order sha=f30b9a5120df v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:176
quote: if let Some(refusal) = holder() {
at: src/cmd/revive.rs:179
quote: let session = match session()? {
at: src/cmd/revive.rs:183
quote: if let Some(refusal) = session_live(&session) {
at: src/cmd/revive.rs:186
quote: Ok(recorded_dir(&session).map(|dir| (session, dir)))
@row_add n=134 row=contract/viola-revive/session-live-check area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive start
item: session-live check
promise: One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on.
@row_check n=134 row=contract/viola-revive/session-live-check sha=06159eb33855 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
@row_add n=135 row=contract/viola-revive/malformed-id area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive
item: malformed --id
promise: A malformed `--id` is clap's usage error, exit 2.
@row_check n=135 row=contract/viola-revive/malformed-id sha=aa07074426a3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:30
quote: #[arg(long, value_name = "ID", value_parser = parse_session_id)]
@row_add n=136 row=contract/viola-revive-json/refusal-document area=surface-cli rec=fct-i322ygxm@2,fct-un7m7e5w@1
boundary: viola revive --json
item: refusal document
promise: Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
@row_check n=136 row=contract/viola-revive-json/refusal-document sha=b76d1a6ab1ec v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/revive.rs:106-111
quote: if json { client::refused_document(self.detail()); } else { let (unable, hint) = self.pair(name.as_ref()); human::refuse(&unable, &hint); }
@row_add n=137 row=contract/viola-revive-json/strict-modes-failed area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: strict-modes-failed
promise: The code for a refused home tree or instance state files.
@row_check n=137 row=contract/viola-revive-json/strict-modes-failed sha=ba2207bd57f5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
at: src/cmd/revive.rs:138
quote: Refusal::StrictModes.write(&args.name, args.json);
@row_add n=138 row=contract/viola-revive-json/already-live area=surface-cli rec=fct-i322ygxm@2,fct-un7m7e5w@1
boundary: viola revive --json error codes
item: already-live
promise: The code for a live holder or a fresh replayed beat.
@row_check n=138 row=contract/viola-revive-json/already-live sha=0f8785dcb85d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=139 row=contract/viola-revive-json/stale-heartbeat area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: stale-heartbeat
promise: The code for a stale holder.
@row_check n=139 row=contract/viola-revive-json/stale-heartbeat sha=1f099fdbaf21 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=140 row=contract/viola-revive-json/no-session area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: no-session
promise: The code for both no-logged-session pairs: no logged session, and no logged session with that id.
@row_check n=140 row=contract/viola-revive-json/no-session sha=f7c57cf547ee v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=141 row=contract/viola-revive-json/session-live area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: session-live
promise: The code for a session open in another claude.
@row_check n=141 row=contract/viola-revive-json/session-live sha=fd4a83d7b894 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=142 row=contract/viola-revive-json/cwd-missing area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: cwd-missing
promise: The code for a missing recorded directory.
@row_check n=142 row=contract/viola-revive-json/cwd-missing sha=abb5cca65071 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
at: src/cmd/revive.rs:224
quote: |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
@row_add n=143 row=contract/viola-revive-json/list-document area=surface-cli rec=fct-i322ygxm@2,fct-un7m7e5w@1
boundary: viola revive --json
item: --list document
promise: `--list --json` prints the rows as one document `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…}]}}` in log order, fields as the human rows, or the `no-session` document with exit 1.
@row_check n=143 row=contract/viola-revive-json/list-document sha=6b9654b7fd44 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
at: src/cmd/revive.rs:290
quote: .map(|row| json!({"ts": row.ts, "cause": row.cause, "id": row.id}))
at: src/cmd/revive.rs:283
quote: Refusal::NoSession.write(name, json);
@row_add n=144 row=contract/run-revive-start-json/pinned-hash-mismatch area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: pinned-hash-mismatch
promise: After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty.
@row_check n=144 row=contract/run-revive-start-json/pinned-hash-mismatch sha=f54c28e4e259 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
at: src/cmd/run.rs:420
quote: super::client::refused_document(detail);
@row_add n=145 row=contract/run-revive-start-json/squatted-name area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: squatted-name
promise: After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty.
@row_check n=145 row=contract/run-revive-start-json/squatted-name sha=96694861e557 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=146 row=contract/run-revive-start-json/socket-dir-refused area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: socket-dir-refused
promise: After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair.
@row_check n=146 row=contract/run-revive-start-json/socket-dir-refused sha=16ceda913aed v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:301
quote: refuse_socket_dir(name);
@row_add n=147 row=contract/run-revive-start-json/internal-error area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: internal-error
promise: A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
@row_check n=147 row=contract/run-revive-start-json/internal-error sha=6a6aad4b03ff v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:270
quote: Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
at: src/cmd/revive.rs:124
quote: client::refused_document("internal-error");
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=148 row=contract/viola-plugin-install/output area=surface-cli rec=fct-4an42gpj@1,fct-bievnfhg@2
boundary: viola plugin install
item: human output
promise: After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`.
@row_check n=148 row=contract/viola-plugin-install/output sha=506f59e9e424 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:97-100
quote: "viola@viola  written  {dir}\n", "run these to add it to Claude Code:\n", "  claude plugin marketplace add {dir}\n", "  claude plugin install viola@viola\n",
@row_add n=149 row=contract/viola-plugin-install/dir-quoting area=surface-cli rec=fct-bievnfhg@2
boundary: viola plugin install
item: <dir> quoting
promise: `<dir>` is the canonical home joined with `marketplace`, single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`, else printed bare.
@row_check n=149 row=contract/viola-plugin-install/dir-quoting sha=5ce6f260c940 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:112-113
quote: .chars() .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':'));
@row_add n=150 row=contract/viola-plugin-install/json-ok area=surface-cli rec=fct-4an42gpj@1,fct-bievnfhg@2
boundary: viola plugin install
item: --json ok document
promise: Under `--json` it prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, no path.
@row_check n=150 row=contract/viola-plugin-install/json-ok sha=f169fb5af63e v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:67-70
quote: client::document(&json!({ "v": 1, "ok": {"marketplace": marketplace::NAME, "plugin": PLUGIN_REF}, }));
@row_add n=151 row=contract/viola-plugin-install/strict-modes-refused area=surface-cli rec=fct-4an42gpj@1,fct-bievnfhg@2,fct-7zli33yx@3
boundary: viola plugin install
item: strict-modes refusal
promise: A refused home or `marketplace/` tree dir is exit 21 with the home pair on stderr, stdout empty, nothing created, no log opened; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
@row_check n=151 row=contract/viola-plugin-install/strict-modes-refused sha=7080137b697f v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:53-60
quote: if !tree_passes(&home) { if args.json { client::unreachable_document(UnreachableCause::StrictModes); } else { let (unable, hint) = human::home_state_pair(); human::refuse(unable, hint); } return Ok(ExitCode::from(21));
@row_add n=152 row=contract/viola-plugin-install/internal-error area=surface-cli rec=fct-bievnfhg@2
boundary: viola plugin install
item: internal error
promise: A failed exe read, dir creation or file write is `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.
@row_check n=152 row=contract/viola-plugin-install/internal-error sha=37aaa571b2cb v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67-70
quote: if json_asked(std::env::args_os()) { cmd::internal_error_document(); } else { human::internal_error();
@row_add n=153 row=contract/viola-plugin-install/bare-plugin area=surface-cli rec=fct-bievnfhg@2
boundary: viola plugin install
item: bare viola plugin
promise: `viola plugin` alone is a clap usage error, exit 2; clap serves its own `viola plugin help [install]`.
@row_check n=153 row=contract/viola-plugin-install/bare-plugin sha=ba53e7740873 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
@row_add n=154 row=decision/only-header-bay-line area=surface-cli rec=fct-ngdz2kjd@1
name: Only header is the BAY line
rule: The only header on any verb is the one static BAY line at the top of `viola list`.
@row_check n=154 row=decision/only-header-bay-line sha=b4e9a9a48000 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:94
quote: let mut lines = vec![bay_line(budget, &board.skipped, now)];
searched: header|Header · in src/**/*.rs (only a doc comment in bay.rs and no other printed header)
@row_add n=155 row=contract/bay-line/cell-order area=surface-cli rec=fct-bz2eawmt@1
boundary: the BAY line of viola list
item: cell order
promise: `BAY`, `5H <n> %`, `7D <n> %` (each followed by `expired` when the window has passed), `read <age> ago`, `gate open` or `budget-paused`, `skipped a - b - c`, separated by the two-space gutter.
@row_check n=155 row=contract/bay-line/cell-order sha=0ff86d6a747c v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
@row_add n=156 row=decision/bay-line-unstyled area=surface-cli rec=fct-hjvzsdrb@1
name: BAY line unstyled
rule: The BAY line's label words and data figures are all in the default foreground, with no bold, no colour, no box drawing and no rule line under it.
@row_check n=156 row=decision/bay-line-unstyled sha=9060e6c104c7 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
@row_add n=157 row=decision/no-banner area=surface-cli rec=fct-igaudjuy@1
name: No banner anywhere
rule: Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
@row_check n=157 row=decision/no-banner sha=7c643ecbf468 v=holds judge=sonnet head=8834195b0683
searched: banner|tagline|ASCII art · in src/**/*.rs
searched: println!|print!( · in src/**/*.rs (only src/bin/viola-fake-agent.rs, a test fake)
@row_add n=158 row=decision/flat-verbs area=surface-cli rec=fct-ejymuula@2,fct-v22aegyt@1
name: Flat verbs plus a target
rule: Each verb is one lower-case word with no nested noun hierarchy or subcommand tree.
except: `plugin install` is the one two-word verb.
@row_check n=158 row=decision/flat-verbs sha=c0db6cbb41ea v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:59-60
quote: #[derive(Subcommand)] enum Command {
at: src/cmd/plugin.rs:31
quote: enum PluginVerb {
at: src/cmd/plugin.rs:33
quote: Install(InstallArgs),
@row_add n=159 row=decision/no-prompt-or-pager area=surface-cli rec=fct-ejymuula@2
name: No interactive prompt or pager
rule: No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
@row_check n=159 row=decision/no-prompt-or-pager sha=8f32e0f5b425 v=holds judge=sonnet head=8834195b0683
searched: dialoguer|rpassword|pager|Confirm · in src/**/*.rs and Cargo.toml
@row_add n=160 row=decision/argument-order-mirrors-channel area=surface-cli rec=fct-thhsjl2y@1
name: Argument order mirrors the channel params
rule: CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
@row_check n=160 row=decision/argument-order-mirrors-channel sha=3fa0f1569ede v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:24
quote: pub(super) driver: ViolaName,
at: src/cmd/link.rs:27
quote: pub(super) driven: ViolaName,
at: src/cmd/answer.rs:30
quote: pub(super) name: ViolaName,
at: src/cmd/answer.rs:32
quote: dialog_id: u64,
@row_add n=161 row=contract/cli-global-flags/home area=surface-cli rec=fct-lp2knewg@1
boundary: CLI global flags
item: --home
promise: `--home` is the same on every verb.
@row_check n=161 row=contract/cli-global-flags/home sha=3a7980eae612 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:53
quote: #[arg(long, global = true)]
@row_add n=162 row=decision/json-on-every-data-verb area=surface-cli rec=fct-lp2knewg@1
name: --json on every data verb
rule: `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
@row_check n=162 row=decision/json-on-every-data-verb sha=0de2f8397a3b v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:40
quote: json: bool,
at: src/cmd/revive.rs:271
quote: fn list(home: &Path, name: &ViolaName, json: bool) -> anyhow::Result<ExitCode> {
@row_add n=163 row=decision/hints-name-next-verb area=surface-cli rec=fct-xpsr3izn@1
name: Hints name the next verb
rule: `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
@row_check n=163 row=decision/hints-name-next-verb sha=a813ae11a2bc v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:48
quote: "turn-running" => format!("a turn is running; viola wait {name} first"),
@row_add n=164 row=contract/readback-mirror/layout area=surface-cli rec=fct-3k27ozst@1
boundary: the readback mirror line
item: layout
promise: A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields.
@row_check n=164 row=contract/readback-mirror/layout sha=2a1532baed93 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:9
quote: format!("{boxed} {word:<MIRROR_WORD$}  {name}  {rest}\n")
@row_add n=165 row=contract/readback-mirror/issue-and-outcome area=surface-cli rec=fct-gyr4lim4@1
boundary: the readback mirror line
item: issue and outcome lines
promise: The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column.
@row_check n=165 row=contract/readback-mirror/issue-and-outcome sha=9c3b6716a441 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:104
quote: let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
@row_add n=166 row=decision/readback-mirror-no-colour area=surface-cli rec=fct-u27psd7j@1
name: Readback mirror has no colour
rule: The readback mirror is words and brackets only: `read back` is never green and `unable` is never red.
why: Colour on the CLI is reserved for `DIALOG` and `stale`.
@row_check n=166 row=decision/readback-mirror-no-colour sha=dd20030a5f6c v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:9
quote: format!("{boxed} {word:<MIRROR_WORD$}  {name}  {rest}\n")
at: src/cmd/list.rs:64
quote: let colour = human::read_colour(false, false);
@row_add n=167 row=decision/no-progress-glyphs area=surface-cli rec=fct-fce3gzpi@1
name: No send progress glyphs
rule: A send shows no `Sending…` spinner, no `✓` and no `done`.
@row_check n=167 row=decision/no-progress-glyphs sha=a34564e5d2a8 v=holds judge=sonnet head=8834195b0683
searched: Sending|✓|"done · in src/**/*.rs (only turn-ended test data)
@row_add n=168 row=decision/turn-ended-not-send-outcome area=surface-cli rec=fct-fce3gzpi@1
name: turn-ended is not a send outcome
rule: `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
@row_check n=168 row=decision/turn-ended-not-send-outcome sha=e85eafe2fac5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:115
quote: let _ = human::write_woken(&mut out, kind, name, at, cursor);
at: src/human/wait.rs:12
quote: /// `<kind>  <name>  <HH:MM:SS.mmmZ>  cursor <n>`: a woken `wait` (`turn-ended`, `session-end`).
@row_add n=169 row=contract/send-json/output area=surface-cli rec=fct-wi75uj5p@1
boundary: viola send --json
item: typed document
promise: `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
@row_check n=169 row=contract/send-json/output sha=ea86be5191fb v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:110
quote: return client::document(&json!({"v": 1, "ok": ok}));
@row_add n=170 row=contract/viola-list/columns area=surface-cli rec=fct-xqbw4mbe@1
boundary: viola list
item: columns
promise: NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders.
@row_check n=170 row=contract/viola-list/columns sha=28ac3bf8d899 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=171 row=decision/viola-list-styling area=surface-cli rec=fct-tgkwzcus@1,fct-33ocject@1
name: viola list styling
rule: On a colour terminal NAME is bold, the `DIALOG` word is accent amber with its kind word plain, a `stale` row is dim end to end with `DIALOG` still amber, nothing else is styled, and no SGR is written when colour is off.
@row_check n=171 row=decision/viola-list-styling sha=1c09a89c8309 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
at: src/human/style.rs:62
quote: if facts.json
at: src/human/style.rs:66
quote: || !facts.stdout_tty
@row_add n=172 row=contract/viola-list/unwrapped-separator area=surface-cli rec=fct-hrcl6tma@1,fct-33ocject@1
boundary: viola list
item: rack separator
promise: `-- UNWRAPPED - READ-ONLY --` on its own line before the unwrapped rows, written only when unwrapped rows exist; the wrapped rack is always first, with no separator.
@row_check n=172 row=contract/viola-list/unwrapped-separator sha=514deca3b8d6 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:99-100
quote: if wrapped < board.items.len() { lines.push(UNWRAPPED_SEPARATOR.to_owned());
@row_add n=173 row=decision/truncate-only-unwrapped-names area=surface-cli rec=fct-zy7whx5p@1,fct-33ocject@1
name: Truncate only unwrapped names
rule: Only unwrapped NAMEs are cut, with `...`, only where the terminal width is known and too narrow, never inside an escape; wrapped names are never clipped and no line ever wraps.
@row_check n=173 row=decision/truncate-only-unwrapped-names sha=20aa4a99db47 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:52
quote: let name = if !item.wrapped && name.chars().count() > column {
at: src/human/board/cells.rs:132
quote: shown.push_str(ELLIPSIS);
at: src/human/board/cells.rs:128
quote: if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) {
@row_add n=174 row=decision/board-row-escaping area=surface-cli rec=fct-d7l52glo@1,fct-33ocject@1
name: Board row escaping
rule: Every board field derived from `claude agents --json` has C0 and C1 controls, newline and tab included, escaped as hex text, so a row never splits and the columns stay fixed-width.
why: Ratified; stricter than the security floor, which keeps newline and tab for `wait` and `last` text.
@row_check n=174 row=decision/board-row-escaping sha=f614ec349581 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:23
quote: if c.is_control() {
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
@row_add n=175 row=decision/no-link-column area=surface-cli rec=fct-ehkdenpx@1
name: No link column on the board
rule: The six shared board columns stay fixed; links are visible only in `link` / `unlink` verb output.
@row_check n=175 row=decision/no-link-column sha=ef877db86b7a v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
@row_add n=176 row=contract/cli-refusal-line/form area=surface-cli rec=fct-7zli33yx@3
boundary: CLI refusal line
item: line form
promise: `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
@row_check n=176 row=contract/cli-refusal-line/form sha=b4c4917914df v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
@row_add n=177 row=contract/viola-verify-refusals/cli-not-found area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: claude CLI not found
promise: Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
@row_check n=177 row=contract/viola-verify-refusals/cli-not-found sha=438d2aada775 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:161-162
quote: "the claude CLI was not found", "install Claude Code or put it on PATH",
@row_add n=178 row=contract/viola-verify-refusals/version-unreadable area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: CLI version unreadable
promise: Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
@row_check n=178 row=contract/viola-verify-refusals/version-unreadable sha=8db03b4086d8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:196-197
quote: "the CLI version could not be read", "run claude --version to check the install",
@row_add n=179 row=contract/viola-verify-refusals/fixture-not-clean area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: recorded fixture not clean
promise: Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
@row_check n=179 row=contract/viola-verify-refusals/fixture-not-clean sha=af3cb9687b62 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:626-627
quote: &format!("a recorded fixture is not clean: {named}"), "record with a viola home under your user home",
at: src/cmd/verify.rs:603
quote: return Err(format!("{name} row {}{at} {}", fault.row, fault.why.code()));
at: crates/viola-agent-claude/src/ledger.rs:1306-1309
quote: Self::HomePath => "home-path", Self::AbsolutePath => "absolute-path", Self::Username => "username", Self::Email => "email",
@row_add n=180 row=contract/viola-verify-refusals/stopped area=surface-cli rec=fct-7zli33yx@3,fct-g6zaewhr@1
boundary: viola verify refusals
item: verify stopped
promise: A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
@row_check n=180 row=contract/viola-verify-refusals/stopped sha=db411fa08cc4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:83-87
quote: human::refuse( "verify was stopped before it finished", "run viola verify again to stamp this CLI", ); (1, Some("interrupted"))
@row_add n=181 row=contract/viola-verify-refusals/home-not-private area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: viola home not private
promise: A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
@row_check n=181 row=contract/viola-verify-refusals/home-not-private sha=44205e69ed18 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
at: src/cmd/verify.rs:286-288
quote: Err(StampsError::StrictModes(_)) => { refuse_home(); return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=182 row=contract/run-revive-start/socket-dir-refused area=surface-cli rec=fct-7zli33yx@3
boundary: viola run and viola revive starts
item: socket directory refused
promise: Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_check n=182 row=contract/run-revive-start/socket-dir-refused sha=a330134a0845 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:738-739
quote: &format!("the socket directory for {name} cannot be trusted"), &human::send_hint(name, "socket-dir").unwrap_or_default(),
at: src/human/hints.rs:59-60
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \ you; viola does not change its permissions"
@row_add n=183 row=contract/viola-list/strict-modes-refused area=surface-cli rec=fct-7zli33yx@3
boundary: viola list
item: strict-modes refusal
promise: Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.
@row_check n=183 row=contract/viola-list/strict-modes-refused sha=aee4f6efcb35 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:45-47
quote: Err(refused) => { refuse(args.json, &refused); return Ok(ExitCode::from(21));
@row_add n=184 row=contract/frame-verbs/strict-modes-refused area=surface-cli rec=fct-7zli33yx@3
boundary: the frame verbs
item: strict-modes refusal
promise: For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
@row_check n=184 row=contract/frame-verbs/strict-modes-refused sha=98d2cdf17910 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35-37
quote: unreachable_document(UnreachableCause::StrictModes); } else { let (unable, hint) = human::instance_state_pair(name);
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/wait.rs:43
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/last.rs:30
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/answer.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/pause.rs:32
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/release.rs:35
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/unlink.rs:12
quote: use super::link::{LinkArgs, Verb, link_call};
@row_add n=185 row=contract/cli-catch-site/internal-error area=surface-cli rec=fct-7zli33yx@3
boundary: CLI verbs outside a revive start
item: internal error
promise: A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty.
@row_check n=185 row=contract/cli-catch-site/internal-error sha=a68ceb469976 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67-70
quote: if json_asked(std::env::args_os()) { cmd::internal_error_document(); } else { human::internal_error();
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=186 row=contract/send-refusals/control-character area=surface-cli rec=fct-bffy2yb3@1
boundary: viola send refusals before any request
item: control-character
promise: A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line.
@row_check n=186 row=contract/send-refusals/control-character sha=7b923ce8c29a v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/cmd/send.rs:191-195
quote: if let Some(detail) = refusal_of(text) { refused_client_side(detail); out.unable(RefusalReason::NotDelivered.as_str(), Some(detail.as_str())); run::log_self_exit(13, None); return Ok(13);
at: src/human/hints.rs:33-34
quote: "control-character" => { "the text contains a control character (only LF, CR, TAB are allowed)".to_owned()
@row_add n=187 row=contract/send-refusals/empty-text area=surface-cli rec=fct-bffy2yb3@1
boundary: viola send refusals before any request
item: empty-text
promise: A text empty once trailing CR and LF are removed is refused before any frame, exit 13, `[/ ] unable  <name>  not-delivered  empty-text` then `hint: the text is empty once its trailing newlines are removed; send a text with content`.
@row_check n=187 row=contract/send-refusals/empty-text sha=7d5e52ca85cf v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:185-187
quote: typed_text(text) .is_empty() .then_some(NotDelivered::EmptyText)
at: src/cmd/send.rs:195
quote: return Ok(13);
at: src/human/hints.rs:36-37
quote: "empty-text" => { "the text is empty once its trailing newlines are removed; send a text with content"
@row_add n=188 row=decision/wrapper-repeats-send-refusals area=surface-cli rec=fct-bffy2yb3@1
name: Wrapper repeats the send text refusals
rule: The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself.
@row_check n=188 row=decision/wrapper-repeats-send-refusals sha=e5f77aabb790 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:94
quote: return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=189 row=decision/hint-for-every-cause area=surface-cli rec=fct-qhojlk2s@3
name: A hint for every refusal cause
rule: Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint.
why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line.
except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
@row_check n=189 row=decision/hint-for-every-cause sha=ad5b7081d84b v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:62
quote: _ => return None,
at: src/human/mirror.rs:46
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=190 row=contract/frame-verb-hints/instance-not-running area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: instance not running
promise: `hint: <name> is not running; viola list shows the live instances` under `unable  <name>  instance-unreachable` for `wait`, `last`, `link`, `unlink`, and `send` in its mirror.
@row_check n=190 row=contract/frame-verb-hints/instance-not-running sha=432e4a382ffe v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:49
quote: "not-running" => format!("{name} is not running; viola list shows the live instances"),
at: src/cmd/client.rs:144
quote: Self::Dead => "not-running",
at: src/cmd/wait.rs:51
quote: client::unable_unreachable(name, cause);
at: src/cmd/last.rs:38
quote: client::unable_unreachable(name, cause);
at: src/cmd/link.rs:100
quote: client::unable_unreachable(driven, cause);
at: src/cmd/send.rs:147
quote: let hint = human::send_hint(self.name, cause.hint_cause());
@row_add n=191 row=contract/frame-verb-hints/not-wrapped area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: name with no instance directory
promise: `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, the last stderr line.
@row_check n=191 row=contract/frame-verb-hints/not-wrapped sha=9ab8eef2bc3d v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:50-52
quote: "unwrapped-name" => format!( "{name} is not a wrapped session; only sessions started with viola run take commands" ),
at: src/cmd/client.rs:145
quote: Self::Unwrapped => "unwrapped-name",
@row_add n=192 row=contract/frame-verb-hints/wrapper-went-away area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: wrapper lost during the call
promise: `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`, the last stderr line.
@row_check n=192 row=contract/frame-verb-hints/wrapper-went-away sha=d72e1d5cefe8 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:53-55
quote: "endpoint-lost" => format!( "{name}'s wrapper went away during the call; viola list shows whether it is still running" ),
at: src/cmd/client.rs:200
quote: return Ok(Err(Unreachable::Lost));
@row_add n=193 row=contract/frame-verb-hints/server-verify-failed area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: endpoint not the recorded wrapper
promise: No frame written, exit 21, `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`; under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, stderr empty.
@row_check n=193 row=contract/frame-verb-hints/server-verify-failed sha=ef751fe92287 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:56-58
quote: "server-verify" => format!( "the process on {name}'s endpoint is not its recorded wrapper; it was not contacted" ),
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=194 row=contract/frame-verb-hints/socket-dir-refused area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: socket directory refused
promise: No frame written, exit 21, `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`; under `--json` the `socket-dir-refused` detail, stderr empty.
@row_check n=194 row=contract/frame-verb-hints/socket-dir-refused sha=70309ec12cca v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:59-60
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \ you; viola does not change its permissions"
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
@row_add n=195 row=decision/hints-never-quote-or-locate area=surface-cli rec=fct-qhojlk2s@3
name: Hints never quote text or name paths
rule: A hint never quotes the sent text or any upstream text, and never names a path or a pid.
@row_check n=195 row=decision/hints-never-quote-or-locate sha=8d4e6baaa84a v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:25
quote: /// `endpoint-lost`, `server-verify` or `socket-dir`. Never the sent text, a path or a pid.
at: src/human/hints.rs:26
quote: pub(crate) fn send_hint(name: &str, cause: &str) -> Option<String> {
@row_add n=196 row=contract/cli-exit-codes/10 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 10
promise: `human-typing`.
@row_check n=196 row=contract/cli-exit-codes/10 sha=ee633e0d1b01 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=197 row=contract/cli-exit-codes/11 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 11
promise: `budget-paused`.
@row_check n=197 row=contract/cli-exit-codes/11 sha=281c8ba45948 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
@row_add n=198 row=contract/cli-exit-codes/12 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 12
promise: `unverified-cli`.
@row_check n=198 row=contract/cli-exit-codes/12 sha=1a1b07c91fa0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
@row_add n=199 row=contract/cli-exit-codes/13 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 13
promise: `not-delivered`.
@row_check n=199 row=contract/cli-exit-codes/13 sha=db4416db8d68 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=200 row=contract/cli-exit-codes/14 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 14
promise: `unknown`.
@row_check n=200 row=contract/cli-exit-codes/14 sha=9290f9466bbe v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
@row_add n=201 row=contract/cli-exit-codes/20 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 20
promise: Wrapper fault, `error: wrapper fault  <code>`.
@row_check n=201 row=contract/cli-exit-codes/20 sha=d8db542f96e5 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
at: src/cmd/client.rs:245
quote: 20
@row_add n=202 row=contract/cli-exit-codes/21 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
@row_check n=202 row=contract/cli-exit-codes/21 sha=a003ed4bc768 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:275
quote: "instance-unreachable",
at: src/cmd/client.rs:37
quote: let (unable, hint) = human::instance_state_pair(name);
at: src/cmd/list.rs:105
quote: let (unable, hint) = human::home_state_pair();
at: src/cmd/plugin.rs:57
quote: let (unable, hint) = human::home_state_pair();
@row_add n=203 row=contract/cli-exit-codes/json-form area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: --json form
promise: Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
@row_check n=203 row=contract/cli-exit-codes/json-form sha=d9656bd14cb4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:250-255
quote: pub(crate) fn unreachable(json: bool, cause: Unreachable, human: impl FnOnce()) -> u8 { if json { unreachable_document(cause.cause()); } else { human(); }
at: src/cmd/client.rs:221
quote: let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
@row_add n=204 row=contract/instance-unreachable-detail/strict-modes-failed area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: strict-modes-failed
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_check n=204 row=contract/instance-unreachable-detail/strict-modes-failed sha=e0532c95034b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
@row_add n=205 row=contract/instance-unreachable-detail/instance-dead area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: instance-dead
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_check n=205 row=contract/instance-unreachable-detail/instance-dead sha=f69dbb4a7760 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: src/cmd/client.rs:119
quote: Self::Dead => UnreachableCause::InstanceDead,
@row_add n=206 row=contract/instance-unreachable-detail/unwrapped-name area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: unwrapped-name
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_check n=206 row=contract/instance-unreachable-detail/unwrapped-name sha=ebaa640925ce v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: src/cmd/client.rs:120
quote: Self::Unwrapped => UnreachableCause::UnwrappedName,
@row_add n=207 row=contract/instance-unreachable-detail/endpoint-lost area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: endpoint-lost
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_check n=207 row=contract/instance-unreachable-detail/endpoint-lost sha=f5ade3a25391 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
@row_add n=208 row=contract/instance-unreachable-detail/server-verify-failed area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: server-verify-failed
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_check n=208 row=contract/instance-unreachable-detail/server-verify-failed sha=35e360f286ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: src/cmd/client.rs:122
quote: Self::ServerUnverified => UnreachableCause::ServerVerify,
@row_add n=209 row=contract/instance-unreachable-detail/socket-dir-refused area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: socket-dir-refused
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_check n=209 row=contract/instance-unreachable-detail/socket-dir-refused sha=c3f49198ee4b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
@row_add n=210 row=decision/stdout-results-stderr-rest area=surface-cli rec=fct-enuexet3@1
name: Results on stdout, the rest on stderr
rule: Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
@row_check n=210 row=decision/stdout-results-stderr-rest sha=71dcff4f1065 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:59
quote: out.write_all(format!("{line}\n").as_bytes())
at: src/cmd/wait.rs:70
quote: let _ = human::write_waiting(&mut io::stderr().lock(), name);
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
@row_add n=211 row=decision/no-terminator-word area=surface-cli rec=fct-gnhj2wau@1
name: No terminator word
rule: No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
@row_check n=211 row=decision/no-terminator-word sha=992b591e06ba v=holds judge=sonnet head=8834195b0683
searched: "done." · in src/human/**/*.rs and src/cmd/**/*.rs
searched: success · in src/human/**/*.rs (only ExitCode::SUCCESS in src/cmd, no printed banner)
@row_add n=212 row=decision/hint-last-on-stderr area=surface-cli rec=fct-io65ypkg@1
name: Hint is the last stderr line
rule: On a refusal, the `hint:` line is the last line on stderr.
@row_check n=212 row=decision/hint-last-on-stderr sha=b3adcfcb880d v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=213 row=contract/viola-verify/summary area=surface-cli rec=fct-g6zaewhr@1
boundary: viola verify output
item: summary line
promise: `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
@row_check n=213 row=contract/viola-verify/summary sha=b97ea2ec3909 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:293-294
quote: human::result(&format!( "stamped {version}  {} pass  {failed} fail",
at: src/cmd/verify.rs:249-251
quote: let Some((typed, agents)) = typed else { return Ok(Ended::Interrupted); };
@row_add n=214 row=decision/no-persistent-footer area=surface-cli rec=fct-ilu5rgbe@1
name: No persistent footer
rule: The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
@row_check n=214 row=decision/no-persistent-footer sha=0ff1f73fe495 v=holds judge=sonnet head=8834195b0683
searched: footer · in src/**/*.rs
searched: footer · in crates/viola-core, crates/viola-state, crates/viola-mcp, crates/viola-channel
@row_add n=215 row=absence/cli-deep-links area=surface-cli rec=fct-6tq7nrxg@1
capability: Deep links or URL coupling in the CLI
@row_check n=215 row=absence/cli-deep-links sha=81daf7684f97 v=holds judge=sonnet head=8834195b0683
searched: https?:// · in src/**/*.rs
searched: https?:// · in crates/viola-core, crates/viola-state, crates/viola-mcp, crates/viola-channel
@row_add n=216 row=decision/violaname-never-case-transformed area=surface-cli rec=fct-6tq7nrxg@1
name: ViolaName never case-transformed
rule: `ViolaName` is a shared reference and is never case-transformed.
@row_check n=216 row=decision/violaname-never-case-transformed sha=b0e15933b7f8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:49
quote: validate(predicate = is_valid_name),
searched: to_lowercase|to_ascii_lowercase|to_uppercase|to_ascii_uppercase · in crates/viola-core, crates/viola-state, crates/viola-mcp, crates/viola-channel
@row_add n=217 row=contract/cursor/byte-offset area=surface-cli rec=fct-6tq7nrxg@1
boundary: cursor
item: cursor
promise: `cursor` is the byte offset `send` prints and `wait --after` consumes.
@row_check n=217 row=contract/cursor/byte-offset sha=aac9060f8990 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:24
quote: /// Return the first event whose line starts at or after this cursor (a byte offset)
at: src/cmd/send.rs:113
quote: let cursor = ok["cursor"].as_u64().unwrap_or_default();
@row_add n=218 row=decision/human-output-is-contract area=surface-cli rec=fct-6hpmxl26@1
name: Human output is a contract
rule: The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice.
why: LLM drivers may read the human output.
@row_check n=218 row=decision/human-output-is-contract sha=88b590007f61 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=219 row=contract/readback-vocabulary/words area=surface-cli rec=fct-mpm2axrl@2
boundary: readback vocabulary
item: state words
promise: The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
@row_check n=219 row=contract/readback-vocabulary/words sha=79ad217b99b7 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:15
quote: out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
at: src/human/mirror.rs:33
quote: out.write_all(mirror_line("[  ]", "unconfirmable", name, note).as_bytes())
@row_add n=220 row=decision/cli-field-separators area=surface-cli rec=fct-mpm2axrl@2
name: CLI field separators
rule: ` - ` separates inside a CLI field and an arrow is `->`; there is no CLI inbound `<-` form.
@row_check n=220 row=decision/cli-field-separators sha=6ecfd83ef185 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
searched: <-  · in src/**/*.rs (non-test code; no inbound arrow form found)
@row_add n=221 row=decision/amber-means-dialog area=surface-cli rec=fct-mpm2axrl@2
name: Amber means DIALOG only
rule: Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal.
@row_check n=221 row=decision/amber-means-dialog sha=a09588de6ec2 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
at: src/human/style.rs:38
quote: stale: "\x1b[2m",
@row_add n=222 row=decision/cli-holds-every-action area=surface-cli rec=fct-mpm2axrl@2
name: The CLI holds every action
rule: The CLI is where every action lives.
@row_check n=222 row=decision/cli-holds-every-action sha=fa68467d157d v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=223 row=decision/targets-are-wrapped-names area=surface-cli rec=fct-thhsjl2y@1
name: A CLI target is always a ViolaName
rule: A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
@row_check n=223 row=decision/targets-are-wrapped-names sha=3e04b566ccc0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:44
quote: ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
at: src/cmd/link.rs:24
quote: pub(super) driver: ViolaName,
@row_none n=1 rec=fct-hc5anmlh@1 type=S
@row_none n=2 rec=fct-ph6oztbi@1 type=8
@row_none n=3 rec=fct-ltetwsoo@1 type=0
what: a note on when the document may be edited by hand
@row_none n=4 rec=fct-2hfcvv5g@1 type=S
@row_none n=5 rec=fct-xc3aytgy@1 type=S
@row_none n=6 rec=fct-nlcn5hiz@1 type=S
@row_none n=7 rec=fct-psc5isem@1 type=S
@row_none n=8 rec=fct-j2ytpooj@1 type=0
what: a sample output wireframe of viola list
@row_none n=9 rec=fct-d6ktoce7@1 type=0
what: a sample output of viola list --json
@row_none n=10 rec=fct-6pvzlllg@1 type=S
@row_none n=11 rec=fct-bamlmik6@1 type=S
