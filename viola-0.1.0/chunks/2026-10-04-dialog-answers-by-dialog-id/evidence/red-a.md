# Red A — `cli_send` stdin `BrokenPipe` (ci#37196414168, `test (ubuntu-latest)`)

## The CI reading
- Job 111419123020, step "Coverage and doctest": `send_leading_slash_argument_is_a_usage_error` FAIL,
  `panicked at tests/cli_send.rs:84:38: stdin: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }`.
  The test failed before its own assertions ran.

## The forced window
`send_leading_slash_argument_exits_before_its_stdin_is_written` starts `viola send builder /clear` with its stdin
piped and nothing written, waits (bounded `try_wait` under `WITHIN`) for the child to EXIT on the usage error, then
writes. The write always meets a closed pipe.

## Readings (this host, Linux, 2026-10-04)
- **Before the fix** (`feed` still `.expect("stdin")`s the write, as `spawn_send` did at `c540254`):
  `cargo nextest run --features fake-agent --test cli_send -E 'test(=send_leading_slash_argument_exits_before_its_stdin_is_written)'`
  → exit 100, `FAIL … send_leading_slash_argument_exits_before_its_stdin_is_written`,
  `panicked at tests/cli_send.rs:88:38: stdin: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }` — the
  CI signature, reproduced on demand.
- **After the fix** (`feed` returns the write's `Result`; `spawn_send` tolerates `ErrorKind::BrokenPipe` only and
  panics on any other error): the same selector widened to `test(/send_leading_slash/)` → exit 0,
  `PASS send_leading_slash_argument_exits_before_its_stdin_is_written` (asserts `Err(BrokenPipe)`, exit 2, empty
  stdout) and `PASS send_leading_slash_argument_is_a_usage_error` (every assertion kept: exit 2, empty stdout, no
  `events.ndjson` growth, no typed prompt).
