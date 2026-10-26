# Report — 2026-10-26-server-verification-before-any-frame

**Chunk:** every frame-writing client (CLI, hook, MCP) verifies the wrapper's pid and start time first; mismatch reads unreachable, hooks fail open
**Date:** 2026-10-26
**Commits:** since `last_wrap` 2026-10-26T07:03:04Z (`git log --oneline --since=…`): `6bdfdda8` fix(2026-10-26-tokio-rustsec-2026-0386-lock-bump) (the last wrap's own commit) · `4aa9def9` chore(2026-10-26-server-verification-before-any-frame): operator pre-CI commit, for the run this chunk's verdict reads · `775641fa` fix(2026-10-26-server-verification-before-any-frame): the last and pause test modules gated whole off macOS, closing the macos lint red. Outcome basis: `git log --format='%h %s' 6bdfdda8..HEAD` = `775641fa`, `4aa9def9` (base `6bdfdda8`, the parent of the oldest pre-CI commit).

## Changes (structured — detectors read this)
- **Files:** 18 changed (basis: `gate.py scope`, `scope: clean — changed 18 · listed 16 · recorded 2`; `cites.py added`, `new text: 18 file(s) · 1 new`). New: `tests/security_negatives_server_identity.rs`. Modified: `crates/viola-channel/src/client.rs`, `crates/viola-channel/src/tokio_client.rs`, `crates/viola-channel/src/lib.rs` (companion), `crates/viola-state/src/liveness.rs`, `src/cmd/client.rs`, `src/human/hints.rs`, `src/cmd/send.rs`, `src/cmd/wait.rs`, `src/cmd/last.rs`, `src/cmd/answer.rs`, `src/cmd/pause.rs`, `src/cmd/release.rs`, `src/cmd/hook.rs`, `crates/viola-mcp/src/tools.rs`, `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/contract_fake_agent_drift.rs` (in-intent, recorded). Plus the chunk folder (`scope.md`, `research.md`, `plan.md`, `scope-record.md`, `evidence/*`, `inputs/*`), `verification-matrix.json` (two P5 notes, `v1-41` and `v1-44`), the master record (`pending`) and the working-route stamp.
- **Symbols / APIs:** (ranges copied from `## New text, by line`)
  - `viola_channel::Client::server_pid(&self) -> Option<u32>` (`crates/viola-channel/src/client.rs` 57-61) and `pub(crate) fn server_pid_of(stream: &impl StreamCommon) -> Option<u32>` (132-139): the connected stream's peer pid, `peer_creds().pid()` (interprocess 2.4.4 `StreamCommon`): Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end (the open stays `open_handle`'s SQOS + overlapped `CreateFileW` adopted by `Stream::try_from`), macOS none. `SERVER_PID_REPORTED` (`client.rs` added line 12), a `const bool`, false on macOS; re-exported at `crates/viola-channel/src/lib.rs:20`. Writes nothing, logs nothing.
  - `viola_channel::tokio_client::TokioClient::server_pid` (`tokio_client.rs` 44-47), behind the `tokio` feature as the file is.
  - `viola_state::liveness`: `live_endpoint(instance_dir) -> Option<LiveEndpoint>` — signature changed from the bare endpoint: `pub struct LiveEndpoint { endpoint, pid, started_at }` (liveness.rs added 72-82 range; struct at the tree's 79-83), all from the one `read_snapshot` call (no second read). Callers: `src/cmd/{send,wait,last,answer,pause}.rs` and `crates/viola-mcp/src/tools.rs` (6 product callers, research.md graph; `release` reaches the wrapper through `client::connect` as the others do). New: `pub enum ServerVerdict { Matches, Mismatch, Unverifiable }` (102-111), the pure `pub fn server_verdict(recorded_pid, started_at, observed_pid: Option<u32>, observed_start: Option<u64>, peer_pid_reported: bool)` (113-140; `peer_pid_reported == false` → `Matches` with no comparison, the macOS recorded gap), the joined `pub fn verify_server(recorded_pid, started_at, observed_pid, peer_pid_reported)` (the start time read through `process_start_time`, a zombie none), the private `recorded_secs` (63-68; `same_process` now reads through it).
  - `src/cmd/client.rs`: `pub(crate) enum Unreachable { Dead(&'static str), ServerUnverified }` (75-82) with `detail()` (`instance-dead` | `server-verify-failed`), `during()` (the server cause always `connect`), `hint_cause()` (`not-running` | `server-verify`); `pub(crate) enum CallError { Channel(ChannelError), ServerUnverified }` (110-116); `pub(crate) fn connect(live: &LiveEndpoint) -> Result<Client, CallError>` (124-139) — the verified connection every CLI frame verb now uses; `answer_of` maps `CallError::ServerUnverified` to `Unreachable::ServerUnverified`; `unreachable` / `unable_unreachable` take the closed cause. Test-only: `pub(crate) mod refused_inside` (227-324, `assert_refused_inside`, a captured-span check shared by the six verbs' cases).
  - `src/human/hints.rs`: `send_hint`'s new cause `server-verify` (50-52) prints exactly `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` (design-system surface.md cli-send's string, byte for byte).
  - `src/cmd/{send,wait,last,answer,pause,release}.rs`: each `request` goes through `client::connect` inside its dispatch span (`send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client`, `release.client`); the client's own refusals (exit 13) stay first; a mismatch or an unmakeable check is exit 21 with the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` document, or the `unable  <name>  instance-unreachable` line with the new hint last on stderr.
  - `src/cmd/hook.rs`: `enum NotSent` (181-187, `detail()` `channel-unreachable` | `server-verify-failed`), a shared `connect` (the tree's 199-217: `read_snapshot` → `connect_by` → `verify_server`) used by `deliver` (event) and `ask` (dialog); `Asked::ServerUnverified`; a SessionEnd whose check fails takes `append_session_end`. **Remaining-caller fact:** `src/cmd/hook.rs` now calls `read_snapshot(` ONCE (`:204`), down from twice at the base (`6bdfdda8` `:246`, `:494`) — the two paths share one read site.
  - `crates/viola-mcp/src/tools.rs`: `frame` verifies after `TokioClient::connect` through `async fn server_verified(live: LiveEndpoint, observed: Option<u32>) -> Result<bool, McpError>` (348-357), the start-time read on `tokio::task::spawn_blocking` (351-353); a mismatch returns `result::unreachable()` with no frame, no new log line.
  - IPC methods, endpoints, ports, sockets, env vars: none added or changed.
- **Crates / modules:** none added or removed; changed `viola-channel` (client, tokio_client, lib), `viola-state` (liveness), `viola-mcp` (tools), root bin (`cmd::client`, the six verbs, `cmd::hook`, `human::hints`).
- **Dependencies:** none added or bumped (`Cargo.toml` / `Cargo.lock` absent from `git diff --stat 6bdfdda8`); interprocess 2.4.4, sysinfo, tokio pins unchanged; `viola-channel`'s normal graph stays tokio-free (`channel-tokio-free.sh` green).
- **Schema / config:** none — `server-verify-failed` was already a member of `schemas/diag-line.v1.json`'s `process-exit` and `hook-decision` `detail` enums; this build is the first that WRITES it: `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, and `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson` (dialog path: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`). No pid, start time, path or endpoint in any line (asserted). MCP writes no line for the mismatch (as its unreachable path). No config key, `--json` code, exit code, `RefusalReason` or MCP error word added.
- **Spec-master edits:** none (no master edited by implement; the wrap applies the expected amendments).
- **Retired:** the server-identity half of the first and the fourth to eighth dated gaps on Linux and Windows (`hook.event`; CLI `send`; `wait` / `last`; `answer` + `hook.dialog`; `pause` / `release`; `viola mcp`'s four frame tools) — each now verifies the serving process before its first frame. Their strict-modes half stands until "Home and code-bearing file integrity"; macOS stays on the recorded gap (no peer pid; the euid and socket-directory checks owned by "Unix endpoint and home hardening"). No census row (no `census.md`).
- **Counts / qualifiers moved:**
  - The snapshot-readers count (pattern `readers`): `read_snapshot(` under `src/` outside test modules reads **3** product callers at this tree — `src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:379` (basis: `grep -rn 'read_snapshot(' src/`, every other hit inside a `#[cfg(test)] mod tests` read per file: `src/run/{snapshot,links,wheel,dialog,budget}.rs`, `src/run/send/slot.rs`, `src/cmd/run.rs:1137`). The masters state **five** (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`); the base `6bdfdda8` read four (client.rs gone since chunk 2026-10-17-mcp-server-for-drivers, which moved the CLI read into `viola_state::liveness::live_endpoint`); this chunk moved hook.rs's two to one. The plan's expected «four product callers remain» is superseded by the measured three. Sites — `readers: 7 standing · 5 changed (architecture-detail/standard-contracts.md:25, security-plan-detail/input-validation.md:52, test-plan-detail/e2e-test-strategy.md:235, test-plan-detail/unit-test-strategy.md:18, obs-plan-detail/span-trace-coverage.md:167 — each names five and `src/cmd/client.rs`) · 2 no-change (obs-plan.md:394, no count, «the other snapshot readers write no `state-recovered` line» true; registries/contracts/architecture/crate-dependency-direction.md:5, `src/cmd/client.rs` named as the history of `live_endpoint`'s move)`.
  - The hook fail-open matrix (pattern `matrix11`): `hook_fails_open_silently_within_the_spine_bound` gains one row, `case::impostor` (`Env::Impostor`, `stop`, `server-verify-failed`), under `cfg_attr(not(target_os = "macos"), …)` — 11 `#[case]` rows at the base (`git show 6bdfdda8:tests/hook_fail_open.rs`), 12 off macOS now; `hook_dialog_tier_fails_open_silently` gains `question_impostor` and `permission_impostor`, the same gate. `matrix11: 3 standing · 3 changed (test-plan.md:598 and test-plan-detail/e2e-test-strategy.md:242 ×2, :244 list the matrix's causes without the server-verification mismatch; :242 states «the 11-case matrix», a count the masters do not state per the Tier 1 rule — name the functions)`.
  - Sweep reach: 11 patterns · 3 moved facts, 3 with a pattern (the report's own count: readers, matrix11, and the readers' hook.rs remaining-caller fact under `readers`) · owner pattern 11 standing · by line 0 lines · 0 offsets. (`owned` and `macgap` controls unfired — `owned · control UNFIRED over the seven masters (bodies and sets) at 775641fa`, `macgap · control UNFIRED …`; neither is a count stated here.)
- **Dev-tool versions:** a HOST std component added on the Linux dev host during the operator pass: `rustup target add aarch64-apple-darwin` (the rust std for the macOS target, the "lint target" for a check-only clippy under `CARGO_TARGET_DIR=target/maccheck`), 2026-10-26 (operator-pass.md §6). The toolchain itself is unchanged (`rust-toolchain.toml` 1.98.1). No lockfile resolves it. cargo-nextest, cargo-deny re-read unchanged.
- **Harness / gate surface:** none changed (no `scripts/`, `.github/`, `xtask` file in the diff). New test binary `security_negatives_server_identity` (root, `#![cfg(not(target_os = "macos"))]`). A host lint recipe used for the first time: the macOS-target clippy (`--target aarch64-apple-darwin`, `target/maccheck`), its known-positive control reproducing CI's `lint (macos-latest)` error line for line — not a plan gate entry.
- **Cross-project / external claims:**
  - CI, the final HEAD: ci#40295857215 on `775641fa58b3f3e5cf2db1a7dfb966e398b92e91`, push, `run_attempt` 1, conclusion `success`, `verdict: green · checks 15/15` (fuzz-replay · lint ×3 · msrv · perf ×3 · release ×3 · supply-chain · test ×3); test legs' new-case `PASS` lines: windows-2025 39, ubuntu-latest 39, macos-latest 10, `FAIL` 0 each (evidence/operator-pass.md). The pre-CI run ci#40295285860 on `4aa9def9` was red, `lint (macos-latest)`: `unused import: super::*` at `src/cmd/last.rs:85:9` and `src/cmd/pause.rs:95:9` — this chunk's own, fixed by `775641fa`. The wrap's own commit adds bookkeeping on top of `775641fa`; no source moves after it.
  - interprocess 2.4.4's Windows peer pid: read in source (`interprocess-2.4.4/src/os/windows/named_pipe/stream/impl/handle.rs:16`, `stream/impl.rs:88`), witnessed by `client_server_pid_is_the_serving_process` / `tokio_client_server_pid_is_the_serving_process` on `test (windows-2025)`.
  - Inputs (`inputs.py verify`, fired before this report; I7 and I8 snapped at this wrap and cited here): `I1 · ../additional/viola-overseer/phase-server-verification-directive.md · copy no-repo · unchanged` · `I2 · message (the operator, 2026-10-26T07:09Z, the phase invocation) · n/a` · `I3 · message (delegate the Viola overseer, 2026-10-26T08:08Z — fork 1's PROVISIONAL answer, option 1) · n/a` · `I4 · message (delegate the Viola overseer, 2026-10-26T08:19Z — the word on P5 review card 1) · n/a` · `I5 · message (the overseer, founder-delegated, 2026-10-26 08:39Z — fold the ETXTBSY fix in) · n/a` · `I6 · message (the operator, live, 2026-10-26 — the /andromeda-implement invocation, the operator pass and the must-trace check) · n/a` · `I7 · ../additional/viola-overseer/wrap-server-verification-directive.md · copy no-repo · unchanged` (the wrap directive: forced sample at P2; keep every transcript and the three run dirs; stop at a hard cascade failure; report at P2 validated; note F354 (a)/(e) on first use; fork 1 held PROVISIONAL; obs-plan §4's `wait` / `last` sentence now false) · `I8 · message (the operator, the wrap invocation, 2026-10-26T09:15Z) · n/a`. Verify's summary: `inputs: 8 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 6 · uncited 5 · unparsed 0` — the five it read `UNCITED` (I4, I5, I6, I7, I8) are cited by this bullet. No DRIFT.
  - inputs#I4 names P5 review card 1, `.andromeda/runs/2026-10-26T07-09-33-phase/p5-card-1.md` (the only card; none superseded). The word: «yes — the overseer's technical answer (founder-delegated; he is asleep, his word: continue the work), 08:19Z …; fork 1 stays PROVISIONAL (inputs#I3) and the four Beyond-the-plan re-pins carry it to his word.» Authority: delegate the Viola overseer, founder-delegated — PROVISIONAL on fork 1 (the founder's own fork). Its `## Beyond the plan` rows, verbatim:
    > 1. **CARRY 7 / 8 re-pinned, held for the founder's own word** — the readers' move onto `read_snapshot_or_replay`
    >    (`verification-matrix.json#v1-41`, test-plan §6 E5 steps 1–3) and what a client does with an unreadable or newer
    >    snapshot leave this entry as one CARRY on the working-route entry **"Home and code-bearing file integrity"**: the
    >    classified read is taken only after `check_instance` (security-plan §Input Validation, own state files), the check
    >    that entry adds before these same readers' reads. The fork stands answered provisionally (option 1, `inputs#I3`)
    >    until his word.
    > 2. **CARRY 2 re-pinned, unclosed** — a hook reaching an exiting wrapper's own Windows pipe (ci#36532038635), its
    >    SessionEnd fallback skipped in that window: the check passes there by design (the exiting wrapper is the recorded
    >    process). The measurement — whether sysinfo still lists a terminated Windows process whose pipe handle is open, and
    >    SessionEnd's outcome in that window — moves as one CARRY to **"Windows-only live measurements"**.
    > 3. **The macOS server-verification cases** — test-plan §6's macOS exit-cause case (the per-user socket directory
    >    loosened to 0755 → exit 21) and the macOS client euid check join **"Unix endpoint and home hardening"** as one
    >    CARRY beside its own directory and euid freight; this chunk's negatives compile out on macOS.
    > 4. **"Home and code-bearing file integrity"'s strict-modes CARRYs** for `send`, `wait` / `last`, `answer`,
    >    `pause` / `release`, the MCP frame tools and the hook client keep their strict-modes half only: their
    >    server-identity half is closed by this chunk (the wrap edits their text, the CARRYs stay on that entry).
    These four rows are not yet on the working route (`git diff 6bdfdda8 -- viola-0.1.0/working-route.md` holds only this entry's stamp): wrap P5 lands them as recorded directions, cited to this card.
- **Reverted / negative API facts:** a first gate of the new `refused_inside` helper module, `#[cfg(all(test, not(target_os = "macos")))]`, put its `cfg!(windows)` ahead of `src/cmd/client.rs`'s first `#[cfg(test)]`, so `contract_windows_mutation_scope` read the file as Windows-gated product code (block run 2 red); restacked as `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`. The `last.rs` / `pause.rs` test modules' inner per-case macOS gate (pushed at `4aa9def9`) was replaced by a whole-module gate (`775641fa`).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - obs-plan §4 Scenario `wait` / `last` and its detail: «`strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands» (obs-plan.md:361; obs-plan-detail/span-trace-coverage.md:90) — false for `server-verify-failed` on Linux and Windows: `tests/security_negatives_server_identity.rs` reads it on `wait` and `last` (evidence/must-trace-check.md, «Finding»; inputs#I7 item 7). `strict-modes-failed` still cannot occur there (the strict-modes half stands). Pattern `svf`: 5 standing · 2 changed (those two) · 3 no-change (design-system-detail/surface.md:73 and obs-plan-detail/log-coverage.md:28, :30 — closed-enum lists, still true; an «as landed: written by this build» qualifier is a candidate).
  - The snapshot-readers list «five product callers (`src/cmd/client.rs`, … `src/cmd/hook.rs` twice …)» (pattern `readers`, the five sites under Counts) — measured three (above); `src/cmd/client.rs` holds no `read_snapshot(` call.
  - The MCP impostor case's home: test-plan-detail/integration-test-strategy.md:47 «The MCP half is the same impostor case in `cross_exit_causes.rs`, the crate with the rmcp client» — the case landed as `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` in `crates/viola-mcp/tests/handlers.rs` (359-403), driving the transport-free `Tools` handle, no rmcp client; `cross_exit_causes.rs` does not exist (`ls tests/cross_exit_causes.rs` absent). Pattern `tests`.
  - The server-verification exit-cause case's home: test-plan.md:604 and test-plan-detail/e2e-test-strategy.md:261, :264 place it in the `cross_exit_causes.rs` matrix (rstest table, a last `hint:` line) — it landed as `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` and its control `security_negative_cli_verb_reaches_the_recorded_wrapper` in `tests/security_negatives_server_identity.rs` (175-263, 265-289), six verbs × two modes; the macOS 0755-directory case is not built (P5 card row 3 → "Unix endpoint and home hardening"). Pattern `tests`.
  - The reader move's owner: test-plan.md:590 «owed to the route entry "Server verification before any frame"» and test-plan-detail/e2e-test-strategy.md:235 «the five other snapshot readers taking the replay are owed to "Server verification before any frame"» — this entry closes without it; the move is re-pinned to "Home and code-bearing file integrity", held for the founder's word (P5 card row 1, fork 1 PROVISIONAL). Pattern `owner`.
  - security-plan-detail/authentication-authorization.md:27 «**Collision check:** `run` uses the same verification.» — standing before this chunk and not built: `run`'s collision check (`src/cmd/run.rs:379`) reads `read_snapshot` and verifies no server (plan Graph impact: «no change: no frame, no connection»). Pattern `gaps` (the row at :27). Disposition owed at Validate.
- **Expected amendments (from plan):**
  - security-plan §Authentication & Authorization (IPC client-side server verification row; `~/.viola/` access control; own state files), §Security Anti-Patterns › Authentication, §Error Handling (the `hook` row), §Input Validation (CLI row) — carried: **Retired** and the hook/CLI/MCP Symbols bullets. Search: patterns `owner` (security-plan 5: security-plan.md:439, authentication-authorization.md:25, :26, input-validation.md:30, security-anti-patterns.md:8), `gaps` (security-plan 15, of which changed: security-plan.md:114, :439, authentication-authorization.md:25, :26, :27, input-validation.md:30, security-anti-patterns.md:8, error-handling.md:8, bootstrap-phases.md:15; no-change: security-plan.md:142 (its gap clause is the strict-modes half, which stands), :325, :516, authentication-authorization.md:23 (the requirement title), :39 (the reference-values rule, true), security-anti-patterns.md:37 (the no-switch-off list)), `liveep` (security-plan 4: authentication-authorization.md:26, :27, :37, input-validation.md:30 — the liveness-only pre-check now precedes the server check), `hookjoin` (security-plan 1: error-handling.md:8 «server verification and strict-modes join at Epoch 6» — server verification joined, `server-verify-failed` a fail-open code this build writes), `peerpid` (security-plan 5: security-plan.md:100, authentication-authorization.md:21, :24, bootstrap-phases.md:14, security-anti-patterns.md:8 — the Windows check names `server_process_id()` while the build reads `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the client end; Unix client euid not built (Unix endpoint and home hardening); disposition per site at Validate).
  - architecture §Standard Contracts (Snapshot envelope) and test-plan §4 / §6 E5, obs-plan §4 E5, security-plan §Input Validation — the readers list: carried, **Counts / qualifiers moved** (superseded in its number: three, not four). Search: pattern `readers`, 7 standing (architecture 2 — standard-contracts.md:25, registry crate-dependency-direction.md:5; security-plan 1; test-plan 2; obs-plan 2).
  - architecture §Occupied Resources — `cli-<name>.ndjson`'s exit-21 `process-exit` gains `detail:"server-verify-failed"`; `hook-<name>.ndjson`'s `hook-decision` writes it: carried, **Schema / config**. Search: pattern `svf` reads 0 architecture sites (5 standing: design-system 1, obs-plan 4) — no architecture owner named on a hit; the architecture detector's own registry read is the control for reach.
  - obs-plan §4 Scenario `wait` / `last` (the «cannot occur» sentence retired) and §6 log-details (written by this build): carried, **Spec claims disproved** and **Schema / config**. Search: `svf` obs-plan 4 (obs-plan.md:361, log-coverage.md:28, :30, span-trace-coverage.md:90), `owner` obs-plan 2 (obs-plan.md:361, span-trace-coverage.md:90).
  - test-plan §6 Exit-cause matrix / §5 Wrapper channel — the server-verification cases as landed; `cross_exit_causes.rs` does not exist; the macOS 0755 case owed to "Unix endpoint and home hardening": carried, **Spec claims disproved** (the case homes) and **Counts** (matrix11). Search: `tests` 8 standing, all test-plan (test-plan.md:449 no-change (handler-test home, true), :596 no-change (layer note), :604 changed, e2e-test-strategy.md:242 changed (matrix list), :261 changed, :266 no-change (E1's own home), :274 no-change (revive's), integration-test-strategy.md:47 changed); `gaps` test-plan 10 (changed: test-plan.md:590, e2e-test-strategy.md:235, :264; no-change: test-plan.md:46, :435 (testable, now landed), :250, :767 (macOS euid + dir-only, not built, owned elsewhere), e2e-test-strategy.md:226 (route order), :290 (the Windows SQOS case, not this chunk's), test-scope-summary.md:11 (testable)); `matrix11` 3.
  - design-system surface.md cli-send / cli-exit-codes and layout-templates §Component — Primary content block 2 — the server-verification hint built for the six verbs: carried, **Symbols / APIs** (hints.rs). Search: `hint` 1 standing (design-system-detail/surface.md:58, no-change: the built string equals it byte for byte); `gaps` design-system 2 (surface.md:58, :110, no-change — the hint row lists «server verification» as a cause; an «as built» qualifier is a candidate). layout-templates: 0 hits for `hint` and `gaps` (`hint · … standing 1`, all in design-system); its block at layout-templates.md:141-143 shows only the not-running hint — no layout-templates owner named on a hit.
  - none of the entries is a ledger-note entry; the two matrix notes (`v1-41`, `v1-44`) were written at phase P5 (`verification-matrix.json` diff), nothing owed at P7.3.
- **Coverage of new surfaces:**
  - CLI `send` / `wait` / `last` / `answer` / `pause` / `release` server check → validation mechanism✓ (pid + start time against the one snapshot read; closed `ServerVerdict`) · instrumentation log✓ (`process-exit` `server-verify-failed`, `during:"connect"`; the dispatch span kept, no `channel.request`) · PII redacted✓ (no pid, path, endpoint in any output or line — asserted) · tests unit (decision table, `unreachable_logs_its_cause`, six span cases) + integ (`security_negatives_server_identity`: 12 mismatch rows, 6 verbs × 2 modes, and 6 control rows each looping both modes) · a11y n/a (stderr text; the hint plain ASCII, no SGR — asserted) · tokens n/a
  - `viola hook` event + dialog server check → validation✓ · instrumentation log✓ (`hook-invoked` + `hook-decision` `server-verify-failed`) · PII redacted✓ · tests unit (`ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper`, `hook_refuses_an_unverified_server_inside_hook_handle`, the SessionEnd mismatch row) + integ (`hook_fail_open` three impostor rows, `< 1.0 s`) · a11y n/a · tokens n/a
  - `viola mcp` frame tools server check → validation✓ · instrumentation n/a (no line by plan: the closed `event` enum; inside `mcp.tool_call`) · PII n/a (codes only) · tests integ (`handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`) · a11y n/a · tokens n/a
  - `Client::server_pid` / `TokioClient::server_pid` → validation n/a · instrumentation n/a (writes, logs nothing) · PII n/a · tests unit (each OS: the serving process's pid; macOS none) · a11y n/a · tokens n/a

## Deviations from intent
1. **`tests/contract_fake_agent_drift.rs` (in-intent, recorded)** — block run 1's pre-push red was ETXTBSY in `fake_agent_agents_replay_matches_every_recorded_agents_set`: two threads each copied the fake agent (the tmpfs `target/e2e-home` defeats the hard link) and one thread's forked child inherited another's write handle. 6 of 40 standalone runs red before, 0 of 40 after planting every set's bin sequentially before the threads start (evidence/etxtbsy-drift-test.md). Folded by the standing rule (a red met during a chunk folds into that chunk) on the overseer's founder-delegated word, `inputs#I5`.
2. **`crates/viola-channel/src/lib.rs` (companion, recorded, self)** — re-exports `SERVER_PID_REPORTED` for step 1.
3. **Step 10's hook unit impostor** varies the start time (this process recorded a second early; an unparseable `started_at`) instead of recording another live pid: a root-bin unit test has no portable second process; the other-live-pid shape is the integration rows'.
4. **Steps 13 and 14's held process** is the fake agent's holder mode `--hold-stdout-internal` (sleeps 10 s, writes nothing, killed and reaped by a `Drop` guard), not a bare `viola-fake-agent` spawn.
5. **Step 14's rows** are `cfg_attr(not(target_os = "macos"), case::…)` rows in the two existing matrices.
6. **Added beyond the plan, on the operator's check (`inputs#I6`):** seven captured-span cases (one per CLI verb, one hook) and the shared `cmd::client::refused_inside` helper, each red with the check removed (evidence/red-before-green.md, second reading 7 of 7 red).
7. **Hook read sites:** the plan kept `deliver` and `ask` each on its own `read_snapshot` (`:246`, `:494`); implement joined them in one `connect` (`:204`), so `src/cmd/hook.rs` holds one call (Counts).
8. **The operator pass's CI red was this chunk's own** (`lint (macos-latest)`, `4aa9def9`), fixed by gating the two test modules whole (`775641fa`); the host gained a macOS-target check-only lint (Dev-tool versions).

Scope record (`gate.py scope`: `scope: clean — changed 18 · listed 16 · recorded 2 (companion 1 · mechanical 0 · in-intent 1 · widening 0) · absorbed 0 · excluded 469`):
- companion — `crates/viola-channel/src/lib.rs` · serves step 1 · self
- in-intent — `tests/contract_fake_agent_drift.rs` · serves the pre-push gate (entry 21): its ETXTBSY red, met during this chunk and folded into it · word: «A red met during a chunk folds into that chunk: a test-only fix of a measured mechanism (ETXTBSY from an inherited write fd, 6/40 standalone). [...] Show the fix with the 40-run loop at 0 red, and put the guard's outside-the-plan line for tests/contract_fake_agent_drift.rs in the report; accepted as named.» — delegate the overseer (founder-delegated), 2026-10-26 08:39Z, inputs#I5. The guard's line (implement block run 3, entry 13): `outside tests/contract_fake_agent_drift.rs (recorded in-intent · serves the pre-push gate (entry 21): its ETXTBSY red, met during this chunk and folded into it)` · `guard: outside 1 files`.

## Decisions & corrections
- Fork 1 (CARRY 7 / 8 — what a verified client does with an unreadable or newer snapshot; the readers' move to the replay): option 1, keep today's outcome, every reader on `read_snapshot` — the overseer's PROVISIONAL delegate answer, 2026-10-26 08:08Z (`inputs#I3`), standing only until the founder's own word; held as such through P5's re-pin (inputs#I7 item 6).
- The P5 review card's word, `inputs#I4` (delegate, 08:19Z, PROVISIONAL on fork 1): the four Beyond-the-plan re-pins (above) carry the fork to his word.
- The ETXTBSY fold, `inputs#I5` (delegate, 08:39Z): fold the test-only fix in; the 40-run loop at 0 red; the guard's outside line accepted as named.
- The operator's must-trace check (`inputs#I6`): confirmed — every new pre-frame refusal opens its scenario's root span and writes its outcome line; no `channel.request` span or `channel-request` line, since no frame is sent (evidence/must-trace-check.md).
- Recurrence of the deferred handoff learning (the Windows scope contract's `is_windows_gate` reads `cfg!(windows)` as well as `cfg(windows`): block run 2's red (Reverted bullet) — a second occurrence, now measured.
- A host-reachability gap: a test gated off macOS left a macOS-only unused import no local lint could see; the macOS-target check-only clippy (`rustup target add aarch64-apple-darwin`, `CARGO_TARGET_DIR=target/maccheck`) reproduced CI's line exactly — a candidate host recipe beside `target/wincheck` (outcome deviation 8).
- Stamp hook, twice: a time written by estimate (the journal's run-1 line, the ETXTBSY evidence's loop times) was refused as ahead of the clock and re-read from `date -u` and the logs' mtimes (implement outcome, Decisions & corrections).
- Guard refusals at implement: a `cd` out of the cwd (re-issued with an absolute path) and a `cat` heredoc to a file (re-issued as a python heredoc), one re-issue each. At this wrap: a `cd` into `.andromeda/` (re-issued as a subshell) and a pipeline tool's output piped into `grep` (re-issued bare), one re-issue each.
- Testing learning 2026-10-16 (copy an oracle from the first run's output) recurred at implement: a hand-typed sorted key list, red on its first run.
- Sweep hazard (this wrap): the plan's «four product callers remain» was counted at P3 against a tree where hook.rs held two reads; implement merged them, so the plan's number was stale by the time the wrap read the tree — a count carried from a plan is re-measured at the wrap, never copied.
- Sweep hazard (this wrap): pattern `macgap` (`recorded gap|pid gap`) never fired over the masters although the macOS gap is stated at authentication-authorization.md:24 as «recorded as a known gap» — the wording sits between the two tokens; the `gaps` and `peerpid` patterns reach those sites instead.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (security) Linux and Windows, an impostor server: the six verbs write no frame, exit 21, the exact document under `--json` (stderr empty), the hint last on stderr in human mode — MET: `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (12 rows), green on host and on CI ubuntu-latest and windows-2025.
- (security) The control reaches the recorded wrapper, one frame each — MET: `security_negative_cli_verb_reaches_the_recorded_wrapper` (6 rows, both modes each).
- (obs) One `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, no `channel-request`, no pid / start time / path / endpoint — MET (the same binary reads `cli-builder.ndjson`; `digit_runs` scans every output).
- (security) The hook's `stop`, `pre-tool-use`, `permission-request` against the impostor: exit 0, empty streams, `< 1.0 s`, no frame, `hook-decision` `server-verify-failed` — MET (`hook_fail_open`'s three impostor rows).
- (arch) A SessionEnd whose check fails takes the direct append — MET (the hook unit case's mismatch row).
- (security) MCP `send` / `wait` / `last` / `answer` against the impostor: `isError`, `{"error":"instance-unreachable","detail":null}`, text `refused: instance-unreachable`, no frame — MET (`binary(handlers)`).
- (tests) The decision table's seven rows — MET (`server_verdict_rows`, under `run --unit`).
- (tests) `run --perf`: every hook row `max < SPINE_DEADLINE`, zero panics, the check in the timed path — MET (hook rows max 2.2–2.4 ms over 30 runs each, against P5's 1.23–1.6 ms; the bound 1 s).
- (arch) No sync crate gains tokio — MET (`channel-tokio-free.sh` green, `cargo deny check` green).
- (tests) CI of the final HEAD `verdict: green`, every leg — MET (ci#40295857215 on `775641fa`).
- No verification-matrix capability claimed — MET (`matrix.py`: claimed 0; P5 notes on `v1-41`, `v1-44`).

Gates (implement block run 3 of 3, trail `.andromeda/runs/2026-10-26T08-22-16-implement/gate-2026-10-26-server-verification-before-any-frame.json`: `entries 24 · green 19 · red 1 (13) · recorded 1 · timeout 0 · not-run 3`), by `run`:
- `df -h /tmp` — recorded (tmpfs 25G avail, 22 %; report-only, above the 9 GiB keepers floor)
- `cargo fmt --all --check` — green
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green
- `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc …` — green
- G1 `rg … '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)'` — green (exit 1, no output)
- `bash scripts/lint-probes.sh` — green
- `bash scripts/agent-run.sh run --unit` — green
- `cargo nextest … -E 'binary(security_negatives_server_identity)'` — green
- `cargo nextest … -E 'binary(hook_fail_open)'` — green
- `cargo nextest run -p viola-mcp -E 'binary(handlers)'` — green
- `cargo deny check` — green
- `bash scripts/channel-tokio-free.sh` — green
- `gate.py guard …` — red · `last line guard: clean` (the line read `guard: outside 1 files`): **red — the plan's wording: the guard's `--allow` list against `scope-record.md`'s in-intent line for `tests/contract_fake_agent_drift.rs` · word: «put the guard's outside-the-plan line for tests/contract_fake_agent_drift.rs in the report; accepted as named.» — delegate the overseer (founder-delegated), 2026-10-26 08:39Z (inputs#I5)**
- `agent-run.sh cleanup --session p-svf-smoke` (pre-clean) — green
- `agent-run.sh boot --session p-svf-smoke --instance builder` — green
- `agent-run.sh status --session p-svf-smoke` — green
- `bash scripts/g2-zero-panics.sh` — green
- `agent-run.sh schema-check` — green
- `agent-run.sh cleanup --session p-svf-smoke` — green (`processes_gone`, `endpoint_gone` true)
- `bash scripts/agent-run.sh run --perf` — green (hook rows max 2.2–2.4 ms)
- `bash scripts/agent-run.sh pre-push` — green at `linux-tests` (88.21 s); re-run green by the operator pass before each push (87.67 s, 88.17 s)
- `gate.py hygiene` (`leg = 'operator'`) — recorded in evidence/operator-pass.md: `hygiene: clean` before the pre-CI commit and before the fix commit
- `git diff --quiet … && git push origin HEAD` (`leg = 'operator'`) — recorded: pushed `4aa9def9`, then `775641fa`, no force
- `ci.py conclusion --sha HEAD --wait 2400` (`leg = 'operator'`) — recorded: `4aa9def9` red (`lint (macos-latest)`, this chunk's own, fixed); `775641fa` `verdict: green · checks 15/15`, ci#40295857215 — the final HEAD's run, the record P7 re-verifies.
Smoke: ✓ — boot · status · cleanup green in all three block runs; by hand a booted session, `viola last` and `viola wait --json` reached the real wrapper through the new check (exit 0, one exchange each; evidence/smoke-p3.md).

Watches: none folded.

Outcome basis: the operator pass ran (Setup 4's pre-CI commit `4aa9def9`, fix commit `775641fa`); the gate verdicts rest on its final state — ci#40295857215 on `775641fa`, green, recorded in `evidence/operator-pass.md`. This window holds no implement run: implement's persisted report `.andromeda/runs/2026-10-26T08-22-16-implement/outcome-2026-10-26-server-verification-before-any-frame.md` (the only implement dir for this marker) is read as its P4 report; the operator directives between implement and this report: `inputs#I5` (the ETXTBSY fold), `inputs#I6` (the operator pass and the must-trace check). Post-implement artifacts: `evidence/operator-pass.md` §6 (the fix round, uncommitted at HEAD), `evidence/must-trace-check.md`.

Process hygiene: implement P4's census — `17 running, none started by this run` (every row another tree's); every process it started terminated (harness sessions `p-svf-smoke` ×3 and `p-svf-p3` with `processes_gone` / `endpoint_gone` true; held fake agents reaped by their `Drop` guards; 80 nextest probe runs exited). Re-measured here, `gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --comm claude --args projects/viola/target`: `census: 16 running`, none under this repository's `target/` (the overseer sessions, the prototype's and the product's `viola`, `~/.viola/bin/…/viola mcp`, this session's `claude`).
## Addendum — resumed 2026-10-26
- inputs#I8 · message · copy · n/a — the operator's wrap invocation, 2026-10-26T09:15Z: the wrap run whole, forced sample at P2, the directive `inputs#I7` read first
  - bears on **Cross-project / external claims**: adds to it (the bullet names it `I8 ·`; cited here by its id)
- inputs#I9 · message · copy · n/a — the P2 halt's three answers (the overseer, founder-delegated, 10:21Z): the four check-7 contradictions against retired lines applied with their companion; security-plan-detail/authentication-authorization.md:27 «Collision check: `run` uses the same verification.» rewritten as built (`run` classifies the recorded wrapper by pid + start time and heartbeat and connects to nothing); the check-7 playbook rule not appended (n=1, recorded in the friction log for overseer1)
  - bears on **Spec claims disproved by measurement**: adds to it (the :27 row's disposition, owed at Validate, given)
- inputs#I10 · ../additional/andromedaV3:andromeda-overseer/docs/design-slots/masters-classify-pilot-2026-10-15/rulings.md · pointer committed@f4dcb218 · unchanged — the founder's own ruling on fork CARRY 7 / 8, option 1 (`rulings.md:699`, «Вариант 1 — утвердить», given by dialog at his wake on 2026-10-26)
  - bears on **Decisions & corrections** (fork 1): supersedes «the overseer's PROVISIONAL delegate answer … standing only until the founder's own word» — option 1 is the founder's own word
- inputs#I11 · message · copy · n/a — the fork-1 recording answer (the overseer, 10:22Z): the masters record fork 1 as founder-ratified, verbatim; PROVISIONAL dropped; directive item 6 (`inputs#I7`) superseded on that item
  - bears on **Decisions & corrections** (fork 1 and the P5 card's word): supersedes «held as such through P5's re-pin» and «PROVISIONAL on fork 1»
- inputs#I12 · message · copy · n/a — the P7.1 basis question's answer (the overseer, founder-delegated, 11:18Z): the parent pre-push loop on `6bdfdda8` in a clean worktree, up to 8 runs; read 8 of 8 green, no coverage-merge refusal — no basis (`.andromeda/runs/2026-10-26T09-14-57-wrap/basis-loop.md`)
  - bears on **Outcome** (the `pre-push` gate line): adds to it — the light gate's first firing at this wrap (entry 21) read red (`llvm-cov-exit-1`, one cut-short `viola-fake-agent` profile), every test passing; no basis, so the red was this chunk's
- inputs#I13 · message · copy · n/a — the operator's decision on that red (the overseer's technical answer, 11:34Z): it folds into this chunk and goes back to implement, the fix taking the fake agent's exit out of the hangup that cut its profile, as Epoch 5 did for its hooks
  - bears on **Deviations from intent**: adds to it — a second folded test-only fix, below
- inputs#I14 · message · copy · n/a — the operator's implement re-entry invocation, 11:36Z: the fix, its gates, the operator pass and the CI read, this wrap's uncommitted P1–P6 writes left as they are. Its run: `.andromeda/runs/2026-10-26T11-36-36-implement/outcome-2026-10-26-server-verification-before-any-frame.md` (green + smoke; operator pass complete, CI green)
  - bears on **Files**: supersedes «18 changed» — 20 changed (`cites.py added` re-fired at this resume: `new text: 20 file(s) · 1 new`, written aside to `new-text-resume.md`), adding `src/bin/viola-fake-agent.rs` and `tests/cli_fake_agent.rs`, each recorded in-intent on `inputs#I13` in `scope-record.md`
  - bears on **Symbols / APIs**: adds to it — `src/bin/viola-fake-agent.rs` `fn ignore_hangup()` (1090-1099, `new-text-resume.md`), called at both of `read_stdin`'s exits (1113, 1123): on Unix SIGHUP set to `SIG_IGN` from the exit's first step (`\x03` or stdin's end), before `close_hooks`; test-only (`fake-agent`)
  - bears on **Harness / gate surface**: adds to it — the fake agent's exit behaviour changes (test-plan.md:670 states its exit and its hooks' own process groups, and names no SIGHUP disposition); new case `tests/cli_fake_agent.rs::fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` (925-978, Linux only, with `ignores_sighup` 894-905 and `wait_while_running` 907-923)
  - bears on **Deviations from intent**: adds to it — the mechanism differs from `inputs#I13`'s wording («the fake agent's own exit taken out of the PTY's foreground group»): the agent is its PTY's session leader (portable-pty `setsid` + `TIOCSCTTY`), which `setpgid` refuses, so its exit ignores SIGHUP instead; the `--exit-no-eof` holder inherits the disposition; red-before-green, 40/40 stress and two profile censuses (200 runs, 801 profiles, 0 refused; 40 runs, 120 profiles, 0 refused) in `evidence/fake-agent-exit-hangup.md`; the original cut-short profile was never reproduced on demand
  - bears on **Outcome**: adds to it — the implement block (trail `.andromeda/runs/2026-10-26T11-36-36-implement/gate-2026-10-26-server-verification-before-any-frame.json`): `entries 24 · green 19 · red 1 (13) · recorded 1 · timeout 0 · not-run 3`, the guard reading `guard: outside 3 files` (`tests/contract_fake_agent_drift.rs`, `src/bin/viola-fake-agent.rs`, `tests/cli_fake_agent.rs`, each recorded in-intent); `pre-push` green at `linux-tests`, 91.35 s
- inputs#I15 · message · copy · n/a — the push form (the overseer, founder-delegated, 11:46Z): plan entry 23 fired as written from a clean worktree of the fix commit, the main tree with this wrap's writes untouched, the worktree removed after
  - bears on **Outcome** (the operator entries): adds to it — fix commit `7948850a` (the two fix files only), pushed `775641fa..7948850a`; CI on `7948850acd4b835b23f0504ff77a53482c4d14b0`: `verdict: green · checks 15/15 · wall 841 s · runs ci#40318429755 completed/success`, `run_attempt` 1, the new case `PASS` on `test (ubuntu-latest)`, 0 FAIL (`evidence/operator-pass.md` §7) — the final HEAD's run, superseding ci#40295857215 as the record P7 re-verifies
- inputs#I16 · message · copy · n/a — the operator's resume of this wrap at P7 step 0, carrying the overseer's ruling on its `the listing differs` halt: the difference is the two folded test-only fixes (ETXTBSY, the fake-agent exit hangup, `7948850a`, CI green), taken in through this addendum, the wrap continued; the wrap directive `inputs#I7` still stands
  - bears on **Outcome**: adds to it — P7 step 0 read `the listing differs … First differing line 54, now: ### src/bin/viola-fake-agent.rs`; the only difference is the fake-agent fix's two files (`new-text.md` against `new-text-resume.md`)
- inputs.py verify, re-fired: `inputs: 16 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 13 · uncited 9 · unparsed 0`; moved since the report: entries 8 → 16, uncited 5 → 9 (I8–I16, each cited by this addendum)
## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 6bdfdda8 (the parent of the oldest pre-CI commit 4aa9def9) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-channel/src/client.rs — added 37 line(s) in 4 range(s)
added: 12 · 57-62 · 128-140 · 562-578
  - 57-61 @59 «pub fn server_pid(&self) -> Option<u32> {»
- 132-139 @133 «pub(crate) fn server_pid_of(stream: &impl StreamCommon) -> Option<u32> {»
  - 562-569 @565 «fn client_server_pid_is_the_serving_process() {»
  - 571-577 @573 «fn client_server_pid_is_none_where_peer_credentials_hold_no_pid() {»
### crates/viola-channel/src/lib.rs — added 1 line(s) in 1 range(s)
added: 20
### crates/viola-channel/src/tokio_client.rs — added 40 line(s) in 3 range(s)
added: 18 · 44-48 · 178-211
  - 44-47 @45 «pub fn server_pid(&self) -> Option<u32> {»
  - 178-193 @180 «fn tokio_client_server_pid_is_the_serving_process() {»
  - 195-210 @197 «fn tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid() {»
### crates/viola-mcp/src/tools.rs — added 27 line(s) in 8 range(s)
added: 4-6 · 24 · 28 · 320-322 · 334 · 337-339 · 341-344 · 348-358
  - 341-343 «if !server_verified(live, client.server_pid()).await? {»
- 348-357 @350 «async fn server_verified(live: LiveEndpoint, observed: Option<u32>) -> Result<bool, McpError> {»
  - 351-353 «let verdict = tokio::task::spawn_blocking(move || {»
### crates/viola-mcp/tests/handlers.rs — added 106 line(s) in 4 range(s)
added: 4-5 · 61-69 · 125-173 · 358-403
- 128-136 @131 «fn handlers_held_child_entry() {»
  - 132-134 «if std::env::var_os(HELD).is_none() {»
- 142-155 @143 «impl Held {»
  - 144-154 «fn spawn() -> Self {»
- 157-163 @158 «impl Drop for Held {»
  - 159-162 «fn drop(&mut self) {»
- 165-172 @166 «fn text(result: &CallToolResult) -> String {»
  - 167-171 «result»
- 359-403 @364 «fn handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper() {»
  - 366-381 «let wrapper = booted_recording(»
  - 383-400 «for (name, args) in [»
### crates/viola-state/src/liveness.rs — added 189 line(s) in 6 range(s)
added: 63-69 · 72-82 · 86-89 · 95-159 · 243-245 · 251-349
- 63-68 @64 «fn recorded_secs(started_at: &str) -> Option<u64> {»
  - 65-67 «DateTime::parse_from_rfc3339(started_at)»
  - 72-73 «recorded_secs(&snapshot.started_at)»
  - 95-99 «Some(LiveEndpoint {»
- 102-111 @104 «pub enum ServerVerdict {»
- 113-140 @116 «pub fn server_verdict(»
  - 123-125 «if !peer_pid_reported {»
  - 126-128 «let Some(recorded_start) = recorded_secs(started_at) else {»
  - 129-131 «let Some(observed_pid) = observed_pid else {»
  - 132-134 «if observed_pid != recorded_pid {»
  - 135-139 «match observed_start {»
  - 149-152 «let observed_start = match observed_pid {»
  - 153-159 «server_verdict(»
  - 310-328 «fn server_verdict_rows(»
  - 330-348 @333 «fn verify_server_reads_the_observed_start_time_from_the_os() {»
### src/cmd/answer.rs — added 26 line(s) in 8 range(s)
added: 6-8 · 20 · 22 · 114-120 · 128 · 130 · 148-149 · 155-164
  - 114-118 «let unreachable = |cause| {»
  - 156-164 @160 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
### src/cmd/client.rs — added 279 line(s) in 16 range(s)
added: 3-5 · 11 · 14 · 75-142 · 144-145 · 148-155 · 195-197 · 208-209 · 215-218 · 227-325 · 376 · 380-381 · 383-384
       388-389 · 391-395 · 399-474
- 75-82 @79 «pub(crate) enum Unreachable {»
- 84-108 «impl Unreachable {»
  - 85-91 @86 «fn detail(self) -> &'static str {»
  - 93-99 @94 «fn during(self) -> &'static str {»
  - 101-107 @102 «pub(crate) fn hint_cause(self) -> &'static str {»
- 110-116 @113 «pub(crate) enum CallError {»
- 118-122 «impl From<ChannelError> for CallError {»
  - 119-121 «fn from(error: ChannelError) -> Self {»
- 124-139 @127 «pub(crate) fn connect(live: &LiveEndpoint) -> Result<Client, CallError> {»
  - 129-134 «let verdict = verify_server(»
  - 135-138 «match verdict {»
- 227-324 @232 «pub(crate) mod refused_inside {»
  - 247-271 «impl<S> Layer<S> for Seen»
  - 277-282 «impl Dispatch for Frames {»
  - 284-323 @287 «pub(crate) fn assert_refused_inside(»
  - 403-415 «impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Lines {»
  - 434-473 «fn unreachable_logs_its_cause(»
### src/cmd/hook.rs — added 227 line(s) in 21 range(s)
added: 24 · 29 · 172 · 174 · 178-217 · 229 · 267 · 277-278 · 284-285 · 287-290 · 355 · 531-533 · 540-542 · 772-773
       777-787 · 800 · 908-999 · 1460-1482 · 1484-1518 · 1521 · 1523
- 181-187 @184 «enum NotSent {»
- 189-197 «impl NotSent {»
  - 190-196 @191 «fn detail(self) -> &'static str {»
  - 206-207 «let client =»
  - 208-213 «let verdict = verify_server(»
  - 214-217 «match verdict {»
  - 540-542 «client»
  - 908-917 @911 «fn an_earlier_start() -> String {»
  - 919-949 @924 «fn ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper(»
  - 951-998 @957 «fn hook_refuses_an_unverified_server_inside_hook_handle() {»
  - 1464-1469 «impl viola_channel::Dispatch for Notified {»
### src/cmd/last.rs — added 28 line(s) in 7 range(s)
added: 14 · 16 · 33-39 · 45 · 49 · 61-62 · 82-96
  - 33-37 «let unreachable = |cause| {»
- 83-96 @85 «mod tests {»
  - 88-95 @91 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
### src/cmd/pause.rs — added 37 line(s) in 9 range(s)
added: 4-6 · 16 · 18 · 44-48 · 54 · 59-65 · 70 · 74 · 89-105
  - 59-63 «let unreachable = |cause| {»
  - 98-105 @101 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
### src/cmd/release.rs — added 16 line(s) in 5 range(s)
added: 4-5 · 15 · 17 · 73-74 · 80-89
  - 81-89 @85 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
### src/cmd/send.rs — added 22 line(s) in 9 range(s)
added: 21 · 23 · 168-170 · 218-219 · 227 · 229 · 250 · 252-253 · 269-278
  - 270-278 @274 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
### src/cmd/wait.rs — added 24 line(s) in 8 range(s)
added: 14 · 16 · 46-52 · 67 · 72 · 85 · 89-90 · 118-127
  - 46-50 «let unreachable = |cause| {»
  - 118-126 @122 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
### src/human/hints.rs — added 10 line(s) in 4 range(s)
added: 24-25 · 50-52 · 84-87 · 116
### tests/contract_fake_agent_drift.rs — added 18 line(s) in 6 range(s)
added: 18 · 571-583 · 585 · 588 · 595 · 597
  - 571-583 @575 «let planted: Vec<(String, TestHome, PathBuf, Vec<u8>)> = recorded_versions(&root)»
### tests/hook_fail_open.rs — added 133 line(s) in 11 range(s)
added: 6-9 · 151-252 · 290 · 305-306 · 310-311 · 314-315 · 325-328 · 401-408 · 423-424 · 428-429 · 436-439
- 161-167 @162 «impl Drop for Held {»
  - 163-166 «fn drop(&mut self) {»
- 174-180 @175 «impl viola_channel::Dispatch for Frames {»
  - 176-179 «fn dispatch(&self, method: &str, _: &Value) -> Result<Value, viola_channel::ProtocolError> {»
- 182-190 @185 «struct Impostor {»
  - 194-248 «fn serve(home: &Path, label: &str) -> Self {»
  - 250-252 «fn frames(&self) -> Vec<String> {»
  - 325-328 @326 «if let Some(impostor) = &impostor {»
  - 436-439 @437 «if let Some(impostor) = &impostor {»
### tests/security_negatives_server_identity.rs — new file · 296 line(s)
- 26-27 «const HINT: &str =»
- 35-46 «impl Held {»
  - 36-45 «fn spawn() -> Self {»
- 48-53 «impl Drop for Held {»
  - 49-52 «fn drop(&mut self) {»
- 59-73 «impl Dispatch for Frames {»
  - 60-72 «fn dispatch(&self, method: &str, _: &Value) -> Result<Value, ProtocolError> {»
- 75-83 @77 «struct Served {»
- 85-130 «fn served(label: &str, recorded: u32) -> Served {»
  - 91-99 «let endpoint = if cfg!(windows) {»
  - 101-103 «let serving = Server::bind(&endpoint)»
  - 104-120 «let snapshot = InstanceSnapshot {»
  - 123-129 «Served {»
- 132-146 «impl Served {»
  - 133-135 «fn frames(&self) -> Vec<String> {»
  - 137-145 «fn cli_lines(&self) -> Vec<Value> {»
- 148-166 @149 «fn verb(name: &str, json: bool) -> (Vec<&'static str>, Option<&'static str>) {»
  - 150-161 «let (mut args, stdin): (Vec<&'static str>, _) = match name {»
  - 162-164 «if json {»
- 168-173 @169 «fn digit_runs(text: &str) -> Vec<&str> {»
  - 170-172 «text.split(|c: char| !c.is_ascii_digit())»
- 175-263 @193 «fn security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper(»
  - 204-217 «if json {»
  - 221-224 «let exits: Vec<&Value> = lines»
  - 232-235 «assert!(»
  - 238-244 «let log = std::fs::read_to_string(»
  - 247-261 «for (what, text) in [»
- 265-289 @274 «fn security_negative_cli_verb_reaches_the_recorded_wrapper(#[case] name: &str) {»
  - 276-280 «for json in [true, false] {»
  - 282-288 «assert!(»
- 291-296 @293 «fn digit_runs_reads_whole_numbers_only() {»
