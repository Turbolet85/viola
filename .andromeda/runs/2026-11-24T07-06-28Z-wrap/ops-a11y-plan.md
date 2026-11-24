@row_add n=1 row=contract/viola-up-output/instance-line area=1-a11y-scope-summary fact=F1 detector=none
boundary: viola up output
item: instance line
promise: `viola up` prints one line per instance, `<name>  <word>` or `<name>  live  <placement>`, plain ASCII, with no SGR, each line in one write.
basis: fct-ykd4mawd
because: The chunk adds the `viola up` verb, whose per-instance line is a new human output surface the plan's output discipline must name.
@row_add n=2 row=contract/viola-up-output/line-order area=1-a11y-scope-summary fact=F1 detector=none
boundary: viola up output
item: line order
promise: Every line that is not a spawn comes first, in name order; then the spawned or `would-open` lines follow in spawn order.
basis: fct-tpkehh2e
because: The chunk fixes the linear order of the `viola up` lines, a structure a reader relies on.
@row_add n=3 row=contract/viola-up-output/outcome-word area=cognitive-accessibility fact=F2 detector=none
boundary: viola up output
item: outcome word
promise: Each instance's outcome word is one of the closed set `live`, `already-live`, `stale-heartbeat`, `would-open`, `no-session`, `cwd-missing`, `session-live`, `not-live`, `not-taken`.
basis: fct-ykd4mawd
because: The chunk adds `viola up` with a closed set of outcome words printed to the reader.
@row_add n=4 row=contract/viola-up-output/exit area=cognitive-accessibility fact=F2 detector=none
boundary: viola up output
item: exit code
promise: `viola up` exits 0 when every line reads `live`, `already-live`, `stale-heartbeat` or `would-open`, else 1.
basis: fct-ykd4mawd
because: The chunk adds `viola up`, whose exit code is read from its outcome words.
@row_add n=5 row=contract/viola-up-output/json area=cognitive-accessibility fact=F3 detector=none
boundary: viola up output
item: --json document
promise: Under `--json`, `viola up` prints one `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}` document and no hint.
basis: fct-ykd4mawd
because: The chunk adds the `viola up --json` document, a machine-output promise beside the human lines.
@row_add n=6 row=contract/viola-up-output/not-live-hint area=cognitive-accessibility fact=F4 detector=none
boundary: viola up output
item: not-live hint
promise: After all its lines, `viola up` prints one stderr `hint:` line when any line reads `not-live`, and none under `--json`; no other `up` line carries a hint.
basis: fct-ykd4mawd
because: The chunk adds the one recovery hint `viola up` gives, which the plan's word-detail-hint discipline must name.
@row_add n=7 row=contract/viola-up-refusal/boot-time-unread area=cognitive-accessibility fact=F5 detector=none
boundary: viola up refusals
item: boot-time-unread
promise: When the host boot time cannot be read, `viola up` refuses with exit 1: its own `unable`/`hint` pair, or under `--json` one document.
basis: fct-wivrj76k
because: The chunk adds `boot-time-unread` as a new refusal code word with its own fixed pair and document.
@row_add n=8 row=contract/viola-up-refusal/exit-1-documents area=cognitive-accessibility fact=F6 detector=none
boundary: viola up refusals
item: exit 1
promise: `viola up` refuses with exit 1 and a document for `boot-time-unread`, `strict-modes-failed`, `pinned-hash-mismatch` and `terminal-not-found`.
basis: fct-ykd4mawd
because: The chunk adds the refusals of `viola up` and the exit code each one carries.
@row_set n=1 row=contract/cli-exit-codes/21 cell=promise seen=2 fact=F6 detector=none
value: instance-unreachable: a frame verb, viola focus before any compositor read, or viola up on the board's strict-modes refusal, was refused before reaching the instance, the cause in detail.
basis: fct-ykd4mawd
because: `viola up` now exits 21 with no line on the board's strict refusal, so the exit-21 promise names it beside the frame verbs and focus.
