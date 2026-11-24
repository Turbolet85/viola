@row_set n=1 row=contract/viola-list-json/items cell=promise seen=4 detector=none
value: In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window? {address, workspace}, origin? cli|agent}.
basis: fct-wrqnldxo
because: The list --json item gains window? {address, workspace} and origin?, so the row's closed item shape is now incomplete.
@row_set n=2 row=contract/run-start-events/order cell=promise seen=4 detector=none
value: The events log reads wheel with cause start, then lineage with source wrapper, then launch with source wrapper, then, for a start under --origin, one window line with source wrapper, then the agent's session-start with source hook.
basis: fct-wrqnldxo
because: A start under --origin now appends one window line after launch, so the start's event order the row promises gains that line.
@row_set n=3 row=decision/run-start-verify-and-trust-after-bind cell=rule seen=1 detector=none
value: Between its exclusive endpoint bind and its first snapshot, a viola run start runs any automatic verify and then, on a verified final verdict, writes an untrusted spawn folder's trust record, and then, under --origin, takes its window step.
basis: fct-xxzbm355
because: The window step now runs after the trust step and before the slots and the first snapshot, only under --origin, so the row's steps between bind and first snapshot are incomplete.
@row_set n=4 row=decision/fake-agent-on-test-path cell=rule seen=2 fact=F5 detector=none
value: The fake agent stands as claude in a test-scoped bin dir, or as hyprctl or as foot, the terminal stand-in, by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
basis: fct-xxzbm355
because: The fake agent now also stands as foot, the terminal stand-in, by its file name, so the row naming its stand-in roles says it less.
@row_set n=5 row=decision/fake-agent-as-claude cell=rule seen=2 detector=none
value: The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, as hyprctl in a test-scoped bin dir, and as foot, the terminal stand-in.
basis: fct-xxzbm355
because: The terminal viola open spawns is a new external child process served in tests by the fake agent under the file name foot, so the row's list of what the fake serves is incomplete.
@row_set n=6 row=decision/fakes-for-external-services cell=rule seen=4 detector=none
value: The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, the real hyprctl by the same binary as hyprctl, and the real foot terminal by the same binary as foot; tests never reach the real services.
basis: fct-xxzbm355
because: The real terminal is now an external service replaced in tests by the fake agent as foot, so the row's list of replaced services is incomplete.
@row_add n=1 row=decision/viola-open-argument-allowlist area=integration-test-strategy fact=F1 detector=D-tests-coverage
name: viola open argument allowlist
rule: viola open accepts only its allowlisted flags and claude alone after --; any other word exits 2 before any home read, log, pin or spawn, and a parent naming the opened instance itself exits 2.
guard: open_args_refuse_a_word_after_dashes_other_than_claude, open_refuses_a_word_after_dashes_and_reads_nothing
basis: fct-xxzbm355, fct-4kn64u2g
because: The new open verb's closed argument surface is a contract its integration tests hold, and no row states it.
@row_add n=2 row=decision/run-window-flags-closed area=integration-test-strategy fact=F3 detector=D-tests-coverage
name: viola run's hidden window flags are closed
rule: viola run's hidden --origin takes only an origin word, cli or agent, and its hidden --place given without --origin is a usage error.
basis: fct-xxzbm355
because: viola run gains two hidden flags with a closed word set and a pair check, and no row states them.
@row_add n=3 row=contract/open-exit-1/terminal-not-found area=e2e-test-strategy fact=F2 detector=none
boundary: viola open exit 1 causes
item: terminal-not-found
promise: The terminal program is not found; human output ends with a hint of its own, and under --json the document is {"v":1,"error":"terminal-not-found","detail":null}.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open adds the exit-1 cause terminal-not-found with its own hint and named document, a new item at the CLI boundary.
@row_add n=4 row=contract/open-exit-1/not-live area=e2e-test-strategy fact=F2 detector=none
boundary: viola open exit 1 causes
item: not-live
promise: The opened instance does not read live within OPEN_LIVE_BOUND; human output ends with a hint of its own, and under --json the document is {"v":1,"error":"not-live","detail":null}.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open adds the exit-1 cause not-live with its own hint and named document, a new item at the CLI boundary.
@row_add n=5 row=contract/event-log/window area=test-scope-summary fact=F4 detector=none
boundary: the event log
item: window
promise: A start under --origin logs one window line with source wrapper after launch, holding address, workspace, placement and origin; placement is one of placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed or refused, and origin is cli or agent.
basis: fct-xxzbm355, fct-wrqnldxo
because: The event log gains a window kind with fixed keys and closed placement and origin words, a new item of the log's contract.
@row_add n=6 row=decision/snapshot-replay-window area=test-scope-summary detector=none
name: Snapshot replay recovers the window record
rule: The snapshot replay recovers the window record from the event log, the last window line after the last launch line winning, a later launch line clearing it, and a line with an unknown word dropped.
guard: window_rule_the_last_line_after_launch_wins, window_rule_a_later_launch_clears_it
basis: fct-wrqnldxo, fct-4kn64u2g
because: The replay now recovers window, and the standing snapshot-replay-fields row cannot take it within its bound, so the recovered window stands as a row of its own.
@row_add n=7 row=decision/window-placement-proven-live area=e2e-test-strategy fact=F7 detector=none
name: Real window placement proven only live
rule: A real terminal window placed on its parent's workspace on Hyprland is proven only by a live reading; the fakes prove the placement words and the window records only.
basis: fct-4kn64u2g, fct-pko6nyrw
because: The open's acceptance rests real placement on the live reading alone and the fakes on words and records, a tier decision no row states.
@row_add n=8 row=decision/live-rig-stand-in-parent area=e2e-test-strategy fact=F8 detector=none
name: Live rig stand-in parent and clean environment
rule: A live rig's stand-in parent program takes the wrapper's own arguments, the rig unsets every inherited VIOLA_* name first, and its board reads run with no claude on PATH.
why: A stand-in refusing the wrapper's arguments ends its child at once, an inherited name leaks into the scratch home, and a board read under PATH runs the real claude.
basis: fct-njf4xxzw, fct-spgqytne
because: R2's red was measured to these three rig faults and the rig was mended to hold each, a rule every later live rig must keep.
@row_add n=9 row=decision/echo-oracle-skips-allowed-word area=11-test-anti-patterns-never-do-these fact=F9 detector=none
name: Echo oracle skips the one allowed word
rule: An oracle asserting that a refusal echoes none of the given words skips the one allowed word when the refusal's fixed line itself names it.
why: Otherwise the fixed line's own mention of the allowed word reads as an echo.
basis: fct-5z6lqruv
because: The chunk met this sweep hazard in its refusal oracles, a test convention no anti-pattern row states.
@row_none n=1 fact=F6 type=5
what: An inventory of three tap recorder tests over a planted stub; the code and the carry's closure hold it.
