# Codebase Research — 2026-10-31-exit-cause-code-catalogue

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 9
- **Harness rules consulted:** `.claude/rules/testing.md` — its Session Additions, auto-loaded on the tests this chunk touches, applied (the stamped-home 10 s mutants kill, `Running`-bounded witnesses, literal oracles copied from a first run); `.claude/rules/verification-harness.md` not read: no live leg in this chunk, every leg runs under the fake agent.
- **Platform issues consulted:** none — no runner-only bullet: Setup 5a's one read is green, and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's directive to this phase (forced sample, classifiers at high with three draws, the evidence kept, one re-spawn, founder forks at P4, the standing items, the freight and the WATCH tally, the store reports, the two local commits).

## Files inspected
- `src/cmd/client.rs` (full) — `refuse_untrusted` (`:24`), `Unreachable` (`:98`), `answer_of` (`:162`), `refused_document` (`:198`), `unreachable` (`:216`): the exit-21 seam. One document for every cause (`:218`); the cause already reaches the log through `detail()` and `during()`.
- `src/cmd/run.rs` (`:160–:460`, `:500–:580`, `:680–:770`) — `start` takes no `json` (`:256`); `refused` logs (`:409`); `collision_check` carries a `json` flag, and `start` passes `false` (`:278`); the four refuse helpers print their pair whatever the caller asked (`:735–:770`); `resolve_program`'s other error is `refused("internal-error")` with nothing printed (`:270`).
- `src/cmd/revive.rs` (`:1–:420`) — `Refusal` and `detail()`, the strict-modes check ahead of the log (`:121`), `checked_exe`'s refusal under `--json` (`:127`), `preflight`, `start`'s call (`:138`).
- `src/main.rs` (`:45–:160`) — `role_of` reads argv before clap; `prints_internal_error` is `Cli`-only (`:146`); a failure of `Other` (`run`, a `revive` start) prints nothing, by obs-plan §7.
- `src/cmd/mod.rs` (`:100–:250`) — dispatch and the verb list: no `ui` verb.
- `src/cmd/send.rs` (`:90–:200`), `src/cmd/wait.rs` (`:30–:110`), `src/cmd/list.rs` (`:80–:130`), `src/cmd/plugin.rs` (`:35–:75`), `src/cmd/verify.rs` (`:60–:100`, `:150–:205`, `:625–:636`), `src/cmd/link.rs` (`:95–:112`), `src/cmd/pause.rs` (`:60–:75`).
- `src/human/hints.rs` (full) — `send_hint` keys the not-running and server-verify hints; `no_hint_names_release` lists every cause; the unwrapped-name hint is absent.
- `crates/viola-mcp/src/result.rs` (`:1–:150`), `crates/viola-mcp/src/tools.rs` (`:300–:410`) — `unreachable()` takes no cause (`result.rs:29`); `frame` returns it from three places (`tools.rs:338`, `:341`, `:348`); `of_reply` folds Connect, Closed and Io (`result.rs:52`); `of_board` reads `error` (`:89`).
- `crates/viola-channel/src/client.rs` (`:185–:215`) — `unreachable_outcome` writes the `outcome` word `instance-unreachable` on a vanished `wait`: the channel line's own word, kept.
- `tests/contract_error_bodies.rs` (`:1–:135`), `tests/cli_error_bodies.rs` (`:1–:80`), `tests/chaos_wait_vanish.rs` (full), `tests/support/start.rs` (`:1–:75`), `tests/path1_run_start.rs` (`:1–:60`), `tests/cli_instance_state.rs` (`:180–:200`, `:840–:860`), `tests/cli_wait_last.rs` (`:440–:480`).
- `.andromeda/test-plan-detail/e2e-test-strategy.md` (`:272–:300`), `schemas/diag-line.v1.json` (`:76–:80`), `.andromeda/playbook.md` (full).

## Cause list (closed against the code at HEAD)
Each row: the exit · the cause · its site · the code today on the log · whether `--json` tells it · its hint.

