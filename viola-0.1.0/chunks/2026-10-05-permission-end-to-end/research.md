# Codebase Research — 2026-10-05-permission-end-to-end

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 22 (+ 2 code-graph queries, 9 static bundle probes)
- **Harness rules consulted:** none — no live leg in this chunk (zero live `claude` sessions; the gate commands are
  `testing.md` §Running tests forms, already in the window)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (Setup
  5a read `2bd08e9` green, 15/15)
- **External inputs:** `inputs#I1` — the operator's directive at take-up: fold F1 (overseer-decided), plan zero live
  sessions (cap 3 of 16 left), Epoch 3 unsplit

## Files inspected
- `tests/cli_answer.rs` (`:1-425`, test map `:246-702`) — the Path 4 shape the permission case copies: `boot` over a
  gated script (`:45-52`), `raise` (`:236-244`), `answer` (`:179-188`), `hooks_at_least` / `clean_stdout` (`:91-109`),
  `wait_request_logged` (`:195-213`), `assert_logs_clean` (canary + diag-line schema, `:217-233`). The header
  (`:1-9`) names the relayed 2.1.287 captures and the stale owner `:84`. `relayed()` (`:36-42`) reads any file under
  `fixtures/claude/2.1.287/` (`RECORDED_CLI_VERSION`, `tests/support/fake.rs:16`). `path4_dialogs_…` (`:249-425`)
  carries the `question` and `plan` `v1-30` wake witnesses (docstring `:246-248`; asserts `:256-267`, `:316-324`) and
  ends asserting exactly 3 dialog lines and no `activity` line (`:384`, `:390`).
- `fixtures/fake-scripts/path4.json` (full) — six gated steps (question PreToolUse/PermissionRequest, plan ×3, Stop);
  no permission step.
- `fixtures/claude/2.1.288/PermissionRequest.permission-1.json`, `PostToolUse.permission-1.json` (full) — the recorded
  Run C ordinary-tool call: `tool_name: "Bash"`, `tool_input: {command: "touch viola-probe-permission", description}`;
  `~`-scrubbed paths; no canary (recorded, not synthetic). Both files are present under `2.1.287/` too (listing).
- `src/bin/viola-fake-agent.rs` (`:198-224`, `:313-374`, `:473-487`) — a script step fires
  `<fixtures>/<cli-version>/<Event>.<variant>.json` (`fixture`, `:322-324`), so `{"event":"PermissionRequest",
  "variant":"permission-1"}` and `{"event":"PostToolUse","variant":"permission-1"}` replay the recorded files with no
  fake-agent change. A script step fires PostToolUse only when scripted (`replay_dialogs`' auto-PostToolUse is the
  `--dialogs` mode, `:352-374`).
- `crates/viola-agent-claude/src/dialog.rs` (`:40-117`, `:326-400`) — `classify`: a PermissionRequest whose tool is
  not a `DialogTool` is `DialogKind::Permission` with `data {tool, input}`, `continuation: false` (`:101-115`).
  `decision_body`: permission on PermissionRequest → `allow` / `deny` + `message` (`:373-379`); "a question on
  PermissionRequest → no decision (its body is unmeasured)" (`:340`, `:380`).
- `src/run/dialog.rs` (`:180-238`, `:368-400`, `:720-736`) — `hook_dialog`: a continuation with no matching arm is
  raised as its own dialog; a question raised that way is `unanswerable` → `null` at once, logged (`:189-194`; unit
  witness `continuation_raised_as_a_question_is_null_at_once`, `:724-736`, docstring "its body is unmeasured"). Arms
  are kept ONE PER TOOL: a new PreToolUse for the same tool drops the older arm (`state.arms.retain(|a| a.tool !=
  tool)`, `:199`).
- `plugin/hooks/hooks.json` (full) — PreToolUse matcher `AskUserQuestion|ExitPlanMode` (timeout 75); PermissionRequest
  unmatched (every tool); PostToolUse `async: true`.
- `.config/nextest.toml` (full) — `[profile.ci]`: `test(/verify_window_/)` 15 s × 3 = 45 s (first), then the twelve
  verify-driving binaries (`cli_answer` among them) 10 s × 2 = 20 s; default 30 s × 4; `retries = 0`.
