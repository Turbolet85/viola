# Scope — 2026-09-29-verify-stamped-test-homes-and-harness · Verify-stamped test homes and harness

**Source:** `viola-0.1.0/working-route.md:59` (Epoch 2b — Windows slice I b: events and ledger), taken up 2026-09-29.
**Working entry (verbatim title + hint):** Verify-stamped test homes and harness — test homes and harness boot stamped
through viola verify, recorded-fixture readiness, contract ledger probes, local-live run, fake agent at the recorded
version

## Take-up direction (overseer relay, 2026-09-29, at this chunk's `/andromeda-phase` invocation)
- "CI on 31dd995 is RED, run 36532038635, test windows-2025: hook_fail_open case_08_unreachable_endpoint at
  tests/hook_fail_open.rs:238, events [hook-invoked, channel-request, hook-decision] where [hook-invoked,
  hook-decision] was expected. 933/934 passed, first seen." Setup 5a's read agrees (§CI verdict below). The job log
  is saved at `.andromeda/runs/2026-09-29T06-43-45-phase/ci/job-109287650444.log`.
- "Fold it at P1 as an [inferred] item with a witness." This is the operator's word to fold the red into this chunk,
  given before the intersection question was asked. The red intersects this chunk's work anyway: the failing case
  boots a wrapper through `StampedHome` + `Wrapper::boot` (`tests/hook_fail_open.rs:192-201`), which is the seam this
  chunk re-plumbs. Folded as §What it builds, item 8.
- "Hypothesis, not a claim: the unreachable endpoint is sometimes reachable on Windows (a pipe-name collision with a
  concurrent test)." Kept verbatim as a hypothesis in item 8.
- "D: had filled from Pulse and has 178 GB free again." Context only. The disk headroom warning from the
  session-start dashboard (3.4 GB) is resolved by the operator; it does not shape scope.

