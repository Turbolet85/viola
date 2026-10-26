# Task — architecture · 2026-10-25-windows-endpoint-admission

## The session
The session added Windows-only integration tests showing that viola's named-pipe listener rejects remote clients (a read-back of the pipe-type bit, checked against an accept-remote and a reject-remote control pipe) and that its DACL denies an anonymous-token open (checked against a default-descriptor control pipe that lets the same open in).
Both are now asserted on CI's `test (windows-2025)` leg (ci#40247500584, green). The existing DACL text read-back, non-inheritance, first-instance and `squatted-name` cases pass there unedited.
It added a root-package Windows dev-dependency table, `windows-sys` with two features, `Wdk_Storage_FileSystem` and `Win32_System_IO`. `Cargo.lock` is unchanged, and no release build carries the features.
It routed the root bin's nine scoped tracing captures in test code through a new `#[cfg(test)]` helper, `test_support::with_scoped_default` in `src/main.rs`, with a witness test. The one global subscriber install is untouched.
No product file, behaviour, wire contract, exit code, refusal word or output line changed. No crate or dependency version was added or bumped, and nothing was retired.
The full CI row is red only on `supply-chain`, from the advisory RUSTSEC-2026-0386 against the unchanged tokio 1.53.1. That red now belongs to a new route entry, "tokio RUSTSEC-2026-0386 lock bump", placed right after "Windows endpoint admission".

## Reached files
- tests/security_negatives_channel.rs
- Cargo.toml
- src/main.rs
- src/run/send/test_support.rs
- src/run/dialog.rs
- src/run/budget.rs
- src/run/gate.rs
- src/obs.rs
- src/cmd/run.rs
- src/cmd/hook/statusline.rs

