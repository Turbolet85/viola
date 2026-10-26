# arch · Conventions — category read

## Carried
- (.andromeda/architecture-detail/conventions.md:23) The squatted-name start refusal stays exactly this exit-1 shape (detail word, the two fixed stderr lines, no path, pid or pipe name) while the chunk asserts first-pipe-instance at the listener; no exit code or refusal word changes.
  > and `viola run` losing the exclusive endpoint bind to another holder (`detail:"squatted-name"`, `unable: the endpoint for <name> is held by another process` / `hint: another process holds this name's endpoint; stop it or pick another name`, no path, pid or pipe name)
- (.andromeda/architecture.md:100) The broad exit-code line the start refusal sits under: a squat is an exit-1 start refusal, never a new code.
  > `0` ok · `1` internal error, and the start, revive-preflight and verify refusals · `2` usage error
- (.andromeda/architecture-detail/conventions.md:34) A client the pipe refuses (a remote client, a principal outside the two ACEs), if asserted at the CLI, reads as an unreachable instance, exit 21, never a new code.
  > `21` instance unreachable (no live endpoint for the `ViolaName`)
- (.andromeda/architecture-detail/conventions.md:26) A failed connect is exit 21 for `wait` / `last`, not exit 1; a denied open must land on that arm.
  > any channel error but a failed connect or a closed / reset connection (those are exit 21)
- (.andromeda/architecture-detail/conventions.md:10) The folded CARRY's test case `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` must assert the window word from this closed detail set, whichever way the CARRY is closed (three plain `cargo test` runs or a fixed log capture).
  > `budget-paused`: `five-hour` · `seven-day`
- (.andromeda/architecture-detail/conventions.md:15) The budget refusal the CARRY case exercises comes in this fixed position of `send`'s refusal order; a fix to its log capture must not reorder it.
  > then `human-typing`, `budget-paused`, `not-delivered`/`turn-running` (a turn running, or another `send` in flight)
- (.andromeda/architecture.md:106) Any new failure the listener tests or code add in `viola-channel` (a remote-reject or second-instance bind failure) goes into `ChannelError`, a `thiserror` enum with fixed messages.
  > **Rust error types:** one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`), each a `thiserror` enum except `PtyError`, whose `Display`/`Error` are hand-written with fixed messages
- (.andromeda/architecture.md:129) The pipe the squat and first-instance assertions hold is the product name derived by this rule (FNV-1a, never `DefaultHasher`); the chunk changes no endpoint naming.
  > - **Endpoint name:** `viola-<h>`, where `<h>` is the first 12 hex characters of an FNV-1a 64-bit hash over `ViolaName` + `\0` + the absolute viola home path. It is hand-written in `viola-channel` and recorded in the instance snapshot. std `DefaultHasher` is forbidden because its output is not stable across Rust releases, and mixed binary versions must agree on the name.

## Bindings
- **squatted-name exit 1 ↔ the body's Standard Contracts (CLI `--json` output)**: the start refusal prints no `--json` document as landed, its document owed to "Exit-cause code catalogue"; this chunk adds none (.andromeda/architecture-detail/conventions.md:23) ↔ (.andromeda/architecture.md:227)
- **squatted-name exit 1 ↔ security-plan §Authentication & Authorization (IPC access control, Windows)**: the first-instance listener property the chunk asserts is what produces the squatted-name refusal; both sides keep `squatted-name`, exit 1 (.andromeda/architecture-detail/conventions.md:23)
- **exit 21 on a refused open ↔ security-plan (client SQOS open, "Server verification before any frame")**: a denied or remote-rejected client maps to exit 21; the client-side verification is the next route entry, not this chunk (.andromeda/architecture-detail/conventions.md:34)
- **budget-paused window word ↔ the body's Standard Contracts (`send-refused`, channel `send`) and test-plan (the folded CARRY)**: the refusal comes before `send-issued` with no `cursor`, its detail `five-hour` / `seven-day` (.andromeda/architecture-detail/conventions.md:10) ↔ (.andromeda/architecture.md:217)
- **Endpoint name ↔ the body's Occupied Resources (IPC endpoints)**: listener tests bind the test-only `viola-test-chan-<pid>-<label>` names outside the `viola-<h12>` namespace, never a product name (.andromeda/architecture.md:129) ↔ (.andromeda/architecture.md:261)
- **`ChannelError` ↔ security-plan §Error Handling**: fixed `Display` messages, no pipe name or path in any new variant (.andromeda/architecture.md:106)
