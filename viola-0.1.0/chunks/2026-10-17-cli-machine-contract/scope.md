# Scope — 2026-10-17-cli-machine-contract · CLI machine contract

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry at take-up, eleventh under
`### Epoch 5 — Driver surface`, directly after "End-to-end path binary and harness legs"): CLI machine contract —
--json per verb mirroring channel results, typed exits 0/1/2/10–14/20/21, cause-named hints, global --home,
config.json thresholds — plus three CARRY blocks (folded below; completeness check `route.py pins --run-dir` → this
run dir's trail: three rows on the entry, CARRY 274 · 696 · 354 chars; no `BLOCKED-ON`, no `PREREQ`, no `CONTEXT`, no
`WATCH`).

**The operator's direction at take-up:** none beyond the invocation (`/andromeda-phase`, no flags). During P2 the
operator folded the last wrap's red CI into this chunk (§7, `inputs#I1`, snapped at P3). Every other fact below is
read from the entry, from the masters (`architecture.md` §Conventions CLI exit codes, §Standard Contracts CLI `--json`
output, §Cross-cutting Config management), from the verification matrix, or from a file of this repository. The
earlier chunks' snapshots the CARRY text cites are spelt by those chunks' paths (P5 check 10).

**CI read at Setup 5a** (the last wrap's flip `f6bcce5` = HEAD, `ci.py conclusion`, this run dir's trail):
- `f6bcce5ba21a`: **in progress** at Setup — `CI f6bcce5ba21a: verdict not yet available`; checks 15/15 registered,
  `ci#38994749835` push in_progress, 6 running, the oldest `test (windows-2025)` at 128 s when read.
