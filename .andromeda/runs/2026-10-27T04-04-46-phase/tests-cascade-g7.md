# tests · cascade read · group 7 of 8

## Carried
- (.andromeda/test-plan-detail/test-data-fixtures.md:10) Every oversize `snapshot.json` and over-long `events.ndjson` line the chunk's cases plant must be a corruption of a file the product already wrote in a fresh home, never a hand-written state file.
  > Produced only by running real `viola run` / `hook` / CLI verbs against `viola-fake-agent` in a fresh home, never hand-written. Three exceptions: the chaos tests, which corrupt files the product already wrote;
  (.andromeda/test-plan.md:634)
  > **On-disk product state:** produced only by real `viola` runs against the fake agent in a fresh home, never hand-written — except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`;
- (.andromeda/test-plan-detail/test-data-fixtures.md:12) An oversize or malformed `ledger/stamps.json` reader case must start from the file `viola verify` wrote and only alter those bytes, since no test or harness code writes the file; where it does alter them, the row must record that reader-unit form as the `budget.json` row does.
  > `viola verify --home <home>` run against the fake agent. No test or harness code writes the file.
  (.andromeda/test-plan.md:636)
  > **`ledger/stamps.json`:** produced by `viola verify` against the fake agent; no test or harness code writes the file.
- (.andromeda/test-plan-detail/test-data-fixtures.md:13) The `budget.json` cap and failure witnesses must take the bytes `write_budget` wrote and pad, replace or re-version them, and booted-wrapper cases must reach `budget.json` only through generated statusline stdin.
  > Generated statusline stdin piped into `viola hook statusline`, in every booted-wrapper test. The reader's own unit cases (`budget_read_*`, `crates/viola-state/src/budget.rs`) take the bytes `write_budget` wrote and pad, replace or re-version them, because no writer produces an over-cap, unparseable or `v` 2 file
  (.andromeda/test-plan.md:638)
  > **`budget.json`:** generated statusline stdin piped into `viola hook statusline` in every booted-wrapper test; the reader's own unit cases alter only bytes the writer wrote.
- (.andromeda/test-plan-detail/test-data-fixtures.md:14) An oversize `statusline-source.json` (or default-home `settings.json`) for the size-capped start read must be planted by the test through the `tests/support/home.rs` helpers, never written by viola.
  > Written by the test, never by viola, which only reads it: `tests/support/home.rs` `plant_statusline_source(home, command)`, with the command from `statusline_echo_command(marker, extra)` and the marker at `statusline_marker(home)`.
  (.andromeda/test-plan.md:640)
  > **The statusline source:** written by the test, never by viola, which only reads it.
- (.andromeda/test-plan-detail/test-data-fixtures.md:16) The oversize and malformed `claude agents --json` reads must be driven by the fake agent placed as `claude` in a test-scoped bin dir with `agents-mode` `oversize` / `malformed` beside its exe, no env var, every `list` / `revive` under a test-scoped `PATH`.
  > `viola-fake-agent` placed as `claude` in a test-scoped bin dir (`tests/support/fake.rs` `claude_bin` / `claude_in`; `empty_bin` for a `PATH` holding no `claude`), answering `agents --json` from files beside its exe: `agents-mode` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `agents-rows.json` (planted rows) and the live `sessions/<pid>.json` registry its interactive runs write; no env var (the founder's own answer to P4 fork 2 of chunk 2026-10-16-unwrapped-sessions-on-the-board, `inputs#I1`); every test-started `list` / `revive` runs under a test-scoped `PATH` (`support::cli::spawn_on` / `viola_on`, or a plain `spawn` of `list` under an empty bin dir)
  (.andromeda/test-plan-detail/test-data-fixtures.md:70)
  > `agents` (chunk 2026-10-16-unwrapped-sessions-on-the-board): a first word `agents` answers `agents --json` from files beside its exe (`agents-mode`, `agents-rows.json`, and the live `sessions/<pid>.json` registry
  (.andromeda/test-plan.md:642)
  > **`claude agents --json` output:** `viola-fake-agent` placed as `claude` in a test-scoped bin dir, answering from files beside its exe with no env var; every test-started `list` / `revive` runs under a test-scoped `PATH`.
- (.andromeda/test-plan-detail/test-data-fixtures.md:22) The chunk's properties (paste chars over decoded chars vs raw bytes, `ViolaName` with its invalid neighbours, `Percent` out of range, the hook stdin, statusline and agents parser inputs) must be proptest strategies, and a strategy the chunk adds must be recorded in this list.
  > proptest strategies (`ViolaName` + invalid neighbours, `Percent` ± range, event lines, `rate_limits` JSON).
  (.andromeda/test-plan.md:654)
  > **Randomized data:** proptest strategies, their seeds persisted in committed `proptest-regressions/`.
- (.andromeda/test-plan-detail/test-data-fixtures.md:43) The fake agent offers only recorded fixture bytes on a hook's stdin, so an oversize, malformed or deep-nested hook payload must reach `viola hook` by a direct spawn, and planted status line stdin through the fake agent only with `--settings` naming an absolute command and `--statusline-stdin`.
  > The payload on stdin is the bytes of `<fixtures>/<cli-version>/<Event>.<variant>.json`;
  (.andromeda/test-plan-detail/test-data-fixtures.md:43)
  > Given `--settings <file>` it reads that file's `statusLine.command` and splits it on ASCII white space; only when `--statusline-stdin <file>` is given too and the command is absolute does it run the command, with that file's bytes on stdin, and write a `statusline` receipt (landed at chunk 2026-10-10-statusline-pass-through).
- (.andromeda/test-plan-detail/test-data-fixtures.md:58) The wrapper-side paste witnesses must read what `send` typed (LF / CR / TAB and multi-byte UTF-8 accepted, CR LF as one LF, a refused text typing nothing) from the fake agent's `prompt` receipt.
  > - `prompt` {`text`, `hex`, `bare_esc`, `origin` `human`|`harness`, `submit` `fired`|`suppressed`|`local-command`|`no-fixture`|`no-hooks`}
  (.andromeda/test-plan-detail/test-data-fixtures.md:50)
  > - It writes receipt lines to `--receipt <path>`: ndjson, one `write_all` per line, every line `"v":1` + a kebab `kind`. The kinds are:
- (.andromeda/test-plan-detail/test-data-fixtures.md:70) The forced vt100 panic of the re-verified degrade must come from the fake agent's `--vt100-panic-bytes`, with no new env seam, and that case keeps its own screen verdict.
  > `--vt100-panic-bytes` is built (with confirmed `send`'s chaos case): after its `start` receipt it writes `e4 b8 ad` once to its terminal and receipts nothing new. Its bytes are measured: under a 24×1 PTY a wide character (`e4 b8 ad`) panics vt100 0.16.2, so no new env seam is needed (as measured at chunk 2026-10-04-readiness-gate-and-timing-constants).
  (.andromeda/test-plan-detail/test-data-fixtures.md:69)
  > a test whose fake agent writes another screen (e.g. `--vt100-panic-bytes`) keeps its own verdict
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) The agents and vt100-panic witnesses must use the real fake-agent stub executable resolved through `CARGO_BIN_EXE_viola-fake-agent` in root tests, any double stays on a seam trait, and the pure validators, parsers and refusal ordering the chunk re-verifies are tested directly.
  > `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only.
  (.andromeda/test-plan.md:714)
  > - Pure functions in modules under test (`validate_paste_text`, parsers, refusal ordering), which are tested directly.
  (.andromeda/test-plan.md:729)
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) A witness of the client-side channel frame cap made at a live socket must serve its oversize response from a test-owned interprocess listener, the only fake server the plan admits.
  > interprocess 2.4.4 server API (squatter listener; impostor listener); `viola-fake-agent` as the fake CLI;
  (.andromeda/test-plan.md:727)
  > - **Fake servers:** the interprocess server API binds the squatter before `run`, and `viola-fake-agent` is the fake CLI; no HTTP fake servers.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20) Every oversize state file and over-long ndjson line the chunk plants must sit on a real tempfile home and be refused or skipped by the product's own reader, never through a stubbed filesystem.
  > none (a real tempfile home, real modes and ACLs)
  (.andromeda/test-plan.md:710)
  > - Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
  (.andromeda/test-plan.md:715)
  > - On-disk state at integration level (the arch has no DB). Use real files in a tempfile home.

## Bindings
- **test-data-fixtures (state files altered only from the product's writer) ↔ mocking-stubbing-discipline (filesystem not mocked)**: an oversize `budget.json`, `snapshot.json` or `ledger/stamps.json` and an over-long `events.ndjson` line are the product's own bytes altered on a real tempfile home, read by the product's reader (.andromeda/test-plan-detail/test-data-fixtures.md:13) (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20)
- **test-data-fixtures (fake agent `agents` mode, `--vt100-panic-bytes`) ↔ mocking-stubbing-discipline (process mocking)**: the agents oversize/malformed reads and the forced vt100 panic come from the real stub executable, by files beside its exe and an argv option, never an env seam or a double (.andromeda/test-plan-detail/test-data-fixtures.md:16) (.andromeda/test-plan-detail/test-data-fixtures.md:70) (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18)
- **test-data-fixtures (fake agent hook stdin is recorded bytes) ↔ mocking-stubbing-discipline (pure functions tested directly)**: planted hook stdin reaches the parser directly at the unit and property tier or `viola hook` by a direct spawn, since the fake agent replays recorded payloads only (.andromeda/test-plan-detail/test-data-fixtures.md:43) (.andromeda/test-plan.md:714)
