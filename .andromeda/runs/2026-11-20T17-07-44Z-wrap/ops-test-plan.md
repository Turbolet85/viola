@row_set n=1 row=decision/human-never-blocked cell=except seen=1 detector=none
value: The start's verify window, before the agent child is spawned.
basis: fct-uidyyps4
because: A keyed start now runs viola verify before the child, and the human-never-blocked decision takes the start's verify window as its named exception.
@row_set n=2 row=decision/run-checks-before-child-spawn cell=rule seen=1 fact=F6 detector=none
value: viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint, runs the start's verify where keyed, writes the trust record on the final verdict, its first snapshot, heartbeat and start events, then spawns the child.
basis: fct-zfntgo5a
because: The start order gains the start's verify after the bind and moves the trust record onto the final verdict after it, so the row's order is incomplete.
@row_set n=3 row=decision/single-writer-state-files cell=rule seen=1 detector=none
value: Only the wrapper writes snapshot.json and only verify writes stamps.json, the verify a keyed start runs included; every other writer-capable entry point leaves both byte-identical.
basis: fct-uidyyps4
because: A keyed viola run start now starts verify, which stays the one stamps writer, so the row must say a start's own verify is that writer.
@row_set n=4 row=contract/fake-agent-cli/argv-only-options cell=promise seen=2 detector=none
value: The fake agent's modes and options are taken from argv and from a fake-args file beside its exe, one argument per line within MAX_FRAME, put ahead of argv so argv's own value wins, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
basis: fct-zfntgo5a
because: The fake agent now reads a fake-args file beside its executable ahead of argv, so options no longer come from argv alone.
@row_set n=5 row=decision/fake-agent-argv-only cell=except seen=2 fact=F8 detector=none
value: As hyprctl by its own file name it reads its mode and reply set from files beside its exe; a fake-args file beside its exe, one argument per line within MAX_FRAME, goes ahead of argv, argv winning.
basis: fct-zfntgo5a
because: The fake-args file is a new named exception to argv-only configuration and the row's except must name it.
@row_add n=1 row=contract/config-json/verify-new-cli area=integration-test-strategy fact=F1 detector=none
boundary: config.json
item: verify_new_cli
promise: A JSON boolean, default false; true makes a viola run start meeting an unstamped claude version run viola verify once before the child; any other value reads false with one parse-rejected config-json malformed line.
basis: fct-xgc7qjiw
because: A new config.json key turns on the start's automatic verify and its parse is promised outward.
@row_add n=2 row=decision/one-verify-across-concurrent-starts area=integration-test-strategy fact=F2 detector=none
name: One verify across concurrent starts
rule: Two concurrent keyed starts run one verify: an owner-only attempt lock in the ledger, taken without waiting after the strict-modes check and never the stamps lock, makes the other start read verifying-elsewhere and start unverified.
guard: verify_window_two_starts_at_once_run_one_verify
basis: fct-zfntgo5a
because: The start's verify adds an attempt lock so that two starts never run two verifies, a decision later work must respect.
@row_add n=3 row=decision/attempted-version-not-reverified area=integration-test-strategy fact=F3 detector=none
name: An attempted version is not verified again
rule: A version whose stamp entry holds every relied row id yet is unverified runs no verify again at start and prints the earlier-red line; an entry short of one relied id verifies again once.
guard: verify_window_run_whose_verify_goes_red_records_it_and_the_next_start_runs_none
basis: fct-jupau6dq
because: The start's verify reads an attempted entry so that a red verify never loops, a new rule of the start.
@row_add n=4 row=contract/run-start-verify-stop/signal area=integration-test-strategy fact=F4 detector=none
boundary: viola run start's verify interrupted
item: SIGINT or SIGTERM
promise: TERMs the verify child, waits 30 s, then KILLs, writes its process-exit line and ends the start by that signal: no stamp, no snapshot, no session receipt, no verify probe dir.
basis: fct-gookuxzl
because: A stop during the start's verify is a new outward behaviour of viola run, measured by the stop case.
@row_add n=5 row=contract/diag-subject/auto-verify area=log-format fact=F7 detector=D-tests-obs-harness
boundary: diag spawn subject
item: auto-verify
promise: One process-start and process-exit pair per start's verify, the exit carrying child_exit_status and duration_ms.
basis: fct-xgc7qjiw
because: The closed diag-line spawn subject enum gains auto-verify, a log contract test-plan's log format must hold alongside obs-plan.
@row_add n=6 row=contract/start-verify-lines/format area=unit-test-strategy fact=F10 detector=none
boundary: viola run start's verify lines
item: L1 to L5
promise: Five fixed ASCII stderr lines, one write each, no SGR, carrying the parsed version and a closed word only; none is printed under revive --json.
basis: fct-zfntgo5a
because: The start's verify adds five human stderr lines whose format is promised outward and pinned by unit literals.
@row_none n=1 fact=F5 type=4
relied: the start's verify child runs with stdin null, the start's environment strip and the instance variables removed, tied to the start by parent-death TERM, silent under revive --json
@row_none n=2 fact=F9 type=5
what: the live rig's AV arm, labels and summary lines are an inventory the rig's code holds; its reading-not-verification rule already stands in decision/live-failing-reading-not-verification
