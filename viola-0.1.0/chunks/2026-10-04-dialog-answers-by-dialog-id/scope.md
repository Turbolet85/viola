# Scope — Dialog answers by dialog_id

**Marker:** `2026-10-04-dialog-answers-by-dialog-id` · **Version:** viola-0.1.0 · **Epoch:** Epoch 3 — Windows slice II: driving verbs and live proof
**Working entry:** `viola-0.1.0/working-route.md:78`: "Dialog answers by dialog_id — question, permission and plan through hooks, plan answered as plan, permission-suggestion decision, one pending dialog, deadline expiry, unknown-dialog", with one PREREQ and five CARRY blocks. All six are folded below. `route.py pins` indexed 6 freight blocks on :78, of 260 · 346 · 307 · 198 · 581 · 678 chars.
**Host:** Linux (Omarchy, btrfs). Windows / ConPTY behaviour is witnessed only by the `windows-2025` CI runner.
**CLI:** installed `claude` 2.1.287. Recorded fixtures exist only for 2.1.283 (`fixtures/claude/2.1.283/`), so 2.1.287 is an unverified build until a `viola verify` stamps it (handoff, 2026-10-04).
**Operator directives at take-up (2026-10-04):**
- fold the two CI reds of ci#37196414168 (on `c540254`, the `:76` wrap commit) as `[inferred]` items, each with its own acceptance criterion and witness (§10, §11);
- the mutants item first MEASURES which cause it is (a timeout counted as survived, or a slow runner) before it fixes either (§11);
- carry the `v1-30` dialog-kind wait witness the `:78` pin names (§9);
- Epoch 3 stays unsplit (founder ruling 2026-09-29); no boundary proposal is made.
**Held widening:** the PTY typed-input `viola verify` probe, the live 2.1.287 recording, and the signature / quiet-period / max-wait ledger rows stay HELD for the founder's ruling (handoff Notes; owned by `:82`). §5 touches a recording; how it relates to the held one is a fork for P4 (§5).
**P3 closure:** every `[inferred]` bullet below is closed against `research.md`. A closed bullet is either verified (tag dropped, evidence named) or marked `[premise-corrected: …]`.

## What this chunk builds

The dialog tier. Until now `run` answers `hook.dialog` and `answer` with `-32601` (`src/cmd/run.rs:588` pins both as unknown methods), and no dialog event is ever logged. This chunk lets a driver see a pending `question` / `permission` / `plan` dialog through `wait`, and answer it by its `dialog_id`. The hook then prints the matching decision body. viola stays mechanism: it carries the driver's answer and never picks one (architecture §Established Decisions, the first principle).

