# Report — 2026-10-19-newer-claude-cli-rows

**Chunk:** the four ledger rows claude 2.1.302 fails, re-measured and brought back to a full stamp
**Date:** 2026-10-20
**Commits:** since `last_wrap` (2026-10-19T23:05:24Z), from `git log --format='%h %s' 50e2f761..HEAD`:
- `3bdcd4a7` chore(2026-10-19-newer-claude-cli-rows): operator pre-CI commit, for the run this chunk's verdict reads
- `4eaca1d3` chore(2026-10-19-newer-claude-cli-rows): operator pre-CI commit, the gated live round, for the run this
  chunk's verdict reads

## Changes (structured — detectors read this)
- **Files** (`gate.py scope`: clean — changed 37 · listed 36 · recorded 1 · absorbed 29):
  - `crates/viola-agent-claude/src/ledger.rs`
  - `crates/viola-agent-claude/src/screen.rs`
  - `src/cmd/verify/typed.rs`
  - `src/bin/viola-fake-agent.rs`
  - `tests/cli_verify.rs`
  - `tests/support/home.rs`
  - `tests/contract_ledger_probes.rs`
  - `tests/contract_fixture_hygiene.rs` (the recorded companion)
  - `fixtures/claude/2.1.302/` (29 files)
  - The chunk folder: `footer-reading.sh`, `macos-run-start.py`, `scope-record.md`, and `evidence/` (`reproduction.md`,
    `macos-run-start.md`, `operator-pass.md`, `footer-2.1.302.md`, `live-round.md`, `remove-the-guard.md`, three
    `round-*.txt`).
- **Symbols / APIs:**
  - `viola_agent_claude::screen::SIGNATURES.input_box`: `["for agents"]` → `["for agents", "(shift+tab to cycle)"]`.
    - Its doc comment is `screen.rs:36-39`; the literal is `screen.rs:41`.
    - The change reaches every reader of `SIGNATURES` unchanged in code (16 references, research.md Graph impact).
      Among them are `run`'s pre-send gate on a verified CLI (`src/run/gate.rs:508`), verify's settles
      (`src/cmd/verify/typed.rs`), `run`'s screen read (`src/cmd/run.rs:646`) and the ledger's recording scrub
      (`ledger.rs`).
    - New unit cases (`screen.rs:449-475`):
      - `holds_any_reads_the_resting_footer`, 8 rows;
      - `verdict_over_the_2_1_302_resting_footer_alone_is_ready`;
      - `verdict_over_the_resting_footer_beside_a_modal_is_input_not_ready`.
  - `viola_agent_claude::ledger::TypedRun::long_turn: Option<Vec<String>>` (`ledger.rs:603-604`, `:610`):
    - It holds the rows verify's Run B settles on after the long paste's turn. It is never written to `measured`
      (the stamp holds no row text; `run` reads no number from it).
  - `check(InputBoxSignature)` (`ledger.rs:1011-1013`):
    - It now reads `ready`, `turn` and `long_turn`, all three required to hold an input-box literal and no modal.
    - Its row words are unchanged: `a trusted start shows a compiled input-box literal and no modal`.
    - Unit cases `check_input_box_signature_long_turn_*` (`ledger.rs:2429-2467`).
  - `src/cmd/verify/typed.rs` `framing_turns` keeps that screen (`typed.rs:157-162`, `:391-398`, `:414-416`). It
    stays `None` when the long turn's `box_wait` fails or the turn never ends.
  - The fake agent's test-only `--long-turn-screen <phase>` (`src/bin/viola-fake-agent.rs:99`, `:144`, `:670-671`,
    `:722-725`, `:736-741`):
    - It draws `Screen.<phase>` of the set after the long paste's turn alone, after its Stop, in place of
      `Screen.turn`, in the way `--tag-turn-screen` acts after the tag-like turn.
    - `cfg(feature = "fake-agent")`.
    - Parse cases `opts_long_turn_screen_*` (`:1328-1342`).
  - Test support (`tests/support/home.rs`): `wait_ready`'s stall report.
    - At the deadline, before the panic, the watch report gains the stalled wrapper's last `run-<name>.ndjson` event.
    - On macOS it also gains `/usr/bin/sample <pid> 1`'s call-graph stack lines (bounded at 4 s; at most 120 lines).
    - On Linux it gains `/proc/<pid>/wchan` and the `stat` state.
    - Functions: `note_stall` (`home.rs:645-664`), `wrapper_pid` (`:896-911`, one sysinfo read) and `process_state`,
      per OS (`:913-952` macOS, `:954-965` Linux, `:967-970` elsewhere).
    - It signals nothing and adds nothing to a passing boot.
  - `tests/cli_verify.rs`: `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal` (`:1486-1542`).
  - `tests/contract_ledger_probes.rs:69`: `STAMPED` is `["2.1.287", "2.1.302"]`.
  - `tests/contract_fixture_hygiene.rs:355-358`: `SIGNATURE_LITERALS` is `[&str; 4]` and gains
    `"(shift+tab to cycle)"`.
  - No IPC method, endpoint, event kind, field, port, socket, env var or CLI flag changed.
