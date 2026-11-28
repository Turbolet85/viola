@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** — registered `name/command/console`, gave `decision/cli-human-text-json-exit-codes` and `decision/cli-output-conventions` the except «viola console takes no --json and draws its own screen», gave `decision/new-capabilities-as-verbs` the same except, put the console's two refusals into `contract/cli-exit-codes/1` and added the `console` role to `decision/codes-only-process-logs`. This is the surface the actions extend: an action's `unable` line drawn on the console's own screen, with no --json, has to fit these excepts. Several marks are a delegate's word (PROVISIONAL).

@history_item n=2 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** — added viola console to the ungated agents readers (`decision/agents-join-read-ungated`). Gave `decision/strict-modes-check-before-log` and `decision/refused-check-writes-no-line` a named except: a home refused mid-run ends the console with exit 1, the home pair and one `process-exit` strict-modes-failed line. This is the view this chunk builds on: STATUS comes from the agents thread, and the console's mid-run refusal shape is an exception the action refusals must not widen without a new row.

@history_item n=3 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census. It made the keyboard-driven terminal fleet console tiled on Hyprland viola's one view beyond `viola list`, owed by the Epoch 6 entries «Console scaffold», «Fleet console view» and «Console actions», with no web or graphical frontend (`decision/terminal-fleet-console`, the graphical absences). It names this chunk as the owner of the console's actions and sets its bounds.

@history_item n=4 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — made 0.1.0 a pure CLI and removed the GUI, the listening TCP port and the `ui` verb. This explains the absences the console must respect: no listening network port and no graphical surface.

@history_item n=5 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — added `decision/switch-in-place`, `decision/switch-cli-only` (no MCP tool switches) and `decision/switch-writes-no-user-settings`, the `switch` channel method and `contract/cli-verbs/switch` (exits 0/10/12/13/14/20/21; at least one flag, else exit 2), and a switch standing as a `turn-running` cause in `decision/send-never-queued`. The console's switch leg (carry cry-mf2w3ioc) must go through this CLI verb and its refusals.

@history_item n=6 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — ratified the delegate marks of Epoch 5. `decision/switch-cli-only`, `decision/switch-in-place` and `decision/up-parents-first` are now standing, and the PROVISIONAL rows of the switch, focus, open, up, adopt and lineage chunks are ratified. The verbs the console will call stand on the operator's word, not on provisional marks.

@history_item n=7 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added `contract/cli-verbs/focus` (`viola focus <name> [--json]`), `decision/focus-has-no-driver-form` (CLI only, no MCP tool; any driver form is a widening of its own), focus's exit-1 refusals and its exit-21 strict-modes refusal, and the process-exit words window-not-found, unsupported, unreachable, oversize, malformed and refused. Focus is one of the v2-15 verbs, and its refusals are the `unable` lines the console must show. Its sibling entry ratified F-A and F-C.

@history_item n=8 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — added `viola open` (`contract/cli-verbs/open`, exit codes 0/1/2, terminal-not-found / not-live) and extended `decision/no-start-over-live-instance` to open. It also added the `window` event and snapshot field and the list items' `window?` and `origin?`. «Open a child» in v2-15 goes through this verb, and its refusals reach the console as exit-1 `unable` pairs.

@history_item n=9 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — added `viola up` and its exit codes 0/1/21. It extended `contract/cli-exit-codes/1` to «an up whose lines are not all successes», added the hidden `revive --origin` and narrowed `decision/lineage-label-only` so that up may read the parent for ordering. This precedent shapes exit 1 and revive's argument surface, both of which the console's revive action reaches.

@history_item n=10 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — served `pause` and `release` and landed `release-from-driver`: a string `from` is refused, which deters a driver but does not enforce anything. It made the closed non-editing byte list and «a human-filed prompt moves the wheel before its line is appended». This supports the scope's premise that `viola release` is a human verb the console may reach: run from the console, release carries no `from` unless the console itself runs inside a wrapped session.

