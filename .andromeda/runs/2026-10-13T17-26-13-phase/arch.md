# arch extract

## Relevance
partial — the chunk is test and restatement work inside the root `viola` bin's `src/cmd/verify/typed.rs`. Architecture governs where the code and any test seam live (the root package, the `pty` seam, crate direction) and pins `viola verify`'s Run B behaviour, which must not move. The mutation-disposal forms and the ledger are test-plan's.

## Constraints
- Workspace placement: `viola verify`'s interactive PTY runs live in the root `viola` bin (`src/cmd/verify/typed.rs`), and a kill's test belongs to that package. No product crate may depend on the test-only `viola-e2e`. Per architecture §Established Decisions → [Module Boundaries].
- The `pty` seam (spawn · read · write · resize · wait · kill) over portable-pty `=0.8.1` is viola's own boundary. The chunk may add a `Pty` double behind `SharedPty` / `Run` without touching `viola-pty`'s trait. If research finds no other route and the trait must change, that crate must still depend on no viola crate and know no agent. Per architecture §Infrastructure Patterns → Crate dependency direction and per architecture §Established Decisions → [PTY].
- Run B's behaviour is fixed by the plan, and a restatement must preserve it exactly:
  - a direct spawn under the R8 strip at 80×24, with output capped at `MAX_FRAME`;
  - the four compiled pastes in order, each added one only into input-box rows that hold no modal literal;
  - `box_wait`'s end conditions;
  - the Ctrl-C, Ctrl-C after 500 ms, then kill at `PROBE_DEADLINE` ending;
  - every wait bounded by `PROBE_DEADLINE`.

  The seventeen-row `check(row, &Probes)` outcome must not change either. Per architecture §Established Decisions → [CLI Version Compatibility].
- Capability-ledger discipline: the chunk adds no ledger row, probe text, deadline or order. `viola-agent-claude::ledger` stays the only home of the rows and the compiled paste texts, and the root bin stays a consumer. Per architecture §Cross-cutting Patterns → Capability ledger as the single gate for CLI-specific behaviour.
- Spawn discipline: any PTY child a test starts is spawned directly, never through `sh`, `bash` or `cmd`. Every OS-specific branch must compile and be tested on its CI runner. Per architecture §Cross-cutting Patterns → Cross-platform discipline.
- Tokio containment: `verify` is on the sync path, so a test double or a framing-wait restatement uses std threads and blocking I/O and no runtime. Per architecture §Established Decisions → [Concurrency / Backend Framework].
- No new environment-variable seam: the only non-`VIOLA_*` variables any build reads are `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, under `cfg(feature = "fake-agent")`. A test seam the chunk adds is code behind `cfg(test)` or an existing test feature. Per architecture §Cross-cutting Patterns → Config management.

## Patterns to follow
- Use the injected-clock pattern. Deadlines are read against an injected clock, so unit cases can reach a bound without wall time; `wait` re-reads the injected clock, and the readiness gate's bound refusal is held on the injected clock in unit cases only. `framing_turns`' `clock.now() + PROBE_DEADLINE` is the same shape. Whether `typed.rs`'s clock is already injectable is research's question. Per architecture §Established Decisions → [CLI Version Compatibility] and per architecture §Standard Contracts (the parked `wait`).
- Use the passed-in decision restatement. A behaviour-preserving restatement passes the decision in as a parameter so that a unit case on every host exercises the branch. The precedent is `retry_replace(_, HOST_IS_WINDOWS)`, from chunk 2026-10-13-epoch-4-survivors. Per architecture §Established Decisions → [Snapshot writer].
- Use the fake agent as the CI stand-in. CI runs `verify` against the fake agent's interactive `--screens --turn-stop --trusted-root --dialogs --framing` modes, and the real `claude` probe runs only locally. A non-`claude` PTY child, if research finds one within the test rules, follows that precedent. Per architecture §Established Decisions → [CLI Version Compatibility] and per architecture §Established Decisions → [CI/CD].

## Anti-patterns to avoid
- Do not reach the sites by changing the probe's program, order, deadlines or texts, or by moving a Claude-specific shape out of `viola-agent-claude`. Per architecture §Inherited Defaults (Agent isolation) and per architecture §Established Decisions → [CLI Version Compatibility].
- Do not give `viola-pty` a viola-crate or agent dependency. Do not put `tokio` in the root bin's sync paths. Per architecture §Infrastructure Patterns → Crate dependency direction.
- Do not let a test-built `Run` create a probe directory under a recorded name in the repository cwd (`<cwd>/.viola-verify-<pid>/` and its siblings), or create any other unrecorded resource. A new test-only path is an Occupied Resources amendment at wrap. Per architecture §Occupied Resources.

## Contract bindings
- arch ↔ tests: the root package's inline `mod tests` in `typed.rs` is the owning-package test site that test-plan §10's Mutation gate disposal forms name. The fake agent's interactive modes are the CI substitute for the real CLI (architecture §Established Decisions → [CI/CD]).
- arch ↔ security: the spawn rules bind any PTY child a test starts (direct spawn, no `.cmd`/`.bat` child, the R8 strip). So do the 0700 probe-directory and drop-guard rules (architecture §Occupied Resources; security-plan, `viola verify`'s four interactive PTY children).
- arch ↔ obs: verify's `verify-pty-probe` `process-start` / `process-exit` pair wraps each interactive run. A restatement keeps one pair per run (architecture §Occupied Resources).

## Acceptance criteria contributions
- Every change and test lives in the root `viola` package under `src/cmd/verify/`. `crates/viola-pty`'s `Pty` trait is unchanged, unless research recorded that no other route exists. Per architecture §Established Decisions → [Module Boundaries].
- No new dependency edge or Cargo feature appears in any manifest. `viola-pty` still depends on no viola crate, and no `tokio` enters the root bin. Per architecture §Infrastructure Patterns → Crate dependency direction.
- No new environment-variable read in any build. A test-only seam compiles only under `cfg(test)` or an existing test feature. Per architecture §Cross-cutting Patterns → Config management.
- `viola verify` against the fake agent still checks and stamps the same seventeen rows with the same per-row outcome. Run B's paste order, its texts, `PROBE_DEADLINE` and its Ctrl-C/kill ending are unchanged. Per architecture §Established Decisions → [CLI Version Compatibility].
