@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Input Validation: 1 row(s)) — added + decision/console-key-reader-bounded (reads of at most 64 bytes, one closed class per read: 0x03 interrupt, exactly `q` quit, else unbound; no key byte on any line), PROVISIONAL on a delegate word. This chunk's key-sequence decoder replaces that decoder (carry cry-pjtbjcco), so the row is amended here: the bound, the closed decoding and "no key byte reaches any log line or output" are the floor the new decoder must keep.

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Authentication & Authorization: 2 row(s)) — added + decision/console-terminal-and-home-check (both stdin and stdout terminals, then the home's read rule before any log, a refusal writing no line, mode left as found) and + contract/strict-modes-refusal/console (the fixed unable:/hint: pair). The console's own start checks stand ahead of every action; each action's verb then runs its own checks.

@history_item n=3 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Authentication & Authorization: 1 row(s)) — ~ contract/strict-modes-refusal/console: a home widened mid-run ends the console with exit 1, the pair and one process-exit strict-modes-failed line (no line only at start), PROVISIONAL. Actions run mid-run, so a refusal met during an action must match this shape, not the start's.

@history_item n=4 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Input Validation: 1 row(s)) — ~ decision/claude-agents-resolved-from-path now names viola console among the callers of the PATH-resolved, MAX_FRAME-bounded claude agents read. STATUS comes from that read, so a send's readback must be the verb's reply, never STATUS (carry cry-qjw64paz).

@history_item n=5 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Data Protection: 1 row(s)) — + decision/console-agents-child-outlives-quit, accepted as built and provisional. This is a precedent: if an action runs its verb as a child process (P3 premise), any residual left by that child after a quit needs its own accepted row.

@history_item n=6 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the claude agents --json child of list, revive and verify Run B) — an unwrapped NAME (the row's sessionId) is never a ViolaName or target and is printed through escape_row. A console selection that lands on an unwrapped row must offer no verb a target from it (decision/unwrapped-name-never-target, decision/agents-row-name-not-a-target); select by the board item, never by the drawn text.

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** (viola list joins the strict-modes entry points; the replay and budget.json gain a reader) — set the human-output floor: wait / last keep \n and \t, list rows escape every control as \xHH. An action that draws a verb's output (last's reply, a send's readback) on the console's surface falls under decision/human-output-escapes-controls and decision/escape-terminal-controls.

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the seventh dated gap: pause and release frames, and their validation) — set release's string-from refusal (-32602, reason release-from-driver) and CLI `viola release` forwarding VIOLA_NAME, so a release run from a driver session exits 20. A trap for the console: launched from inside a viola session it inherits VIOLA_NAME, and its release action, run as the CLI verb, would be refused. This bears on the [inferred] premise that release is a human verb.

@history_item n=9 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Authentication & Authorization: 5 row(s)) — gave the release rows their current form: a string from is refused before any other check of its params, and params are {from?}. These are the rows the console's release action rests on (decision/driver-release-refused, decision/release-from-driver-guard, contract/channel-release/*).

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's closed detail word) — the release-from-driver refusal logs a WARN channel-response line with detail release-from-driver and no param name or value, on the server's and the client's line. A console release refused this way writes exactly that line.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — sanitised_fault and is_code_word: a client takes a refusal or detail only as a code word and a fault only through the sanitised fault, a code word outside the list staying visible by design. A send's readback and every unable line the console shows must be built from those screened values only (decision/client-reads-closed-values-only, decision/client-reply-code-words-only, decision/reply-code-words-validated).

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census and what it does not drive) — product_error_writers_are_each_named fails on an unnamed error writer. It also names the exception that answer's and send's exit-2 paths write a fixed unusable line to stderr whatever --json says. Unable lines drawn on the console's surface are a new writer the census must name, and the exit-2 lines are the shape a console send or answer meets on unusable text.

@history_item n=13 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`viola link` / `viola unlink`'s instance check before any log) — a trap for later verbs: opening a role file narrows an existing home to 0700, so a check placed after client::start passes a 0770 home. Any in-process path the console uses to call a verb must keep the instance check ahead of every log or role-file open (decision/strict-refusal-leaves-no-trace, decision/strict-check-before-narrowing).

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six CLI frame verbs share `link` / `unlink`'s instance check) — client::refuse_untrusted runs check_instance first in send, wait, last, answer, pause and release, before client::start. A refusal is exit 21 with the instance pair, nothing opened and no line. This is the one path an action must take if the console calls verbs in-process: no second code path that skips it.

