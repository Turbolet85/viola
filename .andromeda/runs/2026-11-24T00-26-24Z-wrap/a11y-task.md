@task_prose part=session
Added the CLI verb viola open: one terminal window (foot) per session beside its parent, a closed flag allowlist, only claude after --, no shell, then a non-interactive wait of up to 30 s for the new session to go live.
viola open prints a human line `<name>  live  <placement>` or one --json ok document, and refuses with exit 1 codes strict-modes-failed, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live, internal-error, each a --json error document; two new codes terminal-not-found and not-live carry one fixed unable/hint pair each.
A word after -- other than claude alone, or a self parent, is exit 2 with a fixed usage line before any home read, log or spawn.
viola run gained two hidden flags --origin and --place; under --origin the wrapper's start runs a window step (bounded 5 s wait for its window, one Hyprland move) after the trust step and before the slots, and records a window line in the event log and a window field in the snapshot.
viola list --json and the MCP list item gain optional window and origin; the help's setup: group now lists open, run, revive, verify, plugin install; the diag-line schema gains the subject terminal and the exit details terminal-not-found and not-live.
After implement only the operator's live-reading rig script changed; R2, the live Hyprland reading, fired once and read red on the rig, and nothing is gated by it.
@task_file n=1 path=src/cmd/open.rs
what: new; the viola open verb: its flag allowlist, its usage refusal, its exit-1 refusals and --json documents, its human live line, its 30 s live wait bound on the driver
@task_file n=2 path=src/run/window.rs
what: new; the wrapper's window step at run start: a 5 s bounded wait for its terminal's window and one move, its placement word
@task_file n=3 path=src/bin/viola-fake-agent/foot.rs
what: new; a test-only stand-in for the foot terminal that records its argv and runs its child
@task_file n=4 path=tests/cli_open.rs
what: new; the integration suite for viola open: its argv, placement words, ok document, refusals, every written line schema-checked
@task_file n=5 path=scripts/r2-open.sh
what: new, then edited after implement; the operator's live-reading rig for the real foot window on Hyprland, gated by nothing but a syntax check
@task_file n=6 path=Cargo.toml
what: edited; the product crate's dependency list (the Hyprland seam crate now a product dependency)
@task_file n=7 path=crates/viola-core/src/lib.rs
what: edited; the shared event kinds (new window kind) and the closed placement and origin word lists
@task_file n=8 path=crates/viola-state/Cargo.toml
what: edited; the state crate's dependency list (the Hyprland seam's address validator)
@task_file n=9 path=crates/viola-state/src/snapshot.rs
what: edited; the instance snapshot document, gaining an optional window record
@task_file n=10 path=crates/viola-state/src/replay.rs
what: edited; the snapshot replay from the event log, recovering the window record
@task_file n=11 path=crates/viola-state/src/board.rs
what: edited; the board item behind viola list --json and the MCP list, gaining optional window and origin
@task_file n=12 path=crates/viola-state/src/liveness.rs
what: edited; process liveness reads, gaining an ancestor-chain read
@task_file n=13 path=src/cmd/mod.rs
what: edited; the CLI verb table and the --help verb groups (setup: gains open)
@task_file n=14 path=src/cmd/run.rs
what: edited; viola run's start order and its hidden --origin and --place flags, the window line after the launch line
@task_file n=15 path=src/run/mod.rs
what: edited; the wrapper's run module wiring (the window step module)
@task_file n=16 path=src/human/hints.rs
what: edited; the fixed unable/hint pairs per refusal code (terminal-not-found, not-live)
@task_file n=17 path=src/bin/viola-fake-agent.rs
what: edited; the test-only fake agent dispatch, now answering as the foot stand-in
@task_file n=18 path=src/bin/viola-fake-agent/hyprctl.rs
what: edited; the test-only fake hyprctl, gaining a terminal mode listing one window
@task_file n=19 path=crates/viola-e2e/tests/hypr_tap.rs
what: edited; the Hyprland seam's recorded tap tests, gaining refused-move cases
@task_file n=20 path=schemas/diag-line.v1.json
what: edited; the run diagnostics line schema: the subject list and the process-exit detail list
@task_file n=21 path=tests/cli_output_plain.rs
what: edited; the plain-output discipline check of the --help verb groups
@task_file n=22 path=tests/cross_exit_causes.rs
what: edited; the exit-code cause table across verbs (two exit-1 causes added)
@task_file n=23 path=tests/contract_error_bodies.rs
what: edited; the error-body census: every --json error document with its writer and its witness, and the fixed messages that are no error bodies
@task_file n=24 path=tests/contract_diag_schema.rs
what: edited; the run diagnostics schema check, taking the open's exit details
@task_item n=1 part=touches
contract/cli-help/group-labels and contract/cli-help/served-verbs-only: the setup: group gains open (open, run, revive, verify, plugin install, four to five verbs); a row listing setup:'s members is stale.
@task_item n=2 part=touches
contract/cli-help/usage-line and contract/cli-help/handoff-row: --help gains a served verb open; check whether the usage or handoff rows name the verb set.
@task_item n=3 part=touches
decision/time-bounds-fall-on-the-driver, decision/time-bounds-never-on-human and decision/no-unwarned-timeouts: three new compiled time bounds, open's 30 s live wait (OPEN_LIVE_BOUND, ending in not-live exit 1), the wrapper's 5 s window wait (WINDOW_WAIT, ending unplaced or window-unseen and the start goes on) and each hyprctl call's 5 s deadline (HYPRCTL_DEADLINE); each falls on the driver, none on the human.
@task_item n=4 part=touches
absence/user-time-limit: the new 30 s and 5 s bounds must stay off the human; the absence holds only if each is the driver's.
@task_item n=5 part=touches
decision/one-hint-per-code, decision/refusal-ends-with-hint and decision/reason-code-beside-unable: two new exit-1 codes terminal-not-found and not-live, each one fixed unable/hint pair.
@task_item n=6 part=touches
contract/cli-refusal/unable-line, contract/cli-refusal/hint-line, contract/cli-refusal/exit-code and contract/cli-refusal/stderr-layout: viola open joins the refusal layout with its exit-1 codes.
@task_item n=7 part=touches
contract/cli-json-failure/document, contract/cli-refusal/json, contract/start-json-refusal/document and decision/json-one-document-no-hint: viola open's every exit-1 refusal is the {"v":1,"error":code,"detail":null} document under --json, and its success one ok document {"v":1,"ok":{name,parent,window,placement}}.
@task_item n=8 part=touches
contract/start-holder-refusal/already-live and contract/start-holder-refusal/stale-heartbeat: viola open now also refuses a live or stale-heartbeat name before any spawn; a row that names only run and revive start as the verbs is stale.
@task_item n=9 part=touches
contract/start-refusal/home-strict-modes, contract/frame-verb-refusal/strict-modes-failed and decision/strict-modes-refusal-before-any-write: viola open refuses strict-modes-failed before any log, a new verb under the rule.
@task_item n=10 part=touches
contract/revive-json-refusal/pinned-hash-mismatch: viola open shares the pinned-copy refusal pinned-hash-mismatch, its witness run's tampered-pin case, no open-specific case.
@task_item n=11 part=touches
decision/error-bodies-have-writer-and-witness: the census gains five open documents (strict-modes-failed, terminal-not-found, not-live, pinned-hash-mismatch, already-live witnessed by open) and two fixed non-error bodies (the open usage line, the open ok document); the Hyprland seam now has product callers, so its non-error-body reason no longer says no verb calls it.
@task_item n=12 part=touches
contract/cli-usage-error/exit-2, contract/cli-exit-codes/2 and contract/cli-fixed-message/form: a word after -- other than claude alone, or a self parent, is exit 2 with the fixed open usage line, before any home read or log; the usage line echoes none of the refused words.
@task_item n=13 part=touches
decision/fixed-cli-strings-pinned-by-literal, decision/fixed-cli-strings-pinned, decision/fixed-copy-from-design and decision/refusal-layout-literal-pins: new fixed strings, open's live line `<name>  live  <placement>`, its usage line and two hint pairs, each pinned by literal tests.
@task_item n=14 part=touches
decision/printed-names-are-parsed-arguments: open's live line and ok document print the name and parent as parsed arguments; the parent defaults to the caller's own name read from its environment.
@task_item n=15 part=touches
decision/state-carried-by-printed-word: open's placement is printed as a word from a closed list of eight (placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused).
@task_item n=16 part=touches
decision/cli-not-keyboard-interactive and decision/cli-no-interactive-input: viola open takes no input and waits non-interactively for liveness; a new verb under the rule.
@task_item n=17 part=touches
decision/run-diagnostics-off-terminal and decision/zero-viola-bytes-on-the-terminal: the wrapper's new window step at start (span run.window, hyprctl probes) writes diagnostics and a window event line, none to the child's terminal.
@task_item n=18 part=touches
contract/tui-passthrough/entry and contract/tui-passthrough/run-diagnostics: the wrapper's start order gains the window step under --origin, after the trust step and before the slots and first snapshot; the a11y-plan row that states the run start order is stale.
@task_item n=19 part=touches
decision/screen-reader-target-tui, decision/tui-terminal-belongs-to-child and decision/wrapped-tui-reads-as-unwrapped: viola open spawns a foot window on Hyprland holding the wrapped claude, the same terminal the tui screen-reader target names; the live behaviour is unproven (R2 red on the rig, gated).
@task_item n=20 part=touches
decision/live-readings-never-gate and decision/manual-a11y-pass-gates-nothing: R2, a live reading of the real foot window, ran once by the operator and gates nothing; its rig is checked by a syntax check only.
@task_item n=21 part=touches
decision/paths-not-carried: open hands its --cwd and the pinned path only to the spawned argv; the list --json window carries an address and a workspace, no path.
@task_item n=22 part=touches
decision/a11y-runs-in-existing-test-suites and decision/a11y-cases-inside-coverage-suite: the open's literal and schema checks run inside the existing integration layer by its cli_ prefix, no separate a11y suite.
@task_item n=23 part=touches
decision/linux-only: the window step and open's parent placement read Hyprland through hyprctl; off Hyprland the placement is unsupported and the session still goes live.
@task_fact n=1 slug=cognitive-accessibility
fact: viola open waits at most 30 s (OPEN_LIVE_BOUND, polled every 100 ms) for the opened session to go live; the bound falls on the open verb's own run and ends in the not-live refusal, exit 1, with its hint, never on the human.
@task_fact n=2 slug=cognitive-accessibility
fact: The wrapper's window step waits at most 5 s (WINDOW_WAIT) for its terminal's window under --origin; the bound falls on the wrapper's own start, and past it the placement is window-unseen and the start goes on.
@task_fact n=3 slug=cognitive-accessibility
fact: Each hyprctl call is bounded by HYPRCTL_DEADLINE 5 s, a compiled constant; the bound falls on the driver (open's parent read and the wrapper's window step), never on the human.
@task_fact n=4 slug=cognitive-accessibility
fact: Exit-1 refusal codes terminal-not-found (no foot terminal found, nothing spawned) and not-live (the opened session not live within its bound) each carry one fixed unable/hint pair and a --json error document with detail null.
@task_fact n=5 slug=cognitive-accessibility
fact: viola open refuses any word after -- other than claude alone, and a parent equal to the name, with exit 2 and one fixed usage line before any home read, log, pin or spawn; the line is a fixed message, no error body, and echoes none of the refused words.
@task_fact n=6 slug=a11y-scope-summary
fact: viola open's success output is the human line `<name>  live  <placement>` or, under --json, the one document {"v":1,"ok":{name,parent,window:{address,workspace}|null,placement}}.
@task_fact n=7 slug=visual-design-verification
fact: A session's placement is carried by a printed word from a closed list of eight: placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused.
@task_fact n=8 slug=7-screen-reader-support
fact: A session started by viola open runs in its own foot window on Hyprland, the terminal of the tui screen-reader target; its live reading is gated, unproven.
@task_item n=1 part=untouched
The human-takes-the-wheel, confirmed send and readback, and dialog pending then answer critical paths, and their keystroke classifier and wheel rows: no wheel, send or dialog behaviour moved.
@task_item n=2 part=untouched
viola link and viola unlink, viola plugin install output, viola revive --list, viola verify step and interrupted lines, viola wait waiting and timed_out: none of these outputs changed.
@task_item n=3 part=untouched
The instance-unreachable details and exit 21 document, the MCP wait tool timeout_ms and the config dialog_hold_s bound: no reach.
@task_item n=4 part=untouched
The absences of a web or graphical frontend, a touch surface, internationalization and a rendered page, and the N/A harness areas (contrast, focus, WCAG mapping, structured violation JSON): still true.
@task_item n=5 part=untouched
The CI and SLO decisions (fail-fast gate, zero flakiness, no a11y artifact, no separate driver, no performance budget): no agent-run, CI or verdict shape changed.
