# Report — 2026-10-04-dialog-answers-by-dialog-id

**Chunk:** Dialog answers by dialog_id — question, permission and plan through hooks, answered by a wrapper-assigned
dialog_id, one pending dialog, deadline expiry, unknown-dialog, withheld on an unverified CLI; the dialog-kind wait
witness; two folded CI reds
**Date:** 2026-10-04T16:55Z
**Commits:** `7f1364f` chore(…): operator pre-CI commit · `2080e3f` fix(…): a Windows strict-modes measurement test naming
the refused ACE · `2484b77` fix(…): a home outside the profile gets the protected owner-only DACL at creation · `ca69e84`
fix(…): the unreadable-stamps stand-in made inside a home viola created (basis: `git log --format='%h %s' c540254..HEAD`)

## Changes (structured — detectors read this)
- **Files:** 61 files, +4477/−110 outside `.andromeda/` and the version dir (`git diff --stat c540254 -- . ':!.andromeda'
  ':!viola-0.1.0'`). New: `src/cmd/answer.rs`, `src/run/dialog.rs`, `crates/viola-agent-claude/src/dialog.rs`,
  `crates/viola-agent-claude/src/snapshots/*.snap` (6), `crates/viola-state/src/strict.rs`, `tests/cli_answer.rs`,
  `fixtures/fake-scripts/path4.json`, `fixtures/claude/2.1.287/` (4 spine `*.default.json` recorded, 4 relayed dialog
  fixtures, `RELAYED.md`). Modified: per research §Files to modify plus the 18 scope-record lines (Deviations).
- **Symbols / APIs:**
  - viola-core: `DIALOG_DEADLINE = 60 s` (PROVISIONAL) beside `SPINE_DEADLINE`.
  - viola-agent-claude: `HookEvent::{PreToolUse, PermissionRequest}` (ALL 7→9; `kind()` now `Option<EventKind>`,
    `None` for the dialog tier; `is_dialog()`); `AgentError::{NotAnEvent, DialogMalformed, ResponseMalformed}`;
    `ledger::event_name` two arms; new `dialog` module: `DialogKind`, `DialogTool`, `Dialog`, `classify`,
    `Response::{parse, free_text, to_value}`, `decision_body` (S3 / S7 / S8 bodies). `LedgerRow::ALL` unchanged at 6.
  - viola-channel: `Client::request_until(method, params, deadline)` (exchange on its own thread, `Deadline` past it);
    `ChannelError::Deadline`.
  - viola-state: `strict` module (`check_stamps`, `check_path`, `unix_verdict`, `windows_verdict`, `trusted_sid`,
    `WRITE_RIGHTS = 0x500D0046`, `Refused`); `stamps::read_stamps_strict` + `StampsReadError`; `snapshot::PendingDialog`
    and `InstanceSnapshot.pending_dialog` (additive); `fs::outside_profile`; `fs::create_private_dir` now, on Windows,
    gives the topmost folder it creates outside the user's profile folder the protected DACL
    `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` (profile from `GetUserProfileDirectoryW`, canonical paths, no env read;
    a DACL that cannot be set fails the creation).
  - root bin: channel methods `hook.dialog` `{kind, data, hook_event, tool?, input?, continuation?}` → `{dialog_id,
    response}` and `answer` `{dialog_id, from?, response}` → `{}` now SERVED (were `-32601`); `src/run/dialog.rs`
    `DialogSlot` (counter restored in `WaitFeed::rebuild`'s pass — `rebuild` now returns the highest `dialog_id`; one
    pending; Condvar await on the injected clock; per-tool armed continuation matched on equal `tool_input`);
    `WaitFeed::{appending_dialog, dialog_settled}` + the pending-dialog `after`-less start; `viola hook
    pre-tool-use|permission-request` (dialog path, read bound `DIALOG_DEADLINE + 5 s`, one stdout write); new verb
    `viola answer <name> <dialog_id> [--file] [--json]` (exits 0/1/2/12/13/20/21); `human::{write_answered,
    answer_hint}`; `send::{exit_of, reason_of}` made `pub(super)` (shared with `answer`); `version_gate` reads stamps
    through `read_stamps_strict` (a refusal → `cli_verified:false`, `parse-rejected{ledger-stamps,
    strict-modes-failed}`).
  - fake agent: `hook_commands(…, tool)` evaluates a group's `matcher` against a tool-bearing payload's `tool_name`;
    `DEFAULT_CLI_VERSION` 2.1.287.
  - harness: `perf::ROWS` 4→5 (`pre-tool-use`), the perf session boots unstamped; mutants `--build-timeout=400`.
