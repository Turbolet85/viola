# Codebase Research — 2026-10-17-cli-machine-contract

## Scope
- **Depth:** deep (mature codebase; the CLI's every verb is built) · **Reads:** 19 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 17 Session Additions; `.claude/rules/testing.md` — read in full, 47 Session Additions (both loaded whole into this window at P3). Applied: the stamped-home fixture chain and Run E under the fake agent (verification-harness §Fake agent, §boot), `binary(<stem>)` selection and the E2E regex complement (testing §Running tests, 2026-10-17), a new stamped-home root case sized against the `mutants` profile's 10 s kill (testing 2026-09-24 extended 2026-10-06), a timing window forced open with a test-only hold rather than sampled (testing 2026-09-27), a timing red never fixed by a raised bound (testing 2026-09-28), a stalled-start red on the tmpfs backing (verification-harness 2026-09-29 extended 2026-10-07).
- **Platform issues consulted:** `actions/runner-images ubuntu-24.04 20261012 PTY openpty flaky concurrent processes issue` (github.com) → the fetched results name the image release `20261012.334.1` (Ubuntu 24.04.5, kernel 6.17.0-1022-azure) on the releases page, the `ubuntu-latest` → 26.04 move from 2026-10-19 (actions/runner-images#14748) and an HTTPS instability issue (#11886); none names a PTY, `openpty` or concurrent-process failure on that image. The red's signature is this project's own panic text, so no tracker can hold it.
- **External inputs:** `inputs#I1` — the operator's word, in this session at 2026-10-17T02:33Z, folding the last wrap's red CI `ci#38994749835` into this chunk as a runner-only bullet with its own acceptance and witness.

## Files inspected
- `src/cmd/mod.rs` (full) — `Cli.home: Option<PathBuf>` global `--home` (mod.rs:31-33); `dispatch` resolves every verb but `hook` through `resolve_home` (mod.rs:174-183): `--home` as given, else `std::env::home_dir()` + `default_home` — no `VIOLA_DIR` step, no canonicalisation. `hook` takes its home from `VIOLA_DIR` (mod.rs:77).
- `src/cmd/hook.rs` (70-160) — `instance_of(VIOLA_NAME, VIOLA_DIR)` (hook.rs:80-97): a valid `ViolaName`, an absolute dir whose last two components are `instances/<name>`, the home its grandparent. The one landed `VIOLA_DIR` shape check.
- `src/cmd/client.rs` (full) — the shared `--json` writers: `document` (client.rs:116-118, one write), `fault` → 20 with `{"v":1,"error":"wrapper-fault",…}` (client.rs:121-131), `unreachable` → 21 with `{"v":1,"error":"instance-unreachable","detail":null}` (client.rs:134-153), `reply_of` reading ok / refusal / fault (client.rs:44-59).
- `src/cmd/send.rs` (87-93, 317) — `exit_of`: the closed `RefusalReason` → 10–14 table and its rstest `exit_of_is_the_closed_table`.
- `src/cmd/revive.rs` (20-260) — `ReviveArgs` (`name`, `--id`, `--fork`, `--list`, `extra`; no `--json`); `Refusal` {StrictModes, NoSession, UnknownId, SessionLive, CwdMissing} with `detail()` (`strict-modes-failed`, `no-session` ×2, `session-live`, `cwd-missing`) and the fixed `pair()` texts (revive.rs:47-100); `already-live` is `run`'s own collision refusal (revive.rs:45-46, `collision_check`); `--list` (`list`, revive.rs:228-245): the strict-modes check, then rows `<ts>  <cause>  <id>` on stdout through `human::result`, `no-session` exit 1 on an empty chain — it opens no process log.
- `src/main.rs` (91-110, 338-363) — `role_of`: `hook` → Hook; `run`, `revive` and a leading `-` → Other; every other first word → Cli. `revive --list` therefore runs as Role::Other.
- `src/obs.rs` (25-40, 150-215, 400-500) — `CONFIG_KEYS` = `v`, `diagnostics_level`, `claude_env_keep`, `five_hour`, `seven_day` (obs.rs:32-38); `read_diagnostics_level` checks `v` = 1 (obs.rs:185) and counts unknown keys as `ConfigRejection::UnknownKeys(n)` → `parse-rejected{parser:"config-json", detail:"unknown-keys"}` (obs.rs:155, its case `unknown_keys` counts `port` and `x`, obs.rs:434-437).
- `src/run/budget.rs` (56-94) — `read_thresholds` / `parse_thresholds`: `take(MAX_FRAME)`, `v` = 1 else both defaults, a non-whole-percent key → its default + one `Malformed` rejection.
- `src/run/env.rs` (1-20) — `claude_env_keep` read from `config.json`, names only.
- `crates/viola-state/src/replay.rs` (145-170) — `read_snapshot_or_replay(instance_dir)` writes the one `state-recovered` line (`detail`, `file`, `v_seen`) through `obs_event!`, whose `instance` is the process's own; the scanned instance is `instance_dir`'s last component, in hand at the call.
- `schemas/diag-line.v1.json` (31, 82-84, 109) — the `state-recovered` row: `detail` ∈ {torn-line-healed, snapshot-replayed, snapshot-unsupported-v}, `file`, `offset`, `v_seen`; default-deny.
- `crates/viola-channel/src/endpoint.rs` (31, 55) — `endpoint_name(name, home)` / `endpoint_path(name, home)` hash the home path: a home spelled differently names a different endpoint.
- `tests/support/home.rs` (300-380) — `stamped_home`: `viola verify` against the fake agent at the recorded version, asserting exit 0 and the `stamped <ver>  <n> pass  0 fail` summary; the panic at home.rs:368 prints the failing row ids.
- `src/cmd/verify/typed.rs` (230-330) — Run E (`resume_run`, typed.rs:251-276): `ledger::resume_target` of Run B's captures, a `TrustedDir` `<cwd>/.viola-verify-<pid>-resume/`, `--resume <id>`, `wait_capture(SessionStart, probe_until)`, settle, kill; `poll_until` sleeps `POLL` between reads.
- `crates/viola-agent-claude/src/ledger.rs` (2988-3050) — `check_resume_session` cases: fails on another source, another id, no Run E capture, an off-shape Run B id.
- `tests/cli_controls_not_disableable.rs` (fn index) — rows for the hook panic seam (four values), the send paste control, the unstamped answer, the human wheel, turn-running and the budget gate (`over_one_hundred`, `negative`, `a_string`, each a `stamped_home` case); no completeness case over every flag, `VIOLA_*` variable and `config.json` key.
- `tests/` (listing) — `cli_revive.rs`, `cli_list.rs`, `cli_verify.rs`, `cli_output_plain.rs` exist; no `cross_exit_causes.rs` and no per-verb `--json` / exit witness file.
- CI `ci#38994749835`, job 117284718714 (`test (ubuntu-latest)`), its log (read whole for the failure block and the summary): image `ubuntu-24.04` `20261012.334.1`; `setting_does_not_disable_the_budget_gate::case_1_over_one_hundred` PASS 4.340 s finishing 02:25:23.905; `::case_3_a_string` FAIL 4.439 s and `::case_2_negative` FAIL 4.464 s finishing 02:25:24.846 / .854 — both `panicked at tests/support/home.rs:368:5: viola verify did not stamp the home: exit Some(1), summary line matched false, failing rows ["resume-session"]`; `Summary [ 226.739s] 2666 tests run: 2664 passed (5 slow), 2 failed`. The run's artifacts include `harness-ubuntu-latest` (58 740 250 B) and `diag-ubuntu-latest`, unexpired.

## Graph impact
- **resolve_home** — 10 callers, all `dispatch` (src/cmd/mod.rs:80, 93, 105, 110, 115, 120, 126, 135, 140, 145); no other file names it (sweep below). A `VIOLA_DIR` step changes every non-hook verb's home at once.
- **default_home** — `resolve_home` (src/cmd/mod.rs:178), `statusline_source` (src/cmd/run.rs:491), its unit (src/cmd/mod.rs:191): the "default home only falls back to `~/.claude/settings.json`" rule rides it.
- **document** (client.rs) — 7 product callers (answer.rs:65, 72; client.rs:123, 136; last.rs:48; list.rs:54, 69; pause.rs:74; send.rs:132, 148, 155; wait.rs:67): every landed `--json` document goes through it. `release` reaches it through `pause::wheel_call`.
- **unreachable / fault** — the exit-21 / exit-20 writers for `answer`, `last`, `pause` (`wheel_call`), `send`, `wait`.
- **read_diagnostics_level** — 5 product callers (client.rs:23, hook.rs:126, hook/statusline.rs:59, run.rs:180, verify.rs:64); **read_thresholds** — 2 (list.rs:98, run.rs:257); **log_config_rejection** — 7 sites.
- **role_of** — `main` (src/main.rs:53) and its unit table (src/main.rs:342).
- **refuse** (human) — the `unable:` / `hint:` pair writer for `list`, `revive` (`write`), `run` (five start refusals), `verify` (five sites), the wrapper's `send`.
- Trace: `.andromeda/runs/2026-10-17T02-25-51-phase/tree-query-2026-10-17-cli-machine-contract.json` (one query, plane `rust`, non-empty).

## Patterns detected
- **One shared `--json` writer** (src/cmd/client.rs:116): every landed document is one `write_all` on stdout; refusal / fault / unreachable documents are built in `client` and the verb's own module.
- **Strict-modes before any log** (src/cmd/revive.rs:105-108, 228-232; src/cmd/list.rs:41-46): a refused check opens nothing under the home; `list`'s exit-21 document is the refused-scan template.
- **The closed refusal → exit table** (src/cmd/send.rs:87-93) with its rstest `exit_of_is_the_closed_table` (send.rs:317).
- **The `VIOLA_DIR` shape check** (src/cmd/hook.rs:80-97): name parsed, dir absolute, `instances/<name>` suffix, home = grandparent.
- **A replay line from the library** (crates/viola-state/src/replay.rs:162-168): the line's fields come from the call site, `instance` from `ProcessCtx`.

## Conventions to follow
- **Human output through `human`**: pairs via `human::refuse` (revive.rs:97), results via `human::result` (revive.rs:242); a `--json` writer only in the owning module (obs-plan §11 Logs).
- **Every `--json` document carries `v` = 1 and `preserve_order` key order** (client.rs:69-71, 145).
- **rstest `#[case::label]` tables for closed maps** (send.rs:317, main.rs:338-363).
- **Root CLI tests through `tests/support/cli.rs`'s runner and a fresh `TestHome`**; a revive test puts the fake agent first on `PATH` (testing 2026-10-10).

## New files to create
- `tests/cli_machine_contract.rs` — the per-verb witness: each verb's `--json` document and literal exit read side by side for every outcome the verb reaches, the `--home` / `VIOLA_DIR` resolution order and home isolation.
- `schemas/config.v1.json` — the tolerant `config.json` schema test-plan §5's completeness case reads ("the config schema"; `ls schemas/` holds no config schema at P4): `v` and the four setting keys, unknown keys allowed.

## Files to modify
- `src/cmd/mod.rs` — `resolve_home`: `--home`, then the `VIOLA_DIR` grandparent through the shared shape check, then the default; canonicalisation per security-plan §Input Validation CLI row.
- `src/cmd/hook.rs` — `instance_of` shared with the CLI verbs' home resolution (moved or re-exported, no behaviour change).
- `src/cmd/revive.rs` — `--json` on both arms: `--list` rows as one document, each exit-1 refusal as one document; `--list` opens the `cli` role log after its strict-modes check.
- `src/main.rs` — `role_of`: `revive --list` files under `cli`.
- `src/cmd/client.rs` — the exit-1 error document writer beside `fault` / `unreachable`.
- `crates/viola-state/src/replay.rs` — `state-recovered` names the scanned instance.
- `schemas/diag-line.v1.json` — the scanned-instance field on the `state-recovered` row.
- `tests/cli_revive.rs` — revive's `--json` documents and the `--list` log lines.
- `tests/cli_list.rs` — `e5_list_reads_a_live_wrapper_through_the_replay` reads the scanned-instance field.
- `tests/cli_controls_not_disableable.rs` — the completeness case.
- `src/obs.rs` — its test module only: a unit pinning `CONFIG_KEYS` to `schemas/config.v1.json`'s properties.
- `tests/contract_diag_schema.rs` — the new field validated.
- `src/cmd/run.rs` — the home canonicalised once `run` has created it, before any read.
- `tests/support/cli.rs` — the test runner's child drops `VIOLA_DIR` and `VIOLA_BIN` beside `VIOLA_NAME`.
- `tests/support/home.rs` — the wrapper boot's child drops `VIOLA_DIR` and `VIOLA_BIN`; the stamped-home fixture's verify run, if the red's cause lies in it.
- `tests/support/start.rs` — its `endpoint_name` computed over the resolved home.
- `tests/cli_instance_state.rs` — the default-home case compares the resolved home.
- `tests/cli_verify.rs` — the concurrent-start stamp witness for the CI red.
- `src/cmd/verify/typed.rs` — Run E's wait, if the red's cause lies in it.
- `src/bin/viola-fake-agent.rs` — the fake agent's resume or registry path, if the red's cause lies in it.

## Sweeps
- `read_snapshot_or_replay` over `src crates tests` (`*.rs`): 4 hits · 2 changed (replay.rs, the field; state_replay.rs reads the line only if it asserts the field set — read at implement) · 2 no-change (board.rs, revive.rs: callers, signature unchanged) — `grep -rln --include=*.rs`.
- `resolve_home`: 1 hit (src/cmd/mod.rs) · 1 changed — same command.
- `state-recovered`: 7 hits · 3 changed (replay.rs, cli_list.rs, contract_diag_schema.rs) · 4 no-change (viola-core obs.rs the enum; events.rs the torn-line emitter; chaos_torn_append.rs and cli_revive.rs assert other details) — same command.
- `stamped_home`: 21 hits (20 test binaries + `tests/support/home.rs`) · 1 changed if the red's cause is in the fixture · 20 no-change (consumers) — same command.
- `role_of`: 1 hit (src/main.rs) · 1 changed — same command.
- `endpoint_name(\|endpoint_path(` over `tests crates/viola-e2e src` (`*.rs`): 3 hits · 1 changed (tests/support/start.rs: a test-side hash of the home's spelling) · 2 no-change (src/cmd/run.rs, src/cmd/hook.rs: product sites, which read the one resolved home) — `grep -rln --include=*.rs`.
- `default_of_user` over `tests crates/viola-e2e`: 3 hits · 2 changed (tests/cli_instance_state.rs:409's default-home case, tests/support/home.rs's `boot_with`, which passes no `--home` and so reads the inherited `VIOLA_DIR` once the step lands) · 1 no-change (the helper's own definition, home.rs:188) — `grep -rn`.
- `env_remove("VIOLA_NAME")` over `tests` and `crates/viola-e2e/src`: 10 hits; `env_remove("VIOLA_DIR")` 2 (tests/cli_output_plain.rs:23, tests/cli_version_gate.rs:216) — every test-started child given `--home` keeps working after the step (`--home` wins first); the runner and the boot strip `VIOLA_DIR` so a default-home case started from a wrapped session (this host's sessions carry `VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`: `env | grep -o '^VIOLA_[A-Z_]*'`) never reads the bridge's home — `grep -rn`.

## Findings that close scope premises
- **Per-verb `--json` as landed** — `grep -n -B1 'json: bool' src/cmd/*.rs`: `send`, `wait`, `last`, `answer`, `list`, `pause`, `release` carry `--json`; `revive` and `verify` do not. architecture §Established Decisions [CLI Conventions] names `viola revive` as the ONE verb without a machine form; `verify`'s agent-readable outcome is its stdout step lines + summary + exit (architecture-amendments `2026-09-28-capability-ledger-and-viola-verify`; obs-plan-amendments, the same marker); `run` is the interactive wrapper whose stdout is the child's screen.
- **`config.json` as landed** — the `v` check (obs.rs:185, budget.rs:72) and the unknown-key count (`unknown-keys`, obs.rs:155) are built; `port` reads as an unknown key (obs.rs:434). What stays owed is the completeness witness that no flag, `VIOLA_*` variable or key disables a control.
- **The red is a non-deterministic failure on unchanged code** — `git diff --stat ae21376 f6bcce5 -- '*.rs' 'Cargo.*' '.config/*' '.github/*'` prints nothing: the green `ci#38992234808` and the red `ci#38994749835` built the same code. The first recorded failure of the `resume-session` row since it landed (`3fc2e61`, 2026-10-14): `gh run list --workflow ci.yml --limit 20` reads ten completed runs since, every other red a known chunk red. `hypothesis:` two `stamped_home` verify runs started in the same instant on one runner contend for something Run E reads (case_1, started alone, passed; case_2 and case_3 finished 8 ms apart) — unmeasured; its witness is the kept home in `harness-ubuntu-latest` and a forced concurrent start, never a re-run.

## Open questions
- The red's cause (Run E's capture missing, another id, or another source) → blocks: implementation-scope — the last two files of `Files to modify` stand provisional until implement reads the kept artifact and forces the concurrent start.
- `std::fs::canonicalize` of `--home`: the home may not exist yet (`run` creates it), and on Windows canonicalisation yields a `\\?\` verbatim path while `endpoint_name` hashes the home's spelling → blocks: plan-decision — where in the order canonicalisation happens (after creation for `run`, before any read for every verb) and that `run` and every client spell the same home.
- The machine shape of `revive`'s exit-1 refusals (no channel `result` exists for them; arch's `error` documents are 20 / 21 only; design marks exit 1 as having no `--json` document "until arch amends it") → blocks: plan-decision.
