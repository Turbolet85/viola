@row_set n=1 row=contract/cli-verbs/help cell=promise seen=5 detector=D-design-cli-wording
value: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list and focus, `handoff:` holding link, unlink and `setup:` holding open, run, revive, adopt, up, switch, verify and plugin install.
basis: fct-vocxyji6
because: The `setup:` row of viola --help gains `switch` after `up`, so the row listing the setup verbs must name it.
@row_set n=2 row=contract/viola-help/setup-row cell=promise seen=1 detector=D-design-cli-wording
value: The `setup:` verb group row prints exactly `setup:    open, run, revive, adopt, up, switch, verify, plugin install`, `adopt` after `revive` and `switch` after `up`.
basis: fct-vocxyji6
because: The printed setup row now holds `switch` after `up`, so the exact literal the row states is no longer what viola --help prints.
@row_set n=3 row=contract/frame-verbs/strict-modes-refused cell=promise seen=1 fact=F1 detector=none
value: For send, wait, last, answer, pause, release, link, unlink and switch: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
basis: fct-pmolmu2p
because: viola switch runs the frame verbs' strict-modes check first, so the verbs this refusal is promised for now include switch.
@row_set n=4 row=contract/cli-verbs/run cell=promise seen=2 fact=F6 detector=none
value: Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs, nor at a switch's respawn of the child.
basis: fct-pctfyjeu
because: A switch respawns the wrapped child under the same terminal and viola writes no byte to that terminal at a respawn, so the passthrough promise must cover it.
@row_add n=1 row=contract/cli-verbs/switch area=surface-cli fact=F1 detector=D-layout-surface
boundary: CLI verbs
item: viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]
promise: Switches a running wrapped session to a new model and effort in place, resuming the same session id; at least one of --model and --effort is required, else a usage error, exit 2; it exits 0, 10, 12, 13, 14, 20 or 21 and writes nothing to the user's own settings.
basis: fct-xvxcbpio
because: The chunk adds the viola switch verb, a new CLI surface that no cli-verbs row states.
@row_add n=2 row=contract/viola-switch-output/switched area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola switch output
item: switched
promise: A switch that succeeds prints `<name>  switched  <model> <effort>` on stdout, exit 0.
basis: fct-xvxcbpio
because: The chunk adds a new printed line for viola switch, which no master states.
@row_add n=3 row=contract/viola-switch-output/json-ok area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola switch output
item: --json ok
promise: Under --json a switch that succeeds prints one stdout document whose `ok` carries the `model` and `effort` of the new pair.
basis: fct-xvxcbpio
because: The chunk adds the --json document of viola switch, which no master states.
@row_add n=4 row=decision/switch-refusal-order area=surface-cli fact=F3 detector=none
name: viola switch refusal order
rule: viola switch refuses in a fixed order, the first rung winning: human-typing (exit 10), not-delivered turn-running and not-delivered dialog-pending (exit 13), unverified-cli (exit 12), then not-delivered no-session, child-not-ended or resume-failed (exit 13).
state: provisional
basis: fct-vkgo67d5
because: The chunk refuses a switch mid-turn, with a dialog pending and while the human types, in a fixed order built on the F3 fork ahead of the founder's answer.
@row_add n=5 row=decision/switch-cli-only area=surface-cli fact=F4 detector=none
name: switch is a CLI-only verb
rule: viola switch is a CLI-only verb with no viola mcp tool.
state: provisional
basis: fct-vkgo67d5
because: The chunk builds switch with no MCP tool on the F5 fork, a delegate's word, so the CLI-only choice stands as a provisional decision.
@row_none n=1 fact=F5 type=4
relied: a send while a switch stands is refused not-delivered turn-running until the new child's session start; the send's turn-running refusal rows already state the output, and the switch mark is the wrapper's mechanism.
