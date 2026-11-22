# Founder forks — the phase of «Epoch 4 cleanup» (`2026-11-22-epoch-4-cleanup`), P3's one block

**This block holds NO fork of the operator's: P4 follows at once, without waiting.** Every lean below rests on an
artifact named beside it; the P5 card shows each again for the word.

- **Window:** the phase run `.andromeda/runs/2026-11-22T18-00-22Z-phase`, one builder session, P1 18:00:22Z → this
  block 2026-11-22T18:43:20Z (`date -u`); the directive snapped `inputs#I1`.
- **Host:** `/proc/pressure/io` before each timed measurement (io `some avg10` 0.00–0.02 each time); `hostwatch.py
  read --from 2026-11-22T18:20:00Z --to 2026-11-22T18:45:00Z`: `CONTENDED - 15 s stalled on IO` (18:36:51Z–18:37:06Z,
  18:39:06Z–18:39:11Z), `build tools alive in them: viola 15 s` — this phase's own measurement builds alone; escher
  alive 10 s in no stalled second. (A first search for the tool missed the overseer's path; corrected here, the block
  rewritten before P4 read it.) No live start, no rig start of a real `claude`, no mutation run, no witness run was
  made.

## The measurements (each from a test or a no-start probe; detail in `doc:2026-11-22-epoch-4-cleanup/research`)

