
## 2026-10-17-cli-machine-contract — obs: revive --list logs as cli; exemption 6 retired; state-recovered names the scanned instance
**Section:** §1 Obs Scope Summary (cli Exporter, exit-cause bullet); §2 zero-unlogged-panics bullet; §4 cli row, E5; §6 `state-recovered` row, `process-exit` exit 1, Filesystem refusals; §7 Panic hooks (main-thread catch site); §10 bounded exemptions 1, 6, 7; §3 → OTel SDK init (`run` role bullet)
**Change:**
- `viola revive <name> --list` is role `cli` (`role_of`: `revive` then `--list` before a `--`): under a parsed `VIOLA_NAME`, after its strict-modes check passes, `process-start` / `process-exit` (`exit_code` 0, or 1 `detail:"no-session"`) in the caller's `cli-<VIOLA_NAME>.ndjson`; no name, no log; its `Err` or panic reaches the `cli` catch site (was: "opens no process log and writes no line").
- §10: exemption 6 retired, its number kept as a retired marker so 7 and the citations keep theirs; a no-name `--list` is exemption 2; exemption 1's pre-init window and exemption 7's refused check cover both revive arms; 7 states the founder's direction in place (was: "named in exemption 6").
- Revive under `--json`: its refusals print `{"v":1,"error":"<code>","detail":null}`; the logged lines are unchanged; a refused check still writes none.
- `state-recovered` gains `scanned` on the two snapshot details (the instance dir's own last component when it parses as a `ViolaName`, absent otherwise, never a path or literal `null`; none on `torn-line-healed`); E5's line carries `instance` the caller and `scanned` (was: "never the scanned one", owed to "CLI machine contract").
**Why:** the chunk filed `revive --list` under the `cli` role (the overseer, technical: the role's existing rule after the check) and named the scanned instance.
**Kept:** no `cli.revive` dispatch span is named for `--list` — the report does not carry one.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/
