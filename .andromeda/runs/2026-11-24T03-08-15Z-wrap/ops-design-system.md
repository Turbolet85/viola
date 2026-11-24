@row_set n=1 row=contract/cli-refusal-line/fixed-unable-form cell=promise seen=2 detector=D-design-cli-wording
value: The exit-1 refusals of `run`, `revive`, `open`, `focus` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
basis: fct-yz4mggmo
because: `viola focus`'s exit-1 refusals print a fixed unable/hint pair, so the closed list of fixed-form verbs must name focus.
@row_set n=2 row=contract/cli-stderr/fixed-message-refusal cell=promise seen=2 detector=D-design-cli-wording
value: The exit-1 refusals of `viola run`, `revive`, `open`, `focus` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
basis: fct-yz4mggmo
because: `viola focus`'s exit-1 refusals use the fixed unable/hint form, so the closed list of fixed-message verbs must name focus.
@row_set n=3 row=contract/help-output/verb-groups cell=promise seen=2 detector=D-design-cli-wording
value: `viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, verify, plugin install`, then `Options:`.
basis: fct-ud37d4a7
because: The help template's board row moved from `list` to `list, focus` (PROVISIONAL, F-B), so the stated help wording must follow.
@row_set n=4 row=contract/viola-help/verb-groups cell=promise seen=2 detector=D-design-cli-wording
value: `viola --help` lists the served verbs only, grouped `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, verify, plugin install`, then `Options:`.
basis: fct-ud37d4a7
because: The help template's board row moved from `list` to `list, focus` (PROVISIONAL, F-B), so the stated help wording must follow.
@row_set n=5 row=contract/exit-21-json-cause/strict-modes-failed cell=promise seen=1 fact=F4 detector=none
value: The `list`, `link`, `unlink`, `focus` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.
basis: fct-yz4mggmo
because: `viola focus` refuses a home or instance failing strict modes with exit 21 `strict-modes-failed`, so the closed list of verbs reading that cause must name focus.
@row_add n=1 row=name/command/focus area=surface-cli fact=F1 detector=none
class: command
name: focus
basis: fct-yz4mggmo
because: The command name `focus` is now occupied by the new `viola focus <name> [--json]` verb, a board verb beside `list`.
@row_add n=2 row=contract/cli-verb-output/focus area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: CLI verb output
item: viola focus
promise: `viola focus <name>` prints `<name>  focused` on stdout, or under `--json` `{"v":1,"ok":{"name":"<name>"}}`; no window address, pid or workspace is ever printed or logged.
basis: fct-yz4mggmo
because: `viola focus` adds a new success output wording that no master states.
@row_add n=3 row=contract/viola-focus-exit-1-codes/window-not-found area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola focus exit-1 codes
item: window-not-found
promise: `window-not-found` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"window-not-found","detail":null}`, in human mode its fixed `unable:` / `hint:` pair, nothing dispatched.
basis: fct-yz4mggmo
because: `viola focus` adds a closed set of exit-1 refusal codes, one contract row an item.
@row_add n=4 row=contract/viola-focus-exit-1-codes/unsupported area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola focus exit-1 codes
item: unsupported
promise: `unsupported` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"unsupported","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
basis: fct-yz4mggmo
because: `viola focus` adds a closed set of exit-1 refusal codes, one contract row an item.
@row_add n=5 row=contract/viola-focus-exit-1-codes/unreachable area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola focus exit-1 codes
item: unreachable
promise: `unreachable` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"unreachable","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
basis: fct-yz4mggmo
because: `viola focus` adds a closed set of exit-1 refusal codes, one contract row an item.
@row_add n=6 row=contract/viola-focus-exit-1-codes/oversize area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola focus exit-1 codes
item: oversize
promise: `oversize` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"oversize","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
basis: fct-yz4mggmo
because: `viola focus` adds a closed set of exit-1 refusal codes, one contract row an item.
@row_add n=7 row=contract/viola-focus-exit-1-codes/malformed area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola focus exit-1 codes
item: malformed
promise: `malformed` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"malformed","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
basis: fct-yz4mggmo
because: `viola focus` adds a closed set of exit-1 refusal codes, one contract row an item.
@row_add n=8 row=contract/viola-focus-exit-1-codes/refused area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola focus exit-1 codes
item: refused
promise: `refused` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"refused","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
basis: fct-yz4mggmo
because: `viola focus` adds a closed set of exit-1 refusal codes, one contract row an item.
@row_add n=9 row=contract/viola-focus-refusals/exit-21 area=surface-cli fact=F4 detector=none
boundary: viola focus refusals
item: exit 21
promise: `viola focus` exits 21 with `instance-dead` or `unwrapped-name` for a name not live, and with `strict-modes-failed` for a home or instance failing strict modes, each before any compositor read.
basis: fct-yz4mggmo
because: `viola focus` adds exit-21 refusals made before any compositor read, a promise at the CLI boundary no row states for this verb.
@row_add n=10 row=contract/viola-focus-refusals/socket-dir area=surface-cli fact=F5 detector=none
boundary: viola focus refusals
item: socket directory
promise: `viola focus` connects to no wrapper socket, so a loosened socket directory does not refuse it with `socket-dir-refused`, unlike the frame verbs.
basis: fct-o7pgtknq
because: `viola focus` reads liveness only and is the exception to the frame-verb socket-directory refusal, which the frame-verb row does not say.
@row_add n=11 row=decision/focus-raises-only-its-session-window area=surface-cli fact=F6 detector=none
name: viola focus raises only its session's window
rule: `viola focus` raises the recorded window only while its pid is in the wrapper's ancestry, else the nearest ancestor's window; a recorded window outside the ancestry is never focused, and none found refuses `window-not-found` with nothing dispatched.
state: provisional
basis: fct-yz4mggmo
because: The new verb chooses which window to raise by ancestry, a decision future work on focus must respect, built on the provisional arm of F-A.
@row_add n=12 row=decision/focus-human-verb-only area=surface-cli fact=F7 detector=none
name: focus is a human CLI verb only
rule: `focus` is a human CLI verb with no MCP tool; a driver-facing focus is a further widening, taken only on the operator's word.
state: provisional
basis: fct-tfjv3ute
because: The chunk left a driver-facing focus out on F-A's provisional arm 2, a standing limit on the verb's callers that no row states.
