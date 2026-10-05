# Report — 2026-10-05-permission-end-to-end

**Chunk:** Permission end to end — the permission kind's end-to-end Path 4 case and v1-30 wake witness over the
recorded fixtures, the question's PermissionRequest body measured, the cli_answer owner reword, the timing-red rule's
designed-floor exception
**Date:** 2026-10-05
**Commits:** `c06caf3 chore(2026-10-05-permission-end-to-end): operator pre-CI commit, for the run this chunk's verdict
reads` (`git log --format='%h %s' 2bd08e9..HEAD`; the only commit since `last_wrap` 2026-10-05T14:49Z besides the prior
wrap's `2bd08e9`)

## Changes (structured — detectors read this)
- **Files** (basis: `git diff --name-only 2bd08e9` plus untracked files; `gate.py scope` reads `changed 4 · listed 4`):
  - new: `fixtures/fake-scripts/path4-permission.json`;
  - modified: `tests/cli_answer.rs`, `crates/viola-agent-claude/src/dialog.rs` (doc lines only), `src/run/dialog.rs`
    (one comment and one test docstring only);
  - chunk folder: `evidence/verify-window-hang-control.md`, `evidence/operator-pass.md`;
  - ledger: `viola-0.1.0/verification-matrix.json` (`v1-16` → `implemented`).
- **Symbols / APIs:** no product symbol, signature, IPC method, endpoint, env var or port changed. The guard entry
  `git diff 2bd08e949138 -- crates/viola-agent-claude/src/dialog.rs src/run/dialog.rs | … | grep -cvE '^[+-][[:space:]]*//'`
  reads 0 at exit 1: every changed line in the two product files is a `//` line.
  - `decision_body` keeps its two product callers (`ledger::probe_body`, `hook::body_of`) and `hook_dialog` keeps one
    (`Dispatch::dispatch_call`), per research §Graph impact.
  - New TEST symbols in `tests/cli_answer.rs`:
    - `path4_permission_is_logged_once_woken_and_answered_by_id` (rstest, `stamped_home`);
    - `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused` (`#[test]`, `StampedHome::unstamped`);
    - helpers `boot_over(stamped, script)` (`boot` now delegates to it with `PATH4`), the const `PATH4_PERMISSION`, and
      `relayed()` renamed to `fixture()` (5 call sites; doc "A fixture of the stamped CLI version (relayed or
      recorded)").
- **Doc-comment changes (product, behaviour unchanged):**
  - `crates/viola-agent-claude/src/dialog.rs` `decision_body` doc, the question-on-PermissionRequest bullet: was "no
    decision (its body is unmeasured)". It now reads: no decision, because 2.1.288 and 2.1.287 would take an `allow`
    with `updatedInput` there (read statically), but that body has no ledger row, so the human answers.
  - `src/run/dialog.rs` `hook_dialog`'s `unanswerable` comment, and the docstring of the unit test
    `continuation_raised_as_a_question_is_null_at_once`: the same reason (a static read on 2.1.288 / 2.1.287 and no
    ledger row).
- **Crates / modules:** none changed (only comments in `viola-agent-claude` and the root bin).
- **Dependencies:** none.
- **Schema / config:**
  - New fake script `fixtures/fake-scripts/path4-permission.json` (`v: 1`, five gated steps):
    - PermissionRequest / `permission-1`;
    - PostToolUse / `permission-1`;
    - PermissionRequest / `permission-1`;
    - PermissionRequest / `ask-user-question`, with no PreToolUse before it;
    - Stop / `default`.
  - It replays existing fixtures under `fixtures/claude/2.1.287/` (`fake::RECORDED_CLI_VERSION`). It passes the
    `contract_fixture_hygiene` walk and `schemas/fake-script.v1.json` (gate entry 5 green).
  - No fixture, schema, ledger row, stamp, hook, plugin, `.config/nextest.toml` or harness change: the guard entry
    `git diff --quiet 2bd08e949138 -- tests/cli_verify.rs .config/nextest.toml tests/support plugin src/cmd src/bin
    crates/viola-core/src … schemas fixtures/claude scripts` exits 0.
- **Spec-master edits:** none this chunk (the seven masters are untouched before P2).
- **Counts / qualifiers moved:**
  - `verification-matrix.json`: `v1-16` `planned` → `implemented` (ref
    `tests/cli_answer.rs::path4_permission_is_logged_once_woken_and_answered_by_id`), via `matrix.py implement`.
    The version tally `verified 13/53` becomes `14/53` after P7's flip.
  - Path 4 now covers all three dialog kinds end to end in `tests/cli_answer.rs`. test-plan.md:779 still says "The
    `permission` kind is unit / insta only", and test-plan.md:757 "the `permission` wake stays unit-level".
  - The fixtures dir count, the ledger's fourteen rows and the stamp shape are unchanged (the guard above).
- **Dev-tool versions:** none — cargo-nextest re-read at 0.9.146 on the dev host (the F1 control).
- **Harness / gate surface:** none changed (`scripts/`, `crates/viola-e2e/src` under the guard's `git diff --quiet`).
- **Cross-project / external claims:**
  - **CI:** ci#37333536319 measured `c06caf3`: `verdict: green · checks 15/15 · wall 1424 s` (`ci.py conclusion`).
    The extra wall time over the prior run (ci#37328148791, `2bd08e9`, 415 s) is all in ubuntu `test`'s `Chromium
    (with system deps)` step, 1177 s (`gh run view 37333536319 --json jobs`).
    - The suite's own steps: `Coverage and doctest (sh shim)` 116 s, `Secret scan` 62 s; Windows `test` 371 s,
      macOS `test` 325 s.
    - No queue over 14 s.
  - **Inputs** (`inputs.py verify`):
    - `I1 · message: the operator, /andromeda-phase invocation args, 2026-10-05T15:00Z (overseer-decided fold) · copy
      · n/a — a message has no live source`, cited at scope.md:24, :96, research.md:9 and plan.md:9, :32, :326,
      :327, :375.
    - `UNPARSED: inputs#I2 — no entry (scope.md:69)`: scope.md:69 cites the PREVIOUS chunk's `inputs#I2` (the
      prototype's AskUserQuestion census, `2026-10-05-dialog-rows-and-re-probe/inputs/`). It is a cross-chunk
      citation, not an entry of this chunk; scope.md is immutable here.
  - **Static bundle reads (research M1):** both installed CLI bundles,
    `~/.local/share/mise/installs/claude/{2.1.288,2.1.287}/claude`, read by byte search, never executed.
    - 2.1.288: `U2n` / `gXo = new Set([ExitPlanMode, AskUserQuestion])`. 2.1.287: `rzn` / `BVo`.
    - On both, a PermissionRequest `allow` WITH `updatedInput` satisfies `requiresUserInteraction` for those two
      tools; a bare `allow` does not.
    - Not a ledger post-condition: nothing was run.
- **Reverted / negative API facts:** F1's planted `std::thread::sleep(std::time::Duration::from_secs(60))` in
  `tests/cli_verify.rs` `verify_window_without_screens_fails_every_interactive_row`, a one-off control, was reverted
  (the `git diff --quiet` guard above exits 0).
  - Also not shipped: a question body for PermissionRequest. The plan's rejected alternative was a 15th ledger row,
    its verify probe and a re-stamp, ≥ 8 live sessions (research M3).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **The plan's gate entry 12 atom** — `grep -cE 'TIMEOUT \[4[0-9]\.[0-9]+s\]' …/evidence/verify-window-hang-control.md`
     (`plan.md:202`, its `note` claiming the atom form from the :86 record) — assumes nextest prints an unpadded
     bracket.
     - Measured: cargo-nextest 0.9.146 prints `TIMEOUT [  45.004s]`, padded inside the bracket (the verbatim line in
       `evidence/verify-window-hang-control.md`).
     - The atom matches only the evidence table's padding-collapsed `TIMEOUT [45.004s]`. That is the :86 record's own
       form: its `TIMEOUT [20.003s]` was likewise collapsed from nextest's padded print.
     - The entry read green on that table row. The padding-tolerant form is
       `grep -cE 'TIMEOUT \[ *4[0-9]\.[0-9]+s\]'`.
     - Disposition: a **dated plan correction, 2026-10-05**, on the operator's word at this wrap's invocation ("entry 12
       grep missed the padded TIMEOUT form: record it as a plan defect in the report (a dated correction, as at
       :82)").
     - The plan is a chunk artifact, not a master. No master quotes the atom (`grep -c 'TIMEOUT \[' .andromeda/*.md`
       → 0 per master).
  2. **The plan's F1 command** `cargo nextest run --profile ci -E 'test(/verify_window_without_screens/)'`
     (`plan.md:100-102`) omits `--features fake-agent`, which the `cli_verify` target requires
     (`Cargo.toml:106-109`, `required-features = ["fake-agent"]`). It was run with the feature (evidence records the
     form). Disposed with correction 1, as the same plan-text class.
  3. **architecture.md:91** says "The `permission` kind's end to end and a `question` first raised by PermissionRequest
     are owed to the 'Permission end to end' route entry". Both are now discharged: the end-to-end case landed, and
     the question's body was measured, with the `null` kept (Expected amendments E1). P2's.
  4. **test-plan.md:779** says "The `permission` kind is unit / insta only (`dialog::tests`), its end-to-end case owed
     to 'Permission end to end'". **test-plan.md:757** says "the `permission` wake stays unit-level, its end-to-end
     witness owed to 'Permission end to end'". Both are falsified by the two new cases (E2, E3). P2's.
- **Expected amendments (from plan):**
  - **E1** — architecture §Standard Contracts Hook contract / `decision_body` mapping: a question first raised by
    PermissionRequest stays `null`, its reason measured statically; retire "its body is unmeasured".
    - **Carried.** Fact: Doc-comment changes, Spec claims disproved 3, Residual (Decisions).
    - Search: `grep -n 'first raised by PermissionRequest' .andromeda/*.md` finds architecture.md:91 (1) and
      test-plan.md:779 (1). `grep -c 'unmeasured'` finds architecture 1 and test-plan 1, both about a key typed after
      a resize (`:47`, `:621`), a different subject. So the "its body is unmeasured" wording lives only in the two
      product files, which this chunk rewrote.
    - The master sites are architecture.md:91 (the "owed to" clause) and architecture.md:308. :308's list of `null`
      at once ("wheel `human`, unverified CLI, or another dialog already pending") omits the question first raised by
      PermissionRequest, which the wrapper also logs and answers `null` at once (`src/run/dialog.rs` `unanswerable`;
      witnessed by the new case's step (d)).
  - **E2** — test-plan §6 Path 4 Surfaces: the `permission` kind is end to end, as landed in `tests/cli_answer.rs` over
    `fixtures/fake-scripts/path4-permission.json`: allow, deny + message, the activity line, the question first raised
    by PermissionRequest, the unstamped negative. Retire "unit / insta only … owed to 'Permission end to end'".
    - **Carried.** Fact: Symbols (test), Schema (script), Spec claims disproved 4.
    - Search: `grep -c 'Permission end to end'` finds test-plan 3 (:757, :779, :781) and architecture 1 (:91). The
      :781 step text ("the step owed to 'Permission end to end'") names the target single-script form. As landed, the
      permission step is a separate gated script (plan Provenance, Leaned).
  - **E3** — test-plan §6 Path 3 Surfaces: the dialog kinds' end-to-end wake witness has landed (`question` / `plan` in
    `path4_dialogs_…`, `permission` in `path4_permission_…`).
    - **Carried.** Fact: Spec claims disproved 4. Site test-plan.md:757 (the `grep -c 'Permission end to end'` hit
      above).
  - **E4** — `matrix#v1-30 notes` — 2026-10-05 (2026-10-05-permission-end-to-end): the acceptance clause "the dialog
    kinds' end-to-end wait witness is owed to working-route.md:78" is met.
    - **ledger-note — owner P7.3.** `question` / `plan` in
      `tests/cli_answer.rs::path4_dialogs_are_logged_once_woken_and_answered_by_id`, `permission` in
      `tests/cli_answer.rs::path4_permission_is_logged_once_woken_and_answered_by_id`.
    - `v1-30` is `verified` (chunk 2026-10-04-wait-and-last), so `matrix.py claim` refuses it (research M5).
- **Coverage of new surfaces:**
  - `fixtures/fake-scripts/path4-permission.json` (test data) → validation schema✓ (`fake-script.v1.json`, hygiene
    walk) · instrumentation n/a · PII n/a (variant names only; payloads are the committed scrubbed recordings) · tests
    integ (2 cases) · a11y n/a · tokens n/a.
  - `permission` end to end over the existing cli/channel surface (no new surface) → validation✓ (the existing
    `Response::parse` drops unknown fields; `allow` + `updatedPermissions` + `suggestion` emits `decision.behavior`
    alone) · instrumentation✓ (asserted: `dialog-raised` ×3, `dialog-answered` ×2, `hook-invoked` / `hook-decision` per
    id, wait `channel-response` `outcome:"permission"` ×3) · PII redacted✓ (`assert_logs_clean`: no canary in any
    home-level line, diag-line schema) · tests integ on all three CI OSes · a11y n/a · tokens n/a.

## Deviations from intent
- **F1 run with `--features fake-agent`.** The plan's command lacks it and the target requires it (Spec claims
  disproved 2).
- **F1 evidence form.** The evidence keeps nextest's verbatim padded lines AND a table with the padding collapsed (the
  :86 record's form). Gate entry 12's unpadded atom reads the table row (Spec claims disproved 1; a dated plan
  correction, 2026-10-05, on the operator's word at this wrap).
- **`relayed()` renamed `fixture()`**, as plan Step 4 asked ("a name true for both sets").
- **Raw run outputs moved out of the committed run dir.** The F1 nextest logs and the smoke JSON held absolute host
  paths. They went to the session scratchpad; their facts are in `evidence/` and this report.
- scope record: none — `gate.py scope` clean (`changed 4 · listed 4 · recorded 0`), 0 recorded.

## Decisions & corrections
- **W3d-b kept `null`** (plan Leaned, the overseer's word at P5 review, 2026-10-05). A question first raised by
  PermissionRequest is logged once and answered `null` at once; a late `answer` is refused exit 13 `unknown-dialog`.
  - **Residual** (the operator's carry at this wrap): the body is measured statically. A PermissionRequest `allow`
    with `updatedInput` (the input plus `answers` and any `annotations`) would satisfy AskUserQuestion on 2.1.288 and
    2.1.287.
  - Building it needs a 15th ledger row, its `viola verify` probe and a re-stamp of both versions: ≥ 2 × (3 + 1) = 8
    live sessions against the 3 left of the founder's cap of 16 (research M3). Judged not worth it now.
  - The case is reachable, rarely: parallel AskUserQuestion calls (one arm per tool, `src/run/dialog.rs` arms
    `retain`), a wrapper restart between the two hooks, or a PreToolUse hook that failed open.
- **Research M6 (wrap note):** a PermissionRequest `allow` WITH `updatedInput` would also satisfy ExitPlanMode on both
  installed CLIs. `dialog.rs`'s "a PermissionRequest `allow` is ignored for `ExitPlanMode`" holds for viola's bare
  `allow` (measured live on 2.1.288 at the previous chunk) and is left as is.
- **F1 — the designed-floor exception** (the operator's carry; the overseer-decided fold, inputs#I1). The
  `verify_window_` class's 45 s override now has its planted-hang control:
  - under `--profile ci`: `SLOW [> 15.000s]`, `SLOW [> 30.000s]`, then `TIMEOUT [  45.004s]`;
  - under the default profile: `SLOW [> 60.000s]`, then `PASS [  80.480s]`.
  With the :86 20 s pair, both moved bounds kill a planted hang. The plan's curation text for
  `.claude/rules/testing.md:70` is P3's.
- **Plan defect, dated correction 2026-10-05** (the operator's word at this wrap): gate entry 12's atom misses
  nextest's padded TIMEOUT. Sweep hazard: nextest status lines pad durations inside the bracket (`[  45.004s]`), so a
  grep for `\[4x.` written from a hand-collapsed record never matches a verbatim line. Write the atom `\[ *…`.
- **Run-dir hygiene during implement:** a raw `agent-run.sh boot` JSON holds the session home's absolute path, and a
  nextest log holds compile paths. Keep such captures in the scratchpad and record the figures.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests, arch) Logged once as `permission` (`source: hook`, `data {dialog_id, tool:"Bash", input}`), the parked
  `wait` printing `permission  builder  dialog {id}  cursor {end}` with no ESC, `wait --json` without `--after`
  returning it at once with that cursor — **met**, `path4_permission_is_logged_once_woken_and_answered_by_id`
  steps (a) and (e), green on windows-2025 / macos-latest / ubuntu-latest (ci#37333536319).
- (tests, arch) `allow`, then `deny` + `message` on a second dialog; each `answer` exits 0 printing
  `answered  builder  dialog {id}`; the hook prints the exact `allow` body and the `deny` + `message` body, exit 0,
  empty stderr — **met**, steps (b) and (c).
- (arch, obs) The PostToolUse `activity` line wakes no parked `wait`; the wait parked across it returns the second
  `permission` — **met**, step (c) (it waits on the `activity` line before releasing the next step).
- (matrix) `v1-16`: `allow` with `updatedPermissions` and `suggestion` emits `decision.behavior` alone — **met**,
  step (b); `implemented`, flipped at P7.
- (tests, security) A question first raised by PermissionRequest is logged once as `question`; the hook prints
  nothing; a late `answer` is refused exit 13 `not-delivered` / `unknown-dialog` — **met**, step (d).
- (security) Unstamped: the permission is logged, the hook prints nothing, `answer` `allow` is refused exit 12
  `unverified-cli`, and no hook receipt carries a body — **met**,
  `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused`.
- (obs) One `dialog-raised` per dialog (`corr` = ids, `dialog_kind` permission/permission/question, `hook_event`
  `permission-request`); two `dialog-answered` (`from:"overseer"`, `from_trust:"self-reported"`); all three wait
  `channel-response`s `outcome:"permission"`; `hook-invoked` + `hook-decision` per id (`decision_emitted` true, true,
  false); `assert_logs_clean` — **met**, step (e).
- (tests) "unmeasured" is gone from `dialog.rs` and `src/run/dialog.rs`, replaced by the static measurement and the
  missing ledger row; the guard shows only comment lines changed — **met** (`grep -c unmeasured` in both files → 0;
  guard entry 14 green).
- (tests) `cli_answer.rs`'s header names no `:84` owner and describes both fixture sets — **met**.
- (tests) F1: `TIMEOUT [4x.xxxs]` under `--profile ci`, the no-kill contrast under the default profile, the plant
  reverted — **met**, with the entry-12 atom matching the table's collapsed form (Spec claims disproved 1).
- (tests) CI green on the final HEAD, its run id in `evidence/` — **met**, ci#37333536319 on `c06caf3`
  (`evidence/operator-pass.md`).
- (tests) Zero live `claude` sessions; no ledger row, stamp, fixture, test seam, nextest bound or schema change —
  **met** (the `git diff --quiet` guard; no session started).

Gates (implement's run, `.andromeda/runs/2026-10-05T15-22-03-implement/`, 16 green · 0 red · 3 not run — leg operator):
- `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
  green.
- `bash scripts/agent-run.sh run --unit --filter 'test(/dialog::tests::/)'` green · `run --unit` green.
- `run --integration --filter 'binary(cli_answer) | binary(contract_fixture_hygiene) | binary(contract_fake_agent_drift)'`
  green · `run --integration --filter 'test(/path4_permission_is_logged_once_woken_and_answered_by_id|path4_unstamped_permission_is_left_to_the_human/)'`
  green (`"passed":2,"failed":0`; P5 baseline red, 0 selected, its red-before-green control) · `run` green.
- Smoke `cleanup` / `boot` / `status` / `cleanup --session p-permission-smoke` green.
- Probes:
  - the `grep -cE 'TIMEOUT …'` atom: green, exit 0, count 1, on the collapsed table row (correction 1);
  - `git diff --quiet 2bd08e949138 -- …`: green;
  - the comment-only guard: green, exit 1 with last line 0;
  - `! (git diff … '*.rs' | grep -E '^\+.*(#\[ignore|retries|test\.skip|std::env::var|thread::sleep)')`: green,
    no output.
- `bash scripts/agent-run.sh pre-push` green (39.84 s, re-run green on the final tree, 39.67 s: coverage 1612/1612,
  playwright 1/1, gate no breaches).
- `gate.py hygiene` — `leg = 'operator'`, by hand: `hygiene: clean`, read twice (`evidence/operator-pass.md`).
- `git … push origin HEAD` — `leg = 'operator'`: exit 0, `2bd08e9..c06caf3`.
- `ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`: `verdict: green · checks 15/15`, ci#37333536319.
- Smoke (P3): driven by hand, boot → status (`ready`, builder alive) → cleanup (`processes_gone:true`,
  `endpoint_gone:true`). The boot path is unchanged.

Watches: none folded.

Outcome basis: the operator pass ran.
- The pass's commits: `c06caf3` alone, no fix commits.
- The final HEAD's CI: ci#37333536319 green, recorded in `evidence/operator-pass.md`.
- Implement's P4 report (this conversation) for what only it holds.

Process hygiene (implement P4's census, re-measured at this wrap: `ps` finds no `viola-harness`, `viola-fake-agent` or
`target/harness/debug/viola` process):

| Process | Started by | Final state |
|---|---|---|
| Harness sessions `p-permission-smoke` and `p3-permission-smoke` (supervisor, wrapper, fake agent) | implement | terminated |
| F1 nextest runs | implement | ended |
| Gate block, `pre-push`, `ci.py` | implement / the operator pass | exited |