| exit | cause | site | log `detail` today | `--json` today | human |
|---|---|---|---|---|---|
| 21 | strict modes, frame verbs | `client.rs:31` | no line (founder's) | shared | instance pair |
| 21 | strict modes, `list` | `list.rs:91` | no line (founder's) | shared | home or instance pair |
| 21 | strict modes, `plugin install` | `plugin.rs:52` | no line (founder's) | shared | home pair |
| 21 | not running | `live_endpoint` none, `answer_of` Connect | `instance-dead` · `connect` | shared | not-running hint |
| 21 | an unwrapped name | not told apart | `instance-dead` · `connect` | shared | not-running hint (owed) |
| 21 | endpoint lost mid-call | `answer_of` Closed, Io | `instance-dead` · `call` | shared | not-running hint |
| 21 | server verification (off macOS) | `connect` | `server-verify-failed` · `connect` | shared | server-verify hint |
| 1 | already live | `collision_check` | `already-live` | `already-live` | viola list |
| 1 | stale heartbeat | `collision_check` | `already-live` | `already-live` | stale hint |
| 1 | squatted name | `start` `:297` | `squatted-name` | none | squat hint |
| 1 | pinned-hash mismatch | `start` `:283` | `pinned-hash-mismatch` | none | pin hint |
| 1 | script child (Windows) | `start` `:267` | `batch-script-child` | none | script hint |
| 1 | a program not on `PATH` | `start` `:270` | `internal-error` | none | nothing (run's terminal) |
| 1 | strict modes, home tree | `checked_exe` | no line (founder's) | `strict-modes-failed` under revive | home or instance pair |
| 1 | `revive` `no-session`, `session-live`, `cwd-missing` | `preflight` | as named | as named | own pairs |
| 1 | `verify` strict modes, home tree | `verify.rs:67` | no line (founder's) | no `--json` | home pair |
| 1 | `verify` `update_stamps` refused | `verify.rs:291` | `process-exit`, no detail | no `--json` | home pair |
| 1 | `verify` CLI not found · script CLI · version unreadable · unclean recording | `verify.rs:157`, `:164`, `:199`, `:630` | `process-exit`, no detail | no `--json` | own pairs |
| 1 | `verify` interrupted | `verify.rs:81` | `interrupted` | no `--json` | own pair |
| 1 | internal error, `cli` catch site | `main.rs:72` | `internal-error` | none: stderr line under `--json` | fixed line, no hint |
| 1 | internal error, a `revive` start | `main.rs:146` | `internal-error` | none | nothing |
| 1 | `mcp`: refused home, a line before `initialize` | `src/cmd/mcp.rs` | no line · `parse-rejected` | no `--json` | streams empty |

Not in the code: a socket-directory refusal (`run.rs:531` is `?`), a `viola ui` refusal. `verify`'s failing row is an outcome.

## Graph impact (from the code-graph query)
`.andromeda/runs/2026-10-31T15-42-40-phase/tree-query-2026-10-31-exit-cause-code-catalogue.json` holds nine queries, `db_state` regenerated then fresh; lines cited as `line + 1`.
- **`unreachable`** (`client.rs`) — its callers `deliver` @ `src/cmd/answer.rs:118`, `last` @ `src/cmd/last.rs:37`, `link_call` @ `src/cmd/link.rs:99`, `wheel_call` @ `src/cmd/pause.rs:63`, `Out::unreachable` @ `src/cmd/send.rs:169`, `wait` @ `src/cmd/wait.rs:50`: its signature keeps; `cause` carries the new word.
- **`result::unreachable`** (`result.rs`) — `of_reply` @ `crates/viola-mcp/src/result.rs:53`, `of_board` @ `:90`, `frame` @ `crates/viola-mcp/src/tools.rs:338`, `:341`, `:348`, a test @ `result.rs:249`: its signature changes, to take the cause; every caller is on the list.
- **`refused_document`** — `refuse_untrusted` @ `src/cmd/client.rs:35`, `Refusal::write` @ `src/cmd/revive.rs:107`, `collision_check` @ `src/cmd/run.rs:440`, `install` @ `src/cmd/plugin.rs:54`; `refused_document` @ `src/cmd/list.rs:86`, `:98` is `list`'s own function of that name. Its shape stays for exit 1; exit 21 gets a second writer.
- **`start`** (`run.rs`) — `run` @ `src/cmd/run.rs:183`, `revive` @ `src/cmd/revive.rs:138`, a test @ `src/cmd/run.rs:1656`: `Launch` gains `json`, so the signature keeps.
- **`collision_check`** — `preflight` @ `src/cmd/revive.rs:200`, `start` @ `src/cmd/run.rs:278`, a test @ `:1214`.
- **`live_endpoint`** — `frame` @ `crates/viola-mcp/src/tools.rs:340`; `deliver` @ `src/cmd/answer.rs:122`, `last` @ `src/cmd/last.rs:41`, `link_call` @ `src/cmd/link.rs:103`, `wheel_call` @ `src/cmd/pause.rs:67`, `deliver` @ `src/cmd/send.rs:221`, `wait` @ `src/cmd/wait.rs:54`: its signature keeps; each `else` arm reads the new cause.
- **`of_reply`** — `frame` @ `crates/viola-mcp/src/tools.rs:345`, `:351` and three tests in `result.rs`.
- **`internal_error`** (`human`) — `main` @ `src/main.rs:73`, the one printer.
- **`Dead`** (`Unreachable`) — six product sites (`command grep -n 'Dead("connect")' -r src`), each becoming `Unreachable::without_endpoint(&instance_dir)` where it stands on no endpoint.

## Patterns detected
- **One cause word on the log, read from a small closed type** (`src/cmd/client.rs:98`): `Unreachable::detail` and `during` are the catalogue's seam; a new variant adds the row.
- **`--json` beside the pair, chosen at the refusal** (`src/cmd/revive.rs:105`, `src/cmd/run.rs:440`): the document goes in place of the pair, the log line unchanged.
- **Hints keyed by a cause word** (`src/human/hints.rs:26`): adding a hint is one arm, one rstest case and one row of `no_hint_names_release`.
- **Literal document constants in the test binaries** (`UNREACHABLE` in `tests/cli_list.rs:30`, `tests/cli_links.rs:286`, `tests/cli_machine_contract.rs:30`, …): a moved document moves each constant, and a constant per cause replaces the shared one.

## Conventions to follow
- **Closed lists every surface reads live in `viola-core`** (`crates/viola-core/src/lib.rs:160` `RefusalReason`): `viola-mcp` does not depend on the root bin.
- **Scoped captures through `with_scoped_default`** (`src/cmd/client.rs:483`).
- **Stamped homes on the `ci` profile's 20 s kill line** (`.config/nextest.toml:24`, the `binary(…)` list), its count by that rule.
- **Root `cross_` binaries report in the E2E layer** (`^(path[0-9]*|tui|mcp|cross|chaos|contract)_`), with `[[test]]` `required-features = ["fake-agent"]` as `path1_run_start` and `cli_error_bodies` (`Cargo.toml:21`, `:36`).

## New files to create
- `tests/cross_exit_causes.rs` — the exit-cause table: one rstest case per cause in each mode it has, and the shared-hint and no-`release` checks over its literal table.

## Files to modify
- `Cargo.toml` — the `[[test]]` table of `cross_exit_causes`.
- `.config/nextest.toml` — `binary(cross_exit_causes)` on the verify-driven kill line.
- `schemas/diag-line.v1.json` — `process-exit` `detail` gains the new words.
- `crates/viola-core/src/lib.rs` — the closed `UnreachableCause` and its literal table.
- `crates/viola-state/src/liveness.rs` — the no-endpoint reading.
- `src/main.rs` — the catch site's `--json` form.
- `src/cmd/mod.rs` — the catch site's document writer, `client` being private to `cmd`.
- `tests/support/start.rs` — `tests/cli_revive.rs`'s start setups, shared with the table.
- `src/cmd/client.rs` — `Unreachable`, the document, `refuse_untrusted`.
- `src/cmd/run.rs` — `Launch.json`, the start refusals' documents, the stale code.
- `src/cmd/revive.rs` — `json` handed to the start, the internal-error document.
- `src/cmd/verify.rs` — the refusals' log codes.
- `src/cmd/send.rs` — the no-endpoint arm.
- `src/cmd/wait.rs` — the no-endpoint arm.
- `src/cmd/last.rs` — the no-endpoint arm.
- `src/cmd/answer.rs` — the no-endpoint arm.
- `src/cmd/pause.rs` — the no-endpoint arm.
- `src/cmd/link.rs` — the no-endpoint arm.
- `src/cmd/list.rs` — the refused document's cause.
- `src/cmd/plugin.rs` — the refused document's cause.
- `src/human/hints.rs` — the unwrapped-name and endpoint-lost hints.
- `crates/viola-mcp/src/result.rs` — `unreachable` takes the cause; `of_reply` and `of_board` read it.
- `crates/viola-mcp/src/tools.rs` — `frame`'s three unreachable arms.
- `crates/viola-mcp/tests/handlers.rs` — the moved bodies, a case per cause.
- `crates/viola-e2e/tests/mcp_tools.rs` — the moved bodies.
- `tests/chaos_wait_vanish.rs` — `endpoint-lost`.
- `tests/cli_send.rs` — the moved document.
- `tests/cli_list.rs` — the moved documents, the unwrapped hint.
- `tests/cli_machine_contract.rs` — the moved documents.
- `tests/cli_wait_last.rs` — the moved documents and the catch site's document.
- `tests/cli_controls_not_disableable.rs` — the moved document.
- `tests/cli_plugin_install.rs` — the moved document.
- `tests/cli_links.rs` — the moved documents.
- `tests/cli_error_bodies.rs` — the moved rows.
- `tests/cli_instance_state.rs` — `stale-heartbeat`.
- `tests/cli_revive.rs` — the start refusals' documents.
- `tests/cli_verify.rs` — the refusals' log codes.
- `tests/security_negatives_server_identity.rs` — the moved document.
- `tests/security_negatives_strict_modes.rs` — the moved document.
- `tests/contract_error_bodies.rs` — the moved and the new rows.

## Sweeps
- `instance-unreachable` over `tests crates src`: 22 files (`command grep -rl`): 19 changed; 3 no-change for the word: `crates/viola-channel/src/client.rs` (the `outcome` word on a vanished `wait`), `src/human/mirror.rs` (the human column and its unit), `src/cmd/send.rs:174` (the mirror's column, the file changing at its no-endpoint arm). Inside the changed files the word stays in the human assertions (`tests/cli_links.rs:437`, `tests/security_negatives_server_identity.rs:225`).
- `instance-dead` over `tests crates src schemas`: 6 hits: 4 changed where `during` is `call` or the name unwrapped (`tests/chaos_wait_vanish.rs:108`, `src/cmd/client.rs:466`, and the two fresh-home tests `tests/cli_wait_last.rs:436` and `tests/cli_send.rs:1085`, corrected at P5 by reading their setups); 2 stay (`src/cmd/client.rs:107`, `:462`, the schema's word).
- `is not running` over `tests crates src`, added at P5: `tests/cli_send.rs:1068` and `tests/cli_wait_last.rs:418` stand on a fresh home and move to the unwrapped hint; `tests/cli_links.rs:442` is a stopped instance and stays; `src/human/mirror.rs:157` and `src/human/hints.rs:82` are the hint's own units and stay.
- `already-live` over the tests: 13 hits: 2 changed (`tests/cli_instance_state.rs:196`, `:855`, the stale refusals); 11 stay (a live holder, a fresh replayed beat).
- `internal error` over the tests: 11 hits: 0 changed; `tests/cli_wait_last.rs` `cli_verbs_print_internal_error_once` is a human-mode case, and the `--json` form is a new case beside it.
- `human::refuse(` under `src`: 18 sites at HEAD (`command grep -rcE`), moved by none: the start refusals' document goes beside each helper's pair.

## Mechanisms re-derived at HEAD
- **A document for a start refusal after a passed preflight comes before any child.** `start` returns `Refused` from `:268`, `:270`, `:279`, `:283`, `:297`, each ahead of `spawn_child` at `:384`; `pump_child` runs only on `Launched`. So printing the document at those refusals is printing before the terminal holds a child, the TUI boundary kept.
- **A catch-site document under `--json` cannot print after a child runs, on `revive`'s arm.** `revive`'s `pump_child` error also reaches `main` as `Failed` (`revive.rs:142`), so `main` must not print for `Role::Other`; the document for a start's anyhow error is written in `revive` on `start`'s and `preflight`'s `Err`, ahead of the pump (`revive.rs:137`–`:138`), and `main` prints it for `Role::Cli` alone, which never holds a child.
- **An absent instance directory is reachable under the strict-modes check.** `check_instance` passes an absent directory ("a home with no instance directory passes", test-plan §4), `client::start` creates only `diagnostics/`, so `live_endpoint` returns none with the directory still absent: X = no directory yields `unwrapped-name` for the name `stranger` of `tests/cli_list.rs:378`, which has no instance. A stopped instance holds its directory (`tests/cli_links.rs` `link_verbs_against_a_stopped_instance_exit_21`), so it stays `instance-dead`.
- **`during` already splits the mid-call cause.** `answer_of` gives `Closed` and `Io` `call` (`client.rs:171`), on the same `ChannelError` set MCP's `of_reply` folds at `result.rs:52`, so `Connect` and the other two can be told apart there with no change in `viola-channel`.

## Open questions
- Where a cause's code is carried on exit 21, and whether it reaches MCP → blocks: plan-decision (P4 fork, the founder's).
- How an unwrapped name is told from a stopped instance → blocks: plan-decision (P4 fork; an agents read in a frame verb is a boundary widening).
- The endpoint-lost hint's wording → blocks: plan-decision (P4 fork; design-system holds none).