### 1. `hook.dialog` — raised, logged once, awaited (working entry)
- Surfaces: the dialog-tier hooks, PreToolUse with matcher `AskUserQuestion|ExitPlanMode` and PermissionRequest, sync with a decision (architecture [Hook Transport], `architecture.md:63`). The channel method `hook.dialog` `{kind:"question"|"permission"|"plan", data}` → `{dialog_id, response:<object|null>}` (`architecture.md:286`). `METHODS` already lists `"hook.dialog"` (`crates/viola-channel/src/server.rs:364`).
- On receipt the wrapper assigns the `dialog_id`, an integer, monotonic per instance, with the counter restored from `events.ndjson` on start so an id never repeats across restarts (`architecture.md:52`). It appends the dialog event (`source: hook`) before it can wake `wait` or reply, even when it replies `null` at once. So every dialog appears exactly once in the log (`architecture.md:307`). Event `data` per kind: `architecture.md:293-295`.
- `pending_dialog` `{dialog_id, kind}` is in the instance snapshot while a `hook.dialog` is open. It is removed on answer, on expiry, or on a `null` wheel-move answer. It is the only source of `dialog_pending`, and is not rebuilt from the log (`architecture.md:259`).
- **Deadline expiry** (working entry): `run.dialog_await` closes on answer or deadline. On the deadline the hook emits `hook-decision{decision_emitted:false, deadline_hit:true}`, exits 0 with empty stdout, and the dialog renders for the human (obs-plan `:678`). VERIFIED (architecture [Hook Contract], `architecture.md:69`): the deadline's value is an open item, but it must be ONE built-in value compiled into both `hook` and the embedded `hooks.json`, and the `hooks.json` `timeout` must exceed it by a margin, so viola, never Claude Code, ends the wait. The value itself is P4's (named and PROVISIONAL, on the `SPINE_DEADLINE` / `CONNECT_DEADLINE` precedent). Research: `Client::request` has no read deadline (`crates/viola-channel/src/client.rs:60-100`), so the hook's own bound on the wait is new work.
- Obs (obs-plan `:325-326`, `:670-678`, `:820`): the spans `hook.handle` › `channel.request(hook.dialog)` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`, and on the driver side `answer.client` › `channel.request(answer)`. The log events are `hook-invoked`, `dialog-raised`, `dialog-answered` and `hook-decision`. The question, answer and tool `input` text never appear in them.
- The hook fails open: any internal failure gives exit 0 with no body, exit 2 is never produced, and stderr is never written (CLAUDE.md Critical Warnings; `architecture.md:69`).

### 2. `answer` — the driver's verb, normalised per kind (working entry)
- Surfaces: the CLI verb `viola answer <target> <dialog_id>`, with the `response` object as JSON from stdin or `--file`. VERIFIED by the arch extract (architecture §Conventions CLI), the design and layouts samples (`viola answer builder 7 < response.json`) and the Session Learning "prompt text only from stdin or `--file`". The JSON is read through `take(MAX_FRAME + 1)` like `send`'s text (`src/cmd/send.rs:75-82`). The channel method `answer` `{dialog_id, from?, response}` → `{}`, with `response` normalised per kind (`architecture.md:278-281`):
  - `question`: `{answers:{<question>:<text>}, annotations?}`;
  - `permission`: `{behavior:"allow"|"deny", message?}`;
  - `plan`: `{behavior:"approve"|"revise", message?}`.
- Refusal order for `answer`: `human-typing` → `unverified-cli` → `not-delivered`/`unknown-dialog`, with `control-character` before all of them for free text (`architecture.md:136`; test-plan `:587`, `:314`). Exits: `12` `unverified-cli`, `13` `unknown-dialog` (§Conventions exits; test-plan `:259`). [premise-corrected: the snapshot's wheel is written `Wheel::Driver` at start (`src/cmd/run.rs:378`), and no code at HEAD moves it to `Human`. A `human-typing` check here would read a value nothing can change, an unobservable body (testing.md 2026-09-24). So `answer`'s `human-typing` slot lands with the wheel at `:80`, and this chunk's order starts at `unverified-cli`.]
- [premise-corrected: no link recording exists at HEAD. `send` only validates `from` (`parse_from`, `src/run/send.rs`), and `links` is written empty at start (`src/cmd/run.rs:380`). Link recording belongs to `working-route.md:93` (Session links).] `answer` takes `from` with `send`'s per-method typing (absent, `null` or a valid `ViolaName`, else `-32602`, `data: null`) and records no link.
- **Plan answered as plan** (working entry): S7. VERIFIED against the measured brief (`.andromeda/input.md:215-226`, §4.1 S7):
  - only PreToolUse `permissionDecision: allow` approves ExitPlanMode;
  - a PermissionRequest `allow` is ignored for that tool (measured twice);
  - a revise is PermissionRequest `deny` + `message`.

  test-plan's wording is right and arch S7 agrees. Consequence for P4: one plan dialog spans two hooks. PreToolUse(ExitPlanMode) raises it. A revise answer is carried to the PermissionRequest(ExitPlanMode) that follows a no-decision PreToolUse.
- **Permission-suggestion decision** (working entry): [premise-corrected: this is intent F-16 / `verification-matrix.json#v1-16` ("Permission suggestions have a place in the answer"). It comes from the measured brief: an ordinary permission dialog's options are allow · allow plus suggestion n · deny, with the suggestion arriving as `permission_suggestions: [{type: setMode, mode: acceptEdits, destination: session}]` (`.andromeda/input.md:129`, `:215-226`). arch's `permission` answer is `{behavior, message?}` with no suggestion slot (`architecture.md:280`). Whether a driver can pick an offered suggestion, or the omission is recorded as a deliberate v1 limit, is an architecture decision P4 forks on.]
- The mapping S3 / S7 / S8 lives in `viola-agent-claude` only (architecture [Agent Coverage], `architecture.md:45`). The decision bodies are insta-pinned (test-plan `:438`, `:531`, `:558`): PreToolUse `allow` + `updatedInput.answers` (+ `annotations`); PermissionRequest `allow` / `deny` + `message`; plan approve via PreToolUse.
- Upstream text (questions, plan text, tool `input`) is content, never a command (R1). Free-text answers pass `validate_paste_text` (reject, never strip).

