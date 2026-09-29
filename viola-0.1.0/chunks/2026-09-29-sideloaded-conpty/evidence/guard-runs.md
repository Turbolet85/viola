# Remove-the-guard runs (testing.md 2026-09-25)

| guard | neutralised as | test run | reading |
|---|---|---|---|
| the System32 DLL-search restriction, `src/main.rs` (second statement, Windows) | the call compiled out (`#[cfg(any())]`) | `conpty_sideload::run_never_runs_a_planted_conpty` | **red**: the child was hosted by the planted `plant-cwd/OpenConsole.exe` (expected System32 `conhost.exe`); restored, green |
| the companion hash compare, `crates/viola-state/src/pin.rs` `pin_companions` | `found != pin` made unreachable | — | **NOT RUN**: the permission classifier refused running the tests with the check disabled ("Security Weaken"); per the overseer it was not routed around and not handed to the operator. The check was restored at once (read back). |

**Witness for the hash compare (overseer's ruling):** the acceptance-2 tampered-pinned-copy tests stand in its
place — `tests/conpty_sideload.rs` `run_on_a_tampered_open_console_falls_back_to_conhost_and_says_nothing` and
`run_on_a_tampered_conpty_dll_records_the_fallback` (one byte flipped in the pinned companion: `sideload_fallback`
`hash-mismatch`, host System32 `conhost.exe`, the file left as found), and the viola-state unit
`pin_companions_refuses_a_tampered_one_and_leaves_it`.