- **Crates / modules:** none added or removed. Changed: `viola-agent-claude` (`screen`, `ledger`) and the root bin
  (`cmd::verify::typed`, the `viola-fake-agent` test bin).
- **Dependencies:** none.
- **Schema / config:**
  - The new recorded fixture set `fixtures/claude/2.1.302/` (29 files: the spine, dialog, framing, status line,
    resume, compact, agents and screen variants), recorded by `viola verify --record` at `24 pass  0 fail` on the
    Linux dev host.
  - Its `Screen.ready.json` and `Screen.turn.json` keep one row, `  ⏵⏵ auto mode on (shift+tab to cycle) · ← for
    agents`. Its `Screen.modal.json` keeps `   Yes, I trust this folder`.
  - The `StatusLine.*` variants keep `workspace.repo.owner` `Turbolet85` (this repository's own remote, the kept
    exception).
  - No schema file, config key or scrub shape changed.
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved** (the site sweep: `cascade.py sweep --pre-pass`, patterns
  `{run_dir}/report-patterns.toml`, trail `cascade-2026-10-19-newer-claude-cli-rows-pre-pass.json`, baseline HEAD
  `4eaca1d3`):
  - **F1-literal — the compiled input-box literals: 1 → 2** (`for agents`, `(shift+tab to cycle)`). `F1-literal:
    standing 15`:
    - **Changed (2 sites):**
      - `architecture.md:45`, at @c2084 and @c2111 (`input_box: ["for agents"]`), stale;
      - `test-plan.md:508` (`SIGNATURES`: the input-box literal `for agents`), stale.
    - **No change, true claims sharing the token (13):**
      - `architecture.md:45`'s other offsets (@c2028, 2201, 2267, 2479, 2780, 3678: the gate's rule, still true);
      - `architecture.md:4`, `:93`, `:101` @c6386 and @c8589;
      - `architecture.md:111` (`--json` for agents, another sense);
      - `security-plan.md:115`, `:528` (never build signatures from screen text, true);
      - `security-plan.md:349`, `:440`, `:533` (self_update `signatures`, another subject);
      - `test-plan.md:56`, `:1090`; `obs-plan.md:76`;
      - the key file `registries/contracts/architecture/crate-dependency-direction.md:6` (names the symbol).
  - **F2-ibsig — the `input-box-signature` row reads ready + turn → ready + turn + the long turn's settled screen.**
    `F2-ibsig: standing 4`:
    - **Changed (1):** `architecture.md:45` @c2417 ("the ready and turn screens hold the input-box literal"), stale.
    - **No change (3):** row-id lists at `architecture.md:101`, `layout-templates.md:204` and the key file
      `registries/contracts/test-plan/5-command-implementation.md:43`.
  - **F3-stamp — CLI versions stamped `24 pass  0 fail` on the Linux dev host: 2.1.287 → 2.1.287 and 2.1.302.**
    `contract_ledger_probes`' `STAMPED` sets: 1 → 2. `F3-stamp: standing 80`:
    - **Changed (3):**
      - `architecture.md:101` @c19987 ("a re-stamp of every stamped version (2.1.287 alone today …)"), stale;
      - `test-plan.md:601` ("for each stamped set (`fixtures/claude/2.1.287/` alone today)"), stale;
      - `test-plan.md:968` (the hygiene walk's set enumeration, "today the sets `2.1.283`, `2.1.287` and `2.1.288`",
        and the screens "of `2.1.287` and `2.1.288`"), stale by omission.
    - **No change (77), by class:**
      - the history of when each row was first stamped on 2.1.287 (`architecture.md:46`, `:97`, `:425` ×11,
        `security-plan.md:217`, `test-plan.md:667`, …), still true;
      - the `stamped <ver>  <n> pass  <m> fail` format and its examples (`layout-templates.md:193`, `:197`, `:204`,
        `:337`, `design-system.md:506`, `:515`, `test-plan.md:102`, `:249`, `:603`, `:785`, `obs-plan.md:664`,
        `security-plan.md:543`);
      - the `stamped_home` / `unstamped` / `read_budget_stamped` / "timestamped" tokens, another subject;
      - `architecture.md:110` @c3494 and `security-plan.md:261` @c2854 ("as measured on 2.1.296 and 2.1.302"), true.
    - **Leaf:** `CLAUDE.md:42` ("stamped on 2.1.287 on the Linux dev host"). It is the cascade's to re-derive, not
      this chunk's edit.
  - **F4-fakeopt — the fake agent's test-only screen switches: 1 → 2** (`--tag-turn-screen`, `--long-turn-screen`).
    `F4-fakeopt: standing 3`:
    - **Changed (3), stale by omission:** `architecture.md:315` @c2085, `test-plan.md:589` @c2053 and
      `test-plan.md:960` @c3290.
  - **F5-stall — the test-side `wait_ready` stall report** (new; no master states it). `F5-stall: standing 4`, all
    the product's `Gate::wait_ready` or the `not ready for input` hint, another subject (`architecture.md:45`,
    `:101`, `design-system.md:460`, `layout-templates.md:114`): no change. Its home is test-plan (the expected
    amendment below).
  - **F6-residual — the footer reading's crossing**, one scratch `viola run` start of 2.1.302 whose bottom three
    rows went to `evidence/` (the founder's own answer to Fork 2, `inputs#I6`). `F6-residual: standing 2`:
    - **No change (2):** `architecture.md:110` @c2846 and @c3448, and `security-plan.md:262` (the plugin round's
      residual), another round.
    - **The new fact's home:** security-plan §Data Protection, next to `security-plan.md:261` (verify's trusted runs).
      The footer reading left **0** transcripts (no prompt typed; `evidence/live-round.md`).
  - **F7-hygiene — the hygiene walk's literal copy (`SIGNATURE_LITERALS`, 3 → 4)** and the walked sets.
    `F7-hygiene: standing 6`:
    - **Changed (1):** `test-plan.md:968` (under F3).
    - **No change (5):** `architecture.md:45` (markup, `modal-signature` row), `architecture.md:374`, `:378`,
      `test-plan.md:964` and `:971`. They describe the walk's rule, which is unchanged: every non-empty screen row a
      signature row.
  - **F8-repro — the `cli_verify` cases** gain `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal`.
    `F8-repro: standing 1`:
    - **Changed (1):** `test-plan.md:589` @c2072, stale by omission.
  - **Owner patterns:** owner pattern: control unfired. The refusal line, verbatim: `control: owner-title,
    owner-generic never fired over the seven masters' pre-pass text (4eaca1d3)`.
  - Sweep reach: 8 patterns · 8 moved facts, 8 with a pattern (the report's own count) · owner pattern control
    unfired · by line 15 lines · 55 offsets.
- **Dev-tool versions:**
  - none — `claude` re-read at 2.1.302 by path on the Linux dev host (`$HOME/.local/share/mise/installs/claude/2.1.302/claude
    --version` → `2.1.302 (Claude Code)`, 2026-10-20T05:05Z, `footer-reading.sh`). The builder session's own PATH
    `claude` reads 2.1.296 through the overseer's pin shim, unchanged.
  - `tmux` 3.7c was read on the dev host as the footer rig's pane, a new use and no version change.
- **Harness / gate surface:**
  - the fake agent's `--long-turn-screen <phase>` (test-only);
  - `tests/support/home.rs` `wait_ready`'s stall report (test-only).
  - No `agent-run` verb, status shape, CI step or verdict shape changed.
- **Cross-project / external claims:**
  - **CI:**
    - ci#39345408847 (sha `3bdcd4a7b539…`): green 15/15, attempt 1.
    - ci#39374691224 (sha `4eaca1d3af32b3562ef00f58d41684e559100b5d`): green 15/15, attempt 1
      (`gh run view … --json attempt,conclusion,headSha` → `1 · success · 4eaca1d3…`).
    - ci#39321117796 (sha `b6ddc198…`, the switch chunk's): attempt 1 red on `test (macos-latest)` (the readiness
      stall), attempt 2 green. It was read for step 6(b).
  - **The 2.1.302 CLI bundle,** read by path with no start, outside the repository:
    - the footer hint drawn while `agentsHintExpired` is false (`inputs#I3` R4, re-read);
    - the tip `Hit shift+tab to cycle between manual mode, auto-accept edit mode, and plan mode`;
    - the Skills dialog hint `enter/space to cycle, / to search, …`.

    The last two were read at implement and are not snapshotted; their text is quoted in `evidence/footer-2.1.302.md`
    and the `resting_footer` cases.
  - **Inputs** (`inputs.py verify`, fired before this report: `inputs: 8 entries — unchanged 2 · drifted 0 · vanished
    0 · broken 0 · altered 0 · unreachable 0 · n/a 6 · uncited 4 · unparsed 0`):
    - I1 · `../additional/viola-overseer/phase-cli-rows-directive.md` · copy · unchanged — the phase directive.
    - I2 · message · copy · n/a — the phase invocation.
    - I3 · message · copy · n/a — P3's outside readings R1–R5.
    - I4 · message · copy · n/a — the forks and the delegate direction.
    - I5 · message · copy · n/a, cited here as `inputs#I5` — the P5 word on
      `.andromeda/runs/2026-10-19T23-10-05-phase/p5-card-1.md`: "yes — the overseer's word … as the founder's
      delegate … PROVISIONAL". The card's `## Beyond the plan` rows, verbatim: `none`.
    - I6 · message · copy · n/a, cited here as `inputs#I6` — the founder's own answers: Fork 1 «19 (Recommended)»,
      Fork 2 «Один scratch-старт (Recommended)», the route «Оставить после (Recommended)», read
      2026-10-20T04:59:04Z. His own word.
    - I7 · message · copy · n/a, cited here as `inputs#I7` — the operator's implement invocation for steps 8–13.
    - I8 · `../additional/viola-overseer/wrap-cli-rows-directive.md` · copy · unchanged, snapped at this wrap, cited
      here as `inputs#I8` — the overseer's wrap directive. Its items: the outcome basis; entry 24 carried as this
      chunk's own red on a recorded companion, the commit and flip proceeding, the guard text unedited; the
      remove-the-guard reading over a scratch clone; the macOS criterion unmet, routed by a WATCH on the next
      markerless entry; «Epoch 5 cleanup» after this entry on the founder's own word.