- **Crates / modules:** new modules `viola_agent_claude::dialog`, `viola_state::strict`, `viola::run::dialog`,
  `viola::cmd::answer`; no crate added or removed.
- **Dependencies:** `insta =1.48.0` (default-features off) in `[workspace.dependencies]`, root and viola-agent-claude
  dev-dependency; viola-state takes `libc` (unix) and `windows-sys` (windows), both already workspace-pinned; the
  windows-sys pin gains feature `Win32_UI_Shell`. `cargo deny check` green.
- **Schema / config:** `plugin/hooks/hooks.json` gains PreToolUse (`matcher` `AskUserQuestion|ExitPlanMode`) and
  PermissionRequest, `timeout` 75; `schemas/diag-line.v1.json` `parse-rejected.detail` enum gains `strict-modes-failed`;
  `schemas/claude-fixture.v1.json` unchanged (its description still names only `viola verify` recordings — not edited).
- **Spec-master edits:** none (all owed to this wrap — Expected amendments).
- **Counts / qualifiers moved:** hook events registered 7→9 (`HookEvent::ALL`, `hooks.json`); perf rows 4→5
  (`perf::ROWS`); test fixture default 2.1.283→2.1.287 (three constants); fixture versions committed: 2.1.283, 2.1.287;
  `parse-rejected` detail codes +1; `LedgerRow::ALL` stays 6 and `viola verify`'s counter stays `/06` (verified:
  `git diff c540254 -- crates/viola-agent-claude/src/ledger.rs` adds no row arm, gate entry read 0).
- **Dev-tool versions:** `claude` (the agent CLI) on the dev host: mise installed 2.1.288 at ~14:19Z (2026-10-04); every
  running session, this builder's included, still runs 2.1.287; the spine recording was taken against the pinned
  2.1.287 binary. Rust toolchain 1.98.1: `x86_64-pc-windows-msvc` std target added on the dev host (type-check only).
- **Harness / gate surface:** `run --perf` times five rows on an unstamped session; `gate --require perf` requires five;
  `.github/workflows/windows-mutants.yml` viola-state file list gains `crates/viola-state/src/strict.rs`.
- **Cross-project / external claims:**
  - viola-lab prototype captures `~/.viola/sessions/*/events.ndjson` (outside this repo): relayed fixture sources
    `viola-builder` lines 884/888 and `andromeda-worker` lines 124/129; 8 of 8 PermissionRequests repeat their
    PreToolUse's `tool_input` (basis `.andromeda/runs/2026-10-04T12-15-37-phase/capture-pairs.py`).
  - Print-mode measurement on `claude` 2.1.287: no `AskUserQuestion` / `ExitPlanMode` in the tool list, no dialog hook,
    in default, plan and `--tools` modes (`evidence/print-mode-dialog-probe.md`).
  - CI: ci#37213772796 `7f1364f` red (windows) · ci#37214281447 `2080e3f` red (measured) · ci#37217684692 `2484b77`
    red (1 test) · **ci#37218087331 `ca69e84` green 15/15** (`evidence/operator-pass.md`).
  - windows-2025 runner measurement: the workspace drive grants BUILTIN\Users `0x2` and `0x4`, container-inherit, on
    every folder below it (`evidence/operator-pass.md`).
- **Reverted / negative API facts:** the dialog probe, four ledger rows (S3/S7/S8/concurrency) and the `/06`→`/10`
  counter were planned and never written (founder ruling R1); the first spine recording landed under
  `fixtures/claude/2.1.288/` and was moved out of the repo by the overseer.
- **Insufficient fixes (written, kept, not the remedy):** `2080e3f` (the Windows measurement test) — kept as a guard,
  it measured the cause and did not resolve it; `2484b77` resolved it.
