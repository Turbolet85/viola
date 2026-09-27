# Evidence — the replace retry's remove-the-guard pair (CARRY 5, plan step 4)

Host: Windows 11 (the retry is `cfg(windows)`-effective; the predicate itself runs on every OS). Base: `273e1ab`
plus this chunk's uncommitted tree. Measured 2026-09-27T21:55Z by `/andromeda-implement`.

The guard: `retry_replace` in `crates/viola-state/src/fs.rs`,
`host_is_windows && raw_os_error == Some(ACCESS_DENIED)`.

## Red — the guard neutralised
The predicate body replaced by `let _ = (host_is_windows, raw_os_error); false` (the edit confirmed landed by a
`grep -n` of the neutralised line before the run).

`cargo test -q -p viola-state --lib -- replace_private_lands_once_a_holding_reader_lets_go
snapshot_write_lands_through_a_reader_holding_the_file` → exit 101:

- `fs::tests::replace_private_lands_once_a_holding_reader_lets_go` panicked:
  `replaced once the reader let go: Io(Os { code: 5, kind: PermissionDenied, message: "Access is denied." })`
- `snapshot::tests::snapshot_write_lands_through_a_reader_holding_the_file` panicked:
  `written once the reader let go: Io(Os { code: 5, kind: PermissionDenied, message: "Access is denied." })`
- `test result: FAILED. 0 passed; 2 failed`

The first attempt fails with raw os error 5 exactly as research.md M1 measured, and nothing waits it out.

## Green — the guard restored
The predicate restored (`grep -c` of the original line = 1), the same command → exit 0,
`test result: ok. 2 passed; 0 failed`.