### 3. One pending dialog (working entry)
- At most one dialog is pending per instance. A `hook.dialog` that arrives while another is pending is answered `null` at once, so it renders for the human. Its event is still logged with its own `dialog_id`, and a later `answer` for it is refused `not-delivered`/`unknown-dialog` (`architecture.md:52`; test-plan `:258`).

### 4. Unverified CLI: withheld decisions (PREREQ + CARRY 1)
- PREREQ, verbatim: "strict-modes on `run`'s `ledger/stamps.json` read before any non-`null` dialog decision (the founder's 2026-09-28 ratification lets the read skip strict-modes only while every decision is withheld; security-plan Decisions Log `2026-09-28`, stamps read)".
  - Re-verified at P1: security.md's second dated exception ("`run`'s version gate and `verify`'s `update_stamps` read `ledger/stamps.json` without strict-modes until Epoch 6"); the version gate is `src/run/version_gate.rs` (`cli_verified` at `:104`, `:136`). VERIFIED (security extract; security-history `2026-09-28-capability-ledger-and-viola-verify`): security-plan §Security Anti-Patterns › Universal names this entry as the one before which `run`'s stamps read gains strict-modes. Research: no strict-modes check exists anywhere at HEAD (`crates/viola-state/src/fs.rs:3`, "the DACL check belongs to the strict-modes work"; `grep -rn 'strict_modes\|fn strict' crates src` → 0 hits). So this chunk builds the first one (Unix mode/owner; Windows owner + DACL), scoped to that read, or takes a founder ruling. That is a P4 fork against the playbook's escalate patterns.
- On an unverified CLI `hook.dialog` replies `null` at once (hook stdout empty), and `answer` is refused `unverified-cli` (exit 12), keyed on the snapshot's `cli_verified` (test-plan `:283-291`; `architecture.md:286`). A non-`null` dialog decision requires a `viola verify` stamp for the child's CLI version (security.md).

### 5. Ledger rows S3 / S7 / S8 and dialog concurrency (CARRY 1 + CARRY 4)
- CARRY 1, verbatim: "chunk 2026-09-28-capability-ledger-and-viola-verify: the dialog ledger rows S3 / S7 / S8 and dialog concurrency with their typed probes, the transport-only withholding and `answer`'s `unverified-cli` refusal keyed on the snapshot's `cli_verified` land here (the plan named this entry by line :62, measured stale; its subject is this entry)".
- CARRY 4, verbatim: "chunk 2026-09-29-fake-agent-drift-contract (P4 operator forks, the overseer agreeing): (1) hook `matcher` evaluation in the fake agent lands here, with the first tool-bearing fixtures this entry records (PreToolUse `AskUserQuestion|ExitPlanMode`) — no recorded fixture carried a `tool_name`, and a docs-only matcher would put unmeasured behaviour in the fake agent; (2) the S8 witness — `contract_*` asserting the forwarded `annotations` in the question answer path (PreToolUse `updatedInput.answers` + `annotations`, test-plan §6 Contract suite) — lands with that path here".
  - Re-verified at P1: `fixtures/claude/` holds only `2.1.283/`. The fake agent is `src/bin/viola-fake-agent.rs`. `contract_ledger_probes.rs` and `contract_fake_agent_drift.rs` exist under `tests/`.