- **Spec claims disproved by measurement:**
  - plan (first pass) step 7 / Constraints: "a print-mode probe can raise the dialog hooks" — false on 2.1.287
    (`evidence/print-mode-dialog-probe.md`); disposed by the phase revision.
  - scope §2: "one plan dialog spans two hooks" holds for `question` too (5 PermissionRequest `AskUserQuestion`
    captures) — premise-corrected in scope at the revision.
  - plan step 11: `parse-rejected{… detail:"strict-modes"}` — the closed schema admits no such code; landed as
    `strict-modes-failed` (catalog spelling) — disposition owed: obs-plan §6 amendment.
  - plan step 8 (as revised): "17 lines in 7 files" for the version move — re-derived 16 lines in 7 files, 6 moved
    (`grep -rn '2\.1\.283' src crates tests --include=*.rs` at phase).
- **Expected amendments (from plan):** (site search per line; hit counts are `grep -c` per master at this wrap)
  - security-plan §Authentication & Authorization + Anti-Patterns — sixth dated gap (`answer` / `hook.dialog` frames):
    carried (Symbols: the two methods served, liveness-only `live_endpoint`); search `fifth dated gap|fourth dated gap`
    security-plan 1 (other masters 0).
  - security-plan §Anti-Patterns › Universal + architecture [CLI Version Compatibility] — non-null decisions on the
    6-row stamp until `:82` (founder ~14:08Z): carried (Counts: ALL stays 6; Symbols: decisions flow when
    `cli_verified`); search `stamps.json.*without|stamps read` architecture 1, security-plan 4.
  - security-plan `~/.viola/` access control + Own state files — `run`'s stamps read leaves the second gap: carried
    (Symbols: `read_stamps_strict` in `version_gate`); same search.
  - **added by the founder's ruling (~16:40Z):** security-plan `~/.viola/` access control — the creation half for a home
    outside `%USERPROFILE%` landed (protected user + SYSTEM DACL): carried (Symbols: `fs::create_private_dir`); search
    ``outside `%USERPROFILE%` `` security-plan 4, test-plan 1.
  - architecture §Standard Contracts `hook.dialog` — the continuation (equal `tool_input`, no event, revise only):
    carried; search `hook.dialog` architecture 9, security-plan 8, test-plan 8, obs-plan 9.
  - architecture §Standard Contracts `answer` — `from` per method; suggestions a v1 limit: carried; search
    `answer.*from|from.*answer` architecture 10, security-plan 5, test-plan 5, obs-plan 3; `suggestion` architecture 1,
    a11y-plan 1.
  - architecture [Hook Contract] — `DIALOG_DEADLINE` 60 s PROVISIONAL, `hooks.json` 75: carried; search
    `DIALOG_DEADLINE|dialog deadline` architecture 2.
  - architecture [CLI Version Compatibility] — fixtures/default at 2.1.287, relayed dialog fixtures: carried; search
    `2\.1\.283` architecture 3, test-plan 2, obs-plan 1, layout-templates 2, design-system 1; `relayed|verify --record`
    architecture 4, security-plan 6, test-plan 1, obs-plan 2.
  - architecture §Occupied Resources — fifth perf row; `answer` a `cli-<name>.ndjson` producer: carried; search
    `Occupied Resources` architecture 3; `perf.*row|four rows|pre-tool-use` architecture 4, test-plan 7, obs-plan 8.
  - test-plan §2 / §7 — relayed captures as a fixture source; §6 Path 4 as landed (`permission` e2e owed `:82`); Path
    3's dialog witness for question/plan; insta in use: carried; search `Path 4` test-plan 1, obs-plan 1, a11y-plan 1.
  - test-plan §10 perf five rows; obs-plan §9 hyperfine five rows: carried; search as above.
  - layout-templates §Component — Primary content block 2: catch-site list gains `answer`: carried; search
    `catch-site|catch site` architecture 2, obs-plan 5, layout-templates 1.
  - plan first-pass items now superseded (revision): test-plan §3 verify `/10`, layout-templates verify wireframe `/10`,
    design-system verify sample — superseded: the counter stays `/06`; search `/10\]|\[0[0-9]/06\]` layout-templates
    2, design-system 1 (unchanged truth).
  - not in the plan, owed: obs-plan §6 detail catalog — `parse-rejected{ledger-stamps}` gains `strict-modes-failed`;
    search `strict-modes-failed` obs-plan 5, design-system 1.
  - matrix: `v1-15` unclaimed at phase (note written there); no ledger-note entry here.
