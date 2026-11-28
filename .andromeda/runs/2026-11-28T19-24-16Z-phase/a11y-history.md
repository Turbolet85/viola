@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (1. A11y Scope Summary: 2 rows) — `~ decision/static-linear-output` and `~ decision/cli-not-keyboard-interactive` gained the named exception «the terminal fleet console (`viola console`), its own surface» (redraw on resize; raw-mode keys), on the operator's «Именное исключение (Recommended)». This is the precedent F-R repeats for security-plan `decision/refusals-never-block-human`: a named-surface exception on a row, no origin change. Any new console redraw or key behaviour (the F-K key line) has to stay inside these exceptions or amend them.

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (5. Keyboard Navigation: 1 row; 1. A11y Scope Summary: `contract/log-identity/process`) — `~ decision/cli-no-interactive-input` gained the console exception. `process:"console"` with no instance was added on a delegate's word, PROVISIONAL. That row is the keyboard-area basis for the F-K key line, and a console that drives starts/restores still logs as `console`.

@history_item n=3 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (8. Cognitive Accessibility: 2 rows) — `~ contract/cli-fixed-message/form` now names «viola console's two refusals», and `~ decision/strict-modes-refusal-before-any-write` added console to its exit-1 list. These are the console refusal forms the F-R exception speaks of: the verb's own lines shown to the human who typed the action.

@history_item n=4 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (1. A11y Scope Summary: 1 row) — `~ decision/static-linear-output` widened the console except to «or the fleet it shows changes», PROVISIONAL (delegate viola-overseer). A fleet started or restored by `viola up` changes what the console shows, so that redraw is already covered; no re-claim of v2-14.

@history_item n=5 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (5. Keyboard Navigation: 1 row) — `~ decision/cli-no-interactive-input` except widened to «redraws on a resize or a change in the fleet it shows», PROVISIONAL. This is the keyboard-area row the key-line change sits beside.

@history_item n=6 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (8. Cognitive Accessibility: 2 rows) — `~ decision/claude-agents-read-fails-open` now says a failed read in the console hands an empty list (STATUS `unknown`). `~ decision/strict-modes-refusal-before-any-write` gained the except for the console's mid-run refusal: the terminal is restored first, then exit 1 with the home pair. Both are PROVISIONAL on delegate words. The live-step guard (no `claude` on PATH / a private config) makes the agents read fail, so the console and `revive`/`adopt` preflight take the fail-open path. The mid-run refusal is a refusal shown after frames were drawn, which F-R's exception must stay consistent with.

@history_item n=7 marker=2026-11-28-console-actions
**2026-11-28-console-actions** (1. A11y Scope Summary: 1 row) — `~ decision/static-linear-output` except widened to «or a key moves its selection or opens a prompt», PROVISIONAL for the epoch batch. This is the last state of the console exception the F-K key line and any further console change extend. Its PROVISIONAL mark still waits on the founder's epoch batch.

@history_item n=8 marker=2026-11-28-console-actions
**2026-11-28-console-actions** (5. Keyboard Navigation: 2 rows) — `+ decision/console-no-key-trap` (guard `console_ctrl_c_ends_from_every_mode`, provisional) and `+ decision/console-editing-line-exception` (one editing line for send, switch and open), PROVISIONAL. The F-K key line lists exactly these actions (`s send  m sw  a ans  o open`, confirm/escape modes). Shortening its labels must keep every mode leavable by one key and must not add a second interactive input.

@history_item n=9 marker=2026-11-28-console-actions
**2026-11-28-console-actions** (6. Visual Design Verification: 1 row) — `+ decision/console-selection-by-gutter-mark`: the console selection is carried by a gutter mark, not colour alone (PROVISIONAL). Fitting the key line to 79 columns at the 80-column floor must not take the gutter column or turn the selection into a colour-only cue.

