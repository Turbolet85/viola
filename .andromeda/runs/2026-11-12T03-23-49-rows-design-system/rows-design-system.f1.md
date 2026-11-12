@row_add n=1 row=decision/cli-ascii-only area=typography rec=1,2,3
name: ASCII-only CLI output
rule: CLI output is ASCII only, separators included, at high density.
@row_add n=2 row=contract/cli-colour-tokens/attention area=surface-cli rec=4,5,6,7
boundary: CLI colour tokens
item: attention
promise: Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
@row_add n=3 row=contract/cli-colour-tokens/reset area=surface-cli rec=4,8
boundary: CLI colour tokens
item: reset
promise: `\x1b[0m` at every colour depth, after every styled span.
@row_add n=4 row=contract/cli-colour-depth/16-colour area=surface-cli rec=9,10
boundary: CLI colour depth
item: 16-colour
promise: Where no earlier depth matches, the depth is the 16-colour fallback column; the depth is chosen once per process, first match wins.
@row_add n=5 row=absence/cli-tape area=surface-cli rec=11,12,13
capability: A tape in the CLI
@row_add n=6 row=decision/no-hint-for-faults area=surface-cli rec=14,15
name: No hint for faults and unknown
rule: `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`.
why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
@row_add n=7 row=contract/viola-verify-output/step-line area=surface-cli rec=16,17
boundary: viola verify output
item: step line
promise: One static appended uncoloured stdout line per relied ledger row, ending `pass` or `fail`, counted against `MM`, the relied count.
@row_add n=8 row=decision/start-refusals-show-no-path-or-pid area=surface-cli rec=18,15
name: Start refusals show no path or pid
rule: The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
@row_add n=9 row=contract/viola-revive-output/json-refusal area=surface-cli rec=19,20,21,22
boundary: viola revive output
item: --json refusal
promise: Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
@row_add n=10 row=contract/cli-exit-codes/0 area=surface-cli rec=23,24
boundary: CLI exit codes
item: 0
promise: No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
@row_add n=11 row=contract/cli-exit-codes/1 area=surface-cli rec=23,22
boundary: CLI exit codes
item: 1
promise: `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
@row_add n=12 row=contract/cli-exit-codes/2 area=surface-cli rec=23,25
boundary: CLI exit codes
item: 2
promise: Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`) or `UNUSABLE_TEXT` line (unusable `viola send`) on stderr, whatever `--json` says; no document.
@row_add n=13 row=contract/cli-exit-codes/20 area=surface-cli rec=23,26
boundary: CLI exit codes
item: 20
promise: `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
@row_add n=14 row=contract/cli-exit-codes/21 area=surface-cli rec=23,27
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
@row_add n=15 row=absence/rendered-page area=border-radius rec=28,29,30,31
capability: A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's
