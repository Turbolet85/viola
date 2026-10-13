# P5 controls — 2026-10-13-interrupted-verify-cleanup

Each control fired AS THE ENTRY through `gate.py run --plan … --entry {n} --project-root $TMPDIR/{scratchpad}/clone`
(no `--run-dir`), over a `git clone --shared` of the project at HEAD `d9f01b1b8642`. Each control input was an edit
left uncommitted in the clone and restored (`git checkout -- .`) before the next. The inputs were applied by
`$TMPDIR/{scratchpad}/ctl.py`.

| entry | clone input | reading |
|---|---|---|
| 8 (literal probe) | unmodified | red — last line '3' (as on the real tree: two `"--settings"` in typed.rs, one `"claude"` in verify.rs, above each test module) |
| 8 | `"--settings"` → `SETTINGS_FLAG` above typed.rs's `#[cfg(test)]`, `OsString::from("claude")` → `OsString::from(PROGRAM)` above verify.rs's (must pass) | green — last line '0' |
| 9 (preservation guard) | unmodified | green — exit 0 (as on the real tree) |
| 9 | `Cargo.toml` edited (must fail) | red — exit 1 |
| 9 | the six modify-set files edited and `src/cmd/verify/stop.rs` minted (must pass) | green — exit 0 |
