# P5 review — 2026-10-26-home-and-code-bearing-file-integrity · «Home and code-bearing file integrity»

Plan: `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/plan.md` (493 lines, about 280 of prose outside
the gate fence) · research: `…/research.md` · scope: `…/scope.md` · run dir `.andromeda/runs/2026-10-26T16-59-13-phase`.

## Goal
Every process that reads or opens anything under the viola home first refuses a home, instance tree, code-bearing folder
or diagnostics file another user could write (or read, where content lives), before its first open. The check lands
at the six CLI frame verbs, `hook`'s nine events, `mcp`'s frame tools, `verify`, `run` / `revive`'s start and the
diagnostics opens, which retires the eight dated strict-modes gaps. The master's read rule (Unix `0o077`; Windows read
grants) lands too. By the founder's own answers (`inputs#I3`):
- the four snapshot readers move onto the replay, after the check;
- a refused check writes no line;
- the six verbs print `link`'s instance pair;
- the `pin_exe` race and build reproducibility go to a new entry.

## Top acceptance criteria
- The six frame verbs: a widened instance tree → exit 21, `link`'s pair or the `instance-unreachable` document, no
  frame, no line, the mode or ACL as found, on all three OSes (`binary(security_negatives_strict_modes)`).
- `hook`: refused → exit 0, empty streams, no frame, no line, no SessionEnd append. `mcp` tools → `instance-unreachable`
  with no line.
- `run` refuses a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy with exit 1 and its pair before any log.
  `verify` refuses a widened `ledger/`, and `update_stamps` checks before it reads.
- The read rule: unit tables on every OS. On `windows-2025`, a tree granted Users write is refused beside a
  `create_private_dir` tree that passes.
- The `conpty/` leg: a refused companion degrades to the inbox ConPTY (`sideload_fallback` `strict-modes-failed`). It
  never refuses.
- Single writers, umask-independent modes and the Windows `--home` outside the profile (test-plan §6, owed until now).
- E5 steps 1–3: a cut-short or `v` 99 snapshot is replayed at `run`'s next start, and the hook, statusline and
  `live_endpoint` readers take the replay.
- `seed_conpty` homes pass on Windows (the three `hook_statusline` cases of CI run 38080061631). The plain home's SDDL is
  recorded.
- `run --perf`: the five hook rows stay under `SPINE_DEADLINE` with the check in the timed path. `pre-push` and CI are
  green.

## Touchpoints
- **viola-state:** `strict.rs`, `stamps.rs`, `liveness.rs`, `pin.rs`
- **viola-mcp:** `tools.rs`
- **Root bin:** `client.rs`, the six verb files, `link.rs`, `hook.rs`, `hook/statusline.rs`, `run.rs`, `revive.rs`,
  `verify.rs`, `obs.rs`
