# Scope — 2026-09-27-hooks-to-normalised-events

**Working entry:** `viola-0.1.0/working-route.md:47` — "Hooks to normalised events — exec-form plugin hooks by absolute path,
hook-to-kind map, harness/paste/tag normalisation, silent unwrapped no-op, fail-open exit 0, perf-job-gated hook deadlines"
(Epoch 2b — Windows slice I b: events and ledger). It carries six CARRYs, folded below.

**Intent (the val-1 anchor):** the `claude` child's hooks reach viola's event log. A wrapped session's embedded plugin registers
exec-form hook commands by the absolute pinned path; the new `viola hook` verb reads the hook payload, maps the hook to a
normalised event kind (normalising `prompt-submitted` text and origin), and hands it to the owning wrapper, which appends it to
`events.ndjson`. Outside a wrapped session the verb is a silent no-op. Whatever happens, `viola hook` exits 0, writes nothing to
stderr and fails open with no body. Its deadlines are gated by a perf job.

## What this chunk builds

1. **Exec-form plugin hooks by absolute path.** The embedded `plugin/hooks/hooks.json` (today `{"hooks": {}}`, read at P1) is
   filled with one `type:"command"` entry per hook event viola consumes, each `command` the absolute pinned path of the running
   `viola` (never a PATH lookup, security.md), written out atomically by `viola run` on every start. Verified at P3: this chunk
   registers the spine (SessionStart, UserPromptSubmit, Stop), SessionEnd, and the async tier (Notification, PostToolUse,
   PostToolUseFailure → `activity`), per architecture [Hook Transport] (architecture.md:62–66). The dialog tier (PreToolUse
   `AskUserQuestion|ExitPlanMode`, PermissionRequest) needs `hook.dialog`, `dialog_id` and the `question`/`permission`/`plan`
   kinds, all owned by :62.
2. **The `viola hook` verb.** A new clap verb (`src/cmd/` holds only `mod.rs` and `run.rs` today, read at P1). It reads the hook
   stdin under `Read::take(MAX_FRAME)`, parses it in `viola-agent-claude` (the only crate that knows Claude shapes), and maps it
   to a normalised event kind from `viola-core`. The hand-off is the id-less `hook.event` notification to the endpoint recorded
   in `snapshot.json`; the wrapper answers it by appending the event line (`source:"hook"`).
   [premise-corrected: the server already dispatches an id-less `hook.event` and never answers it (server.rs:202–212), but the
   client has only `request` — no id-less send (client.rs:51–80) — and performs no server verification (no
   `GetNamedPipeServerProcessId` / `peer_creds`). Server verification before any frame and home strict-modes are their own route
   entries, "Server verification before any frame" and "Home and code-bearing file integrity" (Epoch 6, working-route.md:88–93),
   while security.md states the rule for `hook.event` now. Whether this chunk lands them ahead of Epoch 6 is a P4 fork.]
3. **Hook-to-kind map and `prompt-submitted` normalisation.** UserPromptSubmit becomes `prompt-submitted{text, origin}`: the
   `<pasted_content id=…>` long-paste wrapper is removed first, then the CLI's tag escaping is reversed (`<\pasted_content` →
   `<pasted_content`), so a literal the user typed is never mistaken for the wrapper (architecture §Standard Contracts, both
   ledger rows). `origin` is `harness` for harness-injected prompts (test-plan names the M2 prefixes `<agent-message from=` and
   `<task-notification>`), else `human`/`driver`. [premise-corrected: `driver` is not decidable here. Architecture ties the
   driver/human split to the wrapper's send window and byte-source claim ([Delivery Confirmation], [Human Takeover / Wheel]),
   and no `send` exists until :58. So the hook classifies `harness` by prefix (in `viola-agent-claude`), and the wrapper assigns
   `human` to every other prompt in this chunk — truthful, since no driver path exists yet. `driver` joins with :58 (route note).]
4. **Silent unwrapped no-op.** A `viola hook` fired in a session that viola does not wrap (no resolvable instance) exits 0 with no
   body, no stderr and no event. Verified at P3: the key is `VIOLA_NAME` (+ `VIOLA_DIR`, whose grandparent is the home), which
   `viola run` sets in the child (src/run/mod.rs:101–106) and the R8 strip never touches (it removes only `CLAUDE*` names,
   viola-agent-claude lib.rs:94); architecture [Hook Contract] fixes "exit 0 at once when `VIOLA_NAME` is absent".
