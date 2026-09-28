# Scope — 2026-09-28-capability-ledger-and-viola-verify · Capability ledger and viola verify

**Source:** `viola-0.1.0/working-route.md:53` (Epoch 2b — Windows slice I b: events and ledger), taken up 2026-09-28.
**Working entry (verbatim title + hint):** Capability ledger and viola verify — versioned rows with probes and
post-conditions, stamps, scrubbed fixture recording, largest-payload row, transport-only degrade, fake-agent verify in CI

## Narrowed at P4 (operator rulings, 2026-09-28 — validation-1: intent-incomplete, scope amended)
Four forks were put to the operator (the Viola overseer), and the recommended option was taken on each:
1. **Split, live-first head.** This chunk is the HEAD. It builds the ledger rows, `viola verify` (print-mode probes,
   raw-payload capture, sole writer of `stamps.json`, `--record`), `run`'s version gate (`cli_version`/`cli_verified`),
   the `cli` catch-site line, the scrub, the claude-fixture schema with the hygiene walk, and the first recorded
   `fixtures/claude/<ver>/` set. The named TAIL **"Verify-stamped test homes and harness"** is a new markerless entry
   after :53 (written by the wrap). It carries `stamped_home` stamping through `viola verify` (5 root test files), harness
   boot step 4 / `--unstamped` / `verify-failed` / readiness line 3, `supervise --fixtures`, `contract_ledger_probes.rs`,
   `agent-run run --local-live` with its `live-in-ci` refusal (v1-14), and the fake agent's default version moving to the
   recorded one. Folded CARRY 1's `stamped_home` half and all of CARRY 2 therefore move to the tail.
2. **Print-mode probes.** `claude -p <synthetic prompt> --model haiku --plugin-dir <capture plugin>
   --no-session-persistence`. The rows needing TYPED input (the long-paste wrapper, tag escaping, harness prefixes,
   local commands, dialogs S3/S7/S8, screen signatures) get their rows and probes with the chunks that own typed input:
   :58 readiness gate, :60 send, :62 dialogs, :68 real-CLI verify. The rows with no consumer at HEAD (statusline,
   `agents --json` join, plugin precedence, dialog concurrency, turn end without Stop) join their consumers.
3. **Stamps trust deferred.** `run` reads `ledger/stamps.json` without strict-modes until Epoch 6 (:95), with a CARRY
   on :95 and a PREREQ on :62 (no non-`null` dialog decision before the check). **Founder ratified live at 2026-09-28
   12:39:40 in the Viola overseer session (AskUserQuestion), relay: the Viola overseer.** The wrap records it as the
   founder's ratification and does not halt on it.
4. **Live recording in the operator pass.** /implement builds and tests against the fake agent with test-written
   fixture sets. The operator pass runs `viola verify --record fixtures/claude` once against the installed CLI (Haiku),
   and the overseer reviews the scrubbed files before the pre-CI commit.

The sections below stay as the working entry's full intent, for the tail and the CARRY texts to cite.

## What it builds (full intent, pre-narrowing)
The single gate for every CLI-specific behaviour viola relies on (architecture §Established Decisions → [CLI Version
Compatibility]; §Conventions → "Capability ledger as the single gate"), and the one verb that measures it.

- **Ledger rows in `viola-agent-claude`.** A compiled-in, closed set of versioned rows, each with an id, a `viola verify`
  probe and an expected post-condition (architecture :73-89 lists the rows: S3, S7, S8, the R8 identity floor, npm-shim
  resolution, statusline `rate_limits.*` / settings override / statusline shell, harness prompt prefixes, the long-paste
  wrapper, modal and input-box screen signatures, the hook tier map, turn end without Stop, dialog concurrency, the
  `claude agents --json` join field, plugin precedence, local commands, tag escaping). The rows stay Claude-specific,
  so they live only in `viola-agent-claude`.
- **Largest-hook-payload row** (security-plan Amendment 7, "before the `viola verify` chunk"): bytes per hook event kind,
  measured by `viola verify`, checking `MAX_FRAME` (16 MiB) against data.
- **`viola verify`** — the first `cli`-process verb. It runs the probe suite against the resolved `claude` (the real CLI
  locally, using Haiku, never in CI; the fake agent in CI and in the harness), checks each post-condition, and prints the
  result line `stamped <ver>  <n> pass  0 fail` (test-plan :290, :518). It is the **only** writer of
  `ledger/stamps.json` (+ `.lock`, 0600, through `viola_state::fs::replace_private`). Each stamp maps a CLI version to its
  verified behaviours and the values measured for that version (architecture :385).
- **Scrubbed fixture recording.** `viola verify` records the hook payloads it observes as
  `fixtures/claude/<cli-version>/<Event>.<variant>.json` (PascalCase hook event name, test-plan §2). The recorder
  rewrites home paths to `~` and usernames to `<user>` (test-plan :1388) and keeps the NEVER-log floor: no token, no
  R8-stripped `CLAUDE*` value.
- **Version gate in `viola run`.** Before the spawn, `run` runs the resolved child executable once with `--version`.
  Unparseable output counts as an unlisted version. `run` reads `ledger/stamps.json` (under strict-modes) and sets
  `cli_verified` in the snapshot, which today is hard-coded `false` (src/cmd/run.rs:278).
