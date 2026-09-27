# Scope — 2026-09-27-wrapper-channel

**Working entry (working-route.md:40, Epoch 2 — Windows slice I: wrapper, events, ledger):** Wrapper channel —
JSON-RPC 2.0 ndjson over the per-instance named pipe (Unix: per-user socket directory), SQOS-opened client,
v/sender/conn in every frame, MAX_FRAME, newer-peer refusal.

## What it builds
- A new workspace crate `crates/viola-channel` (sync, std threads; no tokio at all in this chunk — the Tokio client
  behind a feature is the MCP chunk's) holding the JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde
  (architecture §Established Decisions [API Style]; §Standard Contracts "Wrapper channel frames").
- Framing: one frame per line; every external reader bounded by `Read::take(MAX_FRAME)` (16 MiB, `viola-core`) before
  `read_line`; serde_json default depth; a frame over `MAX_FRAME` is refused, never truncated-and-parsed.
- Envelope fields: every request/notification `params` carries `v` and `sender` (`CARGO_PKG_VERSION`), and the client
  adds the obs-plan additive `conn` = `"<process>-<pid>-<t0>-<n>"` (obs-plan §3 D-10); a server receiving a frame
  without `conn` logs `srv_conn = "srv-<accept counter>"`.
- Newer-peer refusal: a request whose `v` is newer than the wrapper supports gets `-32602` "unsupported protocol
  version" with `data: {supported, wrapper}` (architecture §Cross-cutting Mixed-version tolerance; the frame example).
  Readers skip unknown fields (never `deny_unknown_fields`).
- The per-instance endpoint: Windows named pipe (interprocess 2.4.4 local sockets), Unix per-user socket directory
  (verified: security-plan Decisions Log amendment 2, folded "before the `viola-channel` chunk" per §Bootstrap
  phases; architecture §Occupied Resources still registers `$TMPDIR/viola-<h12>.sock` — an Expected amendment).
  `viola run` binds it, serves it on std threads, and writes `endpoint` into `snapshot.json` (architecture §Instance
  snapshot: "`endpoint` is written once 'Wrapper channel' binds one").
- The exclusive endpoint bind is the ARBITER of two concurrent starts of one name — "two concurrent starts cannot both
  win" (architecture); the loser exits per the start-order refusal (security-plan: a squatted name blocks startup,
  exit 1).
- The Windows client open with SQOS (`SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`), never interprocess's
  default connect (security.md §Local trust boundary).
