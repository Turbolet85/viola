# Implement outcome — 2026-10-26-server-verification-before-any-frame (re-entry)

**Outcome: green + smoke; operator pass complete, CI green.** The re-entry for the wrap's P7.1 red (light gate entry 21:
one cut-short `viola-fake-agent` coverage profile refused by the merge), folded into this chunk on the operator's word
(`inputs#I13`), run on his word at invocation (`inputs#I14`). Operator pass: pre-push green, hygiene clean twice, fix
commit `7948850a` (the two fix files only), push entry 23 from a clean worktree of it (`inputs#I15`), CI
`verdict: green · checks 15/15`, ci#40318429755, `run_attempt` 1 (`chunks/…/evidence/operator-pass.md` §7). Next →
`/andromeda-wrap-session` resuming from `.andromeda/runs/2026-10-26T09-14-57-wrap/` at P7 step 0. The wrap's
uncommitted P1–P6 writes were not touched (never staged, never stashed).

## What was built
The fake agent, from its exit's first step (`\x03` or stdin's end, before it waits for a running hook), ignores SIGHUP
(`ignore_hangup`, Unix). It is its PTY's session leader (portable-pty's `setsid` + `TIOCSCTTY`), so a hangup of that
terminal — a killed wrapper closing the PTY, or portable-pty's SIGHUP-first `kill` — reaches it, and one landing mid its
exit-time coverage write leaves a cut-short profile. A hangup before the exit begins still ends it as before. Test-only
(`fake-agent`); no product code changed.

## Files changed by this run (2)
- `src/bin/viola-fake-agent.rs` — `ignore_hangup`, called at both of `read_stdin`'s exits (in-intent, recorded,
  `inputs#I13`)
- `tests/cli_fake_agent.rs` — `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` and its two Linux helpers
  (in-intent, recorded, `inputs#I13`)

Records written: `chunks/…/scope-record.md` (two lines), `chunks/…/evidence/fake-agent-exit-hangup.md`, `inputs#I14`,
`inputs#I15`, this run dir.

## Gates (block run 1 of 1, trail `.andromeda/runs/2026-10-26T11-36-36-implement/gate-2026-10-26-server-verification-before-any-frame.json`)
`entries 24 · green 19 · red 1 (13) · recorded 1 · timeout 0 · not-run 3` — every non-leg entry green but the guard;
22–24 are `leg = 'operator'`.
- 7 `run --unit` green · 8 `binary(security_negatives_server_identity)` · 9 `binary(hook_fail_open)` · 10
  `binary(handlers)` green · 11 `cargo deny check` · 12 `channel-tokio-free.sh` green · 2–6 fmt, clippy (host and the
  Windows target), G1, lint probes green · 14–19 the smoke and G2 / G4 green · 20 `run --perf` green, hook rows max
  2.2–2.5 ms over 30 runs each (the release build cached, 1.86 s) · 21 `pre-push` green at `linux-tests`, 91.35 s · 1
  `/tmp` tmpfs 25G avail, 22 %.
- Red-before-green: `ignore_hangup` neutralised (`SIG_DFL`), the new case red («the agent exited … before the exit
  ignoring SIGHUP»); restored, `binary(cli_fake_agent)` 43 passed; 40/40 stress.