@history_item n=11 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — only a `release` that returns a human-held wheel clears the running-turn state, and the recovery is `pause` then `release`. This is the wheel semantics behind the console's pause and release actions and their readback.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `contract/cli-verbs/release` now refuses `--budget` with exit 2, exit 11 is retired and never reused (`decision/retired-exit-codes-never-reused`), and `budget-paused` left the send refusal order. A console action must not offer a budget lift, and no new console exit code may reuse 11.

@history_item n=13 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — built `viola link` / `viola unlink` (exits 0/1/20/21; their instance check before any log; `{since, already_linked}`). `release`, `pause`, `link` and `unlink` stay human, CLI-only controls (`decision/human-controls-cli-only`). This is a precedent for the human verbs the console may offer, and it sets the reply shapes its readback would show.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — closed exit 21's causes (`UnreachableCause`, carried in `detail`). It put the unwrapped-name and endpoint-lost hints in `src/human/hints.rs` (each the last line on stderr, plain ASCII), added the `--json` internal-error document at the catch site, and made `tests/cross_exit_causes.rs` hold that no two codes share a hint. The `unable` lines the console shows come from these writers and must keep their wording and closed codes.

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — added `socket-dir-refused` as exit 21's sixth cause, with its `unable:` / `hint:` pair carrying no path, pid or uid and no SGR (`contract/exit-21-human-hints/socket-dir`). It added the peer-euid check, and moved `viola list` to one read rule on the home. These are refusal lines the console may draw, and they must stay sanitised.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — every CLI frame verb runs `client::refuse_untrusted` first: a refused strict-modes check exits 21 and writes no line. The order is strict-modes check, then liveness pre-check, then server check. Each verb the console calls keeps this order and its no-line refusal.

@history_item n=17 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — every frame-writing client compares the peer pid and its start time before its first frame (`decision/verify-server-before-first-frame`), and a mismatch exits 21 `server-verify-failed`. The console's verb calls carry this guard. A console that opened its own channel client would have to repeat it.

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — established `decision/sanitised-error-surfaces` (codes and fixed messages only; `sanitised_fault`; the census `tests/contract_error_bodies.rs`) and the code-word reader rule (`contract/peer-reply/*`). Any refusal text the console draws must pass through these closed values and land in the census.

