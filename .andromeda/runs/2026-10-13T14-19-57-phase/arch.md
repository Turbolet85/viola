# arch extract

## Relevance
partial — the chunk adds tests and behaviour-preserving restatements, not new surfaces, so architecture governs where the tests live (which package and which dependency edges), which test seams are allowed, and the invariants each restatement must keep.

## Constraints
- Each new test lands in the package that owns the mutated file, and its manifest edges stay inside the locked direction. `viola-agent-claude` stays pure (no I/O, no `viola-state` dependency), so a `shell_argv` test is a pure unit test. `viola-pty` depends on no viola crate and knows no agent. `viola-state`'s only extra dev-dependency beyond rstest is tracing-subscriber. Any new dev-dependency is a manifest change this contract governs (per architecture §Infrastructure Patterns → Crate dependency direction).
- No new test seam reaches the binary. The only non-`VIOLA_*` variables any build reads are `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, both under `cfg(feature = "fake-agent")`, and a third would need its own ruling (per architecture §Cross-cutting Patterns, Config management; §Conventions, Environment variables). Test-only code sits behind `cfg(test)`, the code-free `fake-agent = []` features, or `viola-channel`'s `test-support`, which only the root package's `[dev-dependencies]` enables (per architecture §Occupied Resources, Workspace crates).
- Tokio containment holds for test code in the sync crates (`viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`): only `viola-mcp` and `viola-ui` may build a runtime, and the per-crate `deny-sync.toml` ban enforces it. Whether that ban also reaches dev-dependencies is research's question (per architecture §Established Decisions [Concurrency / Backend Framework]; §Cross-cutting Patterns, Tokio containment).
- A restatement of `replace_private_with`'s retry decision must keep the snapshot writer's contract: only a `persist` refused with raw OS error 5 is retried, every 10 ms, re-persisting the same temp file, at most `REPLACE_ATTEMPTS` = 100 attempts (a count, never a time bound); any other error returns at once; the target stays as it was on failure. Making the Windows decision caller-supplied must not change Windows behaviour (per architecture §Established Decisions [Snapshot writer]).
- A restatement or test in `src/run/wheel.rs` (`Recorder`, `Classifier::step` / `abort`, `is_reply`) must keep the classifier's contract: a closed non-editing list matched by exact grammar, a sequence past 64 bytes of parameters or payload takes the wheel, a C0 inside a sequence aborts it, a lone ESC at the end of a read is the Esc key, and the observer returns every byte whole and unchanged. Any error only ever favours the human (per architecture §Established Decisions [Human Takeover / Wheel]).
- `statusline::shell_argv` stays the one shell-out: the user's command is one argument of `/bin/sh -c`, and on Windows it returns `None` for every input (per architecture §Standard Contracts, the hook contract's `hook statusline` arm; §Cross-cutting Patterns, Cross-platform discipline).
- Tests for `viola verify`'s Run B framing driver (`src/cmd/verify/typed.rs`) run against the fake agent. The real `claude` CLI is only for local live runs, and CI runs `verify` against the fake agent's interactive `--screens --turn-stop --trusted-root --dialogs --framing` modes (per architecture §Established Decisions [CLI Version Compatibility]; §Established Decisions [CI/CD]).

## Patterns to follow
- Swap the fake agent in by naming its executable after `--`, with each test in its own `--home`, so parallel tests share no endpoint, log or `budget.json` (per architecture §Established Decisions [CI/CD]; §Cross-cutting Patterns, Config management).
- Real-PTY resize tests wait for the child to observe the new size before reading on (the `size 120x40` watcher pattern). This bears on the `PortablePty::resize` survivor's hand-application (per architecture §Established Decisions [PTY]).
- Package-scoped mutation builds resolve through each sync crate's and `viola-e2e`'s code-free `fake-agent = []` feature. A new test must build under `--package <member> --features fake-agent` (per architecture §Occupied Resources, Workspace crates).
- `viola-state` unit tests capture log lines through the `#[cfg(test)]` tracing-subscriber dev-dependency, not a new product edge (per architecture §Infrastructure Patterns → Crate dependency direction).

## Anti-patterns to avoid
- Starting a live `claude` from any test, or adding a new env-var or feature seam to a product crate to reach a mutant's site (per architecture §Established Decisions [CI/CD]; §Cross-cutting Patterns, Config management).
- Pulling Tokio, or a C-building crate, into a sync crate's test graph (per architecture §Established Decisions [Concurrency / Backend Framework]).
- A "restatement" that changes behaviour on another OS, for example dropping the `cfg!(windows)` retry branch in `fs.rs` or `shell_argv`'s Windows `None` (per architecture §Cross-cutting Patterns, Cross-platform discipline).

## Contract bindings
- arch ↔ tests: the three disposal forms and the mutation reading (`run --mutants`, `mutants.host_excluded`) are test-plan §3 `run` step 4 / §10. Architecture supplies only the scratch and target layout (`target/mutants/`, `<repo parent>/viola-mutants-scratch/`; per architecture §Occupied Resources).
- arch ↔ CI: `windows-mutants.yml` is dispatched only at the epoch-boundary audit, and `tests/contract_windows_mutation_scope.rs` keeps its per-item file lists equal to the Windows-gated sources. A restatement that changes which files are Windows-gated binds that test (per architecture §Infrastructure Patterns → CI/CD approach).
- arch ↔ security: `fs::replace_private` (the one replace helper, mode before bytes) and `shell_argv` (the only shell-out) are security-plan invariants too. A restatement at either site is read against both plans.

## Acceptance criteria contributions
- Every new test sits in the package that owns its mutated file, and no `Cargo.toml` gains a dependency edge outside the locked direction (per architecture §Infrastructure Patterns → Crate dependency direction).
- No new environment variable is read and no new feature is declared; test-only code compiles only under `cfg(test)`, `fake-agent` or `test-support` (per architecture §Cross-cutting Patterns, Config management).
- After any `replace_private_with` restatement, the retry still covers raw OS error 5 only, at 10 ms, for at most `REPLACE_ATTEMPTS` = 100 attempts, and any other error returns at once (per architecture §Established Decisions [Snapshot writer]).
- `windows-mutants.yml` is not dispatched in the chunk, and `tests/contract_windows_mutation_scope.rs` stays green on the chunk's diff (per architecture §Infrastructure Patterns → CI/CD approach).
