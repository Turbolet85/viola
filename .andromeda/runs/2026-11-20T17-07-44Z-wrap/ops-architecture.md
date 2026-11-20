@row_set n=1 row=decision/run-start-order cell=rule seen=3 detector=none
value: A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a final verified verdict writes the trust record of an untrusted spawn folder, writes the first snapshot, heartbeat and start events, then spawns the child.
basis: fct-zfntgo5a
because: The start now runs its automatic verify after the endpoint bind and writes the trust record after it on the final verdict, so the row's order no longer holds.
@row_set n=2 row=decision/on-disk-file-naming cell=except seen=1 detector=D-arch-resources
value: ledger/verify.lock, the automatic verify's attempt lock, guards no file of its own name and stands apart from stamps.json.lock.
basis: fct-xgc7qjiw
because: The chunk lands ledger/verify.lock, a lock file named for no guarded file, so the naming rule gains it as its named exception.
@row_set n=3 row=contract/instance-snapshot/cli-verified cell=promise seen=1 detector=none
value: `cli_verified` is the start's final verdict: run's version gate, its stamps read again after the start's automatic verify where one ran.
basis: fct-zfntgo5a
because: The start now records the verdict read after its automatic verify, not the gate's first reading alone.
@row_set n=4 row=decision/run-terminal-child-only cell=except seen=1 fact=F8 detector=none
value: Before the child: the start's five automatic-verify stderr lines, one ASCII write each, no SGR, only the version and a closed word, none under --json, and verify's own inherited output.
basis: fct-zfntgo5a
because: The start now writes its automatic-verify lines and lets verify's output through before the child, an exception the rule must name.
@row_set n=5 row=decision/viola-home-resolution cell=rule seen=1 detector=none
value: The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it.
basis: fct-zfntgo5a
because: The start's verify child gets the home by --home with VIOLA_DIR removed, so the row's only-through-VIOLA_DIR claim must name that child.
@row_add n=1 row=decision/start-verifies-new-cli-version area=established-decisions fact=F1 detector=none
name: Start verifies a new CLI version when keyed
rule: With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind, before the trust record and the claude child, and gates on the final verdict; the key is off by default.
basis: fct-zfntgo5a
because: The chunk adds the start's automatic verify, a decision future start work must respect.
@row_add n=2 row=decision/no-automatic-reverify-of-red area=established-decisions fact=F2 detector=none
name: No automatic verify of an earlier red version
rule: An automatic verify runs only for a version whose stamps entry is short of a relied row; an entry holding every relied row that did not verify is never verified again automatically.
why: So a red version does not loop.
basis: fct-to36e532
because: The chunk decides that a red stamp is never retried by a start, a rule the automatic verify keeps.
@row_add n=3 row=decision/stop-during-start-verify area=established-decisions fact=F3 detector=none
name: A stop during the start's verify ends the start
rule: A SIGINT or SIGTERM during the start's automatic verify TERMs the verify child, waits 30 s, then KILLs it, writes the exit line and re-raises the signal, making no snapshot or session; the verify child is tied to the start by a parent-death SIGTERM.
basis: fct-zfntgo5a
because: The chunk fixes how a stop ends a start whose verify runs, a bound later start and cleanup work relies on.
@row_add n=4 row=decision/automatic-verify-attempt-lock area=occupied-resources fact=F4 detector=D-arch-resources
name: Automatic verify under its own attempt lock
rule: A start runs its automatic verify only while holding the ledger's attempt lock, separate from the stamps writer's lock; a start that finds it held runs no verify and starts unverified.
basis: fct-zfntgo5a
because: The chunk lands a new on-disk lock that keeps two starts from both verifying, a resource and rule the registry must hold.
@row_add n=5 row=name/other/ledger-verify-lock area=occupied-resources fact=F4 detector=D-arch-resources
class: other
name: ledger/verify.lock
basis: fct-xgc7qjiw
because: ledger/verify.lock is a new occupied on-disk name.
@row_add n=6 row=contract/config-json/verify-new-cli area=occupied-resources fact=F5 detector=D-arch-resources
boundary: config.json
item: verify_new_cli
promise: A JSON boolean, default false, turning on the start's automatic verify; any other value reads false with one parse-rejected config-json malformed line, and v must be 1.
basis: fct-xgc7qjiw
because: The chunk adds a config.json key, a new contract item the registry must list beside the other keys.
@row_add n=7 row=contract/diag-spawn-subjects/auto-verify area=occupied-resources fact=F6 detector=D-arch-resources
boundary: diagnostics spawn subjects
item: auto-verify
promise: One process-start and process-exit pair per automatic verify, with child_exit_status and duration_ms, on the run start's span.
basis: fct-xgc7qjiw
because: The closed spawn subject enum gains auto-verify, a new occupied value of the diagnostics line.
@row_none n=1 fact=F7 type=4
relied: The start's verify child runs with the start's strip plan and without VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, stdin null.
@row_none n=2 fact=F9 type=4
relied: The fake agent puts the arguments of a fake-args file beside its executable ahead of argv, argv's own value winning.