## CI verdict read at Setup (Setup 5a)
- `31dd995` (HEAD, the last wrap's flip, `2026-09-29-h2-conpty-resize-probe`) · **red** · checks 15/15 · first-fail
  +196 s `test (windows-2025)` · wall 254 s · ci#36532038635 push completed/failure · failed 1: `test (windows-2025)`
  (job 109287650444). Suite `coverage` 933 passed / 1 failed:
  `viola::hook_fail_open hook_fails_open_silently_within_the_spine_bound::case_08_unreachable_endpoint`, panicked at
  `tests\hook_fail_open.rs:238:5`, `left: ["hook-invoked", "channel-request", "hook-decision"]`,
  `right: ["hook-invoked", "hook-decision"]` (log lines 514-534). Dispositioned: folded (item 8).

## What it builds
The test homes and the headless harness stop being an interim no-stamp seam. Every stamped test home and every harness
`boot` gets its stamp the one sanctioned way: `viola verify` against the fake agent at the recorded CLI version, never a
hand-written `ledger/stamps.json` (testing.md §Test data). Around that it lands the readiness line, the recorded
fixtures, the ledger contract probes and the local-live run.

1. **`stamped_home` stamps through `viola verify`** (CARRY 1, CARRY 2). The interim seam
   (`tests/support/home.rs:152-168`: `StampedHome { stamped: false }`, whose doc comment says "that verb does not exist
   yet") runs `viola verify` against the fake agent into the test home, and `stamped` reads true only from the verb's
   result.
   - [coordinate re-verified; corrected] The CARRY names "the 5 root test files that use it,
     `grep -rlE 'stamped_home|booted_wrapper' tests`". At HEAD that grep lists 4 root test files (`channel_endpoint.rs`,
     `cli_fake_agent.rs`, `cli_instance_state.rs`, `tui_channel_fds.rs`) plus `tests/support/{home,mod}.rs`. A wider
     grep for the type and the boot (`StampedHome|Wrapper::boot`) adds `cli_version_gate.rs`, `hook_events.rs` and
     `hook_fail_open.rs`. P3 fixes the real consumer set, and which consumers need a stamped home and which need an
     explicitly unstamped one.
   - Some consumers assert unstamped behaviour on purpose. The seam therefore needs both a stamped and an
     explicitly-unstamped form, not one flip. [verified at P3: four literal `StampedHome { … stamped: false }`
     constructors, `cli_fake_agent.rs:630`, `cli_version_gate.rs:61`, `hook_events.rs:26`, `hook_fail_open.rs:192`;
     `cli_version_gate.rs` asserts the unstamped `cli_verified:false` path; research.md §Graph impact]
   - [P3 finding] Moving the fake agent's default version (item 6) reaches tests that write their own fixtures under
     the old default (`hook_events.rs:38`, `cli_instance_state.rs:368`) or assert the default `--version` answer
     (`run_cli.rs:538`, `tui_passthrough.rs:193,249`, `cli_fake_agent.rs:199`, `cli_version_gate.rs:76-115`). They
     ride the modify list.
2. **Harness `boot` stamps** (CARRY 1, CARRY 3). `scripts/agent-run.{sh,ps1}` `boot` gains its step 4 (stamp through
   `viola verify`), a `--unstamped` opt-out, a `verify-failed` typed outcome, and readiness line 3: `events.ndjson`
   line 3 is `session-start{source:"hook"}` (test-plan §3 `boot` readiness). [verified at HEAD: `agent-run.sh` carries
   none of `--unstamped`, `verify-failed`, `--local-live`, `live-in-ci` or `--cli-version`]
3. **`supervise --fixtures <root>/fixtures/claude`** (CARRY 1, CARRY 3). The harness `supervise` passes the recorded
   fixture root, so the fake agent replays recorded payloads. The first recorded set exists:
   `fixtures/claude/2.1.283/{SessionEnd,SessionStart,Stop,UserPromptSubmit}.default.json` (committed at `01f22aa`,
   2026-09-28). [verified at P3: the fake agent fires `SessionStart.default` through the run's plugin hooks only when
   `--fixtures` is passed (`src/bin/viola-fake-agent.rs:213-222`), and `supervise` passes none today
   (`crates/viola-e2e/src/harness/supervise.rs:55-60`). With `--fixtures`, record three is `session-start{source:
   "hook"}`, as `tests/cli_instance_state.rs:364-375` proves over a test-written set. The committed set holds
   `SessionStart.default.json`.]
   - CARRY 3's origin: chunk `2026-09-27-hooks-to-normalised-events` proved line 3 and the M6 witness only through root
     integration tests with fixtures the test writes (its P4 fork 3, operator ruling). The harness path lands here.
4. **`tests/contract_ledger_probes.rs` over committed sets** (CARRY 1). The capability-ledger rows' probes and
   post-conditions run against the committed fixture sets. [verified at HEAD: no such file exists; new]
5. **`agent-run run --local-live`** (CARRY 1) with its `live-in-ci` refusal: a run against the real `claude` on the
   operator's host, refused under CI. **Claims `v1-14`** per the CARRY. [verified at P3: `matrix.py show --id v1-14`
   reads unclaimed, `method: integration`, acceptance "CI test homes are stamped only by viola verify run against the
   fake agent; the real claude CLI never runs in CI; the local-live mode refuses to start under CI" — items 1, 2 and 5
   together]
   - `--local-live` does not claim the H2 product measurement. The handoff pins that to "First live test and
     self-drive". The `2026-09-29-h2-conpty-resize-probe` arch wording "owned by the real-CLI verify entry" is read as
     that entry (research.md §Open questions).
6. **The fake agent's default version and `boot`'s `--cli-version` move to the recorded one** (CARRY 1), `2.1.283` as
   recorded today. [verified at P3: one recorded set (`fixtures/claude/2.1.283/`). The default lives in two
   constants, `src/bin/viola-fake-agent.rs:18` and `crates/viola-e2e/src/harness/boot.rs:21` (used by
   `viola-harness.rs:35` and `run/perf.rs:164`); `supervise.rs:58` already forwards `--cli-version`]
