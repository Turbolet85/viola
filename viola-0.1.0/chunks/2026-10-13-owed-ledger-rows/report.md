# Report — 2026-10-13-owed-ledger-rows

**Chunk:** the statusline settings override, the `/bin/sh` shell and the statusline payload's `rate_limits` as three
probed, stamped capability-ledger rows, relied on where the status line is wrapped (Unix); `--resume` / `/compact`
and the `claude agents --json` row moved off this entry by the founder's forks (`inputs#I1`)
**Date:** 2026-10-14
**Commits:** `9ee8309 chore(2026-10-13-owed-ledger-rows): operator pre-CI commit, for the run this chunk's verdict
reads` · `1c4f225 fix(2026-10-13-owed-ledger-rows): the fake agent's status line runs after the turn screen; the
missing-row case drops a relied row` (basis: `git log --format='%h %s' 14e62329f808..HEAD`; 14e62329f808 is the parent
of the oldest pre-CI commit)

## Changes (structured — detectors read this)
- **Files:** 18 product and test files (basis: `git diff --stat 14e62329f808 -- crates src tests schemas fixtures`,
  1765 insertions, 180 deletions):
  - modified: `crates/viola-agent-claude/src/ledger.rs`, `crates/viola-agent-claude/src/statusline.rs`,
    `crates/viola-e2e/src/harness/run.rs`, `src/bin/viola-fake-agent.rs`, `src/cmd/hook.rs`, `src/cmd/verify.rs`,
    `src/cmd/verify/typed.rs`, `src/run/version_gate.rs`, `tests/cli_verify.rs`, `tests/contract_fake_agent_drift.rs`,
    `tests/contract_fixture_hygiene.rs`, `tests/contract_ledger_probes.rs`, `tests/hook_fail_open.rs`,
    `tests/support/home.rs`, `tests/support/verify.rs`;
  - new: `schemas/claude-statusline-fixture.v1.json`, `fixtures/claude/2.1.287/StatusLine.default-1.json` (1081 B),
    `fixtures/claude/2.1.287/StatusLine.rate-limits-1.json` (1724 B), both recorded live and copied alone;
  - records: `viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/evidence/` (`live-round.md`, `round-232112Z.txt`,
    `operator.md`), the implement run dir `.andromeda/runs/2026-10-13T23-05-48-implement/`.
