# Codebase Research — 2026-09-29-h2-conpty-resize-probe

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 12
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full; 5 Session Additions, and 2026-09-27 on the `test(=tests::name)` filter form applies to the seam tests) · `.claude/rules/testing.md` (read in full; 19 Session Additions: 2026-09-27 "force a timing window open with a test-only hold, red before / green after, never sample the race", and 2026-09-28 "a timing red is never fixed by raising a timeout: measure the phases on the runner that failed, timing-only pushes allowed", apply directly)
- **Platform issues consulted:**
  - `ConPTY ResizePseudoConsole input lost keystroke after resize microsoft/terminal issue` (web, 2026-09-29). The fetched rstudio/rstudio#18884 states: "A keystroke typed within ~50ms of a process_set_size RPC was lost on Windows CI (ConPTY + MSYS bash)". Its author found "no loss point in RStudio's input path" and says the drop "appears to happen below RStudio (conhost / the MSYS runtime)". The PR's workaround is detect-and-retype (`retryTerminalInput`), not a fix.
  - The fetched microsoft/terminal#18725 ("ConPTY: Ask for the cursor position after each resize", closed, Terminal v1.24) states: "The request must block the console server … but it must not block further incoming resizes". Its first implementation, PR #19089 (DSR CPR `ESC[6n` after a resize, with the console server waiting for the reply), was merged 2025-08-06 and reverted 2025-08-11 by #19237. The revert says it "may inject the DSR CPR while e.g. a DCS is going on". The second, PR #19535 (`WriteDSRCPR()`, `_lookingForCursorPosition`), was merged 2025-11-18 and "marked for cherry-picking in the Inbox Servicing Pipeline". No timeout, and no behaviour when the terminal never answers, is stated in the fetched pages.
  - The fetched microsoft/terminal#10400 states a resize issued close to a client attach can be ignored ("ConPTY must be the last requested size"; Resolution-Fix-Committed). That was at creation time, not mid-session.
  - The fetched `ResizePseudoConsole` reference page (Microsoft Learn) states only that it "resizes the internal buffers". It says nothing about whether it is synchronous, or about input.
  - Recorded builds. Failing runner (job 108974874287 log header): Microsoft Windows Server 2025 10.0.26100 Datacenter, image `windows-2025-vs2026` 20260922.246.2, provisioner 20260828.587. Host: `ver` 10.0.26200.9457, `conhost.exe` 10.0.26100.8875, `kernelbase.dll` 10.0.26100.9278. The runner's UBR is not in the log, so whether its inbox conhost carries PR #19535 is not measurable from here.

