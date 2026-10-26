# Implement outcome — 2026-10-26-server-verification-before-any-frame

**Outcome: green + smoke.** Next → the operator pass (plan entries 22 hygiene, 23 push, 24 CI read), fired by the
implementer on the operator's word at invocation («please run implement per the skill on
2026-10-26-server-verification-before-any-frame, through its operator pass and the CI read»), then
`/andromeda-wrap-session`.

## What was built
Every frame-writing client checks, before its first frame, that the process serving the endpoint is the wrapper
`snapshot.json` records — the connection's server pid (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows
`GetNamedPipeServerProcessId` on the SQOS-adopted client end) and that pid's OS start time against `pid` /
`started_at`. On a mismatch or an unmakeable check no frame is written: the six CLI verbs exit 21 with the unchanged
`instance-unreachable` document (or the `unable` line and the new hint `the process on <name>'s endpoint is not its
recorded wrapper; it was not contacted`) and log `process-exit{exit_code:21, detail:"server-verify-failed",
during:"connect"}`; `viola hook` fails open (exit 0, nothing printed) with `hook-decision{detail:"server-verify-failed"}`,
a SessionEnd taking its direct append; MCP's four frame tools return `instance-unreachable`. macOS keeps the recorded gap
(no peer pid: the check matches with no comparison).

## Files changed (18)
- `crates/viola-channel/src/client.rs` — `Client::server_pid`, `SERVER_PID_REPORTED`, `server_pid_of`; unit cases (step 1)
- `crates/viola-channel/src/tokio_client.rs` — `TokioClient::server_pid`; unit cases (step 2)
- `crates/viola-channel/src/lib.rs` — the re-export (companion, recorded)
- `crates/viola-state/src/liveness.rs` — `LiveEndpoint` from `live_endpoint`'s one read, `ServerVerdict`, `server_verdict`
  (pure), `verify_server` (joined); the decision table and the joined case (step 3)
- `src/cmd/client.rs` — `Unreachable` (closed cause), `CallError`, `connect` (the verified connection), `answer_of`,
  `unreachable`, `unable_unreachable`; unit cases; the shared captured-span check `refused_inside` (step 4)
- `src/human/hints.rs` — the `server-verify` hint (step 5)
- `src/cmd/{send,wait,last,answer,pause,release}.rs` — each `request` through `client::connect`, inside its dispatch span;
  one captured-span case each (steps 6–9)
- `src/cmd/hook.rs` — `NotSent`, `connect`, `deliver` / `ask` verified, `Asked::ServerUnverified`; the `served` fixture
  records its real start time; unit cases (step 10)
- `crates/viola-mcp/src/tools.rs` — `frame` verified, the start-time read on `spawn_blocking` (step 11)
- `crates/viola-mcp/tests/handlers.rs` — the impostor case, the held-child entry (step 12)
- `tests/security_negatives_server_identity.rs` (new) — six verbs × two modes and their controls (step 13)
- `tests/hook_fail_open.rs` — three impostor rows (step 14)
- `tests/contract_fake_agent_drift.rs` — the ETXTBSY fix (in-intent, recorded, on the overseer's word)

## Gates (block run 3 of 3, trail `.andromeda/runs/2026-10-26T08-22-16-implement/gate-2026-10-26-server-verification-before-any-frame.json`)
`entries 24 · green 19 · red 1 (13) · recorded 1 · timeout 0 · not-run 3` — every non-leg entry green but the guard,
whose one outside file was accepted as named (`inputs#I5`); entries 22–24 are `leg = 'operator'`.
- 7 `run --unit` green · 8 `binary(security_negatives_server_identity)` green · 9 `binary(hook_fail_open)` green ·
  10 `binary(handlers)` green · 11 `cargo deny check` green · 12 `channel-tokio-free.sh` green · 3/4 clippy (host and
  the Windows target) green · 20 `run --perf` green, hook rows max 2.2–2.4 ms over 30 runs each (P5: 1.23–1.6 ms; the
  spine bound 1 s) · 21 `pre-push` green at `linux-tests`, 88.21 s · 1 `/tmp` tmpfs 25G avail, 22 %.
- Red-before-green: the check neutralised, 25 new cases red then 7 more (the span cases), controls green, restored and
  re-read (`chunks/…/evidence/red-before-green.md`).

**The guard's lines, entry 13 (copied whole from its log):**

    outside tests/contract_fake_agent_drift.rs (recorded in-intent · serves the pre-push gate (entry 21): its ETXTBSY red, met during this chunk and folded into it)
    guard: allowed 16 — crates/viola-channel/src/client.rs, crates/viola-channel/src/tokio_client.rs, crates/viola-mcp/src/tools.rs, crates/viola-mcp/tests/handlers.rs, crates/viola-state/src/liveness.rs, src/cmd/answer.rs, src/cmd/client.rs, src/cmd/hook.rs, src/cmd/last.rs, src/cmd/pause.rs, src/cmd/release.rs, src/cmd/send.rs, src/cmd/wait.rs, src/human/hints.rs, tests/hook_fail_open.rs, tests/security_negatives_server_identity.rs · companions excluded 1 — crates/viola-channel/src/lib.rs (serves step 1) · changed 18 · excluded 0
    guard: outside 1 files

The one outside file is `tests/contract_fake_agent_drift.rs`, accepted as named by the overseer's delegated answer
(`inputs#I5`); every other changed file is allowed or an excused companion.

## Smoke
✓ — the P2 smoke entries (boot · status · cleanup) green in all three runs; by hand a booted session, `viola last` and
`viola wait --json` at the real wrapper (exit 0, one exchange each), cleanup with `processes_gone` and `endpoint_gone`
true (`evidence/smoke-p3.md`).

## The operator's check — must-trace paths
Confirmed: every new pre-frame refusal opens its scenario's root span (`send.client`, `wait.block`, `last.client`,
`answer.client`, `pause.client`, `release.client`, `hook.handle`; MCP inside `mcp.tool_call`) and writes its outcome
line; no `channel.request` span or `channel-request` line, since no frame is sent. Measured in process by seven new
captured-span cases (red with the check removed) and on disk by the integration rows (`evidence/must-trace-check.md`).
One obs-plan text no longer holds and is the wrap's to amend: §4 `wait` / `last`'s «`server-verify-failed` cannot occur …
until client-side server verification lands».

