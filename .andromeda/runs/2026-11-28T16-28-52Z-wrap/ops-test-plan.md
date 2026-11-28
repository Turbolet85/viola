@row_set n=1 row=decision/tui-verdicts-from-records cell=rule seen=2 detector=none
value: TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status; the fleet console's verdicts come from its console.ndjson lines, its terminal's line mode and the records of the verbs it runs, never from the screen.
basis: fct-ewvlmjfa, fct-dxzom25q, fct-ucszbfnd
because: The console's action cases now read their verdicts also from the records the verbs it runs leave (the send's events, the hook's decision body and dialog-answered line, the switch's launch line), so the rule naming console.ndjson and the line mode alone is incomplete.
@row_add n=1 row=contract/console-log-line/console-action area=log-format fact=F1 detector=D-tests-obs-harness
boundary: console log line
item: console-action
promise: Written once per finished action child, carrying only verb, a word of the diag-line schema's closed verb enum, and exit_code, both required; no name, argument or output.
basis: fct-vfoixk62, fct-twmcx6cs, fct-outvcy4j
because: The chunk adds the console-action event to the log format the harness greps, and test-plan's log-format holds no row of what that line promises.
@row_add n=2 row=contract/console-log-line/console-key-editing-line area=log-format fact=F2 detector=D-tests-obs-harness
boundary: console log line
item: console-key on the editing line
promise: One key read on the editing line writes at most one console-key line per class (text, backspace, unbound), each with a count of 1 or more, never a line per character and never the typed text; a text word requires count.
basis: fct-vfoixk62, fct-zcv5bkss, fct-netg2s3x, fct-ctp7s6d6
because: The console-key line gains a count and a per-read fold on the editing line, a log-format promise test-plan's log-format holds no row of.
@row_add n=3 row=decision/console-actions-proven-live area=e2e-test-strategy fact=F5 detector=none
name: Real console actions proven only by the live reading v2-15
rule: The fleet console's actions are proven only over the fakes; real focus and real terminals, a real claude's replies and dialogs, the editing line under a real input method and the key line at the operator's tile width are proven only by the gated live reading of v2-15.
state: provisional
basis: fct-um6swzva, fct-3d7suckd
because: The chunk proves its actions over the fakes alone and leaves their real halves to the gated v2-15 reading, a gate no E2E row of test-plan states for the console.
@row_none n=1 fact=F3 type=4
what: How the console's two answer E2E cases raise and read their dialog; the test file holds it, and decision/hook-e2e-recorded-stdin already states that hook verbs run as processes over recorded stdin.
@row_none n=2 fact=F4 type=4
what: How one E2E case reaches a refused action; the test file holds it, and contract/list-json/liveness-stale already states that a killed wrapper is omitted as gone.
