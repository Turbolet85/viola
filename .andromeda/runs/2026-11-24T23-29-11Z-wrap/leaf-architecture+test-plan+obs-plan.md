@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: a refusal exits 1, under `--json` the `{"v":1,"error":"<code>","detail":null}` document (`strict-modes-failed` · `session-live` · `already-live`
source: ctr-yzm3hmbz «`<code>` is one of `strict-modes-failed` · `session-live` · `not-resumed` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch`»
replace-with: a refusal, or a resumed child ending before its session starts (`not-resumed`, its document printed after the child released the terminal), exits 1, under `--json` the `{"v":1,"error":"<code>","detail":null}` document (`strict-modes-failed` · `session-live` · `not-resumed` · `already-live`