1. **The advisory.** `cargo deny check advisories` (cargo-deny 0.20.2), 2026-11-22T18:34:06Z, the advisory database
   fetched at 18:34:07Z (HEAD `d1fe6ac3`): `advisories ok`; RUSTSEC-2026-0543 reads `patched = []`, cargo-deny's own
   line `Solution: No safe upgrade is available!`, crates.io's newest chrono `0.4.45` = `Cargo.lock`'s. → **none**:
   the ignore stays, the re-read re-pinned to the Epoch 5 boundary (the operator's), `deny.toml:19`'s reason line
   unchanged. An ignored id that matches no crate reads `warning[advisory-not-detected]`, never an error (a no-start
   probe over a copy of `deny.toml`).
2. **The claimed-line wait — reproduced.** Under a failing snapshot write (its directory removed after init, no
   product seam): `append_err=true · state_claimed=true · lines_on_disk=0`, `confirm` not returned within 3 s with its
   deadline 60 s past, a later send `not-delivered` / `turn-running`; `settle(None)` then ended it at once.
3. **Verify's agents read.** Quiet, the fake agent's `agents --json`: 171 ms first, 108–113 ms after (10 runs). A
   stall cannot be produced here without loading the neighbours (shared disk); a slow fake `claude` would show only
   the kill mechanism the unit cases already show. The recorded red: 26 s stalled of 31 s. **Supported bound: the
   standing 5 s** (~29× the first quiet read); no measurement supports a raise, and no bound under the 20 s test lint
   outlasts a 26 s stall.
4. **A SIGKILLed verify** (test-owned home and user home, fake agent): the record appeared at 5 891 ms, `SIGKILL`,
   then the config held the sentinel unchanged plus `<tmp>/viola-verify-SSgqV5 → {"hasTrustDialogAccepted": true}`,
   nothing else. The fake tells viola's own writes only; the real CLI's own per-cwd `projects` entries and the found
   entry's writer stay unmeasured.
5. **The boot supervisor test, quiet:** `PASS [2.221s]`; its three copies (182 MB, into tmpfs) 60–139 ms;
   `ABORT_DEADLINE` 20 s = the test's `ci` kill (10 s × 2).
6. **(item 6)** under `--profile mutants`, fully parallel, warm: the two drift replay tests 7.005 s / 7.010 s of a
   2019-test, 61.9 s run — each a whole `viola verify`, the `verify_window_` class by `tests/support/verify.rs:564`.

## The mends, each with the test that holds it (P4 writes them as steps)

1. **The claimed line's end — a settle on the early return**, not a deadline for `Claimed`: `append_hook_event`
   settles a claimed line `None` when the snapshot write fails, so the send waits on as `Waiting` and its window's
   existing expiry ends it `not-delivered` / `no-prompt-submitted` (a word that stands). Why: it adds no closed word
   and no time bound, and it keeps the one rule that only `settle` resolves a claimed line (`Flight::claim`'s doc).
   Held by a unit case in `src/run/send/slot.rs` on the P3 reproduction, with its remove-the-guard run.
2. **The seven `viola-agent-claude` survivors:** five killed by cases in the crate the mutants run —
   `env_names_fault` ×3 (`test_cases_checks.rs`: a `Some` case; a `user = ""` case over a name with an empty part),
   `attempted` `&& → ||` (`test_cases_stamps.rs`: a `v: 1` document of a failing shape reads `StampsMalformed`),
   `option_shaped` `< → <=` (`screen.rs`: a dot-led, digitless row before a modal literal is not option-shaped). **Two
   are equivalent, said plainly:** `ledger.rs:1327` (the widened slice adds only the PreToolUse itself to a
   PermissionRequest search) and `screen.rs:135` (the added row holds a modal literal, so it is never a rule row): no
   input separates them, the project's rule forbids reshaping code to drop a mutant (testing.md 2026-09-28), and the
   next boundary's audit reads them as these two named equivalents.
3. **The three paste-writer survivors**, cases in `crates/viola-pty/src/pump.rs`'s own test module: a queue still full
   before its deadline is not cut at once (`:191`); `end_writer` waits its bound for a running writer and returns
   (`:337`); the exit drain runs when output is not yet done (`:372`) — each wait bounded below the kill line.
4. **The agents read's own bound:** a named constant of `src/run/agents.rs` in place of the version probe's
   `VERSION_DEADLINE`, **at the standing 5 s** (measurement 3), for every agents read (list, revive, verify — one
   `read_logged`), a unit case pinning it. No value moves: security-plan `decision/claude-agents-read-fail-open`
   already reads «killed at 5 s», and both a11y lists already name «agents read» as a member.
5. **The boot test:** its three binaries hard-linked into a test-owned directory beside the build dir (same
   filesystem) in place of the 182 MB copies; `ABORT_DEADLINE` 20 s → 15 s, strictly below the 20 s kill (a test-only
   harness bound lowered, never raised; with the 2 s readiness deadline the test's worst case is 17 s).
6. **The `mutants` profile and the two drift replay tests:** both taken into the `verify_window_` class's override
   filter in `.config/nextest.toml` (its own kill, both profiles), by the class rule they already fall under; no
   default kill moves.
7. **A SIGKILLed verify:** measured — it leaves its own trust record, a residual security-plan already names
   (`decision/nothing-written-under-claude-home` and three siblings). **No mend**: a sweep of stale records would be a
   new viola write into the user's real config. The wrap's rows carry the measurement.

At discretion: **(8) not taken** — `framing_turns`' step function and `send_through`'s section: a refactor of verify's
largest function with no live witness allowed in this chunk; the next boundary's audit reads M1. **(9) not taken** —
the relied-row count and the stamped-set table: no relied row is added on Epoch 5's route before its boundary; the
next boundary's audit reads M3. **(10) taken** — `testing.md`'s `paths:` gains the `test_`-stemmed source files
(`src/**/test_*.rs`, `crates/*/src/**/test_*.rs`), a harness edit through `rec.py harness`.

The two rows the boundary wrap named as possibly short, read: test-plan `decision/never-parse-rendered-child-screen`
still says the readiness gate «is tested through its refusal outputs», short of its sibling
`decision/readiness-gate-tested-by-refusals` (which names the one delivered case); layout-templates
`contract/viola-run/start-output` says «a key-on start's automatic verify line», singular, where obs-plan and a11y-plan
rows say «automatic-verify lines». Both short; neither is reached by this chunk's change — named for this chunk's wrap.

## Every reached row (all seven masters searched, `rec.py find` per master; the terms in research)

- **Reached:** security-plan `decision/advisory-ignore-list` and obs-plan `decision/direct-chrono-dependency`
  (re-read recorded, words unchanged) · security-plan `decision/claude-agents-read-fail-open` (its bound now the
  read's own, value unchanged) · security-plan `decision/nothing-written-under-claude-home`,
  `decision/no-writes-under-claude-home`, `decision/viola-writes-nothing-in-claude-dir`,
  `decision/verify-probe-dirs-cleanup` (the SIGKILL residual, now measured) · obs-plan
  `contract/confirm-window-close/clear` and `/compact` (a claimed line whose snapshot write fails settles unconfirmed)
  · test-plan `decision/verify-window-80s-kill` (two more tests in the class) · test-plan
  `decision/boot-deadline-not-a-budget` and `decision/viola-e2e-30s-mutants-kill` (read; the abort wait is not the
  20 s boot deadline they name).
- **Read as not reached:** a11y-plan `decision/time-bounds-fall-on-the-driver` and `decision/time-bounds-never-on-human`
  (they already name «agents read»; **no new time bound is added**, so neither full cell — 297 and 291 of 300 — gains
  a member) · a11y-plan `decision/claude-agents-read-fails-open` · architecture `contract/board/agents-read-failure`,
  `contract/list-payload/agents-read-failure` · design-system and layout-templates agents-read board rows · test-plan
  `decision/session-live-read-fails-open`, `decision/mutants-kill-below-cargo-mutants-timeout`, `decision/root-wait-bound`,
  `decision/mutants-tested-by-owning-package` · obs-plan `decision/session-live-check-fails-open`,
  `contract/diag-subject/agents-probe` · architecture `decision/chrono-timestamps`,
  `decision/chrono-utc-millisecond-timestamps`.

## Forks of the operator's

None. No new closed word, no new time bound's value (the agents read keeps 5 s; `ABORT_DEADLINE` is a test-only
harness bound lowered), no product behaviour a user sees changes beyond a send that hung now ending with its existing
word, and no split.

## One implement window

Yes: one product mend of a few lines, one constant, eight killing cases, the boot test's links and one bound, one
nextest filter, one rule file's `paths:`.

## Correction at the P4 return of card 1's review (2026-11-22T19:00:06Z, `inputs#I2` item 1; nothing above edited)

Measurement 3 and mend 4 above read as if the 5 s were supported for the real CLI. They are not: 171 ms and
108–113 ms are the FAKE agent's `agents --json`; the real CLI's cold agents read was not measured, quiet or stalled.
The 5 s is kept only because no measurement supports moving it, and a constant keeping 5 s does not mend the recorded
red (26 s stalled of 31 s). That finding stays open, a carry the wrap re-pins, owing the real CLI's cold agents read
quiet and with its disk reads throttled — a start the operator caps, on a host where one child's IO can be limited
(research measurement 7: here the `io` controller is not delegated to the user, so it cannot be).
