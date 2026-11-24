@row_set n=1 row=decision/state-tree-strict-modes-before-use cell=rule seen=2 detector=none
value: run, revive's start, open, verify and focus hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1, and focus's exits 21 strict-modes-failed.
basis: fct-yz4mggmo (focus runs the strict-modes check before any log; exit 21 for a home or instance failing strict modes, strict-modes-failed); fct-fesrwmoj
because: viola focus is a new verb that holds the home tree to the strict-modes check before any log opens, refusing with exit 21 rather than 1, so the row's list of verbs and its exit must name it.
@row_set n=2 row=contract/cli-exit-codes/21 cell=promise seen=1 detector=none
value: A frame verb, or viola focus, refused because the instance failed the strict-modes check.
basis: fct-yz4mggmo (Exit 21 for a home or instance failing strict modes, strict-modes-failed)
because: viola focus opens no frame yet exits 21 on a strict-modes refusal, so exit 21 is no longer a frame verb's alone.
@row_add n=1 row=decision/focus-ancestor-walk-in-memory area=data-protection fact=F1 detector=none
name: Focus's ancestor walk held in memory
rule: viola focus's window lookup walks at most 8 ancestor pids of the session's wrapper, held in memory only and never recorded.
why: A boundary widening of what viola focus reads.
state: provisional
basis: fct-yz4mggmo (a private ANCESTRY of 8, the liveness ancestors walk); fct-pwx4cppm (the ancestry walk held in memory, never written)
because: viola focus adds a second ancestor walk beside open's, and its bound and its never being written are a decision the next verb reading ancestry must respect.
@row_add n=2 row=decision/focus-raises-only-ancestry-windows area=authentication-authorization fact=F2 detector=none
name: Focus raises only a window of the session's ancestry
rule: viola focus raises a session's recorded window only while that window's pid is in the wrapper's ancestry, else the window of the nearest ancestor pid; a recorded address outside the ancestry is never focused.
why: A boundary widening of what viola focus may raise.
guard: focus_client_skips_a_recorded_address_outside_the_ancestry
state: provisional
basis: fct-yz4mggmo (client_of: the recorded address only while its window's pid is in the wrapper's ancestry, else the first ancestor pid's window); fct-gtbzkyct
because: viola focus is the first product verb to dispatch a focus to the compositor, and which window it may raise is the security decision that bounds it.
@row_add n=3 row=decision/focus-liveness-before-hyprctl area=authentication-authorization fact=F3 detector=none
name: Focus refuses a session not live before any hyprctl read
rule: viola focus refuses a dead or unknown name with exit 21 (instance-dead or unwrapped-name) before any hyprctl read, after its strict-modes check.
guard: focus_of_a_dead_or_unknown_name_is_unreachable_before_any_hyprctl_read
basis: fct-yz4mggmo (live_endpoint then a same_process snapshot, else exit 21 with the not-live cause); fct-fesrwmoj
because: viola focus asks the compositor nothing for a session that is not live, an order the next compositor-reading verb must keep.
@row_add n=4 row=decision/focus-refusals-fixed-closed-codes area=error-handling fact=F4 detector=none
name: Focus refusals are fixed pairs of closed codes
rule: viola focus refuses with six closed exit-1 codes (window-not-found, unsupported, unreachable, oversize, malformed, refused), each a fixed unable and hint pair, and none of its output or log lines carries a window address, pid or workspace.
basis: fct-yz4mggmo (the six exit-1 refusal codes, each a --json document with detail null and one process-exit detail; no address, pid or workspace printed or logged)
because: viola focus adds a closed set of refusals over compositor replies, and that they are fixed and carry nothing of the window is the error-handling promise its writers must keep.
@row_add n=5 row=decision/focus-human-cli-only area=authentication-authorization fact=F6 detector=none
name: Focus is a human CLI control only
rule: viola focus is a human CLI control only; no MCP tool or other driver-facing focus exists, and adding one is a further boundary widening.
why: A boundary widening held to its narrow arm.
state: provisional
basis: fct-tfjv3ute (left out: a driver-facing focus, an MCP tool, a further widening); fct-vviqcehe (no MCP tool added)
because: focus reaches the compositor for the human only, and a driver-facing focus would widen the boundary, so its absence binds the work to come.
@row_add n=6 row=decision/focus-seam-calls-bounded area=input-validation fact=F7 detector=D-security-input
name: Focus makes at most three Hyprland seam calls
rule: viola focus makes at most three Hyprland seam calls, each a hyprctl child under the hyprctl deadline: one clients read, a workspace dispatch only for a workspace id of 1 or more, then the window focus dispatch.
basis: fct-yz4mggmo (one Request::Clients, then Request::Workspace(w) for w >= 1, then Request::FocusWindow); fct-pwx4cppm (at most three seam reads, each under HYPRCTL_DEADLINE)
because: viola focus is a new caller of the compositor seam whose replies decide its dispatches, so the bound on its calls and the workspace id it accepts are its input contract.
@row_none n=1 fact=F5 type=4
relied: viola focus only reads the recorded window and writes no snapshot or event line, which the standing rows on the snapshot's one writer and on state readers never writing already hold.
