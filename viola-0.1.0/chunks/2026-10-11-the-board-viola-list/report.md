# Report — 2026-10-11-the-board-viola-list

**Chunk:** viola list for wrapped sessions: the read model over instances, list --json (items, budget, skipped) and
the uncoloured board rows; no unwrapped rows, no colour
**Date:** 2026-10-11T08:25Z
**Commits:** `f1ee634` chore(2026-10-11-the-board-viola-list): operator pre-CI commit, for the run this chunk's verdict
reads · `acad7af` fix(2026-10-11-the-board-viola-list): operator fix after CI run 38123340339, the four list readings
on an unstamped home start from a stamped one (`git log --format='%h %s' 995dc55a3d6b..HEAD`; the basis
`995dc55a3d6b` is the parent of the oldest pre-CI commit)

## Changes (structured — detectors read this)

- **Files:** new `crates/viola-state/src/board.rs` · `src/cmd/list.rs` · `tests/cli_list.rs` ·
  `tests/chaos_forward_compat.rs`; modified `crates/viola-state/src/lib.rs` · `crates/viola-state/src/budget.rs` ·
  `src/cmd/mod.rs` · `src/cmd/revive.rs` · `src/human.rs` · `src/main.rs` · `tests/chaos_torn_append.rs` ·
  `tests/cli_budget.rs` · `.config/nextest.toml`. Not modified: `src/run/budget.rs` (the plan's conditional entry:
  `decide` and `read_thresholds` are `pub(crate)` and reachable from `src/cmd/list.rs`). Basis: `gate.py scope` at
  this wrap, `clean — changed 13 · listed 13 · recorded 0`, base `995dc55a`. Line ranges of every added block: the
  section `## New text, by line` below (`cites.py added`).
