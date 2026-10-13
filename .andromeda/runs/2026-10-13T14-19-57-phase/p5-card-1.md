# P5 review — 2026-10-13-epoch-4-survivors · Epoch 4 survivors

**Sized against one window (`inputs#I1`) and split by the founder (`inputs#I2`, «Делить на две», relayed verbatim
by the operator):** this chunk disposes 31 of the Epoch 4 audit's 47 survivors, keeps the 2 `scratch.rs` records
standing, and owes `typed.rs`'s other 16 (the Run B driver restatement) to a new entry directly after it.

## Goal
Each of the 31 gets a nextest case in the package that owns it, shown red under its hand-applied mutant and green
restored. Two survivors first get a behaviour-preserving restatement. No mutation run, witness run or dispatch, and
no live `claude` (`inputs#I1`). One ledger, `evidence/survivors.ndjson`, holds all 47 rows with their form and
disposer, for the Epoch 5 boundary audit to read against its own run.

## What it does, by file
- **`viola-pty` `resize` (1)** — measured first. Under the mutant the three resize tests passed in about 5 s. The
  likely cause, a HYPOTHESIS: the audit ran two cargo-mutants jobs under one `TMPDIR`, and the test's report file is
  named after the test alone. Branch b (red when run alone): the report file gains the process id. Branch c (green
  alone): a new case on the child's report, and the cause recorded.
- **`statusline.rs` `shell_argv` (2)** — the test asserts the whole argv, not `is_some()`.
- **`fs.rs` `replace_private_with` (6)** — restatement: the loop moves into `replace_private_retrying(…, retry)`, and
  `replace_private_with` keeps its signature and passes `retry_replace(_, HOST_IS_WINDOWS)`. Two Linux cases (retry
  always true: 99 pauses then the error; always false: no pause) kill all six on Linux.
- **`board.rs` (4)** — `unknown_fields` summed over two instances; an unstattable home refused; an unlistable
  `instances/` is `Err` (`cfg(unix)`, mode 000, restored before drop).
- **`wheel.rs` (9)** — split-read classifier cases (the carried state tells each mutant apart), boundary pairs at
  `BOUND`, DA with no field, the kitty reply with two fields, DCS ended by BEL; one recorder case where the log append
  fails and `pause` answers `Internal`. F-W2's list is unchanged.
- **`dialog.rs` (2)** — `millis` directly; tool A's arm survives tool B's PreToolUse.
- **`hook.rs` `answer` (1)** — a relative answers path that resolves to a planted dir prints nothing.
- **`send.rs` `Out::open` (1)** — restatement: `issue_line_shows(json, terminal)`, a four-case table; the line is
  byte-identical.
- **`send/slot.rs` `close` (1)** — after `confirm` expires, a later matching line is not claimed.
- **`typed.rs` (2 of 18)** — `max_gap` with a chunk outside the window; `Feed::take` at exactly `MAX_FRAME`. Test
  module only (a hunk guard keeps it there).

## Top acceptance criteria
- 31 cases, each red under its mutant and green restored, with both readings in `evidence/survivors.md`; none
  disposed by argument.
- The ledger entry reads `last line 1`: 47 rows keyed on the audit record, 16 `owed`, 2 `not-measured-here`, every
  disposer named.
- `replace_private_with`'s contract holds (mode first, 100 attempts, Windows error 5 only); `shell_argv` is exact;
  the `send` and `list` output is byte-identical; F-W2's list is exact.
- No manifest, lockfile, env var, feature or other file changes (the preservation guard); `typed.rs` changes only in
  its test module.
- `run --unit` and `pre-push` green; CI `verdict: green` on HEAD on all three OSes, the run id named.

## Touchpoints
`crates/viola-pty/src/lib.rs` · `crates/viola-agent-claude/src/statusline.rs` · `crates/viola-state/src/{fs,board}.rs`
· `src/run/{wheel,dialog}.rs` · `src/cmd/{hook,send}.rs` · `src/run/send/slot.rs` · `src/cmd/verify/typed.rs` (test
module). No new file.

## Test commands (20 entries)
fmt · clippy · Windows-target clippy · G1 · lint probes · `run --unit` · **the ledger read** (new; baseline red,
the file is absent; controls 47 rows green, 46 red) · **the typed.rs hunk guard** (new; baseline green; controls red
at `:300`, green at `:700`) · **the preservation guard** (new; baseline green; controls red on `src/main.rs`, green on
two listed files) · `cargo deny` · smoke cleanup/boot/status/cleanup (new, all green) · G2 · G4 · `pre-push` · the
operator's hygiene, push and CI read. The controls are in the run dir's `p5-controls.md`.

## Capabilities
None claimed. Of the 37 unclaimed, none names mutation strength or a survivor site.

## Expected amendments (wrap)
- test-plan §10 — the Mutation gate paragraph: the Epoch 4 survivors' disposals, the 2 `prepare` records standing,
  16 owed.
- architecture §Established Decisions [Snapshot writer] — the retry decision supplied by the caller, behaviour
  unchanged.
- architecture §Occupied Resources — under branch b, the pty watch report named with the process id.
- test-plan §3 `run` step 4 or §10 — under branch b, two mutation jobs must not share a test's report file
  (HYPOTHESIS).

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | A new working-route entry **"Epoch 4 survivors II"**, placed directly after "Epoch 4 survivors" under `### Epoch 5 — Driver surface`. It carries `typed.rs`'s 16 survivors by audit coordinate: `framing_turns` `:271:84` ×2, `:286:29`, `:288:40` ×2; `SharedPty` `resize` `:403:9`, `close` `:415:9`, `child_pid` `:419:9` ×3; `Run::end_by_ctrl_c` `:648:12`, `:651:12`, `:648:45`, `:651:45`; `Run::wait_pump` `:659:9`; `Run::finish` `:671:53`. It also carries research.md §Measured facts 4 and 5 (no `Pty` double seam; `SharedPty::resize` / `child_pid` uncalled by any product path) and the disposal forms of test-plan §10. It inherits `inputs#I1`'s direction (no mutation run, witness run or dispatch; no live `claude`) and rests on the founder's split (`inputs#I2`). | route insertion (the wrap mints it) |
