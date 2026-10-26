# Outcome — 2026-10-25-windows-endpoint-admission (/implement)

**Outcome: green + smoke.** P2 green on its first full block run (1 iteration): `entries 24 · green 19 · red 0 ·
recorded 1 · timeout 0 · not-run 4` (trail `gate-2026-10-25-windows-endpoint-admission.json`); the four not-run are
the `operator` entries 21–24. Smoke: the plan's `role = 'smoke'` entries 14–19 (cleanup → boot → status → G2 → G4 →
cleanup) ran green as P2 gates; no product file changed, so no boot path moved and none was re-driven.

**Operator pass: done** (on the operator's word at invocation; record `chunks/…/evidence/operator-pass.md`): entry 21
hygiene `clean` → `pre-push` re-run green (100.13 s) → pre-CI commit `e2bb59c8` → entry 22 pushed
(`origin/build/viola-0.1.0 b6740d59→e2bb59c8`) → entry 23 `test (windows-2025)` **`verdict: green`** on
**ci#40247500584**, its JUnit holding all five new cases `passed` → entry 24 the full row `verdict: red · checks 15/15`,
its one red `supply-chain` (`unsound` RUSTSEC-2026-0386, the expected owner's), the other 14 checks `success`.

Next → `/andromeda-wrap-session`.

## Files changed (10, all in research's "Files to modify")
- `tests/security_negatives_channel.rs` — five `#![cfg(windows)]` cases: `security_negative_viola_listener_rejects_remote_clients`,
  `security_negative_control_accept_remote_pipe_reads_the_bit_clear`, `security_negative_control_reject_remote_pipe_reads_the_bit_set`,
  `security_negative_viola_listener_denies_an_anonymous_open`, `security_negative_control_default_pipe_admits_an_anonymous_read`,
  with the helpers `wide`, `test_pipe`, `listen_in`, `open`, `pipe_type`, `anonymous_read`.
- `Cargo.toml` — `[target.'cfg(windows)'.dev-dependencies] windows-sys = { workspace = true, features = ["Wdk_Storage_FileSystem", "Win32_System_IO"] }`.
- `src/main.rs` — `test_support::with_scoped_default` (a `OnceLock<tracing::Dispatch>` over a bare `registry()`, never
  the global default) and the witness `tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first`.
- `src/run/send/test_support.rs`, `src/run/dialog.rs` (2), `src/run/budget.rs`, `src/run/gate.rs` (the scoped one at
  :465; `set_global_default` at :429 untouched), `src/obs.rs`, `src/cmd/run.rs` (2), `src/cmd/hook/statusline.rs` —
  each `tracing::subscriber::with_default(` → `crate::test_support::with_scoped_default(`, nothing else.

## Deviations from the plan, with justifications
1. **Step 5 declares two features, not one.** windows-sys 0.61.2 gates `NtQueryInformationFile` on `Win32_System_IO`
   (`#[cfg(feature = "Win32_System_IO")]` above its `link!`, `src/Windows/Wdk/Storage/FileSystem/mod.rs`) and the call
   takes `Win32::System::IO::IO_STATUS_BLOCK`. Research's "no new feature is needed" missed it. Both features already
   reach the Windows graph by unification (`cargo tree -e features -i windows-sys@0.61.2 --target x86_64-pc-windows-msvc`:
   `Wdk_Storage_FileSystem` from mio 1.2.4, `Win32_System_IO` from interprocess 2.4.4, mio and socket2), so the lock
   does not move and no release graph changes; the entry declares what the test names, rather than leaning on a
   transitive crate's features. Same listed file; no scope-record line. The wrap's expected amendment for architecture
   §Infrastructure Patterns → Crate dependency direction should name both features.
2. **The anonymous cases bind before impersonating, on one spawned thread** (`anonymous_read` impersonates, opens and
   reverts inside one call), as step 7 orders it; the default-pipe control's pipe carries `PIPE_REJECT_REMOTE_CLIENTS`
   so its only difference from the viola listener is the descriptor.
3. **The remote-client read-back opens a client end** (`CreateFileW`, `FILE_READ_ATTRIBUTES`) on all three pipes,
   so the viola listener and both controls are read by the one instrument (`pipe_type`). Whether NPFS reports the bit
   through `NamedPipeType` on a client open is decided on CI by the two controls (plan acceptance 1).

## Gates
- Entry 8 (the witness by name): red first on a bare scoped default (`left: []`, exit 100), then green on the helper
  (`1 test run: 1 passed, 3005 skipped`) — `chunks/…/evidence/capture-witness.md`.
- Entry 9: `last line 1` — the one site left is the helper's own (`src/main.rs:287`).
- Entry 10 (plain `cargo test --bin viola`): case_4 `ok`, the witness `ok`; `FAILED. 1037 passed; 2 failed` on two of the
  three known-cause reds (`run::gate::tests::feed_panic_…`, `obs::tests::log_config_rejection_writes_one_warn_line`;
  `case_2_sigterm` ok this run) — `chunks/…/evidence/plain-run.md`.
- Entry 4 (Windows-target clippy, check only): green, `Checking viola v0.1.0 (.)` among the checked crates.
- Entry 20 (`pre-push`): green at `linux-tests` in 114.8 s.
- Entry 1 (`/tmp`): 26G avail — `chunks/…/evidence/tmp-reading.md`.
- No deferral: rust delta 10 files; every rust gate ran.
- The `watch:` subject (the macOS readiness stall) has no host gate; it is read on CI's `test (macos-latest)` in the
  operator pass.

### Guard (entry 13), its pass line whole (`gate.py show --n 13`)
```
scope-record.md: absent — no companion is excused
guard: allowed 10 — Cargo.toml, src/cmd/hook/statusline.rs, src/cmd/run.rs, src/main.rs, src/obs.rs, src/run/budget.rs, src/run/dialog.rs, src/run/gate.rs, src/run/send/test_support.rs, tests/security_negatives_channel.rs · companions excluded 0 · changed 10 · excluded 0
guard: clean
```
All ten allowed files are the plan's Files to modify; no file was excluded.

## Scope read
`scope: clean — changed 10 · listed 10 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 434`.
No `scope-record.md` is needed.

## Matrix
`matrix.py show --chunk 2026-10-25-windows-endpoint-admission`: `claimed by …: 0` — no ref written (the plan: `matrix#v1-44`
is claimed on no part; its note is P5's).

## Surfaced
None at /implement. The Windows witnesses' truth is CI's `test (windows-2025)` reading (operator pass).

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola run` wrapper + fake agent, session `p-wea-smoke` | a gate entry (15, boot) | terminated — entry 19 `cleanup` read `"processes_gone":true`, `"endpoint_gone":true`; census at 00:00Z lists none |
| cargo / nextest / rustc of the block | the gate entries | terminated — census lists no viola-tree process |
| `cargo-nextest` pids 1994712, 2022615 (`--package=blitz-dom@…`) | not this run (another project's build, ppid 1563122) | left running — not this run's; its owner stops it |

Census (`gate.py census --comm fake-agent --comm cargo-nextest --args projects/viola/target/debug/viola --args p-wea-smoke`,
trail `gate-2026-10-25-windows-endpoint-admission.json`): `census: 2 running`, both the rows above.

## Decisions & corrections
- The operator's word at invocation (2026-10-25T23:49Z, this session): run the chunk through its operator pass and the
  CI read; `supply-chain` red is expected (owned by "tokio RUSTSEC-2026-0386 lock bump"); the `windows-2025` leg is the
  witness; journal each long run in the run dir before waiting; keep every subagent and workflow transcript (none was
  spawned at /implement).
- Guard refusal: the session's Bash guard refused a `cd` into the skill's references dir (`cd … moves the session cwd`);
  re-issued once with absolute paths.
- Hook refusal: the `stamp-ahead` PostToolUse hook refused a journal line stamped 23:58Z at a 23:51Z clock; the stamp
  was re-read with `date -u` and corrected. Standing rule met: a stamp comes from the clock, never an estimate.
- Plan-premise correction: deviation 1 above (research said no new windows-sys feature; `Win32_System_IO` was needed).
