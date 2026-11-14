# Implement outcome — 2026-11-14-budget-gate-removed

**Outcome: green + smoke.** Next → the OPERATOR PASS (entries 32 hygiene, 33 push, 34 the CI read), then `/andromeda-wrap-session`.

## Gates
- Firing 1 (whole block): entries 34 · green 23 · red 7 (3, 16, 18, 19, 20, 21, 31) · recorded 1 · not-run 3.
- Firing 2 (`--only 3,16,18,19,20,21,31`): green 7 · red 0.
- Firing 3 (whole block): `entries 34 · green 30 · red 0 · recorded 1 · timeout 0 · not-run 3` — the three `operator` entries (32, 33, 34) not fired, as the letter says.
- No deferral: Rust, both lockfiles, schemas and fixtures changed; the walk-class line read `uncommitted 331`, every gate ran.
- Entry 11 (the identifier probe) and entry 12 (the config/log-key probe): green, exit 1, no output.
- Control `release-budget` (step 16, `{run_dir}/control-1.toml`, case `release_carrying_a_budget_key_is_invalid_params_and_moves_no_wheel`): `control release-budget · entry 16 · holds — in place green, removed red · exit 0 ✗ (exit 100) · 19.64s · 4.31s`; `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (evidence/control-1.txt).
- No `gate.py guard` entry in the block.

## Smoke (P3, by hand)
`agent-run.sh cleanup` → `boot --session p3-budget-smoke --instance builder` (ok:true) → `status` (`state ready`; the list document `{items, skipped}`, the item with no budget field) → the instance's `events.ndjson` opens `wheel{cause:"start"}` then the hook's `session-start` → no budget file in the home → `cleanup` (`processes_gone:true`, `endpoint_gone:true`). smoke ✓.

## Matrix
`v2-04` planned → implemented; ref: `src/run/wheel.rs::release_carrying_a_budget_key_is_invalid_params_and_moves_no_wheel` (control holds) · `src/cmd/release.rs::release_args_refuse_the_budget_flag` · `src/cmd/send.rs::exit_codes_of_the_refusals_leave_eleven_unused` · `src/run/send.rs::send_refusal_order` · `src/cmd/list.rs::list_document_is_the_contract_envelope` · `tests/contract_ledger_probes.rs` (ROW_IDS 23) · plan entries 11 and 12.

## Files changed
102 changed (scope: `clean — changed 102 · listed 94 · recorded 8 (companion 5 · mechanical 0 · in-intent 3 · widening 0)`). Deleted: `src/run/budget.rs`, `src/run/send/budget.rs`, `crates/viola-state/src/budget.rs`, `crates/viola-agent-claude/proptest-regressions/statusline.txt`, `crates/viola-e2e/tests/path6_budget_harness.rs`, `tests/cli_budget.rs`, `fixtures/claude/{2.1.287,2.1.302}/StatusLine.rate-limits-1.json`. New: `fixtures/viola-0.1.0/{events.ndjson, retired-words.json, snapshot.json}`. `fuzz/Cargo.lock` lost `chrono` and its 223 lines of dependents (refreshed by `cargo metadata --offline`, read under `--locked` by entry 9).

## Deviations from the plan, with justifications
1. **The retired words live in fixtures (inputs#I6).** Entry 11's probe (exit 1, no output over `src crates tests …`) and the plan's own cases (steps 2, 3, 6) that must spell `budget_paused`, `budget_override_until`, `budget-gate`, `statusline-rate-limits` and `budget-paused` were jointly unsatisfiable. Asked at P1; the overseer's delegate answer (the operator away): the 0.1.0 shapes and every retired word go to committed files under `fixtures/viola-0.1.0/`, read by the cases (`include_str!` in unit tests, `workspace_path` in root tests), nothing dropped. Scope-record lines, in-intent, word quoted (`fixtures/viola-0.1.0/*`). The fixture-hygiene walk gained `viola_format_fixtures_pass_hygiene` over that directory.
2. **`hook statusline`'s payload check kept.** With `statusline::reading` gone, the arm still refuses a payload that is not one JSON object (`malformed-json`, its `parse-rejected` line), now through `serde_json::from_slice` in `src/cmd/hook/statusline.rs` — the rejection the step-7 «stays as it is» covers.
3. **`statusline_variants` keeps its first-without-`rate_limits` choice** (`crates/viola-agent-claude/src/ledger.rs`): only the rate-limits variant left; `--record` writes `StatusLine.default-1.json` alone.
4. **`tests/contract_diag_schema.rs`'s null-corr case re-pointed, not dropped**: `non_dialog_hook_decision` now carries `duration_ms` in place of `budget_written` — the case's subject (a statusline decision with no corr) stands.
5. **`write_board` and `list::read` lost their `now`** — the BAY line was its only reader; `src/human/board/test_support.rs` lost `at`/`NOW` with it.
6. **Companions the research did not list** (scope-record, `self`): `crates/viola-agent-claude/src/ledger/test_cases_checks.rs` (the verdict table, 23 rows) · `tests/cli_verify/runs.rs` (the summaries' counts) · `tests/security_negatives_strict_modes.rs` (its statusline payload helper) · `tests/hook_events.rs` (start-record counts) · `tests/cross_list_rows.rs` (the BAY literal).
7. **The release guard's case holds the wheel with a recorded `pause`**, not a human key: `human_input`'s record is queued unwaited, and firing 1 read the snapshot before it landed (a race in the new case, fixed at firing 2).

## Surfaced
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola run` / fake agent of `p-budget-smoke` (entries 25–30) | a gate entry | terminated (entry 30: `processes_gone:true`, `endpoint_gone:true`) |
| `viola run` / fake agent of `p3-budget-smoke` | this run (P3) | terminated (`processes_gone:true`, `endpoint_gone:true`) |
| control clone `~/.cache/andromeda-control/3117e718` | this run (control) | removed (`clone removed`) |
| 7 `viola` processes the census lists (`~/.local/viola-product/bin/viola wait`, `~/.viola/bin/0.1.0-…/viola mcp`, the viola-lab prototype's `run`/`wait`) | NOT DERIVED — none is this repository's `target/` build | left running: not this run's — their owners' |

Census: `census: 7 running` (`--comm viola --comm viola-fake-agent --args projects/viola/target/debug/viola --comm cargo-nextest`), none matching this repository's build.

## Decisions & corrections
- The probe clash ruling (deviation 1): «the old 0.1.0 shapes go to committed fixture files outside the probed paths (fixtures/viola-0.1.0/), every case that must spell an old word reads it from there — the exit-code case's «budget-paused» word included, nothing dropped» — delegate the Viola overseer, 2026-11-14, the operator away (inputs#I6, relay-1.md). PROVISIONAL until the operator's own word.
- Q1 ratified by the founder's own word, relayed by overseer1 (2026-11-14T15:53:20Z): «Не поднимать, исключение в правило» — every `v` stays 1 (inputs#I5); the wrap writes the exception into `architecture · decision/protocol-version-bump-rule`.
- Guard refusals: bash-guard refused a `cat` heredoc to a file (re-issued through the Write tool) and a `cd` out of the root (re-issued with an absolute path) — one re-issue each.
- Hook false positive: stamp-ahead read a clock-time test literal (the budget-lifted line's end) in two scratch scripts as a stamp ahead of the clock; it was a test literal being removed from `src/human/verbs.rs` / quoted in a friction record, not a timestamp. Left as written.
- A journal stamp written by estimate (16:33Z) was corrected to the clock's 16:22Z (`date -u` read 16:22:36Z) — the 2026-11-14 learning «a timestamp is read from `date -u`», recurred and corrected; later stamps read from `date -u`.
- The operator's directive for this run: no live `claude` start, no write under `~/.viola/`, the pinned product binary untouched — held (every leg on the fake agent; the census shows the pinned copy only as a running process, not touched).
