# Outcome — 2026-11-06-version-done-check (/implement, run `.andromeda/runs/2026-11-06T19-58-45-implement`)

**Outcome: SURFACED.** Every gate entry this skill fires is green, except entry 39, the live round's reader. It reads
`last: 2`: the verify read red and the hold went unread, so two readings are not green. No code fault lies behind
either. The live clauses are surfaced for the wrap and the founder (below). Next → the **operator pass** (entries 41
hygiene, 42 push, 43–49 the seven CI reads), then `/andromeda-wrap-session`.

## Files changed (51, all in research's lists; scope: clean, 0 recorded, 0 widening)

- **Step 1, the dependency move:** `Cargo.toml` (`portable-pty = "=0.9.0"`, the Windows comment block replaced by one
  line); `Cargo.lock` (`cargo update -p portable-pty` only: nix 0.25.1 → 0.28.0, serial2 0.2.39 and cfg_aliases 0.1.1
  added; pin-utils, serial, serial-core, serial-unix, serial-windows, ioctl-rs, memoffset 0.6.5 and termios removed;
  exactly research's delta); `deny.toml` (`ignore = []`).
- **Step 2, the remnants:** 47 files, listed in `evidence/remnants.md`, which names every removed and kept site with
  its reason. Code removed: `plain_drive` (`src/cmd/mod.rs`) and its table; the Git Bash warning, its unit table and its
  CLI witness (`src/cmd/send.rs`, `tests/cli_send.rs`); the `.exe` trim (`tests/cli_output_plain.rs`); the `\\?\`
  strip in `tests/support/home.rs` `resolved`; every `EXE_SUFFIX` join. The rest is comments.
- **Step 5:** `tests/contract_fake_agent_drift.rs` gains `dialog_drift_names_a_changed_annotation`, since no planted
  annotations case stood.
- **New:** `evidence/` holds `remnants.md`, `census.md`, `v1-09-amendments.md`, `v1-26-fix-pass.md`, `ci.md`, the
  copied rig (`live-pty.py`, `live-ledger.py`, `cmp` 0 against 2026-10-19's), `live-sessions.ndjson`, `live-round.md`
  and `overseer-drive.md`.

## Gates (gate trail `gate-2026-11-06-version-done-check.json`, three firings)

- First batch (entries 1–28, 31–36, 50): **green 33, recorded 2, red 0.** It includes `cargo deny check`
  (`advisories ok, bans ok, licenses ok, sources ok`), `cargo tree -i pin-utils` (exit 101, no package), the deny
  probes (`14/14 banned, control clean`), the remnant probe (exit 1, no output), viola-pty 33 tests, unit on 1.96 and
  1.98.1, e2e (censuses, drift, E2 fds, tui) and integration (help, send, budget, links, controls). Entry 1 `df -h /tmp`:
  tmpfs 32G, 361M used, 31G free. Entry 50, the last scheduled `nightly.yml`: success, 2026-11-02T12:34:01Z
  (`evidence/ci.md`).
- Second batch (29, 30, 37–40): perf green and `gate --require perf` green; 37 green (2 starts); 38 green (0 left);
  **39 red** (`last: 2`); 40 `pre-push` green: suite `coverage` 3367 passed, 0 failed, and `gate --require
  coverage,doctest` reports no breach.
- No gate was deferred: Rust delta is present, so every gate ran. Census by its rule: `GATES` 41 rows, 39 functions,
  2 files, as predicted (`evidence/census.md`).
- No fix iteration was needed: no red came from code.

## Smoke

Smoke passed: entries 31–36 (cleanup, boot, status, G2, G4, cleanup) ran green as P2 gates on this tree. Beyond them,
the live wrapped start booted real claude 2.1.287 and read `session-start` within 5 s.

## The live round (`evidence/live-round.md`, ledger `evidence/live-sessions.ndjson`): 2 starts of 2, no key

- **Session 1, the verify: RED.** It ended `stamped 2.1.287  23 pass  1 fail` on row `statusline-rate-limits` and was
  not re-fired.
- **Session 2, an unverified wrapped start** (`inputs#I6`):
  - start green: `openpty`, start order, socket dir 0700 and socket 0600, pinned copy, fresh plugin;
  - fd table green: fds 0–2 on the pty, no viola socket, no home file;
  - killed wrapper green: child gone 0.401 s after SIGKILL, no self `process-exit`;
  - hold **unread**: an unverified CLI holds no dialog.

## Surfaced (author nothing; the wrap and the founder rule)

1. **The `statusline-rate-limits` row reads the account, not the CLI.**
   - The row demands two whole-number `rate_limits` windows (architecture's capability ledger; the
     `budget-paused` refusal rests on it).
   - This builder session's own wrapped status line, read 2 s before, carries `five_hour` `unknown` and only
     `seven_day`. So on an unchanged CLI 2.1.287, verify reads red whenever the account's payload lacks a window.
   - Spec ↔ reality: the row's premise (a CLI behaviour) vs. what it measures (the account's plan state now). The
     overseer named it a finding (`inputs#I6`).