## Files inspected
- `crates/viola-pty/src/lib.rs` (60-200, 230-360, 400-670) — the seam (`PortablePty::resize` at :180-185 with span `pty.resize`; `PtyError` hand-written fixed `Display` at :66-98), `HostTerminal` raw input (`raw_input_mode` :243: line/echo/processed off, VT input on), `host_size` (:332, `GetConsoleScreenBufferInfo`), and the whole real-PTY test rig inside `#[cfg(test)] mod tests` (:361): `pty_child_entry` (:424-467), `report_path` (:513, `<temp>/viola-pty-watch/<test>.report`), `spawn_child_entry` (:525, the output drain thread discards every byte at :552-556), `CHILD_WITHIN` 7 s (:568), `lines` (:571), the red test (:598-639) and the forced-window sibling (:643-669).
- `crates/viola-pty/src/pump.rs` (36-139) — production: `copy` input → writer on a worker thread (:76-80), while the main loop calls `pty.resize(size)` every `RESIZE_EVERY` 250 ms when the host size changed (:101-108). Nothing orders a resize against a concurrent key write.
- `crates/viola-pty/Cargo.toml` — deps exactly portable-pty, tracing, windows-sys (Windows), libc (Unix); dev-deps mockall, tempfile; the `fake-agent` feature is empty.
- root `Cargo.toml:201` — the workspace `windows-sys =0.61.2` features already include `Win32_System_Console`.
- portable-pty 0.8.1 `src/win/psuedocon.rs` (:27-30, :44-62, :82-86, :98-108) and `src/win/conpty.rs` (:54-88) — `resize` is a bare `ResizePseudoConsole` under the master's mutex, S_OK-checked. `CreatePseudoConsole` is called with `PSEUDOCONSOLE_RESIZE_QUIRK | PSEUDOCONSOLE_WIN32_INPUT_MODE`. `load_conpty` prefers a sideloaded `conpty.dll` over kernel32.
- `src/bin/viola-fake-agent.rs:463-473` — `receipt_size` samples `host_size()` at start and "before any byte read after it changed". The fake agent's size oracle is key-gated too.
- `tests/tui_pty_seam.rs:57-73` (`pty_resize_reaches_the_child`: resize, then `a`, then waits for the size receipt) and `tests/tui_passthrough.rs:103-169` (`resize_reaches_the_child`, a timed trail file `viola-resize-<pid>.ndjson`) — both are root key-after-resize paths. Both passed in the red run (job log lines 548, 557).
- `.config/nextest.toml` — the `ci` profile has retries 0 and slow-timeout 30 s × 4. The `mutants` profile (5 s × 2) is now used by no gate.
- `.github/workflows/ci.yml` (job outline) — `test (windows-2025)` runs `agent-run.ps1 run --coverage` (llvm-cov nextest) only. There is no uninstrumented Windows nextest run in CI.
- Evidence records: `chunks/2026-09-28-capability-ledger-and-viola-verify/evidence/h2-ci-red.md`, `chunks/2026-09-27-epoch-2-cleanup/evidence/pty-forced-window.md`, `chunks/2026-09-27-instance-state-and-start-order/evidence/ci-red-36296402785.md`; job log 108974874287 (fetched with `gh api repos/Turbolet85/viola/actions/jobs/108974874287/logs`).

## Graph impact (rust plane, trace `tree-query-2026-09-29-h2-conpty-resize-probe.json`; editor lines = graph line + 1)
- **resize** (viola-pty) — 5 callers: `tests::spawn_runs_a_raw_child_…` @ `crates/viola-pty/src/lib.rs:619` · `tests::spawn_delivers_a_key_…` @ `lib.rs:658` · `pump` @ `crates/viola-pty/src/pump.rs:105` · `cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start` @ `src/cmd/run.rs:574` · `OuterPty::resize` @ `tests/support/outer_pty.rs:80`. A fix inside `PortablePty::resize` reaches production only through `pump` (`pump_child` @ `src/cmd/run.rs:345`) and the harness's outer PTY.
- **pump** (viola-pty) — 12 in-crate test callers + 1 production caller (`cmd::run::pump_child` @ `src/cmd/run.rs:345`). A pump-side fix's contract tests are `pump_forwards_a_host_size_change_once`, `pump_forwards_a_resize_that_lands_before_its_first_look`, `pump_does_not_resize_while_the_size_is_unchanged`, `pump_looks_at_the_size_once_per_period` and `pump_does_not_look_at_the_size_before_its_first_period`.
- **OuterPty::resize** — `resize_reaches_the_child` @ `tests/tui_passthrough.rs:121` · `pty_resize_reaches_the_child` @ `tests/tui_pty_seam.rs:65`.
- **crate_edges** — viola-pty has inbound edges from `viola` and `viola-e2e` only, and no outbound viola crate. A recorder change inside `mod tests` has zero cross-crate blast.

## Patterns detected
- **Kill-proof streamed report** (`crates/viola-pty/src/lib.rs:412-420, 511-523`): the child appends one line per step to a known file outside the test's tempdir. `lines()` polls it under `CHILD_WITHIN` and panics with the report so far (`lib.rs:582`). It is kept on a failure, removed on a pass (`Drop` at :500-509). The localisation lines extend this file.
- **Forced window by a test-only hold** (`lib.rs:408-410, 450-453`, `CHILD_MODE=hold`, 750 ms): the child mode is chosen through the spawn env (`PTY_SEAM_TEST_MODE`), which only the test child reads. It is not a product seam. New child modes go the same way.
- **Resize observed only on a key** (`lib.rs:454-459`; `src/bin/viola-fake-agent.rs:463-473`): both oracles re-read the size when a byte arrives. That is why the red's report could not say whether the resize applied.
- **Output drained and discarded** (`lib.rs:552-556`): nothing in the rig answers a terminal query the ConPTY writes to its output (a DSR CPR `ESC[6n` included). A real terminal (the host behind `viola run`) would answer one.

