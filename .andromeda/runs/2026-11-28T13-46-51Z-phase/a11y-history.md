@history_item n=1 marker=amd-gb4zpgld
**amd-gb4zpgld** — (2026-11-28-console-scaffold) `~ decision/static-linear-output` and `~ decision/cli-not-keyboard-interactive` gained the named exception for `viola console`, its own surface: it redraws and reads keys from stdin in raw mode. The operator chose «Именное исключение». This is the precedent: any new key mode, selection redraw or text entry the actions add has to be written into these excepts, not into the base rules.

@history_item n=2 marker=amd-jrakrwd7
**amd-jrakrwd7** — (console-scaffold) `~ decision/cli-no-interactive-input` (§5 Keyboard Navigation) gained the same console exception: raw-mode keys and a redraw on resize. §5 stays N/A for the web and points the console's keyboard reach to «Fleet a11y verification». A send's text entry or a confirm prompt in the console would widen this except again.

@history_item n=3 marker=amd-dqq4etdo
**amd-dqq4etdo** — (console-scaffold) `~ contract/log-identity/process`: `viola console` logs as `process:"console"` with no instance. This is PROVISIONAL on a delegate's word. Matters if actions run through CLI verbs in-process or as child processes (a P3 premise): which process tag and instance their lines carry.

@history_item n=4 marker=amd-6lueixiq
**amd-6lueixiq** — (console-scaffold) `~ contract/cli-fixed-message/form` now names viola console's two refusals as fixed `unable:`/`hint:` pairs with no document. `~ decision/strict-modes-refusal-before-any-write` adds console to the exit-1 list. The console's own refusals take the one-line form, but an action's refusal must show the verb's own `unable  <name>  <reason>  <detail>` line. Keep the two forms apart.

@history_item n=5 marker=amd-pchaa2vn
**amd-pchaa2vn** — (2026-11-28-fleet-console-view) `~ decision/static-linear-output` except widened to a redraw when the fleet shown changes. This is PROVISIONAL (delegate viola-overseer). A redraw on selection move, scroll or an action's result line is a further widening, owed in this except.

@history_item n=6 marker=amd-bvnsjedg
**amd-bvnsjedg** — (fleet-console-view) `~ decision/cli-no-interactive-input` except widened to a redraw on a fleet change. This is PROVISIONAL. The row the actions' key modes and no-key-trap claim extend.

@history_item n=7 marker=amd-6tq6husi
**amd-6tq6husi** — (fleet-console-view) `~ decision/claude-agents-read-fails-open`: in the console a failed agents read hands an empty list, so STATUS reads unknown. `~ decision/strict-modes-refusal-before-any-write` except: the console's mid-run refusal restores the terminal first, then exits 1 with the home pair. Both are PROVISIONAL. This is why a send's readback must be the verb's reply and never STATUS (carry cry-qjw64paz). It is also the precedent for restoring the terminal before any refusal text after frames were drawn.

