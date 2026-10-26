# arch · cascade read · group 1 of 2

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:15) The Windows listener must keep its pipe DACL converted from SDDL by windows-sys, one named pipe per wrapper, which is the admission the chunk asserts on the Windows leg.
  > The server's pipe DACL is converted from SDDL by windows-sys (`Win32_Security_Authorization`); on Unix, libc gives the uid for the per-user socket directory and the socket is chmod 0600 after the bind | One endpoint per `viola run`: a named pipe on Windows, a Unix domain socket elsewhere.
- (.andromeda/architecture-detail/established-decisions.md:60) The listener's remote rejection, first-instance and non-inheritable properties must stay interprocess 2.4.4 behaviour held behind viola's own `channel` seam, with hashed endpoint names, never a raw name or path.
  > **[Message Broker / IPC] interprocess 2.4.4 `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam.**
  (.andromeda/architecture-detail/established-decisions.md:61)
  > Endpoint names are a short hash of the instance, never the raw `VIOLA_NAME` + path.
  (.andromeda/architecture.md:39)
  > endpoint names are a short hash of the instance; on Windows the client connects only through the SQOS Identification open, never interprocess's default connect.
  (.andromeda/architecture.md:129)
  > **Endpoint name:** `viola-<h>`, where `<h>` is the first 12 hex characters of an FNV-1a 64-bit hash over `ViolaName` + `\0` + the absolute viola home path. It is hand-written in `viola-channel` and recorded in the instance snapshot.
- (.andromeda/architecture.md:444) The IPC default the chunk hardens is interprocess local sockets with one endpoint per wrapper.
  > - IPC: interprocess 2.4.4 local sockets, one endpoint per `viola run`, JSON-RPC 2.0 over ndjson with `v` + sender version in every `params`.
- (.andromeda/architecture-detail/stack-and-technologies.md:11) The channel server code the chunk edits must stay std-thread and blocking, with Tokio built only inside `mcp`.
  > Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.1 built only inside `mcp` | No async runtime on the hot `hook` path.
- (.andromeda/architecture-detail/established-decisions.md:8) Any change to `viola-channel` must keep its normal graph without the `tokio` feature proven tokio-free by the supply-chain step's script.
  > CI enforces this with a sole-root `cargo deny` `tokio` ban (`deny-sync.toml`, run once per crate listed in `scripts/sync-crates.txt`), evaluated for the Windows, macOS and Linux target triples, plus a `cargo check` of the listed sync crates on all three OSes (CI job 3). `viola-channel`, whose optional `tokio` feature `viola-mcp` enables, is not on that list (since chunk 2026-10-17-mcp-server-for-drivers): its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh` in the same CI supply-chain step.
  (.andromeda/architecture.md:23)
  > **[Concurrency / Backend Framework] Hybrid sync core + Tokio only in `mcp` (KEYSTONE).** CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on all three OSes.
- (.andromeda/architecture-detail/established-decisions.md:190) New `viola-channel` tests must build under the crate's package-scoped test build, whose test-only `fake-agent` feature enables `tokio`, while no other product member's normal graph reaches it.
  > Among the product crates only `viola-mcp` enables `tokio` (`rt`); `viola-channel` lists it optional behind its `tokio` feature (`["dep:tokio", "interprocess/tokio"]`, the `TokioClient`), which `viola-mcp` enables and, in a package-scoped test build, the channel's own test-only `fake-agent = ["tokio"]` (chunk 2026-10-17-mcp-server-for-drivers);
