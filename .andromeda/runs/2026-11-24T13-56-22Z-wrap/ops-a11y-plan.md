@row_set n=1 row=contract/frame-verb-refusal/strict-modes-failed cell=promise seen=1 detector=none
value: send, wait, last, answer, pause, release and switch refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.
basis: fct-pmolmu2p
because: The new `viola switch` verb runs the strict-modes refusal first, before any frame, so the closed verb list this contract names must carry it.
@row_set n=2 row=contract/frame-verb-refusal/server-verify-failed cell=promise seen=1 detector=none
value: send, wait, last, answer, pause, release, link, unlink and switch refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty.
basis: fct-pmolmu2p
because: `viola switch` is served only after the peer and server checks of the client connect and exits 21 on them, so the verb list this contract names must carry it.
@row_set n=3 row=contract/cli-exit-codes/10 cell=promise seen=1 fact=F5 detector=none
value: human-typing: the human holds the wheel, so the automated send or switch is refused with nothing typed and no child replaced.
basis: fct-xvxcbpio
because: `viola switch` is now refused `human-typing` with exit 10 while the human types, so the exit-10 promise no longer covers send alone.
@row_set n=4 row=decision/send-refused-while-human-typing cell=rule seen=1 fact=F5 detector=none
value: When the human takes the wheel, `send` and `switch` are refused with `human-typing` on stderr and a typed exit code.
basis: fct-xvxcbpio
because: The switch verb is refused `human-typing` while the human types, a decision the row states for send alone.
@row_add n=1 row=decision/respawn-reap-bound area=cognitive-accessibility fact=F1 detector=none
name: A switch's reap wait is bounded at 10 s
rule: A `viola switch` waits at most 10 s (RESPAWN_REAP_BOUND) for the old child to exit after its hang-up; a child not exited is not replaced, and the switch is refused not-delivered child-not-ended, exit 13. The bound falls on the wrapper, never on the human.
state: provisional
basis: fct-vocxyji6
because: The chunk adds a new time bound, ratified only on a delegate's word (fork F6), and the plan owes it a row of its own beside the other bounded waits.
@row_add n=2 row=decision/switch-refusal-order area=cognitive-accessibility fact=F2 detector=none
name: Switch refusals, first rung wins
rule: `viola switch` refuses on the first rung that holds: human-typing (exit 10), not-delivered turn-running or dialog-pending (exit 13), unverified-cli (exit 12), then not-delivered no-session, child-not-ended or resume-failed (exit 13); each cause has its own hint, none naming release.
state: provisional
basis: fct-xvxcbpio
because: The new verb brings a refusal set and order of its own, built on a delegate's word (fork F3), that no standing row states.
@row_add n=3 row=contract/viola-switch-output/result-line area=1-a11y-scope-summary fact=F3 detector=none
boundary: viola switch output
item: result line
promise: `viola switch` prints one line, `<name>  switched  <model> <effort>`, on stdout in plain ASCII with no SGR.
basis: fct-pctfyjeu
because: The new verb's human result line is an output-discipline promise the plan holds verb by verb, and none stands for switch.
@row_add n=4 row=contract/viola-switch-output/json-output area=1-a11y-scope-summary fact=F3 detector=none
boundary: viola switch output
item: --json output
promise: Under `--json`, `viola switch` prints one document, `{"ok":{"model","effort"}}`.
basis: fct-xvxcbpio
because: The new verb's `--json` result document is an output promise the plan holds verb by verb, and none stands for switch.
@row_add n=5 row=decision/switch-respawn-same-terminal area=1-a11y-scope-summary fact=F4 detector=none
name: A switch respawns under the same host terminal
rule: At a `viola switch` the wrapper replaces its child under the same host terminal, in a new PTY pair at the host's current size; viola writes no byte of its own to the terminal, and human keystrokes keep flowing across the respawn.
state: provisional
basis: fct-przrtlcy
because: The TUI boundary now spans a child replaced in place, built on a delegate's word (fork F1), and the standing passthrough rows hold no clause for the respawn within their bound.
@row_add n=6 row=decision/send-refused-while-switching area=4-aria-patterns-roles fact=F5 detector=none
name: Send refused while a switch stands
rule: While a `viola switch` stands, from its request until the new child's session-start, a send is refused not-delivered turn-running.
state: provisional
basis: fct-ionzjh33
because: A send is now held off for the span of a switch, a decision on a delegate's word (fork F3) that no standing row states.
@row_none n=1 fact=F6 type=4
what: The verb's argument check (at least one of --model and --effort, a bad value a clap usage error with exit 2) is how one verb is built; the standing exit-2 usage-error contract already covers it, and the code holds the rest.
