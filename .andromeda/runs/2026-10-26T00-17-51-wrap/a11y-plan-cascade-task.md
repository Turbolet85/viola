# Task — a11y-plan · 2026-10-25-windows-endpoint-admission

## The session
Test-only chunk: no product file changed, and no product behaviour, wire contract, exit code, refusal word, human output line, SGR byte or terminal byte moved.
Added five Windows-only integration cases in tests/security_negatives_channel.rs asserting the wrapper IPC pipe listener rejects remote clients (pipe-type read-back, with accept-remote and reject-remote controls) and denies an anonymous-token open (with a default-descriptor control); test pipes stay in the viola-test-chan-<pid>-<label> namespace.
Added the root-bin test helper test_support::with_scoped_default in src/main.rs plus its witness test, and moved nine root-bin scoped tracing captures onto it (the census of subscriber::with_default in src goes 9 to 1, the helper's own); the folded case_4 capture loss is now root-caused and witnessed.
Added a root Windows dev-dependency table for windows-sys (features Wdk_Storage_FileSystem and Win32_System_IO); Cargo.lock unchanged, no release graph change.
Measured on CI test (windows-2025) (ci#40247500584, green; JUnit 2988 tests, 0 failures): remote-client rejection and the DACL's effect, which test-plan had called undrivable on hosted runners; the full CI row is red only on supply-chain (tokio RUSTSEC-2026-0386), owned by a new route entry "tokio RUSTSEC-2026-0386 lock bump".
Plain cargo test --bin viola still reads two known-cause reds, recorded with no owner (plain cargo test unsupported); the macOS readiness watch read 1 green run, not recurred.

## Reached files
- tests/security_negatives_channel.rs
- Cargo.toml
- src/main.rs
- src/run/send/test_support.rs
- src/run/dialog.rs
- src/run/budget.rs
- src/run/gate.rs
- src/obs.rs
- src/cmd/run.rs
- src/cmd/hook/statusline.rs

## Touches
- Scope's not-assertable entity "wrapper IPC channel" (ipc-internal, no UI surface): the session adds Windows security negatives on exactly that pipe listener; re-read, the entity stays not-assertable and gains no a11y assertion.
- Scope's not-assertable entity "Internal crates": the new cases bind viola_channel::Server::bind from test code only; no signature changed, the entity's status is unmoved.
- CI integration: the new Windows cases run inside the existing per-OS test (windows-2025) job's single instrumented coverage suite; no new suite enum value, no new driver, no CI step changed, so the "no new suite" and "inside the coverage suite on all three OS legs" statements stand unmoved.
- Gate: the per-OS test job's gate --require coverage,doctest is unchanged; the windows-2025 leg read green on ci#40247500584, the full row red only on supply-chain, which is no a11y gate.
- Where results land: the run JSON suites[] shape is unchanged; the CI JUnit artifact junit-windows-2025 carries the new cases, which is no a11y artifact (artifact retention statement unmoved).
- SLO cross-input parallel (zero-flakiness, a flake keeps the chunk red until its root cause is fixed): the folded case_4 capture loss is now root-caused (a callsite first registered on a subscriber-less thread while the scoped capture is the only dispatcher) and fixed by with_scoped_default; the rule is honoured, its text unmoved.
- SLO cross-input parallel, indirect through the plain cargo test reds: two known-cause reds of a plain cargo test --bin viola are recorded with no owner because plain cargo test is not the harness runner; the a11y runner statement (scripts/agent-run.sh run, the 5-command discipline) is unmoved.
- Cognitive timeouts list (the status-line deadline, the budget pause, the dialog hold, the send readiness gate, the confirmation windows): src/cmd/hook/statusline.rs, src/run/budget.rs, src/run/dialog.rs, src/run/gate.rs and src/cmd/run.rs were reached only in their #[cfg(test)] capture call sites; no bound moved and none newly falls on the human.
- P4 wheel and TUI boundary (viola writes zero terminal bytes; diagnostics for run go to diagnostics/): src/run/gate.rs and src/cmd/run.rs test captures only; its one global subscriber install (set_global_default) is untouched, so no byte reaches the terminal.
- Unit-tier proofs that run under root-bin scoped captures (the dim-on-stale-rows unit proof and any obs-line assertion inside the reached modules): every captured line still goes through obs_event!, the captures only route through the new helper; the "proven at the unit tier only" owed item to "Linux and macOS parity" is unmoved.
- Tool inventory (assert_cmd 2.2.2, predicates 3.1.4, trycmd 1.2.1, portable-pty =0.8.1): Cargo.toml gained only a Windows windows-sys dev-dependency table; no a11y tool version moved and Cargo.lock is unchanged.
- Outcome's (design · layouts · a11y) acceptance: no human output, refusal text or SGR byte changed, met with no product file edited; every new surface's coverage row reads a11y n/a and tokens n/a.

## New facts
- (none)

## Untouched
- Visual SGR rules (amber only on DIALOG, dim on stale rows, bold NAME, no SGR under non-TTY / NO_COLOR / TERM=dumb / --json / viola run / a passed viola revive, unable and fail never coloured, NAME cutting): no styling or list code was reached.
- The CAPTION literal and caption order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI and the -- UNWRAPPED - READ-ONLY -- separator: viola list's code and output were not reached.
- Error recovery layout (unable <reason> <detail>, final hint:, unknown exit 14 hint-free, hint-free --json) and the trycmd pins: no refusal word, exit code or copy changed.
- Plain-language fixed copy set and the kebab-case codes beside unable: no fixed CLI string changed.
- The win32-input-mode mouse fact (F-W3) and the Windows outer-PTY wheel cases: the Windows work here is the IPC pipe listener, not the PTY input path.
- Manual screen-reader passes (NVDA, VoiceOver, Orca on P4 and P6) and the SR test pattern table: nothing in the session reaches them.
- A11y triggers (triggered and not-triggered) and the compliance cross-check: no regime, surface or user signal changed.
- The revive --json owed item to "Exit-cause code catalogue" and the --help handoff: row owed to "Explicit session links": both owners stand; the new route entry is the tokio lock bump, unrelated to them.
- The frontend deferral to 0.2.0 and the 0.1.0 no-browser-a11y-leg statement: unchanged.