@history_item n=15 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (a failed server check: no frame, and what each client reports) — fixed the CLI frame verbs' server-verify refusal: no frame, exit 21, one process-exit WARN line, and in human mode the unable line with the fixed "not its recorded wrapper" hint last on stderr. This is the unable line a console action must show on that refusal.

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client's peer euid check and the order at a seam) — fixed the order on every client seam: the socket-directory check, the connect, the peer euid decision, then the server verification. A refused euid reads server-verify-failed. Console actions reaching a wrapper inherit this order through the verb (decision/client-seam-check-order, decision/client-peer-euid-check).

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused`, the sixth cause and the eleventh code) — added socket-dir-refused to exit 21's closed causes, with its human unable line and hint of cause socket-dir. It is one of the refusal lines an action must surface as the verb prints it.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tests holding the directory and peer checks, and no seam to drive them) — the ban: no env, flag or feature seam may force a refusing peer-euid decision. A foreign peer is held by injected values only. Console action tests of these refusals may not add such a seam (decision/no-seam-for-foreign-peer, decision/no-seam-forcing-peer-refusal).

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (client server verification is one Linux flow; the macOS carve-out and the Windows SQOS open retired) — every frame-writing client checks the socket directory, the peer euid and the peer pid with its start time before the first frame, with no carve-out. This is the check set every console action's frame meets (decision/verify-server-before-frame, decision/server-verified-before-any-frame).

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21 names its cause in detail) — made exit 21's detail a closed cause set (now strict-modes-failed · instance-dead · unwrapped-name · endpoint-lost · server-verify-failed · socket-dir-refused), each with its own hint. These are the refusals a console action must show verbatim as the verb's unable line.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the no-endpoint reading, dead against unwrapped) — without_endpoint reads the instance directory alone: instance-dead or unwrapped-name, opening nothing and writing no line. An action against a stopped or never-started row reads one of these causes, and the console must show it as the verb does.

@history_item n=22 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (Authentication & Authorization: 1 row(s)) — ~ contract/instance-unreachable-detail/unwrapped-name dropped the symlink case: a symlinked instance directory now reads strict-modes-failed first. This matters for which unable line an action shows on such a row.

@history_item n=23 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation as landed; the feed bound closed) — validate_paste_text runs in viola send before any frame (exit 13) and again first in the wrapper. Send's stdin / --file text is capped at MAX_FRAME (over the cap or non-UTF-8 is exit 2). The console's send text entry must hand its text through the same verb input and checks (decision/paste-validation-on-both-sides, contract/cli-send-text/input).

@history_item n=24 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — an empty typed text is refused not-delivered/empty-text (exit 13), client-side before any frame and in the wrapper. A console send of an empty or newline-only entry meets this refusal, and must show it as the unable line, never typing.

@history_item n=25 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** (an inner CR or CR LF is typed as one LF after validation, the founder's ruling) — typed-text normalisation runs after validation, and any other rewrite of sent text needs its own ruling. The console's text entry must not strip or rewrite what it hands the send verb (decision/refused-text-never-stripped, decision/reject-dont-strip-paste-text).

@history_item n=26 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (Threat Model Summary: 2 row(s)) — ~ decision/send-text-is-one-bracketed-paste and decision/bridge-send-one-bracketed-paste gained the paste-cut exception: a paste not finished within its bound is closed with no Enter and refused not-delivered paste-cut. This is one more send readback the console must show.

@history_item n=27 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (Error Handling: 1 row(s)) — + contract/channel-refusal/paste-cut, the closed not-delivered detail, CLI exit 13. It belongs to the closed set of send refusals a console readback reads.

@history_item n=28 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Threat Model Summary: 1 row(s)) — ~ contract/wrapper-channel-methods/send: a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable), never a confirmation. A send's readback on the console must keep that distinction and never draw it as delivered.

@history_item n=29 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (a new attack-surface vector: the human's terminal on `viola run`'s stdin) — any byte typed on viola run's stdin is the human's, whoever typed it, and takes the wheel (contract/send-refusal/human-typing). An agent typing keys was a widening ratified live. Consequences here: a console send is refused human-typing while the human holds the wheel, and the console's keystroke test driver must type only into its own pty.

@history_item n=30 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (the sixth dated gap: answer and hook.dialog frames, and their validation) — set answer's input: dialog_id a u64, a closed per-kind response, then validate_paste_text on free text, client-side before any frame (exit 13) and authoritatively in the wrapper. A console answer action must pass through these checks (decision/answer-behavior-closed-enum, decision/dialog-free-text-control-character-set, contract/channel-frames/answer-params).

@history_item n=31 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization: 1 row(s)) — ~ decision/unverified-cli-gate: no non-null answer without a verify stamp on which every relied row counts. A console answer, or a switch, on an unverified CLI meets exit 12 unverified-cli and must show it as the verb's refusal.

@history_item n=32 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Threat Model Summary: 3 row(s)) — ~ contract/wrapper-channel-methods/answer: a question first raised by PermissionRequest is answered allow with updatedInput carrying the tool input plus the driver's answers. This is the answer method form the console's answer action reaches.

@history_item n=33 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (the fifth dated gap: CLI wait / last after the liveness-only pre-check) — set the wait / last params (after and timeout_ms a u64, unknown fields ignored, else -32602) and their <name> through ViolaName::try_new. The console's last action rests on contract/channel-frames/wait-last-params. The gap itself was later retired.

@history_item n=34 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Authentication & Authorization: 2 row(s)) — + decision/switch-ends-only-its-own-child and + decision/switch-refusal-order (human typing, running turn, pending dialog, unverified CLI, no session; no refusal appends a line or ends the child). The console's switch leg (carry cry-mf2w3ioc) reaches it through the CLI verb and must show those refusals in that order.

@history_item n=35 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Threat Model Summary: 2 row(s)) — + contract/wrapper-channel-methods/switch and + decision/switch-method-behind-ipc-controls. This is the precedent that a new channel method needs its own IPC-controls entry: if the console's actions needed any channel method beyond the CLI verbs', it would be that widening.

@history_item n=36 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Input Validation: 2 row(s)) — + contract/channel-frames/switch-params and ~ decision/launch-flags-checked (switch's --model / --effort are model and effort words, else exit 2 before anything is read). Any model or effort the console offers for a switch must pass those checks through the verb.

@history_item n=37 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Error Handling: 1 row(s)) — ~ decision/cli-frame-verbs-strict-modes-first names switch among the verbs that check the instance first (exit 21, no line).

@history_item n=38 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Security Anti-Patterns (NEVER do these): 1 row(s)) — ~ contract/cli-exit-codes/12: answer or switch refused unverified-cli. A console switch or answer can meet this exit.

@history_item n=39 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (Authentication & Authorization: 8 row(s)) — set switch-ends-only-its-own-child, switch-refusal-order, launch-pair-authenticates-nothing, window-origin-authenticates-nothing and up-checks-before-log-and-spawn standing on the operator's own word. These rows stand as ruled when the console builds on them.

@history_item n=40 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Authentication & Authorization: 1 row(s)) — + decision/focus-raises-only-ancestry-windows: focus raises only a recorded window inside the wrapper's ancestry. The console's focus action reaches this boundary through the CLI verb and may raise nothing else.

@history_item n=41 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (the operator's answer to F-A ratified) — fork F-A was ruled arm 1, focus as "a command for the human only". Exit 21 and the strict-modes-first row stand as written. The console is the human's surface, so reaching focus from it fits that ruling, and any driver-facing route would not.

@history_item n=42 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Data Protection: 1 row(s)) — + decision/focus-ancestor-walk-in-memory (at most 8 ancestor pids, never recorded). An action invoking focus must not log or draw the walked pids or a window address (decision/window-record-kept-to-snapshot-and-event-log).

@history_item n=43 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Threat Model Summary: 1 row(s)) — corrected decision/state-tree-strict-modes-before-use: focus runs the instance check (refuse_untrusted), not check_home_tree. It exits 21 per contract/cli-exit-codes/21, which is the refusal a console focus shows.

@history_item n=44 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Input Validation: 5 row(s)) — + decision/open-closed-argument-allowlist and + decision/open-terminal-direct-spawn (closed argv, no shell) and + decision/run-window-flags-checked. The console's "open a child" action must hand viola open only words inside that allowlist; the verb refuses any other word with exit 2.

@history_item n=45 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Threat Model Summary: 1 row(s)) — ~ decision/state-tree-strict-modes-before-use names open among the verbs holding the home tree before any log (exit 1). This is the refusal shape a console open action shows.

@history_item n=46 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Data Protection: 2 row(s)) — + decision/window-record-kept-to-snapshot-and-event-log and + decision/open-ancestor-walk-in-memory. No window address, pid or workspace enters a process-log line or other output, so console action results must not draw them.

@history_item n=47 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Input Validation: 3 row(s)) — widened open-terminal-direct-spawn to up and gave revive a hidden --origin (cli|agent) that conflicts with --list (contract/revive-arguments/id). A console revive action run through the verb takes only these checked arguments.

@history_item n=48 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (Input Validation: 8 row(s) (1 of 2)) — set open-closed-argument-allowlist, open-terminal-direct-spawn, launch-flags-refused-after-dashes, run-lineage-flags-checked and the board re-parse rows standing on the operator's word. These are the ruled bounds the console's open, revive and selection build on.

@history_item n=49 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start) — revive's closed exit-1 codes (now ten after the Linux-only cut: strict-modes-failed · already-live · no-session · session-live · cwd-missing · stale-heartbeat · pinned-hash-mismatch · squatted-name · socket-dir-refused · internal-error). A console revive action's refusal is one of these codes and shows as the verb's line.

@history_item n=50 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — ratified the CLI home resolution (--home, else the VIOLA_NAME/VIOLA_DIR instance's home, else ~/.viola, canonicalised). A console action that runs a CLI verb must make it resolve the console's own home, so the action acts on the board the console shows (decision/cli-home-resolution-order, contract/cli-state-root/*).

@history_item n=51 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (the terminal colour facts NO_COLOR / TERM / COLORTERM join the admitted env set) — the colour facts are read only by viola list's human mode. Any other environment read, or a colour fact changing a control, needs its own Decisions Log entry. If the console's action surface (selection highlight, unable lines) reads new env or colour facts, that is a widening.

@history_item n=52 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census, the compare read bounded by its own length, the own-executable reads named) — product_readers_are_each_bounded_or_named requires every product reader to be bounded in its statement or named. The new key-sequence decoder's reads, and any read of a verb child's output, fall under that census (decision/bounded-readers-census).
