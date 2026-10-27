# Remove-the-guard controls — 2026-10-27-bounded-inputs-at-every-boundary (implement)

## Steps 1–2: the MCP stdin line cap (2026-10-27T05:40Z, Linux dev host)
Guard neutralised in the working tree, both at once: `crates/viola-mcp/src/stdin.rs` `fits` read
`len <= self.cap || true` (no line ever cut), and `serve` built the reader with `u64::MAX` in place of
`MAX_FRAME`. Then restored, each restore read back by grep (`CappedLines::new(stdin, MAX_FRAME)`,
`len <= self.cap)`).

Command: `cargo nextest run -p viola-mcp --no-fail-fast -E 'test(/(^|::)stdin::tests::/) | test(/(^|::)tests::serve_/)'`

- Neutralised: exit 100, `13 tests run: 7 passed, 6 failed` — red: `one_past_the_cap`,
  `past_the_cap_with_no_newline`, `capped_lines_yields_a_fitting_line_then_ends_at_an_over_long_one`,
  `capped_lines_cuts_an_over_long_line_split_across_reads`,
  `capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more`, and
  `serve_ends_the_session_at_a_line_past_max_frame` at `5.030s` (the child killed at the case's 5 s bound,
  its exit code `None`, `lib.rs:192`); green the cases with no cut to make.
- Restored: exit 0, `13 tests run: 13 passed, 63 skipped`.

## Step 5: the bounded compare read (2026-10-27T05:42Z, Linux dev host)
Guard neutralised: `holds_exactly`'s cap `len.saturating_add(1)` read `len.saturating_add(0)` (a read of
exactly `bytes.len()`, which a longer target with `bytes` as its prefix would pass). Restored and read back
by grep (`len.saturating_add(1));`, `crates/viola-state/src/fs.rs:365`).

Command: `cargo nextest run -p viola-state --no-fail-fast -E 'test(/(^|::)tests::replace_private_shared/)'`

- Neutralised: exit 100, `3 tests run: 2 passed, 1 failed` — red
  `replace_private_shared_reads_no_more_than_one_byte_past_the_bytes` (`fs.rs:400`, the `same+` row).
- Restored: exit 0, `3 tests run: 3 passed, 305 skipped`.

## Step 8: the reader census (2026-10-27T05:51Z, Linux dev host)
Control: step 5 undone in the working tree (`replace_private_shared`'s guard back to the whole-file
`fs::read(path).ok().as_deref() != Some(bytes)`), the file then restored from a copy and read back by grep
(`crates/viola-state/src/fs.rs:354` `Err(e) if !holds_exactly(path, bytes) => Err(e),`).

Command: `cargo nextest run --features fake-agent -E 'binary(contract_bounded_readers)'`

- Step 5 undone: exit 100, `11 tests run: 10 passed, 1 failed` — red `product_readers_are_each_bounded_or_named`
  naming exactly `crates/viola-state/src/fs.rs:354` (the unbounded compare read research's sweep found).
- Restored: exit 0, `11 tests run: 11 passed`.
- Before the exception match read the call's whole line, the first run read red on exactly the two named
  own-executable reads (`src/cmd/run.rs:197`, `crates/viola-state/src/pin.rs:83`) and no other site: the
  census finds every unbounded product read the research sweep found and no more.

## Step 7: the passthrough half (2026-10-27T05:53Z, Linux dev host)
Two forced-behaviour controls in `src/run/gate.rs` `Tee::write` (the plan's lists do not hold the file: each
edit restored from a copy, `git diff --stat src/run/gate.rs` empty after), each under
`cargo nextest run --features fake-agent --profile ci -E 'binary(chaos_feed_panic)'`:

- A, passthrough harmed — the tee drops any write holding `0xe4` (the panicking bytes never reach the outer
  terminal): exit 100, red at `7.293s`, `tests/support/watch.rs:54` "timed out waiting for the child's bytes".
- B, viola's own bytes shown — the tee writes `vt100 panic` after a write holding `0xe4`: exit 100, red at
  `0.297s`, `tests/chaos_feed_panic.rs:124` "viola's own `panic` reached the outer terminal".
- Unmodified: exit 0, `1 test run: 1 passed` (`0.248s`).
