# Codebase Research — 2026-10-26-home-and-code-bearing-file-integrity

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 14 · one code-graph query (`.andromeda/runs/2026-10-26T16-59-13-phase/tree-query-2026-10-26-home-and-code-bearing-file-integrity.json`, plane rust; graph lines are 0-indexed, cited here +1)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 18 Session Additions applied (the e2e-home tmpfs backing and its headroom floor, the `binary(<stem>)` filter form, the 2026-10-09 build-hash rule for CARRY 11, the 2026-10-11 `schema-check` reads kept homes only); `.claude/rules/testing.md` (auto-loaded) — the stamped-home seed rule, the remove-the-guard rule, the OS-gate rules
- **Platform issues consulted:** none — no runner-only bullet: the one CI run Setup read (ci#40363908098 at `99df45362ac6`) closed green at P3
- **External inputs:** `inputs#I1` — the operator's directive for this phase (forced sample, two-draw classify, evidence kept, the founder's forks at P4, no epoch boundary, `df -h /tmp` before a stamped-home gate) · `inputs#I2` — rulings.md:699, the founder's option 1 for fork CARRY 7/8 and the readers' move re-pinned here · `inputs#I3` — the founder's own answers to the four P4 forks (2026-10-26T18:11Z): the readers move now, no line on a refused check, `link`'s instance pair on the six frame verbs, CARRYs 11 and 16 carved into a new entry

## Files inspected
- `crates/viola-state/src/strict.rs` (1–200) — `check_stamps`, `check_instance` (home, `instances/`, the instance dir, `snapshot.json`, `events.ndjson`, each that exists), `check_path`, `unix_verdict` (refuses `uid != euid` and `mode & 0o022`), `windows_verdict` (owner user or Administrators; NULL DACL, non-persistent volume, a write right to an untrusted SID refused). No `0o077` read rule exists in code.
- `src/cmd/client.rs` (1–140) — `start` opens the `cli` role file through `obs::viola_obs_init` (line 22); `connect` runs the server check after `live_endpoint`.
- `src/cmd/link.rs` (88–136) — `link_call`: `check_instance` on the absolute home and `instances/<driven>` before `client::start` (line 98), refusal exit 21 with the instance pair or the `instance-unreachable` document, then `client::start` (107), `live_endpoint` (113), the request.
- `src/cmd/send.rs` (181–235), `wait.rs`, `last.rs`, `answer.rs`, `pause.rs`, `release.rs` (entry lines) — each calls `client::start` first (send.rs:190, wait.rs:43, last.rs:30, answer.rs:94, pause.rs:32, release.rs:35); `send`'s text refusals (`deliver`, line 211) and `answer`'s (line 107) run after it and log a client-side line.
- `src/cmd/hook.rs` (74–218) — `hook`: the capture arms first (77–93), `instance_of` (94), then `obs::viola_obs_init` (104) BEFORE any check; `connect` (203) reads `read_snapshot(&instance.dir)` with no strict-modes check; SessionEnd's fallback `append_session_end` (176) appends to `events.ndjson` when no frame was written.
- `src/cmd/hook/statusline.rs` (30–69) — the template: `canonical` (37, `fs::canonicalize` then `instance_of`) and `trusted` (44, `check_instance`) before `viola_obs_init` (60); a refused instance returns exit 0 at once.
- `crates/viola-state/src/liveness.rs` (60–100) — `live_endpoint(instance_dir)` reads `read_snapshot` once (90) and hands `endpoint`, `pid`, `started_at` on; it takes no home.
- `crates/viola-mcp/src/tools.rs` (300–369) — `frame` (323): `live_endpoint` (334) → `TokioClient::connect` → `server_verified`; no instance check; `ctx.home` is available.
- `src/cmd/mcp.rs` (20–55) — `home_passes` (`check_path` on the home and `diagnostics/`) before its log; exit 1, nothing printed.
- `src/cmd/run.rs` (163–499) — `run` calls `open_wrapper_log` (164) FIRST, then `start`: resolution, `canonical_home` (237), `collision_check` (239, `read_snapshot` at 383), `pin_and_plugin` (242; `pin_exe` 406, the plugin files through `replace_private_shared` 419), Windows `conpty_sideload` (247), version gate, `bind_endpoint`, the log passes (`wait.rebuild` 282, `links.restore` 288). No strict-modes check of the home, `instances/<name>/`, `bin/`, `plugin/` or `ledger/` exists on `run`'s path today.
- `src/cmd/revive.rs` (112–138) — `trusted` (`check_instance`) before `open_wrapper_log` (122), then `preflight` and `run`'s `start`: a revived start already passes the instance check, and gets the new `bin/` / `plugin/` checks only if they live in `start` or in a helper both callers run.
- `src/cmd/verify.rs` (55–80) — with a `VIOLA_NAME` it opens the `cli` log (65) before anything; `measure` calls `pin_exe` (157) and `update_stamps` (264); no strict-modes check anywhere on `verify`'s path.
- `crates/viola-state/src/pin.rs` (40–188) — `pin_exe`: `create_private_dir(bin/<key>)`, then on `NotFound` `replace_private_shared(path, bytes, 0700)`; an existing copy re-hashed, a mismatch `HashMismatch`. `pin_companions` (Windows): write-if-absent, held `FILE_SHARE_READ`, whole SHA-256; no owner/DACL check.
- `crates/viola-state/src/fs.rs` (255–315) — `replace_private_retrying`: temp file, mode, write, `sync_all`, then `tmp.persist(path)` — the rename happens while the temp file's write handle is still open (it is dropped only after `persist` returns); `replace_private_shared` counts a failed replace as done over identical bytes.
- `src/obs.rs` (38–80, 276–321) — `open_role_file` calls `ensure_private_dir(home)` and `ensure_private_dir(diagnostics)` (each narrows an existing dir to 0700 on Unix) and opens the file, with no check; `write_detail` (310) does the same for `instances/<name>/diagnostics/detail-<process>.ndjson`. On Windows neither reads the owner or DACL before opening.
- `tests/cli_controls_not_disableable.rs` (420–575) — `SETTINGS` holds ten rows; the `--home` row points at `cli_list::list_refuses_a_home_another_user_could_write`, and `viola_dir_does_not_disable_the_home_scan` (`cfg(unix)`) sets a home 0770 and reads `list`'s exit 21. No frame-verb 0770 `--home` row stands.
- `tests/support/home.rs` (272–310) — `seed_conpty` creates `<home>/bin/<key>/conpty` with `fs::create_dir_all` (the home's first creation, inheriting the runner drive's ACL) and hard-links the companions from `target/conpty-seed/<key>/`, itself made by `create_dir_all`.
- `.andromeda/security-plan-detail/authentication-authorization.md` (line 38) — the master's Unix read rule: refuse when the home or `instances/<name>/` grants any group or other permission (`mode & 0o077 != 0`).
- `.andromeda/security-plan-amendments.md` (805–835) — the founder's Epoch 5 batch (2026-10-20): every Epoch 5 delegate mark is his, the "`marketplace/` off the integrity set" of 2026-10-17-optional-plugin-install among them; the refused checks' no-line shapes accepted as built.