- VERIFIED (research): `viola verify`'s probe is one print-mode turn whose capture plugin registers only the four spine events (`crates/viola-agent-claude/src/ledger.rs:62-68`, `:97-122`). `LedgerRow::ALL` holds six rows (`:15-32`), and `verified()` requires every row `pass` (`:281-291`). So adding rows unverifies every existing stamp until a probe that can pass them exists. `fixtures/claude/2.1.283/` holds only the four spine fixtures.
- **A fork for P4:** CARRY 4 says this entry *records* the first tool-bearing fixtures. Recording takes a `viola verify` run against a live `claude`, and the held widening holds "the live 2.1.287 recording" (handoff Notes, `:82`). Whether a tool-bearing recording here falls inside the hold (and so waits for the founder), or is a separate recording the hold does not cover, is a fork the wrap would halt on. P4 asks it against `.andromeda/playbook.md`'s escalate patterns.

### 6. The `pre-tool-use` perf row (CARRY 2)
- CARRY 2, verbatim: "chunk 2026-09-28-hook-perf-gate timed four hook rows and left `viola hook pre-tool-use` untimed (an unknown event there): this entry adds the `pre-tool-use` row to `run --perf` / `PERF_ROWS` (the gate then requires five) on an unverified CLI, `hook.dialog` → `null` at once (test-plan §10 perf table)".
  - Re-verified at P1: `PERF_ROWS` is `perf::ROWS` (`crates/viola-e2e/src/harness/run.rs:31`), read by the gate at `crates/viola-e2e/src/harness/gate.rs:133`; test-plan `:1081` ("`pre-tool-use` untimed until the dialog-tier chunk").

### 7. The `answer` unstamped negative in the controls table (CARRY 3)
- CARRY 3, verbatim: "chunk 2026-09-28-hook-perf-gate landed `tests/cli_controls_not_disableable.rs` in its interim shape: this entry adds its `answer` unstamped negative (exit 12) to that table (test-plan §5 CLI)".
  - Re-verified at P1: `tests/cli_controls_not_disableable.rs` exists.

### 8. The fake agent's hook `matcher` (CARRY 4 (1))
- Folded with §5 (CARRY 4 verbatim there). The fake agent evaluates each hook's `matcher` only against tool-bearing fixtures this chunk has, never against docs-only behaviour.

### 9. The `v1-30` dialog-kind wait witness (CARRY 5; operator directive)
- CARRY 5, verbatim: "chunk 2026-10-04-wait-and-last (the operator's condition of the `v1-30` claim at its P5 approval, restated as a wrap directive, 2026-10-04): this entry owns the end-to-end `question` / `permission` / `plan` wait witness for `verification-matrix.json#v1-30` — at HEAD those kinds wake only at unit level (`run::wait::tests`). A dialog event's append must go through the wait feed (`WaitFeed::appending`, `src/run/wait.rs`) so it wakes a parked `wait`; the without-`after` pending-dialog return (as landed, an `after`-less `wait` starts at the log's end, arch [Message Broker / IPC]) and the `<kind>  <name>  dialog <id>  cursor <n>` line's end-to-end witness land here too".
  - Re-verified at P1: `WaitFeed::appending` is at `src/run/wait.rs:68` and `rebuild` at `:84`. `architecture.md:52` names this entry as the owner of the pending-dialog return. `v1-30` is already `verified` (handoff), so this witness is an obligation carried by its claim condition, not a new claim.
- The end-to-end witness: a real `run` with the fake agent raising each of `question` / `permission` / `plan`. A parked `wait` wakes on each, a `wait` called without `after` while a dialog is pending returns it at once, and the human-mode line reads `<kind>  <name>  dialog <id>  cursor <n>`.

### 10. CI red A — `cli_send` stdin `BrokenPipe` on ubuntu (operator directive; [inferred])
- VERIFIED against the recorded run (runner-only; closed against the run, not HEAD): ci#37196414168 on `c540254a9e36`, job `test (ubuntu-latest)` (id 111419123020), step "Coverage and doctest": `send_leading_slash_argument_is_a_usage_error` FAIL, `panicked at tests/cli_send.rs:84:38: stdin: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }`. The test fails before its own assertions run. The mechanism claim (overseer): "the child exits on the usage error before the write". EPIPE on a pipe write means the read end was already closed, which is consistent with that claim. It is a timing-dependent race (the same test passed on the other legs).
- Re-verified at P1 (coordinates only): `spawn_send` (`tests/cli_send.rs:71-87`) writes `text` to the piped stdin unconditionally and `expect`s the write.
- Its own acceptance criterion, plus a witness that cannot pass vacuously. The test's assertion (the usage error) must still be checked when the child has exited before the write. A fix that swallows every write error, or skips the assertion, fails it.