- **Transport-only degrade.** On an unlisted or unverified version viola still types, runs the wheel and emits events,
  but withholds dialog answers, and drivers get `unverified-cli` (architecture :91; security.md: "A non-`null` dialog
  decision requires a `viola verify` stamp for the child's CLI version").
- **Fake-agent verify in CI.** The fake agent answers `--version` and serves the probes. Stamps in test homes come only
  from `viola verify --home <home>` run against the fake agent (test-plan :220, :449, :1352). The contract test
  `contract_ledger_probes.rs` checks that, for each `fixtures/claude/<ver>/` set, the run lists every expected row id
  exactly once, each with its post-condition result (test-plan :967).

## Folded route freight (5 CARRYs, all at annotation positions on :53; `route.py pins` lists 5 blocks)
1. **Fixture hygiene + schema** (from 2026-09-24-fake-agent-and-test-data-fixtures). The `fixtures/claude/*/*.json`
   `#[files]` hygiene walk and the claude-fixture schema land with the first recorded fixture (test-plan §7 Fixture
   hygiene; checker `tests/support/hygiene.rs`). Fixtures are recorded as `fixtures/claude/<cli-version>/<Event>.<variant>.json`.
   The root `stamped_home` (`tests/support/home.rs:162`) is an interim no-stamp seam; here it stamps through
   `viola verify`.
2. **Harness `boot` readiness + `supervise --fixtures`** (from 2026-09-27-hooks-to-normalised-events, its P4 operator
   ruling on fork 3). The harness `boot` readiness check of `events.ndjson` line 3 (`session-start{source:"hook"}`) and
   `supervise` passing `--fixtures` land here, with the first recorded fixtures (test-plan §3 `boot` readiness, :174,
   :526).
3. **Catch-site stderr line** (from 2026-09-24-log-redaction-and-never-log-floor). `viola verify` is the first
   `cli`-process verb, so its catch site prints exactly `error: internal error` on stderr, uncoloured and with no hint
   (obs-plan §7; design-system §Surface: cli → Exit-code phraseology), through `src/human.rs`. The chain goes only to
   the detail file.
4. **WSL root-install narrowing** (from 2026-09-27-browser-verdict-reachability, overseer live ratification,
   operator-only). It is **conditional**: the chunk that first re-provisions the WSL distro takes it. Before that
   re-provision, `scripts/wsl-provision.sh --install-deps` must run only `apt-get install` over the package list an
   unprivileged dry run produced (Playwright 1.63.0 `install-deps --dry-run`), checked against a committed allowlist.
   If this chunk re-provisions nothing, it moves on with the next markerless entry.
5. **E1 authoring rule for host-scoped mutants entries** (from 2026-09-28-cli-output-tokens, operator ruling at its
   wrap). A plan's host-scoped `run --mutants --file` entry may expect 0 missed only when every mutant its diff
   regenerates has a killer that compiles on the Windows host. A shared body whose only killer is `#[cfg(unix)]` or
   `#[cfg(windows)]` is authored against the pre-push/CI union instead (verification-harness.md 2026-09-25, extended).
   This is a P4 authoring constraint, not a build item.

## CI verdict read at Setup (second fold source)
- `9b4f6f4` (the last wrap's flip = HEAD): **green**, checks 18/18, wall 287 s, ci#36408767764. Nothing to fold.

## Boundaries
- In: `viola-agent-claude` (the rows, probes, post-condition checks, fixture scrub), the `viola verify` verb in `src/cmd/`,
  `run`'s version gate and `cli_verified`, `ledger/stamps.json` I/O through `viola-state`, the fake agent's
  `--version` and probe support, fixtures under `fixtures/claude/`, the fixture schema + hygiene walk, the harness
  `boot`/`supervise` changes, `src/human.rs` for the catch-site line, CI wiring for the fake-agent verify.
- Out: the fake-agent drift contract (hook sequences and payloads equal to recorded fixtures, `matcher` evaluation) is
  the next entry (:55). `viola list`/`ui` readers of `verified_cli_versions` are Epochs 5 and 8. Driver verbs
  (`send`/`answer`) that would carry an `unverified-cli` refusal are Epoch 3.

## Premises (closed at P3 against research.md)
- VERIFIED: no ledger code exists. `viola-agent-claude/src` holds only `hook.rs` and `lib.rs`. `cli_verified: false,
  cli_version: None` is hard-coded (src/cmd/run.rs:278-279), and `fixtures/` holds only `fake-scripts/gated-turn.json`.
- VERIFIED: no dialog decision path exists. `hook-decision` always logs `decision_emitted = false`
  (src/cmd/hook.rs:208-218), and `hook.dialog` answers `-32601` (src/cmd/run.rs:98-108). In this chunk the transport-only
  degrade is therefore observable only as `cli_verified` in the snapshot and on the `claude-child` `process-start` line.
  "Withholding" dialog answers is vacuous until the Dialog chunk (working-route :62).
- VERIFIED: recording real fixtures needs a live local `viola verify` against the installed CLI. `claude --version`
  prints `2.1.283 (Claude Code)` on this host (measured 2026-09-28), which is the format the fake agent already mimics
  (fake-agent.rs:85-88).
- VERIFIED: the stamps conflict is resolved (test-plan §7 Seed strategies, §12 `2026-09-24` User review 1): stamps come
  only from `viola verify --home` run against the fake agent.
- [premise-corrected: fake-agent.rs:210-219 fires a hook only from a fixture file, and no spec defines the probe drive]
  The fake agent can only replay fixtures. Against it, a probe checks a recorded set's post-conditions and measures
  nothing, and without a fixture set no hook fires at all. The probe mechanism (how `verify` drives the CLI and how it
  captures raw hook payloads) is undefined in every spec and is a P4 fork.
- Size is carried to P4/P5: the full entry is several times the largest recent code diff (research.md §Open questions).
  At P5 the projected implement size is stated against the 55-65 % windows of recent chunks, and a head/tail split is
  proposed (overseer direction at take-up).
