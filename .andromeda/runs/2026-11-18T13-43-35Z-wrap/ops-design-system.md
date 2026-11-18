@row_add n=1 row=decision/start-trust-step-prints-nothing area=surface-cli fact=F1 detector=none
name: Start trust step adds no output
rule: The trust step of a `viola run` or `viola revive` start on a verified CLI prints nothing, puts no viola byte on the terminal, never refuses the start, and moves no exit code, hint or `--json` code.
basis: fct-xro6zomc
because: The start now writes the CLI's trust record before the child starts, and the CLI surface must state that this step is silent and changes no refusal, exit code or document.
@row_add n=2 row=contract/trust-record-line/detail area=surface fact=F3 detector=none
boundary: the trust-record log line
item: detail
promise: Each `viola run` or `viola revive` start writes one `trust-record` log line whose `detail` is one of 16 fixed start words, carrying no path, key or value.
basis: fct-xro6zomc
because: A new log event with its own closed set of detail words is promised outward at the log boundary, beside the standing log detail code rows.
@row_none n=1 fact=F2 type=4
relied: `viola revive` keys the trust record it writes by the session's recorded cwd, and no record is written for the user's home folder, a trusted folder or parent, or an unverified CLI.
