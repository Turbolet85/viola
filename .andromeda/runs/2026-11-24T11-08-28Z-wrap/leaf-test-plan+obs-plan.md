@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=21 class=omission
leaf-text: Both `viola revive` arms (the start arm and `--list`) run their instance check before the log init as well
source: dec-4vztpgai «Commands acting on an instance (revive's start arm, adopt's start, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus) run the instance strict-modes check before opening any log»
replace-with: Both `viola revive` arms (the start arm and `--list`) and `viola adopt`'s start run their instance check before the log init as well
@leaf_edit n=2 leaf=.claude/rules/observability.md line=21 class=omission
leaf-text: `viola run` and `viola revive`'s start arm (after its instance check) run the home-tree check
source: dec-5uh7yhyl «The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens»
replace-with: `viola run`, `viola revive`'s start arm and `viola adopt`'s start (after their instance check) run the home-tree check
@leaf_edit n=3 leaf=.claude/rules/observability.md line=21 class=omission
leaf-text: under `viola revive --json` the `strict-modes-failed` document
source: ctr-4y43s52d «run, revive's start, adopt's start, verify, open and up's home-tree check exit 1 on a strict-modes refusal; revive, adopt, open and up under --json print the strict-modes-failed document.»
replace-with: under `viola revive --json` or `viola adopt --json` the `strict-modes-failed` document
@leaf_edit n=4 leaf=.claude/rules/observability.md line=28 class=omission
leaf-text: `detail` codes are closed kebab-case lists — never a path or pid.
source: dec-a4wnerfa «The session id an operator types to viola adopt never appears in any log line, field pair or detail.»
replace-with: `detail` codes are closed kebab-case lists — never a path or pid. The session id an operator types to `viola adopt` never appears in any line, field or detail.
@leaf_edit n=5 leaf=.claude/rules/observability.md line=38 class=stale
leaf-text: Each wrapped start of `viola run` and `viola revive` on a verified CLI
source: ctr-qod6cpzw «Each wrapped start of viola run, viola revive and viola adopt on a verified CLI writes exactly one INFO trust-record line to the run role file»
replace-with: Each wrapped start of `viola run`, `viola revive` and `viola adopt` on a verified CLI
@leaf_edit n=6 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: `viola revive`'s start arm logs as process `run`
source: dec-eyetd4mi «The start of viola adopt is not a cli verb and logs as process run into the run log of its name, through run's own lines and spans, adding no event, detail word, field, subject or span of its own.»
replace-with: `viola adopt`'s start is no `cli` verb: it logs as process `run` into `run-<name>.ndjson` through run's own lines and spans, adding no event, detail word, field, subject or span of its own. `viola revive`'s start arm logs as process `run`
@leaf_edit n=7 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: `cwd-missing` — one `process-exit{exit_code:1, detail}`
source: ctr-2k6eabzj «viola adopt's preflight ends exit 1 with one process-exit detail session-live and no child started; its strict-modes-failed refusal comes before any log and writes no line.»
replace-with: `cwd-missing` — one `process-exit{exit_code:1, detail}`; `viola adopt`'s preflight likewise ends one `process-exit{exit_code:1, detail:"session-live"}` with no child started
@leaf_edit n=8 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: its `session-live` reading, skipped under `--fork` and passing on a failed read
source: ctr-a5pthapf «Subject of the process-start and process-exit pair of the claude agents read, written by viola list, the session-live reading of viola revive and viola adopt, verify's Run B and the list tool of viola mcp»
replace-with: its `session-live` reading (and `viola adopt`'s start's, alike), skipped under revive's `--fork` and passing on a failed read
@leaf_edit n=9 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: A refused instance check on either `viola revive` arm (start or `--list`)
source: ctr-jomkpi4e «A refused check at run, viola revive, viola adopt, viola verify, viola focus, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.»
replace-with: A refused instance check on either `viola revive` arm (start or `--list`) or on `viola adopt`'s start
@leaf_edit n=10 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: on the home tree of `viola run` or `viola revive`'s start arm
source: dec-5uh7yhyl «The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.»
replace-with: on the home tree of `viola run`, `viola revive`'s start arm or `viola adopt`'s start
@leaf_edit n=11 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: `viola run` and `viola revive`'s start (`socket_dir_ready`
source: ctr-a7k5dmkh «the start of run, revive and adopt writes process-exit exit 1, frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP none.»
replace-with: `viola run`, `viola revive`'s start and `viola adopt`'s start (`socket_dir_ready`
@leaf_edit n=12 leaf=.claude/rules/observability.md line=41 class=stale
leaf-text: The lineage line and the launch line written at every run and revive start
source: dec-3umgztdt «The launch line written at every run, revive and adopt start is log-only product audit data: it wakes no wait and opens no span, writes no process-log line and no terminal byte of its own.»
replace-with: The lineage line and the launch line written at every run, revive and adopt start
