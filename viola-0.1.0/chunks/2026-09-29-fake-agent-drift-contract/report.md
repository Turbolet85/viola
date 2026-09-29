# Report — 2026-09-29-fake-agent-drift-contract

**Chunk:** Fake-agent drift contract — the fake agent's hook sequences and payloads equal the recorded `viola verify`
fixtures per CLI version; the receipt `size` resize oracle made change-driven; the cross-session-message tag filed as
harness origin (founder-ratified widening).
**Date:** 2026-09-29
**Commits:** `055adf4 chore(2026-09-29-fake-agent-drift-contract): operator pre-CI commit, for the run this chunk's
verdict reads` (the only commit since `last_wrap` 2026-09-29T12:45:02Z; base `2d8bc53`, the prior wrap).

## Changes (structured — detectors read this)
- **Files** (basis: `gate.py scope` against base `2d8bc53`: changed 7 · listed 6 · recorded 1):
  - new `tests/contract_fake_agent_drift.rs`;
  - modified `src/bin/viola-fake-agent.rs`, `crates/viola-agent-claude/src/hook.rs`, `Cargo.toml` (one `[[test]]`
    entry), `tests/cli_fake_agent.rs`, `tests/cli_instance_state.rs`, `tests/tui_pty_seam.rs`;
  - the chunk folder (`scope-record.md`, `evidence/{pre-fix-reds,host-reds-two-sided,operator-pass}.md`, this report).
- **Symbols / APIs:**
  - `viola-agent-claude::hook` — `HARNESS_PREFIXES` (crate-private const) goes from 2 to 4 entries:
    `<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message`. The match stays
    a raw `starts_with` with no trim in `prompt_origin` (private; its one caller is `data_of`, reached through the
    public `normalise`, signature unchanged). `prompt_origin`'s doc states the rule and its one exception: a typed tag
    arrives escaped and never classifies, except the cross-session tag, whose escaped form is itself the CLI-injected
    one, so a human who types that tag at a prompt's start is filed `harness`. `origin` stays within
    `driver` · `human` · `harness`; no new event kind or `data` field.
  - `viola-fake-agent` (test-only `[[bin]]`, feature `fake-agent`):
    - `payload(fixture, prompt)` re-appends the fixture's trailing `\n` after re-serialising the object with `prompt`
      set, when the fixture ended with one. Its one production caller is `Agent::fire`.
    - `run_hook`: the `ran:true` `hook` receipt line gains `stdin_hex` = hex of the exact bytes the agent offered on the
      hook's stdin. The two `ran:false` arms are unchanged.
    - new `watch_size(Arc<Agent>)`: a thread `main` starts after `start_receipts` in the interactive (non-print) path;
      it writes the current `viola_pty::host_size()` as a `size` line at once, then polls on `CONTROL_POLL` (10 ms) for
      the life of the process and writes a line on every change; nothing while the size is `None`. It is now the ONLY
      size mechanism: `read_stdin` no longer calls `receipt_size` (its two calls, before the first read and after
      each byte, are removed). `receipt_size` is the watcher's one step. Print mode (`-p`) starts no watcher.
  - Root test `contract_fake_agent_drift`: `drift(fixtures_dir, version, receipt_lines) -> Vec<String>` (test-local)
    returns `order` when the `hook` events are not the spine literal, then each event whose `stdin_hex` bytes are not
    that event's recorded `<Event>.default.json` bytes.
- **Crates / modules:** none added or removed; `viola-agent-claude` gains no dependency.
- **Dependencies:** none (insta is NOT added — the P4 fork; `grep -n insta Cargo.toml crates/*/Cargo.toml` → 0).
- **Schema / config:** the fake-agent receipt (`<…>.receipt.ndjson`, test-only, no schema file — `ls schemas/`:
  `claude-fixture.v1.json`, `diag-detail.v1.json`, `diag-line.v1.json`, `fake-script.v1.json`): the `hook` kind gains
  `stdin_hex` on a hook that ran; the `size` kind is written at start and on every change by a watcher (was: at start
  and before a byte read after a change). No product schema, config key, env var or `--capture` registrant changed.
- **Spec-master edits:** none during the chunk (the wrap applies them).
- **Counts / qualifiers moved:** the harness-prefix set, 2 → 4 (stated with the two literals at architecture.md:70
  [Human Takeover / Wheel] and test-plan.md:270, :872, :1123/:1132 — `grep -n 'task-notification\|agent-message from='`:
  architecture 1 line, test-plan 4 lines). No master states a suite test count (`grep -n '938\|948'` over
  test-plan/architecture → 0).