## Deviations from the plan, with justifications
1. **`tests/contract_fake_agent_drift.rs` (scope record, in-intent, `inputs#I5`)** — block run 1's pre-push red was
   ETXTBSY in the agents drift test: concurrent threads each copied the fake agent (the tmpfs home defeats the hard
   link) and one thread's forked child held another's write handle. 6 of 40 standalone runs red before, 0 of 40 after
   planting every set's bin before the threads start (`evidence/etxtbsy-drift-test.md`). Not this chunk's code; folded
   by the standing rule on the overseer's delegated word.
2. **`crates/viola-channel/src/lib.rs` (scope record, companion)** — re-exports `SERVER_PID_REPORTED` for step 1.
3. **Step 10's hook unit impostor** varies the start time (this process recorded a second early, and an unparseable
   `started_at`) instead of recording another live pid: a root-bin unit test has no portable second process. The
   other-live-pid shape is the integration rows' (steps 13, 14) with the held fake agent.
4. **Steps 13 and 14's held process** is the fake agent's own holder mode `--hold-stdout-internal` (it sleeps 10 s,
   writes nothing, killed and reaped in a `Drop` guard); every case finishes well inside that.
5. **Step 14's rows** are `cfg_attr(not(target_os = "macos"), case::…)` rows in the two existing matrices, `Env::Impostor`
   gated the same way.
6. **Added beyond the plan, on the operator's check:** the seven captured-span cases and the shared
   `cmd::client::refused_inside` helper (in listed files), each red with the check removed.
7. **Block run 2's red was this run's own:** the new helper module's first gate `#[cfg(all(test, not(macos)))]` put
   its `cfg!(windows)` ahead of the file's first `#[cfg(test)]`, so `contract_windows_mutation_scope` read
   `src/cmd/client.rs` as Windows-gated product code; stacked `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`.

## Surfaced (for the wrap's amendments, none authored here)
The plan's expected amendments stand as listed (security-plan's dated gaps losing their server-identity half on Linux and
Windows; architecture's readers list and `process-exit` / `hook-decision` detail; obs-plan §4 `wait` / `last` and §6;
test-plan §5 / §6; design-system cli-send / cli-exit-codes and layout-templates block 2). CARRY 2's Windows exiting-wrapper
window is unchanged and unmeasured, as planned. No matrix capability is claimed by the chunk (`matrix.py show`: 0).

## Process hygiene
`gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --comm claude --args projects/viola/target`:
17 running, none started by this run — every row is another tree's (the overseer sessions and their `claude`s, the
prototype's and the product's `viola`, `~/.viola/bin/…/viola mcp`), none under this repository's `target/`.

| process | started by | final state |
|---|---|---|
| harness session `p-svf-smoke` (supervisor, wrapper, fake agent) ×3 | a leg (block entries 15, 19) | terminated — cleanup `processes_gone` / `endpoint_gone` true |
| harness session `p-svf-p3` | this run (P3 by hand) | terminated — cleanup `processes_gone` / `endpoint_gone` true |
| held fake agents (`--hold-stdout-internal`) in the new tests | the tests | terminated — killed and reaped by each `Drop` guard |
| `cargo nextest` probe loops (80 standalone runs) | this run | terminated — each exited |

## Decisions & corrections
- The overseer's delegated answer (founder asleep, "continue and fix problems at once"), 2026-10-26 08:39Z: fold the
  ETXTBSY fix in, show the 40-run loop at 0 red, the guard's outside line accepted as named — `inputs#I5`.
- Guard refusals: a `cd` out of the cwd (re-issued with an absolute path) and a `cat` heredoc to a file (re-issued as a
  python heredoc), one re-issue each.
- Stamp hook: twice a time was written by estimate (the journal's run-1 line, the ETXTBSY evidence's loop times) and
  refused as ahead of the clock; each was re-read (`date -u`, the logs' mtimes) and corrected.
- Recurrence of the deferred handoff learning on `cfg!(windows)` (deviation 7).
- Testing learning 2026-10-16 (copy an oracle from the first run's output) recurred: a hand-typed sorted key list, red
  on its first run.