- **Schema:** `schemas/diag-line.v1.json` (one `sideload_fallback` word)
- **Tests:** 2 new binaries and 8 edited files (`tests/support/home.rs`'s `seed_conpty` among them). No module added, no
  dependency added.

## Test commands (26 entries)
- `df -h /tmp`, fmt, clippy, the Windows-target clippy, G1, lint-probes.
- `run --unit`.
- Six nextest entries by binary name. The two new ones read red at baseline (exit 94, files not written yet; must-pass
  via entry 10's form, «43 tests run»). The others read green: 43, 95, 83 and 8 tests.
- `cargo deny` (green).
- The guard (green; controls clean / allowed-green / outside red, `p5-controls.md`).
- Smoke cleanup, boot, status, G2, G4 and cleanup (green).
- `run --perf`: max 2.5–2.9 ms per row.
- `pre-push`: green, 3133 passed.
- Operator: hygiene, push, `ci.py conclusion`.

No live leg, no mutation entry.

## Concretization previews (claims at step 4, after the word)
- **v1-45** (integration). Current: «The home is created owner-only; every process that trusts a state file first refuses a
  home or file writable by others (or readable, where content lives); the pinned binary is re-hashed before reuse;
  plugin and settings files are rewritten at every start; only viola verify writes stamps and only the owning wrapper
  writes a snapshot; the statusline command never runs from a home that fails these checks.» Proposed concrete form:
  - On all three OSes, a home, instance tree, `ledger/`, `bin/`, `plugin/`, pinned copy or diagnostics file another
    user could write is refused before any open (Unix 0770 / `0o077` on the home and instance dir; Windows a Users write
    or read ACE). The verdicts:
    - `send` / `wait` / `last` / `answer` / `pause` / `release`: exit 21
    - `hook`: exit 0, silent
    - `mcp` tools: `instance-unreachable`
    - `run` / `verify`: exit 1
    - every case leaves the mode or ACL as found
  - The home is 0700 / 0600 under `umask 000` (Unix) and has the protected user + SYSTEM DACL outside the profile
    (Windows).
  - A tampered pinned copy refuses `run` with exit 1. `plugin/` and `settings.json` are rewritten at every start
    (`path1_run_start`).
  - `stamps.json` and `snapshot.json` stay byte-identical after every other writer-capable entry point runs.
  - `statusline_command` never runs from a refused home (Unix and Windows).
  - Witnesses: `security_negatives_strict_modes`, `hook_fail_open`, `hook_statusline`, `run_cli`, `cli_verify`,
    `handlers`, `path1_run_start` and the strict unit tables, on CI's three legs.
- **v1-17** (integration). Current: «The architecture names both diagnostics roots and every process's sink; on every OS
  the home is created and passes its strict-modes check before any diagnostics file is created or opened; existing
  diagnostics files receive the same owner, mode and symlink checks as other state files; content-bearing detail
  exists only per instance.» Proposed concrete form:
  - The architecture names both roots and the four role sinks plus the instance detail file (architecture.md:323,
    §Occupied Resources — an enumeration implement records, by construction).
  - On every OS, every entry point's check precedes its first `viola_obs_init` / `ensure_private_dir`, with each
    refusal leaving `diagnostics/` absent or as found.
  - `open_role_file` and `write_detail` refuse a symlinked, foreign-owned or widened `diagnostics/` file (the unit cases
    of step 1 and step 9).
  - Detail lines are written only under `instances/<name>/diagnostics/`.
- **v1-41** (e2e). Current: «Logs are append-only one-line records that are never truncated; snapshots are replaced
  atomically; a torn last line or an unreadable or newer snapshot is healed by replay that recovers only log-derived
  fields; unknown kinds and fields are counted and shown rather than failing; a newer peer's request is refused with
  the supported version.» Proposed concrete form: the landed legs keep their witnesses — the torn-line heal, the three
  skip counts on `list --json`, `state_replay.rs`, revive and list as replay readers, and the newer-peer refusal. To
  them this chunk adds:
  - E5 steps 1–3 (`chaos_snapshot_replay`): a cut-short or `v` 99 snapshot is replayed at `run`'s next start, with one
    `state-recovered` line, the logged `links` and no start refusal once the beat is stale.
  - The four remaining readers take `read_snapshot_or_replay` after `check_instance`, each recovering only log-derived
    fields; a replayed snapshot yields no frame.
  - The 0.2.0 page is out of 0.1.0; `list` is the surface that shows the counts.

## Sampler (forced, the operator's word, `inputs#I1`)
- architecture: `sampler: E carried 89 · the cascade lost 5 (established-decisions#17.1, established-decisions#9.1, occupied-resources#6.1, stack-and-technologies#1.12, stack-and-technologies#1.23) · listed 3`
- security-plan: `sampler: E carried 108 · the cascade lost 5 (authentication-authorization#3.6, authentication-authorization#3.7, secret-management#1.1, threat-model-summary#1.5, threat-model-summary#9.1) · listed 4`
- design-system: `sampler: E carried 16 · the cascade lost 0 · listed 1`
- test-plan: `sampler: E carried 100 · the cascade lost 3 (ci-integration#1.2, ci-integration#1.9, test-strategy#1.7) · listed 4`
- obs-plan: `sampler: E carried 95 · the cascade lost 2 (error-capture-reporting#1.1, metric-coverage#1.1) · listed 7`
- a11y-plan: `sampler: E carried 30 · the cascade lost 0`
- The plan's `(sampler)` items stand in its Implementation notes: 15 items, each with the step or entry that holds it.

## Notes for the review
- **Founder forks answered at P4** (`inputs#I3`, his own word, live, 2026-10-26T18:11Z, relayed by the overseer):
  - Q1, readers move: his ratification of a boundary widening. The wrap halts once on it as the answer card (the
    playbook's «Boundary widening the founder answered himself to a P4 fork»).
  - Q2, no line.
  - Q3, `link`'s pair.
  - Q4, scope cut.
- **Decisive leans** (Provenance):
  - A refused `conpty/` companion degrades, never refuses (architecture [PTY]).
  - `run` and `verify` reuse the ratified home and instance pairs. No new copy.
- **Size:** above the ~400-line guideline in total; within it in prose.

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | Insert a new working-route entry «Pinned copy race and reproducible builds» right after «Home and code-bearing file integrity» in Epoch 6. It carries CARRY 16 (the `pin_exe` first-start race, chunk 2026-10-17-mcp-server-for-drivers) and CARRY 11 (two clean release builds of one tree compared by file and by section, chunk 2026-10-09-inner-cr-and-crlf-in-a-sent-text), each verbatim from this entry's line, on the founder's route-split word (`inputs#I3` Q4). | route insertion |
| 2 | CARRY 17 (the `marketplace/` tree's integrity) closes with no work and no owner: the founder's Epoch 5 batch (2026-10-20) made «`marketplace/` off the integrity set» his word. | disposition (closed by ruling) |

Apply? (yes / review / cancel)