2. **CARRY 2 (the 3600 s hold) stays PROVISIONAL.** It was not read, because the verify was red. Its owner, per the
   plan's note: 0.2.0's first entry, named at the wrap.
3. **v1-27 is unproven and has no ref.** Two clauses are unread:
   - `process-start{… cli_verified:true …}` — the stamp is red;
   - «no byte of viola's own on the rig's pty» — the rig counts bytes (2 502 drained) and keeps no screen to compare.

   Every other clause of v1-27 read green in session 2. Per `inputs#I6`, the verified clause goes to the founder at
   the wrap.
4. **The T3 witness reads `link` 0** (`evidence/overseer-drive.md`). The driver's sends are on record: 242 issued, 238
   confirmed, 175 driver-origin prompts. But they carry no other instance's name, so the plan's wanted `link` kind is
   absent. That is a fact about the driver's setup, not the product.

## Matrix

- **Implemented (7):**
  - v1-03: the drift contract, the new annotations case, fixture hygiene;
  - v1-09: by construction, `v1-09-amendments.md`;
  - v1-10: the send-record tests and `wait_log_only_kinds_never_wake`;
  - v1-13: `path6_release_budget_lifts_the_pause`, `link_verbs_print_the_design_lines`;
  - v1-24: `passthrough_with_a_statusline_source_adds_no_viola_bytes`;
  - v1-26: manual, `v1-26-fix-pass.md`;
  - v1-35: path6 ×3, the pass-through, the BAY unit.
- **Waiting on the operator pass's CI reads (5):**
  - v1-01: the seven jobs on the pushed sha;
  - v1-04: G4 in CI `test`;
  - v1-05: G2, G4 and the secret scan in CI `test` / `perf`;
  - v1-07: coverage in CI `test`, `fuzz-replay`;
  - v1-53: `junit-ubuntu-latest` on the pushed sha. Its live clause is green from session 2.
- **Unset (1):** v1-27 (surfaced, item 3).

## Deviations from the plan, with their justifications

- **Step 10's dialog:** the hold would read a **question** dialog, not a permission dialog. The user-level
  `~/.claude/settings.json` allows `Bash(*)`, so a shell command raises no PermissionRequest (`inputs#I5`, the
  overseer, 20:10Z). Not exercised in the end: no hold could be read.
- **Step 9's stop:** after the red verify, one **unverified** session 2 was made instead of none (`inputs#I6`, the
  overseer, 20:16Z: the founder's cap is two starts and his word is to prove v1-53).
- **No `<vhome>/config.json` was written:** an unverified start holds no dialog, so `dialog_hold_s` has nothing to act
  on. The Write tool's guard also refuses files under `target/`.
- **The fixed prompt was never sent:** it lives in the session scratchpad, not under `priv/`, because the write guard
  refuses `target/`. Its sha256 prefix is `bca8c40a6e1ea793`.
- **The start's cwd `priv/cwd/` lies inside the work tree:** the CLI trusts it through the repository root's entry,
  walking up to the git root (read in the 2.1.287 binary). No trust modal showed.

## Process hygiene

| process | started by | final state |
|---|---|---|
| `viola verify` + its 5 PTY children (session 1) | this run | terminated (exit 1; census 0 after) |
| `live-pty.py` → `viola run done-check` → `claude` 2.1.287 (session 2) | this run | terminated: the wrapper by this run's SIGKILL, its child 0.401 s later, the rig on its child's end (census 0 after) |
| harness `supervise` + wrappers (`p-linux-smoke`, perf session, test sessions) | a leg (gate entries) | terminated (cleanup `processes_gone` true; the round census 0) |
| 4× `viola-lab/prototype/target/debug/viola run …-builder` / `andromeda-worker`, 1× `viola hook` | NOT DERIVED (another tree's bridge, the overseer pair's) | left running: not this run's — their owner stops them |
| `~/.viola/bin/…/viola mcp`, `~/.local/viola-product/bin/viola wait viola-builder …` | NOT DERIVED (this session's own wrapping and its driver) | left running: not this run's |

`gate.py census` read 7 running; none is this repository's build. The round dir
`target/done-check-live-20261106T201225Z/` stays on disk (gitignored) for the founder's desk.

## Decisions & corrections

- The overseer, founder-delegated, 20:10Z: the hold reads a question dialog, not a permission one (`inputs#I5`).
- The overseer, founder-delegated, 20:16Z: one unverified session 2 after the red verify; surface the
  `statusline-rate-limits` red as reading the account's plan; no third start (`inputs#I6`).
- `gate.py run --only` takes `N,N` only. A range (`1-28`) died with a ValueError traceback instead of a usage exit 2.
  That was my input error and nothing was patched; I re-fired with an explicit list.
- Guard refusals, each re-issued once in the guarded form:
  - bash-guard refused a `cd` into the inputs dir and a `rec.py find` piped into `grep`;
  - write-guard refused a Write under `target/`; the prompt moved to the scratchpad.
- The user's settings allow `Bash(*)`, so a live plan that needs a PermissionRequest cannot get one from a shell
  command on this host. Measured here; a candidate learning for the wrap.
