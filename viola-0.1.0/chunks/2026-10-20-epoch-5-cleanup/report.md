# Report — 2026-10-20-epoch-5-cleanup

**Chunk:** the Epoch 5 boundary's ten debts closed so the next audit reads them closed. This chunk builds items (1),
(2), (3), (6), (8), (9) and (10), the founder's partition (inputs#I2).
**Date:** 2026-10-20
**Commits:** `5a3fb42a chore(2026-10-20-epoch-5-cleanup): operator pre-CI commit, for the run this chunk's verdict reads`
(the one commit since `last_wrap` 2026-10-20T07:53:24Z; `git log --format='%h %s' d35644a5..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (base `d35644a5`, the parent of the oldest pre-CI commit `5a3fb42a`; `gate.py scope`: `clean — changed 13 ·
  listed 13 · recorded 0 · excluded 67`)
  - Source and tests:
    - `.config/nextest.toml`, `tests/contract_lints.rs`, `tests/contract_fake_agent_drift.rs`,
      `tests/contract_ledger_probes.rs`
    - `crates/viola-e2e/src/harness/run/mutants/scratch.rs`, `crates/viola-pty/src/lib.rs`
    - `tests/support/home.rs`, `tests/cli_instance_state.rs`, `crates/viola-e2e/src/harness/mod.rs`,
      `crates/viola-e2e/Cargo.toml`, `Cargo.lock`
    - `src/bin/viola-fake-agent.rs`, `scripts/profraw-census.sh`
  - Rule and doc files (outside `scope`'s set, counted under excluded): `.claude/rules/frontend.md`,
    `.claude/rules/a11y.md`, `.claude/rules/api.md`, `.claude/rules/testing.md`;
    `.claude/docs/services/viola-ui.md` removed (`git rm`).
  - Chunk folder: `evidence/{baseline,kill-lines,viola-pty-leak,windows-baselines,profraw-cause,headroom,operator-pass}.md`.
  - Run dirs: `.andromeda/runs/2026-10-20T07-58-07-phase/`, `.andromeda/runs/2026-10-20T08-57-19-implement/`.
- **Symbols / APIs:** (every coordinate copied from `## New text, by line`). All of them are test, harness or test-only
  fake-agent code; no product symbol changed.
  - `tests/support/home.rs`:
    - `pub const HEADROOM_FLOOR_MIB: u64 = 9 * 1024` and `pub fn headroom_refusal(free: u64, floor_mib: u64) ->
      Option<String>` (154-159).
    - The `#[cfg(unix)]` `fn free_bytes(path: &Path) -> u64` (161-179), `libc::statvfs`, `f_bavail × f_frsize`.
    - `prepare_home_base`'s link branch panics with `headroom_refusal`'s message after its three existing checks
      (138-140).
    - `const DIALOG_ROWS: [LedgerRow; 4]` (439-445). `dialogless_home`'s expected last line is now `stamped {ver}  {p}
      pass  {f} fail`, `p`/`f` the `LedgerRow::relied()` rows outside/inside `DIALOG_ROWS` (460-462). Its one caller,
      `path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human` (`tests/cli_answer.rs`), is unchanged.
  - `crates/viola-e2e/src/harness/mod.rs`:
    - `pub const HEADROOM_FLOOR_MIB` and `pub fn headroom_refusal` (148-153); `#[cfg(unix)] fn free_bytes(path) ->
      io::Result<u64>` (155-172).
    - `Workspace::ensure_e2e_home`'s link branch returns `io::Error::new(InvalidInput, <message>)` below the floor
      (120-122). `boot` reads it as its closed `build-failed`, and `run --local-live` as `verify-exit-none`; no reason
      was added.
    - The message for both keepers: `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (two counts, no
      path).
  - `tests/contract_fake_agent_drift.rs`: `fn each_set_at_once(versions: &[String], each: impl Fn(&str) -> bool +
    Sync) -> usize` (41-73). It runs each recorded set on a `std::thread::scope` thread and joins all of them; every
    failing set is named by its version, with its panic text, in one panic. The seven walks use it: print turn,
    dialog, framing, resume, compact, agents and statusline. `tests/contract_ledger_probes.rs` has a unit twin,
    `fn each_set_at_once(versions: &[&str], each: impl Fn(&str) + Sync)` (139-168), over `STAMPED`. Assertions, the
    `compared >= N` floors and the empty-walk failure are unchanged.
  - `tests/contract_lints.rs`: `const PASTE_HINT_FILTER` (131) and the case
    `mutants_paste_hint_override_holds_the_class_above_the_default_kill` (304-316).
  - `crates/viola-pty/src/lib.rs` (test module):
    - `impl Drop for Child` calls `self.pty.kill()` unconditionally (832-833, 836). Before, it killed only when
      `try_wait()` read `Ok(None)`.
    - `fn copied_test_binary(dir) -> PathBuf` (869-877). The two PTY children spawned from `current_exe()`
      (`spawn_child_entry`, `close_hangs_up_a_live_child`) now run a per-test copy in the case's `tempfile` dir (910,
      1304).
  - `crates/viola-e2e/src/harness/run/mutants/scratch.rs`: the `#[cfg(windows)]` case's plain sibling is granted
    `BUILTIN\Users` write by a direct spawn of `%SystemRoot%\System32\icacls.exe <plain> /grant
    *S-1-5-32-545:(OI)(CI)(W)`, its exit asserted 0 (271-279; doc 253-256).
  - `src/bin/viola-fake-agent.rs` `run_receipted` (631-632, 635-641): on Unix every command it spawns, each hook and
    the status line command that shares the function, takes
    `std::os::unix::process::CommandExt::process_group(&mut cmd, 0)`, as `hold_inherited_stdout` already did.
  - `scripts/profraw-census.sh`: the second form `--case <binary stem> <test name> <runs> <workers> [<name>]`
    (30-37, 106-135, 150-166).
    - Profiles are named `r<i>-%p-%<nproc>m.profraw`, and each is offered alone to the toolchain's `llvm-profdata merge
      --sparse -o /dev/null`.
    - Refused files are kept under `refused/` and printed `profraw-census: refused <file> · signature <sig> · <size> B
      of <max> B`.
    - The last line is `profraw-census: runs N · passed N · profiles P · refused R`: exit 0 only when every run passed
      and R is 0, else 1; 2 on usage or with no instrumented binary or no `llvm-profdata`.
    - The first form is unchanged. It is still a dev-host script with no CI, `pre-push` or harness hook.
- **Crates / modules:** none added or removed. `viola-e2e` changed (its keeper and its manifest).
- **Dependencies:** `viola-e2e` gains `libc.workspace = true` under `[target.'cfg(unix)'.dependencies]` (the workspace pin
  `=0.2.189`, already in the graph through the root, `viola-pty`, `viola-channel` and `viola-state`). `Cargo.lock` gains
  the one `viola-e2e` → `libc` edge. `cargo deny check` is green. No new crate and no version change.
- **Schema / config:**
  - `.config/nextest.toml` `[profile.mutants]`:
    - The `verify_window_` override moves from `slow-timeout = { period = "15s", terminate-after = 3 }` (45 s) to
      `{ period = "20s", terminate-after = 3 }` (60 s, the class's `ci` kill). It still stands first.
    - A new fourth override, `filter = 'test(/send_under_the_paste_hint_/)'` at `{ period = "15s", terminate-after = 2
      }` (30 s), stands last.
    - The profile now holds four overrides: `verify_window_` · `package(viola-e2e)` · `send_window_` ·
      `send_under_the_paste_hint_`.
    - `[profile.ci]` is unchanged.
  - Four rule files' `paths:`:
    - `frontend.md` drops `crates/viola-ui/assets/**`, and its `## Web-spa (deferred to 0.2.0)` section is removed.
    - `a11y.md` now names `src/human.rs`, `src/human/**`, `src/run/wheel.rs`, `tests/cli_output_plain.rs`,
      `tests/cross_list_rows.rs` and `tests/tui_*.rs` in place of five web paths. Its frontend section keeps one
      sentence: «The frontend is deferred to 0.2.0 and its technology is not chosen.»
    - `api.md` drops `crates/viola-ui/src/**`, and `testing.md` drops `e2e-web/tests/**`.
    - No `## Session Additions` text moved.
- **Spec-master edits:** none in this chunk's tree (all seven masters, the registries and `CLAUDE.md` are unchanged
  since `d35644a5`).
- **Retired:**
  - `.claude/docs/services/viola-ui.md` is removed. `CLAUDE.md:100` (inside `GENERATED:setup:deeper-topics`) still
    names `viola-ui` in its services list, a leaf. `grep -rn 'services/' .andromeda/architecture.md
    .andromeda/registries/contracts/architecture/` reads 0 lines: no master lists the services docs.
  - obs-plan §4's scenario heading `#### Scenario: GUI view: page load, sessions and links, live SSE tape with
    \`Last-Event-ID\` resume` (`.andromeda/obs-plan.md:647`, F15) stands over a one-line deferral stub, so it is the
    standing web leftover.
  - design-system §Brand Identity cites cut page components and the web motion row: F12's rows at design-system.md
    `:16` (`<viola-atis>`), `:21` (`<viola-event-feed>`), `:41` (`web-spa (\`viola ui\`)` row) plus `:136`, `:208`,
    `:247`, `:335`, `:370` (`## Surface: web-spa`), `:584`. Line `:45` («viola's web page is 0.3…») states the same in
    other words and was read by hand: F12 does not reach it.
  - No census row (no `census.md` in `viola-0.1.0/`).
- **Counts / qualifiers moved:**
  - The `mutants` kill of the `verify_window_` class: 45 s → 60 s (F1-verify-kill, F3-mutants-prof). The no-screen
    case's measured floor is 42.874 / 43.078 / 43.714 s (CI legs, ci#39383206773) and 42.952 s alone on the dev host.
    - **Stale:** `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md`:
      - `:11` — `[profile.mutants]` «plus three `[[profile.mutants.overrides]]`» … «(a 45 s kill)», and «34.3 s …
        below the 45 s kill».
      - `:12` — «the `verify_window_` class 45 s».
      - `:20` — «takes that class's 45 s kill».
      - `:10 @c3888` — «against the class's 60 s kill and the `mutants` profile's 45 s (about 2 s of margin locally)».
    - True claims sharing the token: test-plan.md `:591`; the same key file's `:10` at `@c226`, `@c547`, `@c774`,
      `@c2715`.
  - The `mutants` profile's override count: 3 → 4, the paste-hint class at 30 s (F2-paste-hint, F3-mutants-prof). The
    paste-hint pair's floor is 8.7–9.9 s on three OSes under `ci`, against the 10 s default kill.
    - Stale: the same key file's `:11` (the override list and its order) and `:12` (the kill list «10 s; viola-e2e
      30 s; the `verify_window_` class 45 s»).
    - True claims sharing the token: test-plan.md `:671 @c1825,2004` (names the pair, no kill) and
      `registries/contracts/a11y-plan/keyboard-test-harness.md:3` (names the case by name). Also true:
      test-plan.md `:652` (10 s kill), `5-command-implementation.md:27`, `:35`, `:39`, `test-data-bootstrap.md:16`.
  - The replay walks' wall (F4-replay-walk). Sets now run at once: the dialog replay went from 10.003 s TIMEOUT to
    4.877 s under `mutants` on the dev host, and from 15.0 / 14.1 / 17.0 s to 5.7 / 8.4 / 6.2 s under `ci` (CI legs,
    ci#39400402749). The kill-line note in `bootstrap-phases…md:10 @c1457` («the longest verify-driven test under the
    20 s kill read 9.98 s … (`fake_agent_dialog_replay_matches_every_recorded_dialog_set`, one verify per recorded
    set)») is history and holds. The statements of what the walks assert hold unchanged (architecture.md `:374`;
    test-plan.md `:379` ×3, `:603`, `:914`; the key file's `:10 @c1483`, the binary list).
  - `dialogless_home`'s summary is now derived, with the values unchanged (`20 pass  4 fail` on Unix, `17 pass  4 fail`
    on Windows) (F5-dialogless). Its one site, `bootstrap-phases…md:10 @c2923` («through it `stamped_home` /
    `dialogless_home`»), holds.
  - The truncated `.profraw`'s killer is named (F6-profraw, F7-hook-group, F13-census): the terminal hangup after a test
    kills the wrapper, which reached hooks the fake agent ran in its own process group (Spec claims disproved, and
    `evidence/profraw-cause.md`). Stale:
    - test-plan.md `:963` — «a hook is in its PTY's foreground process group»: now it is in its own group on Unix. The
      wait for a running hook still stands.
    - test-plan.md `:1092` — Stack adjustments: it names one mechanism (a self-exiting child) and the census's first
      form only. This chunk's finding is a second mechanism, plus the census's `--case` form.
    - architecture.md `:395` — the census's usage, its last line and its exits are the first form only.
    - `registries/contracts/architecture/project-directory-structure.md:98`, `:100` — «the start test N times»: the
      first form only.
    - Hold: test-plan.md `:624` (rmcp's child kill, another mechanism), `:962` (the `--exit-no-eof` holder's own group),
      `:893` (holder facts).
  - The keepers' checks: three → four, the headroom floor (F8-headroom). Stale-incomplete, as each lists the keepers'
    checks without the floor:
    - architecture.md `:371` («refuse a target that is not a real, owner-only directory») and `:388 @c1300,1390` («is
      accepted only when one `lstat` reads a real directory with no group or other bit. Anything else refuses…»).
    - security-plan.md `:536`.
    - `registries/contracts/test-plan/5-command-implementation.md:6`.
    - `registries/contracts/test-plan/test-data-bootstrap.md:13`.
    - Hold: test-plan.md `:562` (it points at §3), `:1095` (the mutation `TMPDIR`), architecture.md `:406` (the same),
      `5-command-implementation.md:43` (`verify-exit-none` already covers «a base the keeper refused»).
  - The `viola-e2e` → `libc` edge (F9-libc): `registries/contracts/architecture/crate-dependency-direction.md:9` reads
    «`viola-e2e` (test-only…); its dependencies are not listed here» (read by hand, not an F9 row), so no master lists
    `viola-e2e`'s dependencies. F9's eleven master rows name other crates' `libc` and hold.
  - The `viola-pty` test guard and copy (F10-pty-guard): neither row is this fact. architecture.md `:110` is
    `plugin install`'s `current_exe`, and test-plan.md `:1092 @c1310` is the root start test's earlier child. Both
    hold.
  - The scratch case's control is now made (F11-scratch-win). Stale: `5-command-implementation.md:35 @c3573` — «the
    same tree in a plain-created sibling reads `Writable`»: now a sibling granted `BUILTIN\Users` write explicitly.
    Hold: architecture.md `:406 @c1109` (names the witness; the reason a real plain-created scratch is refused holds)
    and security-plan.md `:185` (`icacls` of another fact).
  - `path6_the_boundaries_and_the_window_word` (F16-path6): recorded, not changed; test-plan.md `:772` holds.
  - **Sweep reach:** 16 patterns · 16 moved facts, 16 with a pattern (the report's own count) · owner pattern: control
    unfired (`control: own-title, own-generic never fired over the seven masters' pre-pass text (5a3fb42a)`) · by line
    6 lines · 14 offsets (`cascade.py sweep --pre-pass`, trail `cascade-2026-10-20-epoch-5-cleanup-pre-pass.json`:
    `total (16 patterns) · 124 rows over 32 files`; `standing 74/14 · leaf 44/17 · curation 6/2`).
  - Per-pattern standing counts: F1 5 · F2 2 · F3 8 · F4 5 · F5 1 · F6 6 · F7 3 · F8 9 · F9 11 · F10 2 · F11 3 · F12 11
    · F13 4 · F14 2 · F15 1 · F16 1. Each master row is dispositioned above per class; F12's and F14's under Retired and
    Expected amendments. A listing reaches its patterns' wording only; the detectors' own sweeps stay the control for
    reach.
- **Dev-tool versions:** none — cargo-llvm-cov 0.9.1 and the toolchain's `llvm-profdata` re-read, unchanged.
- **Harness / gate surface:**
  - The two keepers' headroom refusal. The harness boot's refusal reads the existing `build-failed`, and no reason was
    added.
  - The census script's `--case` form.
  - The nextest `mutants` profile (above).
  - No CI step, harness verb, status shape or log format changed.
- **Cross-project / external claims:**
  - CI ci#39400402749 on `5a3fb42ab8fb0aac4c43912407755dd7573cee59` (the pre-CI commit, the chunk's whole tree): `green ·
    checks 15/15 · wall 814 s`, attempt 1 (`evidence/operator-pass.md`). This wrap's commit adds to that tree.
  - CI ci#39383206773 (`0807ee3c`) JUnit and the Windows dispatch 39326811932 were read as recorded runs (research;
    `evidence/windows-baselines.md`).
  - `inputs.py verify`: `inputs: 4 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0
    · n/a 2 · uncited 2 · unparsed 0`.
    - I1 · `../additional/viola-overseer/phase-cleanup-directive.md` · copy no-repo · unchanged (cited).
    - I2 · message (the founder, live, 2026-10-20T08:32Z) · copy message · n/a (cited).
    - inputs#I3 · message (the Viola overseer, the founder's delegate in the operator seat, 2026-10-20T08:54Z) · copy
      message · n/a. It is the P5 word on `.andromeda/runs/2026-10-20T07-58-07-phase/p5-card-1.md`: «yes — the
      overseer's word on p5-card-1.md … Its three narrowings are accepted.» Delegate authority: PROVISIONAL on a fork
      the founder owns. The card's `## Beyond the plan` rows, verbatim:

      | # | proposal | owner / place |
      |---|---|---|
      | 1 | Route insertion «Epoch 5 cleanup II» directly after this entry, carrying items (4), (5), (7) as scope.md folds them | the wrap writes it on the founder's word (inputs#I2) |
      | 2 | Deferral: the Windows mutation-copy readings of (3a), (3b) and (3c) (the scratch case inside a cargo-mutants copy, `path6_…` inside its kill, `viola-pty` unviable ≤ caught) | owed to the next epoch boundary's code audit (its Windows leg); no route entry |
      | 3 | The `.profraw` WATCH retired by its recurrence (P5, 2026-10-20T08:48Z) into item (8). If step 10 ends with no reproduction or an unfixed killer, its open red gets an owner | the wrap proposes the owner then |

      Row 1 rests on the founder's own word (inputs#I2). Row 3's condition did not occur: step 10 named and fixed the
      killer.
    - inputs#I4 · `../additional/viola-overseer/wrap-cleanup-directive.md` · copy no-repo · unchanged — the overseer's
      wrap directive, snapped at this wrap's P1. It carries five measured facts: the outcome basis; the «Два чанка»
      insertion; step 7's criterion routed to the next boundary's Windows `viola-pty` leg «as the item (3) reading owed
      to that audit, which the P5 card's own row already carries», not as a mutation run inside a chunk; item (8)'s
      WATCH to be retired or re-worded by the letter's rule; the macOS WATCH counted by its own form.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):**
  - Step 7's unconditional guard kill and per-test copy are correct and shipped (every `viola-pty` case green on three CI
    legs), but this host cannot show that they close the leak they were written for.
  - The remainder is the next epoch boundary's Windows `mutants (viola-pty)` reading, `unviable` ≤ `caught` (the P5
    card's row 2; inputs#I4 §3).
- **Spec claims disproved by measurement:**
  - **The plan's step 7 premise** — `plan.md` step 7 and its acceptance criterion «Step 7's remove-the-guard reading in
    `evidence/viola-pty-leak.md` shows leaked children under the hand-applied `try_wait -> Ok(Some(1))` on the old guard
    and none on the new one», and research §Mechanisms (3c) «a `try_wait` mutant that reports the child gone makes the
    guard skip its kill … the child … then stays running».
    - Measured on the Linux dev host: the census reads `none running` after the old-guard run and after the new-guard
      run (`evidence/viola-pty-leak.md`; trail `.andromeda/runs/2026-10-20T08-57-19-implement/gate-2026-10-20-epoch-5-cleanup.json`).
    - Why: dropping `PortablePty` closes the master and the kernel hangs up the child, which
      `tests::close_hangs_up_a_live_child` pins.
    - No master states it (a plan/research claim: `no master site — the claim lives in the chunk's plan and research
      alone`). Owner: the next boundary's Windows leg (inputs#I4 §3; the P5 card's row 2).
  - **test-plan.md `:963`** «a hook is in its PTY's foreground process group» was true at HEAD. This chunk changed it,
    so it is not disproved but moved (Counts / qualifiers moved, F7). Listed here only so Validate sees both bullets.
- **Expected amendments (from plan):**
  - test-plan §3 Bootstrap phases (`test-runner-install`, `[profile.mutants]`): the paste-hint override at 30 s and
    `verify_window_`'s `mutants` kill at 60 s, with the measured floor beside each — **carried**: Schema / config and
    Counts / qualifiers moved.
    - F1-verify-kill (test-plan 5 standing, all in the key file `bootstrap-phases-derive-for-route-setup-project.md`,
      which test-plan owns: `:10` ×5, `:11` ×4, `:12`, `:20` ×2).
    - F2-paste-hint (test-plan.md `:671` ×2; a11y-plan's key `keyboard-test-harness.md:3`, names only).
    - F3-mutants-prof (8 standing).
  - test-plan §10 Stack adjustments — item (8)'s finding (the census's second form, the named killer) — **carried**:
    Symbols / APIs (the census, `run_receipted`), Counts / qualifiers moved (F6-profraw 6 standing, F7-hook-group 3,
    F13-census 4), Spec claims disproved.
  - test-plan §3 Test data bootstrap (Mechanism) — the keepers' headroom floor with its basis — **carried**: Symbols /
    APIs and Counts / qualifiers moved (F8-headroom 9 standing, among them
    `registries/contracts/test-plan/test-data-bootstrap.md:13`). The basis is in `evidence/headroom.md`: the EDQUOT
    point at about 8 GiB free plus 1 GiB.
  - design-system §Brand Identity — the web-surface references at its lines 16, 21, 41, 45 cut, the cli anchors and
    the cli expression rows kept, the superseded text into `.andromeda/design-system-amendments.md` — **carried**:
    Retired (F12-web-left, design-system.md 9 standing rows plus `:45` read by hand). The plan names the
    supporting-attention paragraph, the design-direction paragraph, §Motion's `0.3 on web-spa` (`:335`) and
    §Self-Validation's Signature Test as the sweep's to read: they are among F12's rows (`:335`, `:584`) or worded
    otherwise.
  - a11y-plan §1 A11y Scope Summary — the `Standard (1)` tier's basis stated on the terminal surfaces alone, no WCAG
    conformance claimed — **carried** for the detector to confirm.
    - F14-a11y-tier: a11y-plan.md `:18` (the tier line) plus obs-plan.md `:25` (obs's own tier, not this fact).
    - At HEAD the justification line a11y-plan.md `:19` already reads «cli and tui get output-discipline and boundary
      assertions plus manual screen-reader passes, with no WCAG conformance claim», and `:24` «The frontend is deferred
      to 0.2.0».
    - Whether any §1 text still rests on the web surface is the a11y detector's read.
  - obs-plan §4 — the `GUI view: page load, sessions and links, live SSE tape …` scenario heading over its deferral
    stub — **carried**: Retired (F15-gui-view, obs-plan.md `:647`, 1 standing). obs-plan §3 needs no edit (scope's
    premise correction).
  - architecture §Infrastructure Patterns → Project directory structure — `CLAUDE.md`'s `GENERATED:setup:deeper-topics`
    services pointer without `viola-ui`, and the `.claude/docs/services/` list, by the cascade — **carried**: Retired.
    - The pointer is the leaf `CLAUDE.md:100` (F12 leaf row).
    - No master site: `grep -rn 'services/' .andromeda/architecture.md .andromeda/registries/contracts/architecture/`
      reads 0. `registries/contracts/architecture/build-system.md:15` (`e2e-web/tsconfig.json`, history) holds.
- **Coverage of new surfaces:**
  - `headroom_refusal` + `free_bytes` (both keepers, test-side) → validation n/a (two numbers; `CString::new` refuses
    a NUL) · instrumentation n/a (test code) · PII n/a (counts only, no path) · tests unit
    (`home_base_headroom_below_the_floor_is_refused_with_both_counts`, `home_base_headroom_at_or_above_the_floor_passes`,
    `home_base_headroom_floor_is_nine_gib`, `harness::tests::e2e_home_headroom_refuses_below_the_floor_and_passes_at_or_above_it`,
    `harness::tests::e2e_home_headroom_reads_a_real_filesystem`) · a11y n/a · tokens n/a.
  - The `--case` census form (dev-host script) → validation ✓ (stem and test name by regex, runs and workers by regex)
    · instrumentation n/a · PII n/a · tests ✗ (its one-shot readings in `evidence/profraw-cause.md`, as the first form's
    controls; test-plan §10 says the script has no unit test) · a11y n/a · tokens n/a.
  - The fake agent's process-group spawn (test-only bin) → tests: every hook-driven case on ubuntu and macOS
    (ci#39400402749), and the census 0 refused of 800 · others n/a.

## Deviations from intent
1. **Step 7's acceptance is unmet as written.** It is a measured premise disproof (Spec claims disproved), routed to the
   next boundary's Windows leg (inputs#I4 §3; the P5 card's row 2). The change stands.
2. **The leak census keyed on the test binary's name** (`viola_pty-77dd4bdbf6cc1891`), not «the clone's target dir».
   The new side's children run from a tempdir copy, which a target-dir key never reads, so «none on the new one» would
   have been vacuous.
3. **The scratch clones' target dirs sit under the repo's `target/`** (`target/e5c-clone-target`, `target/e5c-h`,
   `target/e5c-pty-{old,new}`), not in the scratchpad. The scratchpad is on the quota'd `/tmp` tmpfs, 14 G free, and
   each clone build is several GB. The clones' sources are in the scratchpad as planned.
4. **Step 3's remove-the-guard was read by `cargo nextest` inside the scratch clone,** not `gate.py run
   --project-root`: the e2e entry would build the harness target inside the clone, on `/tmp`.
5. **The floor is 9 GiB** (the EDQUOT reading of about 8 GiB plus 1 GiB), not that reading plus the whole run's
   3 087 MiB footprint. Each check reads the backing with the run's own homes already in it, so the footprint on top
   (about 11.1 GiB) would refuse the run's own later homes at today's headroom (`evidence/headroom.md`).
6. **Step order:** steps 11 and 12 were written before step 10(b)'s census, which needs only HEAD's instrumented build.
   The plan's order rules (10(a) → (b) → (c)) were kept.
7. Scope record: none — `gate.py scope` clean, 0 recorded (`clean — changed 13 · listed 13 · recorded 0 (companion 0 ·
   mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 67`).

## Decisions & corrections
- **The operator's direction for implement** (2026-10-20, this session): «No mutation run, witness run or dispatch;
  read df -h /tmp before the stamped-home gates. Then the operator pass as usual, the ci.py read included; stop before
  the wrap.» Kept: no mutation run, witness run or dispatch; `df -h /tmp` read before every stamped-home firing (the
  censuses, the coverage run, the block's entry 1).
- **The wrap directive** (inputs#I4, the overseer, 2026-10-20): its five facts, each acted on at the phase it names.
- **A Linux PTY child dies with its master** (measured): dropping `PortablePty` hangs the child up, so a test guard that
  skips its kill leaks nothing on Linux. A guard-leak reading taken on the Linux host cannot separate a skipping guard
  from a killing one; that leak's witness is Windows (`evidence/viola-pty-leak.md`).
- **A cut-short profile ends on a 4 KiB page boundary** (measured): P5's 90 112 B, the switch chunk's 118 784 B, and 11
  of the 17 census files (90 112 ×6, 94 208 ×2, 118 784, 4 096 ×2). A cut inside the data records leaves no counter,
  so the counter method cannot name that writer; only files cut in the counters or names section can be attributed.
- **A census's profile COUNT is a reading too** (measured): 726 profiles in 200 runs before the fix, 800 (exactly 4 per
  run) after. 74 processes had died before their exit-time write began, which the refused count alone does not show.
- **The fake agent's hooks leave the foreground process group** (measured fix): 17 refused / 726 → 0 / 800 over the
  same 200 runs of `chaos_wait_parked_wrapper_killed_exits_21`. The class is the session-leader hangup that already
  moved `hold_inherited_stdout` out of the group.
- **Sweep hazard — a census keyed on a target dir misses copied binaries:** a child run from a per-test copy carries the
  copy's path, so `gate.py census --args <target dir>` reads none whatever leaked. Key on the binary's file name.
- **Sweep hazard — the run-dir spelling of a scratchpad path:** evidence first written with the scratchpad's full path
  (`$TMPDIR/claude-1000/-home-<user>-…/scratchpad/…`) carried the user name dash-spelled. It was respelled
  `$TMPDIR/{scratchpad}/…` before the hygiene read, which then read clean. A `/tmp/viola-e2e-home-1000` mention in
  `evidence/headroom.md` was reworded too.
- **Guard refusal (recurrence, host-linux.md §Paths & the working directory):** the Bash guard refused a `cd` into a
  scratch clone; it was re-issued once as a subshell `( cd … && … )`.
- **Clone builds off the quota'd tmpfs** (decision): a scratch clone in the scratchpad builds into a `CARGO_TARGET_DIR`
  under the repo's `target/`, since the scratchpad is on `/tmp` (14 G free; each build several GB).
- **The `mutants` profile's walls after the fix** (measured): every replay walk about 4.8 s under `mutants` on the dev
  host, 5.4–8.4 s under `ci` on three OSes, down from 9.4–17.0 s.

## Outcome
- **Acceptance criteria**, re-asserted against the diff:
  - (tests) Every entry of the block reads green, the operator trio in the operator pass — **met**: implement's run is
    `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3`; operator entries 24, 25, 26 are in
    `evidence/operator-pass.md`.
  - (tests) The `NEXTEST_PROFILE=mutants` entry reads `exit 0` with no TIMEOUT after steps 2–5, its red-before in
    `evidence/baseline.md` — **met**: `1605 tests run: 1605 passed (112 slow), 0 skipped`; before, `1 timed out`.
  - (tests) Each step-2 case asserts what it asserted before, its walls before and after in `evidence/baseline.md` —
    **met**.
  - (tests) `[profile.mutants]` holds the paste-hint override at 30 s and `verify_window_` at 60 s, still first; the
    three pins are green; each kill has its planted-hang pair — **met** (`evidence/kill-lines.md`): TIMEOUT 30.003 s /
    PASS 48.413 s, and TIMEOUT 60.003 s / PASS 112.961 s.
  - (tests) The paste-hint case keeps its name and assertions — **met**: the test file is unchanged.
  - (tests) Step 6's Windows case keeps both assertions with an explicit grant, CI windows-2025 green, the copy reading
    owed — **met** (0.061 s on windows-2025, ci#39400402749; `evidence/windows-baselines.md`).
  - (tests) Step 7's remove-the-guard reading shows leaked children on the old guard and none on the new — **UNMET as
    written** by a measured host fact. It is not matrix-linked, so it goes to a P2 escalation, its owner being the next
    boundary's Windows leg (inputs#I4 §3). Every `viola-pty` case is green on three CI legs.
  - (tests) `evidence/windows-baselines.md` holds step 8's three readings and the owed one; no kill moved for `path6` —
    **met**.
  - (tests) `dialogless_home` reads from `LedgerRow::relied()`; its caller is green; entry 12 reads `last line 0`;
    `tests/cli_verify.rs:1315`/`:1317` are unchanged — **met**.
  - (tests) `evidence/profraw-cause.md` holds the two-sided reading, with the hangup named and 0 refused after — **met**
    (17/726 → 0/800).
  - (tests) Both keepers refuse below the floor with the two-count message, their decision cases green on every OS, the
    basis in `evidence/headroom.md`, no env/flag/config — **met** (the decision cases on three legs in
    ci#39400402749; remove-the-guard red/green in `evidence/headroom.md`).
  - (a11y) `a11y.md`'s `paths:` names no web path and names the terminal files; its body states no page rule for 0.1.0
    — **met**.
  - (arch) Entry 13 reads no output; `viola-ui.md` is gone — **met** (`git rm`, not refused).
  - (security) Guard `clean`, `libc` passes `cargo deny check`, the `pre-push` launcher unchanged — **met**.
  - (obs) G1, `lint-probes.sh`, G2 and G4 green — **met**.
- **Gates** (implement's run `.andromeda/runs/2026-10-20T08-57-19-implement/`, the tree the pre-CI commit carries;
  outcome words as printed):
  - `df -h /tmp` — recorded, exit 0: `tmpfs 32G 19G 14G 59% /tmp`.
  - Lints:
    - `cargo fmt --all --check` — green.
    - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green.
    - `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` — green.
    - G1 `rg … instrument …` — green (exit 1, no output).
    - `bash scripts/lint-probes.sh` — green.
  - Tests:
    - `bash scripts/agent-run.sh run --unit` — green.
    - `run --integration --filter 'binary(cli_instance_state) | binary(cli_answer) | binary(cli_send) |
      binary(cli_verify) | binary(cli_budget)'` — green.
    - `run --integration --filter 'test(/headroom/)'` — green.
    - `run --e2e --filter 'binary(contract_lints) | binary(contract_fake_agent_drift) | binary(contract_ledger_probes) |
      binary(contract_fixture_hygiene)'` — green.
    - `run --e2e --filter 'test(/mutants_paste_hint_override_holds_the_class_above_the_default_kill/)'` — green.
  - Probes:
    - `grep -c -F 'let passed = if cfg!(unix) { 20 } else { 17 };' tests/support/home.rs` — green (exit 1, last line
      0).
    - `grep -rn -E '^  - "(e2e-web/|crates/viola-ui/)' .claude/rules/` — green (exit 1, no output).
    - `NEXTEST_PROFILE=mutants AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 CARGO_TARGET_DIR=target/mutants cargo
      nextest run --package viola --features fake-agent` — green (exit 0, lacks TIMEOUT).
    - The `gate.py guard` — green, `guard: clean` (`guard: allowed 13 — …`, companions excluded 0).
    - `cargo deny check` — green.
  - Smoke and obs gates:
    - `agent-run.sh cleanup` / `boot` / `status` (the smoke) — green.
    - `bash scripts/g2-zero-panics.sh` — green; `agent-run.sh schema-check` — green.
    - The final `cleanup` — green (`processes_gone`, `endpoint_gone` true).
  - `bash scripts/agent-run.sh pre-push` — green (exit 0, `"ok":true`, stage `linux-tests`, 0 `header is corrupt`
    lines).
  - Operator legs (`leg = 'operator'`), recorded in `evidence/operator-pass.md`:
    - `gate.py hygiene` — `hygiene: clean` (×2).
    - The push entry — green, `history moved: … d35644a5→5a3fb42a`.
    - `ci.py conclusion --sha HEAD --wait 2400` — `5a3fb42ab8fb verdict: green · checks 15/15 · wall 814 s · runs
      ci#39400402749 completed/success`.
  - Smoke where the boot path changed: the boot entry went through the changed `ensure_e2e_home` keeper, green (6.92 s).
- **Watches:**
  - The truncated `.profraw` (0/3 at take-up; its retire condition «a recurrence» met at P5) · its cause is item (8),
    named and fixed by this chunk. 4 green runs since the fix:
    - implement's `run --coverage` (2 980 passed, merge clean);
    - implement's `pre-push` entry (0 `header is corrupt`);
    - the census after the fix (0 / 800);
    - ci#39400402749's coverage-merging `test (ubuntu-latest)`, green.

    Its re-wording or retirement is P5's (inputs#I4 §4).
  - The macOS readiness stall · 1 green run [ci#39400402749 `test (macos-latest)`, attempt 1, no stall report], against
    its witness, the stall report `wait_ready` writes (inputs#I4 §5).
- **Outcome basis:** the operator pass ran.
  - The pre-CI commit `5a3fb42a`, no fix commit (`git log --format='%h %s' d35644a5..HEAD`: that one line).
  - The final HEAD's CI ci#39400402749: green, recorded in `evidence/operator-pass.md`.
  - This window holds implement's run whole, and its `outcome-2026-10-20-epoch-5-cleanup.md` agrees.
  - Post-implement artifacts: `evidence/operator-pass.md`'s CI section (uncommitted, riding this wrap's commit).
- **Process hygiene:**
  - Implement P4's census: `census: none running` (`--args projects/viola/target --args …/scratchpad/clone --args
    viola_pty-77dd4bdbf6cc1891 --args profraw-census`).
  - Re-measured at this wrap with the same arms: `census: none running` (trail
    `.andromeda/runs/2026-10-20T09-40-55-wrap/gate-2026-10-20-epoch-5-cleanup.json`).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: d35644a5 (the parent of the oldest pre-CI commit 5a3fb42a) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 11 line(s) in 3 range(s)
added: 36-38 · 43 · 55-61
### Cargo.lock — added 1 line(s) in 1 range(s)
added: 1915
### crates/viola-e2e/Cargo.toml — added 4 line(s) in 1 range(s)
added: 38-41
### crates/viola-e2e/src/harness/mod.rs — added 84 line(s) in 3 range(s)
added: 120-122 · 141-173 · 377-424
- 148-153 @149 «pub fn headroom_refusal(free: u64, floor_mib: u64) -> Option<String> {»
  - 151-152 «(free_mib < floor_mib)»
- 155-172 @159 «fn free_bytes(path: &Path) -> io::Result<u64> {»
  - 161-162 «let path = std::ffi::CString::new(path.as_os_str().as_bytes())»
  - 163-170 @164 «let stat = unsafe {»
  - 377-409 @380 «fn e2e_home_headroom_refuses_below_the_floor_and_passes_at_or_above_it() {»
  - 411-423 @414 «fn e2e_home_headroom_reads_a_real_filesystem() {»
### crates/viola-e2e/src/harness/run/mutants/scratch.rs — added 13 line(s) in 2 range(s)
added: 253-256 · 271-279
### crates/viola-pty/src/lib.rs — added 15 line(s) in 5 range(s)
added: 832-833 · 836 · 869-878 · 910 · 1304
  - 869-877 @872 «fn copied_test_binary(dir: &std::path::Path) -> std::path::PathBuf {»
### scripts/profraw-census.sh — added 90 line(s) in 10 range(s)
added: 11-17 · 24 · 27-37 · 57-62 · 66 · 70 · 78-88 · 106-138 · 143 · 150-167
- 30-37 «if [ "${1:-}" = --case ]; then»
  - 31-33 «if [ "$#" -lt 5 ] || [ "$#" -gt 6 ] || ! [[ $2 =~ ^[a-z0-9_]+$ ]] || ! [[ $3 =~ ^[A-Za-z0-9_:]+$ ]]; then»
- 59-62 «if [ -n "$case_stem" ]; then»
- 78-85 «if [ -n "$case_stem" ]; then»
  - 80-83 «if ! [ -x "$profdata" ]; then»
- 86-88 «else»
- 106-135 @108 «case_worker() {»
  - 113-133 «for ((i = 0; i < n; i++)); do»
- 150-166 «if [ -n "$case_stem" ]; then»
  - 151-153 «summary=$(find "$out" -name tally -exec cat {} + | awk '»
  - 158-160 «awk 'NR == FNR { if ($2 > max[$1]) max[$1] = $2; next }»
  - 162-164 «if [ "$n" -eq "$runs" ] && [ "$passed" -eq "$n" ] && [ "$refused" -eq 0 ]; then»
### src/bin/viola-fake-agent.rs — added 9 line(s) in 2 range(s)
added: 631-632 · 635-641
### tests/cli_instance_state.rs — added 37 line(s) in 2 range(s)
added: 18-22 · 593-624
- 18-22 «use support::home::{»
- 595-610 @597 «fn home_base_headroom_below_the_floor_is_refused_with_both_counts() {»
  - 598-601 «assert_eq!(»
  - 602-605 «assert_eq!(»
  - 606-609 «assert_eq!(»
- 612-617 @613 «fn home_base_headroom_at_or_above_the_floor_passes() {»
- 619-623 @621 «fn home_base_headroom_floor_is_nine_gib() {»
### tests/contract_fake_agent_drift.rs — added 76 line(s) in 25 range(s)
added: 41-74 · 143 · 190-191 · 255-257 · 261-262 · 270 · 274-275 · 348-349 · 352 · 359 · 371-372 · 457-459 · 463
       466-467 · 522-524 · 529 · 532-533 · 571-573 · 592 · 615 · 625-626 · 671-672 · 675 · 682 · 694-695
- 41-73 @44 «fn each_set_at_once(versions: &[String], each: impl Fn(&str) -> bool + Sync) -> usize {»
  - 46-55 «let outcomes: Vec<(&str, std::thread::Result<bool>)> = std::thread::scope(|scope| {»
  - 56-67 «let failed: Vec<String> = outcomes»
  - 69-72 «outcomes»
### tests/contract_ledger_probes.rs — added 31 line(s) in 2 range(s)
added: 107 · 139-168
  - 146-155 «let outcomes: Vec<(&str, std::thread::Result<()>)> = std::thread::scope(|scope| {»
  - 156-167 «let failed: Vec<String> = outcomes»
### tests/contract_lints.rs — added 15 line(s) in 2 range(s)
added: 131 · 304-317
- 304-316 @307 «fn mutants_paste_hint_override_holds_the_class_above_the_default_kill() {»
  - 309-310 «let default = kill_line(&table(&nextest, "profile.mutants").expect("[profile.mutants]"))»
  - 312-313 «let class = override_at(&overrides, PASTE_HINT_FILTER)»
### tests/support/home.rs — added 57 line(s) in 7 range(s)
added: 13 · 138-140 · 147-180 · 439-446 · 448-451 · 460-462 · 464-467
- 154-159 @155 «pub fn headroom_refusal(free: u64, floor_mib: u64) -> Option<String> {»
  - 157-158 «(free_mib < floor_mib)»
- 161-179 @165 «fn free_bytes(path: &Path) -> u64 {»
  - 168-177 @169 «let stat = unsafe {»
- 439-445 @440 «const DIALOG_ROWS: [LedgerRow; 4] = [»
  - 460-462 «let (failed, passed): (Vec<LedgerRow>, Vec<LedgerRow>) = LedgerRow::relied()»