## Conventions to follow
- **Test naming** `<subject>_<condition>_<expected>` (testing.md §Naming); inline `mod tests` only (testing.md 2026-09-25).
- **Codes and counts only** in the report (`lib.rs:434-459`: pids, sizes, byte hex, booleans). A DSR-CPR observation is recorded as a count, never as output bytes.
- **Waits bounded strictly below a kill, exit-aware** (testing.md 2026-09-24/2026-09-27; `CHILD_WITHIN` 7 s under the 10 s mutants kill and far under the ci profile's 120 s).
- **No product env seam:** the child mode rides the child's spawn env inside the test rig (`spawn_child_entry` :540-543), and no `viola` build reads it.

## New files to create
- `viola-0.1.0/chunks/2026-09-29-h2-conpty-resize-probe/evidence/`

## Files to modify
- `crates/viola-pty/src/lib.rs` — the recorder (a child size watch independent of keys, the test-side sequence, a DSR-CPR count on the drain), the localisation and forced-window tests, and the seam fix if step 3 places the loss in `PortablePty`
- `crates/viola-pty/src/pump.rs` — only on the fix branch, if the loss is placed in the pump's resize-then-write ordering
- `.github/workflows/ci.yml` — only on the CI-probe branch of P4's venue fork
- `.claude/docs/services/viola-pty.md` — the measured H2 finding (both branches)
- `.claude/docs/gotchas.md` — the measured H2 finding, in the gotcha form, on the document branch

## Mechanism claims re-derived
- **Overseer relay: "the recorder stopped after start and byte 78, so the RESIZE itself never reached the child"** — NOT supported by the cited evidence at HEAD. The report prints the size only inside the `byte 79 size=…` line, which is written after the second `read_exact` returns (`lib.rs:454-459`). A missing `byte 79` line therefore says nothing about the resize. The resize may have applied, been ignored, or be pending. The witness (ci#36436266196 job 108974874287) still reads `child report stopped at ["start pid=6880 raw=true size=100x30", "byte 78"]` (job log line 1738 area). The recorded red stands; only the inference from it is corrected. Deciding it is step 1's job.
- **Base rate** — the test failed in 2 CI runs (36296402785, 36436266196) out of at most 32 ci.yml runs since `054ebe4` (re-derived: `gh run list --workflow ci.yml --limit 200`, filtered to `createdAt >= 2026-09-27T00:26Z`, the commit time of `054ebe4`; 25 success · 7 failure; not every run reached the Windows test). That is about 1 in 16 runs, both under `run --coverage` (llvm-cov), both late in the Windows suite (955/956 in the second). The forced-window sibling passed in the same run (900/956).
- **ConPTY post-resize CPR (PR #19535)** — hypothesis, not measured here: after a resize, ConPTY writes `ESC[6n` to its output and waits for a CPR on its input, and the rig's drain never answers. Whether the host's or the runner's conhost carries it is not measurable from the recorded builds. Step 1's DSR count measures it on the host first.

## Open questions
- Reproduction venue and forcing: temporary measurement pushes in the operator pass (the testing.md 2026-09-28 recipe) vs a standing ci.yml probe job vs a host run under a loaded gate. At ~1/16 runs, sampling standing CI cannot reach "a set number of times" in a chunk → blocks: plan-decision.
- Does the host's conhost write `ESC[6n` after `ResizePseudoConsole`, and does answering it (`ESC[r;cR` on the input) change the key's delivery? The answer decides the intervention variant and the fix-vs-document branch → blocks: implementation-scope.
- If the loss is below viola, what may the seam do without holding back a human key (architecture [Human Takeover / Wheel]; a11y-plan §1)? Answering an unanswered CPR is not holding a key, but a retype is → blocks: implementation-scope.