@history_item n=8 marker=amd-tj7y5ae5
**amd-tj7y5ae5** — (0.2.0 census) The frontend stubs were retired. No web or graphical frontend exists. The terminal fleet console (matrix#v2-14) takes its a11y scope at «Fleet a11y verification», §5–§7 read N/A, and the console reuses `viola list`'s caption order. This is where the console's keyboard reach is owned. Do not claim WCAG or ARIA for it.

@history_item n=9 marker=amd-v7ovniia
**amd-v7ovniia** — (2026-11-24-switching-a-session-s-model-and-effort) `~ contract/frame-verb-refusal/strict-modes-failed` and `~ contract/frame-verb-refusal/server-verify-failed` add `switch` to the verb lists. These are the refusals the console's switch action (carry cry-mf2w3ioc) must show unchanged.

@history_item n=10 marker=amd-rcmjjlon
**amd-rcmjjlon** — (switching) `~ contract/cli-exit-codes/10`: human-typing refuses an automated send or switch, nothing typed, no child replaced. It was ratified at amd-us7hgh7v. A console-driven send or switch reads as automated, so it is refused while the human holds the wheel. The console has to show that refusal as its `unable` line.

@history_item n=11 marker=amd-75puvsoa
**amd-75puvsoa** — (switching) `~ decision/send-refused-while-human-typing` names `send` and `switch`. It was ratified at amd-nub44zjy. This is critical-path P4 behaviour the console's actions inherit through the CLI verbs.

@history_item n=12 marker=amd-y32tu2ob
**amd-y32tu2ob** — (switching) `+ decision/respawn-reap-bound`: a switch's 10 s reap wait falls on the wrapper, never the human; otherwise it refuses not-delivered child-not-ended, exit 13. It was ratified at amd-nhjssb2n and amd-yevijrhn. A console switch may block its own surface up to this bound. That wait must not trap keys.

@history_item n=13 marker=amd-6zdhv76v
**amd-6zdhv76v** — (switching) `+ contract/viola-switch-output/result-line`: a switch prints one plain-ASCII line with no SGR. This is the result line the console's switch action shows as its readback.

@history_item n=14 marker=amd-kfy5w7r3
**amd-kfy5w7r3** — (2026-10-07-send-waits-out-the-paste-hint) The readiness gate's 8.5 s bound falls on the driver. A human key during the wait takes the wheel and the send is refused human-typing (`decision/human-key-takes-the-wheel`, guard `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`). The precedent for a console send taking seconds before its readback (P3 premise), and for its refusal outcome.

@history_item n=15 marker=amd-w5adl6jp
**amd-w5adl6jp** — (2026-10-18-compact-post-condition-ledger-row) The confirm windows are 10 s, and 40 s for `/compact`. An unconfirmed send is not-delivered / no-prompt-submitted, and the bound is on the driver only (`contract/send-refusal/no-prompt-submitted`). A console send's readback may take this long. The console must stay keyboard-free during it (no key trap) and show the refusal as its `unable` line.

@history_item n=16 marker=amd-qsik7n3f
**amd-qsik7n3f** — (2026-10-04-the-wheel) The non-editing set became the closed list of focus, mouse and terminal-reply sequences, and every other byte is editing (`decision/non-editing-sequences`, `contract/tui-passthrough/focus-mouse-resize-sequences`). This is the closest precedent for the console's new key-sequence decoder (carry cry-pjtbjcco). A grammar of exact shapes is used, with near neighbours treated as ordinary keys.

@history_item n=17 marker=amd-it5ecplj
**amd-it5ecplj** — (2026-10-08-first-live-test-and-self-drive) The non-editing list was extended by exact grammar, with one-field or one-prefix neighbours staying typing, proven by a unit table plus a probe on foot 1.28.0 (`decision/stdin-classifier-non-editing-list`). It also recorded the takeover's live reading, supplemental and never gating (`decision/live-readings-never-gate`). The model for proving the console's sequence decoder by a unit table, and for leaving any live reading ungated.

@history_item n=18 marker=amd-t4neqrve
**amd-t4neqrve** — (2026-11-05-linux-only-code-and-ci) The TUI is on openpty alone, and the F-W3 Windows readings were retired. Each boundary clause is its own nextest case on ubuntu-latest. This is the platform the console's keystroke driver over its own pseudo-terminal runs on.

@history_item n=19 marker=amd-2fp7yqxh
**amd-2fp7yqxh** — (2026-10-19-overseer-switch-to-viola) The takeover clause's second live reading: the founder's own key wrote `wheel{human}`, the next send exited 10, and his own `viola release` returned the wheel. This bears on the scope's premise that `release` is a human verb the console may offer. It also bears on `decision/critical-path-human-takes-wheel` (pause and release on the cli).

@history_item n=20 marker=amd-nx4hgeqm
**amd-nx4hgeqm** — (2026-10-31-exit-cause-code-catalogue) The exit-cause table in `tests/cross_exit_causes.rs` gives one hint per code (`no_two_codes_share_a_hint`), and `no_hint_names_release` holds every hint from naming `release` (`decision/no-hint-names-release`). Any console action hint or key legend must keep to this: no hint names release.

@history_item n=21 marker=amd-mmjwn3vl
**amd-mmjwn3vl** — (exit-cause-code-catalogue) The `unwrapped-name` and `endpoint-lost` hints end in `hint:` last, name the parsed `ViolaName`, and are pinned by literal. The console shows these refusals as the verb prints them. Names in hints are parsed arguments (`decision/printed-names-are-parsed-arguments`).

@history_item n=22 marker=amd-ruw2kqsf
**amd-ruw2kqsf** — (exit-cause-code-catalogue) Exit 21's `--json` document names its cause in `detail` from a closed list (`contract/cli-exit-codes/21`, `contract/instance-unreachable-detail/*`). This is the refusal set an action on a dead, unwrapped or refused instance surfaces. If the console reads the verb's machine form, these causes are what it gets.

@history_item n=23 marker=amd-iuxrncci
**amd-iuxrncci** — (exit-cause-code-catalogue) A name with no endpoint reads its instance directory: a directory gives `instance-dead`, anything else gives `unwrapped-name` (`contract/instance-unreachable-detail/unwrapped-name`, later narrowed by amd-ttj2fl5m). An action on an unwrapped row of the board meets `unwrapped-name`. The console's read-only unwrapped rows bear on which verbs a selection offers.

@history_item n=24 marker=amd-dh4gr6sw
**amd-dh4gr6sw** — (exit-cause-code-catalogue) A wrapper lost with the call open is `endpoint-lost`. A frame verb on a stale instance reads `instance-dead`. A start against a holder reads `stale-heartbeat` or `already-live` (`contract/start-holder-refusal/*`). These are the refusals a console open or revive action, and a send on a stale row, show.

@history_item n=25 marker=amd-wjuy56h4
**amd-wjuy56h4** — (2026-11-01-unix-endpoint-and-home-hardening) The eight frame verbs refuse a failing socket directory before any frame, with exit 21 `socket-dir-refused` and the hint last naming no path (`contract/frame-verb-refusal/socket-dir-refused`). This is part of the refusal set the console's actions inherit.

@history_item n=26 marker=amd-hv7cbqdf
**amd-hv7cbqdf** — (unix-endpoint-and-home-hardening) The start's socket-directory refusal is exit 1, in the fixed `unable:`/`hint:` form (`contract/start-refusal/socket-dir`, `decision/socket-dir-refusal-before-any-frame`). A console revive or open action that starts a child meets this form, not the frame-verb line.

@history_item n=27 marker=amd-mnwtaiyh
**amd-mnwtaiyh** — (2026-10-26-explicit-session-links) The server-verify refusal covers the eight frame verbs, with exit 21, the hint last and the `--json` document. `switch` was added later (amd-v7ovniia). This is the verb list a console action reaches through the shared `connect`.

@history_item n=28 marker=amd-mtfyjt22
**amd-mtfyjt22** — (linux-only) A home under a regular file is a strict-modes refusal with its hint (`decision/unstattable-home-is-strict-modes-refusal`). The Windows NotFound arm was retired. This is the strict-modes refusal set the console and its actions share.

@history_item n=29 marker=amd-iftqahhw
**amd-iftqahhw** — (2026-10-19-dialog-hold-for-the-driver) The dialog hold is configurable (60–3600 s) and bounds the driver only. Any human key hands a held dialog back at once (`decision/dialog-hold-bounds-the-driver`; made standing at amd-wekentmn). A console answer action races the hold. An expired dialog is refused not-delivered / unknown-dialog, exit 13 (`contract/cli-exit-codes/13`).

@history_item n=30 marker=amd-6ykawwgx
**amd-6ykawwgx** — (2026-11-22T17-29-41Z-wrap) `~ decision/critical-path-dialog-answer` now includes a question first raised by PermissionRequest. A console answer action sits on this must-be-accessible critical path, including the unverified-cli refusal.

@history_item n=31 marker=amd-2slcezqa
**amd-2slcezqa** — (2026-11-24-viola-up) `+ decision/open-live-wait-bound` (30 s), `+ decision/window-wait-bound` (5 s) and `+ decision/hyprctl-read-deadline` (5 s, viola focus among its bearers). Each falls on the verb, never on a key. They were ratified at amd-chrwe53y. Console open and focus actions inherit these waits. The F1 rule says a later bound gets a row only where it is new on a new bearer, which decides whether a console-side wait needs its own row.

@history_item n=32 marker=amd-x2vhdms7
**amd-x2vhdms7** — (2026-11-24-viola-focus) `~ decision/strict-modes-refusal-before-any-write` and `~ contract/cli-exit-codes/21` add `viola focus`, exit 21 before any compositor read. It was ratified at amd-chrwe53y (fork F-A: a command for the human only). The console's focus action reaches a human-only verb, which is the precedent for the human-verb premise.

@history_item n=33 marker=amd-562ap5xd
**amd-562ap5xd** — (viola-up) `~ decision/time-bounds-never-on-human` leads «Eight time bounds». New bounds take rows of their own (F1 option 1). The lead was ratified at amd-k2c3cma2 and amd-y3t2lind. A new console-side wait, for example on an action's readback, lands as its own row, never in this list.

@history_item n=34 marker=amd-y6gxuewh
**amd-y6gxuewh** — (viola-up) `~ decision/time-bounds-fall-on-the-driver` leads «Eight time bounds», with the same F1 rule. It was ratified at amd-p2adyjzy and amd-vgckgjq7. This is the twin list row for any new bound the actions add.

@history_item n=35 marker=amd-7g3eel46
**amd-7g3eel46** — (2026-11-23-viola-open-2) `~ contract/cli-exit-codes/2` adds viola open's exit-2 word refusal, one fixed stderr line that echoes no refused word. `decision/json-one-document-no-hint`'s except names it too. A console open action or send text entry that reaches an exit-2 path shows a fixed line, with no document.

@history_item n=36 marker=amd-6kt7moqw
**amd-6kt7moqw** — (2026-10-31-sanitised-error-surfaces) Under `--json`, the `answer` and `send` exit-2 paths write a fixed stderr line and no document. If the console reads a verb's machine form, an unusable send text or answer response gives no document.

@history_item n=37 marker=amd-xzhtjnkz
**amd-xzhtjnkz** — (sanitised-error-surfaces) `unknown`'s detail is a code word or nothing, and it prints no hint (`decision/unknown-refusal-has-no-hint`, `contract/cli-refusal/unknown-refusal`). A console showing an action's refusal must not invent a hint for `unknown`.

@history_item n=38 marker=amd-ahrrqlje
**amd-ahrrqlje** — (sanitised-error-surfaces) A peer's refusal is screened to a code word, and an unlisted code word stays visible with exit 14 (`decision/peer-replies-screened-to-code-words`, `decision/unlisted-code-words-stay-visible`). The console's readback shows the screened word, never peer text.

@history_item n=39 marker=amd-c3je5vt2
**amd-c3je5vt2** — (sanitised-error-surfaces) A fault's body is `error: wrapper fault  <code>` with no hint (`contract/human-fault-line/wrapper-fault`, `decision/faults-carry-no-hint`). An action's fault shows as this line, not an `unable` line.

@history_item n=40 marker=amd-muk7fxfb
**amd-muk7fxfb** — (exit-cause-code-catalogue) `revive --json` documents every refusal of the start it shares with run. A `claude` not found reads `internal-error` with no human pair (`contract/revive-json-refusal/internal-error`). A console revive action shows these. After the child holds the terminal viola writes nothing.

@history_item n=41 marker=amd-mj3eoy5f
**amd-mj3eoy5f** — (2026-10-10-viola-revive) `revive` joins the CLI verbs, and its exit-1 refusals are one `unable:` line plus one `hint:` line (`contract/revive-human-refusal/pair`). A passed revive enters the TUI passthrough boundary. A console revive action (matrix#v2-15) hands the terminal to a child whose boundary rows bind.

@history_item n=42 marker=amd-tfsofqpx
**amd-tfsofqpx** — (2026-11-24-viola-adopt) `~ contract/tui-passthrough/entry` adds a passed `viola adopt`. This is the list of ways into the passthrough. Any console action that starts a child in a terminal joins this list or stays outside it by name.

@history_item n=43 marker=amd-fyginvql
**amd-fyginvql** — (2026-11-24-adopt-s-resume-folder) `~ contract/cli-refusal/unable-line` counts adopt's three own refusals among the one-line `unable: <text>` forms. The latest form of the row that defines «refusals as `unable` lines», which the console's action refusals rest on.

@history_item n=44 marker=amd-rv4x7y4q
**amd-rv4x7y4q** — (2026-11-25-placing-without-moving-the-view) `~ contract/cli-fixed-message/form` adds verify's untrusted-folder refusal. This is the closed list of fixed-message forms that the console's own two refusals joined after it (amd-6lueixiq).

@history_item n=45 marker=amd-gnbjzc2t
**amd-gnbjzc2t** — (adopt-s-resume-folder) `~ decision/wheel-ignores-terminal-events` except: a not-resumed adopt's refusal reaches the human's terminal after the child ends. It was ratified at amd-j3j6jpwu. It is the precedent for viola's own bytes on the terminal after the child releases it, as in a console action whose child ends.

@history_item n=46 marker=amd-dnp5wv3m
**amd-dnp5wv3m** — (2026-11-22T17-29-41Z-wrap) `~ decision/wheel-ignores-terminal-events` and `~ decision/portable-pty-outer-pty-driver` excepts: the boundary checks hold from the child's start, and a keyed start's verify lines come before it. These are the boundary rows the console's own pty driver works beside.

@history_item n=47 marker=amd-refwghjv
**amd-refwghjv** — (2026-11-22T17-29-41Z-wrap) `~ contract/tui-passthrough/own-bytes` names a keyed start's automatic-verify lines before the child's output. This is the zero-viola-bytes promise for any child a console action starts.

@history_item n=48 marker=amd-o2edema5
**amd-o2edema5** — (2026-11-20-a-new-cli-version-stamped-automatically) `~ contract/tui-passthrough/human-keystrokes` and `~ contract/tui-passthrough/viola-terminal-bytes` hold from the child's start, with the automatic verify window the one exception. They were ratified at amd-j27p7xpb. These are the boundary promises a console-started child inherits.

@history_item n=49 marker=amd-okhtl4vb
**amd-okhtl4vb** — (a-new-cli-version-stamped-automatically) `~ decision/human-keystrokes-never-blocked` except for the automatic verify window, when a key waits in the terminal's own buffer. It was ratified at amd-hxjgyhjn. The never-blocked rule is the one that no key trap in the console must not undercut.

@history_item n=50 marker=amd-ffaj6zcz
**amd-ffaj6zcz** — (2026-11-22-paste-write-bounded) `~ decision/human-keystrokes-never-blocked`: a key waits at most behind the paste's in-flight chunk and closing marker. A console send is a paste on this queue, and a human key in the session still wins.

@history_item n=51 marker=amd-l6dqgcim
**amd-l6dqgcim** — (paste-write-bounded) `~ contract/tui-passthrough/human-keystrokes`: keys are queued with the paste in arrival order and never dropped. This is the twin contract row for a console-issued send's paste.

@history_item n=52 marker=amd-qihirloo
**amd-qihirloo** — (2026-10-27-bounded-inputs-at-every-boundary) After a contained vt100 feed panic, the driver's send is refused input-not-ready (exit 13) while a human key still reaches the child, and no viola literal reaches the terminal (`contract/tui-passthrough/after-feed-panic`). This is one refusal a console send's readback can show.

@history_item n=53 marker=amd-5s4as3gs
**amd-5s4as3gs** — (2026-11-18-session-folder-trusted-by-viola) `~ decision/run-diagnostics-off-terminal`: the start's trust step reports in one `trust-record` line in `diagnostics/`, never as a refusal or a hint. Diagnostics of actions that start a child go off the terminal, never onto the console's surface.

@history_item n=54 marker=amd-mgxi75yc
**amd-mgxi75yc** — (2026-10-17-cli-output-discipline) The colour spans were built: bold NAME, amber only on DIALOG, dim stale, each beside its word, with no SGR under `--json`, a pipe, NO_COLOR or `TERM=dumb` (`contract/viola-list-board/colour`, `decision/colour-decision-inputs`). Any selection highlight the console adds is a new SGR span. It needs a word or glyph beside it (`decision/colour-second-cue-only`), and `unable`/`fail` are never coloured.

@history_item n=55 marker=amd-hwhn4er6
**amd-hwhn4er6** — (2026-10-11-the-board-viola-list) The board caption order is pinned by the `CAPTION` literal, and each state is carried by its word. The console reuses these captions (amd-tj7y5ae5), and a selection names its row by the board's item.

@history_item n=56 marker=amd-yuipwrxy
**amd-yuipwrxy** — (2026-10-16-unwrapped-sessions-on-the-board) The unwrapped separator and read-only rows show the escaped `sessionId` and `n/a` cells (`contract/viola-list-board/unwrapped-separator`). Unwrapped rows are read-only, which bears on which verbs a selected unwrapped row may offer.

@history_item n=57 marker=amd-qkyodghu
**amd-qkyodghu** — (2026-11-22-lineage-registry) `~ contract/viola-list-board/order`: wrapped rows are drawn as a tree, followed by unwrapped rows and then the `-- LINKS --` block. This is the row order a keyboard selection walks.

@history_item n=58 marker=amd-d7o3hjai
**amd-d7o3hjai** — (lineage-registry) `~ decision/terminal-structural-equivalents` adds the `-- LINKS --` separator and the plain-space NAME indent of tree depth (the §4 twin is at amd-2btj5ldw). A selection marker must not stand in for these structural carriers or break them.

@history_item n=59 marker=amd-dhjwvlu4
**amd-dhjwvlu4** — (2026-11-06-version-done-check) Refusal layout, typed exit codes and a hint-free `--json` are held by literal asserts, and every fixed CLI string is pinned by literal (`decision/refusal-layout-literal-pins`, `decision/fixed-cli-strings-pinned`). New console copy, such as key legends, prompts or result framing, takes literal pins.

@history_item n=60 marker=amd-xfp5zvcf
**amd-xfp5zvcf** — (version-done-check) The cli checks are `std::process::Command` with literal pins, standing in for SC 1.3.2, 1.4.1 and 3.3.1 with no conformance claim (`decision/output-discipline-stands-in-for-sc`, `decision/cli-a11y-by-literal-tests`). The console actions' a11y proof takes this form.

@history_item n=61 marker=amd-wojncy7h
**amd-wojncy7h** — (version-done-check) The tui boundary driver is portable-pty `=0.9.0`, inherited and never re-pinned (`decision/tui-boundary-driver-pin`, `decision/portable-pty-outer-pty-driver`). The console's keystroke driver over its own pseudo-terminal sits on the same pin.

@history_item n=62 marker=amd-ob6ilhui
**amd-ob6ilhui** — (linux-only) The cli and tui a11y cases run in the `test` job's `coverage` suite on ubuntu-latest, with `gate --require coverage,doctest` as the verdict. This is where new console action cases land. They get no separate a11y driver (`decision/no-a11y-tool-output-discipline-assertions`).

@history_item n=63 marker=amd-ra6p35f6
**amd-ra6p35f6** — (linux-only) The manual screen-reader pass is Orca alone, founder-owned, and gates nothing (`decision/manual-orca-pass`, `decision/manual-a11y-pass-gates-nothing`). Any screen-reader reading of the console's actions is supplemental and never a gate, in line with the scope's GATED live-reading boundary.

@history_item n=64 marker=amd-hcrd4t4i
**amd-hcrd4t4i** — (explicit-session-links) Names printed in result lines are the verb's own parsed arguments, never read back from a reply or the log (`decision/printed-names-are-parsed-arguments`). A console action's readback line names its row's parsed name.

@history_item n=65 marker=amd-e7zy6lng
**amd-e7zy6lng** — (2026-11-21-key-after-a-resize) `~ decision/no-screen-content-parsing` except: the test-only rig reads closed words, never text. It was ratified at amd-ay7cmzcu. The bound behind the scope's premise that the console's test driver reads state, never the screen.
