@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=42 class=stale
leaf-text: one INFO line per wrapped `viola run` / `viola revive` start on a verified CLI
source: ctr-qod6cpzw «Each wrapped start of viola run, viola revive and viola adopt on a verified CLI writes exactly one INFO trust-record line to the run role file»
replace-with: one INFO line per wrapped `viola run` / `viola revive` / `viola adopt` start on a verified CLI
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: and from `viola revive`'s preflight `no-session|session-live|cwd-missing`,
source: ctr-2k6eabzj «viola adopt's preflight ends exit 1 with one process-exit detail session-live and no child started; its strict-modes-failed refusal comes before any log and writes no line.»
replace-with: and from `viola revive`'s preflight `no-session|session-live|cwd-missing`, and from `viola adopt`'s preflight `session-live` (one `process-exit` line, no child started),
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=67 class=stale
leaf-text: (`run`'s, a revived start's and `verify`'s home-tree check, exit 1;
source: dec-5uh7yhyl «The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1»
replace-with: (`run`'s, a revived start's, an adopted start's and `verify`'s home-tree check, exit 1;
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=67 class=stale
leaf-text: the `strict-modes-failed` document of `revive`, of `viola open`
source: ctr-4y43s52d «run, revive's start, adopt's start, verify, open and up's home-tree check exit 1 on a strict-modes refusal; revive, adopt, open and up under --json print the strict-modes-failed document.»
replace-with: the `strict-modes-failed` document of `revive`, of `viola adopt`, of `viola open`
@leaf_edit n=5 leaf=.claude/docs/obs-summary.md line=72 class=stale
leaf-text: or on the home tree of `viola run` or `viola revive`'s start arm (exit 1, the home or the instance pair)
source: dec-5uh7yhyl «The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1»
replace-with: or on the home tree of `viola run`, `viola revive`'s start arm or `viola adopt`'s start (exit 1, the home or the instance pair)
@leaf_edit n=6 leaf=.claude/docs/obs-summary.md line=72 class=stale
leaf-text: `viola run` and `viola revive`'s start one `process-exit{subject:"self", exit_code:1}`
source: ctr-a7k5dmkh «the start of run, revive and adopt writes process-exit exit 1, frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP none.»
replace-with: the start of `viola run`, `viola revive` and `viola adopt` one `process-exit{subject:"self", exit_code:1}`
@leaf_edit n=7 leaf=.claude/docs/obs-summary.md line=70 class=omission
leaf-text: and `state-recovered`; a refused scan writes no line.
source: dec-eyetd4mi «The start of viola adopt is not a cli verb and logs as process run into the run log of its name, through run's own lines and spans, adding no event, detail word, field, subject or span of its own.»
replace-with: and `state-recovered`; a refused scan writes no line. The start of `viola adopt` is no `cli` verb: it logs as process `run` into `run-<name>.ndjson` through run's own lines and spans, adding no event, detail word, field, subject or span of its own.
@leaf_edit n=8 leaf=.claude/docs/obs-summary.md line=48 class=omission
leaf-text: boundary-only logging — never in the PTY pump loop
source: dec-a4wnerfa «The session id an operator types to viola adopt never appears in any log line, field pair or detail.»
replace-with: boundary-only logging — never in the PTY pump loop; the session id an operator types to `viola adopt` never appears in any log line, field pair or detail