## Graph impact (from the code-graph query)
- **`client::start`** (src/cmd/client.rs) — 10 product callers: answer.rs:94, last.rs:30, link.rs:107, list.rs:50, pause.rs:32, plugin.rs:62, release.rs:35, revive.rs:245, send.rs:190, wait.rs:43 — a check placed inside `start` would reach `list`, `plugin install` and `revive --list`, which run their own checks first; a check placed at each frame verb's entry reaches the six alone.
- **`live_endpoint`** (crates/viola-state/src/liveness.rs:89) — 7 product callers: answer.rs:119, last.rs:38, link.rs:113, pause.rs:64 (`wheel_call`, `release` through it), send.rs:218, wait.rs:51, viola-mcp tools.rs:335 (`frame`) — it takes no home, so a check inside it needs a signature change across all seven.
- **`read_snapshot`** — 4 product callers: liveness.rs:90 (`live_endpoint`), hook.rs:204 (`connect`), hook/statusline.rs:142 (`serve`), run.rs:383 (`collision_check`) — re-derived by the rule `read_snapshot(` under `src/` outside test modules (3) plus `live_endpoint`; the readers CARRY 19 names.
- **`viola_obs_init`** — 6 product callers: client.rs:22, hook.rs:104, hook/statusline.rs:60, mcp.rs:27, run.rs:184 (`open_wrapper_log`), verify.rs:65 — every role-file open the new checks must precede.
- **`update_stamps`** — 1 product caller: verify.rs:264 (`measure`); **`pin_exe`** — 2: run.rs:406 (`pin_and_plugin`), verify.rs:157 (`measure`); **`open_role_file`** — 1: obs.rs:129 (`viola_obs_init`).
- **`check_instance`** — 3 sites under `src/` (re-derived: `grep -rn 'check_instance(' src`: hook/statusline.rs:45, revive.rs:137, link.rs:98) plus `viola-state`'s `board::scan` (board.rs:94).

