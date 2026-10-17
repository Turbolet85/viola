# A red read during P2: two first starts on a fresh home lose one SessionStart hook

Read 2026-10-17 on the Linux dev host, by /implement P2 (run dir `.andromeda/runs/2026-10-17T11-33-11-implement/`).

## The red

The block's second full firing (entry 16, `bash scripts/agent-run.sh run --e2e`, log `16.2.log`) read
`TIMEOUT [ 20.004s] viola-e2e::mcp_tools mcp_tools_path7_unverified_cli_answer`, the 20 s kill; the same case read
0.58 s alone (`15.4.log`). Its kept home (`target/e2e-home/viola-session-Qr1dhv/home`, the drop never ran) holds
`overseer`'s `wheel` and `budget-gate` records and no `session-start`; `diagnostics/` holds `hook-builder.ndjson` and
no `hook-overseer.ndjson`; both fake agents wrote their `sessions/<pid>.json` registry rows, which the agent writes just
before it fires its launch SessionStart. Boot then waits on `overseer` up to its own 20 s instance deadline, past the
test's 20 s kill.

## Two-sided reproduction

`$TMPDIR/{scratchpad}/repro_unstamped.py`: rounds of concurrent `viola-harness boot --unstamped` sessions from
`target/harness/debug`, each fake agent given `--receipt`, each session cleaned up through the harness.

| input | sessions | failed boots | the failed ones' receipts |
|---|---|---|---|
| `--instance overseer` + `--instance builder`, unstamped | 4 rounds × 6 = 24 | 6 (`readiness-timeout`) | one instance's `hook` SessionStart `ran:false` (its spawn failed), the other `ran:true` exit 0 — 4 times `overseer`, 2 times `builder` |
| `--instance builder` alone, unstamped | 4 rounds × 6 = 24 | 0 | every SessionStart `ran:true` exit 0 |

## Cause, and what is not measured

In an unstamped home no `viola verify` has pinned the binary, so both wrappers' `viola_state::pin::pin_exe` find
`bin/<key>/viola` absent and each replaces it (`replace_private_shared`: a temp file, then a rename); one agent's
hook spawn of that path fails while the other wrapper's replace is in flight. A stamped home is pinned by verify
first, which is why every stamped harness session reads clean. The kernel's error is not read (the fake agent
receipts `ran:false` only); `ETXTBSY` on an exec racing a write handle is the likely one, a HYPOTHESIS.

`pin_exe` is untouched by this chunk: the race predates it, and this chunk's new unstamped two-instance boot is
what reached it. The test now boots Path 7's unstamped session with `builder` alone (Path 7 needs no other
instance). The product race — two first `viola run`s of one version starting together on a fresh home — is
surfaced for the wrap, not fixed here (a product behaviour outside this chunk's intent).

The census after the reproduction found no running process whose executable lies under this repository.
