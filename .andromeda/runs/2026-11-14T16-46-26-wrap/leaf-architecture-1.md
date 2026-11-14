@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=7 class=stale
leaf-text: `src/run/` — PTY pump, wheel, budget governor, readiness-gate wiring
source: dec-utofgtfh «The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.»
replace-with: `src/run/` — PTY pump, wheel, readiness-gate wiring
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=22 class=stale
leaf-text: the seam's `malformed` error. Percentages are the `Percent` newtype (0–100, JSON number); an external budget reading's `used_percentage` is a `Reading<f64>` instead, kept from 0 to 100 and otherwise `"unknown"`.
source: dec-vvbdbaq4 «Bounded domain values are nutype newtypes: ViolaName (charset and length cap), DialogHold (whole seconds 60 to 3600).»
replace-with: the seam's `malformed` error.
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=26 class=stale
leaf-text: (Claude's own `last_assistant_message`, `resets_at`, `used_percentage` kept)
source: dec-zblo3ml4 «JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.»
replace-with: (Claude's own `last_assistant_message` kept)
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=33 class=stale
leaf-text: · 11 budget-paused ·
source: ctr-tf6njmtt «CLI exit codes · 11 — Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.»
replace-with: · 11 unassigned (the retired `budget-paused` refusal's code, never reused or renumbered) ·
@leaf_edit n=5 leaf=.claude/docs/conventions.md line=39 class=stale
leaf-text: (`--json`, `--file`, `--budget`, global `--home`)
source: ctr-zfrhupy4 «CLI verbs · release — `viola release <target>`; a `--budget` flag is a usage error (exit 2).»
replace-with: (`--json`, `--file`, global `--home`)
@leaf_edit n=6 leaf=.claude/docs/conventions.md line=40 class=stale
leaf-text: `viola release <target> [--budget]`
source: ctr-zfrhupy4 «CLI verbs · release — `viola release <target>`; a `--budget` flag is a usage error (exit 2).»
replace-with: `viola release <target>` (a `--budget` flag is a usage error, exit 2)
@leaf_edit n=7 leaf=.claude/docs/conventions.md line=45 class=retired
leaf-text: keys: the budget thresholds `five_hour` / `seven_day` (whole numbers into `Percent`, defaults 90 / 85, read once at the wrapper's start and by `viola list` at each run; a bad value takes that key's default with one `parse-rejected` line per key, and no value switches the gate off), `diagnostics_level`
source: ctr-t43swc3q «contract/config-json/five-hour · config.json · five_hour — The five-hour budget threshold, a JSON whole number from 0 to 100, default 90»
replace-with: keys: `diagnostics_level`
@leaf_edit n=8 leaf=.claude/docs/conventions.md line=54 class=stale
leaf-text: the root bin holds the subcommand dispatch, the run pump, the wheel and the budget governor, all of which
source: dec-utofgtfh «The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.»
replace-with: the root bin holds the subcommand dispatch, the run pump and the wheel, all of which
@leaf_edit n=9 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=retired
leaf-text: parses the statusline `rate_limits` and, in the pure `agents` module,
source: abs-ywmx3ndp «absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: parses, in the pure `agents` module,
@leaf_edit n=10 leaf=.claude/docs/services/viola-agent-claude.md line=22 class=retired
leaf-text: no file, clock or environment read): `reading` (the payload's `rate_limits` into a `BudgetReading`; `resets_at` as epoch seconds or RFC 3339), `user_command`
source: abs-ywmx3ndp «absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: no file, clock or environment read): `user_command`
@leaf_edit n=11 leaf=.claude/docs/services/viola-agent-claude.md line=22 class=stale
leaf-text: Its three CLI shapes are the ledger rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` (since chunk 2026-10-13-owed-ledger-rows), relied on every build (`LedgerRow::relied()` holds all twenty-four rows; architecture, Capability ledger).
source: dec-ucvfrmxd «A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 24th row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: Its two CLI shapes are the ledger rows `statusline-override` · `statusline-shell` (since chunk 2026-10-13-owed-ledger-rows), relied on every build (`LedgerRow::relied()` holds all twenty-three rows; architecture, Capability ledger).
@leaf_edit n=12 leaf=.claude/docs/services/viola-agent-claude.md line=23 class=retired
leaf-text: statusline `rate_limits.*`, the settings-override mechanism and the shell (rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` since chunk
source: abs-ywmx3ndp «absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: the settings-override mechanism and the shell (rows `statusline-override` · `statusline-shell` since chunk
@leaf_edit n=13 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: `LedgerRow`, twenty-four closed rows
source: dec-ucvfrmxd «A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 24th row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: `LedgerRow`, twenty-three closed rows
@leaf_edit n=14 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: `statusline-shell` · `statusline-rate-limits` · `resume-session`
source: abs-ywmx3ndp «absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: `statusline-shell` · `resume-session`
@leaf_edit n=15 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: (every row of `ALL`, in its order, all twenty-four)
source: dec-ucvfrmxd «A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 24th row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: (every row of `ALL`, in its order, all twenty-three)
@leaf_edit n=16 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: all twenty-four on every build;
source: dec-yhgeek4o «readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.»
replace-with: all twenty-three on every build, a stamp holding a retired ledger row verifying on the relied rows alone;
