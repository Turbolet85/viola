# design · Surface: {type} — category read

## Carried
- (.andromeda/design-system-detail/surface.md:65) A start refusal the closed pin race might newly reach (a second start reading the first start's copy) must keep the fixed one-line-plus-hint form and show no path or pid.
  > - The exit-1 start refusals of `viola run` have one fixed-message line and one hint per cause (T4). None of them shows a path or a pid:
- (.andromeda/design-system-detail/surface.md:69) The pinned copy's integrity refusal stays byte for byte as written: the race fix changes only the absent-copy write, and a present copy that differs is still refused with this pair, never overwritten.
  > - `unable: the pinned viola copy failed its integrity check` (SHA-256 mismatch) → `hint: the pinned copy was changed after it was written, so viola will not run it`
- (.andromeda/design-system-detail/surface.md:80) The log detail code for the pinned-copy refusal is a fixed name the fix must keep; no new detail code is coined for a concurrent-start outcome without naming it here.
  > obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`,
- (.andromeda/design-system-detail/surface.md:81) A pin write that fails for any other reason under concurrency is a fault, not a refusal: `error: internal error` with no hint, its detail only in `diagnostics/`.
  > No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals, and their detail goes only to `diagnostics/`.
- (.andromeda/design-system-detail/surface.md:141) The exit-1 row of the exit-code table fixes the human stderr form of both outcomes `run`'s pin step can reach (a fault and the integrity refusal).
  > | 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check`
- (.andromeda/design-system-detail/surface.md:113) `viola verify` reaches `pin_exe`; any change there must leave its step-counter output (one static line per relied row, then the summary) unchanged.
  > - **`verify`:** a step counter (the project's one progress pattern) with static appended stdout lines, one per relied ledger row:
- (.andromeda/design-system.md:332) A wait-out or retry added to the absent-copy write (as `pin_companions` does) must print nothing: `run` is silent once the child starts, and a start refusal prints only its fixed line and hint.
  > - **`run`:** prints nothing once the child starts. A start refusal (exit 1) prints its cause's fixed-message line, for example `unable: builder is already live`, then that cause's hint (`hint: viola list`). The per-cause lines are listed under the send hints above. None of them shows a path or a pid.
- (.andromeda/design-system.md:379) No progress or wait feedback for a contended pin may be written to `viola run`'s terminal.
  > - **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
- (.andromeda/design-system.md:380) A contended first start or a verify waiting on the pin gets no spinner or progress bar.
  > - **NEVER use a spinner, progress bar or live-redrawing dashboard.**
- (.andromeda/design-system.md:382) No output of the fix names the pinned path, the `bin/<key>/` hash key, a pid or a kernel error chain.
  > - **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.**
- (.andromeda/design-system.md:385) A failed concurrent pin prints no stack trace; its detail goes only to `diagnostics/`.
  > - **NEVER print stack traces.** Full detail goes to `diagnostics/` only.

## Bindings
- **Pinned-copy refusal string ↔ security-plan (hash-before-reuse) and obs-plan (D-20 `pinned-hash-mismatch`)**: the refusal pair and its detail code are the human and log faces of the re-hash rule the scope keeps unchanged; the fix touches only the absent-copy arm (.andromeda/design-system-detail/surface.md:69) (.andromeda/design-system-detail/surface.md:80)
- **Exit-1 stderr forms ↔ test-plan**: an assertion on a concurrent first start's stderr matches the exit-1 row's fixed strings, never a path or pid (.andromeda/design-system-detail/surface.md:141) (.andromeda/design-system.md:332)
- **Silent `run` passthrough ↔ architecture (`pin_bytes` in `run`'s start)**: any wait-out added on the exe's arm writes no byte to the child's terminal (.andromeda/design-system.md:379)
- **Reproducible-build measurement (part B) ↔ this category**: (none) — a measurement and a build matter, no surface output