## Patterns detected
- **Check before the role-file open** (src/cmd/link.rs:95–106, src/cmd/hook/statusline.rs:53–58, src/cmd/revive.rs:116–122, src/cmd/mcp.rs:20–24): the check runs on the absolute or canonical home before `viola_obs_init`; a refusal opens nothing and writes no line. Every new entry point takes this order.
- **Strict stamps read** (src/run/version_gate.rs:172, `read_stamps_strict` → `check_stamps` first): the model for `update_stamps`.
- **Fail-open hook** (statusline.rs:56): a refused instance returns `ExitCode::SUCCESS` before any read; the event and dialog paths can take the same `canonical` + `trusted` pair before their `viola_obs_init`.
- **Exit-21 frame-verb refusal through `client::unreachable`** (link.rs:108–115) for a dead instance and a failed server check; the strict-modes refusal of `link` / `unlink` is printed before `client::start` through `client::refused_document` / `human::instance_state_pair` + `human::refuse`.
- **Windows DACL pair witness** (`wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not`, viola-e2e scratch.rs:321): a tree made through `create_private_dir` passes `check_instance`, a sibling granted Users write through `icacls` reads `Writable` — the shape for CARRY 12's Windows case.

## Conventions to follow
- **Refusal text**: the fixed pairs live in `src/human/` (`human::instance_state_pair`, the home pair `list` and `plugin install` print); a new hint is a literal in `src/human/hints.rs` pinned by a test (a11y Plain language).
- **No path, pid or start time in any refusal, hint or line** (security-plan §Error Handling); `tests/security_negatives_server_identity.rs`'s `digit_runs` scan is the assertion form.
- **OS gates**: a strict-modes case runs on all three OSes (no peer pid involved); a Unix mode case is `cfg(unix)`, a DACL case `cfg(windows)` (testing.md 2026-10-26: `#[cfg(test)]` first, the OS gate second).
- **One helper per mechanism**: new home checks go in `viola_state::strict` (sync std, no tokio — arch Crate dependency direction); every replace stays on `replace_private` / `replace_private_shared`.

## New files to create
- `tests/security_negatives_strict_modes.rs` — the frame verbs' strict-modes negatives in both modes beside a passing control, and test-plan §6's owed security control negatives: single writers, umask-independent modes (Unix), the Windows `--home` outside the profile
- `tests/chaos_snapshot_replay.rs` — test-plan §6 E5 steps 1–3: a stopped wrapper's snapshot cut short or of `v` 99, the next start taking the replay