### 11. CI red B — the mutants harness `timeout` on msrv (operator directive; [inferred]; measure first)
- VERIFIED against the recorded run (runner-only): ci#37196414168 on `c540254a9e36`, job `msrv` (id 111419122898), step "Unit on 1.96": `run_mutants_passes_when_the_change_is_tested` FAIL at `mutants.rs:425:9` (`survived` 1, not 0).
- [premise-corrected: measured from that run's log, the cause is neither "a mutant that genuinely hangs" nor a counting fault. cargo-mutants printed `Unmutated baseline in 0s build + 0s test`, then `Auto-set build timeout to 1s`, then `caught … replace three -> u32 with 0 in 0s build + 0s test` and `TIMEOUT … replace three -> u32 with 1 in 1s build`. The second mutant timed out in its BUILD, not its test. The build timeout is the harness's own `--build-timeout-multiplier=5` (`mutants.rs:21`) times a sub-second baseline. cargo-mutants 27.1.0 computes it with NO floor (`timeouts.rs:80-95`; the test timeout alone has the 20 s `minimum_test_timeout` floor). So this is a runner-timing case, (b), and the counting rule (a timeout is a survivor) is not at fault.]
- Re-verified at P1 (coordinates only): `mutants_suite` counts `survived = missed + timeout` by design (`mutants.rs:43`, `:49`), pinned by the unit test `mutants_suite_counts_missed_and_timeout_as_survivors` (`:334`). The progress flags carry `--build-timeout-multiplier=5` (`:21`).
- **Measure first (operator directive).** P3 made the first measurement, from the run's own log and cargo-mutants' source (bullet above): (b), specifically the multiplier's missing floor on a sub-second baseline build. The plan still owes the reproduction that confirms it on this host's msrv toolchain before the fix, recorded under `evidence/`. The fix follows that cause only, at the harness's build-timeout argument. The counting rule (a timeout is a survivor) is a designed, unit-pinned contract and stays.
- Its own acceptance criterion and a witness that cannot pass vacuously: the measurement is recorded as evidence in the chunk folder. The fixed test passes on the cause the measurement names, and a mutant left untested by the fixture still fails it.