- **Reverted / negative API facts:** none in the diff. During implement, the literal was neutralised in the tree for
  a remove-the-guard run. The permission classifier refused the run, and the literal was restored (read back at
  `screen.rs:41`). The reading was then taken over a scratch clone (`evidence/remove-the-guard.md`).
- **Insufficient fixes (written, kept, not the remedy):** none.
  - The `wait_ready` stall report (step 6(a)) is written to name the stalled phase. No stall has occurred since it
    landed (two CI runs), so it has resolved nothing yet and is not a remedy. It is the witness for the macOS
    criterion's owner (see Outcome).
- **Spec claims disproved by measurement:**
  1. **F1-literal: `for agents` as the input box's (one) compiled literal is disproved as a stable signature on
     2.1.302.**
     - Where: `architecture.md:45`'s `input_box: ["for agents"]` and the gate's reliance on it; `test-plan.md:508`.
     - Evidence: `evidence/footer-2.1.302.md`. One live 2.1.302 start; the bottom rows at ~15 s and ~25 s lack
       `for agents` and keep `  ⏵⏵ auto mode on (shift+tab to cycle)`.
     - Disposition proposed: amend the two sites to the two-literal set (Expected amendment 1).
  2. **F2-ibsig: "the ready and turn screens hold the input-box literal" is disproved as a sufficient row.**
     - Where: `architecture.md:45` @c2417.
     - Evidence: the overseer's 2.1.302 verify passed that row while the literal vanished later
       (`chunks/2026-10-19-overseer-switch-to-viola/inputs/I20-verify-2.1.302-20261019.out.txt`; reproduced
       `20 pass  4 fail` in `evidence/reproduction.md`).
     - Disposition proposed: amend to the three screens (Expected amendment 1).
  3. **The plan's expectation that the footer reading leaves one transcript** (`plan.md` entry 15's note): measured
     0 (`evidence/live-round.md`). It is a plan claim, not a master's. Disposition proposed: the security amendment
     states the measured 0.