- (.andromeda/architecture-detail/stack-and-technologies.md:21) The budget test's log-capture fix, and any channel unit-test line capture, must read the process log through tracing-subscriber, a dev-dependency only for the library crates.
  > tracing-subscriber 0.3.23 (default features off, `fmt,json,registry,std`; the root bin is its only product dependent, and `viola-channel` and `viola-state` take it as a dev-dependency for their unit tests' line capture);
- (.andromeda/architecture-detail/stack-and-technologies.md:24) Any error the listener's new checks raise must be a typed thiserror error, with anyhow kept to the bin edge.
  > thiserror 2.0.20 (per crate; `viola-pty`'s `PtyError` is hand-written); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains built at dispatch and recorded only in the owner-only instance detail file
- (.andromeda/architecture-detail/established-decisions.md:184) A listener error added in `viola-channel` must fold into its one enum `ChannelError`.
  > **[Error Handling] thiserror 2.0.20, one enum per workspace crate
  (.andromeda/architecture.md:106)
  > **Rust error types:** one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`)
- (.andromeda/architecture-detail/established-decisions.md:176) The exclusive bind must keep refusing a taken name as the exit-1 `squatted-name` start refusal before the first snapshot, and the chunk asserts the same first-instance property on the listener itself.
  > exclusive bind of the endpoint (a bind that finds the name taken is an exit-1 refusal, `squatted-name`, so two concurrent starts cannot both win), first snapshot (with `endpoint`,
  (.andromeda/architecture-detail/established-decisions.md:175)
  > A `viola run` whose `ViolaName` is `live` or `stale` refuses to start (exit 1, with a message naming the instance); once the endpoint exists, a name with an answering endpoint refuses the same way.
  (.andromeda/architecture.md:156)
  > `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
- (.andromeda/architecture-detail/conventions.md:23) The re-verified squat refusal must keep exit 1, detail `squatted-name` and its fixed stderr pair with no path, pid or pipe name.
  > and `viola run` losing the exclusive endpoint bind to another holder (`detail:"squatted-name"`, `unable: the endpoint for <name> is held by another process` / `hint: another process holds this name's endpoint; stop it or pick another name`, no path, pid or pipe name);
  (.andromeda/architecture.md:100)
  > **CLI exit codes:** `0` ok · `1` internal error, and the start, revive-preflight and verify refusals ·
- (.andromeda/architecture-detail/established-decisions.md:211) The `squatted-name` refusal must still print no `--json` document; its document stays owed to "Exit-cause code catalogue".
  > A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".
  (.andromeda/architecture.md:227)
  > A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".
- (.andromeda/architecture-detail/stack-and-technologies.md:31) The chunk's Windows assertions must be measured on the `windows-2025` native runner against the fake agent; the local pre-push is a filter, never the verdict.
  > GitHub Actions matrix `windows-2025`, `macos-latest` (macOS 26), `ubuntu-latest`; toolchains installed by `rustup toolchain install`: from `rust-toolchain.toml`, from `fuzz/rust-toolchain.toml` in the fuzz jobs, and `1.96` in the `msrv` job (no toolchain action); SHA-pinned actions/checkout 7.0.1, Swatinem/rust-cache 2.9.2, taiki-e/install-action 2.87.19, actions/upload-artifact 7.0.1 | Build, lint and test on all three OSes against the fake agent, on native runners.
- (.andromeda/architecture.md:7) A Windows admission property no CI runner can measure stays owed, since Windows live behaviour is proven on the CI runner until an interactive host exists.
  > Windows live behaviour stays proven on the CI runner under the fake agent until an interactive Windows host exists.
- (.andromeda/architecture.md:451) The CI default the chunk's evidence rests on is the three-OS matrix against the fake agent.
  > - CI: GitHub Actions on windows-2025 / macos-latest / ubuntu-latest against the fake agent.
- (.andromeda/architecture-detail/established-decisions.md:131) The budget-paused send test the chunk re-runs or fixes must keep asserting the gate's crossing rule, `five-hour` named when both windows cross.
  > As landed (chunk 2026-10-10-budget-governor; `src/run/budget.rs`, one `BudgetGate` per wrapper, no thread and no timer): a window is crossed only when its `used_percentage` is known and at or above its threshold and its `resets_at` is known, parses as RFC 3339 and is later than now, so a window whose `resets_at` is `"unknown"`, unparseable or passed never pauses. With both windows crossed the refusal names `five-hour`.
  (.andromeda/architecture-detail/established-decisions.md:132)
  > The `send` point sits after the text refusals and the first `human-typing` read and before the running-turn read, so the order is `control-character` → `empty-text` → `human-typing` → `budget-paused` → `turn-running` → the readiness gate's wait;
  (.andromeda/architecture.md:53)
  > **[Budget Governor] Soft stop at the turn boundary.** Once a configured threshold is crossed (defaults `five_hour` ≥ 90 %, `seven_day` ≥ 85 %), automated `send` is refused `budget-paused`;
- (.andromeda/architecture-detail/standard-contracts.md:47) The fixed test must read the refusal `budget-paused` with its window detail before `send-issued`, no cursor and nothing typed, and its one `send-refused` record.
  > A `send` reads the wrapper's budget gate once at its arrival ([Budget Governor]); a closed gate is the refusal `budget-paused` with detail `five-hour` or `seven-day`, before `send-issued`, with no cursor, nothing typed and no flight reserved, and a gate change that cannot be recorded there answers `-32603`.
  (.andromeda/architecture.md:180)
  > A closed budget gate refuses it `budget-paused` before `send-issued`, nothing typed;
  (.andromeda/architecture.md:217)
  > `send-refused`: `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
- (.andromeda/architecture-detail/conventions.md:8) The budget test's refusal must be the `RefusalReason` `budget-paused` with a detail from its closed set.
  > **`RefusalReason` (serialised kebab-case):** `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown`.
  (.andromeda/architecture-detail/conventions.md:10)
  > - `budget-paused`: `five-hour` · `seven-day`

## Bindings
- **stack-and-technologies Wrapper IPC ↔ established-decisions IPC ↔ conventions endpoint name**: the listener properties the chunk asserts (DACL, remote rejection, first instance) stay interprocess behaviour behind the `channel` seam over the hashed `viola-<h12>` name, so a first-instance test binds a held name, never a `viola-<h12>` one viola would bind, (.andromeda/architecture-detail/stack-and-technologies.md:15), (.andromeda/architecture-detail/established-decisions.md:61), (.andromeda/architecture.md:129)
- **established-decisions session liveness ↔ conventions exit codes ↔ established-decisions CLI conventions**: the bind's taken-name outcome is one refusal on three surfaces — the start order (bind before the first snapshot), exit 1 with the fixed stderr pair, and no `--json` document — so a listener-level first-instance assertion must leave all three as landed, (.andromeda/architecture-detail/established-decisions.md:176), (.andromeda/architecture-detail/conventions.md:23), (.andromeda/architecture-detail/established-decisions.md:211)
- **stack-and-technologies concurrency ↔ established-decisions concurrency ↔ module boundaries**: edits in `viola-channel` stay sync, its normal graph tokio-free by `scripts/channel-tokio-free.sh`, its package-scoped test build alone enabling `tokio` through `fake-agent`, (.andromeda/architecture-detail/stack-and-technologies.md:11), (.andromeda/architecture-detail/established-decisions.md:8), (.andromeda/architecture-detail/established-decisions.md:190)
- **stack-and-technologies error types ↔ established-decisions error handling ↔ conventions Rust error types**: any listener failure the chunk adds is a fixed-message `ChannelError` variant, (.andromeda/architecture-detail/stack-and-technologies.md:24), (.andromeda/architecture-detail/established-decisions.md:184), (.andromeda/architecture.md:106)
- **established-decisions budget governor ↔ standard-contracts send ↔ conventions RefusalReason ↔ stack-and-technologies logging**: the budget test the chunk fixes asserts the refusal word and window detail, no cursor before `send-issued`, and reads its `send-refused` line through a tracing-subscriber capture that must hold under libtest threads, (.andromeda/architecture-detail/established-decisions.md:131), (.andromeda/architecture-detail/standard-contracts.md:47), (.andromeda/architecture-detail/conventions.md:10), (.andromeda/architecture-detail/stack-and-technologies.md:21)
- **stack-and-technologies CI ↔ the body's Design Philosophy**: the chunk's Windows evidence is the `windows-2025` runner's, and what that runner cannot measure stays owed to "Windows-only live measurements", (.andromeda/architecture-detail/stack-and-technologies.md:31), (.andromeda/architecture.md:7)