- **Symbols / APIs:**
  - **`viola list [--json]`**, a new top-level subcommand (`src/cmd/mod.rs` `Command::List`, help "Show every wrapped
    session: its liveness, wheel, dialog and CLI, and the usage budget"). It is no channel method: the calling process
    reads `<home>/instances/` itself, contacts no wrapper and connects to no endpoint. Its order (`src/cmd/list.rs`
    `list`): the home made absolute with `std::path::absolute` (not canonicalised) → `board::scan` (every check, no
    file under the home opened) → only when `VIOLA_NAME` parses, `client::start(home, name)` (the `cli` role file and
    `process-start`) → under the span `cli.list` (`#[instrument(skip_all, name = "cli.list", fields(json = json))]`,
    `read`) one `SystemTime::now()`, `board::read`, `viola_state::budget::read_budget_stamped`,
    `run::budget::read_thresholds` (each rejection logged through `obs::log_config_rejection`, as `viola run` does) and
    `run::budget::decide(Some(&reading), thresholds, None, now)` → the document or the board as ONE stdout write →
    `run::log_self_exit(0, None)`, exit 0.
  - **Exit codes of `list`:** 0 a board · 21 the strict-modes refusal of the home, `instances/` or any one instance
    (the whole board refused; `--json`: stdout exactly `{"v":1,"error":"instance-unreachable","detail":null}`, stderr
    empty; human: stderr the pair, the hint last, stdout empty) · 1 an internal error (an `instances/` that cannot be
    listed, a log start that fails), through the `cli` catch site. A refusal opens no log, writes no line and leaves the
    home's mode as found (`inputs#I2`, the operator's answer, not a ruling of the founder).
  - **The two refusal pairs** (`src/human.rs`): `instance_state_pair(name)` = `unable: <name>'s state files can be
    written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`, moved
    from `src/cmd/revive.rs` `Refusal::pair`, words unchanged, now called by `revive` and `list`;
    `home_state_pair()` = `unable: the viola home is not private to you` / `hint: the viola home must be readable only
    by you; viola does not change its permissions` (design-system §Surface: cli, Component Patterns 2; its first
    writer in source).
  - **`--json` document:** `{"v":1,"ok":{"items":[…],"budget":…,"skipped":{…}}}`. Item keys in order `name, wrapped,
    liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?`; an absent optional is no
    key. `wrapped` is always `true`; `status` is always `"unknown"`; `liveness` is `live` or `stale`. `budget` is
    `{five_hour, seven_day, read_at, paused}` (each window `"unknown"` or `{used_percentage, resets_at}` as
    `budget.json` holds it; `read_at` a string or `"unknown"`), or the word `"unknown"` when there is no reading.
    `skipped` is `{unknown_kinds, unknown_fields, torn_lines}`.
  - **`viola_state::board`** (new module, `crates/viola-state/src/board.rs`; sync, std only, no Claude shape):
    `scan(home) -> Result<Result<Scan, ScanRefused>, StateError>` (lines 50-98) and `read(&Scan, now: SystemTime) ->
    Board` (lines 204-261); types `Scan` (private fields), `ScanRefused{Home, Instance(ViolaName)}`, `RowLiveness{Live,
    Stale}`, `SessionStatus{Unknown}`, `Item` (with `#[serde(skip)] dialog_kind`), `SkippedTotal`, `Board{items,
    skipped}`. No type of the module holds `endpoint`, `pinned_bin`, `cwd`, `statusline_command`, `pid`, `started_at`
    or `links`.
    - `scan`: `check_path` on the home and on `instances/`, each that exists (a private `check_if_present`: absent
      passes, unstattable is refused); `instances/`'s entries by `DirEntry::file_type` (a link is never followed):
      a link → counted `links_ignored`; a directory whose UTF-8 name passes `ViolaName::try_new` → an instance;
      anything else → counted `malformed`; a path is joined only from a parsed name; then `check_instance(home, dir)`
      per instance in name order, the first failure named. An absent home or `instances/` passes with no instance.
    - `read`: per instance, `read_snapshot_or_replay`. A snapshot that reads → `classify(beat_age(dir, now),
      same_process(&snapshot))`; `Gone` gives no item, `Live`/`Stale` an item with the snapshot's `wheel`,
      `budget_paused`, `dialog_pending` (= `pending_dialog.is_some()`), `cli_version` when recorded, `cli_verified`,
      and the pending dialog's kind kept as `dialog_kind`. A replay → an item only for a beat at most `STALE_AFTER`
      (5 s) old: `live`, `wheel` / `budget_paused` only where the log gave one, `dialog_pending:false`, no CLI field;
      never `stale`. An absent snapshot, or a replay that fails, → no item. Skip counts: a replay's own `skipped`, else
      one drained `read_from(dir, 0)` pass (what it read before an error, on an error); summed over every scanned
      instance, gone ones included. After the loop, one `parse-rejected{parser:"state-entry", detail:"symlink-ignored"|
      "malformed", count}` per nonzero scan count, naming no entry.
  - **`viola_state::budget::read_budget_stamped(home) -> Option<StampedReading>`** (new; `StampedReading{reading:
    BudgetReading, read_at: Reading<String>}`): the same `check_path` on home and file, shared lock, `take(MAX_FRAME +
    1)`, tolerant parse, `v` = 1, in the same order; `read_at` is `"unknown"` when the key is absent or no string.
    `read_budget(home)` keeps its signature and becomes `read_budget_stamped(home).map(|s| s.reading)`.
  - **`src/human.rs`:** `escape_row` (every control character, `\n` `\t` DEL and C1 included, as `\xHH` text; beside
    `escape_message`, which keeps its 6 product callers: 5 in `src/human.rs`, 1 in `src/cmd/revive.rs`, `grep -n
    'escape_message(' src/*.rs src/cmd/*.rs`); `BayBudget`, `bay_line`, `board_rows`, `write_board` (one write: BAY
    line, caption row, rows; or BAY line, caption row, `no wrapped sessions  start one: viola run <name> -- claude`).
  - **Remaining-caller facts** (product code, read at this tree): `read_budget` — 1, `BudgetGate::decide_at`
    (`src/run/budget.rs`), unchanged; `read_budget_stamped` — 2 product paths: `read_budget` and `cmd::list::read`.
    `read_snapshot_or_replay` — 2 product readers: `viola revive`'s preflight and `board::read` (`viola list`).
    `check_instance` — 3: `revive` `trusted`, `hook statusline` `trusted`, `board::scan`. `events::read_from` — gains
    `board::log_pass`, one pass per instance whose snapshot read or was absent. `read_thresholds` — 2: `viola run`'s
    start and `cmd::list::read`. `decide` — 2: `BudgetGate::decide_at` and `cmd::list::budget_of`.
  - **Board forms** (`src/human.rs`, unit cases `bay_line_*` / `board_row_*`): BAY cells two spaces apart in the
    order `BAY`, `5H <n> %` (+ ` expired` once a known `resets_at` is not later than `now`) or `5H unknown`, the same for
    `7D`, `read <n>m ago` or `read unknown`, `gate open` / `budget-paused`, `skipped a - b - c`. Caption row `NAME LIVE
    STATUS WHEEL DIALOG CLI`; columns padded with a two-space gutter: NAME to the longer of 4 and the longest name,
    LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI last and unpadded; no line ends in a space; nothing truncated. Cells:
    LIVE `live`/`stale`; STATUS `unknown`; WHEEL the holder or `unknown` (a replay with no logged holder), + `
    - budget-paused` when the item's `budget_paused` is true; DIALOG `none`, `DIALOG question|permission|plan`, else
    `DIALOG unknown`; CLI `<escaped version> verified|unverified-cli`, or `unknown` when no version is recorded.
  - **Planning forms, worded by no master** (plan step 3, on the operator's word at P5, `inputs#I3`; none ruled as
    phraseology by the founder): a window's figure rounded down to a whole number (`89.6` → `89 %`); the age in whole
    minutes rounded down at every age (`read 0m ago`, `read 75m ago`), `read unknown` for no instant, one that does
    not parse, or one ahead of `now`; nonzero skip counts as bare numbers in key order; a `human` holder with a pause
    prints `human - budget-paused`; the empty board prints the BAY line and the caption row above its line.
  - **Not built** (the cut, `inputs#I1`): no read, spawn or parse of `claude agents --json`; no unwrapped row and no
    `-- UNWRAPPED - READ-ONLY --` separator; no colour (no SGR, no read of `NO_COLOR` / `TERM` / `COLORTERM`, no
    `IsTerminal` branch); no link column and no link anywhere; no grouped `--help` table (clap's ordered list).
- **Crates / modules:** `viola-state` gains `pub mod board` (`crates/viola-state/src/lib.rs:6`); the root bin gains
  `cmd::list` (`src/cmd/mod.rs:5`). No crate added or removed; the crate dependency direction is unchanged (the read
  model sits in `viola-state`, which `viola-mcp` / `viola-ui` may reach; nothing of it is in the root bin).
- **Dependencies:** none added or bumped (`git diff 995dc55a3d6b -- Cargo.toml Cargo.lock`: empty; the preservation
  guard, entry 6, green). `tests/cli_list.rs` uses `sha2`, already a root dev-dependency.
- **Schema / config:** no schema file changed (`schemas/` under the preservation guard). The lines `list` writes use
  only enum values `schemas/diag-line.v1.json` already holds (`parser` `state-entry`, `detail` `symlink-ignored` and
  `malformed`, lines 102-103; `state-recovered` details `snapshot-replayed` / `snapshot-unsupported-v`). `config.json`'s
  `five_hour` / `seven_day` gain a second reader (`list`, through `read_thresholds`); a bad value under a
  `VIOLA_NAME` writes `parse-rejected{parser:"config-json"}` lines beside the self pair. `budget.json` gains no
  reader beside the one checked read (its second form).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `[profile.ci]` 20 s kill filter line: 15 → 18 binaries (`binary(` occurrences on `.config/nextest.toml:32`, at
    `995dc55a3d6b` and now): `cli_list` (step 8), and `chaos_forward_compat` and `chaos_torn_append`, which start from
    `stamped_home` since the CI fix. The rule stays "every binary that drives verify".
  - Root test files: 39 → 41 (`tests/*.rs`, `git ls-tree` at the basis vs now).
  - Product readers of the snapshot fallback (`read_snapshot_or_replay`): 1 → 2; `check_instance` callers 2 → 3;
    `read_thresholds` callers 1 → 2; the first product reader of the three skip counts (`list`'s `skipped`; before,
    `replay` and `session_chain` carried them out and no product code read them).
  - `EventKind::ALL` 15, unchanged. Masters stating "one product reader" / "one caller" of these: architecture lines
    50, 247, 258, 384, 394, 408; security-plan 122, 213, 241; test-plan 952; obs-plan 731 (pattern `(one|sole|only)
    (product )?(caller|reader)`, scratch script, each hit to be read by the detector — not every hit is one of these
    functions).
  - Unit suite on the dev host: 1972 tests across 9 binaries (`run --unit`, firing 3's log). CI `test (windows-2025)`
    on `f1ee634`: 2402 tests.
- **Dev-tool versions:** none — no host tool installed or upgraded.
- **Harness / gate surface:** none changed. The harness `status` document's `list` member stays `null`: `status` runs
  no `viola list` (`crates/viola-e2e` untouched, the preservation guard green). The key file
  `registries/contracts/test-plan/status-endpoint-shape.md` says those members are `null` "while `viola list` / `viola
  ui` are unbuilt" — `viola list` is now built; the member's owner is not this chunk (scope.md "Boundaries").
- **Cross-project / external claims:**
  - CI: `ci#38123340339` on `f1ee63419e14` (the pre-CI commit) — **red**, 15/15 checks, `test (windows-2025)` failed:
    4 cases, each `viola list --json` exit 21 where 0 was asserted (Outcome, gates). `ci#38124062614` on
    `acad7afc865a` (the fix commit, the final HEAD of the pass) — **green**, 15/15, wall 511 s. Basis: `ci.py
    conclusion --sha HEAD --wait 1800` lines in `evidence/operator.md`.
  - Inputs (`inputs.py verify` at this wrap, 5 entries, 0 drifted · 0 vanished · 0 broken · 5 n/a, each a message):
    `I1` the operator's direction for the take-up (the cut, the founder-only set, logged link names parsed again, no
    mutation / witness / dispatch, the CI read by `ci.py conclusion`) · message · n/a; `I2` the operator's answer to
    the P4 fork (a failed strict-modes check refuses the whole board, exit 21, no log) · message · n/a; `I3` the
    founder's approval of the card, relayed by the operator at P5 · message · n/a; `I4` the operator's direction for
    implement (no live `claude` start, each long gate run journalled before its wait, the operator pass with the
    `ci.py conclusion` read) · message · n/a — cited here; `I5` this wrap's direction (Phase 1 only, the resume point
    naming Phase 2, then stop; the epoch-close reconcile and consolidation in a fresh window) · message · n/a — cited
    here. Before this report existed, `verify` printed `I4` and `I5` `UNCITED`.