## Files to modify
- `crates/viola-state/src/strict.rs` — the `0o077` read rule (Unix) and the read-grant rule (Windows) on the home, `instances/<name>/`, its `diagnostics/` and `events.ndjson`; a home-tree check for `run`, `revive` and `verify`; a diagnostics check; the Windows widened-DACL cases
- `crates/viola-state/src/stamps.rs` — `update_stamps` reads only after `check_stamps`
- `crates/viola-state/src/liveness.rs` — `live_endpoint` reads through `read_snapshot_or_replay`; its dated-gap doc comment retired
- `crates/viola-state/src/pin.rs` — the Windows `pin_companions` owner/DACL leg on `conpty/` and both companions
- `crates/viola-mcp/src/tools.rs` — `frame` runs `check_instance` on `ctx.home` and the target's dir before `live_endpoint`
- `crates/viola-mcp/tests/handlers.rs` — the frame tools' widened-instance case
- `src/cmd/client.rs` — one shared pre-start instance check and its refusal for the frame verbs
- `src/cmd/send.rs` — the check before `client::start`
- `src/cmd/wait.rs` — the check before `client::start`
- `src/cmd/last.rs` — the check before `client::start`
- `src/cmd/answer.rs` — the check before `client::start`
- `src/cmd/pause.rs` — the check before `client::start`
- `src/cmd/release.rs` — the check before `client::start`
- `src/cmd/link.rs` — onto the shared check helper
- `src/cmd/hook.rs` — `canonical` + `trusted` before `viola_obs_init` on the event and dialog paths, no SessionEnd fallback on a refused check; `connect` reads through `read_snapshot_or_replay`
- `src/cmd/hook/statusline.rs` — `canonical` / `trusted` shared with the event path; `serve` reads through `read_snapshot_or_replay`
- `src/cmd/run.rs` — the home-tree check before `open_wrapper_log`; `collision_check` reads through `read_snapshot_or_replay`; the ConPTY owner/DACL outcome
- `src/cmd/revive.rs` — the home-tree check on its start arm
- `src/cmd/verify.rs` — the home and stamps check before its `cli` log and before `update_stamps`
- `src/obs.rs` — `open_role_file` and `write_detail` check before they narrow or open
- `schemas/diag-line.v1.json` — the `sideload_fallback` / `run.conpty_sideload` outcome word for a refused companion check
- `tests/cli_controls_not_disableable.rs` — the `--home` row's 0770 frame-verb negative
- `tests/support/home.rs` — `seed_conpty` lets viola's own `create_private_dir` make the home and the seed
- `tests/hook_fail_open.rs` — the widened-instance row of the fail-open matrices
- `tests/hook_statusline.rs` — the Windows widened-instance case of the statusline arm
- `tests/run_cli.rs` — `run`'s home and `bin/` / `plugin/` refusals
- `tests/cli_verify.rs` — `verify`'s widened stamps refusal
- `tests/conpty_sideload.rs` — a companion its owner/DACL check refuses degrades to the inbox ConPTY

## Sweeps
- `check_instance(` under `src/`: 3 hits (hook/statusline.rs:45, revive.rs:137, link.rs:98) · re-derived, matching the explicit-session-links record
- `read_snapshot(` under `src/` outside test modules: 3 hits (hook.rs:204, hook/statusline.rs:142, run.rs:383) · re-derived, matching the server-verification record
- `human::refuse\(` count under `src`: 15 (verify 5, run 5, list 2, revive 1, plugin 1, link 1) · re-derived, matching obs's census 15
- `client::start` product callers: 10 (graph) · the six frame verbs take the check at their entry, the other four keep theirs
- `"ui/` and `join("ui")` under `src` and `crates`: 0 hits · the scope's `ui/` names no resource (removed 2026-10-16)

## Mechanism re-derivation
- **The narrowing trap (CARRY 14, measured)**: re-derived at HEAD — `open_role_file` → `ensure_private_dir(home)` → `create_private_dir`, which narrows an existing dir to 0700 on Unix (src/obs.rs:43–45, 76–78); a check after `viola_obs_init` reads the narrowed mode. Holds on every verb whose first act is `client::start` / `viola_obs_init`: send, wait, last, answer, pause, release, `hook`'s nine events (hook.rs:104), `verify` with a name (verify.rs:65), `run` (run.rs:164).
- **`ETXTBSY` (CARRY 16, hypothesis)**: not re-derived as a cause. A mechanism read in source: `replace_private_retrying` renames the temp file over `bin/<key>/viola` while its write handle is open (fs.rs:289–297), so in that window the path names an inode open for writing; an `execve` of it in another process (the first wrapper's child's first hook spawn) fails `ETXTBSY` on Linux. The equality the fix needs — a copy closed before the rename gives 0 of N concurrent two-instance boots a failed first hook — is the implement measurement's, two-sided against today's 6 of 24.
- **Seeded home refused on Windows (CARRY 13)**: read, not measured — `seed_conpty` makes the home with `create_dir_all` (home.rs:299), which on `windows-2025` inherits BUILTIN\Users write (measured on the scratch at ci#38375200217, 2026-10-13-epoch-4-cleanup), the entry `check_instance` refuses as `Writable`. The DACL read itself is owed to the Windows leg.
- **Build reproducibility (CARRY 11, measured at chunk 2026-10-09)**: not re-derived here; its measurement (two clean release builds of one tree, by file and by section, harness rule 2026-10-09) is an implement step, its consequence the founder's.

## Open questions
- none — the three plan-decision questions P3 raised were settled at P4: the readers' move, the no-line refusal, the frame verbs' pair and the scope cut by the founder's own answers (`inputs#I3`), the `conpty/` outcome by a decisive lean (plan.md Provenance). The plan names the carved-out entry «Pinned copy race and reproducible builds» on the P5 card.