- `viola-0.1.0/chunks/2026-10-05-dialog-rows-and-re-probe/evidence/verify-window-class.md` (full) — the :86 class's
  authority, the designed-floor measurement, and its Controls table.
- `.claude/rules/testing.md` (full, auto-loaded) — `:70` the 2026-09-28 timing-red entry; `## Session Additions` is
  owned by `/wrap-session` (its own line).
- `tests/contract_lints.rs` (`:1-2`, `:114-116`, `:216-228`) — pins test deadlines below the `mutants` profile kill
  line; it reads the profile and is untouched by an evidence-only control.
- `crates/viola-agent-claude/src/ledger.rs` (`:60-110`, `:238-280`, `:494-520`) — the fourteen rows; no row measures a
  question answered through PermissionRequest (`question-answer` = "a question answered through PreToolUse takes
  effect").
- The installed CLI bundles, read statically (`~/.local/share/mise/installs/claude/{2.1.288,2.1.287}/claude`, by a
  scratch byte-search script, never executed) — see M1.
- `.andromeda/test-plan.md:777-781` — Path 4's target script names `PermissionRequest.permission-1.json` as prompt 2
  ("the step owed to 'Permission end to end'"); as landed, each Path test drives its own gated script.

## Graph impact
- **decision_body** — 2 product callers: `ledger::probe_body` @ `crates/viola-agent-claude/src/ledger.rs:260`,
  `hook::body_of` @ `src/cmd/hook.rs:301`; the rest are `dialog::tests` (`dialog.rs:625-709`). No signature changes in
  this chunk, so no threading.
- **hook_dialog** — 1 product caller: `Dispatch::dispatch_call` @ `src/cmd/run.rs:128`; the rest are
  `run::dialog::tests`. Unchanged by this chunk.
- Trace: `.andromeda/runs/2026-10-05T15-00-34-phase/tree-query-2026-10-05-permission-end-to-end.json` (2 queries,
  rust plane).

## Patterns detected
- **Gated per-test fake script** (`fixtures/fake-scripts/path3.json`, `path4.json`; `cli_answer.rs:45-52`): every
  Path test boots over its own gated script and releases steps through the control file — never timing.
- **Hook-body oracle as a literal** (`cli_answer.rs:338-344`, `:302-306`): the e2e case compares the hook's stdout to
  a literal body; insta snapshots pin the bodies at unit level only (`dialog::tests`).
- **A wait parked before the raise** (`cli_answer.rs:257-267`): spawn `viola wait builder`, wait for the `wait`
  `channel-request` line, release the step, then compare the woken line to
  `<kind>  builder  dialog <id>  cursor <end>`.

## Conventions to follow
- **Naming**: `path4_<slug>` (`testing.md` §Naming); fake scripts `fixtures/fake-scripts/<scenario>.json` under
  `schemas/fake-script.v1.json`, walked by `contract_fixture_hygiene`.
- **Waits**: on the exact line asserted (`testing.md` 2026-09-24) — the async PostToolUse `activity` line is waited
  for before its "did not wake" assertion.
- **Gate selector**: `scripts/agent-run.sh run --integration --filter 'binary(cli_answer)'` (`testing.md` §Running
  tests; `--e2e` is a usage error).

## Measured / re-derived (scope premise closure inputs)
- **M1 — a PermissionRequest `allow` on AskUserQuestion, read statically from both installed CLIs** (`bundle_probe.py`
  over the binary bytes, never executed). 2.1.288: in the PermissionRequest-hook result handler,
  `if(!U2n(e,P)&&e.requiresUserInteraction?.())return;` precedes
  `return{decision:{behavior:"allow",updatedInput:w,…,decisionReason:{type:"hook",hookName:"PermissionRequest"}}…}`,
  with `function U2n(e,n){return n!==void 0&&e.mcpInfo===void 0&&gXo.has(e.name)}` and
  `gXo=new Set([hu,Gs])`, `hu="ExitPlanMode"`, `Gs="AskUserQuestion"`. 2.1.287: the same shape —
  `if(!rzn(e,w)&&e.requiresUserInteraction?.())return;`, `function rzn(e,n){return n!==void 0&&e.mcpInfo===void
  0&&BVo.has(e.name)}`, `BVo=new Set([au,zs])`, `au="ExitPlanMode"`, `zs="AskUserQuestion"`. So on both versions a
  PermissionRequest `allow` WITH `updatedInput` satisfies AskUserQuestion's user interaction (a bare `allow` does not —
  the handler returns no decision and the dialog renders). With the previous chunk's M6 (AskUserQuestion's permission
  result maps `updatedInput.answers` / `annotations`), the candidate body is
  `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow","updatedInput":<input + answers [+ annotations]>}}}`.
  A static read is not a ledger post-condition: nothing here was run.