- **Reverted / negative API facts:** none. (Test-side only: `tests/cli_list.rs` `tree()` first held each file's bytes
  and was changed to its SHA-256, because a red printed the pinned `viola` copy whole.)
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **test-plan.md:591** — "Human-mode C0/C1 escaping in `list`/`wait`/`last`, keeping `\n` and `\t`." For `list` the
     landed rows escape `\n` and `\t` as `\x0A` / `\x09` (`escape_row`; `escape_row_shows_every_control_character_as_
     hex_text` cases `lf` and `tab`, `board_row_cli_cell::case_5_controls`). design-system.md:747 ("In `list` table
     rows, `\n` and `\t` …") and layout-templates.md:580 ("including `\n` and `\t` inside a row") already say the
     row escapes them; test-plan's line contradicts both for `list` (true for `wait` / `last`).
  2. **The plan's WHEEL width 22** (plan step 3) against its own rule "padded to their widest word": `unknown -
     budget-paused` (a replayed row with a logged pause and no logged holder) is 23. Measured by
     `board_row_every_value_starts_in_its_captions_column` (the replayed row). Plan text only; no master states 22
     (`grep -n 'WHEEL' .andromeda/layout-templates.md .andromeda/design-system.md` — to be read by the detectors).
  3. **research.md "The budget readings are Unix root cases"** (`budget.json` written only by `hook statusline`,
     "which runs a command on Unix only"): `path6_list_shows_the_reading_and_the_pause` passed on `test
     (windows-2025)` in both CI runs (`ci#38123340339` log, `PASS … cli_budget path6_list_shows_the_reading_and_the_
     pause`). The arm writes `budget.json` from a payload on Windows too; only the user's command is Unix-only.
     Research text only; the masters' sentences on the statusline's Windows behaviour are the detectors' to read.
  4. **The key file `status-endpoint-shape.md`** — "Interim, while `viola list` / `viola ui` are unbuilt: `list` … are
     `null`": `viola list` is built; the member stays `null` for another reason (`status` runs no `viola list`).
- **Expected amendments (from plan):** sites by the scratch script `sites.py` (one regex per call, hits counted per
  master body and per file under `.andromeda/registries/`, first line numbers kept), at this tree:
  1. **architecture §Standard Contracts** — `list` as landed (no `claude agents --json` read, `status` `unknown`,
     wrapped rows only); the caller sentences of the replay, `check_instance`, `read_budget`, `events::read_from`; the
     sentence that owes the counts to this entry; `list` as a reader of `config.json`'s thresholds; the subcommand and
     tree lines. **Carried:** Changes → Symbols / APIs, Remaining-caller facts, Not built, Schema / config, Crates /
     modules. Sites: `viola list` architecture 3 (50, 71, 139); `list --json` 1 (50); `claude agents --json` 10 + key
     `crate-dependency-direction.md` 1; `read_snapshot_or_replay` 1 (247); `check_instance` 2 (328, 407) + key
     `project-directory-structure.md` 1; `read_budget` 1 (408) + the same key 1; `read_from` 3 (50, 247, 262) + the same
     key 1; `[Nn]o surface (shows|reads|reports)` 1 (50); `` `five_hour` `` 4 (72, 216, 400, 465).
  2. **security-plan §Authentication & Authorization and §Input Validation** — `list` as a strict-modes entry point;
     its refusal recorded as the operator's answer, not ratified; the "one caller" sentences; no line on a refused
     check (the operator's answer, the founder not having ruled). **Carried:** Changes → Symbols / APIs (Exit codes,
     `scan`), Remaining-caller facts. Sites: `check_instance` security-plan 3 (213, 236, 241); `read_budget` 2 (213,
     241); `read_snapshot_or_replay` 1 (241); `read_from` 1 (241); `(one|sole|only) … (caller|reader)` 3 (122, 213,
     241); `exit 21` 3 (209, 211, 213); `claude agents --json` 8 (50, 127, 243, 245, 254, 437 …; 245 says `list`
     resolves `claude` for `claude agents --json` — not landed).
  3. **design-system §Surface: cli and §Color Palette** — the closed set of verbs that print the fixed `unable:` form
     gains `list` with exit 21; the five planning forms, each marked a planning form on the operator's word; the row
     mode's reach. **Carried:** Changes → Symbols / APIs (the two refusal pairs, Board forms, Planning forms), Spec
     claims disproved 1. Sites: `not private to you` design-system 3 (772, 780, 824); `exit 21` 2 (769, 787); `\bBAY\b`
     7 (234, 238, 604, 608, 615, 736 …); `viola list` 10 (79, 135, 354, 734, 768, 770 …); the row-escape sentence at
     747 (`[Rr]ow mode`: 0 hits in all seven masters — the masters word it "C0/C1 … escaped", found by
     `grep -n -i 'C0/C1'`).
  4. **layout-templates §Surface: cli** — Output structure — `viola list` as landed (the styled terminal form and the
     unwrapped rack not built, each with its owner by title); the NAME width rule; the catch-site verb list; the same
     planning forms. **Carried:** Changes → Board forms, Not built. Sites: `viola list` layout-templates 20 (233, 319,
     353, 357, 367, 370 …); `\bBAY\b` 11 (70, 108, 131, 151, 188, 357 …); `list --json` 2 (397, 400); `torn_lines` 1
     (401); `exit 21` 4 (353, 442, 587, 588); the row-escape sentence at 580; `NO_COLOR` 2 (347, 385).
  5. **test-plan** — §3 `status`: the `list` member stays `null`, with its owner by title; §4 root bin, the escaper
     line for `list`; §6 Scenario E5, Chaos suite and Path 6 as landed; the literal pin in place of trycmd; the
     kill-line and root-wait counts with their rules. **Carried:** Changes → Harness / gate surface, Counts /
     qualifiers moved, Spec claims disproved 1 and 4; Outcome. Sites: key `status-endpoint-shape.md` (`viola list` 2,
     the interim sentence at its line 19); `viola list` test-plan 16 (73, 186, 203, 230, 387, 689 …) + keys
     `5-command-implementation.md` 2, `bootstrap-phases-derive-for-route-setup-project.md` 1; `Scenario E5|\bE5\b` 2
     (485, 941) + key `5-command-implementation.md` 1; `torn_lines` 6 (188, 202, 350, 378, 549, 1022); `[Ff]orward[-
     ]compat` 1 (1027); `trycmd` 11 (442, 463, 611, 655, 656, 689 …) + key `bootstrap-phases-…` 1; the escape line 591;
     `fifteen|15 binaries|verify-driven` 0 in the body + key `bootstrap-phases-…` 1 (the kill line's count lives in
     a key file). Root waits (`Instant::now() + WITHIN` under `tests/`): not re-counted by this report — the new
     cases add no such wait (they call `support::` helpers only); the detector reads the stated figure.
  6. **obs-plan §4 Instrumentation per surface and Edge flows (E5); §10** — `cli.list`, the caller counts, the scan
     line's first call site; a refused `list` check opens no log (the operator's answer, not ruled by the founder).
     **Carried:** Changes → Symbols / APIs (`cli.list`, the `parse-rejected` line, the refusal), Remaining-caller
     facts. Sites: `cli\.list` obs-plan 1 (516, already named as an example); `state-entry` 4 (532, 832, 839, 841);
     `Scenario E5|\bE5\b` 4 (347, 426, 725, 731); `read_snapshot_or_replay` 1 (731); `check_instance` 2 (835, 841) +
     key `otel-sdk-init.md` 1; `read_budget` 2 (394, 699); `list --json` 2 (775, 1077).
  7. **a11y-plan §1 CLI verbs, §6 and §8** — `list` as landed, the caption pin by literal. **Carried:** Outcome (the
     a11y criteria), Changes → Board forms. Sites: `viola list` a11y-plan 4 (286, 594, 859, 1022); `\bBAY\b` 2 (173,
     498); `trycmd` 14 (165, 198, 412, 414, 527, 543 …) + keys `a11y-testing-tool-pick.md` 1,
     `keyboard-test-harness.md` 1; `NO_COLOR` 7.
- **Coverage of new surfaces:**
  - `viola list` (CLI verb, human form) → validation ViolaName on every entry name before a join, strict-modes
    before any open ✓ · instrumentation `cli.list` span + `process-start`/`process-exit` under a name ✓ · PII
    `endpoint`/`cwd`/`statusline_command`/`pinned_bin` on no type, the version escaped ✓ · tests unit + integ ✓ · a11y
    ASCII-only, no ESC, caption pinned by literal ✓ · tokens n/a (no colour layer; the domain words only)
  - `viola list --json` (machine form) → validation as above ✓ · instrumentation as above ✓ · PII as above ✓ · tests
    unit literal + integ ✓ · a11y n/a · tokens n/a
  - `viola_state::board::{scan, read}` (read model) → validation symlink not followed, `ViolaName::try_new`,
    `check_instance` per instance ✓ · instrumentation `parse-rejected{state-entry}` per nonzero count;
    `state-recovered` through the replay ✓ · PII no entry name logged ✓ · tests unit (`board_scan_*`, `board_read_*`) ✓
    · a11y n/a · tokens n/a
  - `read_budget_stamped` → validation the same checks as `read_budget` ✓ · instrumentation none, as `read_budget`
    (no span, no line, obs rule) ✓ · PII n/a · tests unit `budget_stamped_*` ✓ · a11y n/a · tokens n/a

## Deviations from intent

1. **WHEEL column 23, not 22** (plan step 3): the step's own rule pads each column to its widest word, and `unknown -
   budget-paused` is 23. Followed the rule; a replayed row with a pause stays aligned.
2. **Two `role_of` rows** (plan step 5 says one row for `list` and `--home <dir> list`): `#[case::list(&["list",
   "--json"], …)]` and `#[case::list_after_home(&["--home", "C:/h", "list"], …)]` (`src/main.rs:338-339`).