- **Dev-tool versions:** none — `claude --version` re-read `2.1.283 (Claude Code)` at the phase (research.md), equal
  to the recorded fixture set; no host tool installed or changed.
- **Harness / gate surface:** a new root integration binary `contract_fake_agent_drift` (`[[test]]`,
  `required-features = ["fake-agent"]`), picked up by `run --integration` / the default `run` / coverage. No
  agent-run verb, CI step, status or verdict shape changed.
- **Cross-project / external claims:**
  - The cross-session tag's literal and the claim that the CLI injects it ESCAPED (`<\cross-session-message from="…"
    from-name="…">`) rest on (a) `D:/dev/projects/additional/viola-lab/prototype/src/hook.rs` (outside this repo, read
    at the phase: its comment "measured 2026-09-28/29 on andromeda-worker, overseer1's F115", its test) and (b) the
    overseer's relay. No recorded fixture or ledger row in this repo witnesses it (`verify`'s `claude -p` probe records
    only the four spine events). It is a relayed measurement — a HYPOTHESIS until "First live test and self-drive"
    (:76) measures a real cross-session UserPromptSubmit `prompt`. The CLASSIFICATION DECISION is not relayed: the
    founder ratified the widening live, 2026-09-29 15:21:44, shown the widening and its side effect (relay: the Viola
    overseer; plan.md §Provenance P4 fork 1).
  - CI: ci#36583175440 on `055adf4` (the pre-CI commit; this wrap's own commit adds to that tree) — `verdict: green ·
    checks 15/15 · wall 270 s` (`ci.py conclusion --sha HEAD --wait 1800`, entry 18; confirmed by the overseer). The
    prior wrap's `2d8bc53`: ci#36571737771 green 15/15 (read at this wrap's P1).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. test-plan.md:1336 (§6 Contract suite) "… must equal the recorded fixture order and payloads, pinned with insta",
     :438 "insta 1.48.0 in check mode pins decision bodies and the fake agent's hook sequences", :1359 "Decision bodies
     / fake-agent transcripts | insta snapshots" — for the FAKE-AGENT half: insta is in no manifest (`grep -n insta
     Cargo.toml crates/*/Cargo.toml` → 0 lines; `grep -c 'name = "insta"' Cargo.lock` → 0), and the contract pins
     against the recorded fixture files byte for byte with the spine order a test literal (P4 fork 4). The
     decision-body half (:438, :839, :866, :1099) is future dialog-chunk tooling, not measured here.
  2. test-plan.md:1365 (§7 Fake agent) "Hook `matcher`s are not evaluated yet; that lands with the [fake-agent drift
     contract]" — the entry's CARRY premise "where recorded `tool_name`s exist" is false at HEAD: the four
     `fixtures/claude/2.1.283/*.json` carry no `tool_name`, and `plugin/hooks/hooks.json` registers no `matcher`
     (research.md). Matcher evaluation moves to "Dialog answers by dialog_id" (:72).
  3. The route entry's "annotations forwarded" (working-route :63 title): the S8 question-answer path (PreToolUse
     `updatedInput.answers` + `annotations`, test-plan :866/:1100; architecture :76/:91 "S3/S7/S8 … with dialog
     answers") does not exist at HEAD, so it cannot be witnessed here; it moves to :72.
  4. test-plan.md:1371 `size` "at start and again whenever the terminal size changed (the resize oracle)" was FALSE at
     the chunk base (CARRY 2; research.md §Mechanism re-derivations, re-verified at `2d8bc53`) and is now TRUE by code
     (the watcher) — the wording stays; the mechanism note ("by a watcher, no key needed") is added.
- **Expected amendments (from plan)** — each with its locating search:
  1. architecture §Established Decisions [CLI Version Compatibility] "Harness prompt prefixes" (:80) / "Tag escaping"
     (:89) rows and [Human Takeover / Wheel] (:70): the prefix set gains `<\cross-session-message` and
     `<cross-session-message`; escaped means typed, except the cross-session tag — **carried** (Symbols / APIs;
     Counts moved). Search: `grep -n 'task-notification\|agent-message from='` architecture → 1 line (:70);
     `grep -n 'Harness prompt prefixes\|Tag escaping'` → :80, :89.
  2. security-plan §Security Anti-Patterns → Code Patterns and §Security Decisions Log: the same widening, recorded as
     the founder's (relay: the Viola overseer), its side effect named — **carried**. Search: `grep -n 'harness
     prefixes'` security-plan → 1 line (:573, "NEVER build … harness prefixes … from runtime or upstream text" — still
     true: the list is compiled); the Decisions Log gets the new entry.
  3. test-plan §4 viola-agent-claude (M2 prefixes → four; :872) and §6 Path 5 (the harness-injected prefixes; :270,
     :1123, :1132) — **carried**. Search: `grep -n 'task-notification\|agent-message from='` test-plan → 4 lines.
  4. test-plan §6 Contract suite (:1336): "pinned with insta" → "compared byte for byte against the recorded fixture
     files, the spine order a test literal"; the S8 `annotations` clause owned by "Dialog answers by dialog_id" —
     **carried** (Spec claims disproved 1, 3). Search: `grep -n 'pinned with insta'` test-plan → 1 (:1336); `grep -nw
     insta` test-plan → 11 lines, of which the fake-agent ones are :438, :1336, :1359.
  5. test-plan §7 Fake agent: matchers deferred to "Dialog answers by dialog_id" (:1365, was "lands with the fake-agent
     drift contract"; also :1800 "matchers deferred"); the `hook` receipt gains `stdin_hex`; `size` written at start
     and on every change by a watcher, no key needed (:1371); the UserPromptSubmit payload keeps the fixture's trailing
     newline — **carried** (Symbols / APIs; Schema; Spec claims disproved 2, 4). Search: `grep -n 'matcher'`
     test-plan → 3 lines (:887 is the send-confirmation matcher, a different subject — no change); `grep -n 'resize
     oracle'` → 1 (:1371); `grep -n stdin_hex` all masters → 0 (new field).
- **Coverage of new surfaces:**
  - `HARNESS_PREFIXES` cross-session forms → validation closed compiled list, raw `starts_with`, no runtime-built
    prefix (census gate: the tag only in `hook.rs`)✓ · instrumentation n/a (no new event or field; `origin` existing) ·
    PII n/a · tests unit (`prompt_origin_files_the_cross_session_tag_as_harness`, 5 cases; red on the unchanged list)
    · a11y n/a · tokens n/a
  - fake-agent `hook.stdin_hex` receipt field → validation n/a (test-only) · instrumentation n/a · PII n/a (scrubbed
    synthetic fixtures, test-home file) · tests integ (`contract_fake_agent_drift`, the two moved pins) · a11y n/a ·
    tokens n/a
  - fake-agent `size` watcher → validation n/a · instrumentation n/a · PII n/a · tests integ
    (`pty_resize_reaches_a_child_that_reads_no_key`, red before the watcher; `pty_resize_reaches_the_child`,
    `tui_passthrough`) · a11y n/a (size lines go to the receipt file only, never the terminal) · tokens n/a
  - fake-agent payload trailing newline → tests unit (`payload_sets_only_the_prompt_field`, both cases) + integ
    (the contract, red before the fix)

## Deviations from intent
- The two moved whole-object pins build their expected `stdin_hex` from a byte literal written in the test, hex-encoded
  inline; no support helper was added. Justification: the plan asked for the bytes "written in the test … never
  computed by calling the fake agent's code"; a 300-char hex literal was unreadable.
- One case beyond the plan's two planted reds: `drift_is_empty_for_the_recorded_set_in_spine_order`. Justification:
  it shows the planted reds are not vacuous (the comparator reads green on the recorded set).
- The watcher keeps the plan's `CONTROL_POLL` (10 ms) cadence. Its idle cost was MEASURED because the host's
  load-bound reds were in question: ≈ 3 ms/s CPU per fake agent in a PTY plus ≈ 1 ms/s in `OpenConsole.exe`
  (20 s idle, a booted session: 46.9 ms and 15.6 ms vs 0 and 0 on the `2d8bc53` control; the wrapper's own 203.1 ms on
  both). Not changed: the quiet-host alone rounds showed the control failing the load-bound tests at least as often.
- The live smoke's receipt-level check was not possible: the booted harness session passes no `--receipt` (only tests
  do). The smoke rests on `status` `ready` and `events.ndjson` lines 1-3 (`session-start{source:hook}` carrying the
  recorded fixture's `agent_session_id`).
- The operator pass pushed through `pre-push`'s windows-tests red (the operator's direction at take-up, "judge local
  reds against a same-day control, with CI as the acceptance leg", and "Go: run the operator pass now").
- Hygiene rewrites of the phase run dir: `baseline/control/src/leak.rs` → `leak.rs.txt` (bytes unchanged; the plan's
  entry-9 `baseline` note still names `src/leak.rs`); `dryrun-p5.txt`'s host paths → placeholders.
- **Scope record** (`gate.py scope` at P1: `clean — changed 7 · listed 6 · recorded 1`):
  - in-intent: `tests/tui_pty_seam.rs` · serves step 5 · self (research's lists predate the P4 forks that added step 5).

## Decisions & corrections
- Operator direction at implement take-up: fold every red into this chunk; judge local reds against a same-day control,
  with CI as the acceptance leg; read CI through the `ci.py conclusion` tool call.
- Operator: `gate v1.5` appearing mid-run is overseer1's V35 deploy (a directory artifact reads the newest mtime under
  it), announced — expected, not drift; the wrap reads the new contract at its start.
- Operator: remove the same-day control worktree after the CI read unless the wrap's light gate needs it (removed after
  entry 18).
- Founder, live 2026-09-29 15:21:44 (relay: the Viola overseer): the escaped AND plain cross-session tag, at a prompt's
  start, is harness origin; side effect accepted — a human typing that tag at a prompt's start is filed harness.
- P4 forks (overseer agreeing): matcher evaluation re-carried to :72; change-driven `size`; fixtures are the pin, no
  insta.
- Finding: the host's local suite grades an IDENTICAL tree differently run to run — the `2d8bc53` control read 44 then
  57 failed across two standalone `run --coverage` runs, and 2 then 0 on a 23-test two-binary run — so one
  subject/control pair cannot separate two trees; reverse-order pairs plus alone-rounds on both trees closed it.
- Sweep hazard: `grep -c insta` counts "instance" (obs-plan 169, test-plan 122 hits); the word form `grep -nw insta`
  reads test-plan 11 / architecture 1 / others 0.
- Sweep hazard (recurrence): the Bash guard refused a command carrying a doubled backslash (a drive-path regex) at the
  hygiene rewrite — host-win32.md Session Additions 2026-09-28 (extended 2026-09-29) already states it; the rewrite went
  through a Write-tool script by path.
- Tool fact: a `pre-push` / `run --coverage` log carries binary bytes, so `grep` reports "Binary file … matches" and
  JSON extraction needs a `utf-8, replace` decode (`grep -a` for lines).

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests) contract per recorded set (one: `2.1.283`), literal spine order, `ran:true`/`exit_code:0`, `stdin_hex` ==
  fixture bytes, empty walk fails — **met**: `contract_fake_agent_drift` 4/4 in the archived integration JUnit
  (run-archive 394, fresh), in the WSL coverage leg (948/948) and CI.
- (tests) a deliberate drift turns it red; pre-fix red on UserPromptSubmit alone, the trailing `0a` — **met**: the two
  planted cases; the pre-fix red and the hand byte-diff (`fixture = sent + b'\n'`) in `evidence/pre-fix-reds.md`.
- (tests) `claude_fixtures_pass_hygiene` green; no new fixture — **met**: absent from every failure list; the diff
  touches nothing under `fixtures/`.
- (tests) key-free resize to 100×30 reaches the receipt; red before step 4; `pty_resize_reaches_the_child` and
  `tui_passthrough` stay green — **met** for the witness (red recorded, then green in every run) and
  `pty_resize_reaches_the_child`; `tui_passthrough` is green alone (7/7 ×3) and in CI on all three OSes, and red under
  the host's parallel runs on BOTH trees → UNMET locally only by a `red — not this chunk's` gate, owner the host-reds
  CARRY (P5).
- (security/arch) the cross-session prefixes classify `harness`, mid-prompt / leading-space stay `human`, table red on
  the unchanged list, `prompt_origin_reads_the_raw_prefix` green, `origin` closed — **met**.
- (security) the tag only in `hook.rs`, no runtime-built prefix — **met** (entry 9 green).
- (arch/security) changes stay in `src/bin/viola-fake-agent.rs` + root `tests/`; `viola-agent-claude` no dependency and
  pure; no env var; no `--capture` registrant — **met** (the diff: `Cargo.toml` gains only the `[[test]]` entry).
- (security) `scripts/wsl-provision.sh` and `.github/workflows/ci.yml` byte-unchanged against `2d8bc53` — **met**
  (entry 10 green).
- (obs) no new `event` value, field, `parser` or `detail` code; clippy `-D warnings`; G2/G4 green in CI — **met**
  (entry 2 green; CI 15/15).
- (a11y) tui boundary cases green on three CI legs; watcher lines to the receipt only — **met** (CI 15/15;
  `Receipt::write` targets the `--receipt` file).
- (tests) `agent-run run` and `pre-push` read `"ok":true`; a local red judged against a same-day control; CI green on
  15 — **CI met** (ci#36583175440, 15/15); the local reads are `red — not this chunk's` (below).
- (matrix) no capability claimed — **met** (`matrix.py show --chunk`: claimed 0).

Gates (`gate.py run`, run dir `2026-09-29T13-30-05-implement`, by `run` text):
- `cargo fmt --all --check` — green.
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green.
- `bash scripts/agent-run.sh run --unit` — green (750/750).
- `… run --unit --filter 'test(/prompt_origin_files_the_cross_session_tag_as_harness/)'` — red (the 2 `harness`
  cases, recorded) → green.
- `… run --integration --filter 'binary(contract_fake_agent_drift)'` — red (UserPromptSubmit, recorded) → green.
- `… run --integration --filter 'binary(tui_pty_seam) & test(/pty_resize_reaches_a_child_that_reads_no_key/)'` — red
  (WITHIN, recorded) → green.
- `… run --integration --filter 'binary(tui_pty_seam) | binary(tui_passthrough) | binary(cli_fake_agent) |
  binary(cli_instance_state)'` — `red — not this chunk's: the same failures by name and message on the 2d8bc53 tree
  today (subject 10 and 12 failed ⊆ control 16; evidence/host-reds-two-sided.md) → the host-reds CARRY (P5 pin)`.
- `bash scripts/agent-run.sh run` — `red — not this chunk's: control 56 of 230 vs this chunk 58 of 235; the 6
  chunk-only reds fail before the fake agent's first receipt line and are green alone 7/7 ×3; contract_ledger_probes
  red on both trees, green alone 3/3 → the host-reds CARRY (P5 pin)`; the archived JUnit carries
  `contract_fake_agent_drift` 4/4 (fresh).
- `! (grep -rl --include='*.rs' 'cross-session-message' crates src tests | grep -vx
  'crates/viola-agent-claude/src/hook.rs')` — green (no output).
- `git diff --quiet 2d8bc53 -- scripts/wsl-provision.sh .github/workflows/ci.yml` — green.
- `cargo deny check` — green.
- `bash scripts/agent-run.sh pre-push` — linux-tests green (coverage 948/948, playwright 1/1, `gate` green; vm
  terminated); windows-tests `red — not this chunk's: its stage command run --coverage as two reverse-order standalone
  pairs (control 44 / chunk 55; chunk 61 / control 57) and the residual two binaries alone, three rounds per tree
  (control 2 reds, chunk 1, the same test on the same 1.0 s bound) → the host-reds CARRY (P5 pin)`.
- `… cleanup --session p-fadc-smoke` · `… boot --session p-fadc-smoke --instance builder` · `… cleanup …` — green
  (boot readiness through `events.ndjson` line 3; cleanup `processes_gone`, `endpoint_gone`).
- `python … gate.py hygiene` (leg operator) — refused 3 → rewritten → `clean` (evidence/operator-pass.md).
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (leg operator) — exit 0, `2d8bc53..055adf4`.
- `python … ci.py conclusion --sha HEAD --wait 1800` (leg operator) — `verdict: green · checks 15/15`, ci#36583175440.
- Smoke (P3, boot path changed): ✓ — boot, `status` `ready`, cleanup clean.

Watches: none folded.

Outcome basis: the operator pass ran — commits `055adf4` only (no fix commit); the final HEAD's CI run ci#36583175440,
recorded in `evidence/operator-pass.md`. implement's conversation is present (this session), its P4 report as given,
plus the operator directives above; post-implement artifacts: `evidence/operator-pass.md`.

Process hygiene: every process this chunk's runs started terminated — implement's census and this wrap's re-read at
P1 (`Get-CimInstance Win32_Process` for `ExecutablePath` under `D:\dev\projects\viola*`: 0; the same query counts the
Git processes, so it reads). The control worktree `<repo parent>/viola-ctl-2d8bc53` was removed after entry 18.