- **Expected amendments (from plan):**
  1. architecture §Established Decisions [CLI Version Compatibility] — carried: the literal set, the
     `input-box-signature` row and 2.1.302 stamped.
     - The facts: Symbols / APIs, and F1-literal, F2-ibsig and F3-stamp under Counts.
     - The search: F1-literal / F2-ibsig / F3-stamp, `architecture.md` standing rows 5 / 2 / 16 (the pre-pass
       rows, counted per master). The sites:
       `:45` [Screen Model], `:101` [CLI Version Compatibility] @c19987, and `:315` (fake agent, F4).
  2. test-plan §4 / §5 / §6 — carried: the `long_turn` cases, the reproduction case, the moved `cli_verify` literals
     (none moved), the fake agent's `--long-turn-screen`, the 2.1.302 set on the stamped list and the stall report
     in `wait_ready`.
     - The facts: Symbols / APIs, and F3, F4, F5, F7 and F8 under Counts.
     - The search: F1 / F3 / F4 / F7 / F8 `test-plan.md` standing rows 3 / 30 / 2 / 3 / 1 (the pre-pass rows,
       counted per master). The sites: `:508`, `:589`, `:601`, `:960`, `:968`.
  3. security-plan §Data Protection, the accepted residual of verify's trusted runs — carried, under Boundary
     widening.
     - The facts: F6-residual under Counts.
     - Authority: the founder's own answer to Fork 2 is on record (`inputs#I6`, read 2026-10-20T04:59:04Z, his own
       word at his keyboard). So it is ratified, not escalated as unanswered.
     - The search: F6-residual, `security-plan.md` 1 standing row (`:262`); its home is next to `:261`.
  4. (Not an amendment) `CLAUDE.md` Critical Warnings, "stamped on 2.1.287". The cascade re-derives it from the
     amended architecture. The search: F3-stamp leaf `CLAUDE.md:42`.
- **Coverage of new surfaces:**
  - `SIGNATURES.input_box` `(shift+tab to cycle)` → validation: compiled literal, never built from screen text ✓ ·
    instrumentation n/a (no new line) · PII n/a (a compiled literal; the recorded rows scrubbed, the hygiene walk
    green) · tests unit (`resting_footer` cases) + contract (`contract_ledger_probes` over 2.1.302) + live (record
    round, `--local-live`) · a11y n/a · tokens n/a.
  - `TypedRun::long_turn` / `input-box-signature` reads it → validation n/a (internal) · instrumentation n/a (no
    `measured` field) · PII n/a (rows never stamped) · tests unit (`check_input_box_signature_long_turn_*`) +
    integration (`cli_verify` reproduction) · a11y n/a · tokens n/a.
  - The fake agent's `--long-turn-screen` → test-only (`cfg(feature = "fake-agent")`) · tests unit (parse cases) +
    integration · others n/a.
  - `wait_ready`'s stall report → test-only · it signals nothing · tests: CI runs both green, no stall fired ·
    others n/a.
  - `fixtures/claude/2.1.302/` → validation: hygiene walk ✓ · PII redacted ✓ (home and user word scrubbed; the
    repository owner the kept exception) · tests contract ✓.