3. **The scan reads each entry's type with `DirEntry::file_type`**, where plan step 2 names `symlink_metadata`: the same
   unfollowed type, and no path is joined to read it, which the step's "no path is joined from a name that did not
   parse" asks for.
4. **Two failure paths the plan did not word:** an `instances/` that exists and cannot be listed (or an entry whose
   type cannot be read) is a `StateError` → exit 1 through the catch site, not a refusal; a per-instance
   `read_snapshot_or_replay` error gives no item and the log pass's counts so far.
5. **Every `cli_list` case but the two `cfg(unix)` refusals and the absent-home case starts from `stamped_home`, and
   `chaos_forward_compat` and `chaos_torn_append` do too** — after `ci#38123340339`. Implement first booted four cases
   from `StampedHome::unstamped` for speed (a deviation from step 6, which names `stamped_home`); on Windows x64
   `seed_conpty` creates such a home with the runner's inherited DACL, and the board refused it (exit 21). The
   fix put them back on `stamped_home`, which moved both chaos binaries onto the 20 s kill line by its rule (step 8
   named only `cli_list` and a conditional `chaos_forward_compat`). The two refusal cases stay unstamped: `cfg(unix)`,
   where `seed_conpty` is a no-op.
6. **Test instants dated 2025-03-01** in the `src/human.rs` and `src/cmd/list.rs` unit cases: the stamp-ahead hook
   read today's synthetic instants as record stamps ahead of the clock.