- **Symbols / APIs:** (coordinates from `## New text, by line`)
  - `viola_agent_claude::ledger`:
    - `LedgerRow` gains `StatusLineOverride` (`statusline-override`, "a --settings statusLine runs in the status
      line's place"), `StatusLineShell` (`statusline-shell`, "a statusline command runs through /bin/sh") and
      `StatusLineRateLimits` (`statusline-rate-limits`, "the statusline payload carries two whole-number rate_limits
      windows"), after `PasteNewline`; `LedgerRow::ALL` is `[Self; 21]`.
    - `pub const fn relied_here(self) -> bool`, `const fn relied_on(self, wrapped)` (false for the three statusline rows
      when not wrapped) and `pub fn relied() -> Vec<Self>` (`ALL` filtered, in `ALL` order) read
      `statusline::WRAPPED_HERE` (`cfg!(unix)`), never a second `cfg!`. Unix: 21 relied rows; Windows: the first 18.
    - The stamp verdict `verified` iterates `LedgerRow::relied()` (was `ALL`): on Unix an 18-id all-`pass` stamp reads
      unverified, on Windows verified.
    - `pub enum ShellVerdict { BinSh, Other }` (file words `bin-sh` / `other`, `read` takes exactly `bin-sh`, else
      `Other`); `pub struct StatusLineCapture { bytes, shell }`; `Probes.statusline: Vec<StatusLineCapture>`.
    - Check arms: override passes on ≥ 1 capture; shell on ≥ 1 capture all `BinSh`; rate-limits when some capture is
      one JSON object whose `rate_limits.five_hour` and `.seven_day` are objects with an integer `used_percentage`
      0–100 and an integer `resets_at` (`whole_rate_limits`, narrower than `statusline::reading`, which still also
      takes an RFC 3339 string and a fraction).
    - `pub const STATUSLINE_DEFAULT = "StatusLine.default-1.json"`, `STATUSLINE_RATE_LIMITS =
      "StatusLine.rate-limits-1.json"`; `pub fn statusline_variants(&[StatusLineCapture])` (the first capture without
      `rate_limits`, the first that passes the rate-limits arm); `statusline_capture_file_name(k)` →
      `StatusLine.<k>.json`, `statusline_shell_file_name(k)` → `StatusLine.<k>.shell`,
      `parse_statusline_capture_file_name`.
  - `viola_agent_claude::statusline`: `WRAPPED_HERE` made `pub`; `pub const BIN_SH = "/bin/sh"`, `pub const SHELL_FLAG
    = "--shell"`; `pub fn probe_document(pinned_bin_fwd, captures_dir_fwd) -> Option<String>` (113-119): on a wrapped
    OS and only for shell-plain paths, `{"statusLine":{"type":"command","command":"<pinned> hook statusline --capture
    <dir> --shell \"$0\""}}`, else `None`.
  - `src/cmd/hook.rs`: `HookArgs` gains a hidden `--shell <WORD>` (`OsString`). `hook statusline --capture <DIR>` takes
    a new arm before the pass-through, `capture_statusline` (404-435): `<DIR>` absolute and existing, stdin through
    `take(MAX_FRAME + 1)` and refused past `MAX_FRAME`, the payload filed raw at the first free `StatusLine.<k>.json`
    (`create_private_new` claim, `replace_private` 0600) and `StatusLine.<k>.shell` holding exactly `bin-sh` when the
    word equals `/bin/sh` byte for byte, else `other` (a missing word is `other`); the word itself is written nowhere;
    nothing printed, exit 0 on every path, no `VIOLA_*`, no obs init, no channel, no `budget.json`, no snapshot.
    `hook statusline` without `--capture` keeps its pass-through unchanged; `--shell` on any other arm changes nothing.
    Sole caller of the arm: `viola verify`'s Run B probe settings.
  - `src/cmd/verify.rs`: `ProbeDir::create` makes `typed/statusline/` (0700) and, when `probe_document` is `Some`,
    writes `typed/statusline.json` (0600); `read_statusline` (411-440) reads `typed/statusline/` in `k` order through
    `take(MAX_FRAME + 1)` (an absent or unreadable shell file reads `Other`, the shell file capped at 16 bytes);
    `check_rows` walks `LedgerRow::relied()` and the step line's total is its length; `named_payloads` returns owned
    `(name, serde_json::Value)` pairs and adds the two `StatusLine` variants to what `--record` scrubs, refuses on
    `unclean` and writes only at 0 failing rows.
  - `src/cmd/verify/typed.rs`: `Inputs.statusline_settings: Option<&Path>`; `trusted_args` (147-157) appends
    `--settings <typed/statusline.json>` after the `--plugin-dir` pair when `Some` (Unix), nothing when `None`
    (Windows). Run B is the one caller; Runs A, C and D keep their argv (C and D their one `--settings`).
  - `src/run/version_gate.rs`: the gate reads `verified` (now over `relied()`); cases added only.
  - `src/bin/viola-fake-agent.rs`: `replay_statusline` (518-535) — under `--framing`, with `--settings <file>` naming an
    absolute `statusLine.command`, the command is split on ASCII white space, the one word exactly `"$0"` handed over as
    `/bin/sh` (`shell_expanded`, 352-364), spawned directly (never a shell) with the set's `StatusLine.default-1.json`
    after the launch SessionStart's hooks and `StatusLine.rate-limits-1.json` once after the first Stop; each a
    `statusline` receipt with `stdin_hex`. After the fix commit the first-Stop replay runs at the END of `end_turn`,
    after the turn screen is written (it had run between the Stop hook and the paste-hint clear — see Deviations 10).
    No new option; a set without the variants fires nothing.
  - `crates/viola-e2e/src/harness/run.rs`: `LEDGER_ROWS` is `[&str; 21]` under `cfg(unix)` and the 18-id form under
    `cfg(not(unix))`; the unit proof `LIVE_PASS` the same split (941-963); `run_local_live_reads_the_relied_rows`.
- **Crates / modules:** none added or removed; changed `viola-agent-claude` (`ledger`, `statusline`), the root bin
  (`cmd::hook`, `cmd::verify`, `cmd::verify::typed`, `run::version_gate`, the fake agent), `viola-e2e` (`harness::run`).
- **Dependencies:** none added or bumped (`cargo deny check` green, entry `cargo deny check`).
- **Schema / config:**
  - new `schemas/claude-statusline-fixture.v1.json`: a tolerant object, `session_id` required as a string;
    `rate_limits`, when present, an object requiring `five_hour` and `seven_day` objects.
  - `tests/contract_fixture_hygiene.rs` validates every `fixtures/claude/*/StatusLine.*.json` against it, every other
    fixture against `claude-fixture.v1.json` as before, with the same path / username / email walk.
  - The record scrub (`ledger::scrub`, `ledger::unclean`) is unchanged: it rewrites the home to `~` and the OS user
    (the home dir's last component) as a whole word to `<user>`. The two recorded `StatusLine` variants carry a
    `workspace` object whose `repo` names this repository's own GitHub remote (`host`, `owner`, `name`); the `owner`
    value is the remote owner's handle, which the scrub's whole-word OS-user rule does not match (a two-digit suffix).
    The founder's word: kept (`inputs#I3`, 2026-10-14T05:57Z, and the wrap invocation `inputs#I4`) — see Spec claims
    disproved and Expected amendments.
  - No `config.json` key, env var or CLI flag added beyond the hidden `hook --shell <WORD>`.
- **Spec-master edits:** none (implement wrote no master).
- **Counts / qualifiers moved:**
  - Capability-ledger rows: 18 → 21 in `LedgerRow::ALL`; the relied set per OS — 21 on Unix, 18 on Windows (the rule:
    a row is relied when `relied_on(row, statusline::WRAPPED_HERE)`, false only for the three statusline rows off Unix).
    The masters state eighteen: sweep `rows18` (`(?i)eighteen|\b18 (ledger |capability-ledger )?rows|\b18 pass`),
    standing 14 master/registry rows: `architecture.md:91` ×4, `:422` ×2, `:472` ×2; `security-plan.md:617`;
    `design-system.md:811` ×2; `layout-templates.md:490`; `test-plan.md:656`, `:666` ×2, `:752`;
    `registries/contracts/architecture/ci-cd-approach.md:20`; `…/crate-dependency-direction.md:6`;
    `registries/contracts/test-plan/5-command-implementation.md:8`, `:53` ×4;
    `…/bootstrap-phases-derive-for-route-setup-project.md:10`. Each moves to the per-OS rule except where it records a
    past reading (`architecture.md:472`'s `stamped 2.1.287  18 pass  0 fail` and `test-plan.md:752`'s are the
    paste-newline chunk's dated live readings — kept as history).
  - The `viola verify` step counter's `MM`: now the relied count (21 Unix, 18 Windows). Sweep `step-counter`
    (`\[NN/MM\]|\[01/1[0-9]\]|/18\]|\bMM\b`), standing 22: the `[NN/MM]`/`/18]` sites `architecture.md:91` ×3,
    `design-system.md:811`, `layout-templates.md:477`–`:489` (the worked example, 12 rows) and `:501` ×2,
    `test-plan.md:656` ×2, `obs-plan.md:739`; no-change 8 — `\bMM\b` matching `HH:MM` / `YYYY-MM-DD` time formats
    (`design-system.md:240`, `:447`, `:591`, `:805` ×3; `layout-templates.md:262`, `:459`, `:471` ×3).
  - Verify spawn pairs: still six (no run added).
  - Run B's `--settings`: 0 → 1 (Unix only).
  - 2.1.287 fixture set: +2 files (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`), every other file
    byte-identical (the fixture guard entry, green).
- **Dev-tool versions:** none — `claude` re-read at 2.1.287 by path for both live rounds.
- **Harness / gate surface:**
  - `viola-harness`: `LEDGER_ROWS` per OS (21 Unix / 18 `not(unix)`), which `run --local-live` asserts.
  - `scripts/agent-run.sh` verbs, CI workflow steps and status shapes: unchanged.
  - `tests/contract_windows_mutation_scope.rs` pins every product `cfg(windows)` line to `windows-mutants.yml`; the
    harness's Windows form is spelled `cfg(not(unix))` (Deviations 4), so the workflow is unchanged.
- **Cross-project / external claims:**
  - CI (`Turbolet85/viola`, read through `ci.py conclusion`):
    - ci#38519633153 on `9ee83095ace5` (the pre-CI commit): **red** — `test (macos-latest)` one test, `test
      (windows-2025)` one test, the other 13 checks green; both this chunk's own, fixed (Deviations 10).
    - ci#38520999442 on `1c4f2258ef7c` (the fix commit, the final HEAD this report rests on): **green**, 15/15, wall
      550 s.
  - inputs (`inputs.py verify`, fired at this wrap after the snap of I4):
    - `I1 · message · copy · n/a` — the founder's four fork answers (the cut, the `agents --json` row to its consumer,
      Run B + `--settings`, ten starts), relayed by the operator 2026-10-13T22:52:37Z; cited by scope, research, plan.
      Cited here.
    - `I2 · message · copy · n/a` — the operator's `yes` to P5 review card 1,
      `.andromeda/runs/2026-10-13T22-31-54-phase/p5-card-1.md`; cited here. The card's `## Beyond the plan`, verbatim
      (the operator's word on a card whose two rows each rest on a founder fork answer, `inputs#I1`):
      > 1. **New route entry, "Resume and compact ledger rows", directly after "Owed ledger rows".** It carries this
      >    entry's CARRY 2 (`claude --resume <id>`, chunk 2026-10-10-viola-revive) and CARRY 3 (`/compact`, the same
      >    chunk) verbatim, and CARRY 1's cost frame verbatim. The source is the founder's fork 1 (`inputs#I1`).
      > 2. **The `claude agents --json` join row and its probe move onto "Unwrapped sessions on the board"**, with this
      >    entry's CARRY 6 verbatim. The source is the founder's fork 2 (`inputs#I1`).
    - `I3 · message · copy · n/a` — the founder's own word, live in the overseer's dialog 2026-10-14T05:57Z, relayed by
      the operator: accept and push the fixtures carrying the remote owner's handle; the wrap amends test-plan §7
      narrowly. Cited here.
    - `I4 · message · copy · n/a` — this wrap's invocation (the operator, 2026-10-14T06:37Z): the §7 amendment's
      wording ("a workspace.repo naming this repository's own remote is kept in a fixture") and the two route moves
      of `inputs#I1`. Cited here.
    - verify's line: `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0
      · n/a 4 · uncited 3 · unparsed 0` (fired before this report existed; I2–I4 are cited by this bullet).
- **Reverted / negative API facts:**
  - A second `--settings` on Runs C or D, a Run E, a shell in the fake agent, the shell word captured as text: rejected
    in the plan, not written.
  - The first-Stop statusline replay placed between the Stop hook and the paste-hint clear shipped in the pre-CI
    commit and was moved in the fix commit (Deviations 10).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - test-plan §7 Fixture hygiene (`test-plan.md:1096`–`:1103`, `:1310`; sweep `fx-hygiene`, standing 3) states recorded
    fixtures are "scrubbed of paths and usernames". The two `StatusLine` variants hold `workspace.repo.owner` — the
    GitHub handle of this repository's own remote — which the scrub does not rewrite and `contract_fixture_hygiene`
    reads green. Measured: the record round's refusal did not fire; `git grep -i` over `fixtures schemas tests src
    crates` held no such handle before this round. Disposition: the founder's word keeps it (`inputs#I3`); the
    amendment is narrow — a `workspace.repo` naming this repository's own remote is kept in a fixture (`inputs#I4`).
- **Expected amendments (from plan):** (searches: the `report-sweep.txt` pre-pass of this run dir, baseline HEAD
  `1c4f2258`)
  - architecture §Established Decisions [CLI Version Compatibility] (twenty-one rows, the per-OS relied set, the
    verdict over it, Run B's one probe `--settings`, the fork-3 answer): carried — Symbols / APIs, Counts; sites
    `architecture.md:472` (rows18 ×2, owed-rows ×6, sl-shapes ×6, settings-flag ×1), `:91` (rows18 ×4, step-counter
    ×3, settings-flag ×2, capture-arm ×1).
  - architecture §Cross-cutting Patterns (Capability ledger) (override, `/bin/sh` and `rate_limits` now rowed;
    `--resume` / `/compact` owed to the new entry "Resume and compact ledger rows"; the `agents --json` row owed to
    "Unwrapped sessions on the board"): carried — sites `architecture.md:79` (sl-shapes ×2), `:86` (agents-json),
    `:291` (owed-rows, agents-json ×3: the row "moved to Owed ledger rows"), `:472` (resume-compact).
  - architecture §Occupied Resources (Repository) — the two `StatusLine` variants (the 2.1.287 set's file count with its
    rule) and `schemas/claude-statusline-fixture.v1.json`: carried — Files, Schema; sites `architecture.md:422`
    (rows18 ×2), `:425` (fixture-schema), `registries/contracts/architecture/project-directory-structure.md:50`
    (fixture-schema).
  - security-plan §Threat Model Summary / §Data Protection (the verify child-set bullet) and §Input Validation (Hook
    stdin row) — Run B's probe `--settings` and the `hook statusline --capture --shell` arm; §Security Anti-Patterns →
    Universal (a non-`null` decision needs every relied row): carried — Symbols / APIs; sites `security-plan.md:131`
    (settings-flag, capture-arm: the verify child set), `:231` ×3 and `:236` (capture-arm, the Hook stdin row), `:282`
    (capture-arm, Data Protection), `:511` ×2, `:617` (rows18), `:248` (owed-rows, sl-shapes: the statusline row's
    "no ledger row" clause). The widening is the founder's own fork-3 answer (`inputs#I1`): playbook "Boundary
    widening the founder answered himself to a P4 fork".
  - test-plan §3 `run`, §4, §5, §7 (row counts per OS with their rule and `LEDGER_ROWS`' named list; the `StatusLine`
    variants and their schema; the fake agent's `--framing` statusline replay with the `"$0"` hand-over; the
    rewritten-resume-payload note re-owed to the new entry): carried — sites `test-plan.md:656`, `:666`, `:752`
    (rows18), `:438`, `:1101` (fixture-schema), `:1076`, `:1093` (settings-flag, owed-rows, agents-json: the fake
    agent's statusline modes and the owed rows), `registries/contracts/test-plan/5-command-implementation.md:8`, `:53`
    (rows18, ledger-rows `LEDGER_ROWS`/`LIVE_PASS`).
  - obs-plan §10 (bounded exemption 5) — the statusline capture inside the hidden capture arm: carried — sites
    `obs-plan.md:1070`, `:740`, `:583`, `:512` (capture-arm), `:739` (step-counter).
  - design-system §Surface: cli pattern 5 — MM is the relied count, the three rows named: carried — site
    `design-system.md:811` (rows18 ×2, step-counter).
  - layout-templates §Output structure — `viola verify` — the row list, MM with its rule, the worked example at
    twenty-one: carried — sites `layout-templates.md:477`–`:490`, `:501`.
  - Added by the founder after the plan (`inputs#I3`, `inputs#I4`): test-plan §7 Fixture hygiene — a `workspace.repo`
    naming this repository's own remote is kept in a fixture: carried — Spec claims disproved; sites
    `test-plan.md:1096`, `:1103`, `:1310` (fx-hygiene).
  - `v1-34`'s not-claimed note: written at phase P5 step 4 (the plan says so), not a wrap entry.
- **Coverage of new surfaces:**
  - `hook statusline --capture <DIR> --shell <WORD>` (hidden) → validation `take(MAX_FRAME + 1)`, absolute existing
    dir, closed `ShellVerdict`✓ · instrumentation n/a (no obs init by design, obs §10 exemption 5) · PII payload raw at
    0600 under the probe dir, the shell word never written✓ · tests unit (`hook.rs` ×4) + integ (`hook_fail_open` ×2)
    · a11y n/a · tokens n/a
  - Run B's probe `--settings` (`typed/statusline.json`) → validation shell-plain paths only, Unix only✓ ·
    instrumentation the existing six spawn pairs, no line carries the command or a path✓ · PII n/a · tests unit
    (`statusline.rs` ×3, `verify.rs` `probe_dir_…`) + integ (`cli_verify`) + live (record round) · a11y n/a · tokens n/a
  - `viola verify` step lines at the relied count → validation n/a · instrumentation n/a · PII n/a · tests integ
    (`cli_verify` per OS) · a11y plain ASCII, no SGR, unchanged form✓ · tokens n/a
  - the fake agent's `--framing` statusline replay → test-only · tests unit ×4 + drift contract

## Deviations from intent
1. **Step 2's order.** The arms were written before the nine row cases (one scripted edit), so no red-before-green
   reading of the cases exists; they read each arm's pass, broken shape and no-capture fail, green.
2. **Step 2's count.** The plan gives the override row no broken shape; its third case is "Run B's hooks fired while
   its status line filed nothing", distinct from "no capture at all". The `passed 9` atom holds.
3. **A shape-pinned reader research did not list.** `merge_stamp_writes_the_envelope_for_a_first_version` (`ledger.rs`,
   a literal `rows` object) went red at the first unit run and gained the three ids; the file was on the list, the
   reader was not (recurs testing.md 2026-10-07).
4. **Harness gate spelling.** The 18-row `LEDGER_ROWS` / `LIVE_PASS` are `cfg(not(unix))`, not `cfg(windows)`:
   `pre-push` went red on `contract_windows_mutation_scope` ("unlisted: crates/viola-e2e/src/harness/run.rs") — a
   product `cfg(windows)` line makes the file a Windows-gated source `windows-mutants.yml` must list. `not(unix)` is
   the exact complement of `WRAPPED_HERE = cfg!(unix)`; test files keep `cfg(windows)` for their Windows literals.
5. **Step 3's parser.** `parse_statusline_capture_file_name` reads `StatusLine.<k>.json` names only; the shell file is
   derived from `k`, never listed.
6. **Step 6's plumbing.** `named_payloads` returns owned `(name, Value)` pairs so the status line variants (bytes, not
   `Capture`s) join the same scrub and refusal path.
7. **Step 8's relative command.** The replay goes through the existing `run_receipted`: a relative command is
   receipted `command_absolute:false, ran:false` and never run, as the `--statusline-stdin` path does.
8. **Cases beyond the plan's list:** `hook.rs` units on `capture_statusline` (k claim, byte-exact word, cap, bad dir);
   `read_statusline` units and the `ProbeDir` settings case; `verified_reads_a_failed_statusline_row_by_the_relied_set`;
   `run_local_live_reads_the_relied_rows` and its statusline row-missing cases; `statusline_drift`'s synthetic case;
   the hygiene walk's planted status line cases; `cli_verify`'s identity-strip case asserting two `statusline` receipts
   on Unix.
9. **A pre-check outside the block before the live round:** `agent-run.sh run --integration --filter 'binary(cli_verify)
   | binary(hook_fail_open) | binary(hook_statusline)'`, fake agent only: 66 passed, 5 failed, all five at
   `stamped_home` over the then-unrecorded set — the designed red until the record entry; green afterwards.
10. **The CI red on the pre-CI commit, fixed in the chunk** (ci#38519633153 on `9ee83095`):
    - macOS, `cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (`tests/cli_send.rs:919`, exit
      `Some(0)` want `Some(10)`): the fake agent's first-Stop status line replay ran between the Stop hook and the
      paste-hint clear; while it spawned `viola hook statusline` the earlier input box stood on screen and the second
      send's gate wait read it as settled. Read on Linux with a 2 s probe sleep in the replay: red at `:919`; with the
      replay moved after the turn screen and the sleep kept: green, and 80/80 over `cli_send`, `cli_verify`,
      `hook_statusline` and `contract_fake_agent_drift`. The sleep was removed.
    - Windows, `ledger::tests::verified_needs_every_row_pass_under_that_version` (`ledger.rs:1985`, "a missing row"):
      it popped `ALL`'s last row, `statusline-rate-limits`, which Windows does not rely on. It now drops `relied()`'s
      last row (the same row on Unix, `paste-newline` on Windows).
    - Fix commit `1c4f2258`, pushed after the whole non-leg block read 17/17 green and a Windows-target clippy read
      clean; CI green on it.
11. **The fixture handle surfaced and held** (implement's outcome `surfaced` → resolved): the operator pass was held on
    the `workspace.repo.owner` handle until the founder's word (`inputs#I3`).

Scope record: none — `gate.py scope` clean, changed 18 · listed 18 · recorded 0.

## Decisions & corrections
- **A per-OS relied set breaks tests that pick a member by position.** `verified_needs_every_row_pass_under_that_version`
  popped `LedgerRow::ALL`'s last row; with the statusline rows Unix-only, on Windows that row is not relied and the
  stamp still read verified (CI `test (windows-2025)`, ci#38519633153). Fixed by naming `relied()`'s last row. The Linux
  host and the Windows-target clippy cannot see it; only the Windows test run did.
- **A fake-agent side effect placed between a hook and a screen change moves a race window.** The first-Stop statusline
  replay (a `viola hook statusline` spawn) sat between the Stop hook and the paste-hint clear; on the macOS runner the
  spawn cost let the gate wait read the stale input box as settled (`cli_send.rs:919`). Moved after the turn screen.
  Linux never showed it.
- **Proving a suspected race by injecting a delay at the suspected site.** A 2 s `thread::sleep` in the replay read red
  on Linux at the CI's own line; the fix with the sleep kept read green across the four replay-driving binaries (80/80);
  then the sleep was removed (`evidence/operator.md`). The cause was known before the fix commit.
- **A product `cfg(windows)` line enrols its file in the Windows mutation scope.** `pre-push` red on
  `contract_windows_mutation_scope` for the harness's `LEDGER_ROWS`; spelled `cfg(not(unix))`, the complement of the
  product's own `WRAPPED_HERE` (implement outcome, deviation 4).
- **A new fixture class can carry identity fields the scrub never met.** The statusline payload is the first fixture
  class with a `workspace` object; its `repo.owner` is the remote owner's handle, which the whole-word OS-user scrub
  misses (suffix) and the hygiene walk reads green. Surfaced before the push; the founder kept it (`inputs#I3`).
- **The founder's word on the handle** (`inputs#I3`, live 2026-10-14T05:57Z, «Принять и пушить»): accept and push; the
  wrap amends test-plan §7 narrowly — a `workspace.repo` naming this repository's own remote is kept in a fixture
  (`inputs#I4`).
- **The founder's route moves** (`inputs#I1` forks 1 and 2, card `inputs#I2`): mint "Resume and compact ledger rows"
  directly after this entry (CARRY 2, CARRY 3, CARRY 1's cost frame verbatim); move the `claude agents --json` row and
  probe onto "Unwrapped sessions on the board" (CARRY 6 verbatim). For P5 (`inputs#I4`).
- **A held dialog timed out.** The AskUserQuestion on the handle returned "No answer from the driver within 58 min — do
  not proceed with this; end your turn and wait for instructions"; the turn ended with the pass held; the answer came as
  a relay (implement journal).
- **Recurrence of testing.md 2026-10-07** (list a moved bound's readers by function): research's sweep missed
  `merge_stamp`'s literal envelope oracle (deviation 3).
- **Guard and hook refusals:** the Bash guard refused a `cat` heredoc to a scratch file (re-issued through the Write
  tool); the stamp-ahead PostToolUse hook blocked a test literal `2026-10-14T00:00:00Z` later than the clock (moved to a
  past date); `cascade.py sweep` refused a pattern id over 16 characters (`statusline-shapes`, shortened).
- **Context compaction mid-chunk** (01:25Z): the run dir's outcome and journal were re-read from disk before the
  operator pass, at the operator's direction.
- **A journal time written by hand was wrong** (`06:33Z` for a reading at `06:23:31Z`), corrected from `date -u`.

## Outcome
- Acceptance, re-asserted against the diff:
  - (arch) `ALL` 21 rows in the stated order; `relied()` 21 Unix / 18 Windows; the verdict over `relied()`; the 18-id
    stamp unverified on Unix (`version_gate` cases, `run --unit` green) and verified on Windows (CI `test
    (windows-2025)` green on `1c4f2258`): **met**.
  - (tests) `test(/check_statusline_/)` 9 passed, 0 failed (entry 6, green): **met**.
  - (security) Run B carries exactly one `--settings`, the probe document, Unix only, shell-plain paths only; no other
    run's argv changed; no run added: **met** (`typed.rs` `trusted_args`; `cli_verify` identity-strip case).
  - (security) the capture arm: 0600 raw, `take(MAX_FRAME + 1)`, `bin-sh`|`other`, word nowhere, nothing printed, exit
    0, no `VIOLA_*`, no role file: **met** (`hook_fail_open`, `hook.rs` units).
  - (tests) the record round 5 starts, `stamped 2.1.287  21 pass  0 fail`, census empty; the `run --local-live` round 5
    starts, `"suite":"local-live","passed":1,"failed":0`, census empty; both in `evidence/live-round.md`: **met**.
  - (tests) the fixture set gains exactly the two variants (the fixture guard, green); `contract_ledger_probes`,
    `contract_fake_agent_drift`, `contract_fixture_hygiene` green: **met**. The hygiene walk's "usernames" reach is the
    founder-kept exception above.
  - (design / layouts / a11y) one `[NN/MM]` line per relied row, MM 21 on this host, the summary n + m = MM, plain
    ASCII, the interrupted pair unchanged (`cli_verify`): **met**.
  - (obs) six spawn pairs, schema-valid, no line carries the probe command, a payload or a path, the capture arm writes
    no line (`cli_verify`, G4 entry `schema-check`): **met**.
  - (tests) `LEDGER_ROWS` 21 Unix / 18 Windows and `LIVE_PASS` (`run --unit`): **met**.
  - (security) no new dependency, `cargo deny check` green, `send` and the pass-through untouched, the preservation
    guard green: **met**.
  - (tests) the pushed HEAD's CI `verdict: green` on all three OSes: **met on `1c4f2258`** (ci#38520999442); red on the
    pre-CI commit `9ee83095` (ci#38519633153), both causes this chunk's, fixed.
- Gates (the final run, `.andromeda/runs/2026-10-13T23-05-48-implement/gate-run-6.txt`, after the fix):
  - `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
    green · the `instrument` rg probe green (exit 1, no hit) · `bash scripts/lint-probes.sh` green ·
    `bash scripts/agent-run.sh run --unit` green · `run --unit --filter 'test(/check_statusline_/)'` green (9 passed).
  - The live round (probe, record, census, probe, `run --local-live`, census): fired once, `round: COMPLETE · legs fired
    2/2`, every line green (`evidence/round-232112Z.txt`); not re-run.
  - The fixture guard green · `run --integration --filter …` green (35.41 s against the P5 baseline 35.9 s) · the
    preservation guard green · `cargo deny check` green.
  - Smoke: `cleanup`, `boot --session p-olr-smoke`, `status`, G2 `g2-zero-panics.sh`, G4 `schema-check`, `cleanup`
    green (boot stamps 21 rows against the fake agent).
  - `bash scripts/agent-run.sh pre-push` green (68.02 s).
  - Operator entries (`evidence/operator.md`): `gate.py hygiene` green twice (`hygiene: clean`); the push green twice
    (`14e62329→9ee83095`, `9ee83095→1c4f2258`); `ci.py conclusion --sha HEAD --wait 1800` red on `9ee83095`
    (ci#38519633153, this chunk's, fixed), green on `1c4f2258` (ci#38520999442, 15/15, wall 550 s).
- Watches: none folded.
- Outcome basis: the operator pass ran — its commits `9ee8309` (pre-CI) and `1c4f225` (fix), and the final HEAD's CI
  run ci#38520999442 recorded in `evidence/operator.md`. Implement's persisted report
  `.andromeda/runs/2026-10-13T23-05-48-implement/outcome-2026-10-13-owed-ledger-rows.md` (rewritten green + smoke at
  2026-10-14T06:24Z; first written `surfaced`), held whole in this window. Operator directives between implement and
  this report: `inputs#I3` (accept and push), `inputs#I4` (this wrap).
- Process hygiene: implement's census (10 `claude` 2.1.287 starts by the two legs, terminated, both censuses empty;
  the harness, fake agent and smoke session `p-olr-smoke`, terminated). Re-measured 2026-10-14T06:25Z after the
  operator pass: no process whose `/proc/<pid>/exe` is under the repository; no `claude` whose cwd is a probe dir.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 14e62329 (the parent of the oldest pre-CI commit 9ee83095) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/ledger.rs — added 464 line(s) in 29 range(s)
added: 17 · 40-42 · 46 · 65-67 · 91-93 · 120-124 · 127-147 · 510-530 · 604-639 · 646 · 883-894 · 898-938 · 1092-1093
       1104-1106 · 1374 · 1397-1399 · 1424-1426 · 1432-1452 · 1691 · 1712 · 1866-1867 · 1975-1976 · 2024-2035
       2039-2058 · 2176 · 2196 · 2585 · 2680 · 2804-3044
  - 128-132 @130 «pub const fn relied_here(self) -> bool {»
  - 134-141 «const fn relied_on(self, wrapped: bool) -> bool {»
  - 143-147 @145 «pub fn relied() -> Vec<Self> {»
- 510-513 @511 «pub fn statusline_capture_file_name(k: u32) -> String {»
- 515-518 @516 «pub fn statusline_shell_file_name(k: u32) -> String {»
- 522-529 @523 «pub fn parse_statusline_capture_file_name(name: &str) -> Option<u32> {»
  - 524-527 «let k = name»
- 604-610 @607 «pub enum ShellVerdict {»
- 612-629 «impl ShellVerdict {»
  - 613-619 @614 «pub const fn word(self) -> &'static str {»
  - 621-628 @622 «pub fn read(bytes: &[u8]) -> Self {»
- 631-636 @633 «pub struct StatusLineCapture {»
- 898-913 @901 «fn whole_rate_limits(bytes: &[u8]) -> bool {»
  - 902-904 «let Ok(Value::Object(payload)) = serde_json::from_slice::<Value>(bytes) else {»
  - 905-909 «let whole = |window: &Value| {»
  - 910-912 «payload»
- 920-937 @922 «pub fn statusline_variants(captures: &[StatusLineCapture]) -> Vec<(String, &[u8])> {»
  - 923-928 «let without = captures.iter().find(|c| {»
  - 930-933 «[»
  - 1104-1106 «Ok(LedgerRow::relied()»
  - 1432-1451 @1433 «fn relied_rows_drop_the_three_statusline_rows_where_the_status_line_is_not_wrapped() {»
  - 2039-2057 @2042 «fn verified_reads_a_failed_statusline_row_by_the_relied_set() {»
  - 2804-2815 @2805 «fn limited(five_hour: Value, seven_day: Value) -> StatusLineCapture {»
  - 2817-2819 «fn window(used: Value, resets_at: Value) -> Value {»
  - 2821-2835 @2823 «fn clean_statusline() -> Vec<StatusLineCapture> {»
  - 2837-2848 «fn statusline_check(row: LedgerRow, statusline: Vec<StatusLineCapture>) -> bool {»
  - 2850-2859 @2851 «fn check_statusline_override_passes_on_a_filed_payload() {»
  - 2861-2868 @2864 «fn check_statusline_override_fails_when_run_b_ran_and_its_status_line_filed_nothing() {»
  - 2870-2880 @2871 «fn check_statusline_override_fails_with_no_capture() {»
  - 2882-2888 @2883 «fn check_statusline_shell_passes_when_every_payload_ran_through_bin_sh() {»
  - 2890-2895 @2891 «fn check_statusline_shell_fails_on_one_other_shell_among_bin_sh() {»
  - 2897-2900 @2898 «fn check_statusline_shell_fails_with_no_capture() {»
  - 2902-2919 @2903 «fn check_statusline_rate_limits_passes_on_two_whole_number_windows() {»
  - 2921-2978 @2922 «fn check_statusline_rate_limits_fails_on_each_broken_window() {»
  - 2980-2986 @2981 «fn check_statusline_rate_limits_fails_with_no_capture() {»
  - 2988-2996 @2989 «fn shell_verdict_reads_only_the_exact_word() {»
  - 2998-3021 @2999 «fn statusline_variants_name_the_first_without_and_the_first_with_rate_limits() {»
  - 3023-3043 @3024 «fn statusline_capture_file_names_round_trip() {»
### crates/viola-agent-claude/src/statusline.rs — added 77 line(s) in 4 range(s)
added: 21-27 · 113-131 · 139 · 402-451
- 113-119 @117 «pub fn probe_document(pinned_bin_fwd: &str, captures_dir_fwd: &str) -> Option<String> {»
- 121-130 «fn probe_on(pinned_bin_fwd: &str, captures_dir_fwd: &str, wrapped: bool) -> Option<String> {»
  - 123-125 «if !wrapped || !plain(pinned_bin_fwd) || !plain(captures_dir_fwd) {»
  - 126-128 «let command = format!(»
  - 402-422 @403 «fn statusline_probe_document_runs_the_capture_arm_with_the_shell_word() {»
  - 424-437 @432 «fn statusline_probe_document_of_a_path_a_shell_could_misread_is_none(»
  - 439-450 @440 «fn statusline_probe_document_where_the_status_line_is_not_wrapped_is_none() {»
### crates/viola-e2e/src/harness/run.rs — added 86 line(s) in 7 range(s)
added: 333-358 · 941-964 · 985-993 · 1026-1041 · 1078 · 1081-1086 · 1093-1096
  - 941-963 @942 «const LIVE_PASS: &str = "[01/21] shim-resolution claude resolves  pass\n\»
  - 985-992 @986 «fn without(row: &str) -> String {»
  - 1026-1040 @1029 «fn run_local_live_reads_the_relied_rows() {»
### fixtures/claude/2.1.287/StatusLine.default-1.json — new file · 1 line(s)
### fixtures/claude/2.1.287/StatusLine.rate-limits-1.json — new file · 1 line(s)
### schemas/claude-statusline-fixture.v1.json — new file · 19 line(s)
- 1-19 «{»
  - 8-18 «"properties": {»
### src/bin/viola-fake-agent.rs — added 139 line(s) in 14 range(s)
added: 13 · 19-20 · 22 · 40-41 · 352-365 · 404-405 · 414 · 436-440 · 442 · 518-536 · 639-641 · 654-656 · 950 · 1452-1535
- 352-364 @354 «fn shell_expanded(args: Vec<String>) -> Vec<String> {»
  - 355-363 «args.into_iter()»
  - 518-535 @522 «fn replay_statusline(&self, name: &str) {»
  - 1452-1490 @1455 «fn statusline_replayed(command: &str, variants: bool, extra: &[&str]) -> Vec<Value> {»
  - 1492-1502 @1493 «fn fake_statusline_replays_the_default_then_the_rate_limits_once_under_framing() {»
  - 1504-1508 @1505 «fn fake_statusline_replays_nothing_without_framing_or_without_the_variants() {»
  - 1510-1520 @1511 «fn fake_statusline_replay_of_a_relative_command_runs_nothing() {»
  - 1522-1534 @1523 «fn fake_statusline_hands_the_shell_name_word_over_as_bin_sh() {»
### src/cmd/hook.rs — added 139 line(s) in 6 range(s)
added: 20-21 · 23 · 47-49 · 101-107 · 404-436 · 1251-1343
- 404-435 @408 «fn capture_statusline(dir: &Path, shell: Option<&OsStr>, stdin: &mut dyn Read) -> Option<u32> {»
  - 409-411 «if !dir.is_absolute() || !dir.is_dir() {»
  - 414-416 «if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {»
  - 417-421 «let verdict = if shell == Some(OsStr::new(BIN_SH)) {»
  - 423-430 «let path = loop {»
  - 1251-1259 «fn names_in(dir: &Path) -> Vec<String> {»
  - 1261-1290 @1262 «fn capture_statusline_files_each_payload_and_its_shell_verdict_under_the_next_k() {»
  - 1292-1307 @1293 «fn capture_statusline_reads_the_shell_word_byte_for_byte() {»
  - 1309-1321 @1310 «fn capture_statusline_files_a_payload_at_the_cap_and_nothing_past_it() {»
  - 1323-1342 @1324 «fn capture_statusline_refuses_a_bad_dir_or_stdin_and_writes_nothing() {»
### src/cmd/verify.rs — added 203 line(s) in 20 range(s)
added: 27 · 29 · 216 · 226-228 · 245 · 281-283 · 310-314 · 334-341 · 408-442 · 444-447 · 449 · 472-474 · 502-507 · 510
       514-517 · 520-526 · 529-531 · 536 · 541-542 · 641-753
  - 334-336 «fn statusline_captures(&self) -> PathBuf {»
  - 338-340 «fn statusline_settings(&self) -> PathBuf {»
- 411-440 @413 «fn read_statusline(dir: &Path) -> Vec<StatusLineCapture> {»
  - 414-422 «let mut found: Vec<(u32, PathBuf)> = std::fs::read_dir(dir)»
  - 424-439 «found»
  - 641-684 @644 «fn step_lines_count_to_the_relied_total() {»
  - 686-712 @687 «fn read_statusline_orders_by_k_and_reads_each_shell_verdict() {»
  - 714-720 @715 «fn read_statusline_reads_one_byte_past_the_cap() {»
  - 722-752 @723 «fn probe_dir_holds_the_statusline_dir_and_on_unix_its_probe_settings() {»
### src/cmd/verify/typed.rs — added 18 line(s) in 5 range(s)
added: 54-55 · 63 · 111-112 · 120 · 147-158
- 147-157 @148 «fn trusted_args(args: &[OsString], inputs: &Inputs<'_>) -> Vec<OsString> {»
  - 152-155 «if let Some(settings) = inputs.statusline_settings {»
### src/run/version_gate.rs — added 36 line(s) in 4 range(s)
added: 260-265 · 310-327 · 336 · 347-357
### tests/cli_verify.rs — added 211 line(s) in 36 range(s)
added: 2-4 · 27-51 · 72-96 · 117-127 · 158-180 · 182-184 · 188 · 199-204 · 242-243 · 253-258 · 332-333 · 369-376 · 404
       486 · 598-629 · 631 · 634-639 · 680-691 · 716 · 829 · 1066-1067 · 1075-1078 · 1120-1121 · 1129-1132 · 1171-1173
       1176-1178 · 1186-1189 · 1195-1196 · 1200-1202 · 1220-1221 · 1233 · 1236-1238 · 1265-1269 · 1271 · 1305 · 1359
- 27-50 @28 «const STEPS_PASS: [&str; 21] = [»
- 72-95 @73 «const ROW_IDS: [&str; 21] = [»
- 122-127 @123 «const STATUSLINE_ROWS: [&str; 3] = [»
- 158-171 @159 «fn steps_failing(failing: impl Fn(usize) -> bool) -> Vec<String> {»
  - 160-170 «STEPS_PASS»
- 173-179 «fn all_pass_stdout() -> String {»
  - 200-204 «assert_eq!(»
  - 372-376 «assert!(»
  - 598-625 «let mut expected = vec![»
  - 626-628 «if cfg!(windows) {»
  - 680-691 @681 «{»
  - 1200-1202 «for id in ROW_IDS.iter().filter(|id| STATUSLINE_ROWS.contains(id)) {»
  - 1236-1238 «for id in &ROW_IDS[18..] {»
  - 1265-1269 «let summary = if cfg!(unix) {»
### tests/contract_fake_agent_drift.rs — added 106 line(s) in 2 range(s)
added: 5-6 · 383-486
- 383-386 @385 «const STATUSLINE_VARIANTS: [&str; 2] =»
- 388-395 @390 «fn statusline_replay(set: &Path) -> Vec<(String, Vec<u8>)> {»
  - 391-394 «STATUSLINE_VARIANTS»
- 397-415 @399 «fn statusline_drift(set: &Path, receipt: &[Value]) -> Vec<String> {»
  - 401-404 «let sent: Vec<Vec<u8>> = of_kind(receipt, "statusline")»
  - 405-410 «let mut drifted: Vec<String> = expected»
  - 411-413 «if expected.len() != sent.len() {»
- 417-451 @422 «fn fake_agent_statusline_replay_matches_every_recorded_statusline_set() {»
  - 425-449 «for version in recorded_versions(&root) {»
- 453-485 @454 «fn statusline_drift_names_a_changed_byte_a_swapped_order_and_a_short_replay() {»
  - 459-462 «let clean: Vec<Value> = statusline_replay(&set)»
  - 471-474 «assert_eq!(»
  - 478-481 «assert_eq!(»
### tests/contract_fixture_hygiene.rs — added 132 line(s) in 7 range(s)
added: 2-4 · 32-33 · 36-51 · 55-63 · 110-206 · 215 · 239-242
- 36-40 «fn named(path: &Path, prefix: &str) -> bool {»
  - 37-39 «path.file_name()»
- 42-44 «fn is_screen(path: &Path) -> bool {»
- 46-48 «fn is_statusline(path: &Path) -> bool {»
- 110-125 @111 «fn statusline_fixtures_pass_hygiene() {»
  - 116-124 «for path in walked {»
- 127-147 @130 «fn statusline_variants_are_walked_for_every_statusline_set() {»
  - 134-146 «for set in sets {»
- 149-155 «fn statusline_payload(extra: &Value) -> Vec<u8> {»
  - 151-153 «if let (Some(doc), Some(extra)) = (doc.as_object_mut(), extra.as_object()) {»
- 163-172 «fn planted_clean_statusline_payload_passes(#[case] extra: Value) {»
  - 164-171 «assert_eq!(»
- 174-186 @181 «fn planted_statusline_schema_violation_is_rejected(#[case] doc: Value) {»
  - 182-185 «assert_eq!(»
- 188-196 @191 «fn planted_statusline_absolute_path_is_rejected(#[case] extra: Value) {»
  - 192-195 «assert_eq!(»
- 198-205 @199 «fn planted_statusline_username_is_rejected() {»
  - 201-204 «assert_eq!(»
  - 239-242 «assert_eq!(»
### tests/contract_ledger_probes.rs — added 32 line(s) in 4 range(s)
added: 2-3 · 16-40 · 61-62 · 129-131
- 16-39 @17 «const ROW_IDS: [&str; 21] = [»
### tests/hook_fail_open.rs — added 83 line(s) in 1 range(s)
added: 387-469
- 387-438 @396 «fn hook_statusline_capture_files_the_payload_and_the_shell_verdict(»
  - 406-408 «if let Some(word) = word {»
  - 419-423 «assert_eq!(»
  - 424-429 @425 «for path in [&json, &shell] {»
  - 430-433 «assert!(»
  - 434-437 «assert!(»
- 440-468 @443 «fn hook_statusline_capture_files_nothing_it_cannot_take() {»
  - 449-452 «let (captures_arg, missing_arg) = (»
  - 454-463 «for (dir, stdin) in [»
### tests/support/home.rs — added 8 line(s) in 2 range(s)
added: 375-377 · 386-390
  - 387-390 «let expected = format!(»
### tests/support/verify.rs — added 10 line(s) in 3 range(s)
added: 83-84 · 97-102 · 109-110
  - 99-102 «limited["rate_limits"] = json!({»
