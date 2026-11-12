@row_add n=1 row=decision/one-strip-per-run-instance area=brand-identity rec=fct-2fkrm5qy@1
name: One flight-progress strip per instance
rule: Every `viola run` instance is shown as exactly one flight-progress strip.
@row_check n=1 row=decision/one-strip-per-run-instance sha=de095317ec87 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: src/human/board/rows.rs:45
quote: for (item, name) in items.iter().zip(names) {
@row_add n=2 row=contract/flight-progress-strip/name area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: NAME
promise: NAME is the first of the six fixed fields, set at callsign weight.
@row_check n=2 row=contract/flight-progress-strip/name sha=98f9f07cfd77 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
@row_add n=3 row=contract/flight-progress-strip/live area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: LIVE
promise: LIVE is the second of the six fixed fields.
@row_check n=3 row=contract/flight-progress-strip/live sha=31f5a3693429 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=4 row=contract/flight-progress-strip/status area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: STATUS
promise: STATUS is the third of the six fixed fields.
@row_check n=4 row=contract/flight-progress-strip/status sha=6d2cc692a760 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=5 row=contract/flight-progress-strip/wheel area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: WHEEL
promise: WHEEL is the fourth of the six fixed fields.
@row_check n=5 row=contract/flight-progress-strip/wheel sha=0db14185590f v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=6 row=contract/flight-progress-strip/dialog area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: DIALOG
promise: DIALOG is the fifth of the six fixed fields.
@row_check n=6 row=contract/flight-progress-strip/dialog sha=bb852de8df17 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=7 row=contract/flight-progress-strip/cli area=brand-identity rec=fct-2fkrm5qy@1
boundary: Flight-progress strip
item: CLI
promise: CLI is the sixth and last of the six fixed fields.
@row_check n=7 row=contract/flight-progress-strip/cli sha=da7bbb3fb90f v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=8 row=contract/flight-progress-strip/no-reading area=brand-identity rec=fct-2fkrm5qy@1,fct-oudd7ujq@2
boundary: Flight-progress strip
item: field with no reading
promise: A field with no reading prints `unknown` in its box, never a shimmer or a guess.
@row_check n=8 row=contract/flight-progress-strip/no-reading sha=df43d83a9881 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:16
quote: let holder = item.wheel.map_or("unknown", Wheel::as_str);
at: src/human/board/cells.rs:47
quote: None => "unknown".to_owned(),
at: crates/viola-state/src/board.rs:136
quote: Self::Unknown => "unknown",
@row_add n=9 row=decision/dialog-pending-cocked-strip area=brand-identity rec=fct-sovfvsa2@1,fct-ve72ntsx@2
name: Dialog-pending shown as a cocked strip
rule: A `dialog_pending` session keeps its slot and shows the word `DIALOG` plus the dialog kind, the word in arrival-holder amber on the CLI.
@row_check n=9 row=decision/dialog-pending-cocked-strip sha=bbbfe4ffdcf3 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:32
quote: Some(kind @ ("question" | "permission" | "plan")) => format!("{DIALOG_WORD} {kind}"),
at: src/human/board/rows.rs:59
quote: dialog: item.wrapped && item.dialog_pending,
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=10 row=contract/send-readback/prompt area=brand-identity rec=fct-l76aktpo@1,fct-v5ralmpr@1
boundary: Send readback
item: prompt send
promise: A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back.
@row_check n=10 row=contract/send-readback/prompt sha=66858583efa8 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:90
quote: if matches!((f.confirms, &f.state), (Confirms::Prompt, Match::Waiting))
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=11 row=contract/send-readback/clear area=brand-identity rec=fct-l76aktpo@1,fct-v5ralmpr@1
boundary: Send readback
item: /clear
promise: A `/clear` send counts only once its post-condition, a `session-start` with cause `clear` (a new session), is observed.
@row_check n=11 row=contract/send-readback/clear sha=18131677629a v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:111
quote: Some("clear") if prior.as_deref() != Some(id) => Some(PostCondition::NewSession),
@row_add n=12 row=contract/send-readback/compact area=brand-identity rec=fct-l76aktpo@1,fct-v5ralmpr@1
boundary: Send readback
item: /compact
promise: On a verified CLI a `/compact` send counts only once a `session-start` of cause `compact` carrying the session's own id is observed.
@row_check n=12 row=contract/send-readback/compact sha=0df0e9d65199 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:112-113
quote: Some("compact") if prior.as_deref() == Some(id) => { Some(PostCondition::CompactedSession)
at: src/run/send/classify.rs:30
quote: Some((_, Some(post))) if cli_verified => Confirms::Post(*post),
@row_add n=13 row=contract/send-readback/unconfirmable area=brand-identity rec=fct-l76aktpo@1
boundary: Send readback
item: unconfirmable
promise: On an unverified CLI a `/compact` send stays `unconfirmable`.
@row_check n=13 row=contract/send-readback/unconfirmable sha=f19d20bb3296 v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:31
quote: Some(_) => Confirms::Nothing,
at: src/run/send.rs:151-152
quote: if matches!(confirms, Confirms::Nothing) { return ctx.unconfirmable(cursor, slot.clock.now());
@row_add n=14 row=contract/send-readback/unable area=brand-identity rec=fct-l76aktpo@1
boundary: Send readback
item: unable
promise: A send not read back is reported as `unable` plus a typed reason.
@row_check n=14 row=contract/send-readback/unable sha=a80c8fb7b5eb v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
@row_add n=15 row=contract/handoff/transfer-marker area=brand-identity rec=fct-dpdlxhra@1
boundary: Handoff display
item: transfer marker
promise: Transfer of control is shown as a `driver -> driven` transfer marker.
@row_check n=15 row=contract/handoff/transfer-marker sha=41f85ed2aac7 v=stale judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
shows: The transfer marker is printed in ASCII as `driver -> driven`, not with the `→` character.
cell: promise
value: Transfer of control is shown as a `driver -> driven` transfer marker.
@row_add n=16 row=decision/bare-strips-never-target area=brand-identity rec=fct-ghjs2mx3@1,fct-juf7qttk@2
name: Unwrapped sessions are never a target
rule: Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
@row_check n=16 row=decision/bare-strips-never-target sha=e4593cb9b809 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:46-51
quote: let (wheel, dialog, cli) = if item.wrapped { (wheel_cell(item), dialog_cell(item), cli_cell(item)) } else { let na = || NOT_APPLICABLE.to_owned(); (na(), na(), na()) };
at: crates/viola-state/src/board.rs:104
quote: /// A row's liveness: a gone instance has no row.
@row_add n=17 row=decision/stale-strip-dimmed-not-deleted area=brand-identity rec=fct-2ks73rzm@1
name: Stale strip dimmed, never deleted
rule: A strip of `stale` liveness is dimmed to lamp-off, never deleted, and keeps its slot.
@row_check n=17 row=decision/stale-strip-dimmed-not-deleted sha=8973faf886a4 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:60
quote: stale: item.wrapped && item.liveness == RowLiveness::Stale,
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
@row_add n=18 row=decision/budget-paused-stops-new-sends area=brand-identity rec=fct-iqsnuw4w@1
name: Budget-paused as minimum-fuel state
rule: The budget governor shows `5H` / `7D` figures, and `budget-paused` is a declared state that stops new sends while the running turn finishes.
@row_check n=18 row=decision/budget-paused-stops-new-sends sha=e5f5992f5051 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:52
quote: window_cell("5H", &budget.reading.reading.five_hour, now),
at: src/human/board/bay.rs:53
quote: window_cell("7D", &budget.reading.reading.seven_day, now),
at: src/human/board/bay.rs:56
quote: "budget-paused"
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
@row_add n=19 row=decision/readings-carry-their-age area=brand-identity rec=fct-xvme6vpp@1,fct-oudd7ujq@2
name: Readings carry their age
rule: Every budget reading carries its age (`read 4m ago`), and old information is shown as old (`expired`), never as current.
@row_check n=19 row=decision/readings-carry-their-age sha=b4f0f71d2118 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:38
quote: Some(at) => format!("read {}m ago", (now - at.with_timezone(&Utc)).num_minutes()),
at: src/human/board/bay.rs:26
quote: let suffix = if expired { " expired" } else { "" };
@row_add n=20 row=contract/cli-verification/verified area=brand-identity rec=fct-tmhb2v3c@1
boundary: CLI verification state
item: verified
promise: A CLI version stamped by `viola verify` prints `verified`.
@row_check n=20 row=contract/cli-verification/verified sha=294cdeecee8c v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:40-41
quote: let verdict = if item.cli_verified == Some(true) { "verified"
@row_add n=21 row=contract/cli-verification/unverified-cli area=brand-identity rec=fct-tmhb2v3c@1
boundary: CLI verification state
item: unverified-cli
promise: An unstamped CLI build runs transport only, shown as `unverified-cli`.
@row_check n=21 row=contract/cli-verification/unverified-cli sha=adfec9f9fcc1 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:43
quote: "unverified-cli"
at: src/run/send/classify.rs:31
quote: Some(_) => Confirms::Nothing,
@row_add n=22 row=decision/pause-release-transfer-control area=brand-identity rec=fct-vgjq5gux@1
name: Pause and release as control transfer
rule: `viola pause` is the human's "I have control" and `viola release` is "you have control".
@row_check n=22 row=decision/pause-release-transfer-control sha=98049ff3dcae v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:17
quote: out.write_all(format!("{name}  wheel human  manual-pause  I have control\n").as_bytes())
at: src/human/verbs.rs:22
quote: out.write_all(format!("{name}  wheel {holder}  you have control\n").as_bytes())
@row_add n=23 row=decision/fixed-phraseology-vocabulary area=brand-identity rec=fct-geooqybe@1
name: Fixed phraseology vocabulary
rule: Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
@row_check n=23 row=decision/fixed-phraseology-vocabulary sha=7119f6bbb1e7 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:15
quote: out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
at: src/human/hints.rs:11
quote: ("human-typing", _) => Some(HUMAN_TYPING_HINT),
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
at: src/run/dialog.rs:736
quote: Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
@row_add n=24 row=decision/readback-box area=brand-identity rec=fct-zdzmu6qp@2
name: The readback box as signature element
rule: The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
@row_check n=24 row=decision/readback-box sha=5e529e53ff13 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:15
quote: out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
@row_add n=25 row=contract/readback-box/unconfirmable area=brand-identity rec=fct-a6oxvtbj@1
boundary: Readback box
item: unconfirmable
promise: The printed word `unconfirmable` reuses the open drawing and is never filled.
@row_check n=25 row=contract/readback-box/unconfirmable sha=98182a824758 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:33
quote: out.write_all(mirror_line("[  ]", "unconfirmable", name, note).as_bytes())
@row_add n=26 row=decision/turn-ended-never-fills-readback area=brand-identity rec=fct-a6oxvtbj@1,fct-juf7qttk@2
name: Turn-ended never fills the readback box
rule: A `turn-ended` event never fills a readback box.
@row_check n=26 row=decision/turn-ended-never-fills-readback sha=c3bd26cd2e5b v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:213-216
quote: if ok["confirmed"] == false { out.unconfirmable(&ok); } else { out.read_back(&ok);
at: src/human/wait.rs:72-73
quote: one_write(|o| write_woken(o, "turn-ended", "builder", "19:44:10.221Z", 49102)), "turn-ended  builder  19:44:10.221Z  cursor 49102\n"
@row_add n=27 row=contract/readback-box/cli-mirror area=brand-identity rec=fct-a6oxvtbj@1,fct-qwc3u462@1
boundary: Readback box
item: CLI mirror
promise: The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
@row_check n=27 row=contract/readback-box/cli-mirror sha=53cf7d06213a v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
at: src/human/mirror.rs:15
quote: out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
@row_add n=28 row=decision/expression-level area=brand-identity rec=fct-cggcilgt@1,fct-4m7ag2lh@2,fct-wjefhzv3@1
name: Expression level 0.2
rule: The base expression level `0.2` is the single source of truth for animation intensity, layout freedom, interaction complexity and motion across the whole project.
@row_check n=28 row=decision/expression-level sha=2b6b507173a1 v=unverifiable judge=sonnet head=8834195b0683
searched: expression · in src/**/*.rs and crates/**/*.rs
why: not a code claim
@row_add n=29 row=decision/cli-has-no-motion area=brand-identity rec=fct-4m7ag2lh@2
name: No motion in the CLI
rule: The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
@row_check n=29 row=decision/cli-has-no-motion sha=23700c89da6a v=holds judge=sonnet head=8834195b0683
searched: spinner|animat|indicatif|blink · in src/**/*.rs (case-insensitive, no motion hit; the only sleep( calls are polling waits)
searched: \x1b\[2K · in src/**/*.rs
@row_add n=30 row=overview/brand-identity area=brand-identity rec=fct-iy6nu2f4@2
text: The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.
@row_check n=30 row=overview/brand-identity sha=f96745669f04 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=31 row=decision/no-success-before-confirmation area=brand-identity rec=fct-oudd7ujq@2
name: Nothing announces success before confirmation
rule: Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
@row_check n=31 row=decision/no-success-before-confirmation sha=eec251d4ea44 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:104
quote: let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
at: src/cmd/send.rs:211-216
quote: let code = match reply { Reply::Ok(ok) => { if ok["confirmed"] == false { out.unconfirmable(&ok); } else { out.read_back(&ok);
@row_add n=32 row=decision/strip-board-density area=brand-identity rec=fct-oudd7ujq@2
name: Calm strip-board density
rule: The density is a strip board's, not a monitoring dashboard's: fixed fields, tabular numbers, one attention colour and one handoff colour, everything else paper and ink.
@row_check n=32 row=decision/strip-board-density sha=36e3877efd87 v=unverifiable judge=sonnet head=8834195b0683
searched: handoff · in src/human/style.rs
why: not a code claim
@row_add n=33 row=decision/style-direction area=brand-identity rec=fct-oudd7ujq@2
name: Precision and Density style direction
rule: The style direction is Precision & Density as primary with Utility & Function as secondary.
@row_check n=33 row=decision/style-direction sha=5cdbe0ffaffb v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=34 row=decision/rejected-style-presets area=brand-identity rec=fct-oudd7ujq@2
name: Rejected style presets
rule: The Dark Mode (OLED), Real-Time Monitoring (pulsing dots, blink, toasts) and HUD / Sci-Fi FUI (glow, telemetry animation) presets are not used.
@row_check n=34 row=decision/rejected-style-presets sha=eb1e72cc7218 v=holds judge=sonnet head=8834195b0683
searched: Dark Mode|OLED|[Ss]himmer|[Bb]link|Real-Time Monitoring · in crates/**, tests/**, src/**
searched: spinner|indicatif · in **/*.rs
@row_add n=35 row=decision/amber-only-for-dialog-pending area=color-palette rec=fct-sjonlmz3@1,fct-w6mrbusn@1
name: Arrival amber for dialog-pending only
rule: Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only.
except: never for errors, warnings, budget or hover
@row_check n=35 row=decision/amber-only-for-dialog-pending sha=11fe55b824e5 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:19-20
quote: /// The `DIALOG` word only. pub(crate) attention: &'static str,
at: src/human/board/rows.rs:59
quote: dialog: item.wrapped && item.dialog_pending,
@row_add n=36 row=decision/no-green-accent area=color-palette rec=fct-uqrncnsb@1
name: No run-green accent
rule: The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
@row_check n=36 row=decision/no-green-accent sha=932f09db4a3e v=holds judge=sonnet head=8834195b0683
searched: \[32m|38;5;(2|34|40|46)\b|green · in src/**
searched: 38;2;217;119;6 · in src/human/style.rs
@row_add n=37 row=decision/refused-send-folds-into-send-line area=color-palette rec=fct-juf7qttk@2
name: Refused send folds into its send line
rule: A refused send folds into its send line, never a separate line.
@row_check n=37 row=decision/refused-send-folds-into-send-line sha=03dd9169e27a v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
@row_add n=38 row=decision/skipped-count-stays-visible area=color-palette rec=fct-juf7qttk@2
name: Skipped count stays visible
rule: A nonzero `skipped` count stays visible and is never auto-cleared.
@row_check n=38 row=decision/skipped-count-stays-visible sha=78359fe16a86 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:69-70
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}", skipped.unknown_kinds, skipped.unknown_fields, skipped.torn_lines
@row_add n=39 row=decision/bare-strip-never-cocked area=color-palette rec=fct-juf7qttk@2
name: Bare strip never cocked
rule: A bare (unwrapped) strip is never shown cocked.
@row_check n=39 row=decision/bare-strip-never-cocked sha=ad29d083a7b3 v=holds judge=sonnet head=8834195b0683
searched: cocked · in src/**, crates/**, tests/**
searched: "n/a" · in src/human/board/rows.rs
@row_add n=40 row=decision/busy-not-animated area=color-palette rec=fct-juf7qttk@2
name: Busy has no animation
rule: The `busy` state has no spinner, pulse or animation.
@row_check n=40 row=decision/busy-not-animated sha=bbab97625278 v=holds judge=sonnet head=8834195b0683
searched: spinner|pulse|animation|keyframes · in src/**, crates/**, tests/**
searched: \x1b[5m · in src/human/style.rs
@row_add n=41 row=decision/cli-terminal-font area=typography rec=fct-qwc3u462@1
name: CLI uses the terminal's own font
rule: The cli uses the user's configured terminal monospace; the brand shows through the fixed-column structure and the phraseology words.
@row_check n=41 row=decision/cli-terminal-font sha=4c0e1235eed2 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=42 row=decision/bold-only-name-column area=typography rec=fct-qwc3u462@1
name: Bold only for the NAME column
rule: In the cli, bold is used only for the NAME column.
@row_check n=42 row=decision/bold-only-name-column sha=6f97df966e06 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
@row_add n=43 row=decision/stale-rows-sgr-dim area=typography rec=fct-qwc3u462@1
name: Stale rows in SGR dim
rule: In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.
@row_check n=43 row=decision/stale-rows-sgr-dim sha=4502c76adda4 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=44 row=decision/cli-two-space-columns area=spacing rec=fct-ae76qcah@1,fct-ukpiycjz@2
name: Two spaces between CLI columns
rule: CLI spacing follows the fixed-width terminal grid, with two spaces between columns.
@row_check n=44 row=decision/cli-two-space-columns sha=d17e0421e47d v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:95
quote: "{open}{shown_name}{}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \
@row_add n=45 row=decision/text-glyphs-only area=iconography rec=fct-le2hvglu@1
name: Custom text glyphs only
rule: Iconography is custom text glyphs only; phraseology words carry the meaning.
@row_check n=45 row=decision/text-glyphs-only sha=83f6d65ab849 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=46 row=contract/cli-glyphs/arrow area=iconography rec=fct-ukpiycjz@2
boundary: CLI glyphs
item: ->
promise: The arrow glyph is `->`, with no inbound form.
@row_check n=46 row=contract/cli-glyphs/arrow sha=34317cdb40a0 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
@row_add n=47 row=contract/cli-glyphs/field-separator area=iconography rec=fct-ukpiycjz@2
boundary: CLI glyphs
item: " - "
promise: ` - ` separates parts inside one field.
@row_check n=47 row=contract/cli-glyphs/field-separator sha=2133ff263dda v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
@row_add n=48 row=contract/viola-list/columns area=iconography rec=fct-ukpiycjz@2
boundary: viola list output
item: columns
promise: `viola list` carries no link column in v1, so its six columns stay fixed.
@row_check n=48 row=contract/viola-list/columns sha=5458b0f2e84b v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
@row_add n=49 row=decision/no-decorative-icons area=iconography rec=fct-3pqyewvo@1
name: No decorative icons
rule: Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
@row_check n=49 row=decision/no-decorative-icons sha=ddfb3f35f470 v=holds judge=sonnet head=8834195b0683
searched: ✓|●|⚠|🤖 · in src/**/*.rs
searched: [^\x00-\x7F] · in src/**/*.rs (only comments, test fixtures)
@row_add n=50 row=absence/web-page-surface area=surface-web-spa rec=fct-ll5xgfku@2,fct-gjdxui7c@2
capability: A web page or web surface of any kind
@row_check n=50 row=absence/web-page-surface sha=300015c31b6d v=holds judge=sonnet head=8834195b0683
searched: axum|actix|warp|rocket|hyper|tide|<html|wasm_bindgen|leptos|yew|dioxus|tower_http|TcpListener · in **/*.rs, **/Cargo.toml, **/*.html, **/package.json (only hyperfine matches in crates/viola-e2e/src/harness, a benchmark tool)
searched: *.html, *.tsx, *.jsx, *.vue, *.svelte · in repository (glob, none found)
@row_add n=51 row=decision/linux-terminal-platform area=surface-cli rec=fct-7z5o76y5@2
name: Linux terminals as the CLI platform
rule: The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
@row_check n=51 row=decision/linux-terminal-platform sha=402c9e1d9ec5 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
@row_add n=52 row=contract/cli-surface/output-streams area=surface-cli rec=fct-rj57r756@1
boundary: CLI surface
item: stdout/stderr
promise: The cli surface is the stdout and stderr of the `viola` binary.
@row_check n=52 row=contract/cli-surface/output-streams sha=39fdbb259fe7 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
at: src/human.rs:58-59
quote: pub(crate) fn write_result(out: &mut impl Write, line: &str) -> io::Result<()> { out.write_all(format!("{line}\n").as_bytes())
@row_add n=53 row=contract/cli-callers/human area=surface-cli rec=fct-35irlzoz@1
boundary: CLI callers
item: human on a TTY
promise: The human calls the cli on a TTY.
@row_check n=53 row=contract/cli-callers/human sha=2d8985c28a97 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:102
quote: if issue_line_shows(self.json, io::stdout().is_terminal()) {
@row_add n=54 row=contract/cli-callers/llm-driver area=surface-cli rec=fct-35irlzoz@1
boundary: CLI callers
item: LLM driver sessions
promise: LLM driver sessions call the cli via `--json` or MCP.
@row_check n=54 row=contract/cli-callers/llm-driver sha=34d9f7838254 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:36
quote: json: bool,
at: src/cmd/mod.rs:96
quote: Mcp,
@row_add n=55 row=decision/clap-without-colour-feature area=surface-cli rec=fct-3n5f23up@1
name: clap built without its colour feature
rule: Argument parsing uses clap (derive) built without its `color` feature, so clap's help and usage output is plain in every mode and no dependency reads a colour or terminal variable.
why: every styled byte of human output is viola's own SGR module's
@row_check n=55 row=decision/clap-without-colour-feature sha=17a31d978044 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:197
quote: clap = { version = "=4.6.7", default-features = false, features = [
at: src/human/style.rs:91
quote: let text = |name: &str| std::env::var_os(name).map(|v| v.to_string_lossy().into_owned());
@row_add n=56 row=decision/tty-detection-std-isterminal area=surface-cli rec=fct-7yh2z5by@1
name: TTY detection by std IsTerminal
rule: TTY detection uses std `IsTerminal`.
@row_check n=56 row=decision/tty-detection-std-isterminal sha=3727897dd771 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:98
quote: stdout_tty: io::stdout().is_terminal(),
@row_add n=57 row=decision/hand-written-sgr-module area=surface-cli rec=fct-qu7ywsq4@1
name: Hand-written SGR styling module
rule: Styling is a small hand-written SGR module in the `viola` bin.
why: adds no dependency
@row_check n=57 row=decision/hand-written-sgr-module sha=cf1b3bdd7445 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:32
quote: Self::Truecolor => "\x1b[38;2;217;119;6m",
@row_add n=58 row=contract/cli-colour-tokens/dim area=surface-cli rec=fct-dufqiv2l@1,fct-erfdav33@1
boundary: CLI colour tokens
item: dim
promise: SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
@row_check n=58 row=contract/cli-colour-tokens/dim sha=0298a394f4cf v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
at: src/human/board/cells.rs:85
quote: let undim = if spans.stale { tokens.reset } else { "" };
@row_add n=59 row=contract/cli-colour-tokens/bold area=surface-cli rec=fct-dufqiv2l@1,fct-erfdav33@1
boundary: CLI colour tokens
item: bold
promise: NAME values are bold.
@row_check n=59 row=contract/cli-colour-tokens/bold sha=753c1c22ca7b v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:79
quote: span(name, tokens.callsign)
at: src/human/style.rs:39
quote: callsign: "\x1b[1m",
@row_add n=60 row=decision/no-handoff-blue-in-cli area=surface-cli rec=fct-dufqiv2l@1
name: No handoff blue in the CLI
rule: The CLI never uses the handoff blue colour.
@row_check n=60 row=decision/no-handoff-blue-in-cli sha=0f5248515522 v=holds judge=sonnet head=8834195b0683
searched: Tokens struct fields (attention, stale, callsign, reset) · in src/human/style.rs
searched: \x1b[34m|\x1b[94m|38;5;33|\[38;2 other than amber · in src/human/style.rs
@row_add n=61 row=decision/no-terminal-background area=surface-cli rec=fct-dufqiv2l@1
name: No background colours
rule: CLI output never sets a background colour.
why: the terminal background belongs to the user
@row_check n=61 row=decision/no-terminal-background sha=4a96be26046c v=holds judge=sonnet head=8834195b0683
searched: \x1b[4[0-7]m|\x1b[48; · in src/human/style.rs
searched: \x1b[48; · in src/human/*.rs
@row_add n=62 row=decision/ascii-mirror-words-always-print area=surface-cli rec=fct-w37zqv3v@1
name: ASCII mirror marks are words, not glyphs
rule: A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
@row_check n=62 row=decision/ascii-mirror-words-always-print sha=521ecec0e869 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:9
quote: format!("{boxed} {word:<MIRROR_WORD$}  {name}  {rest}\n")
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=63 row=contract/cli-colour-decision-order/json area=surface-cli rec=fct-vhpny3ot@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: --json
promise: First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
@row_check n=63 row=contract/cli-colour-decision-order/json sha=35b29fef067c v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62
quote: if facts.json
@row_add n=64 row=contract/cli-colour-decision-order/viola-run area=surface-cli rec=fct-hod2cupz@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: viola run
promise: Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
@row_check n=64 row=contract/cli-colour-decision-order/viola-run sha=c74961d91e1b v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:63
quote: || facts.passthrough
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
@row_add n=65 row=contract/cli-colour-decision-order/viola-hook area=surface-cli rec=fct-gu2srphh@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: viola hook
promise: Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
@row_check n=65 row=contract/cli-colour-decision-order/viola-hook sha=32485569b273 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:63
quote: || facts.passthrough
searched: eprintln! · in src/cmd/hook.rs src/cmd/hook/*.rs
searched: io::stderr · in src/cmd/hook.rs src/cmd/hook/*.rs
@row_add n=66 row=contract/cli-colour-decision-order/viola-mcp area=surface-cli rec=fct-gu2srphh@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: viola mcp
promise: Third match of the colour decision order: `viola mcp` prints no colour, glyph or SGR, and its stdout carries MCP frames only.
@row_check n=66 row=contract/cli-colour-decision-order/viola-mcp sha=ee4ec3715b9f v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:63
quote: || facts.passthrough
searched: println! · in src/cmd/mcp.rs crates/viola-mcp/src/**/*.rs
searched: \x1b · in src/cmd/mcp.rs
@row_add n=67 row=contract/cli-colour-decision-order/no-color area=surface-cli rec=fct-tgbprndw@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: NO_COLOR
promise: Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
@row_check n=67 row=contract/cli-colour-decision-order/no-color sha=47c8adc5abbf v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
at: src/human/style.rs:64
quote: || facts.no_color
@row_add n=68 row=contract/cli-colour-decision-order/term-dumb area=surface-cli rec=fct-6xnn7dqw@1,fct-w37zqv3v@1
boundary: CLI colour decision order
item: TERM=dumb
promise: Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
@row_check n=68 row=contract/cli-colour-decision-order/term-dumb sha=cbb1bae8dea6 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:65
quote: || facts.term.as_deref() == Some("dumb")
@row_add n=69 row=contract/cli-colour-decision-order/stdout-not-terminal area=surface-cli rec=fct-kbiy6u23@2,fct-w37zqv3v@1
boundary: CLI colour decision order
item: stdout is not a terminal
promise: Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
@row_check n=69 row=contract/cli-colour-decision-order/stdout-not-terminal sha=bb5652518f9b v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:66
quote: || !facts.stdout_tty
at: src/human/style.rs:98
quote: stdout_tty: io::stdout().is_terminal(),
@row_add n=70 row=contract/cli-colour-depth/truecolor area=surface-cli rec=fct-kvkzosks@1,fct-ukqwfmzr@1
boundary: CLI colour depth
item: truecolor
promise: Where colour applies, the depth is chosen once per process, first match wins: truecolor if `COLORTERM` is `truecolor` or `24bit`.
@row_check n=70 row=contract/cli-colour-depth/truecolor sha=a48b0666af0c v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:70
quote: if matches!(facts.colorterm.as_deref(), Some("truecolor" | "24bit")) {
@row_add n=71 row=contract/cli-colour-depth/256-colour area=surface-cli rec=fct-vzhczpnh@2,fct-ukqwfmzr@1
boundary: CLI colour depth
item: 256-colour
promise: Where colour applies and truecolor did not match, the depth is 256-colour if `TERM` contains `256color`, chosen once per process.
@row_check n=71 row=contract/cli-colour-depth/256-colour sha=b9399b94474e v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:75
quote: .is_some_and(|term| term.contains("256color"))
@row_add n=72 row=contract/cli-streams/stdout area=surface-cli rec=fct-rjt4bruj@1
boundary: CLI streams
item: stdout
promise: stdout carries results and data: tables, send/wait results, `last` text.
@row_check n=72 row=contract/cli-streams/stdout sha=1efc7b920519 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:66
quote: &mut io::stdout().lock(),
at: src/cmd/send.rs:115
quote: &mut io::stdout().lock(),
at: src/cmd/wait.rs:100
quote: let mut out = io::stdout().lock();
at: src/cmd/last.rs:80
quote: let _ = human::write_message(&mut io::stdout().lock(), message);
@row_add n=73 row=contract/cli-streams/stderr area=surface-cli rec=fct-zolhtb6x@1
boundary: CLI streams
item: stderr
promise: stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
@row_check n=73 row=contract/cli-streams/stderr sha=5a30121b49a1 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human/mirror.rs:64-66
quote: let mut text = format!("unable  {name}  {reason}\n"); if let Some(hint) = hint { text.push_str(&format!("hint: {hint}\n"));
@row_add n=74 row=contract/cli-refusal-line/two-space-form area=surface-cli rec=fct-zolhtb6x@1,fct-5xeughsz@1
boundary: CLI refusal line
item: two-space form
promise: A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
@row_check n=74 row=contract/cli-refusal-line/two-space-form sha=5ca9cdd9dbef v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
@row_add n=75 row=contract/cli-refusal-line/fixed-unable-form area=surface-cli rec=fct-zolhtb6x@1
boundary: CLI refusal line
item: fixed unable form
promise: The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
@row_check n=75 row=contract/cli-refusal-line/fixed-unable-form sha=e7ba9e943625 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/cmd/run.rs:212
quote: human::refuse(unable, hint);
at: src/cmd/verify.rs:99
quote: human::refuse(unable, hint);
at: src/cmd/list.rs:106
quote: human::refuse(unable, hint);
at: src/cmd/client.rs:38
quote: human::refuse(&unable, &hint);
@row_add n=76 row=contract/cli-refusal-line/hint area=surface-cli rec=fct-5xeughsz@1,fct-zolhtb6x@1
boundary: CLI refusal line
item: hint
promise: A refusal is followed directly by one `hint:` line per reason, detail or cause.
@row_check n=76 row=contract/cli-refusal-line/hint sha=184690179a89 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
@row_add n=77 row=decision/json-one-document-on-stdout area=surface-cli rec=fct-6xd7opxn@1
name: --json result is one stdout document
rule: Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code.
except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
@row_check n=77 row=decision/json-one-document-on-stdout sha=637b007ce36a v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:251
quote: if json {
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
at: src/cmd/send.rs:164
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
@row_add n=78 row=decision/silent-catch-sites area=surface-cli rec=fct-6xd7opxn@1
name: Silent catch sites
rule: At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
@row_check n=78 row=decision/silent-catch-sites sha=a8d46092f80e v=holds judge=sonnet head=8834195b0683
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=79 row=decision/no-hint-under-json area=surface-cli rec=fct-6xd7opxn@1,fct-5xeughsz@1
name: No hint line under --json
rule: No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
@row_check n=79 row=decision/no-hint-under-json sha=3e4d839449d7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:251-254
quote: if json {         unreachable_document(cause.cause());     } else {         human();
@row_add n=80 row=decision/truncate-only-unwrapped-name area=surface-cli rec=fct-rhkxdzp7@1,fct-fldutxrq@1
name: Only an unwrapped NAME is truncated
rule: Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
@row_check n=80 row=decision/truncate-only-unwrapped-name sha=2725789e9cc3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:63
quote: let width = viola_pty::host_size().map(|size| usize::from(size.cols));
at: src/human/board/cells.rs:125
quote: pub(super) fn cut(name: &str, column: usize) -> String {
@row_add n=81 row=decision/columns-never-read area=surface-cli rec=fct-rhkxdzp7@1
name: COLUMNS is never read
rule: Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
@row_check n=81 row=decision/columns-never-read sha=b5fc01e581bc v=holds judge=sonnet head=8834195b0683
searched: COLUMNS · in src/ (only a comment at src/cmd/list.rs:61 saying it is never read)
@row_add n=82 row=decision/lines-never-wrapped area=surface-cli rec=fct-fldutxrq@1
name: Lines never wrapped
rule: CLI output lines are never wrapped.
@row_check n=82 row=decision/lines-never-wrapped sha=ff6bc49c2c81 v=holds judge=sonnet head=8834195b0683
searched: textwrap|wrap_text · in src/
searched: \.wrap\(|fn wrap · in src/human/
@row_add n=83 row=decision/ascii-only-human-output area=surface-cli rec=fct-xlz4sgiu@1
name: ASCII-only human output
rule: Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
@row_check n=83 row=decision/ascii-only-human-output sha=d01deac14fbf v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
at: src/human/mirror.rs:15
quote: out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=84 row=contract/viola-list-table/columns area=surface-cli rec=fct-pcmgk4u5@1
boundary: viola list table
item: columns
promise: Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.
@row_check n=84 row=contract/viola-list-table/columns sha=d5a0ece40cd2 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: src/human/board/rows.rs:80
quote: const UNWRAPPED_SEPARATOR: &str = "-- UNWRAPPED - READ-ONLY --";
@row_add n=85 row=contract/viola-list-table/row-order area=surface-cli rec=fct-6pisha67@1
boundary: viola list table
item: row order
promise: Rows are ordered by name.
@row_check n=85 row=contract/viola-list-table/row-order sha=a8719abbdccd v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:95
quote: .sort_by(|(a, _), (b, _)| AsRef::<str>::as_ref(a).cmp(AsRef::<str>::as_ref(b)));
at: crates/viola-state/src/board.rs:346
quote: unwrapped.sort_by(|a, b| a.name.cmp(&b.name));
@row_add n=86 row=contract/viola-list-table/bay-line area=surface-cli rec=fct-dbxcldrz@1
boundary: viola list table
item: BAY line
promise: The BAY line is the ATIS without `resets` times (printed by `viola release <name> --budget`) and without a TAPE cell.
@row_check n=86 row=contract/viola-list-table/bay-line sha=276709982a99 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
@row_add n=87 row=decision/expired-window-shown-expired area=surface-cli rec=fct-dbxcldrz@1
name: Expired budget window shown as expired
rule: A budget window whose `resets_at` has passed still prints its figure followed by `expired`.
why: old information is shown as old
@row_check n=87 row=decision/expired-window-shown-expired sha=ef90f28b5f12 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:26
quote: let suffix = if expired { " expired" } else { "" };
@row_add n=88 row=decision/list-rows-escape-all-controls area=surface-cli rec=fct-hefy4zzc@1
name: list rows escape every control character
rule: Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line.
why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified
except: `wait` / `last` message text keeps `\n` and `\t`
@row_check n=88 row=decision/list-rows-escape-all-controls sha=8820ba425ffe v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:23
quote: if c.is_control() {
at: src/human/escape.rs:8
quote: if c.is_control() && c != '\n' && c != '\t' {
@row_add n=89 row=contract/viola-list-table/empty area=surface-cli rec=fct-7y3vb6h4@1
boundary: viola list table
item: empty
promise: With no wrapped sessions the board prints `no wrapped sessions  start one: viola run <name> -- claude`.
@row_check n=89 row=contract/viola-list-table/empty sha=d3355ffa320f v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:78
quote: const EMPTY_BOARD: &str = "no wrapped sessions  start one: viola run <name> -- claude";
@row_add n=90 row=contract/viola-send-mirror/open area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: open
promise: `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
@row_check n=90 row=contract/viola-send-mirror/open sha=715788bef5ef v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:15
quote: out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
at: src/cmd/send.rs:104
quote: let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
@row_add n=91 row=contract/viola-send-mirror/read-back area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: read back
promise: `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
@row_check n=91 row=contract/viola-send-mirror/read-back sha=acd4dc905003 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
at: src/cmd/send.rs:114-115
quote: let _ = human::write_read_back( &mut io::stdout().lock(),
@row_add n=92 row=contract/viola-send-mirror/unable area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: unable
promise: `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
@row_check n=92 row=contract/viola-send-mirror/unable sha=87986fc1ac0f v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
at: src/cmd/send.rs:141
quote: human::write_send_unable(&mut io::stderr().lock(), self.name, &shown, hint.as_deref());
@row_add n=93 row=contract/viola-send-mirror/unconfirmable area=surface-cli rec=fct-ahmeujab@1
boundary: viola send readback mirror
item: unconfirmable
promise: `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
@row_check n=93 row=contract/viola-send-mirror/unconfirmable sha=7156e558fa57 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:33
quote: out.write_all(mirror_line("[  ]", "unconfirmable", name, note).as_bytes())
at: src/cmd/send.rs:128
quote: let _ = human::write_send_unconfirmable(&mut io::stdout().lock(), self.name);
@row_add n=94 row=contract/cli-strict-modes-refusal/instance-state-pair area=surface-cli rec=fct-5xeughsz@1
boundary: CLI strict-modes refusal
item: instance-state pair
promise: The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
@row_check n=94 row=contract/cli-strict-modes-refusal/instance-state-pair sha=f5a44623255d v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:37
quote: let (unable, hint) = human::instance_state_pair(name);
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
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=95 row=contract/cli-start-refusal/fixed-message area=surface-cli rec=fct-5xeughsz@1,fct-w4pl2fni@1
boundary: run and revive start refusals
item: fixed message
promise: A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
@row_check n=95 row=contract/cli-start-refusal/fixed-message sha=343cd2404c65 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:79-81
quote: fn pair(self, name: &str) -> (String, String) { match self { Self::StrictModes => human::instance_state_pair(name),
at: src/cmd/revive.rs:176-177
quote: if let Some(refusal) = holder() { return Ok(Err(refusal));
at: src/human.rs:42
quote: pub(crate) fn refuse(unable: &str, hint: &str) {
@row_add n=96 row=decision/no-hint-names-release area=surface-cli rec=fct-5xeughsz@1
name: No hint names viola release
rule: No hint names `viola release`.
why: a driver's `release` is refused
@row_check n=96 row=decision/no-hint-names-release sha=4f62f371b1d5 v=holds judge=sonnet head=8834195b0683
searched: release · in src/human/hints.rs (every hint string of send_hint and answer_hint, none names it)
searched: hint · in src/cmd/**/*.rs, src/human/**/*.rs (the other hints are fixed pairs from human::home_state_pair, instance_state_pair, revive pair)
@row_add n=97 row=decision/hint-never-quotes-text area=surface-cli rec=fct-syupqkpm@1
name: Hints never quote text
rule: A hint never quotes the sent text or any upstream text.
@row_check n=97 row=decision/hint-never-quotes-text sha=53b7bbd7251a v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:48
quote: "turn-running" => format!("a turn is running; viola wait {name} first"),
@row_add n=98 row=contract/viola-wait-output/waiting-line area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: waiting line
promise: `wait` prints one static `waiting: <name>` line on stderr, TTY only, before its one stdout result line.
@row_check n=98 row=contract/viola-wait-output/waiting-line sha=7626c3379117 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:69-70
quote: if !args.json && io::stderr().is_terminal() { let _ = human::write_waiting(&mut io::stderr().lock(), name);
at: src/human/wait.rs:9
quote: out.write_all(format!("waiting: {name}\n").as_bytes())
@row_add n=99 row=contract/viola-wait-output/event-form area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: event form
promise: `turn-ended`, `session-end` and every other kind without a `dialog_id` print `<kind>  <name>  <time>  cursor <n>` on stdout.
@row_check n=99 row=contract/viola-wait-output/event-form sha=36265a49a4bc v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:20
quote: let line = format!("{kind}  {name}  {at}  cursor {cursor}\n");
at: src/cmd/wait.rs:115
quote: let _ = human::write_woken(&mut out, kind, name, at, cursor);
@row_add n=100 row=contract/viola-wait-output/dialog-form area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: dialog form
promise: `question`, `permission`, `plan` and `dialog-expired` print `<kind>  <name>  dialog <id>  cursor <n>` on stdout, uncoloured, with `dialog unknown` when the event carries no `dialog_id`.
@row_check n=100 row=contract/viola-wait-output/dialog-form sha=184432c6d0e5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:36
quote: const DIALOG_KINDS: [&str; 4] = ["question", "permission", "plan", "dialog-expired"];
at: src/human/wait.rs:32
quote: let line = format!("{kind}  {name}  dialog {dialog}  cursor {cursor}\n");
at: src/cmd/wait.rs:111
quote: .map_or_else(|| "unknown".to_owned(), |id| id.to_string());
@row_add n=101 row=contract/viola-wait-output/timed-out area=surface-cli rec=fct-iee6axpq@1
boundary: viola wait output
item: timed out
promise: A timeout prints `timed out  <name>  <ms> ms` on stdout.
@row_check n=101 row=contract/viola-wait-output/timed-out sha=b08f2e2049fe v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:38
quote: out.write_all(format!("timed out  {name}  {ms} ms\n").as_bytes())
@row_add n=102 row=decision/no-spinner area=surface-cli rec=fct-qr5hytcv@1
name: No spinner or elapsed-time counter
rule: The CLI shows no spinner and no elapsed-time counter.
@row_check n=102 row=decision/no-spinner sha=8855fd83622c v=holds judge=sonnet head=8834195b0683
searched: spinner|indicatif|elapsed counter · in src/**/*.rs, Cargo.toml, crates/*/src/**/*.rs, crates/*/Cargo.toml
@row_add n=103 row=contract/viola-last-output/header area=surface-cli rec=fct-j6urnvep@1
boundary: viola last output
item: header
promise: `last` prints `last  <name>  <kind> <time>` on stderr, TTY only.
@row_check n=103 row=contract/viola-last-output/header sha=60c70e4a354c v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:43
quote: let line = format!("last  {name}  turn-ended {at}\n");
at: src/cmd/last.rs:73-74
quote: if let Some(ts) = ok["ts"].as_str() &&   io::stderr().is_terminal()
@row_add n=104 row=contract/viola-last-output/message area=surface-cli rec=fct-j6urnvep@1
boundary: viola last output
item: message
promise: `last` prints the message text on stdout with C0/C1 escaped, or `no message` on stderr when the message is `null`.
@row_check n=104 row=contract/viola-last-output/message sha=3eb3e0eae059 v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:49
quote: let mut shown = escape_message(message);
at: src/human/wait.rs:56
quote: out.write_all(b"no message\n")
at: src/cmd/last.rs:83
quote: let _ = human::write_no_message(&mut io::stderr().lock());
@row_add n=105 row=contract/cli-message-escaping/control-characters area=surface-cli rec=fct-gbzjruf2@1
boundary: CLI message-mode escaping
item: control characters
promise: In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
@row_check n=105 row=contract/cli-message-escaping/control-characters sha=729123bbc115 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:8-9
quote: if c.is_control() && c != '\n' && c != '\t' { shown.push_str(&format!("\\x{:02X}", u32::from(c)));
at: src/human/verbs.rs:87
quote: Err(_) => escape_message(since),
at: src/human/wait.rs:21
quote: out.write_all(escape_message(&line).as_bytes())
@row_add n=106 row=contract/cli-message-escaping/json area=surface-cli rec=fct-gbzjruf2@1
boundary: CLI message-mode escaping
item: --json
promise: `--json` output stays serde-escaped.
@row_check n=106 row=contract/cli-message-escaping/json sha=5a796396dad5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:221
quote: let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
@row_add n=107 row=contract/cli-verb-output/pause area=surface-cli rec=fct-s5g62jbv@1
boundary: CLI verb output
item: viola pause
promise: `viola pause <name>` prints `<name>  wheel human  manual-pause  I have control`.
@row_check n=107 row=contract/cli-verb-output/pause sha=27acd8ece72c v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:17
quote: out.write_all(format!("{name}  wheel human  manual-pause  I have control\n").as_bytes())
@row_add n=108 row=contract/cli-verb-output/release area=surface-cli rec=fct-3wxbibgp@1
boundary: CLI verb output
item: viola release
promise: `viola release <name>` prints `<name>  wheel driver  you have control`.
@row_check n=108 row=contract/cli-verb-output/release sha=69d7828aeb9b v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:22
quote: out.write_all(format!("{name}  wheel {holder}  you have control\n").as_bytes())
@row_add n=109 row=contract/cli-verb-output/release-budget area=surface-cli rec=fct-ybxjcc2w@1
boundary: CLI verb output
item: viola release --budget
promise: Prints one unstyled stdout line, `<name>  budget gate lifted until <time>  wheel human` when a pause stood, else `<name>  budget gate open  wheel human`.
@row_check n=109 row=contract/cli-verb-output/release-budget sha=db985fd30e0f v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:49
quote: out.write_all(format!("{name}  budget gate lifted until {end}  wheel {holder}\n").as_bytes())
at: src/human/verbs.rs:54
quote: out.write_all(format!("{name}  budget gate open  wheel {holder}\n").as_bytes())
@row_add n=110 row=contract/cli-verb-output/link area=surface-cli rec=fct-glxbi6ql@1
boundary: CLI verb output
item: viola link
promise: `viola link <a> <b>` prints `link    <a> -> <b>  since <time>`, or `link    <a> -> <b>  already linked` for an existing pair.
@row_check n=110 row=contract/cli-verb-output/link sha=2e85ef82be96 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/verbs.rs:74
quote: out.write_all(format!("link    {driver} -> {driven}  already linked\n").as_bytes())
@row_add n=111 row=contract/cli-verb-output/unlink area=surface-cli rec=fct-vmpiq5lh@1
boundary: CLI verb output
item: viola unlink
promise: `viola unlink <a> <b>` prints `unlink  <a> -> <b>`.
@row_check n=111 row=contract/cli-verb-output/unlink sha=15a66d0fd281 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:79
quote: out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
@row_add n=112 row=contract/cli-verb-output/answer area=surface-cli rec=fct-b2gogylx@1
boundary: CLI verb output
item: viola answer
promise: `viola answer <name> <id>` prints `answered  <name>  dialog <id>`, or `unable  <name>  not-delivered  unknown-dialog`.
@row_check n=112 row=contract/cli-verb-output/answer sha=b9bddaefd84a v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:12
quote: out.write_all(format!("answered  {name}  dialog {dialog_id}\n").as_bytes())
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/run/dialog.rs:736
quote: Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
@row_add n=113 row=contract/cli-verb-output/plugin-install area=surface-cli rec=fct-65srfuid@1
boundary: CLI verb output
item: viola plugin install
promise: Prints four plain ASCII stdout lines in one write, no SGR: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`; `<dir>` is quoted only where the host's shell would split it.
@row_check n=113 row=contract/cli-verb-output/plugin-install sha=286c199986e9 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:97
quote: "viola@viola  written  {dir}\n",
at: src/human/verbs.rs:99
quote: "  claude plugin marketplace add {dir}\n",
at: src/human/verbs.rs:100
quote: "  claude plugin install viola@viola\n",
at: src/human/verbs.rs:113
quote: .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':'));
@row_add n=114 row=decision/step-counter-only-progress-pattern area=surface-cli rec=fct-nz6d6pbn@1
name: Step counter as the one progress pattern
rule: The step counter of `verify` is the project's one progress pattern.
@row_check n=114 row=decision/step-counter-only-progress-pattern sha=af4a4eed89e8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:486
quote: human::result(&step_line(n, total, row, pass));
@row_add n=115 row=contract/viola-verify-output/summary area=surface-cli rec=fct-nz6d6pbn@1
boundary: viola verify output
item: summary
promise: A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count.
@row_check n=115 row=contract/viola-verify-output/summary sha=5842906cb814 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:293-296
quote: human::result(&format!( "stamped {version}  {} pass  {failed} fail", results.len() - failed ));
@row_add n=116 row=contract/viola-verify-output/stopped-or-refused area=surface-cli rec=fct-nz6d6pbn@1
boundary: viola verify output
item: stopped or refused run
promise: A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.
@row_check n=116 row=contract/viola-verify-output/stopped-or-refused sha=b13e3a495565 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
at: src/cmd/verify.rs:83-87
quote: human::refuse( "verify was stopped before it finished", "run viola verify again to stamp this CLI", ); (1, Some("interrupted"))
@row_add n=117 row=contract/viola-verify-output/stamps-refused area=surface-cli rec=fct-nz6d6pbn@1
boundary: viola verify output
item: stamps.json update refused
promise: A `stamps.json` update refused by its strict-modes check prints the home-state `unable:` / `hint:` pair.
@row_check n=117 row=contract/viola-verify-output/stamps-refused sha=0d10d5fde65e v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:286-287
quote: Err(StampsError::StrictModes(_)) => { refuse_home();
@row_add n=118 row=contract/viola-run-output/child-started area=surface-cli rec=fct-w4pl2fni@1,fct-2t54hjhs@2
boundary: viola run and revive output
item: child started
promise: `run` and `revive` print nothing once the child starts.
@row_check n=118 row=contract/viola-run-output/child-started sha=476b3266c314 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
searched: println!|print!\(|stdout\(\) · in src/cmd/run.rs and src/cmd/revive.rs (run: only the child's pump Tee; revive: only the --list rows)
@row_add n=119 row=contract/viola-revive-output/list area=surface-cli rec=fct-2t54hjhs@2
boundary: viola revive output
item: --list
promise: `revive --list` prints one plain stdout line per logged session.
@row_check n=119 row=contract/viola-revive-output/list sha=9c8824385ed2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:295
quote: human::result(&row.line());
@row_add n=120 row=contract/revive-json-refusal-code/strict-modes-failed area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: strict-modes-failed
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=120 row=contract/revive-json-refusal-code/strict-modes-failed sha=be21aad697ad v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=121 row=contract/revive-json-refusal-code/already-live area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: already-live
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=121 row=contract/revive-json-refusal-code/already-live sha=7337540220c9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=122 row=contract/revive-json-refusal-code/no-session area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: no-session
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=122 row=contract/revive-json-refusal-code/no-session sha=ff9fa7b7273e v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=123 row=contract/revive-json-refusal-code/session-live area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: session-live
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=123 row=contract/revive-json-refusal-code/session-live sha=2115e92849f0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=124 row=contract/revive-json-refusal-code/cwd-missing area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: cwd-missing
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=124 row=contract/revive-json-refusal-code/cwd-missing sha=a98970527bd2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=125 row=contract/revive-json-refusal-code/stale-heartbeat area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: stale-heartbeat
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=125 row=contract/revive-json-refusal-code/stale-heartbeat sha=1eb68252d888 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=126 row=contract/revive-json-refusal-code/pinned-hash-mismatch area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: pinned-hash-mismatch
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=126 row=contract/revive-json-refusal-code/pinned-hash-mismatch sha=80ae0699930e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=127 row=contract/revive-json-refusal-code/squatted-name area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: squatted-name
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=127 row=contract/revive-json-refusal-code/squatted-name sha=2554faf12cfd v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=128 row=contract/revive-json-refusal-code/socket-dir-refused area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: socket-dir-refused
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=128 row=contract/revive-json-refusal-code/socket-dir-refused sha=f0e7df8a0511 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=129 row=contract/revive-json-refusal-code/internal-error area=surface-cli rec=fct-2t54hjhs@2
boundary: revive --json refusal code
item: internal-error
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_check n=129 row=contract/revive-json-refusal-code/internal-error sha=0fa42eaf2902 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:124
quote: client::refused_document("internal-error");
@row_add n=130 row=contract/cli-exit-codes/10-14 area=surface-cli rec=fct-tj4jq2zv@1
boundary: CLI exit codes
item: 10 to 14
promise: An `unable` refusal with its reason; a refusal object under `--json`.
@row_check n=130 row=contract/cli-exit-codes/10-14 sha=0b06c72e5aa4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
at: src/cmd/send.rs:133
quote: return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
at: src/cmd/send.rs:141
quote: human::write_send_unable(&mut io::stderr().lock(), self.name, &shown, hint.as_deref());
@row_add n=131 row=contract/exit-21-cause/strict-modes-failed area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: strict-modes-failed
promise: One of the closed set of causes named in exit 21's `detail`.
@row_check n=131 row=contract/exit-21-cause/strict-modes-failed sha=6b65b7ba0a32 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=132 row=contract/exit-21-cause/instance-dead area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: instance-dead
promise: One of the closed set of causes named in exit 21's `detail`.
@row_check n=132 row=contract/exit-21-cause/instance-dead sha=94a9b8f8fa7a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: src/cmd/client.rs:119
quote: Self::Dead => UnreachableCause::InstanceDead,
@row_add n=133 row=contract/exit-21-cause/unwrapped-name area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: unwrapped-name
promise: One of the closed set of causes named in exit 21's `detail`.
@row_check n=133 row=contract/exit-21-cause/unwrapped-name sha=5a6d1678fad3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: src/cmd/client.rs:120
quote: Self::Unwrapped => UnreachableCause::UnwrappedName,
@row_add n=134 row=contract/exit-21-cause/endpoint-lost area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: endpoint-lost
promise: One of the closed set of causes named in exit 21's `detail`.
@row_check n=134 row=contract/exit-21-cause/endpoint-lost sha=d0d13b2e871d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: src/cmd/client.rs:200
quote: return Ok(Err(Unreachable::Lost));
@row_add n=135 row=contract/exit-21-cause/server-verify-failed area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: server-verify-failed
promise: One of the closed set of causes named in exit 21's `detail`.
@row_check n=135 row=contract/exit-21-cause/server-verify-failed sha=05b7d89ff88b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: src/cmd/client.rs:194
quote: Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
@row_add n=136 row=contract/exit-21-cause/socket-dir-refused area=surface-cli rec=fct-tj4jq2zv@1
boundary: exit 21 cause
item: socket-dir-refused
promise: One of the closed set of causes named in exit 21's `detail`.
@row_check n=136 row=contract/exit-21-cause/socket-dir-refused sha=47b25950bac2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
@row_add n=137 row=contract/cli-strict-modes-refusal/plugin-install area=surface-cli rec=fct-t6bk3igt@1
boundary: CLI strict-modes refusal
item: viola plugin install
promise: A refused strict-modes check of `viola plugin install` exits 21 with the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty.
@row_check n=137 row=contract/cli-strict-modes-refusal/plugin-install sha=f55d29003f4e v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:55
quote: client::unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/plugin.rs:57
quote: let (unable, hint) = human::home_state_pair();
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
@row_add n=138 row=contract/strict-modes-refusal/exit-21 area=surface-cli rec=fct-ui5ydw4a@1
boundary: Instance strict-modes refusal
item: exit 21
promise: A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause` and `release`, is made before any frame, exits 21, and writes no frame and no line.
@row_check n=138 row=contract/strict-modes-refusal/exit-21 sha=2b97d96c0d56 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
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
at: src/cmd/unlink.rs:12
quote: use super::link::{LinkArgs, Verb, link_call};
@row_add n=139 row=contract/strict-modes-refusal/human-output area=surface-cli rec=fct-ui5ydw4a@1
boundary: Instance strict-modes refusal
item: unable: / hint: pair
promise: The refusal prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does.
@row_check n=139 row=contract/strict-modes-refusal/human-output sha=b0e29d51c0f4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:37
quote: let (unable, hint) = human::instance_state_pair(name);
@row_add n=140 row=contract/strict-modes-refusal/json-document area=surface-cli rec=fct-ui5ydw4a@1
boundary: Instance strict-modes refusal
item: --json document
promise: Under `--json` the refusal prints the `instance-unreachable` document with `detail` `strict-modes-failed`, and stderr stays empty.
@row_check n=140 row=contract/strict-modes-refusal/json-document sha=00c2e44669fc v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=141 row=decision/flat-cli-verbs area=surface-cli rec=fct-jcytj7ki@1
name: Flat CLI verbs
rule: CLI verbs are flat, each one lower-case word, with no nested subcommand groups.
except: `plugin install`
@row_check n=141 row=decision/flat-cli-verbs sha=420550ce7b75 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:60
quote: enum Command {
@row_add n=142 row=name/command/run area=surface-cli rec=fct-jcytj7ki@1
class: command
name: run
@row_check n=142 row=name/command/run sha=f5bee6f7f3ac v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:62
quote: Run(run::RunArgs),
@row_add n=143 row=name/command/revive area=surface-cli rec=fct-jcytj7ki@1
class: command
name: revive
@row_check n=143 row=name/command/revive sha=067e88eaa1e7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:64
quote: Revive(revive::ReviveArgs),
@row_add n=144 row=name/command/send area=surface-cli rec=fct-jcytj7ki@1
class: command
name: send
@row_check n=144 row=name/command/send sha=a2d95d26750d v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:66
quote: Send(send::SendArgs),
@row_add n=145 row=name/command/wait area=surface-cli rec=fct-jcytj7ki@1
class: command
name: wait
@row_check n=145 row=name/command/wait sha=d3638f1b4826 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:68
quote: Wait(wait::WaitArgs),
@row_add n=146 row=name/command/last area=surface-cli rec=fct-jcytj7ki@1
class: command
name: last
@row_check n=146 row=name/command/last sha=f56cf3e5e58a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:70
quote: Last(last::LastArgs),
@row_add n=147 row=name/command/list area=surface-cli rec=fct-jcytj7ki@1
class: command
name: list
@row_check n=147 row=name/command/list sha=c98e05048bed v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:74
quote: List(list::ListArgs),
@row_add n=148 row=name/command/answer area=surface-cli rec=fct-jcytj7ki@1
class: command
name: answer
@row_check n=148 row=name/command/answer sha=3fa23fa2465a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:72
quote: Answer(answer::AnswerArgs),
@row_add n=149 row=name/command/verify area=surface-cli rec=fct-jcytj7ki@1
class: command
name: verify
@row_check n=149 row=name/command/verify sha=2a022382631c v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:88
quote: Verify(verify::VerifyArgs),
@row_add n=150 row=name/command/pause area=surface-cli rec=fct-jcytj7ki@1
class: command
name: pause
@row_check n=150 row=name/command/pause sha=25dc804a6ef9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:76
quote: Pause(pause::PauseArgs),
@row_add n=151 row=name/command/release area=surface-cli rec=fct-jcytj7ki@1
class: command
name: release
@row_check n=151 row=name/command/release sha=e5bfafd9b038 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:78
quote: Release(release::ReleaseArgs),
@row_add n=152 row=name/command/link area=surface-cli rec=fct-jcytj7ki@1
class: command
name: link
@row_check n=152 row=name/command/link sha=fdb835b87e21 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:80
quote: Link(link::LinkArgs),
@row_add n=153 row=name/command/unlink area=surface-cli rec=fct-jcytj7ki@1
class: command
name: unlink
@row_check n=153 row=name/command/unlink sha=8a9f578a2c4a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:82
quote: Unlink(link::LinkArgs),
@row_add n=154 row=name/command/plugin-install area=surface-cli rec=fct-jcytj7ki@1
class: command
name: plugin install
@row_check n=154 row=name/command/plugin-install sha=8783b7425762 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:33
quote: Install(InstallArgs),
@row_add n=155 row=decision/no-interactive-prompt-or-pager area=surface-cli rec=fct-3l74p2hv@2
name: No interactive prompt or pager
rule: No verb has an interactive prompt or a pager.
@row_check n=155 row=decision/no-interactive-prompt-or-pager sha=cddcc7ef16a9 v=holds judge=sonnet head=8834195b0683
searched: pager|PAGER|read_line\(|dialoguer|rustyline|inquire · in src/**/*.rs
searched: indicatif|spinner|crossterm|ratatui|progress · in Cargo.toml and src/**/*.rs
@row_add n=156 row=decision/fleet-console-not-verb-output area=surface-cli rec=fct-3l74p2hv@2,fct-izuy43cb@2
name: Fleet console is no verb's output
rule: The terminal fleet console is no verb's output, so verb output rules do not govern its redraw.
@row_check n=156 row=decision/fleet-console-not-verb-output sha=e3142d6c0ece v=unverifiable judge=sonnet head=8834195b0683
searched: console · in src/cmd/mod.rs (no console verb among the Command variants, lines 59-97)
why: no evidence found
@row_add n=157 row=contract/help-output/verb-groups area=surface-cli rec=fct-yh7gnttc@1
boundary: viola --help output
item: verb groups
promise: `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.
@row_check n=157 row=contract/help-output/verb-groups sha=92cf60120c3b v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
at: src/cmd/mod.rs:40
quote: Options:
@row_add n=158 row=decision/prompt-text-from-stdin-or-file area=surface-cli rec=fct-7u3hdiu2@2
name: Prompt text from stdin or file only
rule: Prompt text comes only from stdin or `--file`.
@row_check n=158 row=decision/prompt-text-from-stdin-or-file sha=cab04e2aad81 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:48
quote: Err("the text is read from stdin or --file, never from an argument".to_owned())
at: src/cmd/send.rs:161
quote: None => read_text(io::stdin().lock()),
at: src/cmd/answer.rs:88
quote: None => read_response(io::stdin().lock()),
@row_add n=159 row=contract/hook-verb/stderr area=surface-cli rec=fct-cj5433yq@1
boundary: The hook verb
item: stderr
promise: `hook` writes nothing to stderr.
@row_check n=159 row=contract/hook-verb/stderr sha=e43ba98cc550 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
at: src/main.rs:66
quote: if prints_internal_error(role, outcome) {
@row_add n=160 row=contract/hook-verb/exit-code area=surface-cli rec=fct-cj5433yq@1
boundary: The hook verb
item: exit code
promise: `hook` always exits 0.
@row_check n=160 row=contract/hook-verb/exit-code sha=27b3062e59db v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=161 row=contract/mcp-verb/stdout area=surface-cli rec=fct-cj5433yq@1
boundary: The mcp verb
item: stdout
promise: `mcp` writes only MCP frames on stdout.
@row_check n=161 row=contract/mcp-verb/stdout sha=28774239acab v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:76
quote: let (stdin, stdout) = rmcp::transport::stdio();
@row_add n=162 row=decision/mcp-hidden-verb area=surface-cli rec=fct-cj5433yq@1
name: mcp is a hidden verb
rule: `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row.
why: `mcp` is run by the plugin, never by a person.
@row_check n=162 row=decision/mcp-hidden-verb sha=b4e17b892178 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:95
quote: #[command(hide = true)]
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
@row_add n=163 row=name/command/hook area=surface-cli rec=fct-cj5433yq@1
class: command
name: hook
@row_check n=163 row=name/command/hook sha=e613d3cb59fb v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:93
quote: Hook(hook::HookArgs),
@row_add n=164 row=name/command/mcp area=surface-cli rec=fct-cj5433yq@1
class: command
name: mcp
@row_check n=164 row=name/command/mcp sha=d4027effb89e v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:96
quote: Mcp,
@row_add n=165 row=decision/no-stack-traces area=surface-cli rec=fct-zumogwyu@1,fct-wc7eewp7@1
name: No stack traces
rule: Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
@row_check n=165 row=decision/no-stack-traces sha=8d5a1e8bbb44 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=166 row=decision/colour-carries-meaning area=anti-patterns-never-do-these rec=fct-zrpewlh4@1
name: Colour carries meaning, never decoration
rule: Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used.
why: Any third hue has no domain meaning.
@row_check n=166 row=decision/colour-carries-meaning sha=aa758237cf1f v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:32
quote: Self::Truecolor => "\x1b[38;2;217;119;6m",
at: src/human/style.rs:38
quote: stale: "\x1b[2m",
@row_add n=167 row=decision/no-convergent-safe-choices area=anti-patterns-never-do-these rec=fct-gdfaropc@1
name: No convergence on safe design choices
rule: The design never converges on common safe choices across generations; the readback box and the cocked strip stand in their place.
why: Agent dashboards converge on dots, toasts and graphs.
@row_check n=167 row=decision/no-convergent-safe-choices sha=ba8cd364e70f v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=168 row=decision/no-traffic-light-status-dots area=anti-patterns-never-do-these rec=fct-ltouy2sa@1
name: No traffic-light status dots
rule: Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word.
why: A strip prints its status word in a fixed column.
@row_check n=168 row=decision/no-traffic-light-status-dots sha=d78af04651e8 v=holds judge=sonnet head=8834195b0683
searched: \x1b · in src (non-test code: only the amber, dim, bold and reset tokens in src/human/style.rs)
searched: ●|green|red SGR dot status · in src/human
@row_add n=169 row=decision/no-optimistic-send-confirmation area=anti-patterns-never-do-these rec=fct-jkj5umki@1
name: No optimistic send confirmation
rule: The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast.
why: A clearance does not count until it is read back.
@row_check n=169 row=decision/no-optimistic-send-confirmation sha=26d08046f783 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:104
quote: let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
at: src/cmd/send.rs:114
quote: let _ = human::write_read_back(
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
@row_add n=170 row=decision/no-auto-sorting-strips area=anti-patterns-never-do-these rec=fct-m4eli7qh@2
name: No auto-sorting of strips
rule: Strips needing input or with recent sessions are never re-ranked to the top; a strip is cocked in place.
why: Re-ranking breaks the founder's memory of where each session lives, and the `DIALOG` word already says "needs you".
@row_check n=170 row=decision/no-auto-sorting-strips sha=353c67c128d2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:95
quote: .sort_by(|(a, _), (b, _)| AsRef::<str>::as_ref(a).cmp(AsRef::<str>::as_ref(b)));
at: crates/viola-state/src/board.rs:346
quote: unwrapped.sort_by(|a, b| a.name.cmp(&b.name));
@row_add n=171 row=decision/budget-as-ruled-figures area=anti-patterns-never-do-these rec=fct-szsoezvc@1
name: Budget as figures in ruled fields
rule: Budget prints as figures in ruled fields with the ATIS reading age, never as rings or coloured bars; `budget-paused` is a word, not a colour.
@row_check n=171 row=decision/budget-as-ruled-figures sha=7004d38981a5 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:52
quote: window_cell("5H", &budget.reading.reading.five_hour, now),
at: src/human/board/bay.rs:38
quote: Some(at) => format!("read {}m ago", (now - at.with_timezone(&Utc)).num_minutes()),
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
@row_add n=172 row=decision/wheel-as-words area=anti-patterns-never-do-these rec=fct-cr4x4576@1
name: Wheel shown as words
rule: The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`.
why: Taking control is never implied by a switch.
@row_check n=172 row=decision/wheel-as-words sha=1d92bb94e568 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:16
quote: let holder = item.wheel.map_or("unknown", Wheel::as_str);
at: crates/viola-state/src/snapshot.rs:31
quote: Self::Driver => "driver",
at: crates/viola-state/src/snapshot.rs:32
quote: Self::Human => "human",
@row_add n=173 row=decision/no-shimmer-or-glow-theme area=anti-patterns-never-do-these rec=fct-tj74erhr@1
name: No skeleton shimmer or glow theme
rule: No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once.
why: Nothing glows in a tower cab unless the lamp is on it.
@row_check n=173 row=decision/no-shimmer-or-glow-theme sha=10274ee02b7c v=holds judge=sonnet head=8834195b0683
searched: shimmer|skeleton|neon|glow · in src
searched: \x1b · in src (only the four tokens of src/human/style.rs, no background or green)
@row_add n=174 row=decision/no-check-glyph-or-emoji-prefixes area=anti-patterns-never-do-these rec=fct-pbywd7ut@1
name: No check-mark or emoji prefixes
rule: Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
@row_check n=174 row=decision/no-check-glyph-or-emoji-prefixes sha=680f65b474b2 v=holds judge=sonnet head=8834195b0683
searched: ✓|✗ · in src
searched: emoji range [\x{1F300}-\x{1FAFF}] · in src (one hit, a test string in src/human/escape.rs)
@row_add n=175 row=decision/colour-only-as-second-cue area=anti-patterns-never-do-these rec=fct-ibtjtsuk@1,fct-zdmbmsfz@1
name: Colour only as a second cue
rule: Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them.
why: The quiz confirms colour as a second cue only.
@row_check n=175 row=decision/colour-only-as-second-cue sha=709f255a3f2e v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
@row_add n=176 row=decision/plain-output-modes area=anti-patterns-never-do-these rec=fct-jphnedpy@1,fct-e37xe7yc@1
name: Plain output modes
rule: No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`.
why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
@row_check n=176 row=decision/plain-output-modes sha=50be05e6435f v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62
quote: if facts.json
at: src/human/style.rs:64
quote: || facts.no_color
at: src/human/style.rs:65
quote: || facts.term.as_deref() == Some("dumb")
at: src/human/style.rs:66
quote: || !facts.stdout_tty
at: src/cmd/list.rs:64
quote: let colour = human::read_colour(false, false);
@row_add n=177 row=decision/no-spinners-in-verb-output area=anti-patterns-never-do-these rec=fct-izuy43cb@2
name: No spinners in verb output
rule: A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines.
why: A spinner is progress presumed before it is confirmed.
@row_check n=177 row=decision/no-spinners-in-verb-output sha=dcd533d52b63 v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:9
quote: out.write_all(format!("waiting: {name}\n").as_bytes())
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
searched: spinner · in src
@row_add n=178 row=decision/stdout-data-stderr-messages area=anti-patterns-never-do-these rec=fct-oeutfz7w@1
name: Data on stdout, messages on stderr
rule: Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
@row_check n=178 row=decision/stdout-data-stderr-messages sha=e81d436d23d4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/last.rs:80
quote: let _ = human::write_message(&mut io::stdout().lock(), message);
at: src/cmd/wait.rs:70
quote: let _ = human::write_waiting(&mut io::stderr().lock(), name);
at: src/cmd/send.rs:141
quote: human::write_send_unable(&mut io::stderr().lock(), self.name, &shown, hint.as_deref());
at: src/cmd/client.rs:242
quote: let _ = human::write_wrapper_fault(&mut io::stderr().lock(), code);
@row_add n=179 row=decision/no-internals-in-errors area=anti-patterns-never-do-these rec=fct-b7z6jyxk@1
name: No internals in errors or hints
rule: Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources.
why: The security plan's NEVER-log floor.
@row_check n=179 row=decision/no-internals-in-errors sha=2ef7634588a6 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=180 row=decision/escape-controls-in-human-output area=anti-patterns-never-do-these rec=fct-b7z6jyxk@1
name: Control characters escaped in human output
rule: Human output of `last` and `list` escapes C0/C1 control characters.
@row_check n=180 row=decision/escape-controls-in-human-output sha=584cf50fbb2b v=holds judge=sonnet head=8834195b0683
at: src/human/wait.rs:49
quote: let mut shown = escape_message(message);
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
@row_add n=181 row=decision/confirmations-carry-context area=anti-patterns-never-do-these rec=fct-blptucet@1
name: Confirmations carry context
rule: Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
@row_check n=181 row=decision/confirmations-carry-context sha=30640a7b8d6c v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:25-26
quote: let rest = format!("{submitted}  cursor {cursor}"); out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=182 row=decision/no-hardcoded-widths-or-wrapping area=anti-patterns-never-do-these rec=fct-olwc37rq@1
name: No hardcoded widths or wrapping
rule: Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
@row_check n=182 row=decision/no-hardcoded-widths-or-wrapping sha=27eac04bac15 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:63
quote: let width = viola_pty::host_size().map(|size| usize::from(size.cols));
@row_add n=183 row=decision/swap-test area=self-validation-protocol rec=fct-gzgahwnt@1,fct-xohxr7g5@1
name: Swap test
rule: Before UI output is presented, a design that would not feel meaningfully different with a default typeface, default colours and a standard layout is redone with intent.
@row_check n=183 row=decision/swap-test sha=c53c82b83fda v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=184 row=decision/squint-test area=self-validation-protocol rec=fct-3fbi5jyf@1,fct-xohxr7g5@1
name: Squint test
rule: Before UI output is presented, hierarchy must still be perceivable with blurred eyes and nothing may jump out harshly.
except: `open` and `unconfirmable` readback boxes look identical under the squint, and only the word cell tells them apart, as intended.
@row_check n=184 row=decision/squint-test sha=c057dd0d7d50 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=185 row=decision/readback-box-signature-check area=self-validation-protocol rec=fct-4be5q2ri@2,fct-xohxr7g5@1
name: Readback box signature check
rule: Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.
@row_check n=185 row=decision/readback-box-signature-check sha=e019cb4b4308 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=186 row=decision/domain-not-statistical-design area=self-validation-protocol rec=fct-g3srhwja@1,fct-xohxr7g5@1
name: Design from the product's domain
rule: Before UI output is presented, an interface another AI would produce from a similar prompt fails; the interface must emerge from this product's domain exploration.
@row_check n=186 row=decision/domain-not-statistical-design sha=6bd8f722c040 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=187 row=decision/cli-tty-static-expression area=brand-identity rec=fct-zdmbmsfz@1
name: Static expression for the human CLI
rule: Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
@row_check n=187 row=decision/cli-tty-static-expression sha=a3a1839ccc9a v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:75
quote: rows
searched: spinner|indicatif · in **/*.rs
searched: \x1b[2K · in **/*.rs
@row_add n=188 row=decision/run-prints-nothing-while-wrapped area=brand-identity rec=fct-e37xe7yc@1
name: viola run silent while wrapped
rule: `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
@row_check n=188 row=decision/run-prints-nothing-while-wrapped sha=11b9dc2e1efc v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
searched: println!|print!|eprintln!|eprint! · in src/cmd/run.rs, src/cmd/revive.rs
@row_add n=189 row=contract/cli-transfer-arrow/ascii-arrow area=iconography rec=fct-2enypnm7@2
boundary: CLI transfer arrows
item: ->
promise: The CLI prints transfers with ASCII `->` and has no inbound form; links print only as `link` / `unlink` verb output (`overseer -> builder`).
@row_check n=189 row=contract/cli-transfer-arrow/ascii-arrow sha=f32a137d8860 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/verbs.rs:79
quote: out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
@row_add n=190 row=decision/list-six-fixed-columns area=iconography rec=fct-2enypnm7@2
name: viola list has six fixed columns
rule: `viola list` carries no link column, so its six columns stay fixed.
@row_check n=190 row=decision/list-six-fixed-columns sha=c1d94431d5db v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:72
quote: let [name, live, status, wheel, dialog, cli] = cells;
@row_add n=191 row=contract/cli-separators/two-spaces area=iconography rec=fct-g2p3izdb@2
boundary: CLI separators
item: two spaces
promise: Two spaces separate columns, and a refusal's reason from its detail, as separate fields.
@row_check n=191 row=contract/cli-separators/two-spaces sha=5ef62765e6c6 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:95
quote: "{open}{shown_name}{}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \
at: src/cmd/send.rs:136
quote: Some(detail) => format!("{reason}  {detail}"),
@row_add n=192 row=contract/cli-separators/space-hyphen-space area=iconography rec=fct-g2p3izdb@2
boundary: CLI separators
item: ` - `
promise: Space, hyphen, space replaces a middle dot inside one field or label (`driver - budget-paused`).
@row_check n=192 row=contract/cli-separators/space-hyphen-space sha=0eb0ed4832a5 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
@row_add n=193 row=contract/cli-colour-tokens/stale area=surface rec=fct-qgesfouf@1,fct-5r2kfdmj@1
boundary: CLI colour tokens
item: stale
promise: SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
@row_check n=193 row=contract/cli-colour-tokens/stale sha=42a629ed081f v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:38
quote: stale: "\x1b[2m",
at: src/human/board/cells.rs:77
quote: let span = |word: &str, token: &str| format!("{token}{word}{}{base}", tokens.reset);
at: src/human/board/cells.rs:90
quote: let close = if spans.stale { tokens.reset } else { "" };
@row_add n=194 row=contract/cli-colour-tokens/callsign area=surface rec=fct-23ap43hv@1,fct-5r2kfdmj@1
boundary: CLI colour tokens
item: callsign
promise: Bold (`\x1b[1m`) at every colour depth, on every NAME column value.
@row_check n=194 row=contract/cli-colour-tokens/callsign sha=6d8fd33f4161 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:39
quote: callsign: "\x1b[1m",
at: src/human/board/rows.rs:58
quote: name: true,
@row_add n=195 row=contract/cli-colour-tokens/handoff-blue area=surface rec=fct-ashdaqa2@1
boundary: CLI colour tokens
item: handoff blue
promise: Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
@row_check n=195 row=contract/cli-colour-tokens/handoff-blue sha=eaa57493e0bd v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:32
quote: Self::Truecolor => "\x1b[38;2;217;119;6m",
at: src/human/style.rs:33
quote: Self::Ansi256 => "\x1b[38;5;172m",
at: src/human/style.rs:34
quote: Self::Ansi16 => "\x1b[33m",
searched: 5B8DB8 · in src/**/*.rs
searched: 38;2;91 · in src/**/*.rs
searched: \x1b[34m · in src/**/*.rs
@row_add n=196 row=decision/cli-colour-amber-and-dim-only area=surface rec=fct-ashdaqa2@1
name: CLI colour limited to amber and dim
rule: CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted.
why: The quiz limits CLI colour to amber and dim.
@row_check n=196 row=decision/cli-colour-amber-and-dim-only sha=e7a4dd009028 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:16
quote: /// The four SGR tokens of one depth. No background and no blue token exists.
at: src/human/style.rs:38
quote: stale: "\x1b[2m",
@row_add n=197 row=decision/terminal-background-never-set area=surface rec=fct-6f4vi7a3@1
name: Terminal background never set
rule: The CLI never sets a background colour, at any colour depth.
why: The terminal background belongs to the user.
@row_check n=197 row=decision/terminal-background-never-set sha=561d56ba99e9 v=holds judge=sonnet head=8834195b0683
searched: 48; · in src/ (non-test: only src/run/wheel.rs terminal-reply filters, no SGR emitted)
searched: \x1b[4 and \x1b[10 background SGR · in src/
searched: struct Tokens fields · in src/human/style.rs (attention, stale, callsign, reset only)
@row_add n=198 row=contract/cli-stderr/context-lines area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: context lines
promise: Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
@row_check n=198 row=contract/cli-stderr/context-lines sha=4a2ced603752 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:70
quote: let _ = human::write_waiting(&mut io::stderr().lock(), name);
at: src/cmd/send.rs:104
quote: let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
@row_add n=199 row=contract/cli-stderr/refusal-line area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: refusal line
promise: A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
@row_check n=199 row=contract/cli-stderr/refusal-line sha=1fa8c179c883 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/human/mirror.rs:9
quote: format!("{boxed} {word:<MIRROR_WORD$}  {name}  {rest}\n")
@row_add n=200 row=contract/cli-stderr/fixed-message-refusal area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: fixed message form
promise: The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
@row_check n=200 row=contract/cli-stderr/fixed-message-refusal sha=9014d1b92c2c v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/cmd/run.rs:716
quote: human::refuse(&format!("{name} is already live"), "viola list");
at: src/cmd/revive.rs:110
quote: human::refuse(&unable, &hint);
at: src/cmd/verify.rs:99
quote: human::refuse(unable, hint);
at: src/cmd/list.rs:106
quote: human::refuse(unable, hint);
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
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
@row_add n=201 row=contract/cli-stderr/hint-line area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: hint line
promise: Each refusal's `hint:` line follows it directly on stderr.
@row_check n=201 row=contract/cli-stderr/hint-line sha=b9281bbffed6 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=202 row=contract/cli-stderr/error-line area=surface rec=fct-pi5kme4c@1
boundary: CLI stderr
item: error line
promise: Errors print on stderr as `error: …`.
@row_check n=202 row=contract/cli-stderr/error-line sha=1da6975b9f44 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
@row_add n=203 row=contract/peer-reply-screening/code-word area=surface rec=fct-4dwulxx5@1
boundary: peer reply screening
item: code word
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first; both clients, the CLI and the MCP server, screen a peer's `refusal` and `detail` through it.
@row_check n=203 row=contract/peer-reply-screening/code-word sha=ab857be2ef3a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:375-382
quote: pub fn is_code_word(word: &str) -> bool { let bytes = word.as_bytes(); bytes.len() <= 64 && bytes.first().is_some_and(u8::is_ascii_lowercase) && bytes .iter() .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
at: src/cmd/client.rs:77
quote: let reason = if is_code_word(reason) {
at: crates/viola-mcp/src/result.rs:73
quote: let reason = if is_code_word(reason) {
@row_add n=204 row=contract/peer-reply-screening/refusal-not-code-word area=surface rec=fct-y6uokfat@1
boundary: peer reply screening
item: refusal that is no code word
promise: A peer's `refusal` that is no code word reads `unknown`.
@row_check n=204 row=contract/peer-reply-screening/refusal-not-code-word sha=6b46ab7cc488 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:80
quote: RefusalReason::Unknown.as_str().to_owned()
@row_add n=205 row=contract/peer-reply-screening/string-detail-not-code-word area=surface rec=fct-y6uokfat@1
boundary: peer reply screening
item: string detail that is no code word
promise: A peer's string `detail` that is no code word reads as no detail, null under `--json`.
@row_check n=205 row=contract/peer-reply-screening/string-detail-not-code-word sha=75ab5947bcef v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
@row_add n=206 row=contract/peer-reply-screening/non-string-detail area=surface rec=fct-y6uokfat@1
boundary: peer reply screening
item: non-string detail
promise: A peer reply with a non-string `detail` is no known reply.
@row_check n=206 row=contract/peer-reply-screening/non-string-detail sha=cee0c677498f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:85
quote: Some(_) => return None,
@row_add n=207 row=contract/peer-reply-screening/unlisted-code-word area=surface rec=fct-kr6nq63b@1
boundary: peer reply screening
item: code word outside the refusal list
promise: A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does.
@row_check n=207 row=contract/peer-reply-screening/unlisted-code-word sha=c8db6669f822 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:77-78
quote: let reason = if is_code_word(reason) { reason.to_owned()
at: src/cmd/send.rs:76
quote: serde_json::from_value(Value::from(reason)).unwrap_or(RefusalReason::Unknown)
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
@row_add n=208 row=contract/human-tables/name-truncation area=surface rec=fct-kn56sdzi@2
boundary: human tables
item: NAME truncation
promise: Where the width is detectable, human tables truncate only the NAME of unwrapped rows, ending it in `...`.
@row_check n=208 row=contract/human-tables/name-truncation sha=3a33f9fdcf73 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:132
quote: shown.push_str(ELLIPSIS);
at: src/human/board/rows.rs:36
quote: Some(width) => longest(false).min(width.saturating_sub(UNWRAPPED_REST)),
@row_add n=209 row=contract/human-tables/width-source area=surface rec=fct-kn56sdzi@2
boundary: human tables
item: width source
promise: The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
@row_check n=209 row=contract/human-tables/width-source sha=2eaea86ee4f7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:63
quote: let width = viola_pty::host_size().map(|size| usize::from(size.cols));
at: crates/viola-pty/src/lib.rs:260
quote: if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) != 0 {
@row_add n=210 row=contract/viola-list-board/status area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: STATUS
promise: A wrapped row joined to a `claude agents --json` row, by child pid or agent session id, reads STATUS `idle` or `busy`; any other wrapped row reads `unknown`.
@row_check n=210 row=contract/viola-list-board/status sha=7dcffeffb9f5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:160-161
quote: let by_pid = self.pid.is_some() && self.pid == keys.child_pid; by_pid || keys.agent_session_id.as_deref() == Some(self.session_id.as_str())
at: crates/viola-state/src/board.rs:334
quote: item.status = observations[at].status;
at: src/cmd/list.rs:144
quote: AgentStatus::Idle => SessionStatus::Idle,
at: src/cmd/list.rs:145
quote: AgentStatus::Busy => SessionStatus::Busy,
at: src/cmd/list.rs:146
quote: AgentStatus::Unknown => SessionStatus::Unknown,
@row_add n=211 row=contract/viola-list-board/unwrapped-separator area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: unwrapped separator
promise: When the agents read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line follows the wrapped rows, or the empty-board line when there are none; with no unwrapped row no separator prints.
@row_check n=211 row=contract/viola-list-board/unwrapped-separator sha=ca1a9255c72b v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:99-100
quote: if wrapped < board.items.len() { lines.push(UNWRAPPED_SEPARATOR.to_owned());
at: src/human/board/rows.rs:96-97
quote: if wrapped == 0 { lines.push(EMPTY_BOARD.to_owned());
@row_add n=212 row=contract/viola-list-board/unwrapped-row area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: unwrapped row
promise: One row per unwrapped session, sorted by name: NAME the session's `sessionId`, LIVE `live`, its STATUS, and `n/a` in WHEEL, DIALOG and CLI.
@row_check n=212 row=contract/viola-list-board/unwrapped-row sha=da386180e21d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:346
quote: unwrapped.sort_by(|a, b| a.name.cmp(&b.name));
at: crates/viola-state/src/board.rs:193
quote: name: observation.session_id,
at: src/human/board/rows.rs:49-50
quote: let na = || NOT_APPLICABLE.to_owned(); (na(), na(), na())
@row_add n=213 row=contract/viola-list-board/failed-agents-read area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: failed agents read
promise: A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.
@row_check n=213 row=contract/viola-list-board/failed-agents-read sha=93a9b6ee0257 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:121-127
quote: if let Ok(list) = std::env::current_dir() .ok() .ok_or(()) .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ())) { board::join(&mut board, observations(list)); }
@row_add n=214 row=contract/viola-list-board/name-column-width area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: NAME column width
promise: On a terminal of known width NAME's column is the largest of the caption's 4, the longest wrapped NAME and the longest unwrapped NAME capped at the width less the 65 columns the rest of an unwrapped row takes.
@row_check n=214 row=contract/viola-list-board/name-column-width sha=7e8ead86b3e3 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:36
quote: Some(width) => longest(false).min(width.saturating_sub(UNWRAPPED_REST)),
at: src/human/board/rows.rs:39
quote: let column = longest(true).max(unwrapped).max(NAME_CAPTION.len());
at: src/human/board/cells.rs:109-110
quote: pub(super) const UNWRAPPED_REST: usize = LIVE_WIDTH + STATUS_WIDTH + WHEEL_WIDTH + DIALOG_WIDTH + 5 * 2 + NOT_APPLICABLE.len();
@row_add n=215 row=contract/viola-list-board/name-cut area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: NAME cut
promise: An unwrapped NAME longer than its column is cut to the column less 3 characters and ends in `...`, never splitting an escape; a wrapped NAME is never cut; a piped board or `--json` keeps every name whole.
@row_check n=215 row=contract/viola-list-board/name-cut sha=f509f5861880 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:52-53
quote: let name = if !item.wrapped && name.chars().count() > column { cut(&name, column)
at: src/human/board/cells.rs:127
quote: let mut keep = column.saturating_sub(ELLIPSIS.len());
at: src/human/board/cells.rs:128
quote: if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) {
at: src/cmd/list.rs:63
quote: let width = viola_pty::host_size().map(|size| usize::from(size.cols));
@row_add n=216 row=contract/viola-list-board/uncoloured-lines area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: uncoloured lines
promise: The BAY line, the caption, the empty-board line and the separator carry no SGR.
@row_check n=216 row=contract/viola-list-board/uncoloured-lines sha=b3f04770525e v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:100
quote: lines.push(UNWRAPPED_SEPARATOR.to_owned());
at: src/human/board/rows.rs:43
quote: None,
at: src/human/board/bay.rs:69
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
@row_add n=217 row=contract/viola-list-board/colour-spans area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: colour spans
promise: Colour spans are laid over the escaped, padded cells, so the columns are the plain board's, and the board stays one write.
@row_check n=217 row=contract/viola-list-board/colour-spans sha=38c50c51c59c v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:72
quote: depth.map(|depth| (depth.tokens(), spans)),
at: src/human/board/cells.rs:95
quote: "{open}{shown_name}{}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \
at: src/human/board/rows.rs:105
quote: out.write_all(text.as_bytes())
@row_add n=218 row=contract/viola-list-board/row-escaping area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: row escaping
promise: The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
@row_check n=218 row=contract/viola-list-board/row-escaping sha=777c41cb3831 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:23-24
quote: if c.is_control() { shown.push_str(&format!("\\x{:02X}", u32::from(c)));
at: src/human/board/rows.rs:32
quote: .map(|(_, name)| name.chars().count())
@row_add n=219 row=decision/wheel-column-width-23 area=surface rec=fct-5r2kfdmj@1
name: WHEEL column padded to 23
rule: The board's WHEEL column is padded to 23 characters, the width of its widest word `unknown - budget-paused` (a replayed row with a logged pause and no logged holder).
why: Ratified.
@row_check n=219 row=decision/wheel-column-width-23 sha=19c2e653de3d v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:12
quote: const WHEEL_WIDTH: usize = 23;
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
@row_add n=220 row=contract/viola-list-board/window-figure area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: window figure
promise: A window's figure is rounded down to a whole number (`89.6` prints `89 %`).
@row_check n=220 row=contract/viola-list-board/window-figure sha=7d7fa9a08276 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:27
quote: format!("{label} {:.0} %{suffix}", used.floor())
@row_add n=221 row=contract/viola-list-board/reading-age area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: reading age
promise: The reading's age prints in whole minutes rounded down at every age (`read 0m ago`, `read 75m ago`), and `read unknown` for no instant, one that does not parse, or one ahead of now.
@row_check n=221 row=contract/viola-list-board/reading-age sha=462b5b1d021f v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:38
quote: Some(at) => format!("read {}m ago", (now - at.with_timezone(&Utc)).num_minutes()),
at: src/human/board/bay.rs:37
quote: match at.filter(|at| *at <= now) {
at: src/human/board/bay.rs:39
quote: None => "read unknown".to_owned(),
@row_add n=222 row=contract/viola-list-board/skip-counts area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: skip counts
promise: Nonzero skip counts print as bare numbers in key order (`skipped 2 - 0 - 1`).
@row_check n=222 row=contract/viola-list-board/skip-counts sha=c0b5486a7f38 v=holds judge=sonnet head=8834195b0683
at: src/human/board/bay.rs:69-70
quote: "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}", skipped.unknown_kinds, skipped.unknown_fields, skipped.torn_lines
@row_add n=223 row=contract/viola-list-board/human-holder-budget-paused area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: human holder under a budget pause
promise: A `human` holder under a budget pause prints `human - budget-paused`.
@row_check n=223 row=contract/viola-list-board/human-holder-budget-paused sha=3e4a19d59b5b v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:17-18
quote: if item.budget_paused == Some(true) { format!("{holder} - budget-paused")
@row_add n=224 row=contract/viola-list-board/empty-board area=surface rec=fct-5r2kfdmj@1
boundary: viola list board
item: empty board
promise: The empty board prints the BAY line, then the caption row, then its one line.
@row_check n=224 row=contract/viola-list-board/empty-board sha=10a79ee7447a v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:94-98
quote: let mut lines = vec![bay_line(budget, &board.skipped, now)]; lines.extend(rows.by_ref().take(1 + wrapped)); if wrapped == 0 { lines.push(EMPTY_BOARD.to_owned()); }
@row_add n=225 row=contract/viola-list-board/strict-modes-refusal area=surface rec=fct-t547mw3y@1
boundary: viola list board
item: strict-modes refusal
promise: A failed strict-modes check of the home, `instances/` or any one instance refuses the whole board with exit 21, nothing from state printed, in the fixed `unable:` / `hint:` form.
@row_check n=225 row=contract/viola-list-board/strict-modes-refusal sha=9b539e3e4488 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:45-47
quote: Err(refused) => { refuse(args.json, &refused); return Ok(ExitCode::from(21));
at: crates/viola-state/src/board.rs:59-62
quote: if check_if_present(home, check_read).is_err() || check_if_present(&instances_dir, check_path).is_err() { return Ok(Err(ScanRefused::Home));
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human/board/pairs.rs:14-15
quote: "the viola home is not private to you", "the viola home must be readable only by you; viola does not change its permissions",
at: src/human/board/pairs.rs:6-7
quote: format!("{name}'s state files can be written by another user"), "viola will not read them; make the viola home and its files owner-only".to_owned(),
@row_add n=226 row=contract/fixed-refusal-pairs/home-state area=surface rec=fct-t547mw3y@1,fct-pbvocw2a@1,fct-xi5ne4sz@1
boundary: fixed refusal pairs
item: home-state pair
promise: `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
@row_check n=226 row=contract/fixed-refusal-pairs/home-state sha=ba21c0dbdab2 v=holds judge=sonnet head=8834195b0683
at: src/human/board/pairs.rs:14
quote: "the viola home is not private to you",
at: src/human/board/pairs.rs:15
quote: "the viola home must be readable only by you; viola does not change its permissions",
@row_add n=227 row=contract/fixed-refusal-pairs/instance-state area=surface rec=fct-t547mw3y@1,fct-pbvocw2a@1
boundary: fixed refusal pairs
item: instance-state pair
promise: `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
@row_check n=227 row=contract/fixed-refusal-pairs/instance-state sha=19e05f1ccc52 v=holds judge=sonnet head=8834195b0683
at: src/human/board/pairs.rs:6
quote: format!("{name}'s state files can be written by another user"),
at: src/human/board/pairs.rs:7
quote: "viola will not read them; make the viola home and its files owner-only".to_owned(),
@row_add n=228 row=contract/viola-list-board/json-refusal area=surface rec=fct-t547mw3y@1
boundary: viola list board
item: --json refusal
promise: Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
@row_check n=228 row=contract/viola-list-board/json-refusal sha=817c2f088451 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:93-94
quote: let cause = UnreachableCause::StrictModes.as_str(); json!({"v": 1, "error": "instance-unreachable", "detail": cause})
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
at: src/cmd/list.rs:99-101
quote: if json { client::document(&refused_document()); return;
@row_add n=229 row=contract/viola-list-board/strict-modes-scan area=surface rec=fct-nmng4ymy@2
boundary: viola list board
item: strict-modes scan
promise: The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
@row_check n=229 row=contract/viola-list-board/strict-modes-scan sha=e183b44865ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:59-62
quote: if check_if_present(home, check_read).is_err() || check_if_present(&instances_dir, check_path).is_err() { return Ok(Err(ScanRefused::Home));
at: crates/viola-state/src/board.rs:97-98
quote: if check_instance(home, dir).is_err() { return Ok(Err(ScanRefused::Instance(name.clone())));
at: src/cmd/list.rs:105
quote: let (unable, hint) = human::home_state_pair();
@row_add n=230 row=contract/refusal-hints/refusal-line area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: refusal line
promise: A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed.
@row_check n=230 row=contract/refusal-hints/refusal-line sha=971255662f9d v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=231 row=contract/refusal-hints/human-typing area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: human-typing
promise: `the human has the wheel; send again after the human hands it back`
@row_check n=231 row=contract/refusal-hints/human-typing sha=b75fcac68196 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:5
quote: const HUMAN_TYPING_HINT: &str = "the human has the wheel; send again after the human hands it back";
@row_add n=232 row=contract/refusal-hints/budget-paused area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: budget-paused
promise: `budget gate is closed; it lifts at the window reset, or when the human lifts it`
@row_check n=232 row=contract/refusal-hints/budget-paused sha=3e40bc898f6d v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:30
quote: "budget gate is closed; it lifts at the window reset, or when the human lifts it"
@row_add n=233 row=decision/no-hint-names-viola-release area=surface rec=fct-pbvocw2a@1
name: No hint names viola release
rule: No refusal hint ever names `viola release`; `release` stays a human verb.
why: The human-typing and budget-paused refusals reach drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
@row_check n=233 row=decision/no-hint-names-viola-release sha=07a8d5a14ca4 v=holds judge=sonnet head=8834195b0683
searched: viola release · in src/human/hints.rs (no hint string contains it)
searched: viola release · in src/ (only doc comments in src/run/budget.rs, src/run/wheel.rs, src/cmd/release.rs)
@row_add n=234 row=contract/refusal-hints/unverified-cli area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: unverified-cli
promise: `run viola verify for this CLI version`
@row_check n=234 row=contract/refusal-hints/unverified-cli sha=069249b8035f v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:12
quote: ("unverified-cli", _) => Some("run viola verify for this CLI version"),
@row_add n=235 row=contract/refusal-hints/not-delivered-turn-running area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · turn-running
promise: `a turn is running; viola wait <name> first`
@row_check n=235 row=contract/refusal-hints/not-delivered-turn-running sha=10738c1f2801 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:48
quote: "turn-running" => format!("a turn is running; viola wait {name} first"),
@row_add n=236 row=contract/refusal-hints/not-delivered-control-character area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · control-character
promise: `the text contains a control character (only LF, CR, TAB are allowed)`
@row_check n=236 row=contract/refusal-hints/not-delivered-control-character sha=f77388e10dbc v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:34
quote: "the text contains a control character (only LF, CR, TAB are allowed)".to_owned()
@row_add n=237 row=contract/refusal-hints/not-delivered-empty-text area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · empty-text
promise: `the text is empty once its trailing newlines are removed; send a text with content`
@row_check n=237 row=contract/refusal-hints/not-delivered-empty-text sha=adbcfc689cf4 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:37
quote: "the text is empty once its trailing newlines are removed; send a text with content"
@row_add n=238 row=contract/refusal-hints/not-delivered-input-not-ready area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · input-not-ready
promise: `<name> was not ready for input; send again, and if it repeats a human must look at the session`
@row_check n=238 row=contract/refusal-hints/not-delivered-input-not-ready sha=bf4a4aaa1f34 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:42
quote: "{name} was not ready for input; send again, and if it repeats a human must look at the session"
@row_add n=239 row=contract/refusal-hints/not-delivered-no-prompt-submitted area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · no-prompt-submitted
promise: `<name> did not submit the prompt; check it, then send again`
@row_check n=239 row=contract/refusal-hints/not-delivered-no-prompt-submitted sha=3eac52f1e153 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:46
quote: format!("{name} did not submit the prompt; check it, then send again")
@row_add n=240 row=contract/refusal-hints/not-delivered-unknown-dialog area=surface rec=fct-pbvocw2a@1
boundary: refusal hints
item: not-delivered · unknown-dialog
promise: `that dialog is not pending; viola list shows the current DIALOG`
@row_check n=240 row=contract/refusal-hints/not-delivered-unknown-dialog sha=812e4dafccd6 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:14
quote: Some("that dialog is not pending; viola list shows the current DIALOG")
@row_add n=241 row=decision/instance-unreachable-hint-per-cause area=surface rec=fct-pbvocw2a@1
name: One instance-unreachable hint per cause
rule: The `instance-unreachable` refusal (exit 21) prints one hint per cause.
why: The exit code is shared by several causes.
@row_check n=241 row=decision/instance-unreachable-hint-per-cause sha=f3cd806b2ca5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:144-148
quote: Self::Dead => "not-running", Self::Unwrapped => "unwrapped-name", Self::Lost => "endpoint-lost", Self::ServerUnverified => "server-verify", Self::SocketDir => "socket-dir",
@row_add n=242 row=contract/refusal-hints/not-running area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: not-running
promise: When no wrapper is running for the name: `<name> is not running; viola list shows the live instances`; `not-running` is the hint key of the log's `instance-dead`.
@row_check n=242 row=contract/refusal-hints/not-running sha=6076cf2aaba6 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:49
quote: "not-running" => format!("{name} is not running; viola list shows the live instances"),
at: src/cmd/client.rs:119
quote: Self::Dead => UnreachableCause::InstanceDead,
at: src/cmd/client.rs:144
quote: Self::Dead => "not-running",
@row_add n=243 row=contract/refusal-hints/unwrapped-name area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: unwrapped-name
promise: When the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
@row_check n=243 row=contract/refusal-hints/unwrapped-name sha=5e537737b727 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:50-51
quote: "unwrapped-name" => format!(             "{name} is not a wrapped session; only sessions started with viola run take commands"
@row_add n=244 row=contract/refusal-hints/endpoint-lost area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: endpoint-lost
promise: When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
@row_check n=244 row=contract/refusal-hints/endpoint-lost sha=d2412e4079d1 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:53-54
quote: "endpoint-lost" => format!(             "{name}'s wrapper went away during the call; viola list shows whether it is still running"
@row_add n=245 row=contract/frame-verb-refusals/strict-modes area=surface rec=fct-pbvocw2a@1
boundary: frame verb refusals
item: strict-modes
promise: A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
@row_check n=245 row=contract/frame-verb-refusals/strict-modes sha=9a077ea966ac v=holds judge=sonnet head=8834195b0683
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
at: src/cmd/client.rs:35-38
quote: unreachable_document(UnreachableCause::StrictModes); } else { let (unable, hint) = human::instance_state_pair(name); human::refuse(&unable, &hint);
@row_add n=246 row=contract/refusal-hints/server-verify area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: server-verify
promise: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
@row_check n=246 row=contract/refusal-hints/server-verify sha=fe51232dd369 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:56-57
quote: "server-verify" => format!(             "the process on {name}'s endpoint is not its recorded wrapper; it was not contacted"
@row_add n=247 row=contract/refusal-hints/socket-dir area=surface rec=fct-pbvocw2a@1,fct-hrikc24v@1
boundary: refusal hints
item: socket-dir
promise: The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_check n=247 row=contract/refusal-hints/socket-dir sha=c004d546f08d v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:59-60
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \                          you; viola does not change its permissions"
@row_add n=248 row=contract/viola-run-start-refusals/already-live area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: already-live
promise: `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
@row_check n=248 row=contract/viola-run-start-refusals/already-live sha=ac8e416687fe v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:716
quote: human::refuse(&format!("{name} is already live"), "viola list");
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=249 row=contract/viola-run-start-refusals/stale-heartbeat area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: stale-heartbeat
promise: A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.
@row_check n=249 row=contract/viola-run-start-refusals/stale-heartbeat sha=52bbb99edb0a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:722-723
quote: &format!("{name} is still running but not answering"), &format!("viola list shows it as stale; stop that process before starting {name} again"),
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=250 row=contract/viola-run-start-refusals/squatted-name area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: squatted-name
promise: `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`.
@row_check n=250 row=contract/viola-run-start-refusals/squatted-name sha=b6dc368a033e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:747-748
quote: &format!("the endpoint for {name} is held by another process"), "another process holds this name's endpoint; stop it or pick another name",
@row_add n=251 row=contract/viola-run-start-refusals/pinned-hash-mismatch area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: pinned-hash-mismatch
promise: A SHA-256 mismatch prints `unable: the pinned viola copy failed its integrity check` then `hint: the pinned copy was changed after it was written, so viola will not run it`.
@row_check n=251 row=contract/viola-run-start-refusals/pinned-hash-mismatch sha=73f5454cc754 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:729-730
quote: "the pinned viola copy failed its integrity check", "the pinned copy was changed after it was written, so viola will not run it",
@row_add n=252 row=contract/viola-run-start-refusals/home-tree-check area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: home-tree check
promise: At start, before any log and with no child, the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` are checked, each that exists; a home-level refusal prints the home-state pair, an instance-level one the instance-state pair.
@row_check n=252 row=contract/viola-run-start-refusals/home-tree-check sha=033993545fb1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:210-215
quote: TreeRefused::HomeLevel(_) => { let (unable, hint) = human::home_state_pair(); human::refuse(unable, hint); } TreeRefused::InstanceLevel(_) => { let (unable, hint) = human::instance_state_pair(name.as_ref());
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
@row_add n=253 row=contract/viola-run-start-refusals/socket-dir area=surface rec=fct-pbvocw2a@1
boundary: viola run start refusals
item: socket directory
promise: Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_check n=253 row=contract/viola-run-start-refusals/socket-dir sha=919f6a20a720 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:738-739
quote: &format!("the socket directory for {name} cannot be trusted"), &human::send_hint(name, "socket-dir").unwrap_or_default(),
at: src/human/hints.rs:59-60
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \ you; viola does not change its permissions"
at: src/cmd/run.rs:497
quote: if !socket_dir_ready(&endpoint) {
@row_add n=254 row=contract/viola-revive-refusals/order area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: order
promise: `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.
@row_check n=254 row=contract/viola-revive-refusals/order sha=c37a73dcc4a9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:176-179
quote: if let Some(refusal) = holder() { return Ok(Err(refusal)); } let session = match session()? {
at: src/cmd/revive.rs:212
quote: || collision_check(name, &instance_dir, read.as_ref().ok(), json),
@row_add n=255 row=contract/viola-revive-refusals/no-session area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: no-session
promise: `unable: <name> has no logged session to resume` then `hint: start one: viola run <name> -- claude`.
@row_check n=255 row=contract/viola-revive-refusals/no-session sha=bbab71cc0607 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:83-84
quote: format!("{name} has no logged session to resume"), format!("start one: viola run {name} -- {PROGRAM}"),
@row_add n=256 row=contract/viola-revive-refusals/no-such-id area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: no logged session with that id
promise: An `--id` the log does not hold prints `unable: <name> has no logged session with that id` then `hint: viola revive <name> --list`.
@row_check n=256 row=contract/viola-revive-refusals/no-such-id sha=25d05f0e44f8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:87-88
quote: format!("{name} has no logged session with that id"), format!("viola revive {name} --list"),
@row_add n=257 row=contract/viola-revive-refusals/session-live area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: session-live
promise: When one agents read lists the logged id live in another `claude` (skipped under `--fork`; a failed read passes): `unable: <name>'s session is open in another claude` then `hint: close it there first, or start a copy: viola revive <name> --fork`.
@row_check n=257 row=contract/viola-revive-refusals/session-live sha=9a73e1a00e8a v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:91-92
quote: format!("{name}'s session is open in another {PROGRAM}"), format!("close it there first, or start a copy: viola revive {name} --fork"),
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
@row_add n=258 row=contract/viola-revive-refusals/cwd-missing area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: cwd-missing
promise: `unable: <name>'s recorded directory is missing` then `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`.
@row_check n=258 row=contract/viola-revive-refusals/cwd-missing sha=ecf255b711a9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:95-98
quote: format!("{name}'s recorded directory is missing"), format!( "resume it by hand from a directory you choose: viola run {name} -- {PROGRAM} \ {RESUME_FLAG} <id>, ids from viola revive {name} --list"
@row_add n=259 row=contract/viola-revive-refusals/home-tree-check area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: home-tree check
promise: The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
@row_check n=259 row=contract/viola-revive-refusals/home-tree-check sha=0fd1638e435f v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:141-144
quote: let exe = match checked_exe(home, &args.name)? { Ok(exe) => exe, Err(_) if args.json => { Refusal::StrictModes.write(&args.name, true);
at: src/cmd/revive.rs:152
quote: let persistent = open_wrapper_log(home, &args.name)?;
@row_add n=260 row=contract/viola-revive-refusals/socket-dir area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: socket directory
promise: A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
@row_check n=260 row=contract/viola-revive-refusals/socket-dir sha=ad39009047ea v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:154
quote: Ok(launch) => start(home, &launch, &persistent, &exe),
at: src/cmd/run.rs:300-301
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || { refuse_socket_dir(name);
@row_add n=261 row=decision/hints-human-mode-only area=surface rec=fct-pbvocw2a@1
name: Hints in human mode only
rule: Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
@row_check n=261 row=decision/hints-human-mode-only sha=2fa515a33e2a v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:133
quote: return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
@row_add n=262 row=contract/log-detail-codes/server-verify-failed area=surface rec=fct-pbvocw2a@1
boundary: log detail codes
item: server-verify-failed
promise: Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document reads the same word.
@row_check n=262 row=contract/log-detail-codes/server-verify-failed sha=c1a60a004447 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: src/cmd/client.rs:261
quote: detail = cause.detail(),
at: src/cmd/send.rs:146
quote: client::unreachable(self.json, cause, || {
at: src/cmd/wait.rs:50
quote: client::unreachable(args.json, cause, || {
at: src/cmd/last.rs:37
quote: client::unreachable(args.json, cause, || {
at: src/cmd/answer.rs:118
quote: client::unreachable(out.json, cause, || {
at: src/cmd/pause.rs:63
quote: client::unreachable(json, cause, || {
at: src/cmd/release.rs:39
quote: let code = wheel_call(
at: src/cmd/link.rs:99
quote: client::unreachable(args.json, cause, || {
at: src/cmd/unlink.rs:15
quote: let code = link_call(home, args, Verb::Unlink, request)?;
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=263 row=contract/log-detail-codes/socket-dir-refused area=surface rec=fct-pbvocw2a@1
boundary: log detail codes
item: socket-dir-refused
promise: Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
@row_check n=263 row=contract/log-detail-codes/socket-dir-refused sha=1a1555a1ff8c v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: src/cmd/client.rs:261
quote: detail = cause.detail(),
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/revive.rs:154
quote: Ok(launch) => start(home, &launch, &persistent, &exe),
@row_add n=264 row=contract/viola-revive-refusals/strict-modes-no-log area=surface rec=fct-pbvocw2a@1
boundary: viola revive refusals
item: strict-modes
promise: `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
@row_check n=264 row=contract/viola-revive-refusals/strict-modes-no-log sha=365c56d1bb2d v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:137-139
quote: if !trusted(home, &args.name)? { Refusal::StrictModes.write(&args.name, args.json); return Ok(Started::Refused(ExitCode::from(1)));
at: src/cmd/revive.rs:152
quote: let persistent = open_wrapper_log(home, &args.name)?;
@row_add n=265 row=decision/no-hint-for-unknown area=surface rec=fct-pbvocw2a@1
name: No hint for unknown
rule: The `unknown` refusal (exit 14) prints no hint line.
why: Its detail is a code word or nothing, and a hint would have to guess.
@row_check n=265 row=decision/no-hint-for-unknown sha=30a000ef8983 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:85
quote: RefusalReason::UnverifiedCli | RefusalReason::Unknown => None,
@row_add n=266 row=contract/frame-verb-refusals/socket-dir area=surface rec=fct-fjfzfyz5@1
boundary: frame verb refusals
item: socket directory
promise: A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
@row_check n=266 row=contract/frame-verb-refusals/socket-dir sha=594e0969e202 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: src/cmd/client.rs:148
quote: Self::SocketDir => "socket-dir",
at: src/cmd/send.rs:151
quote: "instance-unreachable",
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=267 row=contract/viola-release-budget/lifted-line area=surface rec=fct-tkxomkux@1
boundary: viola release --budget
item: lifted line
promise: When a pause stood, `viola release <name> --budget` prints `<name>  budget gate lifted until <time>  wheel human`, one unstyled stdout line with the two-space gutter of the other verb lines.
@row_check n=267 row=contract/viola-release-budget/lifted-line sha=935590698ec4 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:49
quote: out.write_all(format!("{name}  budget gate lifted until {end}  wheel {holder}\n").as_bytes())
@row_add n=268 row=contract/viola-release-budget/override-time area=surface rec=fct-tkxomkux@1
boundary: viola release --budget
item: override time
promise: The time is the reply's `override_until`: `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date; a value that is no RFC 3339 instant prints escaped.
@row_check n=268 row=contract/viola-release-budget/override-time sha=c9bd485a613b v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:29
quote: let Ok(end) = DateTime::parse_from_rfc3339(override_until) else {
at: src/human/verbs.rs:30
quote: return escape_message(override_until);
at: src/human/verbs.rs:32
quote: let end = end.with_timezone(&Utc);
at: src/human/verbs.rs:34
quote: "%H:%M:%SZ"
at: src/human/verbs.rs:36
quote: "%Y-%m-%d %H:%M:%SZ"
@row_add n=269 row=contract/viola-release-budget/open-line area=surface rec=fct-tkxomkux@1
boundary: viola release --budget
item: open line
promise: When no pause stood the line is `<name>  budget gate open  wheel human`, one unstyled stdout line.
@row_check n=269 row=contract/viola-release-budget/open-line sha=39798a949bd0 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:54
quote: out.write_all(format!("{name}  budget gate open  wheel {holder}\n").as_bytes())
@row_add n=270 row=contract/viola-link-output/result-line area=surface rec=fct-327x7jz3@1
boundary: viola link and unlink output
item: result line
promise: `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
@row_check n=270 row=contract/viola-link-output/result-line sha=1ef075e81c6a v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:65
quote: out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
at: src/human/verbs.rs:79
quote: out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
at: src/cmd/link.rs:125-126
quote: run::log_self_exit(0, None); Ok(0)
@row_add n=271 row=contract/viola-link-output/printed-names area=surface rec=fct-327x7jz3@1
boundary: viola link and unlink output
item: printed names
promise: The names printed are the verb's own parsed arguments, never names read back from the reply or the log.
@row_check n=271 row=contract/viola-link-output/printed-names sha=c8ac3a583327 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:123
quote: let _ = write_line(&mut io::stdout().lock(), verb, driver, driven, &ok);
@row_add n=272 row=contract/viola-link-output/since area=surface rec=fct-clb6azzi@1
boundary: viola link and unlink output
item: since
promise: The `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant prints through the message-mode escaper.
@row_check n=272 row=contract/viola-link-output/since sha=0c07897b9417 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:86
quote: Ok(at) => at.with_timezone(&Utc).format("%H:%M:%SZ").to_string(),
at: src/human/verbs.rs:87
quote: Err(_) => escape_message(since),
@row_add n=273 row=contract/viola-link-output/repeat-link area=surface rec=fct-dfvj3huj@1
boundary: viola link and unlink output
item: repeat link
promise: A second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`, and exits 0.
@row_check n=273 row=contract/viola-link-output/repeat-link sha=ed9db99077de v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-63
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true));
at: src/human/verbs.rs:74
quote: out.write_all(format!("link    {driver} -> {driven}  already linked\n").as_bytes())
@row_add n=274 row=contract/viola-link-output/unlink-not-held area=surface rec=fct-dfvj3huj@1
boundary: viola link and unlink output
item: unlink of a pair not held
promise: `viola unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line and exits 0.
@row_check n=274 row=contract/viola-link-output/unlink-not-held sha=79bf32bdb7e7 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:105-106
quote: if self.since(&set, driver).is_none() { return Ok(false);
at: src/cmd/link.rs:75
quote: (Verb::Unlink, _) => human::write_unlinked(out, driver, driven),
@row_add n=275 row=decision/one-progress-pattern area=surface rec=fct-sx3u7aez@2
name: One progress pattern
rule: A step counter with static appended stdout lines is the project's one progress pattern.
@row_check n=275 row=decision/one-progress-pattern sha=0ad05ac8389b v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
@row_add n=276 row=contract/viola-verify-output/summary-line area=surface rec=fct-sx3u7aez@2,fct-sedcqxww@2
boundary: viola verify output
item: summary line
promise: The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`; a failing row prints no stderr word.
@row_check n=276 row=contract/viola-verify-output/summary-line sha=510853658cbb v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:293-296
quote: human::result(&format!( "stamped {version}  {} pass  {failed} fail", results.len() - failed ));
at: src/human.rs:64
quote: let _ = write_result(&mut io::stdout().lock(), line);
@row_add n=277 row=contract/viola-verify-output/fail-word area=surface rec=fct-sx3u7aez@2
boundary: viola verify output
item: fail word
promise: The word `fail` is uncoloured.
@row_check n=277 row=contract/viola-verify-output/fail-word sha=95c22d994079 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:492
quote: let verdict = if pass { "pass" } else { "fail" };
at: src/human.rs:59
quote: out.write_all(format!("{line}\n").as_bytes())
@row_add n=278 row=contract/viola-verify-output/help area=surface rec=fct-sx3u7aez@2
boundary: viola verify output
item: --help
promise: `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
@row_check n=278 row=contract/viola-verify-output/help sha=bc47836d559a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:85-87
quote: /// Run it from a folder you trust in Claude Code: the input-box probe runs in a subfolder there, /// which inherits that trust. An unapproved external CLAUDE.md import blocks the probe: its /// dialog shows in the subfolder, and verify never answers it.
@row_add n=279 row=contract/viola-verify-refusals/interrupted area=surface rec=fct-sx3u7aez@2,fct-sedcqxww@2
boundary: viola verify refusals
item: interrupted run
promise: A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
@row_check n=279 row=contract/viola-verify-refusals/interrupted sha=e46f1f3a4785 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:82-88
quote: Ended::Interrupted => { human::refuse( "verify was stopped before it finished", "run viola verify again to stamp this CLI", ); (1, Some("interrupted")) }
@row_add n=280 row=contract/viola-verify-refusals/home-tree-check area=surface rec=fct-xi5ne4sz@1,fct-sedcqxww@2
boundary: viola verify refusals
item: home-tree check
promise: Before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`, each that exists; a refusal prints no step line and no summary, only the home-state pair, and exits 1.
@row_check n=280 row=contract/viola-verify-refusals/home-tree-check sha=7edd86246176 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
at: crates/viola-state/src/strict.rs:95-97
quote: (ledger_dir(home), Rule::Write), (home.join("bin"), Rule::Write), (home.join("plugin"), Rule::Write),
@row_add n=281 row=contract/viola-verify-refusals/stamps-update area=surface rec=fct-xi5ne4sz@1,fct-sedcqxww@2
boundary: viola verify refusals
item: stamps.json update
promise: A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1.
@row_check n=281 row=contract/viola-verify-refusals/stamps-update sha=19ea6e48c251 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:286-288
quote: Err(StampsError::StrictModes(_)) => { refuse_home(); return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=282 row=contract/strict-modes/home-under-regular-file area=surface rec=fct-bctn63fi@2
boundary: strict-modes refusals
item: home under a regular file
promise: A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
@row_check n=282 row=contract/strict-modes/home-under-regular-file sha=04085e9e3d6d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:171-172
quote: if !is_dir { return Err(Refused::NotADirectory);
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
at: src/cmd/client.rs:37-40
quote: let (unable, hint) = human::instance_state_pair(name); human::refuse(&unable, &hint); } Ok(Some(21))
@row_add n=283 row=contract/viola-revive-output/after-child-start area=surface rec=fct-gray6vne@1
boundary: viola revive output
item: after the child starts
promise: `viola revive`, as `run`, prints nothing once the child starts.
@row_check n=283 row=contract/viola-revive-output/after-child-start sha=d83006d01d01 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=284 row=contract/viola-revive-output/list-line area=surface rec=fct-gray6vne@1
boundary: viola revive output
item: --list line
promise: `viola revive <name> --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>`, two spaces between fields: no header, no colour, plain ASCII.
@row_check n=284 row=contract/viola-revive-output/list-line sha=3c10f6e32647 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:313
quote: format!("{}  {}  {}", self.ts, self.cause, self.id)
@row_add n=285 row=contract/viola-revive-output/list-json area=surface rec=fct-gray6vne@1
boundary: viola revive output
item: --list --json
promise: `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}`.
@row_check n=285 row=contract/viola-revive-output/list-json sha=cff469b9d3c7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
@row_add n=286 row=contract/viola-revive-json-codes/strict-modes-failed area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: strict-modes-failed
promise: `strict-modes-failed` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=286 row=contract/viola-revive-json-codes/strict-modes-failed sha=dccd099df186 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=287 row=contract/viola-revive-json-codes/already-live area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: already-live
promise: `already-live` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=287 row=contract/viola-revive-json-codes/already-live sha=46c211fc65b6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=288 row=contract/viola-revive-json-codes/no-session area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: no-session
promise: `no-session` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=288 row=contract/viola-revive-json-codes/no-session sha=922deddc292f v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=289 row=contract/viola-revive-json-codes/session-live area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: session-live
promise: `session-live` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=289 row=contract/viola-revive-json-codes/session-live sha=02d46bad41a0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=290 row=contract/viola-revive-json-codes/cwd-missing area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: cwd-missing
promise: `cwd-missing` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=290 row=contract/viola-revive-json-codes/cwd-missing sha=4541c20e8bc8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=291 row=contract/viola-revive-json-codes/stale-heartbeat area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: stale-heartbeat
promise: `stale-heartbeat` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=291 row=contract/viola-revive-json-codes/stale-heartbeat sha=2911e3168a2c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=292 row=contract/viola-revive-json-codes/pinned-hash-mismatch area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: pinned-hash-mismatch
promise: `pinned-hash-mismatch` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=292 row=contract/viola-revive-json-codes/pinned-hash-mismatch sha=717f6fe050cd v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=293 row=contract/viola-revive-json-codes/squatted-name area=surface rec=fct-mep5dn6i@2
boundary: viola revive --json exit-1 codes
item: squatted-name
promise: `squatted-name` is one of the closed exit-1 codes of `viola revive --json`.
@row_check n=293 row=contract/viola-revive-json-codes/squatted-name sha=97d11988c0b2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=294 row=contract/viola-revive-json-codes/socket-dir-refused area=surface rec=fct-mep5dn6i@2,fct-gul3bnzl@1
boundary: viola revive --json exit-1 codes
item: socket-dir-refused
promise: A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
@row_check n=294 row=contract/viola-revive-json-codes/socket-dir-refused sha=f5c6fa5a47f4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:518
quote: (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=295 row=contract/viola-revive-json-codes/internal-error area=surface rec=fct-mep5dn6i@2,fct-4md437yp@1
boundary: viola revive --json exit-1 codes
item: internal-error
promise: A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
@row_check n=295 row=contract/viola-revive-json-codes/internal-error sha=bc8f3aefd74d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:270
quote: Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
at: src/cmd/revive.rs:122-124
quote: let started = started(home, args).inspect_err(|_| { if json { client::refused_document("internal-error");
at: src/cmd/revive.rs:120
quote: // The error still reaches the catch site, for its chain and its line. A `pump_child` error
@row_add n=296 row=decision/no-json-after-child-held-terminal area=surface rec=fct-xtefeabf@1,fct-sedcqxww@2
name: No document once the child held the terminal
rule: An error while pumping the child under `viola revive --json` prints no document.
why: The child has held the terminal and viola writes nothing past it.
@row_check n=296 row=decision/no-json-after-child-held-terminal sha=4421e7dc2bde v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
at: src/cmd/revive.rs:122-126
quote: let started = started(home, args).inspect_err(|_| { if json { client::refused_document("internal-error"); } })?;
@row_add n=297 row=contract/wrapper-socket/refused-peer area=surface rec=fct-f2zfjygd@1
boundary: wrapper socket
item: refused peer
promise: A peer the wrapper refuses is dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.
@row_check n=297 row=contract/wrapper-socket/refused-peer sha=357c4beae5d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
@row_add n=298 row=contract/viola-verify-refusals/cli-not-found area=surface rec=fct-sedcqxww@2
boundary: viola verify refusals
item: CLI not found
promise: `unable: the claude CLI was not found`, followed by its own hint, exit 1.
@row_check n=298 row=contract/viola-verify-refusals/cli-not-found sha=d0c0d738e35f v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:161-162
quote: "the claude CLI was not found", "install Claude Code or put it on PATH",
@row_add n=299 row=contract/viola-verify-refusals/version-unreadable area=surface rec=fct-sedcqxww@2
boundary: viola verify refusals
item: version unreadable
promise: `unable: the CLI version could not be read`, followed by its own hint, exit 1.
@row_check n=299 row=contract/viola-verify-refusals/version-unreadable sha=6e27b27f44a9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:196-197
quote: "the CLI version could not be read", "run claude --version to check the install",
@row_add n=300 row=contract/viola-verify-refusals/unclean-fixture area=surface rec=fct-sedcqxww@2
boundary: viola verify refusals
item: unclean fixture
promise: `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.
@row_check n=300 row=contract/viola-verify-refusals/unclean-fixture sha=1e44f4b1de92 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:626-627
quote: &format!("a recorded fixture is not clean: {named}"), "record with a viola home under your user home",
at: src/cmd/verify.rs:603
quote: return Err(format!("{name} row {}{at} {}", fault.row, fault.why.code()));
at: crates/viola-agent-claude/src/ledger.rs:1306
quote: Self::HomePath => "home-path",
@row_add n=301 row=contract/cli-exit-codes/10 area=surface rec=fct-f5dfwzmp@1
boundary: CLI exit codes
item: 10
promise: `unable  human-typing` (+ `manual-pause`); under `--json` `{"v":1,"refusal":"human-typing","detail":…}`.
@row_check n=301 row=contract/cli-exit-codes/10 sha=0d63f8bd738a v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
at: src/run/send.rs:99-100
quote: RefusalReason::HumanTyping, detail.map(HumanTyping::as_str),
at: crates/viola-core/src/lib.rs:242
quote: Self::ManualPause => "manual-pause",
at: src/cmd/send.rs:133
quote: return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
@row_add n=302 row=contract/cli-exit-codes/11 area=surface rec=fct-spwfscwz@1
boundary: CLI exit codes
item: 11
promise: `unable  budget-paused  five-hour` or `seven-day`; under `--json` the refusal object.
@row_check n=302 row=contract/cli-exit-codes/11 sha=ce54734eda6a v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
at: src/run/send/budget.rs:12
quote: Some(crossed.window.as_str()),
at: crates/viola-core/src/lib.rs:259-260
quote: Self::FiveHour => "five-hour", Self::SevenDay => "seven-day",
@row_add n=303 row=contract/cli-exit-codes/12 area=surface rec=fct-vocxcyzi@1
boundary: CLI exit codes
item: 12
promise: `unable  unverified-cli`; under `--json` the refusal object.
@row_check n=303 row=contract/cli-exit-codes/12 sha=024b986fb265 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=304 row=contract/cli-exit-codes/13 area=surface rec=fct-k7c3yar5@1
boundary: CLI exit codes
item: 13
promise: `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
@row_check n=304 row=contract/cli-exit-codes/13 sha=955488f219bd v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
at: src/run/send.rs:126
quote: return ctx.not_delivered(None, NotDelivered::InputNotReady);
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/run/send.rs:116
quote: return ctx.not_delivered(None, NotDelivered::TurnRunning);
at: src/run/dialog.rs:446
quote: Some(NotDelivered::UnknownDialog.as_str()),
at: crates/viola-core/src/lib.rs:358
quote: Err(NotDelivered::ControlCharacter)
at: src/run/send.rs:94
quote: return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=305 row=contract/cli-exit-codes/14 area=surface rec=fct-f6euya2e@1
boundary: CLI exit codes
item: 14
promise: `unable  unknown  <detail>`, the detail a code word or nothing; under `--json` the refusal object.
@row_check n=305 row=contract/cli-exit-codes/14 sha=cf8ff1e62739 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
at: src/cmd/client.rs:80
quote: RefusalReason::Unknown.as_str().to_owned()
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
at: src/cmd/send.rs:136
quote: Some(detail) => format!("{reason}  {detail}"),
@row_add n=306 row=contract/exit-21-json-cause/strict-modes-failed area=surface rec=fct-hgef4h2n@1
boundary: exit 21 --json causes
item: strict-modes-failed
promise: The `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.
@row_check n=306 row=contract/exit-21-json-cause/strict-modes-failed sha=e87acc6b6af2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:93
quote: let cause = UnreachableCause::StrictModes.as_str();
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/plugin.rs:55
quote: client::unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/unlink.rs:15
quote: let code = link_call(home, args, Verb::Unlink, request)?;
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
@row_add n=307 row=contract/exit-21-json-cause/instance-dead area=surface rec=fct-hgef4h2n@1,fct-zp4y2mcv@1,fct-aej2ym2d@1,fct-ihyxsaec@1
boundary: exit 21 --json causes
item: instance-dead
promise: With no live endpoint, an instance directory that is a directory reads `instance-dead`, as do a failed connect and a frame verb against a stale instance; stale heartbeat is exit 1's cause alone.
@row_check n=307 row=contract/exit-21-json-cause/instance-dead sha=55d72e66a105 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:112
quote: Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead,
at: src/cmd/client.rs:196-197
quote: Err(CallError::Channel(ChannelError::Connect(_))) => { return Ok(Err(Unreachable::Dead));
at: crates/viola-state/src/liveness.rs:97-98
quote: if classify(age, same_process(&snapshot)) != Liveness::Live { return None;
@row_add n=308 row=contract/exit-21-json-cause/unwrapped-name area=surface rec=fct-hgef4h2n@1,fct-zp4y2mcv@1
boundary: exit 21 --json causes
item: unwrapped-name
promise: With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent, a regular file or a symlink reads `unwrapped-name`.
@row_check n=308 row=contract/exit-21-json-cause/unwrapped-name sha=7c2bb60e1e86 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
at: crates/viola-state/src/liveness.rs:111
quote: match std::fs::symlink_metadata(instance_dir) {
@row_add n=309 row=contract/exit-21-json-cause/endpoint-lost area=surface rec=fct-hgef4h2n@1,fct-aej2ym2d@1
boundary: exit 21 --json causes
item: endpoint-lost
promise: A wrapper lost with the call open reads `endpoint-lost`.
@row_check n=309 row=contract/exit-21-json-cause/endpoint-lost sha=916c440e0520 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
@row_add n=310 row=contract/exit-21-json-cause/server-verify-failed area=surface rec=fct-hgef4h2n@1,fct-pxdkcmfq@2
boundary: exit 21 --json causes
item: server-verify-failed
promise: A process on the endpoint that is not the recorded wrapper (pid or start time differs, the check cannot be made, or the peer's euid is not this user's or cannot be read) gets no frame; the eight frame-writing verbs read `server-verify-failed`.
@row_check n=310 row=contract/exit-21-json-cause/server-verify-failed sha=39c3e7625ddb v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177-178
quote: if !peer_admitted(client.server_euid()) { return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:122
quote: Self::ServerUnverified => UnreachableCause::ServerVerify,
at: src/cmd/send.rs:233
quote: Ok(client::connect(live)?.request("send", params)?)
at: src/cmd/wait.rs:95
quote: Ok(client::connect(live)?.request("wait", params)?)
at: src/cmd/last.rs:67
quote: Ok(client::connect(live)?.request("last", params)?)
at: src/cmd/answer.rs:152
quote: Ok(client::connect(live)?.request("answer", params)?)
at: src/cmd/pause.rs:93
quote: Ok(client::connect(live)?.request("pause", params)?)
at: src/cmd/release.rs:77
quote: Ok(client::connect(live)?.request("release", params)?)
at: src/cmd/link.rs:131
quote: Ok(client::connect(live)?.request("link", params)?)
at: src/cmd/unlink.rs:21
quote: Ok(client::connect(live)?.request("unlink", params)?)
@row_add n=311 row=contract/exit-21-json-cause/socket-dir-refused area=surface rec=fct-hgef4h2n@1
boundary: exit 21 --json causes
item: socket-dir-refused
promise: `socket-dir-refused` is one of the six closed causes of exit 21's `--json` document.
@row_check n=311 row=contract/exit-21-json-cause/socket-dir-refused sha=2b8805a45fb9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=312 row=contract/viola-link-json/ok area=surface rec=fct-7mkzemsx@1
boundary: viola link --json
item: ok
promise: Exit 0 prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link.
@row_check n=312 row=contract/viola-link-json/ok sha=20d7b24e4629 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:121
quote: client::document(&json!({"v": 1, "ok": ok}));
at: src/run/links.rs:75
quote: Ok(json!({"ok": {"since": since, "already_linked": already_linked}}))
@row_add n=313 row=contract/viola-unlink-json/ok area=surface rec=fct-7mkzemsx@1
boundary: viola unlink --json
item: ok
promise: Exit 0 prints `{"v":1,"ok":{}}`.
@row_check n=313 row=contract/viola-unlink-json/ok sha=3612d0554404 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:82
quote: Ok(json!({"ok": {}}))
at: src/cmd/link.rs:121
quote: client::document(&json!({"v": 1, "ok": ok}));
@row_add n=314 row=contract/viola-link-output/refused-pair area=surface rec=fct-3vwzogmp@1
boundary: viola link and unlink output
item: refused pair
promise: A pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself) exits 20: `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
@row_check n=314 row=contract/viola-link-output/refused-pair sha=ecffa14f7968 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:95-96
quote: if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
at: src/cmd/link.rs:116
quote: Err(detail) => return Ok(client::fault(args.json, &detail)),
at: src/cmd/client.rs:244-245
quote: run::log_self_exit(20, Some("wrapper-fault")); 20
@row_add n=315 row=contract/wrapper-fault-detail/message area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: message
promise: Exit 20's `--json` `detail` is closed for the five known codes, and `message` is each code's fixed text.
@row_check n=315 row=contract/wrapper-fault-detail/message sha=de963f23b81f v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:124-127
quote: -32700 => (ProtocolError::Parse.message(), Value::Null), -32600 => (ProtocolError::InvalidRequest.message(), Value::Null), -32601 => (ProtocolError::MethodNotFound.message(), Value::Null), -32603 => (Pro
@row_add n=316 row=contract/wrapper-fault-detail/unsupported-version area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: -32602 unsupported version
promise: `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values.
@row_check n=316 row=contract/wrapper-fault-detail/unsupported-version sha=d4d001bc8a2d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:128
quote: -32602 => match (data["supported"].as_i64(), wrapper) {
at: crates/viola-channel/src/lib.rs:130
quote: ProtocolError::UnsupportedVersion.message(),
at: crates/viola-channel/src/lib.rs:131
quote: json!({"supported": supported, "wrapper": wrapper}),
@row_add n=317 row=contract/wrapper-fault-detail/release-from-driver area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: -32602 release-from-driver
promise: `-32602` with `data.reason` exactly `release-from-driver` reads that body.
@row_check n=317 row=contract/wrapper-fault-detail/release-from-driver sha=2853a133ed3d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:133-136
quote: _ if data["reason"] == "release-from-driver" => ( ProtocolError::ReleaseFromDriver.message(), json!({"reason": "release-from-driver"}), ),
@row_add n=318 row=contract/wrapper-fault-detail/other-invalid-params area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: other -32602
promise: Any other `-32602` reads `invalid params` with null `data`.
@row_check n=318 row=contract/wrapper-fault-detail/other-invalid-params sha=91c6e0c4bc92 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:137
quote: _ => (ProtocolError::InvalidParams.message(), Value::Null),
@row_add n=319 row=contract/wrapper-fault-detail/other-known-codes area=surface rec=fct-3b32n36m@1
boundary: wrapper fault --json detail
item: other known codes
promise: Every known code other than `-32602` has null `data`.
@row_check n=319 row=contract/wrapper-fault-detail/other-known-codes sha=9cddeb72d9d4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:124-127
quote: -32700 => (ProtocolError::Parse.message(), Value::Null), -32600 => (ProtocolError::InvalidRequest.message(), Value::Null), -32601 => (ProtocolError::MethodNotFound.message(), Value::Null), -32603 => (Pro
@row_add n=320 row=contract/wrapper-fault-detail/unknown-code area=surface rec=fct-utjcf3wu@1
boundary: wrapper fault --json detail
item: code outside the five
promise: A code outside the five known prints `error: wrapper fault  <code>`, and under `--json` `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
@row_check n=320 row=contract/wrapper-fault-detail/unknown-code sha=4b589748f737 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
at: src/cmd/client.rs:91
quote: json!({"v": 1, "error": "wrapper-fault", "detail": detail})
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
at: crates/viola-mcp/src/result.rs:60
quote: return sanitised_fault(fault)
@row_add n=321 row=decision/json-failure-documents-clean area=surface rec=fct-kqtzijw6@1
name: Clean --json failure documents
rule: Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
guard: tests/cli_error_bodies.rs
@row_check n=321 row=decision/json-failure-documents-clean sha=d889f83423ae v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:221
quote: let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
guard_at: tests/cli_error_bodies.rs:122
@row_add n=322 row=contract/cli-catch-site/json-internal-error area=surface rec=fct-arozinj6@1
boundary: CLI catch site
item: --json internal error
promise: Under `--json` the catch site prints the `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
@row_check n=322 row=contract/cli-catch-site/json-internal-error sha=5543683ce1a9 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67
quote: if json_asked(std::env::args_os()) {
at: src/cmd/mod.rs:101
quote: client::refused_document("internal-error");
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/main.rs:70
quote: human::internal_error();
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=323 row=contract/cli-catch-site/json-flag-scope area=surface rec=fct-jh6curc2@1
boundary: CLI catch site
item: --json flag scope
promise: The catch site counts `--json` only among the arguments before a `--`, read from argv.
@row_check n=323 row=contract/cli-catch-site/json-flag-scope sha=a5750a03b87a v=holds judge=sonnet head=8834195b0683
at: src/main.rs:150-154
quote: fn json_asked(args: impl IntoIterator<Item = OsString>) -> bool { args.into_iter() .skip(1) .take_while(|arg| arg != "--") .any(|arg| arg == "--json")
@row_add n=324 row=contract/cli-catch-site/silent-verbs area=surface rec=fct-ue5y52fu@1
boundary: CLI catch site
item: silent verbs
promise: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
@row_check n=324 row=contract/cli-catch-site/silent-verbs sha=62d0b3f6c7eb v=holds judge=sonnet head=8834195b0683
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
at: src/main.rs:98
quote: return if arg == "hook" {
at: src/main.rs:100
quote: } else if arg == "mcp" {
at: src/main.rs:102
quote: } else if arg == "revive" {
at: src/main.rs:109
quote: } else if arg == "run" || arg.to_str().is_some_and(|a| a.starts_with('-')) {
@row_add n=325 row=contract/viola-help/verb-groups area=surface rec=fct-pjtiz7xa@1
boundary: viola --help
item: verb groups
promise: `viola --help` lists the served verbs only, grouped `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.
@row_check n=325 row=contract/viola-help/verb-groups sha=a6769a2c3643 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
@row_add n=326 row=decision/cli-ascii-only area=typography rec=fct-qwc3u462@1,fct-ukpiycjz@2,fct-g2p3izdb@2
name: ASCII-only CLI output
rule: CLI output is ASCII only, separators included, at high density.
@row_check n=326 row=decision/cli-ascii-only sha=d7510d9c06f8 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
at: src/human/board/rows.rs:80
quote: const UNWRAPPED_SEPARATOR: &str = "-- UNWRAPPED - READ-ONLY --";
@row_add n=327 row=contract/cli-colour-tokens/attention area=surface-cli rec=fct-dufqiv2l@1,fct-erfdav33@1,fct-muqp52c7@1,fct-5r2kfdmj@1
boundary: CLI colour tokens
item: attention
promise: Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
@row_check n=327 row=contract/cli-colour-tokens/attention sha=964f99b41b9b v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:32
quote: Self::Truecolor => "\x1b[38;2;217;119;6m",
at: src/human/style.rs:33
quote: Self::Ansi256 => "\x1b[38;5;172m",
at: src/human/style.rs:34
quote: Self::Ansi16 => "\x1b[33m",
at: src/human/board/cells.rs:86
quote: format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
@row_add n=328 row=contract/cli-colour-tokens/reset area=surface-cli rec=fct-dufqiv2l@1,fct-qe6n467m@1
boundary: CLI colour tokens
item: reset
promise: `\x1b[0m` at every colour depth, after every styled span.
@row_check n=328 row=contract/cli-colour-tokens/reset sha=c1e26ed2195c v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:40
quote: reset: "\x1b[0m",
at: src/human/board/cells.rs:77
quote: let span = |word: &str, token: &str| format!("{token}{word}{}{base}", tokens.reset);
@row_add n=329 row=contract/cli-colour-depth/16-colour area=surface-cli rec=fct-2osj5znw@1,fct-ukqwfmzr@1
boundary: CLI colour depth
item: 16-colour
promise: Where no earlier depth matches, the depth is the 16-colour fallback column; the depth is chosen once per process, first match wins.
@row_check n=329 row=contract/cli-colour-depth/16-colour sha=d82e14b84af8 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:79
quote: Some(Depth::Ansi16)
@row_add n=330 row=absence/cli-tape area=surface-cli rec=fct-rhkxdzp7@1,fct-dbxcldrz@1,fct-kn56sdzi@2
capability: A tape in the CLI
@row_check n=330 row=absence/cli-tape sha=3e9582bcea79 v=holds judge=sonnet head=8834195b0683
searched: Tape|TAPE|tape · in src/**/*.rs
searched: COLUMNS · in src/**/*.rs (only a comment in src/cmd/list.rs:61 saying it is never read)
@row_add n=331 row=decision/no-hint-for-faults area=surface-cli rec=fct-5xeughsz@1,fct-pbvocw2a@1
name: No hint for faults and unknown
rule: `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`.
why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
@row_check n=331 row=decision/no-hint-for-faults sha=322a69062e41 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:242
quote: let _ = human::write_wrapper_fault(&mut io::stderr().lock(), code);
at: src/cmd/client.rs:245
quote: 20
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
at: src/cmd/send.rs:85
quote: RefusalReason::UnverifiedCli | RefusalReason::Unknown => None,
@row_add n=332 row=contract/viola-verify-output/step-line area=surface-cli rec=fct-nz6d6pbn@1,fct-sx3u7aez@2
boundary: viola verify output
item: step line
promise: One static appended uncoloured stdout line per relied ledger row, ending `pass` or `fail`, counted against `MM`, the relied count.
@row_check n=332 row=contract/viola-verify-output/step-line sha=e00204dbafc1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:486
quote: human::result(&step_line(n, total, row, pass));
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
@row_add n=333 row=decision/start-refusals-show-no-path-or-pid area=surface-cli rec=fct-w4pl2fni@1,fct-pbvocw2a@1
name: Start refusals show no path or pid
rule: The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
@row_check n=333 row=decision/start-refusals-show-no-path-or-pid sha=41039bc09277 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:716
quote: human::refuse(&format!("{name} is already live"), "viola list");
at: src/cmd/revive.rs:77-78
quote: /// The `unable:` and `hint:` texts: fixed words and the instance's name, never the recorded /// directory, a pid or an id read from the log.
@row_add n=334 row=contract/viola-revive-output/json-refusal area=surface-cli rec=fct-2t54hjhs@2,fct-gray6vne@1,fct-mep5dn6i@2,fct-sedcqxww@2
boundary: viola revive output
item: --json refusal
promise: Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
@row_check n=334 row=contract/viola-revive-output/json-refusal sha=959afc89941b v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/revive.rs:107
quote: client::refused_document(self.detail());
@row_add n=335 row=contract/cli-exit-codes/0 area=surface-cli rec=fct-tj4jq2zv@1,fct-nr7mro52@1
boundary: CLI exit codes
item: 0
promise: No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
@row_check n=335 row=contract/cli-exit-codes/0 sha=019c48c33ddb v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:110
quote: return client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/send.rs:114-115
quote: let _ = human::write_read_back( &mut io::stdout().lock(),
at: src/cmd/send.rs:102-104
quote: if issue_line_shows(self.json, io::stdout().is_terminal()) { let issued = Utc::now().format("%H:%M:%S%.3fZ").to_string(); let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
@row_add n=336 row=contract/cli-exit-codes/1 area=surface-cli rec=fct-tj4jq2zv@1,fct-sedcqxww@2
boundary: CLI exit codes
item: 1
promise: `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
@row_check n=336 row=contract/cli-exit-codes/1 sha=a9f38fae433d v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:67-68
quote: if json_asked(std::env::args_os()) { cmd::internal_error_document();
at: src/main.rs:138
quote: ) => ExitCode::from(1),
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=337 row=contract/cli-exit-codes/2 area=surface-cli rec=fct-tj4jq2zv@1,fct-4zlzvxze@1
boundary: CLI exit codes
item: 2
promise: Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`) or `UNUSABLE_TEXT` line (unusable `viola send`) on stderr, whatever `--json` says; no document.
@row_check n=337 row=contract/cli-exit-codes/2 sha=658826606d86 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/cmd/answer.rs:91-92
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes()); return Ok(ExitCode::from(2));
at: src/cmd/send.rs:164-165
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes()); return Ok(ExitCode::from(2));
@row_add n=338 row=contract/cli-exit-codes/20 area=surface-cli rec=fct-tj4jq2zv@1,fct-adve5pgh@1
boundary: CLI exit codes
item: 20
promise: `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
@row_check n=338 row=contract/cli-exit-codes/20 sha=31308885a51d v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
at: src/cmd/client.rs:91
quote: json!({"v": 1, "error": "wrapper-fault", "detail": detail})
at: src/cmd/client.rs:245
quote: 20
@row_add n=339 row=contract/cli-exit-codes/21 area=surface-cli rec=fct-tj4jq2zv@1,fct-hgef4h2n@1
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
@row_check n=339 row=contract/cli-exit-codes/21 sha=c379ae97031f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:271-276
quote: let hint = human::send_hint(name, cause.hint_cause()); let _ = human::write_unable( &mut io::stderr().lock(), name, "instance-unreachable", hint.as_deref(),
at: src/cmd/client.rs:37-38
quote: let (unable, hint) = human::instance_state_pair(name); human::refuse(&unable, &hint);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=340 row=absence/rendered-page area=border-radius rec=fct-k5ssxpy7@2,fct-hdk4heck@2,fct-rcoxrota@2,fct-73hf7cl5@2
capability: A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's
@row_check n=340 row=absence/rendered-page sha=2a33623839de v=holds judge=sonnet head=8834195b0683
searched: border-radius · in src/
searched: border_radius · in src/
searched: keyframes · in src/
searched: transition · in src/ (only a tracing span name `run.wheel_transition` at src/run/wheel.rs:277, no style token)
@row_none n=1 rec=fct-5eqdszko@1 type=0
what: a brand personality statement in metaphor
@row_none n=2 rec=fct-ggkugwi5@1 type=S
@row_none n=3 rec=fct-raot4n4o@1 type=S
@row_none n=4 rec=fct-le6pp75z@1 type=S
@row_none n=5 rec=fct-ajwnfbif@1 type=8
@row_none n=6 rec=fct-lx6q6b3t@1 type=S
@row_none n=7 rec=fct-trso3zrq@1 type=S
@row_none n=8 rec=fct-herdsm72@1 type=S
@row_none n=9 rec=fct-fyedyzk2@1 type=S
@row_none n=10 rec=fct-7ghkfhm4@1 type=S
@row_none n=11 rec=fct-3inm2aon@1 type=S
@row_none n=12 rec=fct-x2nilrtw@1 type=S
@row_none n=13 rec=fct-wj2ah4kx@1 type=S
@row_none n=14 rec=fct-b3qkww4h@1 type=S
@row_none n=15 rec=fct-f2egz6qm@1 type=S
@row_none n=16 rec=fct-wbrmyjrf@1 type=S
@row_none n=17 rec=fct-yvy7sgi6@1 type=S
@row_none n=18 rec=fct-eiyxwqq7@1 type=S
@row_none n=19 rec=fct-mfvo2or6@1 type=4
relied: A failed strict-modes check refuses the whole board with exit 21, in the fixed `unable:` / `hint:` form or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`.
@row_none n=20 rec=fct-dgyptkjb@1 type=S
@row_none n=21 rec=fct-vucxgwl4@1 type=S
@row_none n=22 rec=fct-mnnc3kso@1 type=S
@row_none n=23 rec=fct-2uvmlzuw@1 type=S
@row_none n=24 rec=fct-nrsgugtu@1 type=S
@row_none n=25 rec=fct-7btysobr@1 type=S
@row_none n=26 rec=fct-rvsvzrdz@1 type=S
@row_none n=27 rec=fct-oljfb7kw@1 type=S
@row_none n=28 rec=fct-ybq4pqty@1 type=S
@row_none n=29 rec=fct-tmafcfek@1 type=S
@row_none n=30 rec=fct-ok535url@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=31 rec=fct-mwfv3azd@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=32 rec=fct-hvwzk24f@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=33 rec=fct-kh7y66hg@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=34 rec=fct-a2vvlynz@1 type=0
what: a legend item of the generic expression-motion scale
@row_none n=35 rec=fct-ypei347w@2 type=5
