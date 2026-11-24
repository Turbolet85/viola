@history_item n=1 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (§1, 3 rows) — ~ contract/start-refusal/home-strict-modes, ~ contract/start-refusal/socket-dir and ~ contract/log-identity/process now name a `viola adopt` start. Adopt's start runs run's shared start, refuses before any log or child, and logs as `process:"run"` with `instance:<ViolaName>`. Any record this chunk adds for an early child exit (event or log line) carries that identity, and the start's refusal order stays as written.

@history_item n=2 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (§8, 4 rows, 1 of 2) — ~ decision/strict-modes-refusal-before-any-write, ~ contract/json-fault/internal-error, ~ decision/socket-dir-refusal-before-any-frame and ~ decision/claude-agents-read-fails-open gained adopt's start and preflight. Adopt stays silent at the internal-error catch site like run and revive's start. A new early-exit outcome has to fit beside these lists and must not break the "silent there" rule for a start.

@history_item n=3 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (§8, 2 rows, 2 of 2) — ~ contract/cli-fixed-message/form and ~ contract/start-json-refusal/document: adopt's own strict-modes-failed and session-live refusals print the fixed `unable:` / `hint:` pair, and under `--json` they give one `{"v":1,"error":"<code>","detail":null}` document with no hint. A new closed word for "the resumed child ended at once" extends these two closed lists, in both the human form and the `--json` form.

@history_item n=4 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (§1 and §2, 1 row each) — ~ contract/tui-passthrough/entry: a passthrough is entered by a viola adopt whose preflight passed. ~ contract/cli-refusal/unable-line: adopt's two own refusals are one-line `unable:` forms. A child that exits right after the start may already have held the terminal, so the zero-viola-bytes boundary decides whether any line can be printed at all. If the chunk prints one, the unable-line list gains a third adopt form.

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start) — this set contract/viola-revive-json/start-refusal and contract/tui-passthrough/pump-child-error / contract/viola-revive-json/pump-child-error: a `pump_child` error prints no document, because the child has held the terminal and viola writes nothing past it. This is the main trap for the early-exit word. Anything said after the child started breaks that boundary unless it is moved before the hand-over or recorded off-terminal only.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a lost wrapper and a stale holder each read their own code) — added contract/start-holder-refusal/already-live and contract/start-holder-refusal/stale-heartbeat: stale heartbeat is exit 1's cause alone. Whether adopt releases the instance directory and name after an early exit decides which of these codes the next start or re-adopt reads.

@history_item n=7 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a name with no endpoint reads the instance directory) — added contract/instance-unreachable-detail/instance-dead (the directory stands) and contract/instance-unreachable-detail/unwrapped-name (no directory). If the instance directory is kept after an early exit, frame verbs read `instance-dead`. If it is released, they read `unwrapped-name`. The chunk's release choice decides which one users see.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table and one hint per code) — `no_two_codes_share_a_hint` and `no_hint_names_release` in `tests/cross_exit_causes.rs` (decision/one-hint-per-code, decision/no-hint-names-release). A new early-exit code needs a hint of its own that does not name release, plus a row in the exit-cause table.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `unwrapped-name` and `endpoint-lost` hints and their pins) — precedent for a new hint: it ends with its own `hint:` as the last stderr line, in plain ASCII, naming only the parsed ViolaName (decision/printed-names-are-parsed-arguments), and it is pinned by `send_hint_is_the_design_string` and the exit-cause table.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — this set contract/json-fault/internal-error: run's, revive's (and later adopt's) start stays silent there. A `claude` not found at a start reads as internal-error. The early-exit outcome must be told apart from this fault class (decision/faults-carry-no-hint, contract/cli-exit-codes/20).

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the census of error writers and the body check) — `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` (decision/error-bodies-have-writer-and-witness, decision/thiserror-messages-interpolate-nothing). A new early-exit error body needs a named writer and a witness, and its message interpolates nothing.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness) — contract/cli-json-failure/document and decision/json-failure-documents-are-clean: each `--json` failure is one document with a literal exit, empty stderr, no ESC byte, and a body holding no path or upstream text. The early-exit document must not echo the resume folder, the child's exit status text or the session's path.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's refusal screened to a code word) — contract/code-word/format (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) and decision/unlisted-code-words-stay-visible (a word outside the closed list takes unknown's exit 14). The new closed word must be a valid code word and be added to the closed refusal list, or it falls to `unknown`.

