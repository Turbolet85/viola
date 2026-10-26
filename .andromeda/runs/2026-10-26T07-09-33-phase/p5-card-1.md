# P5 review card 1 — 2026-10-26-server-verification-before-any-frame

**Chunk:** Server verification before any frame (Epoch 6 — Security hardening) · plan
`viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md` (456 lines, 15 steps, 10 acceptance
criteria, 24 gate entries) · research `…/research.md` · scope `…/scope.md`.

## Goal
Every frame-writing client — the six CLI verbs `send` · `wait` · `last` · `answer` · `pause` · `release`, `viola hook`'s
`hook.event` and `hook.dialog`, and `viola mcp`'s `send` · `wait` · `last` · `answer` — verifies before its first frame
that the process serving the endpoint is the wrapper `snapshot.json` records (the stream's peer pid equals `pid`, that
pid's OS start time equals `started_at`). A mismatch, or a check that cannot be made, writes no frame: CLI exit 21
(`instance-unreachable`, `detail: null`, the server-verification hint), MCP `instance-unreachable`, hook fail-open
(exit 0, nothing on stdout or stderr). Logged as `server-verify-failed` on `process-exit` / `hook-decision` (already in
the schema's closed enums).

## Top acceptance criteria
- Linux and Windows: an impostor server (a writer-made snapshot recording a live child, the test process serving)
  makes all six verbs exit 21 with the exact document / the hint as the last stderr line, no frame — and the control
  (the snapshot recording the server) reaches it (`tests/security_negatives_server_identity.rs`, new).
- The hook's `stop`, `pre-tool-use` and `permission-request` against the impostor: exit 0, empty streams, `< 1.0 s`,
  `hook-decision{detail:"server-verify-failed"}`, no frame (`tests/hook_fail_open.rs`); a SessionEnd still takes its
  direct append.
- MCP `send` / `wait` / `last` / `answer` against the impostor: `isError` `instance-unreachable`, no frame
  (`crates/viola-mcp/tests/handlers.rs`).
- The pure decision's table (match · other pid · other start time · bad `started_at` · no observed pid · process gone ·
  the no-peer-pid platform passing), the perf rows under `SPINE_DEADLINE`, tokio containment, CI green on all legs.

## Touchpoints
New `tests/security_negatives_server_identity.rs`. Modified: `crates/viola-channel/src/{client,tokio_client}.rs`,
`crates/viola-state/src/liveness.rs`, `src/cmd/{client,send,wait,last,answer,pause,release,hook}.rs`,
`src/human/hints.rs`, `crates/viola-mcp/src/tools.rs`, `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`.

## Test commands (24 entries; baselines on the untouched tree, HEAD `6bdfdda8d374`)
`df -h /tmp` · fmt · clippy · Windows-target clippy · G1 · lint-probes · `run --unit` · the three binaries by name
(the new one red-vacuous, exit 94; `hook_fail_open` 29 and `handlers` 6 green before their new rows) · `cargo deny
check` · `channel-tokio-free.sh` · the preservation guard (controls: clean / clean / red `outside 2 files`) · the
harness smoke, G2, G4, cleanup · `run --perf` (each hook row max 1.23–1.6 ms) · `pre-push` (87.6 s) · the operator
hygiene, push and `ci.py` read (`verdict: green`, every leg). One atom re-spelt at P5: nextest prints no `0 failed`,
so the three binary entries assert `exit 0` + `tests run:`.

## Decisions on this card
- **Fork 1 (CARRY 7 / 8)** — option 1, keep today's outcome: every reader keeps `read_snapshot`; an unreadable or newer
  snapshot reads as absent (exit 21 / `instance-unreachable` / fail open). The overseer's PROVISIONAL delegate answer,
  2026-10-26 08:08Z (`inputs#I3`), standing only until the founder's own word.
- **Leans** (each settled by an artifact): the dated gaps' strict-modes half stays with "Home and code-bearing file
  integrity" (its line's CARRYs); the macOS euid and socket-directory arm stays with "Unix endpoint and home hardening"
  (its line's CARRY); `during:"connect"` on the server cause (obs-plan §6 field list); no MCP log line for the mismatch
  (the closed `event` enum; today's MCP unreachable writes none).
- **Corrected premises** (P3): the "five snapshot readers" list is stale — `src/cmd/client.rs` holds no call; the CLI
  verbs read through `viola_state::liveness::live_endpoint` (an Expected amendment at the wrap); the Windows peer pid
  is read on the SQOS-adopted client end (interprocess source; CI's windows-2025 leg witnesses it).

## Verification matrix — concretization preview
No capability is claimed. After the word, step 4 writes one dated note each:
- `v1-44` — advanced, not proven: this chunk lands client-side server verification before any frame on Linux and
  Windows; the same-user admission on Unix, the macOS euid / socket-directory arm and the macOS pid gap's record stay
  with "Unix endpoint and home hardening" and "Linux and macOS parity".
- `v1-41` — not advanced: the readers' move to the replay stays held for the founder's own word (fork 1, provisional).

## Sampler (forced, the operator's word, `inputs#I1` item 1)
- architecture: `sampler: E carried 66 · the cascade lost 5 (cross-cutting-patterns#1.1, occupied-resources#19.1, occupied-resources#3.1, stack-and-technologies#1.1, standard-contracts#8.1) · listed 2 (established-decisions#18.1, stack-and-technologies#1.15)`
- security-plan: `sampler: E carried 70 · the cascade lost 1 (secret-management#1.1) · listed 3 (bootstrap-phases#2.1, secret-management#2.1, security-anti-patterns#3.1)`
- design-system: `sampler: E carried 9 · the cascade lost 1 (brand-identity#2.2) · listed 2 (brand-identity#2.1, iconography#1.3)`
- test-plan: `sampler: E carried 104 · the cascade lost 13 (ci-integration#1.11, ci-integration#1.2, ci-integration#1.7, ci-integration#1.9, mocking-stubbing-discipline#2.3, quality-gates-coverage-targets#2.0, test-data-fixtures#1.5, test-data-fixtures#1.6 …) · listed 4 (quality-gates-coverage-targets#1.1, test-data-fixtures#1.2, test-scope-summary#7.1, test-strategy#2.1)`
- obs-plan: `sampler: E carried 89 · the cascade lost 8 (ci-integration#2.2, ci-integration#2.5, ci-integration#3.2, error-capture-reporting#2.1, error-capture-reporting#3.1, metric-coverage#2.11, metric-coverage#2.5, obs-scope-summary#45.1) · listed 10 (ci-integration#2.3, ci-integration#3.1, ci-integration#3.3, ci-integration#4.1, pii-scrubbing-compliance#1.12, pii-scrubbing-compliance#1.4, pii-scrubbing-compliance#1.5, pii-scrubbing-compliance#1.8 …)`
- a11y-plan: `sampler: E carried 23 · the cascade lost 0 · listed 3 (cognitive-accessibility#3.1, cognitive-accessibility#4.1, visual-design-verification#1.1)`
- The plan carries every `## Lost` item as `(sampler)`: in Implementation notes (one paragraph naming each), in
  Constraints (secret-management.md:8, ci-integration.md:50, pii-scrubbing-compliance.md:19), in the Test Commands
  prose (quality-gates-coverage-targets.md:15, test-strategy.md:29), in step 14 (test-data-fixtures.md:15) and in the
  G2 entry's note (ci-integration.md:38-39).

## Beyond the plan
1. **CARRY 7 / 8 re-pinned, held for the founder's own word** — the readers' move onto `read_snapshot_or_replay`
   (`verification-matrix.json#v1-41`, test-plan §6 E5 steps 1–3) and what a client does with an unreadable or newer
   snapshot leave this entry as one CARRY on the working-route entry **"Home and code-bearing file integrity"**: the
   classified read is taken only after `check_instance` (security-plan §Input Validation, own state files), the check
   that entry adds before these same readers' reads. The fork stands answered provisionally (option 1, `inputs#I3`)
   until his word.
2. **CARRY 2 re-pinned, unclosed** — a hook reaching an exiting wrapper's own Windows pipe (ci#36532038635), its
   SessionEnd fallback skipped in that window: the check passes there by design (the exiting wrapper is the recorded
   process). The measurement — whether sysinfo still lists a terminated Windows process whose pipe handle is open, and
   SessionEnd's outcome in that window — moves as one CARRY to **"Windows-only live measurements"**.
3. **The macOS server-verification cases** — test-plan §6's macOS exit-cause case (the per-user socket directory
   loosened to 0755 → exit 21) and the macOS client euid check join **"Unix endpoint and home hardening"** as one
   CARRY beside its own directory and euid freight; this chunk's negatives compile out on macOS.
4. **"Home and code-bearing file integrity"'s strict-modes CARRYs** for `send`, `wait` / `last`, `answer`,
   `pause` / `release`, the MCP frame tools and the hook client keep their strict-modes half only: their
   server-identity half is closed by this chunk (the wrap edits their text, the CARRYs stay on that entry).

Apply? (yes / review / cancel)