- Profile censuses on the instrumented build entry 21 left: the Epoch 5 kill-shaped case 200 runs, 801 profiles, 0
  refused; the new case 40 runs, 120 profiles (3 a run, the agent's whole after a SIGHUP inside its exit), 0 refused
  (`chunks/…/evidence/fake-agent-exit-hangup.md`).

**The guard's lines, entry 13 (copied whole from its log):**

    outside src/bin/viola-fake-agent.rs (recorded in-intent · serves the pre-push gate (entry 21): its coverage-merge red (a cut-short `viola-fake-agent` profile), met at the wrap's light gate and folded into this chunk)
    outside tests/cli_fake_agent.rs (recorded in-intent · serves the pre-push gate (entry 21): the red-before-green case of the fake-agent exit fix beside the fake agent's other process cases)
    outside tests/contract_fake_agent_drift.rs (recorded in-intent · serves the pre-push gate (entry 21): its ETXTBSY red, met during this chunk and folded into it)
    guard: allowed 16 — crates/viola-channel/src/client.rs, crates/viola-channel/src/tokio_client.rs, crates/viola-mcp/src/tools.rs, crates/viola-mcp/tests/handlers.rs, crates/viola-state/src/liveness.rs, src/cmd/answer.rs, src/cmd/client.rs, src/cmd/hook.rs, src/cmd/last.rs, src/cmd/pause.rs, src/cmd/release.rs, src/cmd/send.rs, src/cmd/wait.rs, src/human/hints.rs, tests/hook_fail_open.rs, tests/security_negatives_server_identity.rs · companions excluded 1 — crates/viola-channel/src/lib.rs (serves step 1) · changed 20 · excluded 0
    guard: outside 3 files

The three outside files are the folded reds' (the first run's ETXTBSY fix, `inputs#I5`; this run's two, `inputs#I13`),
each recorded in-intent; the plan's allow list cannot name them — the plan's-wording arm for the wrap's ruling.

## Smoke
✓ — recorded from P2, not re-run: entries 15 boot, 16 status, 19 cleanup green on the fixed fake agent; cleanup's Ctrl-C
drives the new exit path, the agent exiting by itself (`killed: []`, `processes_gone` and `endpoint_gone` true).

## Scope
`scope: clean — changed 20 · listed 16 · recorded 4 (companion 1 · mechanical 0 · in-intent 3 · widening 0) · absorbed 0 · excluded 1015`

## Deviations from the plan, with justifications
1. **`src/bin/viola-fake-agent.rs` and `tests/cli_fake_agent.rs` (scope record, in-intent, `inputs#I13`)** — the
   folded red's fix and its test, outside research's lists, as a folded red is.
2. **The mechanism differs from the directive's wording.** `inputs#I13` names the fix as the fake agent's own exit
   «taken out of the PTY's foreground group». The agent is its session's leader, which cannot change its process group
   (`setpgid` refuses a leader), so its exit is taken out of the hangup's reach instead: SIGHUP ignored from the exit's
   first step. Same failure closed (a SIGHUP inside the exit-time write); the census and the case show it.
3. **The `--exit-no-eof` holder** is spawned after `ignore_hangup` and inherits SIGHUP ignored; it already leaves the
   group, and no test signals it with SIGHUP.
4. **The operator pass's fix commit holds only the two fix files**, and its push ran from a worktree (`inputs#I15`):
   the pass commits the whole tree, but the operator asked the wrap's writes left uncommitted (`inputs#I14`). The
   worktree was taken on `build/viola-0.1.0` with `--force` (a detached one leaves `git push origin HEAD` no branch);
   nothing was committed in it, and it was removed after the push. This run's own records (the scope record's two
   lines, the evidence, `inputs#I14`–`I15`, this run dir) ride the wrap's commit.

## Surfaced (for the wrap's amendments, none authored here)
- test-plan §7 "Fake agent" and `.claude/rules/verification-harness.md`'s fake-agent bullet: the exit now ignores SIGHUP
  from its first step (beside the hooks' own process groups); the new case
  `tests/cli_fake_agent.rs::fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` (Linux only).
- The red's original writer stays unnamed: never reproduced on demand (`basis-loop.md`); the fix closes the mechanism
  the Epoch 5 cause names for the agent itself.

## Process hygiene
`gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --args projects/viola/target`: 6 running,
none started by this run and none under this repository's `target/` — the prototype bridge's four `viola run`s, the
installed `~/.viola/bin/…/viola mcp`, the product's `viola wait viola-builder`.

| process | started by | final state |
|---|---|---|
| harness session `p-svf-smoke` (supervisor, wrapper, fake agent) | a leg (block entries 15, 19) | terminated — cleanup `processes_gone` / `endpoint_gone` true |
| the new case's fake agents and their `statusline-echo` hooks (red, green, 40 stress, 40 census runs) | the tests | terminated — each exited and was reaped by the test |
| `profraw-census.sh` loops (240 runs) | this run | terminated — both exited 0 |

## Decisions & corrections
- The operator's word at invocation, 2026-10-26 11:36Z: the re-entry, the operator pass and the CI read, the wrap's
  P1–P6 writes left as they are — `inputs#I14`.
- The overseer's answer (founder-delegated, relayed through the operator's question answer), 11:46Z: entry 23 fired as
  written from a clean worktree of the fix commit, the worktree removed after — `inputs#I15`.
- Guard refusals: two `cd`s out of the cwd (the registry dir, the references dir), each re-issued once with absolute
  paths.
