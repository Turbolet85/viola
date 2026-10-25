# obs-plan · SLO Invariants & Telemetry Budgets
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its SLO Invariants & Telemetry Budgets detail.

## 10. SLO Invariants & Telemetry Budgets

<!-- detail: exception · slo-gates -->

- **Zero unlogged panics:** every panic in `run`, `hook`, `mcp` and `cli` **after init step 4** produces **exactly one** single-line `level:"ERROR", event:"panic"` record in its home-level role file, with no multi-line backtrace in home-level files, and `hook` still exits 0. Retry-once policies for panic handling are forbidden, in parallel with the tests' `retries = 0`.
  - **Counting rule:** "exactly one", the `viola.panic` counter and gate G2 (§9) count home-level role files only. G2 alone also leaves out the `fake-agent` seam's deliberate panic (a `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>`, §9): that line is still written exactly once — the invariant holds for it — and only its count is exempt. The companion line in `detail-<process>.ndjson` also carries `event:"panic"` and is excluded. In merged `agent-run logs` output, count `select(.record.event=="panic" and (.file|startswith("detail-")|not))`.
  - **Bounded exemptions** (no file exists, so no line can be written):
    1. the pre-init window before step 4 (§7: clap parse and home resolution, the instance check of both `viola revive` arms (the start arm and `--list`, whose check runs before its `cli` role file opens) and of `viola hook statusline`, and `viola list`'s strict-modes scan, which runs before its `cli` role file opens, and `viola mcp`'s check of its home and `diagnostics/`, which runs before its `mcp` role file opens, chunk 2026-10-17-mcp-server-for-drivers, and `viola plugin install`'s check of its home and `marketplace/` tree dirs, which runs before its `cli` role file opens);
    2. `cli` without a resolved instance (D-06);
    3. `hook` without `VIOLA_NAME` (D-16);
    4. a failed step-4 role-file open (§3 init step 4). The writer becomes `std::io::sink`, or stderr JSON for `mcp` (D-09), and the panic hook's `OnceLock` stays empty, so it writes nothing, not even to `mcp`'s stderr (D-32).
    5. the hidden capture arms, `hook <event> --capture` and `hook statusline --capture` (which runs no instance check and no pass-through), each doing no obs init and so having no role file; a panic there writes no line and the hook still exits 0, as in exemption 3.
    6. (retired, chunk 2026-10-17-cli-machine-contract: `viola revive <name> --list` opens the `cli` role log under a `VIOLA_NAME` once its check has passed, so a panic there writes its one line; with no `VIOLA_NAME` it is exemption 2, and its refused check is exemption 7. The number is kept so the others keep theirs.) `viola revive`'s start arm is not exempt once its instance check has passed: its role file is then `run-<name>.ndjson`.
    7. a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline` (chunk 2026-10-10-budget-governor), or a refused `viola list` scan of the home, `instances/` or an instance (chunk 2026-10-11-the-board-viola-list, the operator's answer at its planning), or `viola mcp`'s refused check of its home or `diagnostics/` through `viola_state::strict::check_path` (chunk 2026-10-17-mcp-server-for-drivers, its plan's lean, approved by the operator at P5), or `viola plugin install`'s refused check of its home or a `marketplace/` tree dir through `check_path` (chunk 2026-10-17-optional-plugin-install): it opens no role file and writes no line (revive: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1; statusline: exit 0 and nothing printed; list and plugin install: exit 21, the `--json` error document or the stderr pair; mcp: exit 1 with nothing on stdout or stderr), so a panic there writes none, by §7's rule; no forced-panic case reads it. That a refused check writes no line on `viola hook statusline` and on `viola list` is the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's answer at those chunks' planning, superseded 2026-10-13).

    The failure condition "a panic evidenced by an exit code without its line" applies only to processes whose `process-start` is in a home-level role file. For the exemptions the only evidence is the exit status. `hook` still exits 0, so a D-16 panic is unobservable by design (E1).

<!-- /detail -->

<!-- detail: values · slo-budgets -->

| Path / operation | p50 | p95 | Budget assertion |
|------------------|-----|-----|------------------|
| `viola hook session-end` | report-only (§5 snippet over `duration_ms`) | report-only | hyperfine `max < 1.0 s` in `target/agent-run/artifacts/perf-session-end.json`, judged by `gate --require perf` (`.results[0].max < 1.0`; the row file is required by name) |
| Spine hooks (`session-start`, `user-prompt-submit`, `stop`) and `pre-tool-use` on an unverified CLI | report-only | report-only | hyperfine `max <` spine deadline in `target/agent-run/artifacts/perf-<hook>.json`, judged by `gate --require perf` (each spine row and the `pre-tool-use` row required by name; the perf session boots unstamped, so `pre-tool-use` runs on an unverified CLI). The deadline value is tests-owned (provisionally 1.0 s; the hook's own provisional 750 ms connect deadline sits below it); obs requires only that the max sample is gated |
| Heartbeat staleness flip | n/a | n/a | 4.9 s → `live`, 5.1 s → `stale`/`gone`, read as `list --json` `liveness` |
| Obs overhead on `hook` | n/a | n/a | logger init = 1 `open` + appends, inside the same `max < 1.0 s` gate. This holds only if every hyperfine hook command runs with `VIOLA_NAME` set and a home under `target/e2e-home/`; the instance fixture is tests-owned. Without `VIOLA_NAME` it times the D-16 no-op (no open, no channel call), and the gate passes vacuously. The perf job runs G2 over that home, because a hook panic exits 0 and hyperfine cannot see it (D-30) |

<!-- /detail -->