- **M2 — when a question is first raised by PermissionRequest under viola** (`src/run/dialog.rs:184-199`, `:375-391`;
  `plugin/hooks/hooks.json`): a PermissionRequest for AskUserQuestion is a continuation only of the ONE armed
  AskUserQuestion with an equal `tool_input`. It is raised as its own question when no arm matches: two parallel
  AskUserQuestion calls (the second PreToolUse drops the first arm, `:199`; the dialog-concurrency row's premise —
  `isConcurrencySafe() → true`, previous chunk M6), a wrapper restart between the two hooks (arms are in memory), or a
  PreToolUse hook that failed open before its `hook.dialog` frame (`channel-unreachable`). Reachable, rarely.
- **M3 — the cost of building the body** (re-derived from the previous chunk's `evidence/live-sessions.md` convention,
  research M13 there: a full `viola verify` spends ≥ 3 live sessions, + 1 per interactive dialog run): a non-`null`
  body is an undocumented CLI dependence, so it needs its own capability-ledger row with a `viola verify` probe
  (architecture §Cross-cutting Patterns → Capability ledger as the single gate) that makes the model raise a question
  reaching PermissionRequest with no PreToolUse answer, then a re-stamp of both installed versions: ≥ 2 × (3 + 1) = 8
  live sessions, against a cap of 3, and a 15th row the scope rules out.
- **M4 — the :86 planted-hang controls** (`evidence/verify-window-class.md` §Controls): a planted `sleep(40 s)` after
  `verify` in a `cli_verify` test under `--profile ci` read `SLOW [> 10.000s]` then `TIMEOUT [20.003s]` (the 20 s
  binary override), and `PASS [43.346s]` under the default profile (no kill). No control was recorded for the 45 s
  `test(/verify_window_/)` override. The designed floor was measured across CI rounds (`evidence/ci-rounds.md`, four
  rounds).
- **M5 — `v1-30`** (`matrix.py show --id v1-30`): `status: verified`, `chunk: 2026-10-04-wait-and-last`; acceptance
  ends "The dialog kinds' end-to-end wait witness is owed to working-route.md:78". `matrix.py claim` refuses a verified
  cap (verification-matrix-contract §Tool), so this chunk cannot claim it.
- **M6 — an observation outside scope**: by M1, a PermissionRequest `allow` WITH `updatedInput` would also satisfy
  ExitPlanMode. viola's docstring (`dialog.rs:334-337`) says "a PermissionRequest `allow` is ignored for
  `ExitPlanMode`", which holds for viola's bare `allow` (measured live on 2.1.288 at the previous chunk). Nothing here
  changes that body; noted for the wrap.

## New files to create
- `fixtures/fake-scripts/path4-permission.json` — the gated permission script: PermissionRequest `permission-1`,
  PostToolUse `permission-1`, PermissionRequest `permission-1` again, PermissionRequest `ask-user-question` with no
  PreToolUse before it, Stop
- `viola-0.1.0/chunks/2026-10-05-permission-end-to-end/evidence/verify-window-hang-control.md` — F1's one-off control
  for the 45 s `verify_window_` override, both readings

## Files to modify
- `tests/cli_answer.rs` — the `permission` case, the question-first-raised-by-PermissionRequest case, the header reword (W6)
- `crates/viola-agent-claude/src/dialog.rs` — the doc lines that say the question body is unmeasured now state the measured reason
- `src/run/dialog.rs` — the same reason at the `unanswerable` comment and its unit test's docstring

## Open questions
- none — W3d-b's build-or-keep is a decisive lean from M1–M3 under the operator's zero-session directive (P4 states
  it with the costed alternative); v1-30's recording form is P4/P5's (M5).