@history_item n=10 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — retired the frontend stubs. The terminal fleet console (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification», and §5 names the console's keyboard reach as that entry's. The scope says «Fleet a11y verification» follows this chunk, so the console's full keyboard/screen-reader reading belongs to that entry, not here.

@history_item n=11 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (1. A11y Scope Summary: 1 row) — `+ contract/viola-up-output/instance-line`: one plain-ASCII line per instance, no SGR, each in one write. Ratified at 2026-11-27T19-07-11Z-wrap. This is the line R4b reads («one line each»), and anything H1/H2 adds to `viola up` (a resumed session, model/effort, workspace) must keep this form.

@history_item n=12 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (8. Cognitive Accessibility: exit 21; the two time-bound lists; 3 new bound rows) — `~ contract/cli-exit-codes/21` added «viola up on the board's strict-modes refusal». The two lists now lead «Eight time bounds». `+ decision/open-live-wait-bound` (30 s), `+ decision/window-wait-bound` (5 s) and `+ decision/hyprctl-read-deadline` (5 s) were added, operator F1 option 1: «a later bound a row only where it is new on a new bearer». Any new wait this chunk adds to restore or start (a read-back that an instance came up, a wait for a resumed session) takes its own row only if it is a new bound on a new bearer, and must fall on viola's own start, never on a human key.

@history_item n=13 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (8. Cognitive Accessibility: 2 rows; then the F-A/F1 ratification entry) — `viola focus` joined the strict-modes exit-21 list and the exit-21 promise «before any compositor read». The ratification entry records that focus's 5 s compositor read is no new bound, because `decision/hyprctl-read-deadline` already names it. This is the precedent for R7's unmeasured focus-landing reading and for read-only compositor queries: no new bound row when the seam's existing deadline covers it.

@history_item n=14 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (1. A11y Scope Summary; 2. Strategy; 8. Cognitive Accessibility) — adopt's start was written into these rows:
- `contract/start-refusal/home-strict-modes`, `contract/start-refusal/socket-dir` and `contract/tui-passthrough/entry` (a passed adopt enters the passthrough);
- `contract/cli-refusal/unable-line`, `contract/cli-fixed-message/form` and `contract/start-json-refusal/document`;
- `decision/strict-modes-refusal-before-any-write`, `decision/socket-dir-refusal-before-any-frame`, `contract/json-fault/internal-error` and `decision/claude-agents-read-fails-open`.

These are the rows any change to the start path shared by run/revive/adopt must keep true. «An instance driven by another binary» (H1) lands at adopt.

@history_item n=15 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (8. Cognitive Accessibility: the «Eight» leads ratified, 1–4 of 4, and the state flips) — `decision/time-bounds-never-on-human` and `decision/time-bounds-fall-on-the-driver` went from provisional to standing, the lead «Eight time bounds» the operator's own. The lists are full at their bound: a new bound lands as a row of its own (see 2026-11-22-epoch-4-cleanup), not as a ninth list member.

@history_item n=16 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** (2. Strategy; 8. Cognitive Accessibility; 4. ARIA Patterns & Roles) — adopt's third own refusal, `not-resumed`, was added to `contract/cli-refusal/unable-line`, `contract/cli-fixed-message/form` and `contract/start-json-refusal/document`. `decision/wheel-ignores-terminal-events` gained a second exception: after the child ends, a not-resumed adopt's pair or document reaches the terminal. «A session resumed by its own id or the folder's newest transcript» touches this resume path. A resume that fails after the child ran must follow this end-of-child precedent.

@history_item n=17 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (1. Scope; 4. ARIA; 8. Cognitive) — rows changed:
- `viola switch` was added to `contract/frame-verb-refusal/strict-modes-failed` and `/server-verify-failed`, `contract/cli-exit-codes/10` (switch refused `human-typing`, no child replaced) and `decision/send-refused-while-human-typing`;
- `+ decision/respawn-reap-bound` (10 s, `RESPAWN_REAP_BOUND`);
- `+ contract/viola-switch-output/result-line`.

R6a/R6b (List B) read exactly these: the old child reaped inside `RESPAWN_REAP_BOUND`, a switch refused mid-turn. «The model and effort handed again» on restore has to sit beside the switch's result line and refusals.

@history_item n=18 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — ratified, on the operator's own word, `decision/respawn-reap-bound` (now standing), the viola-up instance-line row, the switch rows (exit 10, `send-refused-while-human-typing`) and adopt's `wheel-ignores-terminal-events` except. The rows this chunk's switch and up readings rest on stand on his word, not a delegate's.

@history_item n=19 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (8. Cognitive Accessibility: 3 rows) — `~ contract/cli-exit-codes/2` added viola open's exit-2 refusal of a word after `--` other than `claude` alone or a parent equal to the name, as one fixed line that echoes none of the refused words. `~ decision/json-one-document-no-hint` and `~ contract/cli-refusal/json` now name the open exit-2 path. «A terminal per instance on a named workspace» and «an opening message» run through `viola open`/`up`, and any new argument form there must keep this exit-2 discipline.

@history_item n=20 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** (8. Cognitive Accessibility: 1 row) — `~ contract/cli-fixed-message/form` added viola verify's untrusted-folder refusal, on the operator's word. This chunk is about placing windows without moving his view, and it sits beside R7's unmeasured «where focus lands after the window step's switch back».

@history_item n=21 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (7. Screen Reader; 8. Cognitive; 1. Scope) — rows changed:
- `~ decision/wrapped-tui-reads-as-unwrapped` except: in an untrusted folder a wrapped claude opens on its session box, the start having written the trust record.
- Both time-bound lists gained «the start's Claude config lock wait».
- `~ decision/run-diagnostics-off-terminal`: the trust step reports one `trust-record` line in `diagnostics/`, never a refusal, exit code or hint.

A fleet restored into its folders goes through this trust step. A live step under a private config must not read or copy the founder's `~/.claude*` (U6 arm 3).

@history_item n=22 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (1. Scope: 2 rows; 2. Strategy: 1 row; ratified 2026-11-22T17-29-41Z-wrap) — `contract/tui-passthrough/human-keystrokes` and `/viola-terminal-bytes`, plus the except on `decision/human-keystrokes-never-blocked`, now say the promises hold from the child's start, and the start's automatic verify window is the one exception. A start or restore by `viola up`/`revive`/`open` that runs automatic verify inherits this window, and no further pre-child window may be added without amending these rows.

@history_item n=23 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (1. Scope: 1 row; 2. Strategy: 1 row) — `contract/tui-passthrough/human-keystrokes` and `decision/human-keystrokes-never-blocked` now bound a key's delay by the paste's in-flight chunk and closing marker (paste chunks on one queued writer). «An opening message into a resumed session» is a paste/send into the child and falls under this bound. It must not delay a human key past it.

@history_item n=24 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. Scope; 3. Harness; 4. ARIA; 8. Cognitive; 11. Anti-Patterns) — rows changed:
- `~ contract/tui-passthrough/own-bytes`, and the excepts on `decision/zero-viola-bytes-on-the-terminal`, `decision/wheel-ignores-terminal-events` and `decision/portable-pty-outer-pty-driver`: a keyed start's automatic-verify lines precede the child's bytes.
- Both time-bound lists gained «paste write».
- `~ decision/critical-path-dialog-answer` added a question first raised by PermissionRequest.
- `+ decision/compiled-shapes-over-signatures`.