- **Coverage of new surfaces:**
  - `hook.dialog` (channel) → validation closed kind/data/hook/tool/continuation ✓ · instrumentation
    `run.dialog_register`/`run.dialog_await` spans + `dialog-raised` ✓ · PII no question/plan/input text on a home line ✓
    · tests unit + integ (cli_answer) ✓ · a11y n/a · tokens n/a
  - `answer` (channel + CLI verb) → validation `u64` id, closed `response`, `validate_paste_text` both sides ✓ ·
    instrumentation `dialog-answered{from_trust}` + `answer.client` span ✓ · PII answers never logged (canary-asserted) ✓
    · tests unit + integ ✓ · a11y n/a (CLI text, design-system cli patterns) · tokens n/a
  - `viola hook pre-tool-use|permission-request` → validation `take(MAX_FRAME+1)` + `classify` ✓ · instrumentation
    `hook-invoked`/`hook-decision{corr, deadline_hit, detail}` ✓ · PII ✓ · tests unit + integ (cli_answer,
    hook_fail_open) ✓
  - strict-modes stamps read + protected DACL creation → validation n/a · instrumentation `parse-rejected` ✓ · tests
    unit both OS (Windows on CI) + integ (cli_version_gate) ✓

## Deviations from intent
- Phase-level: implement stopped on the first plan's STOP clause; the plan was revised (founder R1 ~11:55Z, R2 ~14:08Z;
  overseer resolutions 1–5); `v1-15` unclaimed (claimed at `:82`).
- Step order: step 8 (relay) ran before steps 3–7 so the overseer reviewed in parallel.
- The relay was re-fired with in-script redaction of `tool_input.plan` after the overseer's review (public repo,
  private plan text).
- The spine recording's run text gained `-- ~/.local/share/mise/installs/claude/2.1.287/claude` (mise moved `claude` to
  2.1.288; operator option 1).
- `parse-rejected` detail spelled `strict-modes-failed` (schema enum extended) instead of the plan's `strict-modes`.
- insta snapshots live in `crates/viola-agent-claude/src/snapshots/` (research named `tests/snapshots/`).
- `tests/hook_events.rs` needed no re-pin (it never pinned the registered set).
- Operator pass: three fix commits after a red Windows leg (measurement, then the founder-ruled creation half, then a
  test stand-in) — `evidence/operator-pass.md`.
- Scope record (`gate.py scope`: `clean — changed 60 · listed 42 · recorded 18`):
  - companion (10): `crates/viola-agent-claude/Cargo.toml` (step 19) · `crates/viola-state/Cargo.toml` (step 11) ·
    `schemas/diag-line.v1.json` (step 11) · `crates/viola-state/src/liveness.rs` (step 12) · `src/cmd/send.rs`
    (step 16) · `crates/viola-e2e/src/harness/gate.rs` (step 17) · `tests/hook_fail_open.rs` (step 14) ·
    `crates/viola-channel/src/lib.rs` (step 14) · `tests/channel_endpoint.rs` (step 12) ·
    `.github/workflows/windows-mutants.yml` (step 11) — all `self`;
  - mechanical (1, ×6): `crates/viola-agent-claude/src/snapshots/*.snap` (step 19) — `self`;
  - in-intent (1): `tests/cli_version_gate.rs` (step 11) — `self`;
  - widening (1): `crates/viola-state/src/fs.rs` (step 11) — word: "pull the creation half forward from :111, so a home
    outside %USERPROFILE% gets the explicit protected user + SYSTEM DACL at creation, exactly as security-plan words
    it, for the WHOLE home and not ledger/ only" — the founder, live, 2026-10-04 ~16:40Z, relayed by the overseer.

## Decisions & corrections
- Founder live rulings: R1 relayed prototype fixtures, no ledger row here, rows + re-probe to `:82`, no PTY probe
  (~11:55Z); R2 non-null decisions on the 6-row spine stamp as a dated gap closed by `:82`, the residual shown, `v1-15`
  at `:82` (~14:08Z); R3 the protected-DACL creation half pulled forward from `:111` for the whole home (~16:40Z).
- Overseer resolutions: spine-only `verify --record` allowed; PermissionRequest(AskUserQuestion) takes the plan-style
  continuation; `permission` kind unit-only; smallest harmless captures reviewed before commit; canary on answer/driver
  text; plan pair redacted in the relay script (public repo); option 1 on the 2.1.288 move.
