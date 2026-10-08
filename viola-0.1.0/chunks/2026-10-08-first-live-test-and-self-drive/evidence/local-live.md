# `run --local-live`, fired once on the dev host (2026-10-08, 07:27Z)

**Outcome: green. The stamp reads `2.1.287`, 17 `pass`, 0 `fail`.** Five live starts (1 to 5 of 8).

Fired once through `gate.py run --live-legs` (the round's listing: `evidence/round-072838Z.txt`, verdict
`round: COMPLETE · legs fired 1/1`). The liveness probe before it printed `2.1.287 (Claude Code)` by path.

The live entry, as the plan lists it: the prototype's `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` cleared, the
2.1.287 install dir first on `PATH`, `bash scripts/agent-run.sh run --local-live`, from the repository root.
Exit 0 in 50.28 s.

## The firing's document

```
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"local-live","passed":1,"failed":0,"skipped":0,"survived":0,"artifact":null,"failures":[]}]}
```

Its step lines, one per ledger row, each ending `pass`: `shim-resolution`, `spine-hooks`,
`session-start-fields`, `prompt-verbatim`, `stop-message`, `largest-hook-payload`, `modal-signature`,
`input-box-signature`, `quiet-period`, `confirm-window`, `question-answer`, `plan-approve-revise`,
`question-notes`, `dialog-concurrency`, `long-paste-wrapper`, `tag-escaping`, `local-command-clear`. Its last
line before the document: `stamped 2.1.287  17 pass  0 fail`.

## The stamp, read from the home

`target/e2e-home/viola-live-4043089/home/ledger/stamps.json` (mode 0600, writer `verify`, written
07:27:51.144Z): one version, `2.1.287`; 17 rows, 17 `pass`, 0 other. This home is `H`: start 6 read it and came
up `cli_verified` true.

## The census

| when | `.viola-verify-*` dirs at the root | `viola-live-*` dirs | probe-cwd `claude` processes |
|---|---|---|---|
| before (07:26:55Z) | 11 (the operator desk's, from 2026-10-06 and before) | 0 | 0 |
| after (07:28:32Z) | 11, the same names | 1, the firing's own | 0 |

`own_left` 0: no probe dir and no probe process of the round was left. Nothing removes the `viola-live-*` home;
it is on the tmpfs behind the `target/e2e-home` link and is gone at a reboot.

The own compositor stood through the round (alive by pid and start time at 07:28:32Z).