All on the operator's «Привести к построенному». These are the zero-bytes and critical-path rows the restore/start path and the wait-that-prints-a-pending-dialog must keep.

@history_item n=25 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** (8. Cognitive Accessibility: 1 row) — `+ decision/start-verify-stop-grace-off-the-human`, landed as a row of its own because the two time-bound lists are full at their bound (the operator's ruling for full lists). This is the form any new bound from this chunk takes.

@history_item n=26 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1. Scope: 2 rows; 4. ARIA: 1 row) — `~ contract/viola-list-board/order` now has the wrapped rows drawn as a tree, then the `-- LINKS --` block. `decision/terminal-structural-equivalents` and `decision/cli-structural-landmark-equivalents` gained the `-- LINKS --` separator and the NAME indent, on the operator's F4/F5. The lineage readers that restore «parent first» rest on this registry, and the board/console show the restored tree through these structure carriers.

@history_item n=27 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (8. Cognitive Accessibility: 2 rows) — «MCP call deadline» was added to both time-bound lists, and `+ contract/instance-unreachable-detail/call-deadline`. It is one of the eight bounds the lists now hold. A fleet «driven» by overseers through MCP keeps these bounds on the driver.

@history_item n=28 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (11. Anti-Patterns: 1 row) — `~ decision/no-screen-content-parsing` gained the except for the test-only standing live rig reading closed words only. Its raw frames stay local and gitignored. This chunk's live rigs and steps (`scripts/r*.sh` and the reboot reading's script) read the vendor screen only within this exception. Any new reading must amend it, as the four entries after it did.

@history_item n=29 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (11. Anti-Patterns: 1 row) — the live-rig except on `decision/no-screen-content-parsing` gained the box row's cell attributes after a turn end, as closed words, never text. This is part of the except chain a new live reading extends.

@history_item n=30 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (11. Anti-Patterns: 1 row; ratified 2026-11-22T17-29-41Z-wrap) — the rig except now covers cell attributes «after a turn end or send». This is the same except chain.

@history_item n=31 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (11. Anti-Patterns: 1 row; ratified) — the rig except gained «before any send», and the cell was shortened to its bound. The except is at its bound, so a new member must shorten the wording or take a row of its own.

@history_item n=32 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** (11. Anti-Patterns: 1 row; ratified) — the rig except gained «or KR resize». This is the current text of the picked row `decision/no-screen-content-parsing`'s except, the last state a live reading in this chunk would extend.