- The harness surfaces the endpoint lands (added at P5 validation-1, intent-incomplete: test-plan §3 `boot`
  Readiness "`endpoint` joins with Wrapper channel" and `cleanup` step 4 / `endpoint_gone` "null until their
  surfaces exist" both name this chunk): `boot` readiness requires the snapshot `endpoint`, and `cleanup` reports
  `endpoint_gone` from an unconnectable recorded endpoint. The CLI exit-21 half waits for the first connecting verb.

## Boundaries (not this chunk)
- Method semantics: `send`, `wait`, `last`, `answer`, `pause`/`release`, `link`/`unlink`, `hook.dialog` belong to
  Epochs 3–4 chunks. `hook.event` normalisation belongs to "Hooks to normalised events". This chunk ships the
  transport, the envelope, the version handshake and a dispatch table whose unknown/unimplemented methods answer
  JSON-RPC `-32601` (verified: architecture §Established Decisions [Message Broker / IPC] — unimplemented methods
  answer `-32601`); no method is needed to witness the channel end to end — a `-32601` / `-32602` round trip over a
  real endpoint yields both `channel-request` and `channel-response` lines with `corr`.
- Windows endpoint admission (protected SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, `accept_remote(false)`) and
  "Server verification before any frame" (pid + start time vs snapshot) are Epoch 6 route entries.
  [premise-corrected: interprocess 2.4.4's local_socket listener already defaults `accept_remote:false`
  (PIPE_REJECT_REMOTE_CLIENTS), `inheritable:false` and FIRST_PIPE_INSTANCE; only the DACL is open — `None` → the
  default pipe DACL (read to Everyone/anonymous) — research.md §Measured facts] The listener this chunk binds takes
  the security-plan SDDL NOW (one `ListenerOptionsExt::security_descriptor` call + a user-SID lookup), so no
  listener ever binds wider than the security-plan's admission set and no boundary widening arises (overseer
  directive 2026-09-27, item 2). Server verification before the first frame stays Epoch 6's: this chunk's client
  has no product caller that sends a frame yet (the first is Epoch 3's `send`).
- Unix endpoint hardening (0700 dir verify, peer euid) is Epoch 7's; this chunk binds the Unix path so CI's Unix legs
  compile and run the channel.

## Folded freight (working-route.md:40 — 11 CARRY blocks, each a HYPOTHESIS re-verified at P3)
1. **Sync-crate tokio ban** (from 2026-09-24-supply-chain-and-workflow-gates): `viola-channel` joins
   `scripts/sync-crates.txt` (verified present: viola-core, viola-pty, viola-agent-claude, viola-state) and CI job 3's
   `cargo check`, built without its `tokio` feature.
2. **`corr` required** (from 2026-09-24-diagnostics-plane): this first corr producer makes `corr` `required` in
   `schemas/diag-line.v1.json` (verified present) for every corr-bearing event, with a negative test: a corr-bearing
   line without `corr` is rejected. [premise-corrected: "every channel-*, dialog-*, hook-*, send-*,
   release-from-driver line" is wider than obs-plan allows — obs-plan documents a null `corr` for a notification's
   `channel-request` (:749, `hook.event` has no id), the client-side `send-refused{side:"client"}` (:663/:723) and
   `hook-invoked` for a non-dialog hook (:727); and a `-32700` / oversize `-32600` response has no request id
   to copy] `corr` becomes required on exactly the lines whose `corr` obs-plan always defines — unconditionally on
   `dialog-raised` / `dialog-answered` / `release-from-driver` / `hook-decision`, and under a schema discriminator on
   `channel-request` (a method other than `hook.event`), `channel-response` (an `error_code` other than `-32700` /
   `-32600`) and `send-*` (`side:"wrapper"`) — each with its negative case and its null-case positive.
3. **First redaction subjects** (from 2026-09-24-log-redaction-and-never-log-floor): `veil` 0.3.0 in
   `[workspace.dependencies]` without `toggle`, `#[derive(Redact)]` on the first payload types (obs-plan §8),
   `#[instrument(skip_all, name = "<area>.<operation>", fields(..))]` on the first spans (obs-plan §4), and a
   fixed-`Display` `ChannelError`. Verified: 0 `#[instrument` in product crates at HEAD (re-derived:
   `grep -rn '#\[instrument' --include=*.rs src crates | grep -v viola-e2e` → 0).
4. **Framing property + fuzz** (from 2026-09-24-quality-gates): ndjson framing at `MAX_FRAME` ± 1 gets a proptest
   property (`cases: 512`, committed `proptest-regressions/`), a `fuzz/fuzz_targets/<parser>.rs` target and a seeded
   `fuzz/corpus/<parser>/` (test-plan §6 Property suite). Verified: `fuzz/fuzz_targets/` holds only `viola_name.rs`.
5. **SQOS client** (from 2026-09-25-security-prerequisites): reuse the open pinned in `tests/channel_sqos_open.rs`
   (verified present) — windows-sys `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION |
   FILE_FLAG_OVERLAPPED)` adopted by interprocess `local_socket::Stream::try_from`; `FILE_FLAG_OVERLAPPED` required
   (measured at that chunk: a non-overlapped adopted handle hangs). interprocess and windows-sys (Windows only) become
   `viola-channel` dependencies. Land the viola-client `security_negatives_*.rs` SQOS case (test-plan §6 Security
   control negatives → Windows client SQOS). The choice between the windows-sys open and the safe-Rust std
   `OpenOptions::security_qos_flags` + `custom_flags(FILE_FLAG_OVERLAPPED)` open (measured at level 1 too) is this
   chunk's — P4.
6. **E2 Unix half** (from 2026-09-25-pty-wrapper-on-windows): the fake agent's `fds` receipt shows only 0/1/2 plus
   the PTY slave once the channel handle exists; Windows non-inheritance stays the `bInheritHandles = 0` unit fact.
7. **First spans — `pty.spawn`** (from 2026-09-25-pty-wrapper-on-windows): land the `run` start-sequence `pty.spawn`
   CLIENT span (`pty_backend`, `env_stripped_count`) and decide where the `viola-pty` seam-operation spans live
   (root-emitted, or a `tracing` dep on the seam; obs-plan §3 lists `tracing` for `viola-pty`).
8. **Endpoint + arbiter + Scenario 1 spans** (from 2026-09-27-instance-state-and-start-order): write `endpoint`, make
   the exclusive bind the arbiter of two concurrent starts of one name, and add the obs-plan §4 Scenario 1 spans
   `run.start` / `run.collision_check` / `run.pin_copy` / `state.snapshot_write` / `state.heartbeat_start` beside
   `pty.spawn` (obs-plan proposal O1 rejected to this CARRY).
9. **viola-pty resize flake watch** (from 2026-09-27-instance-state-and-start-order): `viola-pty
   tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` red once — run 36296402785, job
   108555954166, `test (windows-2025)` under llvm-cov, `lib.rs:1099` (that run's line; the test fn is at
   `crates/viola-pty/src/lib.rs:1116` at HEAD), key after a ConPTY resize to 120x40 never
   reported within 10 s; cause unobtained (that chunk's `evidence/ci-red-36296402785.md`, verified present). Owner:
   this chunk's /implement — match any recurrence to this test name; on a recurrence force the resize-then-key window
   open with a test-only hold (testing.md 2026-09-27) instead of sampling it.
10. **Windows coverage `test` stage in `pre-push`** (from 2026-09-27-instance-state-and-start-order): the local
    `pre-push` (verified: `crates/viola-e2e/src/harness/pre_push.rs`, not a script) runs the Windows mutation leg
    only; add a Windows llvm-cov `test` stage (overseer direction at that wrap). Runner-speed timeouts stay CI-only —
    `pre-push`'s document says so.
11. **`replace_private_shared` recurrence watch** (from 2026-09-27-instance-state-and-start-order): a fix BY
    REASONING for the one red of `viola-e2e::harness_lifecycle
    harness_session_boots_reports_logs_and_tears_down` (concurrent `overseer` + `builder` boot into one home).
    [inferred] Hypothesis as carried (unmeasured, no captured chain — not re-derivable at HEAD): a Windows
    `MoveFileExW` replace of the shared `bin/<key>/viola` or `plugin/<key>/*` failed while the other start held the
    target open. [premise-corrected: the exclusive-bind arbiter cannot touch this red — it arbitrates two starts of
    ONE name, while that boot was two names (`harness_lifecycle.rs:44`, `["builder", "overseer"]`), whose endpoints
    differ; and by architecture's start order the bind comes after the pin copy and plugin write] This chunk keeps the
    recurrence watch only: match any recurrence to this test name; a new red with a captured chain re-opens it.
    `replace_private_shared` stays the only mechanism against the carried hypothesis.

## CI read at take-up (Setup 5a)
- Base = the last master flip, 17c99c9 (HEAD; range = 1 sha). Run 36302088443 on
  17c99c9cd098256405af2b9312dddbc740fce14b: **green**, 15/15 checks completed success, wall-clock 3m23s
  (07:06:34Z → 07:09:57Z). Nothing to fold.

## Directive standing for this chunk (overseer, founder-delegated, 2026-09-27)
- CI reads in the plan's operator entries: a check that completes FAILED ends the read red at once; a green read
  waits for the run to complete (plan-template leg paragraph) — never the older "read once every run has completed"
  note.
- Boundary widening (security playbook never-routine class) HALTS in the wrap dialog for a live answer, never resolved
  from an earlier recorded direction.
- Sizing: P5 reports plan line + step counts and names a clean split seam if one exists; taken whole by default.
