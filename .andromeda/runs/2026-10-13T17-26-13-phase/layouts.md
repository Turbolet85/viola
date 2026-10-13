# layouts extract

## Relevance
partial — the chunk renders no surface and adds no element; its only contact with this domain is that the restatement of `typed.rs` (`viola verify`'s Run B driver) must leave the `cli` surface's `viola verify` output as the plan specifies it (a no-change invariant, not new layout work).

## Constraints
- The `viola verify` step output keeps its shape: one stdout step line per ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, then the `stamped <ver>  <n> pass  <m> fail` summary as the last stdout line; a restatement of the Run B driver (framing waits, the `SharedPty` seam, `Run`'s ending) may not add, drop, reorder or reword a line (per layout-templates §Surface: cli / Output structure — `viola verify`).
- Each step line is appended once, plain ASCII, no colour, no progress bar and no redraw; a named framing wait or a Pty double introduced by the chunk may not print progress, a spinner or a partial line to stdout (per layout-templates §Surface: cli / Output structure — `viola verify`).
- Exit 0 when every row passes and 1 on a failing row or a refusal, and `--record` adds no human line; a changed ending path (`end_by_ctrl_c`, `wait_pump`, `finish`) may not alter which exit a run reaches (per layout-templates §Surface: cli / Output structure — `viola verify`).
- `verify`'s exit-1 refusals stay the fixed `unable: <text>` / `hint:` pairs naming no name, path or pid, and a failed or panicked `verify` prints exactly `error: internal error` once from the one catch site; a restatement may not introduce a new refusal text or a second print site (per layout-templates §Surface: cli / Component — Primary content block 2: refusal lines and the `unable` column).
- The summary `stamped <version>  <n> pass  <n> fail` closes the run, with no terminator word or banner after it (per layout-templates §Surface: cli / Component — Footer / terminator).
- The human output is a contract that LLM drivers may read, so its columns and words stay stable (per layout-templates §Surface: cli / IA notes).

## Patterns to follow
- `viola verify` is the project's only progress pattern, a static step counter (per layout-templates §Surface: cli / Primary screens (commands)); a test that exercises the Run B driver asserts on the driver's return values and timing, never on a new human line.
- Expression level 0.0 conventions (no colour, no non-ASCII glyph, no cursor control) apply to non-TTY output and hold for verify's plain-ASCII step lines (per layout-templates §Surface: cli, Expression level).

## Anti-patterns to avoid
- Adding a human line, redraw or progress indicator to `viola verify` to make a survivor observable: the plan bans redraw and progress bars (per layout-templates §Surface: cli / Output structure — `viola verify`).
- Changing human output words to suit a test: the plan makes them a stable contract (per layout-templates §Surface: cli / IA notes).

## Contract bindings
- layouts ↔ tests: the `[NN/MM] … pass|fail` step lines and the `stamped` summary line are what any end-to-end check of `viola verify` reads; the chunk's kills go in `typed.rs`'s inline `mod tests` and need not read them, but whether an existing test pins the step-line text is research's question (per layout-templates §Surface: cli / Output structure — `viola verify`).

## Acceptance criteria contributions
- (layouts) After the chunk, `viola verify`'s stdout shape is unchanged: one `[NN/MM] <row id> <row words>  pass|fail` line per ledger row in ledger order, the `stamped <ver>  <n> pass  <m> fail` line last, plain ASCII, no redraw; the chunk's diff adds no stdout or stderr write outside `cfg(test)` (per layout-templates §Surface: cli / Output structure — `viola verify`).
- (layouts) No new `unable:` / `hint:` text or `error: internal error` site appears in `src/cmd/verify/` (per layout-templates §Surface: cli / Component — Primary content block 2: refusal lines and the `unable` column).
