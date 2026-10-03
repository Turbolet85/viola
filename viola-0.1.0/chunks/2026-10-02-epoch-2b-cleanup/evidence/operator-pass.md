# Operator pass — 2026-10-03

Run on the overseer's word ("run the operator pass now"), after /implement run `2026-10-02T19-38-47-implement`.

## Entry 23 — `gate.py hygiene`
- First reading: `refused 4 files — P1 4`. These were the phase run's dry-run captures
  (`.andromeda/runs/2026-10-02T12-57-04-phase/p4-dryrun.txt`, `p5-dryrun.txt`, `p5-dryrun-2.txt`, `p5-dryrun-3.txt`), each
  at line 2 with ×4 hits.
- Redacted on the overseer's word, with the files kept. Each file had 4 host paths outside the repository: the resolved
  shell (→ `<git-bash>`), the gate log dir under the user temp dir (→ `%TEMP%\`), and the two `leg = 'operator'` command
  texts naming the skills dir under the user profile (→ `~/.claude/skills/`). The rewrite was binary-mode: every other
  byte was kept, and the CRLF counts were unchanged (34 · 38 · 38 · 35).
- Re-read: `hygiene: clean — read 48 (runs 40 · evidence 8) · trails 11 not read · binary 0 not read by P1`.

## Entry 19 — `agent-run pre-push` (taken at /implement, on this tree's code)
- Stages:
  - `tools`, `sync` (63 files), `cache` (19.7 GB of a 40 GB cap, not cleaned): passed.
  - `linux-tests`: **green** (coverage 951 passed / 0 failed · Playwright 1/0 · `gate` ok).
  - `vm-release`: terminated.
  - `windows-tests`: **red**, coverage 907 passed / **87 failed**.
- **Basis:** every `windows-tests` failure is the M2 class, a `watch.rs` deadline or the hook's spine bound on D:. It is
  two-sided witnessed, same command, both orders, same day: D: 22 passed / 40 failed against the C: copy 62/0, then D:
  22/40 against C: 62/0 (`m2-diagnosis.md`). M2's cause is the D: volume's per-operation filesystem latency, outside the
  repository; the founder decides it. Per the host-reds CARRY on `working-route.md:66`, **CI is the acceptance leg**. On
  the overseer's word this red is recorded and not chased. Not this chunk's red.
- The code under the pre-push equals the committed code. After the pre-push, the only code edits were a `verify.rs` change
  that was reverted (byte-equal to HEAD) and `home.rs` swapped for the leak arms and restored (byte-equal to the gated
  file). Both were confirmed by `cmp` / `git diff --quiet`.

## Entries 24–25 — push and the CI read
- **Pre-CI commit** `e848944` (65 files). **Entry 24:** `git diff --quiet && git diff --cached --quiet && git push origin
  HEAD`, exit 0: `e0fbc72..e848944 HEAD -> build/viola-0.1.0`.
- **Entry 25:** `ci.py conclusion --sha HEAD --wait 1800`, which returned at the first failure (polled 2× over 33 s):
  `e84894476d76 verdict: red · checks 15/15 · first-fail +23 s lint (macos-latest) · runs ci#37106821284 in_progress`.
  - Failed 2: `lint (macos-latest)`, `lint (ubuntu-latest)`. 11 jobs were still running at the read.
  - **Cause** (job 111156869351's log): `error: unused import: remove_owned --> tests/cli_instance_state.rs:19:5`. The import
    is used only by the `#[cfg(windows)]` case `remove_owned_keeps_the_owner_record_while_a_file_is_held`, so
    `-D warnings` fails it on every non-Windows target. Windows clippy (local, and gate entry 2) cannot see it, and the
    pre-push's Linux leg runs tests, not clippy.
  - This chunk's red. It is not fixed here: the overseer directed "report the CI verdict and stop".
- **ci#37106821284, completed** (left to finish on the overseer's word, so one fix covers every red): conclusion
  `failure`; 13 of 15 jobs green, including `test (windows-2025)`, `test (ubuntu-latest)` and `test (macos-latest)`. The two
  red jobs are `lint (ubuntu-latest)` and `lint (macos-latest)`, each with the same single error, the unused `remove_owned`
  import above. No other red.

## Fix — folded into this chunk (overseer, 2026-10-03)
- `tests/cli_instance_state.rs`: `remove_owned` leaves the top-level `use support::home::{…}` and is imported inside the
  `#[cfg(windows)]` case that uses it.
- **Linux clippy, local, two-sided** (WSL `Ubuntu` through `scripts/wsl-exec.sh`, the operator's launcher, in pre-push's
  clone `~/viola-pre-push`, `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`):
  - on `e848944`'s code: exit 101, `error: unused import: remove_owned --> tests/cli_instance_state.rs:19:5`, CI's exact
    error (the known positive);
  - with the fixed file copied in: exit 0 (`Checking viola`, `Checking viola-e2e`, `Finished`).
- **Windows, local** (gate entries 1, 2, 7, 8): fmt, clippy, the frozen-stale case and the deadline lint, all green.
- **Fix commit** `9e3b850`. **Entry 24:** pushed, `e848944..9e3b850 HEAD -> build/viola-0.1.0`, 0 ahead.
- **Entry 25:** `ci.py conclusion --sha HEAD --wait 1800` (polled 11× over 311 s):
  `9e3b85030d67 verdict: green · checks 15/15 · wall 290 s · runs ci#37107107417 completed/success`.
  **The final HEAD run is ci#37107107417, green, and it is the acceptance leg** for the host-reds CARRY.
