@row_add n=1 row=decision/verify-new-cli-key area=surface-cli fact=F1 detector=none
name: Automatic verify of a new CLI version at start
rule: With the `verify_new_cli` config key true (a JSON boolean, default false, any other value false), a `viola run` or `revive` start meeting a claude version with no complete stamp runs `viola verify` once before the child starts; a green stamp lifts the unverified-cli gate, the key never alone.
basis: fct-xgc7qjiw, fct-ktkiytnv, fct-kbaapdaq, fct-gfs4rlxl
because: The chunk adds a config key that makes a start verify an unstamped CLI version, off by default and never lifting the gate itself, and no row of the surface states it.
@row_add n=2 row=contract/auto-verify-line/verifying area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and revive automatic-verify line
item: verifying
promise: A start that runs its automatic verify first prints the fixed verifying line naming the parsed claude version.
basis: fct-zfntgo5a, fct-ktkiytnv, fct-u3yuycke
because: The chunk adds a fixed stderr line a start prints as it begins its automatic verify, a new printed wording the masters do not state.
@row_add n=3 row=contract/auto-verify-line/did-not-verify area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and revive automatic-verify line
item: did not verify
promise: A start whose automatic verify ran and did not leave a green stamp prints the fixed did-not-verify line naming the parsed claude version.
basis: fct-zfntgo5a, fct-jupau6dq
because: The chunk adds a fixed stderr line for a start's verify that ran red, a new printed wording the masters do not state.
@row_add n=4 row=contract/auto-verify-line/earlier-red area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and revive automatic-verify line
item: earlier red
promise: A start whose claude version already has an entry holding every relied row that is not green prints the fixed earlier-red line naming the parsed version and runs no verify.
basis: fct-zfntgo5a, fct-jupau6dq
because: The chunk adds a fixed stderr line for a version an earlier verify left red, a new printed wording the masters do not state.
@row_add n=5 row=contract/auto-verify-line/verifying-elsewhere area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and revive automatic-verify line
item: verifying elsewhere
promise: A start that finds another start holding the verify attempt lock prints the fixed verifying-elsewhere line naming the parsed version, runs no verify and starts unverified.
basis: fct-zfntgo5a, fct-iudxpiry
because: The chunk adds a fixed stderr line for a second start meeting a verify already under way, a new printed wording the masters do not state.
@row_add n=6 row=contract/auto-verify-line/no-stamp area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and revive automatic-verify line
item: no stamp
promise: A start whose automatic verify left no stamp prints the fixed no-stamp line naming the parsed version and one closed word of `spawn-failed`, `unstamped` or `lock-refused`.
basis: fct-zfntgo5a, fct-ghhz5rjy
because: The chunk adds a fixed stderr line with a closed set of three words for a verify that wrote no stamp, a new printed wording the masters do not state.
@row_add n=7 row=contract/auto-verify-line/line-form area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and revive automatic-verify line
item: line form
promise: A start prints exactly one of the five automatic-verify lines, each one stderr write, ASCII, with no SGR, holding only the parsed version and a closed word; none prints under `--json`.
basis: fct-zfntgo5a, fct-u3yuycke, fct-fstjlpgx
because: The chunk fixes the form of the five new start lines and their silence under --json, which no row of the surface states.
@row_add n=8 row=decision/auto-verify-never-loops area=surface-cli fact=F3 detector=none
name: The automatic verify never loops
rule: A start runs at most one automatic verify of a claude version: a version whose entry already holds every relied row and is not green runs none, and a second start while one verify holds the attempt lock runs none and starts unverified.
basis: fct-jupau6dq, fct-iudxpiry, fct-to36e532
because: The chunk bounds the automatic verify so that a red version or a concurrent start never runs it again, a rule future work must keep.
@row_add n=9 row=decision/start-verify-uses-start-terminal area=surface-cli fact=F4 detector=none
name: The start's verify prints on the start's terminal
rule: The verify a `viola run` or `viola revive` start runs before its child prints its own output on the start's terminal, and prints nothing under `viola revive --json`.
basis: fct-zfntgo5a
because: The chunk lets a start's terminal carry verify's own output before the child starts, which the surface's start-output rows do not state.
@row_add n=10 row=decision/stop-during-start-verify-ends-start area=surface-cli fact=F4 detector=none
name: A stop during the start's verify ends the start
rule: SIGINT or SIGTERM during a start's automatic verify terminates the verify (a 30 s grace, then kill) and ends the start by that signal, with no stamp, no snapshot and no session.
basis: fct-zfntgo5a, fct-gookuxzl
because: The chunk makes a stop during the start's verify end the whole start with nothing left behind, which no row of the surface states.
@row_none n=1 fact=F5 type=4
relied: the start's trust record is written on the final verdict after the automatic verify, which runs after the socket bind