5. **Fail-open exit 0.** Every failure path (bad stdin, unreachable wrapper, verify failure, panic) exits 0 with empty stdout and
   empty stderr (CLAUDE.md universal invariant; exit 2 is forbidden). The panic hook stays the first statement of `main`.
6. **Perf-job-gated hook deadlines.** test-plan §9/§10: `agent-run run --perf` runs hyperfine 1.20.0 (`-N --warmup 3 --runs 30
   --export-json …/perf-<hook>.json`), gated on `max`, per OS in CI; a role-file `event:"panic"` line is a perf breach. Read at
   P1: no perf job, no `--perf` arm and no hyperfine install exist in `ci.yml`, `wsl-provision.sh` or `viola-harness.rs`; only
   `gate.rs` names `perf`. The spine deadline constant is provisional (`max < 1.0 s`, test-plan) until arch names it.
   Verified at P3: `gate.rs` already evaluates every `perf-*.json` on `results[0].max < 1.0` and breaches `artifact-missing`
   without one (gate.rs:192–219), so the verdict needs no product change; test-plan §9 installs hyperfine with
   `cargo install --locked hyperfine@1.20.0` in the job that invokes it. [premise-corrected: test-plan §10's perf session boots
   a *stamped* session (needs `viola verify`, :51) and times a `pre-tool-use` row (needs `hook.dialog`, :62); this chunk can time
   only the spine and SessionEnd rows on an unstamped session. test-plan §9 makes Perf its own per-OS job while obs-plan §10 puts
   it "in the same per-OS job that runs G2/G4/the secret scan" — a P4 fork, together with the overseer's size check.]

## Operator rulings at P4 (2026-09-27, overseer, founder-delegated) — these amend the scope above
- **Split (fork 1): head + perf tail.** Item 6's perf harness and CI half — the `run --perf` arm, the hyperfine 1.20.0 pin, the
  CI perf job and `gate --require perf` — leaves this chunk for a new tail entry, "Hook perf gate", which the wrap inserts
  directly after :47 (so the perf gate is not pushed back). Its first-consumer reason: the hook verb this head lands is the first
  and only subject a hyperfine row can time, and the gate code already reads `perf-*.json` (gate.rs:192–219). The job-shape fork
  (its own per-OS job, test-plan §9, vs inside `test`, obs-plan §10) moves to that entry's phase. This head keeps the spine
  deadline asserted at unit speed: an `Instant` bound in the assert_cmd hook tests (test-plan §10, "The same deadlines are also
  asserted at unit speed inside the assert_cmd hook tests").
- **Server verification (fork 2): deferred to Epoch 6.** The hook sends `hook.event` to the endpoint the wrapper recorded in
  `snapshot.json`, with no server verification and no strict-modes check. CARRYs go on both Epoch 6 entries ("Server
  verification before any frame", "Home and code-bearing file integrity") naming the hook client and its fail-open-on-mismatch
  case. Interim gap, in one line: until Epoch 6 a hook trusts the recorded endpoint (the first-instance pipe and the bind arbiter
  already stop a squatter from taking a live name).
- **Harness readiness (fork 3): deferred to :51.** Lines 1–3 and the M6 witness are proven in this chunk through root
  integration tests with synthetic fixtures the test writes (tests/support/fake.rs `write_fixture`). The harness `boot`
  readiness line-3 check and `supervise` passing `--fixtures` move to :51 (Capability ledger and viola verify), the true first
  consumer, where recorded fixtures land.

## Operator rulings at the P5 review (2026-09-27)
- **No panic seam.** `FAKE_AGENT_HOOK_PANIC` is NOT ratified. Under the boundary-widening rule only the founder's live answer
  ratifies a new seam. This chunk proves the hook's panic path in-process, and does the concurrent-append check without it. The
  forced-panic fail-open case and the seam become a CARRY to the "Hook perf gate" tail, which the overseer puts to the founder
  before that phase.
- **What the concurrent-append check loses without the seam:** a detail line over 4 KiB. The only hook detail line that large
  is the panic payload + backtrace; the drift-report and chain lines are short. That half of obs D-28 goes with the seam to the
  tail.
- **The long-paste wrapper, measured** (the 23:16 UserPromptSubmit in `~/.viola/sessions/viola-builder/events.ndjson`; the
  prototype test `viola-lab/prototype/src/store.rs:241–249`):
  - The CLI's own wrapper arrives UNESCAPED: `\n\n<pasted_content id="2f85">\n…\n</pasted_content id="2f85">\n`. The close has
    a slash and repeats the id.
  - Tag text the user TYPED arrives escaped: `<\pasted_content`, and `<\/pasted_content` for the close.

  So escaping separates the two, as architecture.md:89 states. (A first relay at the review had the forms swapped; the operator
  corrected it.)

## Boundaries (out of scope)
- Dialog answers through hooks (question, permission, plan; `hook.dialog`; decisions) — :62, Epoch 3.
- The capability ledger rows, `viola verify` and recorded fixtures — :51; the fake-agent drift contract and hook `matcher`
  evaluation — :53. Where this chunk relies on an undocumented CLI behaviour, it names the ledger row :51 must probe.
- The CLI's `error: internal error` catch-site print for `cli`-process verbs — :49 (its CARRY). This chunk's `hook` role prints
  nothing.
- Statusline pass-through (the settings.json rewrite, budget readings) — :71. See CARRY 3 for the parser property.

## WSL re-provision: does this chunk take the `--install-deps` CARRY? (overseer question, answered at P1)
**No, as scoped.** Nothing in this chunk's own scope needs a new package in the WSL distro: the hook verb, the plugin file, the
normalisation, the fuzz target and the proptest properties all build and run under the pins `wsl-provision.sh` already installs.
The one candidate is hyperfine: if P4 puts `--perf` into the pre-push Linux leg, the distro needs hyperfine, which is a
re-provision, and then CARRY 6 binds here. The lean is that perf stays a CI per-OS job (test-plan §9 lists it as `per OS`, never
as a pre-push stage). Verified at P3: test-plan §3 `pre-push` `linux-tests` is exactly `run --coverage`, `run --browser`,
`gate --require coverage,doctest,playwright` (test-plan.md:662) — no perf stage — so hyperfine never enters the distro and
**CARRY 6 does not bind here** unless P4 adds perf to pre-push, which the plan does not.

## Folded freight

### CARRY 1 (from 2026-09-24-diagnostics-plane): the first shared writer and the `hook` exit-0 role
- (1) The three-OS concurrent-append check (obs D-28) with a detail line over 4 KiB. Concurrent hook processes are the first to
  share `hook-<name>.ndjson` / `detail-hook.ndjson`.
- (2) The pre-clap argv role classification at the main-thread catch site (obs §7). Today every caught panic exits 1 because
  `run` is the only verb, and `hook` must exit 0. Re-verified at P1: `viola::obs::role_file_name` exists (`src/obs.rs:48`).

### CARRY 2 (from 2026-09-24-log-redaction-and-never-log-floor): the first drift-report producer
Dispatch errors route through `cmd::Failure` (`src/cmd/mod.rs:33`) and `viola::obs::report_internal_error` (`src/obs.rs:242`);
the role line is `run`-only via `internal_error_exit_line` (`src/obs.rs:220`) — all re-verified at P1. This chunk is the first
`serde_path_to_error` consumer: its drift reports go as `drift_report` into `detail-hook.ndjson` (obs-plan §8; the field is in
`schemas/diag-detail.v1.json:23`, re-verified), and its `Failure` sink keeps `hook` at exit 0 with empty stderr.

### CARRY 3 (from 2026-09-24-quality-gates): property and fuzz coverage of the new parsers
The hook stdin parser (statusline JSON with arbitrary `resets_at` included) and the `prompt-submitted` normalisation round-trip
(`<pasted_content` / `<\pasted_content`) each take a proptest property (`cases: 512`, committed seeds), plus a
`fuzz/fuzz_targets/` target and a seeded corpus (test-plan §6 Property suite). Re-verified at P1: `fuzz/fuzz_targets/` holds
`channel_frame.rs` and `viola_name.rs`, with matching `fuzz/corpus/` dirs. [premise-corrected: the statusline JSON is read by
`viola hook statusline`, which belongs to :71 (Statusline pass-through — "readings to budget.json"); this chunk's hook stdin
parser reads hook payloads only. The hook-payload property and the normalisation round-trip land here; the statusline
`resets_at` property moves to :71 as a route note.]

### CARRY 4 (from 2026-09-27-instance-state-and-start-order): fill hooks.json, M6 witness, event line 3
Fill the embedded `plugin/hooks/hooks.json` (re-verified empty) with the absolute pinned-path exec-form commands; land the M6
absolute-command witness through the fake agent's `command_absolute` receipt (re-verified: `src/bin/viola-fake-agent.rs:231–256`,
`tests/cli_fake_agent.rs:308/381`); write the `events.ndjson` line 3 `session-start{source:"hook"}` that test-plan §6 Path 1
expects after `wheel` and `budget-gate` (that chunk writes the first two), with the harness readiness check of those events.
[premise-corrected: filling `hooks.json` alone does not produce line 3. The fake agent never fires SessionStart on its own — hooks
fire only from scripted steps or a prompt submit (viola-fake-agent.rs:268,312) — and it needs a
`<fixtures>/<cli-version>/SessionStart.<variant>.json` payload, while the harness passes it no `--fixtures`
(supervise.rs:53–59) and no `fixtures/claude/` exists (recorded fixtures land with `viola verify`, :51). So the fake agent must
fire SessionStart at start as the real CLI does, root tests supply synthetic fixtures (tests/support/fake.rs:96–105), and the
harness `boot` readiness half needs either committed synthetic fixtures or a deferral to :51 — a P4 fork.]

### CARRY 5 (from 2026-09-27-browser-verdict-reachability): the `snapshot.json` replace fix — overseer direction, P1 fold
- **Measured once, not reproduced (claim kept verbatim):** one wrapper exited `internal-error` (chain `state file i/o failed` /
  `Access is denied. (os error 5)`) and orphaned its fake-agent child while a test read `snapshot.json` during
  `viola_state::fs::replace_private`'s replace on Windows (that chunk's integration gate, report §Insufficient fixes).
- **The fix, its own item with its own acceptance and witness:** `viola_state::fs::replace_private`
  (`crates/viola-state/src/fs.rs:83`; its `persist` is the rename) retries the replace, bounded, when Windows refuses it because a
  reader holds the target open, and still leaves the target as it was on final failure. The caller at risk is `snapshot.rs:66`;
  `pin.rs:75` and `run.rs:195` go through `replace_private_shared`, which inherits the retry. [premise-corrected: the refusal is
  `ERROR_ACCESS_DENIED` (raw os error 5), not a sharing violation (raw 32) — measured at P3 (research.md M1) under every reader
  share mode, std's default `R|W|D` included; the retry keys on raw 5 on Windows.]
- **The witness:** a `viola-state` test (a crate's property needs its own test, testing.md 2026-09-27) that holds `snapshot.json`
  open with a plain `File::open` across the replace — forcing the window open (testing.md 2026-09-27) — and asserts the replace
  lands once the holder lets go, and that a holder that never lets go leaves the old bytes. Verified at P3: today's code fails it
  at the first attempt (M1: `persist` returns raw 5 immediately), so it cannot pass vacuously; the retry is `cfg(windows)`-scoped,
  so only the windows mutation leg judges it (architecture-amendments, ci-chunk-base-and-union-verdict).
- **Why here:** verified at P3 (research.md M2): the wrapper's second snapshot write (`src/cmd/run.rs:275`) lands right after the
  spawn, exactly when the child fires SessionStart, and this chunk makes that hook read `snapshot.json` for the endpoint. The
  window moves from a test-only reader to the hot path.

### CARRY 6 (from 2026-09-27-browser-verdict-reachability, overseer live ratification, operator-only): `--install-deps` narrowing
Before the WSL distro is next re-provisioned, `scripts/wsl-provision.sh --install-deps` must stop running user-writable code as
root: root runs only `apt-get install` over the package list an unprivileged dry run produced (Playwright 1.63.0's
`install-deps --dry-run` lists it and exits 1 while packages are missing, measured at that chunk's
`evidence/wsl-chromium-deps.md`, re-verified present), checked against a committed allowlist. **Binds only if this chunk
re-provisions** — per the answer above it does not, so it moves on with the next markerless entry unless P4 changes that.

## CI verdict read at Setup 5a (the second fold source)
- Base: the last master flip `273e1ab07de3` (the browser-verdict-reachability wrap commit). Shas read: `273e1ab07de3` only.
- `273e1ab07de3 verdict: green · checks 15/15 · wall 262 s · runs ci#36348547463 completed/success`. Nothing to fold.

## Operator directions noted for later steps
- P5 states the projected implement size; if it will not fit one window, P5 proposes a split into a head and a named tail entry
  (overseer, 2026-09-27 23:15: this entry carries six CARRYs, and Epoch 2 implements ended at 60–74 % of the window).
- For the next wrap's handoff: the four `%TEMP%/cargo-mutants-viola-*.tmp` dirs are gone (0 left, overseer-measured), so that
  "for the operator" line drops.