7. **Two code-quality folds from the capability-ledger wrap** (overseer rulings at that wrap):
   - (CARRY 4) `StampError` (`Malformed`), a second thiserror enum in `viola-agent-claude`
     (`crates/viola-agent-claude/src/ledger.rs:226`) beside `AgentError` (`src/hook.rs:64`), folds into `AgentError`
     (architecture [Error Handling]: the one-enum rule stands). [verified: both enums at those lines]
   - (CARRY 5) `viola verify`'s bounded `--version` read and print-mode probe write no `process-start` / `process-exit`.
     Log the `version-probe` pair for the read, and a probe-child pair under a new `subject` value for the probe. That
     value is a `diag-line.v1.json` amendment (obs-plan §6 Child / shell spawns; the spec stays right).
     [coordinate corrected: the CARRY places `run_bounded` in `src/cmd/verify.rs`. It is defined at
     `src/run/version_gate.rs:39` and called from `src/cmd/verify.rs:112` (version) and `:127` (probe). `run`'s own
     version gate shares it. P3 decides whether the log lines go in the shared helper or at the verify call sites.]
8. **The CI red on `31dd995`, folded on the overseer's word** (Setup 5a; §Take-up direction).
   - Red: ci#36532038635, sha `31dd9956195dab272778e7aa5237e24eb4412cb5`, failing job `test (windows-2025)`
     (job 109287650444), test
     `viola::hook_fail_open hook_fails_open_silently_within_the_spine_bound::case_08_unreachable_endpoint`, at
     `tests/hook_fail_open.rs:238`: events `["hook-invoked", "channel-request", "hook-decision"]` where
     `["hook-invoked", "hook-decision"]` was expected. 933/934, first seen. [verified at P3 against THAT RUN (a
     runner-only subject, not closed against HEAD): the job log lines 514-534, and the kept home
     `viola-test-B3yvcs` in its `diag-windows-2025` artifact (saved under the phase run dir `ci/`)]
   - [premise-corrected: the endpoint name carries the home (`crates/viola-channel/src/endpoint.rs:29-37`, called with
     the instance home at `src/cmd/run.rs:253`), and the run's own logs show the hook reached its OWN home's endpoint]
     Overseer hypothesis, verbatim: "the unreachable endpoint is sometimes reachable on Windows (a pipe-name collision
     with a concurrent test)." It does not hold. Two unique test homes share a pipe only on a 48-bit hash-prefix
     collision. What the run shows instead:
     - The stopped wrapper wrote `process-exit{subject:"self", exit_code:0}` at 06:39:42.258Z.
     - The test's hook connected to that same home's `\\.\pipe\viola-<h12>` at .283-.284 and delivered its
       `hook.event`. Its `hook-decision` carries no `detail`.
     - So `stop_keep`'s "stopped" (an exit code, read through portable-pty's `GetExitCodeProcess`-only `try_wait`)
       is not "the endpoint is gone" (a client connect reads NotFound, the harness `endpoint_gone` rule).
     - **Why the endpoint outlived the reported exit is NOT established.** Microsoft's documented `ExitProcess` order
       closes handles before the exit status is set, which argues against the obvious reading. That stays an open,
       unmeasured candidate, beside another handle to the instance (research.md §Item 8).
     - `tests/hook_events.rs:362` stops the same way and carries the same premise.
   - Acceptance of its own (the overseer's word): the cause is established, fixed or bounded, and a witness shows it.
     The witness cannot pass vacuously: it must force the colliding condition (for example a live same-named wrapper
     in another home while the stopped home's hook runs) and read red with the fix removed. A green re-run never closes
     it (Session Learnings: a red stays open until its cause is known).
   - [premise-corrected: the hypothesis does not hold, and no cross-home reach occurred] The defect is test-side: the
     stop helper's meaning of "stopped". No local-boundary question arises, because the hook reached the same user's
     wrapper in the same home. One narrow product consequence is recorded and not in scope: a `hook.event` written into
     an exiting wrapper reads as delivered, so `SessionEnd`'s direct-append fallback (`src/cmd/hook.rs:148-150`) is
     skipped. The dated `hook.event` no-verification exception is untouched.

## P4 forks answered (operator + overseer, 2026-09-29; validation-1, intent-incomplete)
- **Item 8's fix and witness:** the fix is test-side. `stop` / `stop_keep` return only once the endpoint is gone
  (connect NotFound, the harness `endpoint_gone` rule), and a deterministic helper witness reads red with the wait
  removed. There are no measurement pushes ("the founder wants fewer CI cycles") and no product seam.
- The overseer, verbatim: "My collision hypothesis is falsified; record that plainly. Note in the product docs that a
  hook firing within ms of wrapper exit can still reach the dying pipe, and that it fails open anyway." → `.claude/docs`
  gotchas and the `viola` service notes. `viola hook` still exits 0 silently. The notification reads delivered, and the
  `SessionEnd` direct append is skipped in that window.
- **`Wrapper::boot` fixtures:** it passes `--cli-version <recorded>` always, and `--fixtures` stays a per-test opt-in
  ("the narrower diff wins"). The harness `supervise` passes `--fixtures`, per test-plan §3.

## Boundaries
- In: `tests/support/home.rs` and every consumer of `StampedHome` / `Wrapper::boot`; `scripts/agent-run.{sh,ps1}`
  (`boot`, `supervise`, `run --local-live`); the fake agent's default version (`src/bin/viola-fake-agent.rs`);
  `tests/contract_ledger_probes.rs` (new); `crates/viola-agent-claude` error enums; `src/cmd/verify.rs` /
  `src/run/version_gate.rs` spawn logging and `schemas/diag-line.v1.json` (via the wrap amendment path). Item 8's closure
  placed the defect test-side, in the stop helper, so endpoint naming stays out (P3, research.md §Item 8).
- Out: new capability-ledger rows or `viola verify` probes owned by later entries (readiness gate, confirmed send,
  dialogs, first live test); the fake agent's hook `matcher` evaluation and its fixture-equality drift contract
  (working-route :61); the `size` receipt wording (:61); the pipe listener's server verification for `hook.event`
  (Epoch 6, founder-dated exception).
- Invariants to keep: `viola hook` fails open (exit 0, silent) on every path; never hand-write `ledger/stamps.json` or
  `snapshot.json`; zero-flake budget (no sleep, no retries); stamps come only from `viola verify` against the fake
  agent.

## Folded freight (every annotation on working-route:59; `route.py pins`: 6 blocks, 0 abstentions)
1. **CARRY (713 chars)** — chunk `2026-09-28-capability-ledger-and-viola-verify`'s plan named this scope (placed by
   the operator at that wrap): `stamped_home` through `viola verify`, harness `boot` step 4 / `--unstamped` /
   `verify-failed` / readiness line 3, `supervise --fixtures`, `contract_ledger_probes.rs`, `run --local-live` with
   `live-in-ci` (claims v1-14), and the fake agent's default version plus `boot --cli-version` moved to the recorded
   one. → items 1-6.
2. **CARRY (253 chars)** — chunk `2026-09-24-fake-agent-and-test-data-fixtures`: the root `stamped_home` is an interim
   no-stamp seam, to stamp through `viola verify` here. → item 1.
3. **CARRY (391 chars)** — chunk `2026-09-27-hooks-to-normalised-events`: harness `boot` readiness line 3 and
   `supervise --fixtures` with the first recorded fixtures. → items 2-3.
4. **CARRY (281 chars)** — `StampError` into `AgentError` (overseer ruling). → item 7.
5. **CARRY (457 chars)** — `version-probe` and probe-child spawn pairs, a `diag-line.v1.json` `subject` amendment
   (overseer ruling). → item 7.
6. **CARRY (987 chars) — WSL `--install-deps` hardening** (chunk `2026-09-27-browser-verdict-reachability`, overseer
   live ratification, operator-only): before the WSL distro is next re-provisioned, `scripts/wsl-provision.sh
   --install-deps` must stop running user-writable code as root. Root would run only `apt-get install` over the list
   an unprivileged `install-deps --dry-run` produced, checked against a committed allowlist.
   **Binds only if this chunk re-provisions.** [verified at P3: research.md's file lists (28 items, `gate.py scope`
   parsed) name neither `scripts/wsl-provision.sh` (last changed `5f0a809`, 2026-09-27) nor ci.yml]
   If the plan comes to touch either, the CARRY binds and becomes a plan task. Otherwise the wrap moves it forward
   with the next markerless entry, unchanged.
