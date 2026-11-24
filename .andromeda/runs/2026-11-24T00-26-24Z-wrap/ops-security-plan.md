@row_set n=1 row=decision/hyprctl-read-deadline cell=rule seen=2 fact=F9 detector=D-security-input
value: The hyprctl child is resolved on the calling process's PATH and runs with null stdin and the named environment variables removed, the R8 strip's names on the wrapper's read, under a 5 s deadline at which it is killed and reaped; a deadline kill or a non-zero exit reads as unreachable.
basis: fct-xxzbm355
because: The wrapper's window step now reads hyprctl as a product caller, resolving it on PATH and removing the R8 strip's names from each read, which the row did not say.
@row_set n=2 row=decision/state-tree-strict-modes-before-use cell=rule seen=1 detector=D-security-input
value: run, revive's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
basis: fct-xxzbm355
because: The new viola open verb refuses strict-modes-failed with exit 1 before any log, so the row's list of verbs holding the check is incomplete.
@row_add n=1 row=decision/open-closed-argument-allowlist area=input-validation fact=F1 detector=D-security-input
name: viola open's closed argument allowlist
rule: viola open takes only the name and the flags --parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort and --json, and the one word claude after the dashes; any other word, or a parent equal to the name, is exit 2 before any home read, log, pin or spawn.
guard: open_args_refuse_a_word_after_dashes_other_than_claude; open_refuses_a_word_after_dashes_and_reads_nothing
state: provisional
basis: fct-xxzbm355, fct-4kn64u2g
because: The chunk adds viola open as a new CLI input surface whose closed allowlist is the validation that surface must keep.
@row_add n=2 row=decision/open-terminal-direct-spawn area=input-validation fact=F2 detector=D-security-input
name: viola open spawns its terminal directly
rule: viola open spawns the terminal program resolved on the calling process's PATH directly, never through a shell, with a closed argv of one element per value, and starts in it only the pinned viola run with claude.
guard: open_argv_is_the_closed_argv_one_element_each; open_spawns_the_terminal_with_the_closed_argv
state: provisional
basis: fct-4kn64u2g, fct-xxzbm355
because: The chunk adds a new spawn of a terminal by viola open, and its no-shell closed argv is the decision future work must keep.
@row_add n=3 row=decision/run-window-flags-checked area=input-validation fact=F3 detector=D-security-input
name: viola run's hidden window flags checked
rule: viola run's hidden --origin takes only the closed words cli or agent and --place only an integer, and --place without --origin is a usage error.
why: A boundary widening of the started run's argv.
guard: run_args_refuse_a_place_without_an_origin; run_args_take_a_hidden_origin_and_place
state: provisional
basis: fct-xxzbm355, fct-ofc4imyq
because: The chunk adds two hidden flags to viola run's argv, a widening whose closed-word and integer checks no row states.
@row_add n=4 row=decision/board-window-and-origin-re-parsed area=input-validation fact=F8 detector=D-security-input
name: Board window and origin shown only when they re-parse
rule: viola list --json and the MCP list item carry a recorded window only when its address re-parses through the one window address validator, and an origin only when it is a closed origin word.
why: A boundary widening of the list item.
guard: board_read_drops_a_window_address_that_does_not_parse
state: provisional
basis: fct-wrqnldxo, fct-ofc4imyq
because: The board item gains window and origin, a widening whose re-parse rule the board rows for the launch pair and lineage do not cover.
@row_add n=5 row=decision/window-origin-authenticates-nothing area=authentication-authorization fact=F4 detector=D-security-auth
name: Recorded window origin authenticates nothing
rule: The recorded window origin, cli or agent, is self-reported by the caller and authenticates nothing.
state: provisional
basis: fct-ofc4imyq
because: The chunk records a caller-reported origin word, and no row says it is no identity.
@row_add n=6 row=decision/window-step-moves-own-terminal-window area=authentication-authorization fact=F5 detector=none
name: Wrapper's window step moves only its own terminal's window
rule: The wrapper's window step moves only the window of its own terminal, found by its parent pid within a 5 s wait, at most once, and never a window it did not open.
guard: window_step_moves_the_window_of_its_terminal_to_the_place
state: provisional
basis: fct-xxzbm355, fct-m35lqujh
because: The product now moves a desktop window, and the standing own-window row binds only the test-only recorder.
@row_add n=7 row=decision/window-record-kept-to-snapshot-and-event-log area=data-protection fact=F6 detector=none
name: Window record kept to the snapshot and the event log
rule: An instance's window record (address, workspace, placement and origin) is kept only in the snapshot and the event log's window line and leaves only through list --json and the MCP list item; no address, pid or workspace enters a process-log line.
state: provisional
basis: fct-wrqnldxo, fct-m35lqujh
because: The chunk adds a window record to the snapshot and the event log, and where it may and may not go is a new data rule.
@row_add n=8 row=decision/open-ancestor-walk-in-memory area=data-protection fact=F7 detector=none
name: Open's ancestor walk held in memory
rule: viola open's parent-window lookup walks at most 8 ancestor pids, held in memory only and never recorded.
why: A boundary widening of what viola open reads.
state: provisional
basis: fct-ofc4imyq, fct-xxzbm355
because: The chunk adds a bounded read of the process ancestry, a widening whose bound and no-record rule no row states.
@row_add n=9 row=contract/open-refusals/terminal-not-found area=error-handling fact=F10 detector=none
boundary: viola open exit-1 refusals
item: terminal-not-found
promise: Exit 1 with, under --json, exactly {"v":1,"error":"terminal-not-found","detail":null}, else its one fixed hint; it names no path or pid.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open adds a new exit-1 refusal whose fixed, detail-free body is a promise at the CLI boundary.
@row_add n=10 row=contract/open-refusals/not-live area=error-handling fact=F10 detector=none
boundary: viola open exit-1 refusals
item: not-live
promise: Exit 1 with, under --json, exactly {"v":1,"error":"not-live","detail":null}, else its one fixed hint; it names no path or pid.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open adds a new exit-1 refusal whose fixed, detail-free body is a promise at the CLI boundary.
@row_add n=11 row=decision/live-rig-strips-viola-names area=secret-management fact=F11 detector=none
name: Live rig strips VIOLA names, no claude on board reads
rule: A live rig that runs viola commands unsets every inherited VIOLA_* name first and runs every board read with no claude on its PATH.
why: Under a scratch home the board read otherwise runs the real claude agents read under the user's real config.
state: provisional
basis: fct-njf4xxzw, fct-spgqytne
because: The live round showed a rig reading the session's VIOLA names and running the real claude under the user's config, and the mend is a convention every rig must keep.