@history_item n=33 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (11. Anti-Patterns: 1 row; ratified) — the `decision/no-screen-content-parsing` rule gained «the compiled shapes over them». The rule's current text is what this chunk's live steps must respect.

@history_item n=34 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (11. Anti-Patterns: 1 row; ratified 2026-11-27T19-07-11Z-wrap) — `decision/no-screen-content-parsing` now names the input box's rule and prompt rows, reading the prompt row only for its opening char. A read-back that a resumed instance came up must use these signatures, never the text after the prompt.

@history_item n=35 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (§3 Keyboard test harness) — the takeover clause's second live reading came from the founder's own key on the product binary over a live claude session, supplemental and never gating (`decision/live-readings-never-gate`). It is the precedent for List B: live readings at his host, his to time, gate nothing in CI. It is also the precedent for moving overseers onto viola (the «overseers wrapped» of v2-28).

@history_item n=36 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (§3 Keyboard test harness) — this entry recorded the first live reading of the takeover clause on Linux (a `wtype` key on a compositor the test started itself), supplemental and never gating. It is the precedent that a live step uses its own compositor and terminal, never the founder's desktop.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the manual screen-reader pass; the Linux-only platform line) — the manual pass became Orca + GNOME Terminal for the cli and Orca + foot on Hyprland for the tui, founder-owned and gating nothing. The new platform line reads Linux only, 0.2.0 window features on Hyprland (`decision/linux-only`, `decision/manual-a11y-pass-gates-nothing`). The fleet's windows and workspaces are Hyprland features, and the live readings are his, on foot.

@history_item n=38 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (revive's start refusals without batch-script-child) — revive `--json`'s exit-1 codes dropped to ten. This is the closed set `contract/revive-json-refusal/*` and `contract/viola-revive-json/start-refusal` hold. A restore-after-reboot path through revive must not add a code without the founder's word.

@history_item n=39 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start; a lost wrapper and a stale holder) — `revive` now prints one exit-1 document for each refusal of the start it shares with `run`, and a `pump_child` error prints no document. A start against a `stale` holder reads `stale-heartbeat`, and `already-live` reads only for a live holder or a fresh replayed beat. After a real reboot every holder is stale, so H2's restore meets `contract/start-holder-refusal/stale-heartbeat`. R4b's «one line each» must read through these codes.

@history_item n=40 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start: exit 1, revive's code) — `viola run` and a `viola revive` start over a failing socket directory refuse with exit 1 before any snapshot, bind or child, and revive `--json` gives `socket-dir-refused`. After a reboot the runtime socket directory is created afresh, and the restore path meets this refusal (`contract/start-refusal/socket-dir`, `contract/revive-json-refusal/socket-dir-refused`).

@history_item n=41 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive` takes `--json` on both arms. The strict-modes or preflight exit-1 refusals each give one hint-free document, and the human pair stays byte-identical. This is the base of the revive json rows a restore driver (`viola up` over revive) reads.

@history_item n=42 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive` joined the CLI verbs. A passed revive enters the TUI passthrough boundary and writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`). Exit-1 refusals take the fixed-message pair. This is the origin of every revive row picked here; H2's restore is a revive per instance.

@history_item n=43 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (§8 Timeout extensions) — `list` and `revive` each make one `claude agents --json` read with a 5 s kill, fail-open, and the bound falls on viola's own child. This is the read the live-step guard (no `claude` on PATH) makes fail, which is the origin of `decision/claude-agents-read-fails-open`.

@history_item n=44 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** (§8 Timeout extensions; standing since 2026-11-15-live-round-on-the-trimmed-product) — the dialog hold became configurable 60–3600 s, bounding the driver, never the human. Any human key hands the held dialog back. «A wait that prints the last message and a pending dialog's text» must leave this hold and `decision/dialog-hold-bounds-the-driver` as they are.

@history_item n=45 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (1. Scope: 1 row; ratified 2026-11-22T17-29-41Z-wrap) — `~ contract/viola-verify/step-line` became pass|absent|fail, absent only on the browser-tools-prompt row. A start's automatic verify on a restored session prints these lines before the child.

@history_item n=46 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour spans were built. The colour decision reads `NO_COLOR`/`TERM`/`COLORTERM`/TTY. The board's cut width comes from `viola_pty::host_size()`, and an unwrapped NAME is cut with `...` only on a narrow terminal (the width source was ratified at 2026-10-20T06-58-31-wrap). This is the precedent for measuring the console key line against the 80-column floor, and the width source the measurement reads. `decision/name-truncation`, `decision/unwrapped-name-truncation` and `decision/colour-decision-inputs` rest on it.