- Re-read at P2's end (same call, same trail): **red** — `ci#38994749835` push completed/failure, wall 530 s, first
  failed check at +400 s, `failed 1: test (ubuntu-latest)`; the other 14 checks green. Its subject (the shared
  `stamped_home` fixture's `viola verify` stamp) is not a clause of this entry, so it would have been the HALT's
  question; the operator's word arrived first and folds it here (§7 below, `inputs#I1`).

**Premise closure (P3):** every `[inferred]` bullet below was read against `research.md`. A verified bullet has lost
its tag; a falsified one carries `[premise-corrected: …]`.

## What this chunk builds

The CLI's machine contract for every verb a driver or a script calls: one `--json` document mirroring the channel
result, one typed exit from the documented set, one cause-named `hint:` per refusal cause, the global `--home` with
its documented resolution order, and `config.json` as the settings file.

### 1. `--json` per verb mirroring channel results (entry; CARRY 2)
- Every driver-facing CLI verb has a `--json` form whose document is the channel `result` as architecture §Standard
  Contracts "CLI `--json` output" spells it: `{"v":1,"ok":{…}}`, `{"v":1,"refusal":…,"detail":…}`, or the two
  `error` documents (`instance-unreachable`, exit 21; `wrapper-fault`, exit 20).
- As read at P3 (`grep -n -B1 'json: bool' src/cmd/*.rs`), `send`, `wait`, `last`, `answer`, `list`, `pause` and
  `release` carry `--json`; `revive` (its start arm and `--list`) and `verify` carry none; `hook` is the plugin's.
  [premise-corrected: the masters settle which verbs owe a machine form — architecture §Established Decisions [CLI
  Conventions] names `viola revive` as the ONE verb without one; `verify`'s agent-readable outcome is its stdout step
  lines, summary and exit (architecture-amendments `2026-09-28-capability-ledger-and-viola-verify`); `run` is the
  interactive wrapper whose stdout is the child's screen. So the owed document is `revive`'s alone; `run` and `verify`
  gain none here.]
- **CARRY 2** (chunk 2026-10-10-viola-revive, the operator's answers at its wrap's Phase 2 halt,
  `viola-0.1.0/chunks/2026-10-10-viola-revive/inputs/I8-relay-3.md.txt` answers 6
  and 2), verbatim: `viola revive` takes no `--json`. Its `--list` rows (`<ts>  <cause>  <id>`) and its exit-1 refusals
  (`strict-modes-failed` · `already-live` · `no-session` · `cwd-missing`) have no machine document. `viola revive
  --list` also opens no process log, so a panic there writes no line: obs-plan §10 records it as a sixth bounded
  exemption that stands only until `--list` opens the `cli` role log with its `--json` (the operator: the founder's
  direction is a structured log from every process).
  - Read as written: the CARRY states the landed gap, not a ruling against a `--json`; the exemption it names stands
    "only until `--list` opens the `cli` role log with its `--json`", and architecture [CLI Conventions] owes that
    `--json` to this entry. (P2's arch extract read it as a standing ruling; that reading is overridden here.)
  - The revive refusal set now also holds `session-live` (chunk 2026-10-16-unwrapped-sessions-on-the-board;
    `src/cmd/revive.rs` `Refusal::detail`), so its machine document covers five codes: `strict-modes-failed` ·
    `already-live` · `no-session` (two pairs) · `session-live` · `cwd-missing`; `--list` reaches two of them.
  - Retiring obs-plan §10's sixth exemption is this chunk's: `--list` opens the `cli` role log after its strict-modes
    check, which takes `role_of` filing `revive --list` under `cli` (`src/main.rs` `role_of` files `revive` as Other).

### 2. Typed exits 0/1/2/10–14/20/21 (entry)
- Every verb's every outcome lands on one exit of architecture §Conventions "CLI exit codes", and a `--json` document
  and its exit agree (the refusal → exit map: `human-typing` 10, `budget-paused` 11, `unverified-cli` 12,
  `not-delivered` 13, `unknown` 14).
- The exits are already typed per verb (`client::fault` → 20, `client::unreachable` → 21, `send::exit_of` → 10–14 with
  its rstest table, `send` / `answer` usage → 2); no per-verb witness reads each verb's `--json` document and literal
  exit side by side (no such file under `tests/`, listing at P3). That witness and `revive`'s documents are what is
  owed.

### 3. Cause-named hints (entry)
- Each refusal cause a human sees prints one `unable:` line and one `hint:` naming the next step for that cause
  (the design phraseology; architecture §Conventions spells the pairs per verb). The pairs this chunk touches stay
  byte-identical; several are the founder's wording.
- Boundary: the machine-readable code per exit-1 / exit-21 cause (v1-18) and the revive rows joining the exit-cause
  table are the later entry "Exit-cause code catalogue" (test-plan §6 Exit-cause matrix); this chunk words no new
  hint and does not mint the catalogue.

### 4. Global `--home` (entry; CARRY 1)
- **CARRY 1**, verbatim: chunk 2026-09-24-three-os-ci-headless-harness-skeleton resolves `--home` as given, else
  `<user home>/.viola`, without `std::fs::canonicalize` and without the `VIOLA_DIR`-grandparent step (security-plan
  §Input Validation CLI row; arch §Cross-cutting Config management).
- Architecture §Cross-cutting Config management names the `VIOLA_DIR`-grandparent step for the CLI verbs as owed to
  this entry: resolution order `--home`, then the grandparent of `VIOLA_DIR` when set, then the default; as landed only
  `hook` takes the `VIOLA_DIR` step (`src/cmd/mod.rs` `resolve_home`, `src/cmd/hook.rs` `instance_of`), so a CLI verb
  run inside a wrapped session without `--home` resolves the default home.
- Reading `VIOLA_DIR` in the CLI verbs reads a `VIOLA_*` variable `viola run` already sets (architecture Config
  management: wrapper-to-child plumbing), not a new configuration channel. [premise-corrected: the validation is not
  open — `hook`'s `instance_of` is the one landed shape check on it (a valid `VIOLA_NAME`, an absolute dir ending
  `instances/<name>`), and security-plan §Input Validation's CLI row requires `--home` and `VIOLA_DIR` canonicalised;
  the canonicalise half is this entry's (security-plan-amendments `2026-09-24-three-os-ci-headless-harness-skeleton`,
  `2026-10-11-the-board-viola-list`: "owed to CLI machine contract"), the strict-modes half Epoch 6's "Home and
  code-bearing file integrity". `endpoint_name` hashes the home's spelling, so `run` and every client must spell one
  home the same way.]

### 5. `config.json` thresholds (entry)
- `config.json` under the home carries `v` = 1, the budget thresholds `five_hour` / `seven_day` as `Percent`,
  `diagnostics_level` and `claude_env_keep`; parsed tolerantly (unknown keys skipped and counted); no value switches
  off a security control.
- The thresholds are read by `run::budget::read_thresholds` (the wrapper and `list`), `diagnostics_level` in
  `src/obs.rs`, `claude_env_keep` in `src/run/env.rs`. [premise-corrected: the `v` check (`src/obs.rs`
  `read_diagnostics_level`, `src/run/budget.rs` `parse_thresholds`) and the unknown-key count (`unknown-keys`,
  `CONFIG_KEYS` of five names) are built; what stays owed is the completeness witness that no flag, `VIOLA_*`
  variable or `config.json` key disables a control (`tests/cli_controls_not_disableable.rs` holds the budget-gate and
  hook-seam rows, no completeness case).]
- v1-39's `port` clause is out of scope: the founder's ruling of 2026-10-16 moved the frontend to 0.2.0 (the cap's own
  `notes`); `CONFIG_KEYS` holds no `port`, so a `port` key reads as unknown.