@history_item n=19 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — split `src/human.rs` into `src/human/` by phase: `mirror.rs` (send's readback mirror, `write_unable`, `write_wrapper_fault`), `hints.rs`, `verbs.rs`, `board.rs` and `wait.rs`. The human module is the one writer of human output (`decision/single-human-writer`). The console's readback and `unable` lines should reuse these writers, not add a second human writer.

@history_item n=20 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — landed confirmed send: the send records, the refusal order starting with `control-character`, and `src/human.rs`'s readback mirror for `send`. A send's readback in the console is this verb's own reply and mirror, never STATUS (carry cry-qjw64paz).

@history_item n=21 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — added the mirror's fourth line, `[  ] unconfirmable`, and made send consume the local-command list. A console send's readback must show unconfirmable too, as an ok outcome, not a refusal (`contract/ok-payload/unconfirmable`).

@history_item n=22 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8.5 s, and send blocks up to 18.5 s. This supports the scope's premise that a send's readback can take far longer than the view's one-second re-read: the console must not block its view or keys while a send waits.

@history_item n=23 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` waits its own 40 s window (`decision/compact-confirm-window`), so a compact send blocks about 48.5 s plus the paste. This is the longest readback the console's send action must outlast.

@history_item n=24 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — added `decision/paste-bound` (5 s), `decision/paste-cut-mark`, the `paste-cut` detail (exit 13) with its hint line, and the new send bound in `decision/send-time-bound`. It reworded `decision/human-always-wins-wheel` to «past the in-flight chunk and closing marker». These set the send readback's bound and a refusal line the console must show.

@history_item n=25 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — `decision/send-time-bound` now reads «the MCP call deadline bounds the driver's wait, never the send itself», and `call-deadline` is MCP's alone. The console reaches send through the CLI, so its wait is bounded by send's own total, not by a call deadline.

@history_item n=26 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added the `empty-text` refusal (exit 13; client-side before any frame) and its rung in the refusal order, and made the typed text drop trailing CR/LF. A console send whose text entry ends up empty is refused this way.

@history_item n=27 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — the typed-text normalisation (an inner CR or CR LF becomes one LF; trailing CR and LF are dropped; `decision/typed-text-normalisation`). This decides what text a console text-entry mode may hand to `viola send`.

@history_item n=28 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — send types no trailing newline (`decision/send-never-types-trailing-newline`). A console text entry that adds Enter as a newline still produces the same typed text.

@history_item n=29 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `decision/local-commands-by-ledger` by name, `decision/local-command-echo-confirmation`, and the stdout-echo unconfirmable cases in `contract/send-results/unconfirmable`. A slash text sent from the console is classified and confirmed exactly as at a shell.

@history_item n=30 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — served `answer` and `hook.dialog`: `viola answer <target> <dialog_id>` takes a JSON `response` from stdin or `--file`, permission answers have no suggestion slot, and the answer refusal order is control-character, human-typing, unverified-cli, unknown-dialog. «Answer» in v2-15 must build this response from the console's keys.

@history_item n=31 marker=2026-11-20-permissionrequest-question-answered
**2026-11-20-permissionrequest-question-answered** — added `decision/permission-request-question-answered` and narrowed `contract/answer-refusals/unknown-dialog` to late answers. These set which dialogs a console answer action can reach.

@history_item n=32 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — added the configurable dialog hold (60–3600 s), the `dialog-expired` record that wakes `wait`, and the null reply at the hold's end. A console answer given after expiry reads `unknown-dialog`.

@history_item n=33 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `contract/dialog-hold-end/null-reply`: the expiry ends only the hook's wait, and the CLI's dialog stays drawn. The dialog hold is now standing, and the wheel-start except covers trust keys. This bears on what the console shows for an expired dialog.

@history_item n=34 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `wait` / `last` as built, and `role_of` filing the CLI verbs under `cli` with one internal-error printer at the catch site. A `last` action in the console returns this verb's reply. Calling the verbs in-process or as child processes (scope premise) decides how this catch site behaves.

@history_item n=35 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — added the `revive` verb, its preflight refusals and `decision/revive-in-place`. With the 2026-10-17-cli-machine-contract and 2026-10-31-exit-cause-code-catalogue entries, revive's refusals are now the closed `--json` codes in `contract/viola-revive-json-error-codes/*`. «Revive» in v2-15 must show these as `unable` lines.

@history_item n=36 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave revive `--json` on both arms and made the CLI verbs take the `VIOLA_DIR` home step (`decision/session-home-from-viola-dir`, ratified at 2026-10-18-plugin-precedence-live-readings). Every verb the console runs must resolve the console's own home, `--home` included.

@history_item n=37 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the agents join and the read-only unwrapped rows (`contract/list-item/wrapped`: never a valid target), and revive's `session-live` refusal. A console selection on an unwrapped row must offer no frame verb.

@history_item n=38 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` is a caller-side read that contacts no wrapper (`decision/list-is-caller-side-read`). This is the board the console's selection names its row by, by the board's item and never by the drawn text.

@history_item n=39 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified the board's whole-board exit-21 refusal with no log opened, and the link hold was moved to Epoch 6. This sets the refusal shape behind the board the console reads.

@history_item n=40 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — added the list items' `parent?` / `role?`, `links?` and `decision/lineage-label-only` (lineage is a label no control path reads). A console action must not branch on lineage, beyond the `up` exception.

@history_item n=41 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — added `decision/launch-pair-handed-not-verified`, the list items' `model?` / `effort?`, and revive's `--model` / `--effort` with exit 2 for a malformed value. A console switch or revive action must not claim the session runs on the handed pair.

@history_item n=42 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — added `viola adopt`. It extended `decision/run-refuses-held-name` and `decision/no-start-over-live-instance` to adopt and made adopt's start silent at the catch site (`decision/silent-roles-at-internal-error-catch`). This is the starter-verb family the console's open and revive actions belong to.

@history_item n=43 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — added `decision/hyprland-seam-crate` (the one compositor seam, sync and depending on viola-core alone), `decision/hyprctl-direct-spawn` and the closed `contract/hyprland-seam-errors/*`. A console focus action reaches the window only through `viola focus`, which reaches it through this seam.

@history_item n=44 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** — the seam's dispatches are now `hl.dsp.focus` and `hl.dsp.window.move` only. Any window move the console triggers stays inside this closed set.

@history_item n=45 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP tools are send, wait, last, answer and list, with no release, pause, link, focus or switch. `role_of` excludes `mcp` from `cli`. The console is the human's surface, so offering it the human-only verbs does not widen the driver surface (`decision/human-controls-cli-only`, `decision/release-cli-only`).

@history_item n=46 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `decision/colour-env-decides-only-sgr`: NO_COLOR, TERM and COLORTERM are read, never set, and decide only SGR. Ratified as a widening; any further environment read needs its own ratification. The console's action and readback rendering must use this colour layer.

@history_item n=47 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without its `color` feature (`decision/clap-without-color-feature`), and the human module is the one human writer. This is the root of `decision/single-human-writer` and `decision/no-print-macros`, which bind any new console output path.

@history_item n=48 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — gave `decision/vt100-readiness-gate` the except that the test-only live rig uses vt100 as its screen model. This is a precedent for the console's keystroke test driver over its own PTY: a test-only screen use needs a named except, and the scope premise says the driver reads state, never the screen.

@history_item n=49 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `decision/bounded-readers`, held by the census `tests/contract_bounded_readers.rs`. The console's new key-sequence decoder reads the terminal's stdin, so each read must be bounded in its statement or named in the census (carry cry-pjtbjcco).

@history_item n=50 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — added seven terminal reply shapes to the closed non-editing list, by exact grammar only («a reply no terminal was measured to write is not added»), and landed the `input-not-ready` hint wording. The key-sequence decoder meets the same terminal replies. That list's exact-grammar discipline is the precedent, and the hint wording is a line the console may draw.

@history_item n=51 marker=2026-11-16-survey-key-keeps-the-wheel
**2026-11-16-survey-key-keeps-the-wheel** — `decision/closed-non-editing-byte-list`: a survey key takes the wheel, by the founder's ruling («Выкинуть фичу»). The wheel's byte list is not to be special-cased.

@history_item n=52 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `contract/human-hint-lines/input-not-ready` and `contract/send-results/input-not-ready` now name only a standing modal. This is the `unable`/hint wording a console send may show.

@history_item n=53 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — `decision/human-prompt-confirms-wheel-take` gained the fired-wake except. Part of the wheel semantics a console pause, release or send readback reports.

@history_item n=54 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — `decision/viola-home-resolution`: run passes the home to its automatic verify child only through `--home`. This is the precedent for a parent process handing its home to a child it spawns, relevant if the console runs verbs as child processes (scope premise).

@history_item n=55 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Linux only and CI on `ubuntu-latest` (`decision/ci-against-fake-agent`, `decision/no-shell-out`, `decision/libc-only-os-binding`). It retired `batch-script-child`. Any child-process route for the console's verbs spawns directly, never through a shell.

@history_item n=56 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `decision/no-shell-out` lost its one except, so viola runs no shell at all. The console must not run a verb through `sh`.

@history_item n=57 marker=2026-10-10-session-links
**2026-10-10-session-links** — `decision/link-recording`: a send or answer issued from inside a wrapped session records a link from `from`. A console that itself runs inside a wrapped session would carry `VIOLA_NAME`, so its sends and answers would record links and its release would be refused `release-from-driver`.

@history_item n=58 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `decision/submitted-form-prompt-match` (the trailing-whitespace tolerance, later ratified) and `decision/send-flight-lock`. These are part of what the send readback reports.
