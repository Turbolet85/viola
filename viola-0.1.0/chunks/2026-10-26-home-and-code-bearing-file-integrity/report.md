# Report — 2026-10-26-home-and-code-bearing-file-integrity

**Chunk:** the home's strict-modes check before every read or open at every entry point, owner-only modes and DACL, the pinned binary and plugin re-checked, single stamp and snapshot writers
**Date:** 2026-10-26
**Commits:** `af6c67ef` chore(…): operator pre-CI commit · `f65a4ae8` fix(…): the two Windows reds of ci#40398601594 · `e3468500` fix(…): the two Windows reds of ci#40400887151 (basis: `git log --format='%h %s' 99df4536..HEAD`; the chunk's code is in `af6c67ef`, base `99df4536`, the parent of the oldest pre-CI commit)

## Changes (structured — detectors read this)
- **Files:** 35 (basis: `gate.py scope` — `changed 35 · listed 30 · recorded 5`; `git diff --stat 99df4536 -- src crates tests schemas`: 35 files, +2411 −167). Product (21): `crates/viola-state/src/{strict,stamps,liveness,pin}.rs` · `crates/viola-mcp/src/tools.rs` · `src/cmd/{client,link,send,wait,last,answer,pause,release,hook,run,revive,verify}.rs` · `src/cmd/hook/statusline.rs` · `src/obs.rs` · `schemas/diag-line.v1.json`. Tests: new `tests/security_negatives_strict_modes.rs` (475 lines), `tests/chaos_snapshot_replay.rs` (147 lines); modified `crates/viola-mcp/tests/handlers.rs`, `tests/{hook_fail_open,hook_statusline,run_cli,cli_verify,conpty_sideload,cli_controls_not_disableable}.rs`, `tests/support/home.rs`; companions (scope record) `tests/{cli_revive,cli_wait_last,contract_diag_schema,cli_machine_contract,tui_env_strip}.rs`. Records: `viola-0.1.0/verification-matrix.json` (v1-45, v1-17, v1-41 → `implemented`), `chunks/{marker}/scope-record.md`, `evidence/remove-the-guard.md`, `evidence/operator-pass.md`.
- **Symbols / APIs:** (coordinates from `## New text, by line`)
  - `viola_state::strict` (strict.rs): `Refused` gains `Readable` («a trusted state path is readable by another user») and `Symlink` («a trusted state path is a symbolic link»), fixed messages; new private `enum Rule { Write, Read }` (36-41); `pub const DIAGNOSTICS = "diagnostics"`; new `pub fn unix_read_verdict(mode, uid, euid)` (209-217: the write rule plus `mode & 0o077 != 0` refused) and `pub fn windows_read_verdict(…)` (279-296: the write rule plus any allow ACE granting a right in the pinned `pub const READ_RIGHTS: u32 = 0x9000_0001` — `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL` — to a SID `trusted_sid` rejects, inherit-only ACEs included); new `pub fn check_read(path)` (the read rule through the OS reader) and `pub fn check_present(path)` (160-163: the write rule on one path that exists, an unstattable one refused). `check_instance(home, instance_dir)` (signature unchanged, 3 product callers, below) now reads the home (READ rule), `instances/` (write), the instance directory (READ), its `diagnostics/` (READ, new in the set), `snapshot.json` (write) and `events.ndjson` (Unix write rule; Windows READ rule), each that exists, a folder before what it holds. New `pub enum TreeRefused { HomeLevel(Refused), InstanceLevel(Refused) }` (83-89) and `pub fn check_home_tree(home, name: Option<&ViolaName>, key: Option<&str>)` (91-125): the home (read rule) → with a name, `check_instance`'s set → `ledger/`, `bin/`, `plugin/` → with a key, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` (write rule), each that exists. New `pub fn check_diagnostics(path)` (127-145): a symlink refused (`symlink_metadata`), a folder the read rule, a file the read rule on Unix (no group/other bit) and the write rule on Windows; an absent path passes. Unit cases 1111-1385 (`unix_read_verdict_*`, `windows_read_verdict_*`, `check_home_tree_*`, `check_diagnostics_*`, `strict_instance_holds_the_content_folders_to_the_read_rule`) and `cfg(windows)` cases 708-830 (CI's leg: a tree granted Users write beside one that passes, a `diagnostics/` granted Users read, `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, `read_rights_are_the_named_flags`).
  - `viola_state::stamps`: new `pub enum StampsError { StrictModes(Refused), State(StateError) }` (37-44, fixed messages «the capability stamps failed the strict-modes check» / «… could not be written») with `From<io::Error>`; `update_stamps(home, update)` now returns `Result<(), StampsError>` (was `StateError`) and runs `strict::check_stamps(home)` BEFORE `create_private_dir(ledger/)` and again under the lock before the capped read; a refusal reads and writes nothing. Sole product caller `src/cmd/verify.rs` (`measure`), which maps `StrictModes` to the home pair.
  - `viola_state::liveness::live_endpoint(instance_dir)` (signature unchanged, 7 callers: answer, last, link, pause, send, wait, viola-mcp tools.rs) reads `replay::read_snapshot_or_replay` (92-94): only `Recovered::Snapshot` proceeds; `Replayed`, `Absent` or an error → `None` (a replayed snapshot carries no `endpoint`, `pid`, `started_at`). Unit `live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line` (255-291).
  - `viola_state::pin`: `PinError` gains `StrictModes` («the pinned copy failed the strict-modes check»); new `pub fn pin_key(bytes) -> String` (71-74, `<VERSION>-<16 hex>`) and `pub fn pin_bytes(home, bytes)`; `pin_exe(home, exe)` keeps its signature and delegates to `pin_bytes` (verify still calls `pin_exe`; `run`'s start calls `pin_bytes` with the bytes it already checked). Windows `pin_companions` runs `strict::check_present` on `bin/<key>/conpty/` and on each companion that exists before the held open: a refusal is `PinError::StrictModes`, nothing written.
  - `viola-mcp` `tools.rs` `frame` (337-339): `check_instance(&ctx.home, &instance_dir)` before `live_endpoint`; a refusal returns `result::unreachable()` (`instance-unreachable`, `detail: null`), no frame, no line.
  - root bin `src/cmd/client.rs`: new `pub(crate) fn refuse_untrusted(home, name, json) -> anyhow::Result<Option<u8>>` (20-41): `check_instance` on `std::path::absolute(home)` and `instances/<name>`; refused → under `--json` `refused_document("instance-unreachable")` (exactly `{"v":1,"error":"instance-unreachable","detail":null}`), else `human::refuse` of `human::instance_state_pair(name)`; `Some(21)`; opens nothing under the home and writes no line. Called first by `send` (send.rs:190-192, after its text read), `wait` (43-45), `last` (30-32), `answer` (94-96), `pause` (32-34), `release` (35-37); `link_call` (link.rs:94-95) moved onto it unchanged in behaviour.
  - `src/cmd/hook.rs`: `canonical` (75-81) and `trusted` (83-86) moved here from `statusline.rs`, shared by both arms; the event and dialog paths take `instance_of(…).and_then(canonical).filter(trusted)` before `read_diagnostics_level` / `viola_obs_init`: a refused or absent instance returns exit 0 at once — no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append. A `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing. The two `--capture` arms stay ahead of it, unchanged; `FAKE_AGENT_HOOK_PANIC` keeps its place after `viola_obs_init`. Hook `connect` (224-226) reads `read_snapshot_or_replay`: `Replayed` / `Absent` → `NotSent::Unreachable` (fail open, `channel-unreachable`, SessionEnd's direct append as before). Unit `connect_over_a_replayed_snapshot_is_unreachable` (640-694).
  - `src/cmd/hook/statusline.rs` `serve` (128-132): `read_snapshot_or_replay`; a replayed snapshot has no `statusline_command`, so nothing runs or prints. Unit `statusline_over_a_replayed_snapshot_runs_nothing` (524-553).
  - `src/cmd/run.rs`: new `pub(super) fn checked_exe(home, name)` (189-200: reads the exe's bytes once, takes `pin_key`, runs `check_home_tree(home, Some(name), Some(&key))`) and `pub(super) fn refuse_tree(name, TreeRefused)` (202-215: `HomeLevel` → `human::home_state_pair()`, `InstanceLevel` → `human::instance_state_pair(name)`, each through `human::refuse`); `run` (166-172) and `revive`'s start (revive.rs:125-135) call them before `open_wrapper_log`: exit 1, no log opened, no line, no child (revive `--json` keeps `strict-modes-failed`). `collision_check` now takes the caller's ONE read (`Option<&Recovered>`, 425-432): `Replayed` → liveness from the beat alone (`beat_age ≤ STALE_AFTER` → `already-live`, else free); an error or `Absent` → free. `revive`'s preflight reads once and hands that read to both its collision check and its recorded-directory reading. `sideload_outcome` maps `PinError::StrictModes` to the outcome `strict-modes-failed` (run.rs:510). Unit `collision_check_over_a_replayed_snapshot_reads_the_beat_alone` (1173-1224).
  - `src/cmd/verify.rs`: before its `cli` log, `check_home_tree(&home, None, None)` (64-70; the home, `ledger/`, `bin/`, `plugin/`); refused → `refuse_home()` (95-99, `human::home_state_pair()` through `human::refuse`), exit 1, no step line, no summary, no line; `measure`'s `StampsError::StrictModes` maps to the same pair (288-296).
  - `src/obs.rs`: new private `check_diagnostics(dir, file)` (69-76) runs `strict::check_diagnostics` on the existing `diagnostics/` folder and the role or detail file before `ensure_private_dir` narrows anything, in `open_role_file` and `write_detail`; a refusal is an `io::Error`, exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped). Units 610-673.
  - Remaining-caller facts (basis: `grep -rn 'check_instance(' src` and `git grep -n 'check_instance(' 99df4536 -- src`): `check_instance(` under `src/` 3 → 3, its files moved — `src/cmd/hook/statusline.rs:45` → `src/cmd/hook.rs:85` (`trusted`), `src/cmd/link.rs:98` → `src/cmd/client.rs:31` (`refuse_untrusted`, shared by link/unlink and the six verbs), `src/cmd/revive.rs:151` (was :137). `read_snapshot(` product callers under `src/` 3 → 0 (basis: `git grep -n 'read_snapshot(' 99df4536 -- src/cmd` — hook.rs:204, statusline.rs:142, run.rs:383 — against `grep -rn 'read_snapshot(' src` at HEAD, whose only `src/cmd` hit, run.rs:1279, is inside `mod tests` (run.rs:773)). `human::refuse(` under `src/` 15 → 18 (basis: `grep -rcE 'human::refuse\(' src` at HEAD — verify 6, run 7, list 2, revive 1, plugin 1, client 1 — against `git grep -c 'human::refuse(' 99df4536 -- src` — link 1, list 2, plugin 1, revive 1, run 5, verify 5). `client::start` keeps its 10 callers; `viola_obs_init` its 6 (each now behind a check).
- **Crates / modules:** no crate or module added or removed; changed: `viola-state` (strict, stamps, liveness, pin), `viola-mcp` (tools), root bin (cmd/*, obs). `tests/security_negatives_strict_modes.rs` and `tests/chaos_snapshot_replay.rs` are new root test binaries.
- **Dependencies:** none (no `Cargo.toml` / `Cargo.lock` change; no windows-sys feature added; basis: `gate.py scope`'s changed list holds neither).
- **Schema / config:** `schemas/diag-line.v1.json` line 70: `process-start.sideload_fallback` enum gains `strict-modes-failed` (now `hash-mismatch`, `strict-modes-failed`, `unreadable`, `load-failed`, `not-built`); no config key, env var or flag added.
- **Spec-master edits:** none (no `.andromeda/` master or key file changed by the chunk; basis: `gate.py scope` changed list).
- **Retired:** the eight founder-ratified dated strict-modes gaps (security-plan §Authentication & Authorization / `.claude/rules/security.md`), each "until the Epoch 6 route entry 'Home and code-bearing file integrity'": (1) `viola hook`'s `hook.event` snapshot read without strict-modes; (2) `verify`'s `update_stamps` reading `ledger/stamps.json` without strict-modes; (3) Windows x64 ConPTY companions loaded under the re-hash alone, without the owner/DACL check; (4) CLI `send`; (5) CLI `wait` / `last`; (6) CLI `answer` and `hook pre-tool-use|permission-request`'s `hook.dialog`; (7) CLI `pause` / `release`; (8) `viola mcp`'s `send` / `wait` / `last` / `answer` tools — each now runs the strict-modes check before its first read or open. The `connect` doc's dated-gap sentence (hook.rs), `client.rs`'s module-doc gap and liveness.rs's gap note are removed. `read_snapshot` as a product reader under `src/` (3 → 0). No census row (no `census.md` in the version dir).
- **Counts / qualifiers moved:** (each a fact; pattern ids from `{run_dir}/report-patterns.toml`, swept by `cascade.py sweep --pre-pass` at HEAD `e3468500`, trail `cascade-{marker}-pre-pass.json`)
  1. dated strict-modes gaps 8 → 0 — patterns `gap-until` (standing 4, all security-plan: security-plan.md:142, authentication-authorization.md:36, input-validation.md:30, :54 @c886 · 4 stale) and `owner-title` (standing 25: architecture 2, security-plan 15, test-plan 6, obs-plan 2 · 25 stale — every row names this entry as the owner of a gap, a CARRY or a not-yet-built case this chunk closed; dispositions per row below).
  2. `check_instance(` sites under `src/` 3 → 3, files moved (statusline.rs → hook.rs, link.rs → client.rs) and its set widened (the home, the instance dir and `diagnostics/` to the read rule; Windows `events.ndjson` to the read rule) — pattern `check-instance` (standing 27: architecture 7, security-plan 11, test-plan 4, obs-plan 5 · 1 stale count line (occupied-resources.md:112 @c2225/@c2397: «three product call sites (`src/cmd/h…` … `src/cmd/link.rs`, the one site both link verbs share»), 26 true claims sharing the token whose statement of WHAT the check reads is re-read against the new set (detectors)).
  3. the read rule built (Unix `mode & 0o077` on the home, `instances/<name>/`, `diagnostics/`; Windows a read grant to another SID, `READ_RIGHTS`) — pattern `read-rule` (standing 7: security-plan 4 (authentication-authorization.md:18, :38, :40, bootstrap-phases.md:16), test-plan 2 (unit-test-strategy.md:23, :24), obs-plan 1 (registries otel-sdk-init.md:15 @c1154) · 7 standing: the masters already state the rule as the target; each re-read for «not yet in code» wording (authentication-authorization.md:38 «This i…»)).
  4. new tree checks `check_home_tree` / `check_diagnostics` / `check_present` / `TreeRefused` / `refuse_untrusted` — pattern `tree-checks` (standing 1: security-plan authentication-authorization.md:36 (`check_stamps`) · 1 true claim; the four new names stand in no master — a fact to register).
  5. replay product readers 2 (`revive`, `list`) → 6 (+ hook `connect`, statusline `serve`, `run`'s `collision_check`, `live_endpoint`); `read_snapshot(` product callers 3 → 0 — pattern `read-snapshot` (standing 20: architecture 5, security-plan 5, test-plan 5, obs-plan 5 · stale: established-decisions.md:79 ×2, :110, :207, standard-contracts.md:109, authentication-authorization.md:25 @c190/@c673, :27 @c1774, :63, input-validation.md:54 @c187/@c344/@c586/@c634, test-plan.md:590, e2e-test-strategy.md:236, :237, unit-test-strategy.md:19, :22, span-trace-coverage.md:30, :192 — 17 rows stale (each says a reader «stays on `read_snapshot`» or names the 3 callers); 3 true (standard-contracts.md:25, input-validation.md:53, obs-plan.md:394 / log-coverage.md:19 / span-trace-coverage.md:191 name the replay or its two detail words)).
  6. `state-recovered` emitters / the `state.snapshot_recover` span placement (now under `run.start` before `run.collision_check`, and under `hook.handle` on the hook path) — pattern `state-recovered` (standing 24: architecture 4, security-plan 1, test-plan 6, obs-plan 13 · the emitter-list rows (established-decisions.md:46, occupied-resources.md:104, standard-contracts.md:25, e2e-test-strategy.md:237, unit-test-strategy.md:18, :19, span-trace-coverage.md:191, :192, telemetry-strategy.md:33) re-read for the four new readers; the vocabulary rows (log-format-json-schema.md:18, :34, logging-stack.md:10, log-format.md:16, obs-plan.md:396, :460, log-coverage.md:19, metric-coverage.md:25, span-trace-coverage.md:198, telemetry-strategy.md:38, occupied-resources.md:105, input-validation.md:53, tss:172, e2e:327) no-change — the event and its words did not change).
  7. `sideload_fallback` enum 4 → 5 words (+`strict-modes-failed`) — pattern `sideload-fb` (standing 6: architecture 2 (architecture.md:319, established-decisions.md:17), security-plan 1 (input-validation.md:80), obs-plan 3 (log-coverage.md:10, span-trace-coverage.md:55, :56) · 6 stale where the word list or the fallback causes are enumerated).
  8. the six frame verbs' human strict-modes refusal: the instance pair, exit 21 — pattern `instance-pair` (standing 7: architecture 2 (conventions.md:23, registries project-directory-structure.md:23), design-system 3 (surface.md:37, :67, :122), layout-templates 2 (layout-templates.md:253, :329 @c1806/@c2291/@c2326) · 7 re-read: each lists the verbs printing the pair (revive, link/unlink) — the six frame verbs join the list).
  9. `run` and `verify` print the home pair (exit 1) on a refused home tree; `run` the instance pair for an instance-level refusal — pattern `home-pair` (standing 13: architecture 2, design-system 4, layout-templates 3, obs-plan 1, a11y-plan 3 · re-read: design-system surface.md:65 «(strict-modes at…» and :122 name where the home pair stands; the rest are `list` / `plugin install` / `--home` markup rows, no-change).
  10. `human::refuse(` sites under `src/` 15 → 18 — pattern `refuse-count` (standing 3: security-plan 1 (authentication-authorization.md:35 @c931), obs-plan 2 (obs-plan.md:311 «`run` (5 sites), `verify` (5), `list` (2), `revive` (1), `plugin install` (1) and `link` (1…)», obs-anti-patterns.md:8) · 2 stale (obs-plan.md:311 per-file figures; authentication-authorization.md:35 @c931), 1 rule-only (obs-anti-patterns.md:8, no figure)).
  11. «`strict-modes-failed` cannot occur» on `wait` / `last` — now a refused check exits 21 with no line — pattern `cannot-occur` (standing 4: obs-plan 2 (obs-plan.md:361, span-trace-coverage.md:102), a11y-plan 2 (a11y-scope-summary.md:8 @c2438, cognitive-accessibility.md:45) · obs-plan 2 stale; a11y-plan 2 re-read — they say a server-verify refusal «cannot occur» on macOS (no peer pid), a different fact sharing the phrase: no-change).
  12. the instance detail file `detail-<process>.ndjson` not named at architecture.md:323 (§Occupied Resources, `diagnostics/`) — pattern `detail-file` (standing 42: architecture 3 (architecture.md:412, established-decisions.md:213, occupied-resources.md:111), test-plan 6, obs-plan 33 · 42 true claims (each names the detail file); the moved fact's site, architecture.md:323, holds no hit — «Content-bearing detail stays in `instances/<ViolaName>/diagnostics/`» — so it is no row of this pattern).
  13. an unstattable home (a path under a regular file) is now a strict-modes refusal (Unix: `verify` exit 1 home pair; the frame verbs exit 21 instance pair) where the `cli` catch-site line stood; on Windows it stats `NotFound` and the catch-site line stands — pattern `catch-site` (standing 2: architecture 1 (established-decisions.md:213), obs-plan 1 (obs-plan.md:496) · 2 re-read: both describe the catch-site reporter itself, which is unchanged; the narrowed reach is new text).
  14. `viola list`'s `board::scan`: `check_path` (write rule) on the home, then `check_instance` per instance — which now holds the home to the read rule, so a 0750 home passes the board with no instance and refuses naming the instance (`ScanRefused::Instance`) with one — pattern `board-scan` (standing 9: architecture 5, security-plan 3, obs-plan 1 · 9 true as written (they state the two calls); the inconsistency is the new fact, `board.rs` unchanged; owner routed below).
  15. the seed carve-out: viola makes the seeded home (`create_private_dir`) before `bin/<key>/conpty/`, and the seed dir `target/conpty-seed/<key>/` too — pattern `seed` (standing 8: architecture 2 (architecture.md:343, :370), security-plan 1 (security-anti-patterns.md:17), test-plan 5 (test-plan.md:631, integration-test-strategy.md:32, :33, test-data-fixtures.md:10, registries test-data-bootstrap.md:11) · stale: integration-test-strategy.md:33 («a seeded home skips it, so that entry owns making the seed let viola create the home first»), test-data-fixtures.md:10 («the refused home's DACL was not read»); 6 re-read for the home-creation step).
  16. single writers witnessed with the wrapper STOPPED (`Wrapper::stop`), not SIGSTOPped — pattern `single-writer` (standing 6, all test-plan: test-plan.md:606 stale («single writers of `stamps.json` and `snapshot…`» — the method), :613, :943, e2e-test-strategy.md:305 («per stopped (SIGSTOP on Unix; on Windows t…» stale), :330, test-strategy.md:26 — 4 no-change (the chaos SIGSTOP stale-wrapper case is another test)).
  17. `update_stamps` refuses before reading (`StampsError`) — pattern `stamps-err` (standing 7: architecture 4 (architecture.md:154, :332, occupied-resources.md:126, standard-contracts.md:31), security-plan 3 (authentication-authorization.md:36, input-validation.md:54 @c1015, :90) · security-plan 2 stale (the gap: «`update_stamps` reads …» without strict-modes), 5 true (the single writer, the over-cap-as-absent read)).
  18. the `conpty/` companions' owner/DACL leg built (`PinError::StrictModes` → `sideload_fallback` `strict-modes-failed`) — pattern `conpty-dacl` (standing 7: architecture 4 (architecture.md:319, established-decisions.md:205, :225, stack-and-technologies.md:19), security-plan 3 (authentication-authorization.md:36 «Windows owner/DACL check, unt…», data-protection.md:16, input-validation.md:81) · 4 stale (authentication-authorization.md:36, data-protection.md:16 (`PinError` variants), architecture.md:319, established-decisions.md:225), 3 true (they name `pin_companions` as the writer)).
  19. E5's two signals restated (`wheel:"human"` does not hold after step 3 — every start appends `wheel{holder:"driver", cause:"start"}`; the replay writes no file) — pattern `wheel-human` (standing 6: architecture 2 (architecture.md:190, established-decisions.md:204), test-plan 3 (test-plan.md:586, :592, e2e-test-strategy.md:237), obs-plan 1 (span-trace-coverage.md:138) · test-plan 3 stale (:586 lists the signal, :592 is the set-against note «for the entry that builds it to restate», e2e:237); architecture 2 and obs-plan 1 true (the wheel's own contract)).
  20. each new no-line refusal site (the six verbs, the hook event/dialog paths, the MCP frame tools, `verify`, `run`/`revive`'s tree check, the diagnostics opens) — pattern `no-line` (standing 43: architecture 7, security-plan 6, layout-templates 1, test-plan 9, obs-plan 19, a11y-plan 1 · the exemption-list rows re-read for the new sites (occupied-resources.md:103 @c215/@c1095, :105, :111, :112 @c446/@c1336, authentication-authorization.md:27 @c1506, :34, :35 @c292/@c1072, error-capture-reporting.md:15, span-trace-coverage.md:18, :54, :217, obs-scope-summary.md:300, slo-invariants-telemetry-budgets.md:17, log-coverage.md:35, e2e-test-strategy.md:291, integration-test-strategy.md:58); the 6 `Vector 7` source labels (test-plan.md:49, :68, :89, :207, test-scope-summary.md:128, obs-plan.md:213) and the rest no-change).
  21. `Refused` variants 5 → 7 (+`Readable`, +`Symlink`) — pattern `refused-variants` refused: «control: … refused-variants never fired over the seven masters' pre-pass text» — no master site names the variants.
  22. `pin_key` / `pin_bytes` new; `pin_exe`'s race and reproducible builds carved out — pattern `pin-race` refused: «control: … pin-race … never fired over the seven masters' pre-pass text».
  23. a hook whose `VIOLA_DIR` names no existing directory writes nothing — pattern `viola-dir-none` refused: «control: … viola-dir-none … never fired».
  - Owner pattern: `owner-title` standing 25 (above); `owner-generic`'s control unfired (dropped).
  - **Sweep reach: 21 patterns · 23 moved facts, 23 with a pattern (3 of them a refused control) · owner pattern 25 standing · by line 6 lines · 16 offsets** (P from the `total (21 patterns)` line; F and G the report's own count).
  - Lines over 2 000 chars, a disposition per (line, fact) pair, each offset printed: occupied-resources.md:103 (2023) no-line @c215, @c1095 → extend (hook event paths, MCP tools join the statusline sentence) · occupied-resources.md:112 (2428) check-instance @c72, @c997 → true, @c2225, @c2397 → stale (call-site files); no-line @c446, @c1336 → extend (six verbs) · authentication-authorization.md:25 (2173) owner-title @c353, @c558, @c1680 → stale; read-snapshot @c190, @c673 → stale · authentication-authorization.md:27 (2024) owner-title @c610 → stale; read-snapshot @c1774 → stale; no-line @c1506 → extend · authentication-authorization.md:35 (2375) no-line @c292, @c1072 → true (link), extend; check-instance @c658, @c1583 → true, extend (the verbs' order); refuse-count @c931 → stale · input-validation.md:54 (2167) owner-title @c757 → stale; read-snapshot @c187, @c344, @c586, @c634 → stale; gap-until @c886 → stale; stamps-err @c1015 → stale · security-anti-patterns.md:8 (2089) owner-title @c1035 → stale · registries otel-sdk-init.md:15 (2057) read-rule @c1154 → true; detail-file @c1647 → true · registries 5-command-implementation.md:35 (4602) check-instance @c3764 → true · layout-templates.md:329 (3181) home-pair @c1641, @c2049 → true; instance-pair @c1806, @c2291, @c2326 → extend (six verbs); no-line @c2453 → true · a11y-scope-summary.md:8 (2520) home-pair @c1784 → true; cannot-occur @c2438 → true (macOS server-verify, a different fact).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none (no `scripts/agent-run.*`, CI or harness change; `tests/support/home.rs` gains test helpers `widen` · `grant_read` · `unwiden` · `still_widened` (331-389; Unix modes, Windows `icacls` with its exit asserted) and `seed_conpty` makes the home and the seed dir with `create_private_dir`).
- **Cross-project / external claims:**
  - CI (the operator pass, `evidence/operator-pass.md`): `af6c67ef` → ci#40398601594 `red` 14/15 (`test (windows-2025)` 2 of 3175 failed: `verify_under_a_regular_file_refuses_the_home`, `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl`) · `f65a4ae8` → ci#40400887151 `red` 14/15 (2 of 3175: the DACL oracle vs `LA`; `run_rust_log_changes_nothing`, a leaked `strict-modes-failed` fallback) · `e3468500` (HEAD) → ci#40403389629 `verdict: green · checks 15/15 · wall 715 s`, `test (windows-2025)` `3175 tests run: 3175 passed (5 slow), 0 skipped` (job 121854822279); kept homes (artifact `diag-windows-2025`): 264 child starts `conpty-sideload`, 3 `hash-mismatch`, 1 `strict-modes-failed` (the companion test's own). The verdict was read on `e3468500`; this wrap's commit adds no source to it.
  - Measured on `windows-2025` (ci#40400887151's log): the protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` — canonical SDDL spells the runner's user (local Administrator, RID 500) `LA`.
  - `inputs.py verify` (this wrap): `inputs: 5 entries — unchanged 2 · drifted 1 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 2 · unparsed 0` (before this report existed; I4 and I5 are cited here):
    - I1 · `../additional/viola-overseer/phase-home-integrity-directive.md` · copy no-repo · unchanged — cited scope.md, research.md, plan.md.
    - I2 · `../additional/andromedaV3:andromeda-overseer/docs/design-slots/masters-classify-pilot-2026-10-15/rulings.md` · pointer committed@333e4934 · **DRIFT — committed since (HEAD e6962895)**: four commits appended 40 lines (`git -C …andromedaV3 diff --stat 333e4934 HEAD -- …rulings.md`: `40 insertions`); line 699 still reads «**viola's fork CARRY 7/8** («Server verification before any frame»): «Вариант 1 — утвердить (Рекомендую)»», the fact the entry carries.
    - I3 · message, the founder's own word live by dialog, 2026-10-26T18:11Z, relayed by the overseer · copy message · n/a — the four P4 forks: Q1 readers move now (his boundary-widening ratification), Q2 no line on a refused check as link, Q3 link's instance pair for the six frame verbs, Q4 CARRYs 11 and 16 carved into «Pinned copy race and reproducible builds» right after this entry (`inputs#I3`).
    - I4 · message, the founder's own word live by dialog, 2026-10-26T18:59Z, relayed by the overseer · copy message · n/a — «yes» on `.andromeda/runs/2026-10-26T16-59-13-phase/p5-card-1.md`, the whole card (`inputs#I4`). Its `## Beyond the plan` rows, verbatim, with the word's authority (the founder's own word, relayed):
      | # | proposal | kind |
      |---|---|---|
      | 1 | Insert a new working-route entry «Pinned copy race and reproducible builds» right after «Home and code-bearing file integrity» in Epoch 6. It carries CARRY 16 (the `pin_exe` first-start race, chunk 2026-10-17-mcp-server-for-drivers) and CARRY 11 (two clean release builds of one tree compared by file and by section, chunk 2026-10-09-inner-cr-and-crlf-in-a-sent-text), each verbatim from this entry's line, on the founder's route-split word (`inputs#I3` Q4). | route insertion |
      | 2 | CARRY 17 (the `marketplace/` tree's integrity) closes with no work and no owner: the founder's Epoch 5 batch (2026-10-20) made «`marketplace/` off the integrity set» his word. | disposition (closed by ruling) |
    - I5 · `../additional/viola-overseer/wrap-home-integrity-directive.md` · copy no-repo · unchanged — snapped at this wrap (`inputs#I5`): forced sample at P2 (`detail.py sample --side wrap --force`, one classify draw), keep every transcript and the three run dirs, stop at a hard cascade failure, the P2-validated report items, the founder's words (`inputs#I3`, `inputs#I4`), and three outcome items to resolve in the masters: architecture.md:323 not naming the instance detail file; three behaviour changes not yet stated; `viola list`'s home-check inconsistency (`board.rs`) — give it an owner.
- **Reverted / negative API facts:** a check inside `client::start` and one inside `live_endpoint` (plan §Constraints) were never written; logging `strict-modes-failed` after a refused check was rejected by the founder (`inputs#I3` Q2). `pin_exe` was not re-signatured (it delegates to `pin_bytes`). none reverted in the tree.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - test-plan test-data-fixtures.md:10 and integration-test-strategy.md:33 — «the refused home's DACL was not read» (the seeded home refused by `hook statusline`'s instance check, CI run 38080061631): measured on `windows-2025` (ci#40398601594, job 121838141094, green case `strict::win::tests::strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`): a home made with `create_dir_all` before viola is refused `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE. Pattern `seed` (test-plan 2 rows of 5). The plan's step 11 «record once, in `evidence/`, the canonical SDDL of a plain `create_dir_all` home» was not met as written (Deviation 6): the reading is the named ACE, not the SDDL.
  - test-plan unit-test-strategy.md:25 («Not covered: a widened DACL on an instance tree on Windows») and architecture occupied-resources.md:113 («No case widens the DACL of an instance tree on Windows yet») — now covered by the `cfg(windows)` cases green on `windows-2025` (ci#40403389629). Patterns `check-instance` / `owner-title`.
  - Measured on `windows-2025` (ci#40398601594 and ci#40400887151, `evidence/operator-pass.md`): an inheritable `icacls` grant (`(OI)(CI)`) on a seeded home's `bin/` (or the home) propagates through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion, widening every seeded home until removed. No master states the opposite; test-plan §5 / §7's seed carve-out (pattern `seed`) owes the hazard.
- **Expected amendments (from plan):**
  - security-plan §Authentication & Authorization, §Input Validation, §Error Handling, §Security Anti-Patterns and Bootstrap phases — the eight dated gaps retired at every site, the read rule as built, the `conpty/` leg, the readers on the replay (pattern «Boundary widening», answered by the founder at P4, `inputs#I3`) · carried: Retired; Counts 1, 3, 5, 17, 18 · search: `owner-title` security-plan 15, `gap-until` security-plan 4, `read-rule` security-plan 4, `read-snapshot` security-plan 5, `stamps-err` security-plan 3, `conpty-dacl` security-plan 3, `check-instance` security-plan 11.
  - architecture §Occupied Resources (the checked readers, the `check_instance(` count by its rule, `read_snapshot`'s callers), §Standard Contracts (Snapshot envelope: the replay's product readers), §Conventions (exit 21 / exit 1 strict-modes refusals of the six verbs, `run`, `verify`), §Established Decisions ([PTY] the `strict-modes-failed` fallback) · carried: Symbols (remaining-caller facts); Counts 2, 5, 7, 8, 9, 12 · search: `check-instance` architecture 7 (occupied-resources.md:112 the count line), `read-snapshot` architecture 5, `sideload-fb` architecture 2, `instance-pair` architecture 2, `home-pair` architecture 2, `owner-title` architecture 2; the directive's (`inputs#I5`) architecture.md:323 item: `detail-file` 0 at :323 (its 3 architecture hits are :412, established-decisions.md:213, occupied-resources.md:111).
  - obs-plan §1 / §6 Vector 7 list and §10 exemptions (each new no-line site), §4 the `wait` / `last` «`strict-modes-failed` cannot occur» sentence, E5's `state-recovered` emitters, `sideload_fallback`'s new word (D-36 class, its Decisions Log entry) · carried: Counts 6, 7, 10, 11, 20 · search: `no-line` obs-plan 19, `cannot-occur` obs-plan 2, `state-recovered` obs-plan 13, `sideload-fb` obs-plan 3, `refuse-count` obs-plan 2, `owner-title` obs-plan 2.
  - test-plan §6 E5 (built, its signals restated), Security control negatives (single writers with the wrapper stopped, not SIGSTOP), §5 / §7 the seed carve-out (viola creates the home first), §4 the Windows `check_instance` case · carried: Counts 5, 15, 16, 19; Spec claims disproved · search: `read-snapshot` test-plan 5, `wheel-human` test-plan 3, `single-writer` test-plan 6, `seed` test-plan 5, `owner-title` test-plan 6, `check-instance` test-plan 4.
  - design-system and layout-templates — the exit-21 row and Block 2's fixed-form list gain the six frame verbs' instance pair · carried: Count 8 · search: `instance-pair` design-system 3, layout-templates 2; `home-pair` design-system 4, layout-templates 3.
  - a11y-plan §8 — the strict-modes witnesses on three legs · carried: Coverage of new surfaces (tests on three OSes) · search: `home-pair` a11y-plan 3, `cannot-occur` a11y-plan 2, `no-line` a11y-plan 1 (a11y-scope-summary.md:24).
  - Carried out of this chunk (not an amendment): the new route entry «Pinned copy race and reproducible builds» with CARRYs 11 and 16 verbatim (`inputs#I3` Q4, `inputs#I4` row 1) — P5's insertion; CARRY 17 closed by ruling (`inputs#I4` row 2) — P5.
  - The directive's three outcome items (`inputs#I5` item 6): (a) architecture.md:323 → Count 12; (b) the three behaviour changes not yet stated → Counts 13 (an unstattable home a strict-modes refusal), 23 (a `VIOLA_DIR` naming no directory writes nothing), 6 (the `state.snapshot_recover` span under `run.start` / `hook.handle`), with the `sideload_fallback` word (Count 7); (c) `viola list`'s home check → Count 14, its owner a P5 route pin (Outcome).
- **Coverage of new surfaces:**
  - `refuse_untrusted` at `send` / `wait` / `last` / `answer` / `pause` / `release` → validation strict-modes ✓ · instrumentation n/a (no line by the founder's word, `inputs#I3` Q2) · PII n/a (fixed pair, no path, `digit_runs` read) · tests integ `binary(security_negatives_strict_modes)` three OSes · a11y the instance pair, `hint:` last ✓ · tokens n/a
  - `viola hook` event/dialog instance check → validation ✓ · instrumentation n/a (fails open, no line) · PII n/a · tests integ `hook_fail_open`, `hook_statusline` (both OS forms) · a11y n/a · tokens n/a
  - `viola mcp` frame tools' check → validation ✓ · instrumentation n/a (no line) · PII n/a (`detail: null`) · tests integ `handlers` · a11y n/a · tokens n/a
  - `viola verify` / `viola run` / `revive` start tree check → validation ✓ · instrumentation n/a (no log opened) · PII n/a · tests integ `run_cli`, `cli_verify`, `cli_controls_not_disableable`, `security_negatives_strict_modes` (umask 000, Windows protected DACL, read-granted home) · a11y home / instance pair ✓ · tokens n/a
  - diagnostics opens (`open_role_file`, `write_detail`) → validation ✓ (symlink + modes) · instrumentation n/a (a failed open ends at `sink`) · PII n/a · tests unit (obs.rs 610-673) · a11y n/a · tokens n/a
  - `conpty/` owner/DACL leg → validation ✓ · instrumentation `process-start.sideload_fallback` `strict-modes-failed` ✓ · PII n/a · tests unit (pin.rs 453-473) + integ `conpty_sideload` on `windows-2025` · a11y n/a · tokens n/a
  - the four readers on the replay → validation ✓ (the check first) · instrumentation `state-recovered` ✓ · PII n/a · tests unit (liveness, hook, statusline, run) + integ `chaos_snapshot_replay` (E5 steps 1–3) · a11y n/a · tokens n/a

## Deviations from intent
1. Step 2: `update_stamps` runs `check_stamps` before `create_private_dir` as well as inside its lock — under the lock alone the check comes after the create, which narrows a widened `ledger/` to 0700 first (the CARRY 14 trap). The refusal is a new `StampsError` (`StrictModes` · `State`). Justified: the plan's own order rule (Implementation notes, «every check comes before the first … `create_private_dir`»).
2. Step 1: `Refused` gained `Readable` and `Symlink` so the read rule and the symlink check do not borrow `Writable`; `check_home_tree` returns `TreeRefused`; `check_present` serves step 10; `DIAGNOSTICS` is a constant. Justified: fixed messages that name the refused property.
3. Step 8: `pin_exe` keeps its signature and delegates to a new `pin_bytes(home, bytes)`; `pin::pin_key` names the key once. Justified: `verify` still calls `pin_exe`.
4. Step 5: `collision_check` takes the caller's one read (`Option<&Recovered>`); `revive`'s preflight reads once for both its collision check and its recorded-directory reading — a replay in each would write two `state-recovered` lines in one revive (`cli_revive`'s «one replay, one line»). Consequence: `state.snapshot_recover` now sits under `run.start` before `run.collision_check`, and under `hook.handle` on the hook path; two span-order oracles list it.
5. Step 4: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing (statusline already did); `hook_fail_open`'s cases that expect lines run in a viola-made instance dir (`Env::Made`), a `no_instance_dir` row pins the silence; the widened-instance rows are one function over the events rather than one row per `rstest` matrix.
6. Step 11: the plain home's canonical SDDL could not be recorded from a passing CI run; the `cfg(windows)` case asserts the refusal `Writable` and that the refused entry is an inherited `S-1-5-32-545` write ACE (a failure prints every ACE). CARRY 13's answer recorded: a home a test made first is one the product must refuse.
7. Step 13: (a) `send`'s unwidened control runs `last` (an unscripted fake agent's `send` waits out readiness past `WITHIN`); (b) the MCP tools driven over stdio with an `initialize` handshake; (d) the read-granted pre-existing home refusing `viola run` runs on every OS (Unix 0750, Windows a Users read grant); the protected-DACL read-back Windows-only (finally through `icacls /save`).
8. Step 14: `cli_verify`'s catch-site case split in two — a home under a regular file is now the home pair (`verify_under_a_regular_file_refuses_the_home`, per-OS oracle after CI: the catch-site line on Windows, where such a path stats `NotFound`), the catch-site line forced by a regular file where `ledger/` goes in a viola-made home.
9. Steps 12–16: `tests/support/home.rs` gains `widen` · `grant_read` · `unwiden` · `still_widened`; the G4 / G10 witnesses spawn through `support::cli::Running`, bounded by `WITHIN`, after a remove-the-guard hang.
10. After CI (fix commits `f65a4ae8`, `e3468500`): the per-OS oracle (8); the Windows `--home` DACL read back through `icacls /save`, `LA` taken for a user SID ending `-500`; `run_refuses_a_widened_home_tree_before_its_log` starts its home UNSEEDED (a seeded home's companions are hard links to one shared seed file, and `widen`'s inheritable grant reached every seeded home through them).

Scope record (`gate.py scope`: `scope: clean — changed 35 · listed 30 · recorded 5 (companion 5 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 548`), class companion, authority self:
- `tests/cli_revive.rs` · serves step 5 — the two `cwd-missing` cases age the beat past `STALE_AFTER` first (the replayed collision check reads a fresh beat as `already-live`).
- `tests/cli_wait_last.rs` · serves step 3 — the two catch-site cases hold a viola-made home with a regular file where `diagnostics/` goes (a regular-file or 0755 home is now refused by the frame verbs' check).
- `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs`, `tests/tui_env_strip.rs` · serve step 1 — each made its home with `create_dir_all` (0755), which the read rule now refuses; each makes it through `viola_state::fs::create_private_dir`.

## Decisions & corrections
- The founder's own answers at P4 (`inputs#I3`, 2026-10-26T18:11Z, relayed): the four readers move onto `read_snapshot_or_replay` now (his boundary-widening ratification); a refused check at every new entry point writes no line, as `link` / `unlink`; the six frame verbs print `link`'s instance pair; CARRYs 11 and 16 carved into «Pinned copy race and reproducible builds» right after this entry.
- The founder's P5 «yes» on the whole card, both Beyond-the-plan rows included (`inputs#I4`, 18:59Z, relayed).
- The wrap directive (`inputs#I5`, the overseer, founder-delegated): forced sample at P2; keep evidence; stop at a hard cascade failure; the three outcome items to resolve in the masters.
- The overseer's technical answers during implement (20:19Z, 20:27Z; journal): kill only the hung test pids; stop the controls script before G10; bound G4 and G10's witnesses; kill only processes whose `/proc/<pid>/exe` and home path prove them this run's, never viola-builder's wrapper or anything under `~/.viola`.
- Correction (implement): a status named two hung pids from a listing cut at 30 rows (`| head -30`); two more were hidden — a recurrence of «never read a result summary through truncation».
- Correction (implement): the controls script stopped inside G10, not before it.
- Measured: a `/proc/<pid>/exe` reading `… (deleted)` after a rebuild was matched with that suffix removed.
- Guard refusals (implement): `stamp-ahead.py` refused a guessed journal time (19:45Z written, clock 19:30Z) — corrected from `date -u`; `bash-guard.sh` refused two `cat` heredocs with a file target and one session-moving `cd`, each re-issued in the allowed form. (This wrap: `bash-guard.sh` refused two session-moving `cd`s and one tool output piped into a filter, each re-issued bare.)
- Measured on `windows-2025`: an inheritable `icacls` grant on a seeded home's `bin/` (or the home) propagates through `seed_conpty`'s hard links into the shared seed companion, widening every seeded home until removed; a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded.
- Measured on `windows-2025`: canonical SDDL spells the runner's user (local Administrator, RID 500) `LA`; a SID oracle against it fails.
- Measured on `windows-2025`: `Get-Acl` printed nothing for the protected DACL (stderr not captured); `icacls <home> /save <file>` read it back.
- Measured on Windows: a path under a regular file stats `NotFound` (testing.md 2026-09-24 recurring), so a strict-modes check skips it as absent — a per-OS oracle is owed wherever a test plants a regular file above a checked path.
- Remove-the-guard rule (testing.md 2026-09-25) applied: a control whose neutralised guard lets a product process start needs a bounded witness (measured: a 27-minute hang under nextest's default profile, which has no kill line).
- Sweep hazard (this wrap): `git grep -c 'human::refuse(' {rev}` counts the literal `(` while `grep -rcE 'human::refuse\(' src` needs the escape — both give 15 at the base; and `grep -rcE` prints per-file counts including zeros, so a sum needs the `:0` lines dropped.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (security) six frame verbs refuse a widened instance, exit 21, the instance pair / the `instance-unreachable` document, no frame, no line, mode or ACL as found, three OSes — MET: `refuse_untrusted` at each verb's entry (Symbols); `binary(security_negatives_strict_modes)` green here (firing 3) and on all three CI legs (ci#40403389629).
- (security) `viola hook` event/dialog refused by the instance check: exit 0, empty streams under 1.0 s, no frame, no line, no SessionEnd; capture arms unchanged — MET: hook.rs `canonical`/`trusted` before obs init; `hook_fail_open | hook_statusline` green.
- (security) `viola mcp` frame tools → `instance-unreachable`, `detail: null`, no frame, no line — MET: tools.rs 337-339; `binary(handlers)` green.
- (security) `run` refuses a 0770 home, widened `instances/<name>`, `bin/` or pinned copy, exit 1 with its pair before any log; `verify` refuses a widened `ledger/` (exit 1, home pair, no step line, `stamps.json` unchanged); `update_stamps` refuses before reading — MET: `checked_exe` / `refuse_tree`, verify.rs 64-70, stamps.rs; the entry's four binaries and the unit tier green.
- (security) the Unix read rule and the Windows read-grant rule refuse on their unit tables; on `windows-2025` an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes — MET (unit tier here; the `cfg(windows)` cases green on ci#40398601594's and ci#40403389629's Windows leg).
- (security) a `conpty/` companion its owner/DACL check refuses leaves the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, exit unchanged — MET on `windows-2025` (`run_on_a_companion_another_user_may_write_falls_back_to_conhost`; kept homes: exactly one `strict-modes-failed`, that test's).
- (tests) single writers (byte-identical after every other writer-capable entry point, the wrapper stopped), umask-independent modes (Unix), the Windows `--home` outside the profile (protected DACL read back; a pre-existing Users-read dir refused exit 1) — MET (`binary(security_negatives_strict_modes)`, three legs; the DACL read `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`).
- (tests) E5 steps 1–3 and the replayed readers — MET (`binary(chaos_snapshot_replay)` + unit tier).
- (tests) `seed_conpty` homes pass `check_instance` on `windows-2025` and the three `hook_statusline` cases pass; the plain home's SDDL recorded in `evidence/` — PARTLY: the seed and the three cases MET (green Windows leg); the SDDL itself was not recorded (Deviation 6) — the refusal and the refused ACE are asserted by a green `cfg(windows)` case instead. Unlinked criterion; raised for P2 (test-plan §5 / §7's wording owes the reading as measured).
- (obs) every refused check writes no line; `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with the check in the timed path — MET (entry `run --perf` green: max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms).
- (capability) `v1-45`, `v1-17`, `v1-41` — MET by the entries above; `implemented` with refs (implement's `matrix-*.json`); P7 flips them. `v1-17`'s by-construction clause leans on architecture §Occupied Resources naming the detail file (Count 12) — P2.
- (tests) `pre-push` `"ok":true` and CI `verdict: green` on all three OSes — MET (pre-push re-fired green before each push; ci#40403389629 green 15/15).

Gates (the plan's `[[gate]]` block; implement firing 3, 2026-10-26T20:36Z: `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3`; trail `.andromeda/runs/2026-10-26T19-02-07-implement/gate-{marker}.json`):
- `df -h /tmp` — recorded (`tmpfs 32G 7.4G 24G 24% /tmp`).
- `cargo fmt --all --check` · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` · `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc …` · the G1 `rg` instrument grep · `bash scripts/lint-probes.sh` — green.
- `bash scripts/agent-run.sh run --unit` — green.
- `cargo nextest … binary(security_negatives_strict_modes)` · `… binary(chaos_snapshot_replay)` · `… binary(hook_fail_open) | binary(hook_statusline)` · `… binary(run_cli) | binary(cli_verify) | binary(conpty_sideload) | binary(cli_controls_not_disableable)` · `… binary(cli_links) | … | binary(cli_revive)` · `cargo nextest run -p viola-mcp -E 'binary(handlers)'` — green.
- `cargo deny check` — green.
- `gate.py guard …` — green (`guard: clean`; allowed 30, companions excluded 5).
- smoke: `agent-run.sh cleanup` · `boot --session p-hci-smoke --instance builder` · `status` · `g2-zero-panics.sh` · `schema-check` · `cleanup` — green (the boot path changed; P3 smoke by hand also green).
- `bash scripts/agent-run.sh run --perf` — green (`perf passed 7, failed 0`).
- `bash scripts/agent-run.sh pre-push` — green (`"ok":true` at `linux-tests`, `passed 3220, failed 0`).
- `gate.py hygiene` (leg operator) — recorded in `evidence/operator-pass.md`: `hygiene: clean` before each push.
- `git … push origin HEAD` (leg operator) — recorded: three pushes, `history moved` to `af6c67ef`, `f65a4ae8`, `e3468500`.
- `ci.py conclusion --sha HEAD --wait 2400` (leg operator) — recorded on the final HEAD `e3468500`: `verdict: green · checks 15/15` (ci#40403389629); the two earlier runs red on this chunk's own test code, fixed by the two fix commits (the product checks read as specified on every leg).

Watches: none (the entry's line folds no `watch:`).

Outcome basis: the operator pass ran (Setup 4: pre-CI `af6c67ef`, fixes `f65a4ae8`, `e3468500`); the gate verdicts above rest on its final state — implement's firing 3 on the pre-CI tree, re-fired `pre-push` and `hygiene` before each push, and the final HEAD's CI run recorded in `evidence/operator-pass.md`. Implement's persisted report: `.andromeda/runs/2026-10-26T19-02-07-implement/outcome-{marker}.md` (the only `*-implement/` dir for this marker), read whole; its Surfaced items 1–3 are Counts 12, 13 / 23 / 6 / 7 and 14 above, item 4 (Windows-only unmeasured here) is closed by the green Windows leg.

Route asks for P5 (the directive `inputs#I5` and the card `inputs#I4`): insert «Pinned copy race and reproducible builds» right after this entry with CARRYs 11 and 16 verbatim; CARRY 17 closed by ruling; an owner for `viola list`'s home-check inconsistency (Count 14) — a CARRY on the markerless entry «Unix endpoint and home hardening» (the home's checks), unless P2's halt names another.

Process hygiene: implement P4's census (outcome file) — every process its runs started terminated (table there). Re-measured here (`gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --args projects/viola/target --args viola-e2e-home-1000`): `census: 7 running`, none this repository's — four `viola run` wrappers from `~/dev/projects/additional/viola-lab/prototype/target/debug/viola`, one `viola hook` under one of them, the bridge's `~/.viola/bin/…/viola mcp`, and `~/.local/viola-product/bin/viola wait viola-builder`.
## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 99df4536 (the parent of the oldest pre-CI commit af6c67ef) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-mcp/src/tools.rs — added 12 line(s) in 4 range(s)
added: 4-7 · 30 · 322-325 · 337-339
  - 337-339 «if check_instance(&ctx.home, &instance_dir).is_err() {»
### crates/viola-mcp/tests/handlers.rs — added 75 line(s) in 1 range(s)
added: 404-478
- 405-434 @407 «fn widened(dir: &Path, grant: bool) {»
  - 408-413 @409 «{»
  - 414-433 @415 «{»
- 436-478 @441 «fn handlers_frame_tools_refuse_a_widened_instance_before_any_frame() {»
  - 442-448 «let wrapper = booted(»
  - 452-468 «for (name, args) in [»
  - 470-473 «assert!(»
### crates/viola-state/src/liveness.rs — added 47 line(s) in 4 range(s)
added: 12-13 · 87-90 · 92-94 · 255-292
  - 92-94 «let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {»
  - 255-291 @261 «fn live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line(»
### crates/viola-state/src/pin.rs — added 51 line(s) in 10 range(s)
added: 21-22 · 71-75 · 79-86 · 94 · 164-165 · 177 · 183 · 210-214 · 453-474 · 575-578
- 71-74 @72 «pub fn pin_key(bytes: &[u8]) -> String {»
  - 210-213 @211 «fn pin_key_is_the_version_and_the_content_key() {»
  - 453-473 @459 «fn pin_companions_refuses_a_path_another_user_may_write(#[case] rel: &str) {»
### crates/viola-state/src/stamps.rs — added 66 line(s) in 6 range(s)
added: 37-51 · 53-55 · 60-61 · 66 · 70 · 283-326
- 37-44 @39 «pub enum StampsError {»
- 46-50 «impl From<io::Error> for StampsError {»
  - 47-49 «fn from(error: io::Error) -> Self {»
  - 283-313 @287 «fn update_stamps_refuses_a_widened_ledger_and_leaves_it() {»
  - 315-325 @316 «fn stamps_error_messages_are_fixed() {»
### crates/viola-state/src/strict.rs — added 601 line(s) in 24 range(s)
added: 3-6 · 11-12 · 24-27 · 36-60 · 65 · 68-71 · 74-79 · 83-150 · 152 · 160-172 · 181-194 · 209-218 · 225-229 · 279-297
       334 · 349-352 · 356 · 521-530 · 553-557 · 573 · 708-830 · 946-947 · 960-961 · 1110-1385
- 36-41 @38 «enum Rule {»
- 83-89 @86 «pub enum TreeRefused {»
- 91-125 @95 «pub fn check_home_tree(»
  - 101-104 «if let Some(name) = name {»
  - 105-109 «let mut code = vec![»
  - 110-123 «if let Some(key) = key {»
- 127-145 @130 «pub fn check_diagnostics(path: &Path) -> Result<(), Refused> {»
  - 131-135 «let meta = match std::fs::symlink_metadata(path) {»
  - 136-138 «if meta.file_type().is_symlink() {»
  - 139-143 «let rule = if meta.is_dir() {»
- 160-163 @161 «pub fn check_present(path: &Path) -> Result<(), Refused> {»
- 165-170 «fn check_rule(path: &Path, rule: Rule) -> Result<(), Refused> {»
  - 166-169 «match rule {»
  - 187-191 @188 «{»
- 209-217 @211 «pub fn unix_read_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> {»
  - 213-215 «if mode & 0o077 != 0 {»
- 279-296 @281 «pub fn windows_read_verdict(»
  - 288-294 «if dacl»
  - 1111-1128 @1121 «fn unix_read_verdict_needs_the_write_rule_and_no_foreign_bit(»
  - 1134-1160 @1143 «fn windows_read_verdict_refuses_any_read_right_to_another_sid(»
  - 1162-1189 @1165 «fn windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule() {»
  - 1191-1205 @1193 «fn home_tree_viola_wrote(root: &Path) -> (PathBuf, PathBuf, String) {»
  - 1207-1209 «fn builder() -> ViolaName {»
  - 1211-1221 @1212 «fn check_home_tree_of_a_tree_viola_wrote_passes() {»
  - 1272-1297 @1273 «fn check_home_tree_refuses_each_widened_path(»
  - 1299-1313 @1300 «fn check_diagnostics_of_what_viola_wrote_or_of_nothing_passes() {»
  - 1315-1335 @1321 «fn check_diagnostics_refuses_a_widened_mode(»
  - 1337-1354 @1341 «fn check_diagnostics_refuses_a_symlinked_role_file_and_folder() {»
  - 1356-1385 @1366 «fn strict_instance_holds_the_content_folders_to_the_read_rule(»
### schemas/diag-line.v1.json — added 1 line(s) in 1 range(s)
added: 70
### src/cmd/answer.rs — added 7 line(s) in 2 range(s)
added: 5-8 · 94-96
  - 94-96 «if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {»
### src/cmd/client.rs — added 30 line(s) in 3 range(s)
added: 1-6 · 16 · 20-42
- 20-41 @24 «pub(crate) fn refuse_untrusted(»
  - 31-33 «if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {»
  - 34-39 «if json {»
### src/cmd/hook.rs — added 100 line(s) in 8 range(s)
added: 30-31 · 75-87 · 112-116 · 220-222 · 224-226 · 624-695 · 1086 · 1089
- 75-81 @77 «fn canonical(instance: &Instance) -> Option<Instance> {»
- 83-86 @84 «fn trusted(instance: &Instance) -> bool {»
  - 224-226 «let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {»
  - 628-638 «impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Recorded {»
  - 640-694 @647 «fn connect_over_a_replayed_snapshot_is_unreachable(#[case] fault: &str, #[case] detail: &str) {»
### src/cmd/hook/statusline.rs — added 41 line(s) in 6 range(s)
added: 20 · 22 · 128-132 · 305-306 · 313 · 524-554
  - 128-132 @129 «let command = match read_snapshot_or_replay(&instance.dir) {»
  - 524-553 @530 «fn statusline_over_a_replayed_snapshot_runs_nothing(#[case] fault: &str, #[case] detail: &str) {»
### src/cmd/last.rs — added 3 line(s) in 1 range(s)
added: 30-32
  - 30-32 «if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {»
### src/cmd/link.rs — added 2 line(s) in 1 range(s)
added: 94-95
### src/cmd/pause.rs — added 7 line(s) in 2 range(s)
added: 3-6 · 32-34
  - 32-34 «if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {»
### src/cmd/release.rs — added 3 line(s) in 1 range(s)
added: 35-37
  - 35-37 «if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {»
### src/cmd/revive.rs — added 24 line(s) in 6 range(s)
added: 18-21 · 125-135 · 138 · 196-198 · 200 · 202-205
- 18-21 «use super::run::{»
  - 125-135 «let exe = match checked_exe(home, &args.name)? {»
### src/cmd/run.rs — added 153 line(s) in 17 range(s)
added: 21-25 · 166-172 · 183 · 189-216 · 260 · 277-278 · 281 · 414-415 · 421 · 424-432 · 447-448 · 450-451 · 510
       1161-1225 · 1278-1281 · 1654-1656 · 1673-1691
  - 166-172 «let exe = match checked_exe(home, &args.name)? {»
- 189-200 @193 «pub(super) fn checked_exe(»
- 202-215 @204 «pub(super) fn refuse_tree(name: &ViolaName, refused: TreeRefused) {»
  - 205-214 «match refused {»
  - 425-432 «let taken = match recovered {»
  - 1165-1171 «impl<S: tracing::Subscriber> Layer<S> for Events {»
  - 1173-1224 @1182 «fn collision_check_over_a_replayed_snapshot_reads_the_beat_alone(»
### src/cmd/send.rs — added 3 line(s) in 1 range(s)
added: 190-192
  - 190-192 «if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {»
### src/cmd/verify.rs — added 25 line(s) in 5 range(s)
added: 36-37 · 64-70 · 95-100 · 278 · 288-296
  - 64-70 @67 «if check_home_tree(&home, None, None).is_err() {»
- 95-99 @96 «fn refuse_home() {»
  - 289-296 «match written {»
### src/cmd/wait.rs — added 3 line(s) in 1 range(s)
added: 43-45
  - 43-45 «if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {»
### src/obs.rs — added 83 line(s) in 6 range(s)
added: 69-79 · 88-90 · 92 · 321-322 · 328 · 610-674
- 69-76 @71 «fn check_diagnostics(dir: &Path, file: &Path) -> io::Result<()> {»
  - 72-75 «[dir, file]»
  - 610-641 @618 «fn open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it(»
  - 643-654 @645 «fn open_role_file_refuses_a_symlinked_role_file() {»
  - 656-673 @660 «fn write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it() {»
### tests/chaos_snapshot_replay.rs — new file · 147 line(s)
- 20-22 «use support::home::{»
- 28-51 @30 «fn first_life(stamped: StampedHome) -> (StampedHome, Value) {»
  - 36-41 «let linked = viola(»
- 53-65 @54 «fn fault(dir: &Path, fault: &str) {»
  - 57-63 «let faulted = match fault {»
- 67-76 @68 «fn age_beat(dir: &Path, age: Duration) -> Duration {»
  - 69-74 «std::fs::OpenOptions::new()»
- 78-83 «fn recovered(home: &Path) -> Vec<Value> {»
  - 79-82 «support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))»
- 85-110 @88 «fn a_start_over_an_unreadable_snapshot_replays_the_log(»
  - 105-108 «assert_eq!(»
- 112-147 @113 «fn a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live(»
  - 122-128 «command»
  - 129-131 «for name in UNWRAPPED {»
  - 136-140 «let exits: Vec<Value> =»
  - 141-144 «assert_eq!(»
### tests/cli_controls_not_disableable.rs — added 73 line(s) in 5 range(s)
added: 71-73 · 78 · 425 · 576-642 · 665
- 576-641 @582 «fn setting_does_not_disable_the_home_check() {»
  - 584-640 «for (setting, _) in SETTINGS {»
### tests/cli_machine_contract.rs — added 1 line(s) in 1 range(s)
added: 542
### tests/cli_revive.rs — added 13 line(s) in 3 range(s)
added: 146-156 · 266 · 649
- 146-155 @148 «fn age_beat(home: &TestHome) {»
  - 149-154 «OpenOptions::new()»
### tests/cli_verify.rs — added 59 line(s) in 4 range(s)
added: 251-281 · 426-429 · 431 · 438-460
- 251-280 @255 «fn verify_refuses_a_widened_ledger_before_any_step(#[from(home)] home: TestHome) {»
  - 265-272 «assert_eq!(»
  - 273-276 «assert_eq!(»
  - 438-446 «let expected = if cfg!(windows) {»
  - 455-456 «viola_state::fs::replace_private(&home.path().join("ledger"), b"x", 0o600)»
### tests/cli_wait_last.rs — added 6 line(s) in 4 range(s)
added: 443-444 · 448-449 · 455 · 469
### tests/conpty_sideload.rs — added 33 line(s) in 1 range(s)
added: 295-327
- 295-326 @300 «fn run_on_a_companion_another_user_may_write_falls_back_to_conhost(stamped_home: StampedHome) {»
  - 313-315 «for literal in VIOLA_LITERALS {»
  - 316-319 «assert!(»
  - 320-323 «assert_eq!(»
### tests/contract_diag_schema.rs — added 1 line(s) in 1 range(s)
added: 265
### tests/hook_fail_open.rs — added 81 line(s) in 15 range(s)
added: 145-146 · 148-149 · 282-283 · 290-291 · 308 · 314-316 · 320 · 400-401 · 404 · 429 · 435-437 · 466-520 · 675-676
       746-747 · 819-820
  - 314-316 «if matches!(env, Env::Made) {»
  - 435-437 «if matches!(env, Env::Made) {»
- 466-475 @467 «fn widened_payload(event: &str) -> Vec<u8> {»
  - 468-474 «match event {»
- 477-519 @487 «fn hook_refused_by_the_instance_check_fails_open_and_writes_nothing(#[case] event: &str) {»
  - 488-493 «let wrapper = Wrapper::boot(»
  - 501-505 «let out = run_hook(»
  - 510-513 «assert!(»
### tests/hook_statusline.rs — added 20 line(s) in 1 range(s)
added: 283-302
- 283-301 @287 «fn hook_statusline_a_widened_instance_runs_and_logs_nothing(stamped_home: StampedHome) {»
### tests/run_cli.rs — added 81 line(s) in 4 range(s)
added: 225 · 392-393 · 395 · 599-675
- 599-602 «const HOME_PAIR: &str = concat!(»
- 603-606 «const INSTANCE_PAIR: &str = concat!(»
- 608-617 @609 «fn pinned_copy(home: &Path) -> PathBuf {»
  - 610-613 «let mut keys: Vec<PathBuf> = std::fs::read_dir(home.join("bin"))»
  - 615-616 «keys.remove(0)»
- 619-674 @628 «fn run_refuses_a_widened_home_tree_before_its_log(»
  - 634-641 @638 «assert_eq!(»
  - 643-647 «let path = match rel {»
  - 650-664 @652 «let child = Command::new(VIOLA)»
### tests/security_negatives_strict_modes.rs — new file · 475 line(s)
- 23-26 «use support::home::{»
- 30-33 «const INSTANCE_PAIR: &str = concat!(»
- 34-37 «const HOME_PAIR: &str = concat!(»
- 39-57 @40 «fn verb(name: &str, json: bool) -> (Vec<&'static str>, Option<&'static str>) {»
  - 41-52 «let (mut args, stdin): (Vec<&'static str>, _) = match name {»
  - 53-55 «if json {»
- 59-64 @60 «fn digit_runs(text: &str) -> Vec<&str> {»
  - 61-63 «text.split(|c: char| !c.is_ascii_digit())»
- 66-68 «fn cli_lines(home: &Path) -> Vec<Value> {»
- 70-131 @82 «fn frame_verb_refuses_a_widened_instance_before_any_log(»
  - 93-109 «for json in [false, true] {»
  - 111-115 «assert_eq!(»
  - 116-120 «assert_eq!(»
- 133-145 @134 «const HOOKS: [(&str, &str); 10] = [»
- 147-159 «fn hook_payload(event: &str) -> Vec<u8> {»
  - 148-156 «match event {»
- 161-180 @162 «fn hook(dir: &Path, event: &str, payload: Vec<u8>) -> Option<i32> {»
  - 163-172 «let mut child = Command::new(VIOLA)»
  - 174-176 «let writer = std::thread::spawn(move || {»
- 182-244 @184 «fn mcp_calls(home: &Path, bin: &Path) -> Vec<Value> {»
  - 186-193 «command»
  - 194-196 «for name in UNWRAPPED {»
  - 198-205 «let calls = [»
  - 206-211 «let mut frames = vec![»
  - 212-214 «frames.extend(calls.iter().enumerate().map(»
  - 216-220 «for frame in &frames {»
  - 223-230 «std::thread::spawn(move || {»
  - 233-240 «while answers.len() < calls.len() {»
- 246-311 @251 «fn only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps(stamped_home: StampedHome) {»
  - 256-259 «let files = [»
  - 260-263 «let before: Vec<Vec<u8>> = files»
  - 265-267 «for (event, name) in HOOKS {»
  - 268-287 «let ran: Vec<(&str, Ran)> = [»
  - 291-293 «for (verb, out) in &ran[1..] {»
  - 296-301 «for answer in &answers[..4] {»
  - 304-310 «for (file, bytes) in files.iter().zip(&before) {»
- 313-326 @315 «fn walk(dir: &Path) -> Vec<std::path::PathBuf> {»
  - 317-324 «for entry in std::fs::read_dir(dir).expect("read dir") {»
- 328-378 @333 «fn a_start_under_umask_000_makes_every_home_path_owner_only(»
  - 339-347 «command»
  - 348-350 «for name in UNWRAPPED {»
  - 355-361 «let pinned = |path: &Path| {»
  - 362-375 «let wrong: Vec<(String, u32)> = paths»
- 380-404 @384 «fn a_home_another_user_may_read_refuses_run(#[from(support::home::home)] tmp: TestHome) {»
  - 389-393 «command»
  - 394-396 «for name in UNWRAPPED {»
- 406-475 @412 «fn a_home_viola_makes_outside_the_profile_takes_the_protected_dacl(stamped_home: StampedHome) {»
  - 416-420 «let icacls = Command::new(»
  - 426-431 «assert!(»
  - 433-438 «let units: Vec<u16> = bytes»
  - 440-445 «let sddl = text»
  - 446-449 «let whoami = Command::new("whoami")»
  - 451-457 «let user = whoami»
  - 458-461 «let dacl = sddl»
  - 464-469 @465 «let trustee = if user.ends_with("-500") {»
  - 470-474 «assert_eq!(»
### tests/support/home.rs — added 83 line(s) in 6 range(s)
added: 271-273 · 287-289 · 291 · 303 · 305 · 317-390
- 317-329 @319 «fn icacls(path: &Path, args: &[&str]) {»
  - 321-327 «let status =»
- 331-350 @334 «pub fn widen(path: &Path) {»
  - 335-340 @336 «{»
  - 341-349 @342 «{»
- 352-362 @354 «pub fn grant_read(path: &Path) {»
  - 355-359 @356 «{»
- 364-374 @365 «pub fn unwiden(path: &Path) {»
  - 366-371 @367 «{»
- 376-389 @378 «pub fn still_widened(path: &Path) -> bool {»
  - 379-384 @380 «{»
  - 385-388 @386 «{»
### tests/tui_env_strip.rs — added 1 line(s) in 1 range(s)
added: 145