### 6. The scanned instance on `state-recovered` (CARRY 3)
- **CARRY 3** (chunk 2026-10-11-the-board-viola-list, the review card the founder approved,
  `viola-0.1.0/chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt`, `…/inputs/I8-board-card-owners.md.txt`),
  verbatim: when `viola list` replays several snapshots under a `VIOLA_NAME`, each `state-recovered` line carries the
  caller's instance, never the scanned one (obs-plan §4 E5); naming the scanned instance is a field, the diag-line
  schema and the catalogue.
- The field is written where the line is, `crates/viola-state/src/replay.rs` `read_snapshot_or_replay` (the instance
  dir's last component is in hand there), added to `schemas/diag-line.v1.json`'s `state-recovered` row (default-deny)
  and to obs-plan §6's `state-recovered` catalogue row; the NEVER-log floor allows it (a `ViolaName`, no path or pid).

### 7. The last wrap's red CI, folded on the operator's word (`inputs#I1`)
- Runner-only: `ci#38994749835` on `f6bcce5ba21ae3e876d054175afe3cf8806deb86`, job `test (ubuntu-latest)` (job id
  117284718714), failed 2 of 2666 — `viola::cli_controls_not_disableable
  setting_does_not_disable_the_budget_gate::case_3_a_string` (`FAIL [ 4.439s]`) and `::case_2_negative`
  (`FAIL [ 4.464s]`), each `panicked at tests/support/home.rs:368:5: viola verify did not stamp the home: exit Some(1),
  summary line matched false, failing rows ["resume-session"]`. The operator's note places it at verify's Run E under
  the fake agent in the coverage build. Closed at P3 against the recorded run: VERIFIED — still failing there (job log,
  the failure block and `Summary [ 226.739s] 2666 tests run: 2664 passed (5 slow), 2 failed`); its coordinates kept.
  The same code built green in `ci#38992234808` (`git diff --stat ae21376 f6bcce5` over code, manifests, nextest
  config and workflows is empty), so it is a non-deterministic failure on unchanged code. `hypothesis:` two
  `stamped_home` verify runs starting in one instant contend for something Run E reads (case_1, started alone, passed;
  cases 2 and 3 finished 8 ms apart) — unmeasured. Platform tracker searched: nothing matching (research).
- Acceptance: its own criterion and a witness that cannot pass vacuously — the cause named from the recorded run's
  kept artifact (`harness-ubuntu-latest`) or a forced concurrent start, the `stamped_home` stamp (the `resume-session`
  row under the fake agent) made deterministic by removing that cause, never by a raised bound or a retry, and proved
  green on the Linux runner leg it failed on.

## Boundaries — not this chunk

- MCP tools and their `isError` forms — "MCP server for drivers".
- Linear output, grouped `--help`, the colour layer, the stderr-writer framing — "CLI output discipline".
- One machine code per exit-1 / exit-21 cause (v1-18) — "Exit-cause code catalogue".
- External error sanitisation across CLI / MCP / channel (v1-47) — "Sanitised error surfaces".
- `link` / `unlink` and the `link` half of v1-13 — "Explicit session links".
- Server verification and strict-modes before a CLI verb's first frame — Epoch 6's "Server verification before any
  frame" and "Home and code-bearing file integrity".
- A `--json` for `run` or `verify` (§1's closure).

## Surfaces and contracts it touches

- Code: `src/cmd/mod.rs` (`resolve_home`), `src/cmd/hook.rs` (`instance_of`), `src/cmd/revive.rs`,
  `src/cmd/client.rs`, `src/main.rs` (`role_of`), `crates/viola-state/src/replay.rs`, `schemas/diag-line.v1.json`,
  `src/cmd/run.rs` (the home canonicalised after its creation), and the CI red's site (`tests/support/home.rs`,
  `src/cmd/verify/typed.rs` or `src/bin/viola-fake-agent.rs`, provisional). [premise-corrected: no change to
  `src/cmd/verify.rs`, `src/human.rs` or the `config.json` readers, and `src/obs.rs` only in its test module (a pin of
  its key list to a new `schemas/config.v1.json`, the "config schema" test-plan §5's completeness case reads, absent at
  P4) — §1 and §5's closures]
- Contracts: architecture §Standard Contracts (CLI `--json` output), §Conventions (CLI exit codes, CLI flags),
  §Cross-cutting Config management, §Established Decisions [CLI Conventions]; security-plan §Input Validation (CLI
  row); obs-plan §4 (E5), §6 (`state-recovered`), §10 (the exemption list), the diag-line schema; design-system cli
  pattern 5 and layout-templates §Output structure — `viola revive` (their "takes no `--json`"); test-plan §1 and §6
  (revive's machine form); a11y-plan §1 (CLI verbs entity).

## Capability candidates

- `[inferred]` `v1-39` — "CLI contract: machine output, typed exits, cause-named hints" restates this entry; its
  `port` clause leaves 0.1.0 (its notes). Whether it is fully provable by this chunk (its "readback mirror for `send`"
  and "results on stdout and context on stderr" clauses touch "CLI output discipline") is P4's claim lean.