@history_item n=14 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive takes `--json`: its preflight's exit-1 codes include `cwd-missing`, so revive already refuses on its resume folder. This is the nearest precedent for adopt's resume folder. The scope notes that revive and up take the folder from the snapshot's recorded cwd and that adopt has no such record. Copy that word's form and document shape rather than inventing a new one.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (revive's start refusals without batch-script-child) — revive's exit-1 `--json` codes now read ten (contract/revive-json-refusal/document, .../already-live, .../squatted-name, .../stale-heartbeat). A closed word left with its emitter, and that narrowing was ruled on explicitly. Adding a word to the shared start's list touches revive's closed set too, so that needs a ruling (see n=16).

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start: exit 1, revive's eleventh code) — precedent for growing the start's closed refusal set. The new word was the founder's own, given after the widening was shown to him. It was a fixed-message `unable:` / `hint:` pair plus one `--json` document holding no path, pid or uid. A new early-exit word is the same kind of widening and needs the same word from the operator.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities) — set the run / revive-start / verify start-refusal entity: exit 1, stdout empty, the home-state pair, no child, and no line written on a refused check. Adopt's start later inherited this. These refusals happen before the child; the early exit happens after the spawn, a different moment the chunk has to place.

@history_item n=18 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed revive enters the TUI passthrough and writes no line of its own while the child holds the terminal (contract/tui-passthrough/own-bytes, `tests/chaos_revive.rs`). Its exit-1 refusals take the fixed-message form of design-system cli pattern 2. Adopt resumes a session the same way, so this is the boundary any early-exit output must respect.

@history_item n=19 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair: one `unable:` line and one `hint:` line last, exit 1, pinned by literal. This is the other precedent for a fixed one-line pair said after work has begun (contract/cli-refusal/unable-line lists it). Its wording was a planning form whose final wording was owed to the founder, as a new early-exit pair's would be.

@history_item n=20 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (§7 and §1) — ~ decision/wrapped-tui-reads-as-unwrapped except: in an untrusted folder the start writes the CLI's trust record for the folder before the spawn. ~ decision/run-diagnostics-off-terminal: the trust step reports one `trust-record` line in `diagnostics/`, never as a refusal, exit code or hint. Changing adopt's spawn folder changes which folder that trust record is written for. Any early-exit record kept in diagnostics follows the same off-terminal rule.

@history_item n=21 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (§8, time bounds) — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver gained the start's Claude config lock wait, a bound that falls on viola's own start. If the chunk adds a wait to detect an "at once" exit, that wait falls on the start too and needs naming the same way.

@history_item n=22 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (§8 time-bound rows) — the two bound lists were reworded to an «Eight» lead (PROVISIONAL, not the operator's word «These»), and per F1 option 1 later bounds get rows of their own (decision/open-live-wait-bound, decision/window-wait-bound, decision/hyprctl-read-deadline). Any early-exit detection window must not be appended to decision/time-bounds-never-on-human or decision/time-bounds-fall-on-the-driver; it lands as a row of its own.

@history_item n=23 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (the operator's answers to F-A and F1 ratified) — this ratifies F1's rule: a later bound gets a row only where it is new on a new bearer. The «Eight» lead of the two time-bound rows stays provisional. The chunk applies this test to any wait it adds to adopt's start.

@history_item n=24 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — + decision/start-verify-stop-grace-off-the-human: a new bound on the start landed as its own row beside the full time-bound lists. This is the precedent for an early-exit wait on adopt's start.

@history_item n=25 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (§8) — + decision/respawn-reap-bound: a switch's reap wait on a replaced child is bounded at 10 s, in a row of its own (delegate word, PROVISIONAL). This is the nearest precedent for a bounded wait on a resumed child's end.

@history_item n=26 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (§1 and §2) — ~ contract/tui-passthrough/human-keystrokes, ~ contract/tui-passthrough/viola-terminal-bytes and ~ decision/human-keystrokes-never-blocked except: the boundary holds "once the wrapped child holds the terminal", and before that only the automatic verify window may write. This is the one standing pre-child exception. An early-exit line after the child held the terminal is not covered by it.

@history_item n=27 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (§1, §3, §4) — ~ contract/tui-passthrough/own-bytes, ~ decision/zero-viola-bytes-on-the-terminal, ~ decision/wheel-ignores-terminal-events and ~ decision/portable-pty-outer-pty-driver each gained the "before the child starts, a keyed start's automatic-verify lines" exception, so viola's own bytes are allowed only before the child. Any early-exit output after the spawn needs a new named exception in these four rows or must stay off the terminal.

@history_item n=28 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — ~ contract/tui-passthrough/human-keystrokes and ~ decision/human-keystrokes-never-blocked: keys are never blocked or dropped once the child holds the terminal. The current wording stands; an early child exit must leave no key dropped by viola.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone) — the zero-viola-bytes case compares the outer-PTY stream byte for byte with an unwrapped fake-agent run, on `ubuntu-latest` only. This is the harness an early-exit outer-PTY case for adopt would use, with a fake agent and no real claude.

@history_item n=30 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the §8 verification lines pinned by literal) — decision/fixed-cli-strings-pinned, decision/refusal-layout-literal-pins: every fixed CLI string, hints and result lines included, is pinned by a literal over `std::process::Command`, together with its typed exit code and a hint-free `--json`. A new early-exit word, hint and line must each get a literal pin.

@history_item n=31 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's live readings are supplemental and never gating (decision/live-readings-never-gate). The R5a re-fire through `2026-11-24-viola-adopt`'s gated door is likewise a live reading outside this chunk's plan, which holds no live step.

@history_item n=32 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's and the board's one `claude agents --json` read (5 s kill) fails open, which decision/claude-agents-read-fails-open later extended to adopt's preflight. The board's unwrapped rows (contract/viola-list-board/unwrapped-separator) show a session no wrapped row joins. Whether adopt releases its name after an early exit decides whether the session shows up as unwrapped again on the board.