## Touches
- The IPC endpoints test-only line (res-ipc-endpoints) names `tests/security_negatives_channel.rs` as a creator of `viola-test-chan-<pid>-<label>` endpoints. The claim is re-read and still true: the five new cases' pipes, labelled `remote`, `accept-remote`, `reject-remote`, `anon` and `anon-default`, sit in that namespace and are never bound under `viola-<h12>`. Some of them are test-owned `CreateNamedPipeW` pipes that `viola` never binds.
- The Workspace crates and test-only feature items (Occupied Resources) say the root package's `[dev-dependencies]` enables `viola-channel` `test-support` only. That still holds. The root package now also has a separate `[target.'cfg(windows)'.dev-dependencies]` table.
- Module Boundaries (dec-module-boundaries): `Cargo.toml` was edited only by the new dev-dependency table. Workspace members, the flat `crates/` layout and the rule that only `viola-mcp` enables `tokio` are unchanged.
- Concurrency KEYSTONE (dec-concurrency-backend-framework) and the Tokio containment rule (cross-cutting): no Tokio was added. `cargo deny check bans licenses sources` and `scripts/channel-tokio-free.sh` are re-read green.
- Stack (stack): the `windows-sys` pin `=0.61.2` is unchanged. The two dev-dependency features already reached the Windows graph by unification (from mio 1.2.4, interprocess 2.4.4 and socket2 0.6.5). Under `resolver = "3"` a dev-dependency feature reaches no release build.
- The tokio version named in the stack and in Inherited Defaults (1.53.1) is unchanged, but it now fails CI `supply-chain` on RUSTSEC-2026-0386, owned by the route entry "tokio RUSTSEC-2026-0386 lock bump" (a bump to `>=1.53.3`, no ignore widened).
- The Message Broker / IPC decision (dec-message-broker-ipc) and the Local endpoint trust boundary (cross-cutting) say no other OS user may connect to an endpoint. The Windows pipe's remote-client rejection and its DACL's effect are now witnessed on CI. Before this they were asserted only by the DACL text read-back.
- Any architecture text that says remote pipe clients or the Windows pipe DACL cannot be driven on hosted runners is now stale for those two (measured false on `test (windows-2025)`). The cross-euid peer and second-OS-account parts still stand.
- Any architecture text that names the route entry "Windows endpoint admission" as pending, or as the owner of these Windows listener assertions, is affected: the chunk landed them. Nothing is owed to "Windows-only live measurements" for them, because the controls tell the bits apart.
- Route-entry order cited in established decisions (dec-links): «"Explicit session links", after "Windows endpoint admission" and "Server verification before any frame"». It still holds after the new tokio entry is inserted.
- The CI/CD decision: the new cases run inside the existing `test (windows-2025)` instrumented nextest run (the `coverage` suite). No CI step, script, harness command or status shape changed. The JUnit artifact `junit-windows-2025` (from `target/nextest/ci/junit.xml`) carries them: 2988 tests, 0 failures.
- The Design Philosophy line that Windows live behaviour stays proven on the CI runner under the fake agent: the Windows leg now carries two more listener admission witnesses. There is still no interactive Windows host.
- The Root bin role (Module list, and the Error Handling decision's anyhow-only-in-bin): `src/main.rs` gains only a `#[cfg(test)]` `test_support` module. The panic hook and the DLL-search restriction keep their statement order in `main`.
- The Diagnostic output channels rule (cross-cutting): every captured test line still goes through `obs_event!`. No product log line, role file or stdout byte moved. `src/run/gate.rs`'s one `set_global_default` is untouched.
- Infrastructure Patterns → Crate dependency direction (a registry key under `.andromeda/registries/architecture-contracts.toml`, not in the outline's files): this is the plan's named home for the dev-dependency fact. A classifier of an outline file that restates a dependency-direction rule should flag it there too.

## New facts
- F1 · occupied-resources · The root package declares `[target.'cfg(windows)'.dev-dependencies] windows-sys = { workspace = true, features = ["Wdk_Storage_FileSystem", "Win32_System_IO"] }`. It is test-only, used by `tests/security_negatives_channel.rs`. Both features are needed because windows-sys 0.61.2 also gates `NtQueryInformationFile` on `Win32_System_IO` for its `IO_STATUS_BLOCK` argument. `Cargo.lock` does not move, and no release build carries the features.
- F2 · stack-and-technologies · The pinned tokio 1.53.1 carries the `unsound` advisory RUSTSEC-2026-0386. It fails the CI `supply-chain` job on every run since the last wrap. The owner is the route entry "tokio RUSTSEC-2026-0386 lock bump", placed right after "Windows endpoint admission": a lock bump to `>=1.53.3`, with no ignore-list widening.
- F3 · cross-cutting-patterns · On Windows the endpoint trust boundary is now asserted on CI `test (windows-2025)`. The viola listener's pipe reads `FILE_PIPE_REJECT_REMOTE_CLIENTS` set, and an anonymous-token `GENERIC_READ` open of it is denied with `ERROR_ACCESS_DENIED`. Each assertion is checked against test-owned control pipes that tell the outcomes apart.

## Untouched
- `RefusalReason` and its details (conv-refusal-reason): no refusal word or detail moved. The type lives in `viola-core`, which was not edited.
- The MCP results convention and the MCP decision (conv-mcp-results, dec-mcp): `viola-mcp` and its tools are unchanged.
- The capability ledger and CLI version compatibility (crosscut-capability-ledger, dec-cli-version-compatibility): no ledger row, probe, stamp or relied-set count moved, and `viola-agent-claude` was not edited.
- The snapshot envelope, ledger stamps envelope and session liveness contracts (contract-snapshot-envelope, contract-ledger-stamps, contract-session-liveness): `viola-state` is unchanged.
- The `list` payload contract (contract-list-payload): `src/cmd/list.rs` and the board read model are unchanged.
- The Claude Code integration names (res-claude-integration): `plugin/` and the registered hooks are unchanged.
- The environment variable registry (res-env-vars): the new Windows tests and the capture helper read no environment variable, and no seam was added.
- The pinned `bin/` copy and the ConPTY companion filesystem entries, and the vendored ConPTY binaries: `src/conpty.rs` and `vendor/conpty/` were not edited.
- The `fuzz/` workspace and its lockfile: unchanged.
- The crate-level scoped captures in `viola-state` and `viola-channel`: they share the capture mechanism but were left out with no owner. This is a test-code matter, not an architecture fact.
