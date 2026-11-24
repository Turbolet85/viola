@row_set n=1 row=decision/open-terminal-direct-spawn cell=rule seen=1 fact=F1 detector=D-security-input
value: viola open and viola up spawn the terminal program resolved on the calling process's PATH directly, never through a shell, with a closed argv of one element per value; open starts in it only the pinned viola run with claude, up only the pinned viola revive <name> --origin in the recorded cwd.
basis: fct-ykd4mawd (the closed argv of up's spawn_terminal), fct-vdfhytdg (expected amendment to decision/open-terminal-direct-spawn)
because: viola up now spawns its terminal through open's shared direct spawn with a closed argv starting the pinned revive, so the row that names only open's terminal is incomplete.
@row_set n=2 row=decision/run-window-flags-checked cell=rule seen=1 fact=F2 detector=D-security-input
value: viola run's hidden --origin takes only the closed words cli or agent and --place only an integer, and --place without --origin is a usage error; viola revive's hidden --origin takes the same two words, conflicts with --list and asks a window only at the workspace a real snapshot recorded.
basis: fct-ykd4mawd (revive's hidden --origin), fct-vdfhytdg (expected amendment to decision/run-window-flags-checked)
because: viola revive gains a hidden --origin held to the same two closed words, so the row that checks only run's window flags must name it.
@row_set n=3 row=contract/revive-arguments/id cell=promise seen=2 detector=D-security-input
value: --id accepts only the session-id shape, anything else a usage error with exit 2; --list conflicts with --id, --fork, --model, --effort, --origin and the trailing arguments.
basis: fct-ykd4mawd (revive's hidden --origin conflicts with --list)
because: revive's new hidden --origin conflicts with --list, so the closed list of flags --list refuses is incomplete without it.
@row_set n=4 row=decision/max-frame-shared-cap cell=rule seen=2 fact=F3 detector=D-security-input
value: One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config, stamps and the /proc/stat boot time.
basis: fct-ykd4mawd (liveness::boot_time, one /proc/stat read through take(MAX_FRAME)), fct-v57w6gal (the /proc/stat reader's validation)
because: The chunk adds /proc/stat as a new external read bounded by MAX_FRAME, and the row lists the external reads the cap bounds.
@row_set n=5 row=contract/cli-exit-codes/21 cell=promise seen=2 detector=none
value: A frame verb, or viola focus, refused because the instance failed the strict-modes check, or viola up refused by its board scan's strict-modes check.
basis: fct-ykd4mawd (Exits: 21 for the board's strict refusal)
because: viola up exits 21 when its strict board scan refuses, a cause the exit-21 promise does not yet name.
@row_set n=6 row=decision/cwd-only-in-snapshot cell=rule seen=2 detector=none
value: The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot, as the key of the start's trust record in the Claude config, and as the working directory on viola up's terminal argv, never on a log line, error body or stdout.
basis: fct-ykd4mawd (the closed argv carries --working-directory=<recorded cwd>)
because: viola up hands the recorded cwd to its terminal's argv, so a row saying it stands only in the snapshot and the Claude config is contradicted.
@row_set n=7 row=decision/spawn-cwd-snapshot-only cell=rule seen=2 detector=none
value: The start's spawn directory lives only in the 0600 snapshot, as the key of its trust record in the Claude config, and as the working directory on viola up's terminal argv, on no log line, error body or stdout, and is omitted from the snapshot when absent.
basis: fct-ykd4mawd (the closed argv carries --working-directory=<recorded cwd>)
because: viola up hands the recorded spawn directory to its terminal's argv, so a row saying it lives only in the snapshot and the Claude config is contradicted.
@row_add n=1 row=decision/up-checks-before-log-and-spawn area=authentication-authorization fact=F4 detector=none
name: viola up checks before its log and before a spawn
rule: viola up runs the strict board scan before its log opens, and holds the home tree to the strict-modes check and re-hashes the pinned copy before its first spawn.
basis: fct-ykd4mawd (Order: board::scan, then the cli log; before the first spawn check_home_tree and pin_bytes)
because: viola up is a new verb that spawns pinned starts, and no row says which checks precede its log and its first spawn.
@row_add n=2 row=contract/strict-modes-refusal/up area=authentication-authorization detector=none
boundary: strict-modes refusal
item: viola up
promise: The board scan's refusal exits 21 with no line and nothing spawned, under --json the instance-unreachable document with detail strict-modes-failed.
basis: fct-ykd4mawd (Exits: 21 for the board's strict refusal), fct-m4vsf7nm (exit 21 with no line)
because: The per-verb strict-modes refusal contracts hold no item for the new viola up verb.
@row_add n=3 row=decision/up-writes-no-state area=data-protection fact=F5 detector=none
name: viola up writes no state and names no window
rule: viola up writes no snapshot, event line or stamp, and its lines, hint and --json document hold closed words and instance names only, never a window address, pid or workspace.
basis: fct-ykd4mawd (It writes no snapshot, event line or stamp), fct-v57w6gal (codes and names only; no address, pid or workspace), fct-lwvzzjno
because: viola up is a new verb reading every instance's state, and no row says it writes none and what its output may carry.
@row_add n=4 row=decision/test-teardown-signals-recorded-pids-only area=security-anti-patterns-never-do-these fact=F6 detector=none
name: Test teardown signals only recorded live pids
rule: Test-side teardown of a stand-in terminal's tree signals only a pid it recorded whose start time still reads the recorded one, and kills a leaf first only while its recorded parent runs.
why: A kill of a fake agent already in its own exit truncated its coverage profile.
basis: fct-ykd4mawd (support::terminal::end_trees), fct-z7bnkfyd (the cause: guards SIGKILLed a fake agent in its own exit)
because: The chunk adds one shared test teardown that signals processes, and no row bounds which pids a test may signal.
@row_add n=5 row=decision/r4-rig-no-credential-path area=secret-management fact=F7 detector=none
name: R4 live rig holds no credential path
rule: The R4 live rig for viola up refuses a live root inside a repository, refuses its R4b phase until it is granted, and holds no credential path.
basis: fct-w5jqbx2g (scripts/r4-up.sh, R4b refusing first), fct-kolziobl (no credential path stands), fct-vdfhytdg (live root outside every repository)
because: The chunk adds a new test-only live rig, and no secret-management row bounds it.
