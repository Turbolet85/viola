@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=42 class=stale
leaf-text: `viola open` writes one `process-start` / `process-exit{subject:"terminal"}` pair around the foot terminal window it spawns, into the opened name's `cli-<name>.ndjson`.
source: ctr-yt4sqped «Subject of the one process-start, with no process-exit, of each foot terminal window that viola open or viola up spawns, written into that verb's cli role file; the terminal outlives the verb.»
replace-with: `viola open` and `viola up` write one `process-start{subject:"terminal"}`, with no `process-exit`, for each foot terminal window they spawn, into that verb's `cli` role file; the terminal outlives the verb.
@leaf_edit n=2 leaf=.claude/rules/observability.md line=39 class=omission
leaf-text: `viola focus` logs as `cli` into the named instance's `cli-<name>.ndjson`: a `process-start` and a closed-detail `process-exit` inside the `cli.focus` dispatch span, which carries no fields.
source: ctr-ya32ovel «Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson only under a parsed VIOLA_NAME, after its strict board scan, inside the cli.up dispatch span: its own process-start and process-exit, one process-start{su»
replace-with: `viola focus` logs as `cli` into the named instance's `cli-<name>.ndjson`: a `process-start` and a closed-detail `process-exit` inside the `cli.focus` dispatch span, which carries no fields. `viola up` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` only under a parsed `VIOLA_NAME`, after its strict board scan, inside the `cli.up` dispatch span: its own `process-start` and `process-exit`, one `process-start{subject:"terminal"}` per spawn and `state-recovered`; a refused scan writes no line.