7. **`src/run/budget.rs` not edited** — the plan's conditional entry; nothing needed it.
8. **Wording of my own:** the `list` help line and the span's `fields(json = json)`.

Scope record: none — `gate.py scope` clean, 0 recorded (`changed 13 · listed 13`).

## Decisions & corrections

- Operator directions: `inputs#I4` (implement: no live `claude`; journal each long gate run before its wait; the
  operator pass with the `ci.py conclusion` read) and `inputs#I5` (this wrap: Phase 1 only, then a resume point naming
  Phase 2; the epoch-close reconcile and consolidation in a fresh window). No user correction of the agent's work in
  this session; no "from now on" convention given.
- **On Windows x64 an unstamped test home is not viola-made:** `tests/support/home.rs` `seed_conpty` creates the home
  with `fs::create_dir_all` before `viola run`, so any reader that runs the strict-modes check on the home (`list`,
  `revive`, `hook statusline`) refuses it there. A root case that runs such a reader starts from `stamped_home`.
  Found by CI only: `seed_conpty` is a no-op on the Linux dev host, and `pre-push` read green on the red tree.
- **The first unit run after a reboot:** `target/e2e-home` is a link to a tmpfs directory a reboot empties; the harness
  case `remove_session_home_removes_a_boot_created_parent` calls `fs::create_dir_all` on the link itself and fails
  `AlreadyExists` while the link dangles (std's behaviour, measured two-sided in a scratch program); a parallel
  case's `ensure_e2e_home` re-made the target mid-run (birth 2026-10-11 07:50:10Z, host boot 07:18:11Z).
- **Remove-the-guard runs and readable reds:** a tree-equality assert over a viola home must compare hashes, not
  bytes — a home holds the pinned `viola` copy, and one red printed it whole (`evidence/red-green.md`).
- **The Windows target's clippy catches what the host's cannot:** a `cfg(unix)`-only constant read dead on
  `x86_64-pc-windows-msvc` (`CARGO_TARGET_DIR=target/wincheck`), fixed before the push.
- **Sweep hazards:** "row mode" appears in no master (0 hits); the masters word the row escape as "C0/C1 controls
  escaped … including `\n` and `\t`". The status document's `list` member is worded "Its nested `list` and `ui`
  members", not `"list": null` (0 hits for that pattern). The kill line's count lives in a test-plan key file, not
  the body (0 body hits for `fifteen|15 binaries|verify-driven`).
- **Times written ahead of the clock**, four times (three journal lines, one evidence window), each measured and
  corrected; one caught by the stamp-ahead hook (friction record `2026-10-11T07:58:07Z-c`).

## Outcome

Acceptance criteria (plan), re-asserted against the diff:
- (arch) No live wrapper: one document, exit 0, no endpoint — **met** (`list_of_a_stopped_wrapper_is_an_empty_board`;
  `list` holds no channel client call: `src/cmd/list.rs` imports no `viola_channel`).
- (arch) A booted wrapper's item — **met** (`path1_list_shows_a_booted_wrapper`).
- (arch) Gone absent; unreadable/newer + fresh beat → `live`, no CLI field, logged `wheel`/`budget_paused`; old beat →
  absent, never `stale` — **met** (`board_read_*`, `e5_list_reads_a_live_wrapper_through_the_replay` ×2).
- (arch) State files byte-identical; no `budget.json` / lock created — **met** (`path1_…`; F2 and F3 forced red).
- (security) Widened home or instance → exit 21 on both forms, nothing printed from state, mode as found, no file added
  under a `VIOLA_NAME` — **met on Unix** (the two `list_refuses_*` cases; R1, R2, R3 forced red). Windows has no case
  (`cfg(unix)`), as planned.
- (security) Link / non-directory / non-name → no join, no item — **met** (`board_scan_*`; the link case `cfg(unix)`).
- (security) ESC / C1 / DEL / LF / TAB in a version → `\xHH`, one line — **met** (`escape_row_*`, `board_row_*`).
- (security) No type holds `endpoint` / `pinned_bin` / `cwd` / `statusline_command`; the item equals its literal —
  **met** (`board_read_item_serialises_to_the_contract_literal`).
- (layouts) BAY line, captions, rows in name order, values in their caption's column — **met**
  (`list_human_board_is_the_literal_rows`, `bay_line_*`, `board_row_*`), with WHEEL at 23 (Deviations 1).
- (design) Each state as its domain word — **met** (`board_row_*`, `bay_line_*`).
- (a11y) No ESC or non-ASCII byte on either stream, board and empty board — **met** (`list_human_board_is_the_literal_
  rows`, `list_of_a_stopped_wrapper_is_an_empty_board`; a non-ASCII printable in a recorded version would print as
  is — the row mode escapes controls only, and no case feeds one).
- (a11y) Caption row pinned by a literal in a root case — **met** (`CAPTION` in `tests/cli_list.rs`).
- (obs) Self pair only under a name; no `cli-*.ndjson` without — **met** (`list_logs_its_start_and_exit_only_under_a_
  name`; F1 forced red).
- (obs) One `state-recovered` per unreadable / newer snapshot under a name — **met** (`e5_…` ×2).
- (tests) `torn_lines` 1; `unknown_kinds` 1 and `unknown_fields` 1 at exit 0 — **met** (`chaos_torn_append`,
  `chaos_forward_compat`).
- (tests) Path 6's board readings — **met** (`path6_list_shows_the_reading_and_the_pause`).
- (tests) `run --unit`, the two filter entries and `pre-push` read `"ok":true` — **met** (firings 3 to 6).
- (tests) The preservation guard exits 0 against `995dc55a3d6b` — **met**.
- (ci) The final HEAD's run is `verdict: green` — **met**: `ci#38124062614` on `acad7afc865a`.

Gates (the `[[gate]]` block, by `run`; firing 3 the whole block on the final source, firing 6 `--only 1,2,5,13` after
the CI fix; `evidence/gates.md`):
- `cargo fmt --all --check` — green (firings 3, 4, 6).
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green (3, 4, 6).
- `bash scripts/agent-run.sh run --unit` — green (firings 2, 3). Firing 1 read **red** on one case:
  `red — not this chunk's: viola-e2e harness::cleanup::tests::remove_session_home_removes_a_boot_created_parent fails
  AlreadyExists from create_dir_all on the dangling target/e2e-home link after the 07:18:11Z reboot (mechanism measured
  two-sided in scratch; this chunk edits no crates/viola-e2e file, the preservation guard over it green) → owner: this
  wrap's P5 pin (not yet minted)`. The basis is a mechanism measurement and the chunk's empty diff over the crate, not a
  run of the parent tree in a clean worktree; P5 decides whether that suffices or a parent-tree run is owed.
- `run --unit --filter 'test(/board_scan_|…|budget_stamped_/)'` — green: 70 tests across 9 binaries.
- `run --integration --filter 'binary(cli_list) | … | binary(cli_revive)'` — green: 35 passed (firings 3, 4, 6).
- `git diff --quiet 995dc55a3d6b -- …` (the preservation guard) — green.
- `agent-run.sh cleanup` / `boot` / `status` / `cleanup --session p-board-smoke` — green.
- `bash scripts/g2-zero-panics.sh` — green · `agent-run.sh schema-check` — green.
- `bash scripts/agent-run.sh pre-push` — green: coverage 2406/0, playwright 1, breaches `[]` (firings 3, 5, 6); lines
  98.13 %, functions 98 %, regions 97.92 %.
- `gate.py hygiene` (operator) — `clean` before the pre-CI commit and before the fix commit.
- `git … && git push origin HEAD` (operator) — green, twice: `origin/build/viola-0.1.0` 995dc55a→f1ee6341, then
  f1ee6341→acad7afc.
- `ci.py conclusion --sha HEAD --wait 1800` (operator) — `ci#38123340339` **red** on `f1ee634` (this chunk's: four
  cases on unstamped homes, fixed by `acad7af`); `ci#38124062614` **green** 15/15 on `acad7af`, the final HEAD.
- Smoke: the plan's smoke entries green as P2 gates; one hand reading of `viola list` against a booted harness
  session, both forms exit 0 with the `builder` row, cleanup `processes_gone` and `endpoint_gone` true
  (`evidence/smoke.md`).
- Remove-the-guard: R1, R2, R3, F1, F2, F3 each read red and restored (`evidence/red-green.md`).
- Not run: no mutation, witness or dispatch (`inputs#I1`); no live `claude`.
- The Windows-target clippy (testing.md 2026-10-10), beside the block: green after the `cfg(unix)` fix.

Watches: none folded.

Outcome basis: the operator pass ran (pre-CI `f1ee634`, fix `acad7af`); the gate verdicts rest on firings 3 to 6 on the
final source and on `ci#38124062614` for the final HEAD (`evidence/operator.md`). Implement's report is in this
conversation; its deviations are above.

Process hygiene: every process this chunk's runs started is gone — the harness sessions `p-board-smoke` and
`p-board-hand` cleaned (`processes_gone` true), the gate entries ended. Re-measured at this wrap: the host's `viola`
processes are 7, every one of them the prototype tree's build (`/proc/<pid>/exe` under `viola-lab/prototype`), none of
this repository. The code-graph refresh this wrap fired at Setup is its own.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 995dc55a (the parent of the oldest pre-CI commit f1ee6341) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 32
### crates/viola-state/src/board.rs — new file · 691 line(s)
- 25-31 @28 «pub enum ScanRefused {»
- 33-39 @35 «pub struct Scan {»
- 41-48 @42 «fn check_if_present(path: &Path) -> Result<(), Refused> {»
  - 43-47 «match std::fs::symlink_metadata(path) {»
- 50-98 @55 «pub fn scan(home: &Path) -> Result<Result<Scan, ScanRefused>, StateError> {»
  - 57-59 «if check_if_present(home).is_err() || check_if_present(&instances_dir).is_err() {»
  - 60-64 «let mut scan = Scan {»
  - 65-69 «let entries = match std::fs::read_dir(&instances_dir) {»
  - 70-89 «for entry in entries {»
  - 90-91 «scan.instances»
  - 92-96 «for (name, dir) in &scan.instances {»
- 100-106 @103 «pub enum RowLiveness {»
- 108-115 «impl RowLiveness {»
  - 109-114 «pub const fn as_str(self) -> &'static str {»
- 117-122 @120 «pub enum SessionStatus {»
- 124-130 «impl SessionStatus {»
  - 125-129 «pub const fn as_str(self) -> &'static str {»
- 132-151 @134 «pub struct Item {»
- 153-159 @155 «pub struct SkippedTotal {»
- 161-167 «impl SkippedTotal {»
  - 162-166 «fn add(&mut self, skipped: Skipped) {»
- 169-174 @171 «pub struct Board {»
- 176-189 «fn snapshot_item(name: &ViolaName, snapshot: &InstanceSnapshot, liveness: RowLiveness) -> Item {»
  - 177-188 «Item {»
- 191-202 @192 «fn log_pass(instance_dir: &Path) -> Skipped {»
  - 193-195 «let Ok(mut lines) = read_from(instance_dir, 0) else {»
  - 196-200 «for line in lines.by_ref() {»
- 204-261 @208 «pub fn read(scan: &Scan, now: SystemTime) -> Board {»
  - 209-212 «let mut board = Board {»
  - 213-245 «for (name, dir) in &scan.instances {»
  - 246-259 «for (detail, count) in [»
- 263-691 @264 «mod tests {»
  - 278-280 «fn name(text: &str) -> ViolaName {»
  - 282-286 «fn home(tmp: &tempfile::TempDir) -> PathBuf {»
  - 288-292 «fn instance(home: &Path, text: &str) -> PathBuf {»
  - 294-313 @295 «fn own_snapshot() -> InstanceSnapshot {»
  - 315-324 «fn log(dir: &Path, lines: &[(EventKind, Value)]) {»
  - 326-337 «fn start_log(dir: &Path) {»
  - 339-346 @340 «fn booted(home: &Path, text: &str, snapshot: &InstanceSnapshot) -> PathBuf {»
  - 348-350 «fn scanned(home: &Path) -> Scan {»
  - 352-356 «fn beat_time(dir: &Path) -> SystemTime {»
  - 358-360 «fn names(scan: &Scan) -> Vec<&str> {»
  - 362-379 @363 «fn board_scan_of_an_absent_home_passes_with_no_instance_and_creates_nothing() {»
  - 381-389 @382 «fn board_scan_takes_instances_in_name_order() {»
  - 391-415 @392 «fn board_scan_counts_a_plain_file_and_a_name_that_is_no_name_as_malformed() {»
  - 417-434 @420 «fn board_scan_ignores_a_link_and_counts_it() {»
  - 436-440 @437 «fn widen(path: &Path) {»
  - 442-446 @443 «fn mode_of(path: &Path) -> u32 {»
  - 448-461 @450 «fn board_scan_refuses_a_widened_home_or_instances_dir_and_leaves_its_mode() {»
  - 463-483 @466 «fn board_scan_refuses_the_first_widened_instance_in_name_order() {»
  - 485-500 @486 «fn board_read_this_process_with_a_fresh_beat_is_live() {»
  - 502-514 @503 «fn board_read_six_seconds_past_the_beat_is_stale() {»
  - 516-525 @517 «fn board_read_omits_a_snapshot_whose_start_time_is_another_process() {»
  - 527-540 @528 «fn board_read_takes_a_pending_dialog_and_its_kind() {»
  - 542-583 @545 «fn board_read_an_unreadable_or_newer_snapshot_takes_the_replay() {»
  - 585-594 @586 «fn board_read_an_absent_snapshot_gives_no_row() {»
  - 596-623 @598 «fn board_read_sums_the_skip_counts_over_every_instance() {»
  - 625-677 @626 «fn board_read_item_serialises_to_the_contract_literal() {»
  - 679-690 @680 «fn board_read_words_are_their_serde_values() {»
### crates/viola-state/src/budget.rs — added 70 line(s) in 4 range(s)
added: 70-79 · 88-92 · 108-117 · 399-443
  - 108-111 «let read_at = match stored.read_at {»
  - 399-411 @400 «fn budget_stamped_read_returns_the_instant_write_budget_wrote() {»
  - 413-434 @414 «fn budget_stamped_read_of_an_envelope_without_a_string_read_at_is_unknown() {»
  - 436-442 @437 «fn budget_stamped_read_of_an_absent_file_is_nothing_and_creates_nothing() {»
### crates/viola-state/src/lib.rs — added 1 line(s) in 1 range(s)
added: 6
### src/cmd/list.rs — new file · 231 line(s)
- 22-27 @23 «pub(crate) struct ListArgs {»
- 29-33 @30 «struct Budget {»
- 35-61 «pub(crate) fn list(home: &Path, args: &ListArgs) -> anyhow::Result<ExitCode> {»
  - 37-45 @39 «let scan = match board::scan(&home)? {»
  - 46-48 «if let Some(name) = client::own_name() {»
  - 50-58 «if args.json {»
- 63-79 @64 «fn refuse(json: bool, refused: &ScanRefused) {»
  - 65-68 «if json {»
  - 69-78 «match refused {»
- 81-93 @83 «fn read(home: &Path, scan: &Scan, json: bool) -> (Board, Option<Budget>, DateTime<Utc>) {»
  - 89-91 «for rejection in rejections {»
- 95-107 @96 «fn budget_of(»
  - 101-106 «stamped.map(|stamped| {»
- 109-122 @111 «fn document(board: &Board, budget: Option<&Budget>) -> Value {»
  - 112-120 «let budget = match budget {»
- 124-231 @125 «mod tests {»
  - 132-136 «fn at(text: &str) -> DateTime<Utc> {»
  - 140-151 «fn stamped(five_hour: f64) -> StampedReading {»
  - 153-173 «fn board() -> Board {»
  - 175-182 @176 «fn list_document_with_a_reading_is_the_contract_envelope() {»
  - 184-196 @185 «fn list_document_without_a_reading_has_the_budget_word_unknown() {»
  - 198-208 @199 «fn list_document_paused_is_the_gates_rule_under_the_default_thresholds() {»
  - 210-214 @211 «struct Line {»
  - 216-220 «fn parse(words: &[&str]) -> Result<ListArgs, clap::error::ErrorKind> {»
  - 222-230 @223 «fn list_document_args_take_the_json_flag_and_nothing_else() {»
### src/cmd/mod.rs — added 13 line(s) in 3 range(s)
added: 5 · 52-53 · 124-133
### src/cmd/revive.rs — added 1 line(s) in 1 range(s)
added: 70
### src/human.rs — added 523 line(s) in 4 range(s)
added: 3 · 8-11 · 126-316 · 914-1240
- 126-139 @129 «pub(crate) fn escape_row(text: &str) -> String {»
  - 131-137 «for c in text.chars() {»
- 141-147 @142 «pub(crate) fn instance_state_pair(name: &str) -> (String, String) {»
  - 143-146 «(»
- 149-155 @150 «pub(crate) fn home_state_pair() -> (&'static str, &'static str) {»
  - 151-154 «(»
- 157-161 @158 «pub(crate) struct BayBudget<'a> {»
- 163-177 @165 «fn window_cell(label: &str, window: &Reading<BudgetWindow>, now: DateTime<Utc>) -> String {»
  - 166-169 «let Reading::Known(BudgetWindow {»
  - 170-172 «else {»
  - 173-174 «let expired = matches!(resets_at, Reading::Known(at)»
- 179-190 @181 «fn read_cell(read_at: &Reading<String>, now: DateTime<Utc>) -> String {»
  - 182-185 «let at = match read_at {»
  - 186-189 «match at.filter(|at| *at <= now) {»
- 192-221 @194 «pub(crate) fn bay_line(»
  - 199-216 «let (five_hour, seven_day, read, gate) = match budget {»
  - 217-220 «format!(»
- 229-236 «fn wheel_cell(item: &Item) -> String {»
  - 231-235 «if item.budget_paused == Some(true) {»
- 238-246 «fn dialog_cell(item: &Item) -> String {»
  - 239-241 «if !item.dialog_pending {»
  - 242-245 «match item.dialog_kind.as_deref() {»
- 248-260 «fn cli_cell(item: &Item) -> String {»
  - 249-259 «match &item.cli_version {»
- 262-268 «fn board_line(width: usize, cells: [&str; 6]) -> String {»
  - 264-267 «format!(»
- 270-296 @272 «pub(crate) fn board_rows(items: &[Item]) -> Vec<String> {»
  - 273-276 «let width = items»
  - 277-280 «let mut rows = vec![board_line(»
  - 281-294 «for item in items {»
- 300-315 @301 «pub(crate) fn write_board(»
  - 309-311 «if board.items.is_empty() {»
  - 915-929 @924 «fn escape_row_shows_every_control_character_as_hex_text(»
  - 931-947 @932 «fn instance_and_home_state_pairs_are_the_fixed_texts() {»
  - 949-953 «fn at(text: &str) -> DateTime<Utc> {»
  - 957-962 «fn window(used: Reading<f64>, resets_at: &str) -> Reading<BudgetWindow> {»
  - 964-976 «fn stamped(»
  - 978-984 «fn counts(unknown_kinds: u64, unknown_fields: u64, torn_lines: u64) -> SkippedTotal {»
  - 986-996 @987 «fn bay_line_without_a_reading_is_the_unknown_line() {»
  - 998-1021 @999 «fn bay_line_with_a_reading_shows_the_figures_the_age_and_the_gate() {»
  - 1037-1049 @1039 «fn bay_line_window_cell(#[case] five_hour: Reading<BudgetWindow>, #[case] cell: &str) {»
  - 1051-1063 @1058 «fn bay_line_read_cell(#[case] read_at: &str, #[case] cell: &str) {»
  - 1065-1068 @1066 «fn bay_line_read_cell_of_an_unknown_instant_is_unknown() {»
  - 1070-1083 «fn item(name: &str) -> Item {»
  - 1085-1086 «const CAPTION: &str =»
  - 1087-1088 «const DEFAULT_ROW: &str =»
  - 1090-1097 @1091 «fn board_row_caption_and_default_row_are_the_literals() {»
  - 1099-1124 @1100 «fn board_row_every_value_starts_in_its_captions_column() {»
  - 1126-1141 @1132 «fn board_row_wheel_cell(»
  - 1143-1159 @1150 «fn board_row_dialog_cell(»
  - 1171-1180 «fn board_row_cli_cell(»
  - 1182-1193 @1183 «fn board_row_a_32_character_name_prints_whole_and_moves_the_columns_right() {»
  - 1195-1205 @1196 «fn board_row_rows_keep_the_boards_name_order_whatever_their_state() {»
  - 1207-1219 @1208 «fn board_row_the_empty_board_is_three_lines_in_one_write() {»
  - 1221-1240 @1222 «fn board_row_a_board_is_one_write_of_printable_ascii_and_lf() {»
### src/main.rs — added 2 line(s) in 1 range(s)
added: 338-339
### tests/chaos_forward_compat.rs — new file · 67 line(s)
- 19-29 @20 «fn append(log: &Path, line: &Value) {»
  - 23-28 «OpenOptions::new()»
- 31-36 «fn skipped(home: &Path) -> Value {»
- 38-67 @41 «fn chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads(stamped_home: StampedHome) {»
  - 47-50 «assert_eq!(»
  - 53-57 «append(»
  - 58-62 «append(»
  - 63-66 «assert_eq!(»
### tests/chaos_torn_append.rs — added 15 line(s) in 4 range(s)
added: 13 · 15-16 · 47-53 · 74-78
### tests/cli_budget.rs — added 46 line(s) in 1 range(s)
added: 386-431
- 386-391 @387 «fn board(home: &Path) -> Value {»
- 393-430 @397 «fn path6_list_shows_the_reading_and_the_pause(stamped_home: StampedHome) {»
  - 413-418 «assert!(»
  - 419-422 «assert_eq!(»
### tests/cli_list.rs — new file · 358 line(s)
- 21-22 «const ROW: &str =»
- 28-31 @29 «fn boot(stamped: StampedHome) -> Wrapper {»
- 33-36 «fn list(home: &Path, json: bool, from: Option<&str>) -> Ran {»
- 38-44 @39 «fn document(ran: &Ran) -> Value {»
- 46-56 @47 «fn assert_plain(ran: &Ran) {»
  - 48-55 «for stream in [&ran.stdout, &ran.stderr] {»
- 58-61 «fn state_bytes(instance_dir: &Path) -> (Vec<u8>, Vec<u8>) {»
- 63-65 «fn diagnostics(home: &Path, file: &str) -> Vec<Value> {»
- 67-75 «fn cli_logs(home: &Path) -> Vec<String> {»
  - 68-70 «let Ok(entries) = std::fs::read_dir(home.join("diagnostics")) else {»
  - 71-74 «entries»
- 77-103 @80 «fn path1_list_shows_a_booted_wrapper(stamped_home: StampedHome) {»
  - 86-97 «assert_eq!(»
  - 99-101 «for file in ["budget.json", "budget.json.lock"] {»
- 105-119 @106 «fn list_human_board_is_the_literal_rows(stamped_home: StampedHome) {»
- 121-137 @122 «fn list_after_a_pause_reads_the_human_wheel(stamped_home: StampedHome) {»
  - 130-135 «assert_eq!(»
- 139-160 @142 «fn list_of_a_stopped_wrapper_is_an_empty_board(stamped_home: StampedHome) {»
  - 154-157 «assert_eq!(»
- 162-176 @163 «fn list_of_a_home_that_does_not_exist_creates_nothing() {»
  - 169-172 «assert_eq!(»
- 178-206 @179 «fn list_logs_its_start_and_exit_only_under_a_name(stamped_home: StampedHome) {»
  - 188-191 «let starts: Vec<&Value> = lines»
  - 192-195 «let exits: Vec<&Value> = lines»
  - 200-204 «for line in [starts[0], exits[0]] {»
- 208-252 @214 «fn e5_list_reads_a_live_wrapper_through_the_replay(»
  - 223-229 «let altered = match v {»
  - 234-240 «assert_eq!(»
  - 241-244 «let recovered: Vec<Value> = diagnostics(&home, "cli-overseer.ndjson")»
- 254-277 @256 «fn tree(home: &Path) -> std::collections::BTreeMap<std::path::PathBuf, String> {»
  - 260-275 «while let Some(dir) = dirs.pop() {»
- 279-283 @280 «fn set_mode(path: &Path, mode: u32) {»
- 285-289 @286 «fn mode_of(path: &Path) -> u32 {»
- 291-303 @293 «fn stopped_home() -> StampedHome {»
  - 294-299 «let wrapper = Wrapper::boot(»
- 305-331 @309 «fn list_refuses_a_home_another_user_could_write() {»
  - 318-322 «assert_eq!(»
- 333-358 @335 «fn list_refuses_an_instance_another_user_could_write() {»
  - 345-349 «assert_eq!(»
