@row_set n=1 row=decision/strict-modes-refusal-before-any-write cell=rule seen=1 fact=F2 detector=none
value: A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs, link/unlink and focus, exit 1 for run, revive's start and verify.
basis: fct-yz4mggmo, fct-fesrwmoj
because: viola focus now refuses a home or instance failing strict modes with exit 21 before any log, so the row's list of exit-21 verbs must name focus.
@row_set n=2 row=contract/cli-exit-codes/21 cell=promise seen=1 detector=none
value: instance-unreachable: a frame verb, or viola focus before any compositor read, was refused before reaching the instance, the cause in detail.
basis: fct-yz4mggmo, fct-fesrwmoj
because: viola focus, which is no frame verb, now exits 21 for instance-dead, unwrapped-name and strict-modes-failed before any hyprctl read, so the promise that names only frame verbs is incomplete.
@row_add n=1 row=contract/viola-focus-refusal/pair area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus human refusals
item: unable/hint pair
promise: Without `--json`, each exit-1 refusal of `viola focus` is one fixed `unable` line and one `hint:` line last on stderr, one pair per code.
basis: fct-yz4mggmo, fct-gsppqa74
because: The new viola focus verb refuses at exit 1 with one fixed unable/hint pair per code, a refusal contract the plan's cognitive section holds for each verb's refusals.
@row_add n=2 row=contract/viola-focus-json-refusal/document area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus --json refusals
item: exit-1 refusal document
promise: Each exit-1 refusal of `viola focus` under `--json` is one document `{"v":1,"error":"<code>","detail":null}` on stdout with no hint.
basis: fct-yz4mggmo, fct-feiatilz
because: The new viola focus verb gives one clean --json refusal document per exit-1 code, a new promise at the CLI boundary.
@row_add n=3 row=contract/viola-focus-json-refusal/window-not-found area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus --json refusals
item: window-not-found
promise: One of the closed set of `error` codes a `viola focus` exit-1 refusal carries: no window of the session was found, and nothing is dispatched.
basis: fct-yz4mggmo, fct-gsppqa74
because: window-not-found is a new code in viola focus's closed exit-1 refusal set.
@row_add n=4 row=contract/viola-focus-json-refusal/unsupported area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus --json refusals
item: unsupported
promise: One of the closed set of `error` codes a `viola focus` exit-1 refusal carries: off Hyprland, with nothing started.
basis: fct-yz4mggmo, fct-gsppqa74
because: unsupported is a new code in viola focus's closed exit-1 refusal set.
@row_add n=5 row=contract/viola-focus-json-refusal/unreachable area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus --json refusals
item: unreachable
promise: One of the closed set of `error` codes a `viola focus` exit-1 refusal carries: no compositor answered.
basis: fct-yz4mggmo, fct-gsppqa74
because: unreachable is a new code in viola focus's closed exit-1 refusal set.
@row_add n=6 row=contract/viola-focus-json-refusal/oversize area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus --json refusals
item: oversize
promise: One of the closed set of `error` codes a `viola focus` exit-1 refusal carries.
basis: fct-yz4mggmo, fct-gsppqa74
because: oversize is a new code in viola focus's closed exit-1 refusal set.
@row_add n=7 row=contract/viola-focus-json-refusal/malformed area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus --json refusals
item: malformed
promise: One of the closed set of `error` codes a `viola focus` exit-1 refusal carries.
basis: fct-yz4mggmo, fct-gsppqa74
because: malformed is a new code in viola focus's closed exit-1 refusal set.
@row_add n=8 row=contract/viola-focus-json-refusal/refused area=cognitive-accessibility fact=F1 detector=none
boundary: viola focus --json refusals
item: refused
promise: One of the closed set of `error` codes a `viola focus` exit-1 refusal carries.
basis: fct-yz4mggmo, fct-gsppqa74
because: refused is a new code in viola focus's closed exit-1 refusal set.
@row_add n=9 row=contract/viola-focus/result-line area=a11y-scope-summary fact=F3 detector=none
boundary: viola focus output
item: result line
promise: `viola focus` prints `<name>  focused` on stdout in one write with no SGR, and prints no window address, pid or workspace.
basis: fct-yz4mggmo, fct-p6tk5bfr
because: The new viola focus verb promises a plain one-write result line that holds nothing of the window, an output-discipline contract of the scope summary.
@row_add n=10 row=contract/viola-focus/json-ok area=a11y-scope-summary fact=F3 detector=none
boundary: viola focus output
item: --json output
promise: Under `--json`, `viola focus` prints one `{"v":1,"ok":{"name":"<name>"}}` document, with no window address, pid or workspace.
basis: fct-yz4mggmo, fct-feiatilz
because: The new viola focus verb promises one --json success document that holds nothing of the window.
@row_add n=11 row=contract/cli-help/board-row area=aria-patterns-roles fact=F4 detector=none
boundary: viola --help
item: board row
promise: The help `board:` row is `board:    list, focus`.
basis: fct-ud37d4a7
because: The --help board group row moved from list to list, focus, and the help group labels carry the CLI's structure in this section.
@row_add n=12 row=decision/focus-off-hyprland-refuses area=a11y-scope-summary fact=F5 detector=none
name: Focus raises windows only through Hyprland
rule: viola focus raises a session's window only through Hyprland; off Hyprland it refuses `unsupported` and starts nothing.
state: provisional
basis: fct-gsppqa74, fct-cxhak2ir
because: The new verb's reach is bounded to Hyprland on a fork still open, so the scope summary must carry it as a provisional decision beside the standing Linux-only row.