## P4 resolutions (validation-1: intent-incomplete, amended at P5)
- **F1, the frames:** `answer` and `hook.dialog` ride a sixth dated gap until `:109` / `:111`. The founder ruled live, 2026-10-04 ~11:15Z, relayed by the overseer, with the residual on PermissionRequest `allow` shown.
- **F2, the recording:** the hold is scoped. This chunk adds a dialog probe and records tool-bearing fixtures once, locally, from the installed 2.1.287. The typed-input probe and the signature / timing rows stay held at `:82` (founder, live, same exchange). Consequence the plan carries: with four new rows, 2.1.283 can no longer verify, so the fixture default for tests moves to 2.1.287.
- **F3, permission suggestions:** a deliberate v1 limit; the permission answer stays `{behavior, message?}` (founder, live, same exchange).
- **F4, the deadline:** `viola_core::DIALOG_DEADLINE` = 60 s PROVISIONAL, `hooks.json` `timeout` 75 (overseer, founder-delegated).
- **S7 continuation:** a `plan` revise answered on PreToolUse is carried to the following PermissionRequest(`ExitPlanMode`) without a second event (`architecture.md:307`'s exactly-once). This is an expected architecture amendment.
- The four ledger rows assert what a capture-only print-mode probe can show. The decision's effect, the "dialog never renders" half, is owed to `:82`'s live test.

## Revision (founder live rulings, 2026-10-04, relayed by the overseer)
Implement stopped on the plan's own STOP clause: on 2.1.287 a print-mode turn exposes neither `AskUserQuestion` nor
`ExitPlanMode` and fires no dialog hook (`evidence/print-mode-dialog-probe.md`). F2 above is superseded:
- **Fixtures (~11:55Z):** the dialog fixtures are sourced from the viola-lab prototype's live captures
  (`~/.viola/sessions/*/events.ndjson`, interactive `claude` 2.1.287), scrubbed and marked as relayed captures. No PTY
  probe. The smallest harmless captures are chosen, and the overseer reviews them before commit (overseer resolution 4).
- **No new ledger row here (~11:55Z):** S3 / S7 / S8 / dialog concurrency and their own `viola verify` re-probe go to
  `:82`. `LedgerRow::ALL` stays at 6; `viola verify`'s step counter stays `/06`.
- **Spine recording (overseer resolution 1):** one print-mode `viola verify --record` of the existing 6 spine rows at
  2.1.287 is allowed, inside the founder's 2026-10-04 scoped local-recording ruling; it gives the 2.1.287 fixture dir
  its spine set, so the test default moves to 2.1.287 where the dialog fixtures live.
- **Decisions on the spine stamp (~14:08Z, the founder live, the residual shown: an S3 / S7 / S8 shape change in a new
  CLI is not caught by the stamp until `:82`):** non-null decision bodies flow on the 6-row spine stamp as a dated gap
  closed by `:82`'s rows. Pattern: playbook "Boundary widening".
- **PermissionRequest for `AskUserQuestion` (overseer resolution 2):** it takes the plan-style continuation — the
  question dialog raised by PreToolUse is the one dialog; the PermissionRequest that follows it appends no event.
  [premise-corrected: §2's "one plan dialog spans two hooks" holds for `question` too — 5 PermissionRequest
  `AskUserQuestion` captures, each repeating its PreToolUse's `tool_input` (research §Mechanism equalities).]
- **`permission` kind (overseer resolution 3):** no ordinary-tool PermissionRequest capture exists, so the `permission`
  classification and bodies are unit / insta-pinned only; its e2e is owed to `:82`.
- **Canary (overseer resolution 5):** relayed payloads cannot carry the tests-owned canary; the canary checks move to
  the answer and driver text (answers, `message`, annotations, sent prompts).
- **v1-15 (~14:08Z):** claimed at `:82`, not here — its acceptance names the S7 ledger row passing in the 2.1.287 stamp.
- §5's ledger rows, CARRY 1's "dialog ledger rows S3 / S7 / S8 and dialog concurrency with their typed probes" and
  CARRY 4's tool-bearing recording are discharged only in part here: the transport-only withholding and `answer`'s
  `unverified-cli` refusal land; the rows and their probes move to `:82` (a route pin for the wrap).

## Boundaries (what this chunk does not build)
- MCP `answer` (`viola-mcp`, Epoch 5 `Driver surface`, `working-route.md:102`). This chunk is CLI + channel + hook only.
- The web strip's `data-dialog="pending"` / `DIALOG <kind>` / `document.title` (Epoch 8, the web UI).
- The wheel, the `null` answer on a wheel move and `answer`'s `human-typing` slot (`working-route.md:80`). VERIFIED: no code at HEAD moves the wheel (`src/cmd/run.rs:378` writes `Driver`), so `pending_dialog`'s removal on a wheel move is `:80`'s. This chunk builds the removal on answer and on expiry.
- Link recording for an `answer` carrying `from` (`working-route.md:93`, Session links).
- The held typed-input probe, the dialog ledger rows S3 / S7 / S8 / concurrency and their `viola verify` re-probe (`:82`, the founder's rulings), except the spine-only 2.1.287 recording the revision allows.
- Server-identity verification and strict-modes before the first frame (Epoch 6 `:109` / `:111`). The PREREQ's stamps-read strict-modes is §4's.
- No epoch boundary or split proposal (operator directive, founder ruling 2026-09-29).

## CI read at take-up (Setup 5a)
- `c540254` (the last wrap's flip, = HEAD): **red**, checks 15/15, first failed check +74 s (msrv), wall 280 s, ci#37196414168 push completed/failure. Failed 2: `msrv` (failure) · `test (ubuntu-latest)` (failure). Neither failing subject intersects the dialog tier. Both are folded on the operator's word (directive above) as §10 and §11, each with its own acceptance and witness.