- Sweep hazard: a tempdir-based Windows test passes strict-modes (`%TEMP%` lies under the profile) while a home under
  the checkout fails it — a Windows DACL test must create its home where the CI homes live (`target/e2e-home`).
- Sweep hazard: a remove-the-guard run whose neutralised guard parks a Condvar on a fixed test clock hangs without a
  kill line — run guard controls under the nextest `mutants` profile; a killed run skips the restore.
- A test that creates a home's subdirectory with `create_dir_all` creates the home outside viola; on Windows that home
  inherits the drive's DACL and is refused by strict-modes.
- `rm -r` of repo paths was refused by this session's permission layer; the operator moved the files instead.

## Outcome
- Acceptance criteria (against the diff):
  - Red A: met (forced-window case red→green, `evidence/red-a.md`; `test (ubuntu-latest)` green in ci#37218087331).
  - Red B: met (`--build-timeout=400`, basis in `evidence/red-b.md`; counting rule untouched — gate read 0; `msrv`
    green).
  - Exactly one event per raise, ids rising, no `hook.event` for a dialog, a matched continuation logging none: met
    (`run::dialog::tests`, `cli_answer`).
  - Path 4 on three OSes (question+annotations, plan approve, plan revise via the PermissionRequest repeat, the
    question's repeat silent, second concurrent dialog empty, unknown-dialog exit 13): met (ci#37218087331 green).
  - `permission` kind unit/insta only, e2e owed `:82`: met as worded (`dialog::tests` bodies).
  - Continuation matrix: met (`run::dialog::tests`).
  - v1-30 condition for question and plan; `permission` wake unit-only, e2e owed `:82`: met as worded.
  - Unstamped branch + controls row exit 12: met.
  - Strict-modes on the stamps read, each OS leg: met (Linux local + CI; Windows CI after the creation half).
  - control-character first, both sides: met.
  - Dialog-tier fail-open: met (`hook_fail_open` dialog cases, `cmd::hook::tests`).
  - `LedgerRow::ALL` stays 6: met (gate entry read 0).
  - Obs lines + canary absent from home diagnostics, G4 on CI: met (cli_answer `assert_logs_clean`; ci#37218087331).
  - Fifth perf row below `SPINE_DEADLINE`: met (`gate --require perf` green, local).
  - `answered` line / refusal lines / `--json`: met (cli_answer, cli_controls).
  - Recording `stamped 2.1.287  6 pass  0 fail`; relayed fixtures hygiene-walked; review line before commit;
    `DIALOG_DEADLINE` pinned under the `hooks.json` timeout: met (run 2 of the recording; contract_fixture_hygiene;
    review gate read 1).
  - Matrix: claims none: met.
- Gates (implement re-run 2, `.andromeda/runs/2026-10-04T14-20-29-implement/`, 25 green / 0 red / 6 not run):
  `cargo fmt --all --check` green · `cargo clippy …` green · `command -v claude` green · the spine recording leg —
  operator, fired by hand: run 2 exit 0, `contains stamped 2.1.287  6 pass  0 fail` held (`evidence/spine-recording.md`)
  · the relay leg — operator, by hand: exit 0, both atoms held (`evidence/relayed-fixtures.md`) · the four unit filters
  green · `run --unit` green · the integration filter green · `run` green · `run --perf` green · `gate --require perf`
  green · the four smoke entries green · `cargo deny check` green · the sync-crate bans green · `deny-probes.sh` green ·
  the deny-diff count green (exit 1, last line 0) · the seam guard green · the counting-rule count green · the
  ignore/env probe green · the ledger-row count green · `pre-push` green · the review grep — operator: exit 0, last line
  1 · `gate.py hygiene` — operator: `hygiene: clean` · the push — operator: four pushes, exit 0 each · `ci.py
  conclusion` — operator: final HEAD `ca69e84` `verdict: green` ci#37218087331.
- Watches: none folded.
- Outcome basis: the operator pass ran (pre-CI commit `7f1364f`, parent `c540254`); verdicts rest on its final state —
  ci#37218087331 on `ca69e84` (`evidence/operator-pass.md`), and on implement's gate re-run 2 for the local entries.
  Guard controls: `evidence/guard-controls.md`.
- Process hygiene: every process this chunk's runs started terminated (implement P4 census: none left; `ps` re-read at
  this wrap: no process from `target/` of this repo); the viola-lab prototype sessions are the operator's and stay
  running.
