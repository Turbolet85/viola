# arch · Standard Contracts — category read

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:47) The folded CARRY case `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` must assert, and any fix to its log capture must keep, this exact refusal contract: `budget-paused` with the window word, before `send-issued`, no cursor, nothing typed.
  > A `send` reads the wrapper's budget gate once at its arrival ([Budget Governor]); a closed gate is the refusal `budget-paused` with detail `five-hour` or `seven-day`, before `send-issued`, with no cursor, nothing typed and no flight reserved, and a gate change that cannot be recorded there answers `-32603`.
- (.andromeda/architecture.md:180) The body's broad line for the same CARRY case; a fix to the case's capture may not move the refusal after `send-issued`.
  > A closed budget gate refuses it `budget-paused` before `send-issued`, nothing typed;
- (.andromeda/architecture.md:217) The `send-refused` line the CARRY case looks for must carry `{refusal, detail}` with no `cursor`, since this refusal stands before `send-issued`.
  > - `send-refused`: `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
- (.andromeda/architecture.md:220) The wrapper must append that `send-refused` record itself (`source: wrapper`), and it must not wake `wait`; a capture fix may not turn it into a waking or hook-sourced line.
  > and `send-issued` / `send-confirmed` / `send-refused` (`source: wrapper`) for each send. They are log-only and never wake `wait`.
- (.andromeda/architecture.md:227) The `squatted-name` start refusal keeps printing no `--json` document; its document belongs to "Exit-cause code catalogue", so this chunk must not add one while it asserts the squat path.
  > A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".
- (.andromeda/architecture.md:156) If the chunk touches `run`'s bind or its start refusal, the bind must still come before the first snapshot, so that `endpoint` is present from the first one; a refused bind (squatted name) writes no snapshot that names that endpoint.
  > `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
- (.andromeda/architecture-detail/standard-contracts.md:25) If `src/cmd/run.rs`'s start refusal is touched, `run`'s collision check stays one of the five `read_snapshot(` product callers. It must still read an unreadable or unsupported snapshot as absent, and the count is kept by its stated rule.
  > `read_snapshot` keeps its results, and its five product callers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still read every such snapshot as absent.

## Bindings
- **`squatted-name` refusal (no `--json` document) ↔ Conventions, CLI exit codes**: the start refusal stays exit 1 with no document (.andromeda/architecture.md:227), and exit 1 stays the code for start refusals (.andromeda/architecture.md:100).
- **`endpoint` in the instance snapshot ↔ Occupied Resources, IPC endpoints**: the snapshot's `endpoint` comes from the bind (.andromeda/architecture.md:156), and the bound name is `\\.\pipe\viola-<h12>` (.andromeda/architecture.md:260). The chunk's new listener tests must bind only the test-only `viola-test-chan-<pid>-<label>` names, which sit outside that namespace (.andromeda/architecture.md:261).
- **Channel methods ↔ Cross-cutting Patterns, Local endpoint trust boundary**: every method a frame can reach (`send`, `answer` and the rest, .andromeda/architecture.md:178) is reachable by any process that can open the endpoint. The pipe admission this chunk asserts is the Windows enforcement of "must not be connectable by another OS user" (.andromeda/architecture.md:420), and it changes no frame or method shape.
- **`budget-paused` send refusal ↔ Established Decisions, [Budget Governor]**: the CARRY case's refusal contract (.andromeda/architecture-detail/standard-contracts.md:47) is the soft stop that decision sets (.andromeda/architecture.md:53).