## Deviations from intent
- **Implement run 1** (`.andromeda/runs/2026-10-19T23-48-08-implement/outcome-2026-10-19-newer-claude-cli-rows.md`,
  steps 1–7):
  1. Step 6(a): the wrapper's pid comes from one sysinfo read (the `viola` whose args hold the boot's control file).
     `tests/support/outer_pty.rs` exposes none and is outside the guard's paths.
  2. Step 6(a): `sample` is bounded at 4 s, writes to a file and keeps ≤ 120 stack lines, to stay inside the 20 s CI
     kill line. It is compiled per OS.
  3. Step 5: no `cli_verify` literal moved; the `--tag-turn-screen` case stays `21 pass  3 fail`.
  4. Step 4: the Windows literal `stamped 2.1.0  16 pass  5 fail` was a forecast. CI `windows-2025` then held it
     (ci#39345408847).
  5. Step 2: the three cases are named `#[test]` functions; the modal case loops over the two modal rows.
  6. Steps 6(c) and 7 were not done, by the plan's own rule (no reading named the phase).
  7. Research's `for agents` sweep was deferred to the literal change (step 9).
- **Implement run 2** (`.andromeda/runs/2026-10-20T05-01-00-implement/outcome-2026-10-19-newer-claude-cli-rows.md`,
  steps 8–12, gated mode):
  1. **Step 9, the literal's choice.**
     - Taken literally, "the shortest text the resting input box keeps" admits any substring.
     - Read as the shortest phrase on the measured row that is permission-mode-independent (`auto mode on` is the
       user's mode word), is not the rule row, and is on no other bundle text (the tip, the Skills hint).
     - The choice `(shift+tab to cycle)` was validated live at `24 pass  0 fail`.
  2. Step 9: extra cases beyond the plan's two — the beside-a-modal verdict, and the rule, blank, tip and Skills-hint
     rows `false`.
  3. Step 8's "last three rows" were taken as the pane's rows 21–23 (`capture-pane -S 21 -E 23`).
  4. Step 8 reads `claude --version` by path before its start (verification-harness.md 2026-10-10). It starts no
     session.
  5. The footer reading wrote 0 transcripts (the plan expected 1).
  6. The `cli_verified:true` criterion was read from the record home's `ledger/stamps.json` (all 24 rows `pass`),
     which `stamps_verdict` reads as `(true, None)`. No further live start was made.
  7. A limit of CI's replay: the recorded 2.1.302 `ready` and `turn` screens hold both literals on one row. So CI's
     fake agent never draws the hint-less resting row; the unit cases alone cover it.
  8. Entry 17 (the census after `--local-live`) is unreachable by `--entry` without leg 18, which is not run under
     option 1. Its loop was run by hand right after the firing and printed nothing.
- **The scope record** (`gate.py scope`: clean — changed 37 · listed 36 · recorded 1 (companion 1 · mechanical 0 ·
  in-intent 0 · widening 0) · absorbed 29 · excluded 79):
  - companion: `tests/contract_fixture_hygiene.rs` · serves `crates/viola-agent-claude/src/screen.rs` · self. The
    hygiene walk's own copy of the signature literals: without the new one it reads the recorded 2.1.302 screens
    `NotASignatureRow`. Research did not list it; its deferred `for agents` sweep found it at
    `tests/contract_fixture_hygiene.rs:357`.
- **Entry 24 (the preservation guard) reads red on that companion alone.** This is the overseer's technical direction
  (`inputs#I8` item 2), carried as this chunk's own red on a recorded companion, not as a not-this-chunk's red:
  - the commit and the flip proceed on that direction;
  - the guard's text is unedited;
  - the open class is the one the handoff carries ("the preservation guard reads red on any recorded companion edit").

## Decisions & corrections
- The founder's own answers (`inputs#I6`, read 2026-10-20T04:59:04Z): Fork 1 option 1 (19 live starts) and Fork 2
  option 1 (one scratch start, footer rows only). These are his own ratification of the footer-rows crossing. On the
  route, «Epoch 5 cleanup» stays after this entry («Оставить после»), no longer the overseer's PROVISIONAL placement.
- The operator's invocation (`inputs#I7`): snap the answer first; ledger every live start with the clock's time
  before making it; then the operator pass; stop before the wrap. It was followed: `evidence/live-round.md`'s start
  ledger, 13 of 19 starts.
- The overseer's wrap directive (`inputs#I8`): entry 24 carried as this chunk's red on a recorded companion, with the
  commit and flip proceeding; the remove-the-guard reading over a scratch clone of HEAD; the macOS criterion routed
  by a WATCH on the next markerless entry.
- **Guard refusal:** the permission classifier refused the in-tree remove-the-guard run for the new literal
  (`[Security Test Removal]`), refused with a verdict. It was not retried in the tree, and the literal was restored.
  The scratch-clone form (`gate.py run --entry 8 --project-root {clone}`) was not refused, and it read red:
  2 of 10 failed, exactly the two cases that rest on the literal (`evidence/remove-the-guard.md`).
- **Guard refusal:** the Bash guard refused a recursive `rm` of the scratch clone and its 1.1 G target dir. Both are
  left and named for the founder's desk: `$TMPDIR/claude-1000/…/scratchpad/ctl-4eaca1d3`, `target/wrap-ctl-target`.
- **Guard refusal:** the Bash guard refused a `cd` into the wrap skill's `references/` dir. Re-issued with absolute
  paths. This recurs the carried host-linux.md §Paths learning.
- **Guard refusal:** the Bash guard refused a census call piped into a filter (`| head -0`). Re-issued bare. This
  recurs the carried "a pipeline tool's stdout" learning.
- **Hook refusal:** the `evolve.py` append refused a step record whose `ts` was typed ahead of the clock
  (`REFUSED ts: 2026-10-20T05:21:00Z … later than the clock 2026-10-20T05:20:53Z`). Re-fired with the clock's own
  time.
- **A measured fact:** the 2.1.302 resting footer's mode word follows the user's permission mode (`auto mode on` on
  this host, `manual mode on` in 2.1.287's recording). A screen literal tied to the mode word would match one user's
  mode only.
- **A measured fact:** a live round's census entry placed between two `live` legs cannot be fired through
  `--live-legs --entry` when the later leg is a fork option not taken (entry 17 before leg 18). The tool fires a
  round step only with its next leg.
- **A measured fact:** a probe placed in a detached `tmux` pane (`new-session -d`, `capture-pane -S 21 -E 23`) reads
  the CLI's bottom rows without typing a key. The tmux server it starts exits with its last session.
- **A sweep hazard:** the pattern `stamped` (F3) reads the `stamped_home`, `unstamped` and `read_budget_stamped`
  identifiers and "timestamped" as hits. 77 of its 80 master hits were a true claim sharing the token.

## Outcome
- **Acceptance criteria** (re-asserted against the diff):
  - **(arch) The four red rows are one stop, reproduced without a live start — met.**
    - `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal` exits 1 at `19 pass  5 fail` on Unix
      (measured) and `16 pass  5 fail` on Windows (CI `windows-2025` green with the case asserting its whole stdout,
      ci#39345408847 and ci#39374691224).
    - Its five red rows are as planned, and its receipt holds the probe prompt and the long paste only.
    - The red-before `20 pass  4 fail` is in `evidence/reproduction.md`.
  - **(arch) `input-box-signature` reads the long turn too — met.** It is covered by the `long_turn` unit cases
    (entry 7 green). `measured` is unchanged.
  - **(tests) The fake agent still stamps 2.1.287 `24 pass  0 fail` — met.** It is checked by `contract_ledger_probes`,
    `stamped_home` and `boot` (entries 23 and 27 green), with no `cli_verify` literal moved and `--tag-turn-screen`
    unchanged.
  - **(tests, security) Run B's six paste literals, their order, the six-start count and the eight spawn pairs
    unchanged — met in substance; the guard reads red.** `ledger.rs`'s paste literals are not in the diff's added
    ranges, and the spawn-pair cases are green (entry 21: 115/115). The preservation guard (entry 24) reads red on
    the one recorded companion (see Deviations), on the overseer's direction (`inputs#I8` item 2).
  - **(obs) No new `event`, field, `subject` or `detail` — met** (Changes: none). G4 is green (entry 30).
  - **(tests, a11y) The macOS readiness flake — UNMET.**
    - The stalled phase is not named.
    - Across the runs read, the stall occurred on 1 of 4 macOS attempts (ci#39321117796 attempt 1). It did not occur
      on ci#39345408847 or ci#39374691224.
    - The stall report now in `wait_ready` is its witness.
    - `test (macos-latest)` was green on attempt 1 of the final CI run, but that does not close it.
    - Routed at P5: a WATCH on the next markerless entry («Epoch 5 cleanup»), as the directive asks (`inputs#I8`
      item 4).
    - This criterion is not matrix-linked.
  - **(tests) `pre-push` and the CI read green — met.** `pre-push` green 85.46 s; ci#39374691224 green 15/15,
    attempt 1.
  - **Gated (met, on the founder's word `inputs#I6`):**
    - (security) `evidence/footer-2.1.302.md` holds the rows at ~15 s and ~25 s and whether `viola list` expired the
      hint (not separable: both timers had run out by 15 s). The rows are scrubbed, with no row above the footer.
    - (arch) `SIGNATURES.input_box` holds both compiled literals. The `resting_footer` cases read the measured row,
      the 2.1.287 row and a quiet resting-only screen `Ready` (entry 8: 10 passed).
    - (arch, design, layouts) The record round printed the 24 `[NN/24]` lines and `stamped 2.1.302  24 pass  0 fail`,
      exit 0. `fixtures/claude/2.1.302/` is committed, hygiene-green and stamped-listed.
    - (tests) `--local-live` with 2.1.302 first on `PATH`: `local-live` passed 1.
    - (security) The transcript delta is in the accepted class: +5 for the record verify (Run B 3, C 1, D 1, E 0),
      +5 for `--local-live`'s verify, 0 for the footer reading.
    - (arch) The record home's stamp is all 24 `pass`, read as `cli_verified:true` (`stamps_verdict`).
  - **No verification-matrix claim:** `matrix.py show --chunk` → `claimed by 2026-10-19-newer-claude-cli-rows: 0`.
- **Gates** (by `run` text; implement run 2's final block, trail
  `.andromeda/runs/2026-10-20T05-01-00-implement/gate-2026-10-19-newer-claude-cli-rows.json`):
  - **Green:**
    - `cargo fmt --all --check`;
    - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`;
    - `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc …`;
    - the G1 `rg … instrument` (green · exit 1, no output);
    - `bash scripts/lint-probes.sh`;
    - `run --unit`;
    - `run --unit --filter 'test(/long_turn/)'`;
    - `run --unit --filter 'test(/resting_footer/)'` (green · void: block cleared — the founder's own answers);
    - the fixture guard `test -z "$(git diff … fixtures/claude)" && …` (green · void: block cleared);
    - `run --integration --filter 'binary(cli_verify) | …'` (115 passed);
    - `run --integration --filter 'test(/verify_pastes_nothing_more…/)'`;
    - `run --e2e --filter 'binary(contract_ledger_probes) | …'` (90 passed);
    - `cargo deny check`;
    - the smoke `cleanup` / `boot` / `status` / `cleanup`;
    - `bash scripts/g2-zero-panics.sh`;
    - `run schema-check`;
    - `run pre-push` (88.75 s, then 85.46 s before the commit).
  - **Red:** the preservation guard `git diff --quiet 50e2f76167ee -- . ':!…'` — `red · exit 0 ✗ (exit 1)`. Its one
    path is the recorded companion `tests/contract_fixture_hygiene.rs`; this chunk's red, on the overseer's direction.
  - **The live round** (legs, never re-run by the wrap; its listings in `evidence/round-050522Z.txt`,
    `round-051156Z.txt` and `round-051400Z.txt`):
    - the probe `test -x …2.1.302/claude && command -v tmux` — green;
    - the transcript count — recorded, `107`;
    - `bash …/footer-reading.sh` — green, 28.9 s;
    - the census after it — green;
    - the record `verify --record` — green, 69.2 s, `stamped 2.1.302  24 pass  0 fail`;
    - the census after it — green;
    - the count — recorded, `112`;
    - `--local-live` — green, 63.71 s, `local-live` passed 1;
    - each firing's verdict: `round: COMPLETE · legs fired 1/1`.
    - The census after `--local-live` was not run by the tool (unreachable without the option-2 leg); its loop by
      hand was empty.
    - The option-2 2.1.296 verify and its census: not run — `defer` (Fork 1 option 1).
  - **Operator entries** (`evidence/operator-pass.md`):
    - `gate.py hygiene` — clean, twice (pass 1 and pass 2);
    - `git push origin HEAD` — green, `history moved: … 3bdcd4a7→4eaca1d3` (pass 2);
    - `ci.py conclusion --sha HEAD --wait 2400` — `verdict: green · checks 15/15`, ci#39374691224.
  - **Smoke:** green (the boot-path's readers of `SIGNATURES` changed). `boot` stamped through verify with the new
    set; `status` ready; `cleanup` `processes_gone` and `endpoint_gone` true.
  - **The remove-the-guard reading** (`evidence/remove-the-guard.md`): red with the literal removed over a scratch
    clone (2 of 10 failed), green in the tree (10 passed).
- **Watches:** the truncated `viola` `.profraw` at `pre-push`'s coverage merge — 3 green runs, no recurrence:
  implement run 1 once (its block's `pre-push` entry); implement run 2 twice (entry 32 in the block, `32.log`, then
  `--entry 32` before the commit, `32.2.log`). The run ids are the gate trails' `pre-push` records,
  `.andromeda/runs/2026-10-19T23-48-08-implement/gate-…json` and `.andromeda/runs/2026-10-20T05-01-00-implement/gate-…json`.
  [corrected at this wrap's P5: the first write read "4 green runs … three times", a miscount of run 2's firings.]
- **Outcome basis:**
  - The operator pass ran twice, with no fix commit: `3bdcd4a7` (non-live steps) → ci#39345408847 green 15/15,
    attempt 1; `4eaca1d3` (the gated live round) → ci#39374691224 green 15/15, attempt 1. The final HEAD's run is
    recorded in `evidence/operator-pass.md`.
  - Implement run 1's persisted report:
    `.andromeda/runs/2026-10-19T23-48-08-implement/outcome-2026-10-19-newer-claude-cli-rows.md`. It is superseded
    for the macOS readings by the operator pass's CI readings (`evidence/macos-run-start.md`).
  - Implement run 2's report: `.andromeda/runs/2026-10-20T05-01-00-implement/outcome-2026-10-19-newer-claude-cli-rows.md`.
    This window holds it whole.
  - Post-implement artifacts: `evidence/operator-pass.md` (pass 2) and `evidence/macos-run-start.md`
    (ci#39374691224), both written after the commit; `evidence/remove-the-guard.md` (this wrap, `inputs#I8`).
- **Process hygiene:**
  - Implement runs 1 and 2's censuses: no process this chunk started left running (run 2: `census: 17 running`, none
    with `projects/viola/target/` in its command line, no `tmux`).
  - Re-measured at this wrap: `gate.py census --comm viola-fake-agent --comm tmux --args projects/viola/target/`
    (trail `.andromeda/runs/2026-10-20T05-48-20-wrap/census-2026-10-19-newer-claude-cli-rows.json`) →
    `census: none running`.
  - The scratch clone's own processes (cargo, nextest) exited with its gate entry.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 50e2f761 (the parent of the oldest pre-CI commit 3bdcd4a7) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/ledger.rs — added 47 line(s) in 5 range(s)
added: 603-604 · 610 · 1011-1013 · 1834 · 2429-2468
  - 2429-2436 @2430 «fn check_input_box_signature_long_turn_holding_the_literal_passes() {»
  - 2438-2453 @2439 «fn check_input_box_signature_long_turn_holding_a_modal_fails() {»
  - 2455-2467 @2456 «fn check_input_box_signature_long_turn_of_none_fails() {»
### crates/viola-agent-claude/src/screen.rs — added 50 line(s) in 4 range(s)
added: 36-39 · 41 · 416 · 433-476
  - 449-452 @450 «fn holds_any_reads_the_resting_footer(#[case] row: &str, #[case] expected: bool) {»
  - 454-463 @455 «fn verdict_over_the_2_1_302_resting_footer_alone_is_ready() {»
  - 465-475 @466 «fn verdict_over_the_resting_footer_beside_a_modal_is_input_not_ready() {»
### fixtures/claude/2.1.302/Agents.recorded-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PermissionRequest.permission-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PermissionRequest.plan-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PostToolUse.parallel-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PostToolUse.parallel-2.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PostToolUse.permission-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PostToolUse.plan-2.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PostToolUse.questions-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PreToolUse.parallel-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PreToolUse.parallel-2.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PreToolUse.plan-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PreToolUse.plan-2.json — new file · 1 line(s)
### fixtures/claude/2.1.302/PreToolUse.questions-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/Screen.modal.json — new file · 1 line(s)
### fixtures/claude/2.1.302/Screen.ready.json — new file · 1 line(s)
### fixtures/claude/2.1.302/Screen.turn.json — new file · 1 line(s)
### fixtures/claude/2.1.302/SessionEnd.clear-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/SessionEnd.default.json — new file · 1 line(s)
### fixtures/claude/2.1.302/SessionStart.clear-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/SessionStart.compact-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/SessionStart.default.json — new file · 1 line(s)
### fixtures/claude/2.1.302/SessionStart.resume-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/StatusLine.default-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/StatusLine.rate-limits-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/Stop.default.json — new file · 1 line(s)
### fixtures/claude/2.1.302/UserPromptSubmit.default.json — new file · 1 line(s)
### fixtures/claude/2.1.302/UserPromptSubmit.paste-1.json — new file · 1 line(s)
### fixtures/claude/2.1.302/UserPromptSubmit.paste-2.json — new file · 1 line(s)
### fixtures/claude/2.1.302/UserPromptSubmit.paste-3.json — new file · 1 line(s)
### src/bin/viola-fake-agent.rs — added 33 line(s) in 8 range(s)
added: 99 · 144 · 670-671 · 722-725 · 736-741 · 1240-1241 · 1279 · 1328-1343
  - 1328-1333 @1329 «fn opts_long_turn_screen_takes_the_phase_it_names_alone() {»
  - 1335-1342 @1336 «fn opts_long_turn_screen_defaults_to_none() {»
### src/cmd/verify/typed.rs — added 17 line(s) in 3 range(s)
added: 157-162 · 391-398 · 414-416
  - 157-162 «typed.compact_latency_ms = framing_turns(»
### tests/cli_verify.rs — added 58 line(s) in 1 range(s)
added: 1486-1543
- 1486-1542 @1492 «fn verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal(»
  - 1500-1505 «let after = [»
  - 1508-1514 «let red = [»
  - 1521-1525 «assert_eq!(»
  - 1527-1529 «for id in red {»
  - 1530-1532 «for id in ROW_IDS.iter().filter(|id| !red.contains(id)) {»
  - 1534-1540 «assert_eq!(»
### tests/contract_fixture_hygiene.rs — added 3 line(s) in 2 range(s)
added: 355-356 · 358
### tests/contract_ledger_probes.rs — added 1 line(s) in 1 range(s)
added: 69
### tests/support/home.rs — added 100 line(s) in 3 range(s)
added: 637-639 · 645-665 · 896-971
  - 645-664 @647 «fn note_stall(&self, watch: &Watch) {»
- 896-911 @898 «fn wrapper_pid(control: &Path) -> Option<u32> {»
  - 901-910 «sys.processes()»
- 913-952 @916 «fn process_state(pid: u32) -> Vec<String> {»
  - 919-921 «let out = std::env::temp_dir()»
  - 922-928 «let child = std::process::Command::new("/usr/bin/sample")»
  - 929-931 «let Ok(mut child) = child else {»
  - 933-940 «while child.try_wait().ok().flatten().is_none() {»
  - 943-950 «lines.extend(»
- 954-965 @956 «fn process_state(pid: u32) -> Vec<String> {»
  - 960-963 «let state = stat»
- 967-970 @968 «fn process_state(_pid: u32) -> Vec<String> {»
