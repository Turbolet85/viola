# Report — 2026-10-05-real-cli-verify-probes

**Chunk:** Real-CLI verify probes — typed-input PTY probe against the installed claude, screen signatures and timing,
local-command, dialog-body and paste-framing ledger rows, measured and stamped. Built here: W1 + W5 only (the founder's
three-way split, 05:58Z).
**Date:** 2026-10-05
**Commits:** `012fc50 chore(2026-10-05-real-cli-verify-probes): operator pre-CI commit, for the run this chunk's verdict reads`
(the only commit since `last_wrap` 2026-10-05T00:14:30Z; base `caae9ec`, its parent).

## Changes (structured — detectors read this)
- **Files** (`git diff --name-only caae9ec`, run dirs aside: 55 files):
  - **New:**
    - `src/cmd/verify/typed.rs`;
    - `schemas/claude-screen.v1.json`;
    - `fixtures/claude/2.1.288/` (4 spine payloads + `Screen.{modal,ready,turn}.json`);
    - `fixtures/claude/2.1.287/Screen.{modal,ready,turn}.json`.
  - **Modified:**
    - `crates/viola-agent-claude/src/{ledger.rs,screen.rs}`;
    - `src/cmd/{verify.rs,mod.rs,run.rs}`;
    - `src/run/{gate.rs,send.rs,version_gate.rs}`;
    - `src/bin/viola-fake-agent.rs`;
    - `schemas/diag-line.v1.json`;
    - `crates/viola-e2e/src/harness/{boot.rs,run.rs,supervise.rs}`;
    - `tests/{cli_send.rs,cli_verify.rs,cli_version_gate.rs,contract_diag_schema.rs,contract_fixture_hygiene.rs,contract_ledger_probes.rs}`;
    - `tests/support/{home.rs,verify.rs}`;
    - `Cargo.toml`, `.config/nextest.toml`, `.gitignore`.
  - **Chunk folder:** `evidence/{live-sessions.md, screen-probe-2.1.288.md, hand-entries-7-8.md, operator-pass.md,
    round-094211Z.txt, round-100046Z.txt}`, `scope-record.md`.
  - **Phase's own:** `working-route.md` (`:84` stamped), `master-route.md` (pending record), and
    `verification-matrix.json` (P5 `notes` on v1-34, v1-21, v1-29).
- **Symbols / APIs:**
  - **`viola-agent-claude::ledger`:**
    - `LedgerRow::ALL` grows from 6 to 10: after the six come `ModalSignature`/`modal-signature`,
      `InputBoxSignature`/`input-box-signature`, `QuietPeriod`/`quiet-period` and `ConfirmWindow`/`confirm-window`;
    - new types: `TypedRun { modal, ready, turn: Option<Vec<String>>, ready_settle_ms, turn_settle_ms,
      prompt_latency_ms, max_turn_gap_ms: Option<u64> }` and `Probes { print: ProbeRun, typed: TypedRun }`;
    - `check(row, &Probes)` replaces `check(row, &ProbeRun)` (sole production caller: `src/cmd/verify.rs` `check_step`);
    - `merge_stamp(…, typed: &TypedRun, written_at)` writes `measured.typed_probe` (callers: verify's `measure`, and
      the test stamp in `src/run/version_gate.rs`);
    - new helpers `signature_rows`, `screen_is_clean`, `screen_fault -> Option<ScreenFault { row, seam, why }>` and
      `unclean -> Option<Unclean>`;
    - `Unclean` is a closed code set: `home-path` · `absolute-path` · `username` · `email`. `is_clean` now reads
      through `unclean`.
  - **`viola-agent-claude::screen`:**
    - `pub const SIGNATURES: Signatures { input_box: ["for agents"], modals: ["Yes, I trust this folder",
      "Yes, allow external imports"] }`;
    - `Signatures::holds_any`;
    - `Screen::rows() -> Option<Vec<String>>`, the only row-text exit, read by verify alone;
    - the three constant VALUES are unchanged (300 ms / 5 s / 10 s). Their docs now read "compiled; validated per CLI
      version by the stamped row", no longer "provisional".
  - **`src/run/gate.rs`:** `gate::start(clock, size, sigs: Option<&'static Signatures>)`. `Gate::wait_ready` passes
    `sigs` to `verdict`, which was `None` before. Callers: `pump_child` (production) and the `send.rs` and `gate.rs`
    tests, which pass `None`.
  - **`src/cmd/run.rs`:** `Launched.cli_verified`, set from the version gate. `pump_child` passes `Some(&SIGNATURES)`
    exactly when it is true: the full readiness gate on a verified CLI version, the partial gate otherwise.
  - **`viola verify`:**
    - after the print probe, two interactive PTY runs (`src/cmd/verify/typed.rs`), each a direct spawn under the R8
      strip at a fixed 80×24, output capped at `MAX_FRAME` into a `Screen` fed under `catch_unwind`;
    - **Run A:** a fresh `viola-verify-*` 0700 dir under the OS temp dir; input `io::empty()`; ended by `Pty::kill`;
    - **Run B:** `<cwd>/.viola-verify-<pid>/` (0700, drop guard); one bracketed paste of `PROBE_PROMPT`; ended by
      Ctrl-C, again after 500 ms, then a kill at `PROBE_DEADLINE`;
    - **the settle rule:** the first `QUIET_PERIOD`-quiet instant holding a literal, else the first quiet instant past
      `GATE_MAX_WAIT`;
    - **neither run writes a byte into a CLI dialog.** A Run B start showing a modal is killed with no key;
    - **`--record`** also writes `Screen.<phase>.json` (`{"screen_phase","cols":80,"rows":[24]}`, signature rows only,
      the rest `""`);
    - **the refusal** names the file and the check, never the content:
      `unable: a recorded fixture is not clean: <file> <code>`, and for a screen `<file> row <n>[ seam] <code>`.
      It replaces the fixed `a recorded payload still holds a path or a username`.
  - **`viola verify --help`:** one paragraph. Run it from a folder you trust in Claude Code; an unapproved external
    CLAUDE.md import blocks the probe. ASCII, no path.
  - **The fake agent** gains three argv options, no env:
    - `--trusted-root <DIR>`: an ancestor walk, both paths canonicalized;
    - `--screens`: writes `Screen.modal` untrusted (no hook fired) or `Screen.ready` trusted (SessionStart as before),
      a clear then rows joined by CRLF;
    - `--turn-stop`: a submit fires UserPromptSubmit, then Stop, then writes `Screen.turn`.
  - **Test support:**
    - `verify()` passes `--screens --turn-stop --trusted-root <workspace root>`;
    - `verify_without_screens()` has no test-side bound;
    - `Wrapper::boot*` passes `--screens --trusted-root <cwd>`, and `Wrapper::boot_untrusted` passes no root.
  - **The harness:**
    - boot step 4's verify passes `--screens --turn-stop --trusted-root <ws root>`;
    - supervise passes `--screens --trusted-root <cwd>`;
    - `LEDGER_ROWS` holds the ten literals.
- **Crates / modules:** the root bin gains the module `cmd::verify::typed`. No crate edge changes:
  - `viola-agent-claude` gains no viola-pty, viola-state or tokio dependency;
  - `viola-pty`'s source is unchanged (gate entry `git diff --quiet caae9eca9e4d -- … crates/viola-pty/src` green).
- **Dependencies:** `tempfile.workspace = true` (`=3.27.0`, already a workspace and dev dependency) joins the root
  package's `[dependencies]`, for Run A's dir. `Cargo.lock` is unchanged (`git diff --stat caae9ec -- Cargo.lock`
  empty). No new crate.
- **Schema / config:**
  - **New `schemas/claude-screen.v1.json`:** `screen_phase` ∈ modal|ready|turn, `cols` const 80, `rows` exactly 24
    strings.
  - **`schemas/diag-line.v1.json`** `$defs.subject.enum` gains `verify-pty-probe`.
  - **The stamp entry** gains `measured.typed_probe` `{ready_settle_ms, turn_settle_ms, prompt_latency_ms,
    max_turn_gap_ms}` beside `largest_hook_payload`. `run` reads no number from it.
  - **`.gitignore`:** `/.viola-verify-*/`.
  - **`.config/nextest.toml`:** a `profile.mutants` override, `test(/verify_window_/)` at 15 s × 2.
  - **Fixture classes:**
    - the screen class: rows that hold no compiled literal are blank;
    - fixture hygiene now refuses an email-shaped token and a username split across a row seam;
    - `contract_fixture_hygiene` walks `Screen.*.json` apart from the hook payloads.
- **Spec-master edits:** none. No master changed in this chunk's run; the apply is P2's.
- **Counts / qualifiers moved:**
  - **Capability-ledger rows, 6 → 10:** the `[NN/06]` counter becomes `[NN/10]` and the summary `6 pass` becomes
    `10 pass`. `/06` sites: security-plan 2 · design-system 1 · layout-templates 2 · test-plan 1. The phrase "six
    rows|6 rows|six-row": architecture 4 · security-plan 1 · design-system 1. Basis: `sites.py` pattern counts over
    the seven masters at `caae9ec`.
  - **Stamped recorded sets:** 2.1.287 and 2.1.288. 2.1.283 is now drift-only (`contract_ledger_probes` literal
    lists). `2.1.283` sites: architecture 2 · design-system 1 · layout-templates 2 · test-plan 1.
  - **Screen signatures, PROVISIONAL → compiled and validated per version.** `provisional` sites: architecture 8 ·
    test-plan 1 · obs-plan 3 · a11y-plan 1. Not every hit is about these constants; the per-hit disposition is P2's.
- **Dev-tool versions:** none changed. `claude` re-read at 2.1.288 on the dev host (`claude --version`, gate entry
  6, green), and the installed 2.1.287 binary was used by path. Running sessions are still on 2.1.287.
- **Harness / gate surface:**
  - boot step 4 stamps ten rows through both interactive runs against the fake agent;
  - `local-live`'s literal rows become ten (`harness::run::tests` unit-proven). Its live firing was NOT run here: it
    is owed to `:86` (see Expected amendments).
  - `run --local-live` under `CI=true` still exits 2 `live-in-ci` (gate entry green);
  - `viola` reads no `CI` (gate entry, last line 0).
- **Cross-project / external claims:**
  - **CI** ci#37296910661 on `012fc5089498`: verdict green, 15/15 checks, wall 422 s
    (`evidence/operator-pass.md`, `ci.py conclusion`).
  - **The live CLI on the dev host,** read live with no copy:
    - `claude` 2.1.288 and 2.1.287, mise installs;
    - `~/.claude.json` read as booleans and counts only;
    - the repo-root `projects` entry reads `hasTrustDialogAccepted: true`, `hasClaudeMdExternalIncludesApproved: false`
      and `hasClaudeMdExternalIncludesWarningShown: true` before and after every session, unchanged.
  - **Inputs** (`inputs.py verify`):
    - `I1 · ../additional/viola-overseer/split84-route-adaptation.md · no-repo copy · unchanged` (cited scope.md:15,
      :20, research.md:24, plan.md:553);
    - `I2 · operator message, /andromeda-phase arguments ~00:16Z · message · n/a` (cited scope.md:22, research.md:25,
      plan.md:554);
    - drifted 0, vanished 0, broken 0.
- **Reverted / negative API facts:**
  - **The one-key trust accept** (revision 1) was never built. It was superseded by the founder's "two runs, never
    accept" (~07:00Z).
  - **The fixed refusal text** `a recorded payload still holds a path or a username` was replaced by the named form.
- **Insufficient fixes (written, kept, not the remedy):**
  - **The named refusal** (a widening) did not fix the record red. It made the cause visible
    (`SessionStart.default.json absolute-path`). The remedy was the record home's name, `vhome`, which is the plan
    correction below.
- **Spec claims disproved by measurement:**
  1. **Plan step 10 / entries 7 and 8 (`plan.md:349`, `:360`):** the record command `--home "$h/home"` cannot
     produce clean fixtures. The print probe's `cwd` sits under `<record home>/ledger/probes/<pid>`. The scrub turns
     the user home into `~`, but the record home's own `/home/` component survives, and the hygiene check
     (`has_absolute_path`: any `/home/`) refuses it.
     - Measured: two live rounds red, `round-094211Z.txt` and `round-100046Z.txt` (the second named
       `SessionStart.default.json absolute-path`).
     - Corrected by the overseer (founder-delegated): driven by hand with `$h/vhome`, both green
       (`evidence/hand-entries-7-8.md`).
     - The plan is a chunk artifact, not a master: disposed as a recorded plan correction. Any master text that
       quotes the record command is P2's.
  2. **"A probe dir under an already-trusted parent starts with no modal"** (scope W1 / research M13 as first
     written) holds for the trust dialog only. A subdir of this repo shows "Allow external CLAUDE.md file imports?"
     until the git-root-keyed answer is set (research M19, session 3). Already premise-corrected in scope.md at P3
     rev 3; the masters' [Screen Model] text is P2's.
  3. **The plan's order** "fire the record entries after the non-live gates are green" (`plan.md:256-269`) cannot
     hold. Entries 3, 10, 11 and 21 read the recorded fixtures, so the live round fired first and the gates went
     green after. Disposed as a deviation; no master states it.
- **Expected amendments (from plan)** — each carried by the Changes bullet named, sites from `sites.py` at
  `caae9ec`:
  - **architecture [Screen Model]:**
    - signatures compiled in `SIGNATURES` with two modal literals, and the full gate on a verified version (Symbols:
      screen, gate.rs, run.rs);
    - the three constants validated per version, no longer PROVISIONAL (Counts);
    - trust inherited from a trusted parent, and the external-imports approval keyed on the git root (Spec claims
      disproved 2; research M13, M19).
    - Sites: `[Screen Model]` architecture 1 · `provisional` architecture 8. Carried.
  - **architecture [Delivery Confirmation]:** the window is a compiled value that the per-version `confirm-window`
    row validates (Symbols: ledger, `ConfirmWindow`). Sites: `[Delivery Confirmation]` architecture 2. Carried.
  - **architecture [Plugin Scope], §Occupied Resources, [CLI Version Compatibility]:** verify's two interactive runs;
    Run A's `viola-verify-*` under the OS temp dir and Run B's `<cwd>/.viola-verify-<pid>/`; `Screen.<phase>.json`
    and `schemas/claude-screen.v1.json`; rows 6 → 10 (Symbols: verify; Schema; Counts).
    - Sites: `[Plugin Scope]` architecture 1 · `Occupied Resources` architecture 3, security-plan 12 ·
      `[CLI Version Compatibility]` architecture 2 · six-row architecture 4. Carried.
  - **security-plan:** carried (Symbols: verify; Schema; Cross-project; Outcome residual).
    - §Threat Model Summary: verify's two PTY children, no key into either dialog, the kill;
    - §Data Protection: the two probe dirs incl. the user's cwd, the OS temp root, signature-only screen fixtures,
      the seam and email refusal, the named refusal codes;
    - the Run B transcript residual;
    - the dev host's external-imports answer, set by the overseer on the founder's amended live ruling (~09:05Z).
    - Sites: `Threat Model Summary` security-plan 1 · `Data Protection` security-plan 4 · `/06` security-plan 2 ·
      six-row security-plan 1.
  - **test-plan:** carried (Symbols: fake agent; Harness; Counts).
    - §7: the fake-agent options `--screens` / `--trusted-root` / `--turn-stop` and the screen fixture class;
    - §3: `local-live`'s ten ids, with its live firing at `:86`;
    - the stamp walk: 2.1.287 and 2.1.288 stamped, 2.1.283 drift-only.
    - Sites: `--vt100-panic-bytes` (the §7 modes line) test-plan 1 · `local-live` test-plan 4 · `2.1.283` test-plan 1
      · `/06` test-plan 1.
  - **obs-plan:** §6 `subject` + `verify-pty-probe`; §4 Edge flows `verify` (four child spawns: version, print, Run A,
    Run B) (Schema; Symbols). Sites: `verify-probe` obs-plan 5 · `Edge flows` obs-plan 2. Carried.
  - **design-system / layout-templates §Surface: cli:** the verify wireframe at `/10` and its help paragraph (Symbols:
    verify, `--help`). Sites: `/06` design-system 1, layout-templates 2 · `2.1.283` design-system 1,
    layout-templates 2. Carried.
  - **a11y-plan §4 P6:** the counter. Site: a11y-plan:349 reads the generic `[NN/NN]` (no literal `/06`, 0 hits), and
    :587 is the P6 row. Carried. P2 judges whether the generic form needs a change: likely not.
  - **Route adaptation (wrap, the founder's split ruling):** two entries are minted ahead of "First live test and
    self-drive" (`:86`):
    - **"Dialog rows and re-probe" first:** W3 + W6, scope CARRYs 6, 7, 9 and research M7's crossing;
    - **then "Local-command and paste-framing rows":** W2 + W4, CARRYs 2, 3, 5 and M10.
    - **`:86` additionally carries** `local-live`'s live firing at ten rows.
    - Epoch 3 stays one epoch. Carried to P5. This is a trajectory edit: P5 HALTs for the dialogue.
- **Coverage of new surfaces:**
  - `verify typed PTY runs (Run A / Run B)` → validation: direct spawn, R8 strip, `MAX_FRAME` cap, no key into a
    dialog ✓ · instrumentation: `process-start`/`process-exit{subject:"verify-pty-probe", child_exit_status,
    duration_ms}` per run ✓ · PII: no row text, prompt or path in any line ✓ · tests: unit + integ (`cli_verify` 22) ✓
    · a11y n/a · tokens n/a.
  - `verify --record screen fixtures` → validation: signature rows only, plus `screen_fault` (home, absolute path,
    username, email, row and seam) ✓ · instrumentation n/a · PII: refused whole, named by code ✓ · tests: unit, integ,
    contract hygiene with planted reds ✓ · a11y n/a · tokens n/a.
  - `run full readiness gate (verified CLI)` → validation: compiled signatures ✓ · instrumentation: the existing
    `run.readiness_gate` span ✓ · PII n/a · tests: unit (`gate.rs` ×3) + integ (`cli_send` witness) ✓ · a11y n/a ·
    tokens n/a.
  - `viola verify --help paragraph` → validation n/a · instrumentation n/a · PII: no path ✓ · tests: integ ✓ · a11y:
    ASCII, static ✓ · tokens n/a.
  - `fake agent --screens/--trusted-root/--turn-stop` → test-only (`fake-agent` feature) · tests: unit ✓ · the rest
    n/a.

## Deviations from intent
- **Live record round order.** Fired before the full non-live gates were green, because those gates read the fixtures
  the round records. Justified by Spec claims disproved 3.
- **Entries 7 and 8 driven by hand with `--home "$h/vhome"`.** A dated plan correction, 2026-10-05, by the overseer
  (founder-delegated). The product works as designed (`evidence/hand-entries-7-8.md`).
- **Live sessions: 16 of 18,** against the plan's 10 of 12. The cap was raised live by the founder twice. Spent: two
  red record rounds (6 sessions) plus the two green ones (6), with 4 before those.
- **The no-`--screens` verify case** became `verify_window_without_screens_fails_the_four_new_rows`:
  - it runs about 11 s, two `GATE_MAX_WAIT`s, against the 7 s test bound;
  - it has no test-side deadline (the `send_window_` precedent; a named 20 s const failed `contract_lints`'
    kill-line lint);
  - a mutants-profile override `test(/verify_window_/)` covers it.
- **Stop-less fixture sets.** Three cases move the fake's trust root off the cwd:
  - `verify_a_set_without_stop_fails_its_rows_and_still_stamps` (renamed from `…_fails_two_rows_…`; it now expects
    `5 pass  5 fail`);
  - `verify_with_an_instance_logs_its_start_and_exit`;
  - `cli_version_gate::stamp` with `skip`.
  - Reason: the trusted turn would otherwise wait out `PROBE_DEADLINE` for a Stop that never comes.
- **Plan step 3 said `&dyn Clock`; the drive takes `Arc<dyn Clock>`,** because the pump-thread sink must hold the
  clock across threads.
- **`Screen` gained `rows()`,** the only way to record row text. Its module doc now names that exit.
- **`screen_is_clean`'s "after scrub" is implemented as scrub-as-detector:** a kept text the scrub would rewrite is
  dirty, so nothing is written rewritten.
- **Two existing `merge_stamp` unit literals were extended** with `typed_probe` (the plan said existing literals
  stay).
- **`crates/viola-agent-claude/src/lib.rs`** (listed) needed no edit: the modules are already `pub`.
- **The step-0 scratch probe** (outside the repo) widened its no-key kill to any trust, external-imports or
  dialog-footer text before session 4.
- **Operator-pass hygiene** rewrote 9 host-path hits in place:
  - 8 in phase run-dir raw gate listings, to `$TMPDIR/andromeda-gate/…` and `~/.claude/…`;
  - 1 in this chunk's evidence.
  - No file was removed (`evidence/operator-pass.md`).
- **The scope record** (`gate.py scope`: `clean — changed 36 · listed 31 · recorded 5`):
  - **companion:**
    - `src/run/version_gate.rs` (serves ledger.rs) · self;
    - `Cargo.toml` (serves typed.rs) · self;
    - `src/run/send.rs` (serves gate.rs) · self;
    - `tests/cli_version_gate.rs` (serves typed.rs) · self.
  - **in-intent:** `.config/nextest.toml` (serves tests/cli_verify.rs) · self.
  - **widening** on two listed files (`src/cmd/verify.rs`, `crates/viola-agent-claude/src/ledger.rs`): the scope tool
    reads them `record: listed`, outside its widening tally. word: "make the --record refusal name the file and the
    check code, never the content (a scope-record line)" — the overseer, founder-delegated, 2026-10-05, relayed by
    the operator.

## Decisions & corrections
- **The founder's live rulings** (all 2026-10-05):
  - R-S2 (~00:00Z) ratified the PTY-driven typed-input verify probe, local to the dev host;
  - **the three-way split (05:58Z):** this chunk is W1 + W5. "Dialog rows and re-probe" (W3 + W6) comes first, then
    "Local-command and paste-framing rows" (W2 + W4), both before `:86`;
  - **the trust handling (~07:00Z, re-affirmed ~08:50Z):** two runs, never accept. viola never types into a
    CLI-native dialog, and nothing is written to `~/.claude`;
  - **the two dirs (08:25Z):** Run B in `<cwd>/.viola-verify-<pid>/`, Run A in a fresh 0700 dir under the OS temp
    dir;
  - **the external-imports answer (~08:50Z, amended ~09:05Z):** the overseer set the repo-root flags on the founder's
    word, by an atomic `~/.claude.json` edit equivalent to "No", with a backup kept;
  - **the live cap:** 12 (~08:50Z), then 15 after the first entry-7 STOP, then 18 after the second.
- **The overseer (founder-delegated):**
  - `Yes, allow external imports` is the second modal literal, and the verify help names the blocker (~08:50Z);
  - the named refusal, proven offline first (after the first entry-7 STOP);
  - the `vhome` plan correction (after the second).
- **Input-box literal `for agents`.** The implementer chose it at step 0 within the plan's rule: on both screens, on
  neither dialog, no path or user. `manual mode on` was rejected because it is mode-dependent and runtime-built, not a
  binary literal.
- **The Run B residual** (P5-shown, accepted): each trusted run leaves a synthetic-prompt transcript under
  `~/.claude/projects/` for the dead `.viola-verify-<pid>` path. Five such transcripts were left: session 4 and the four
  record rounds. `~/.claude/projects/` went from 13 dirs to 18. Session 3 was killed at its modal and left none. The user's own global hooks and status
  line also run in Run B. Run A leaves nothing (STOP 2 read clear after every run).
- **Sweep hazards:**
  - the hygiene P1 form reads `…/<dir named home>/ledger/…` as `/home/{user}/…`, so describe such a path, never spell
    it, in committed evidence;
  - `has_absolute_path` reads a drive path only at a text's start.

## Outcome
- **Acceptance criteria,** re-asserted against the diff:
  - (arch) ten rows in order, each with a verify measurement and a compiled post-condition; verified only when all
    ten pass (`ledger::verified` over `LedgerRow::ALL`). **Met.**
  - (arch) the PTY drive is in the root bin. `viola-agent-claude` gains no viola-pty, viola-state or tokio
    dependency, and viola-pty is unchanged. **Met** (seam guard entry green; Cargo.toml adds only `tempfile` to the
    root package).
  - (arch) `SIGNATURES.modals` holds exactly the two literals, and `run` passes `Some(&SIGNATURES)` exactly when
    `cli_verified` (gate.rs unit cases; `cli_send` witness `send_on_a_verified_cli_refuses_while_the_trust_dialog_is_up`).
    **Met.**
  - (security) no byte into a CLI-native dialog. Run A's input is empty and it ends by kill; Run B kills a modal
    start with no key. **Met** (`cli_verify`, step-10 evidence).
  - (security) the external-imports answer was set by the overseer and checked before step 0; the implementer wrote
    no `~/.claude.json` field; the flags were unchanged after each record run. **Met** (`screen-probe-2.1.288.md`,
    `hand-entries-7-8.md`).
  - (security) the two dirs are 0700 and gone after passing and failing runs; the R8 strip and the `MAX_FRAME` cap
    hold. **Met** (`verify_leaves_neither_run_dir_behind` ×2; TrustedDir unit test).
  - (security) after Run A's kill, no `projects` key and no projects dir. **Met** after every run.
  - (security) signature-only screens; the recording refused whole on a home path, the username, an email or a seam
    split. **Met**, with planted reds now named by code.
  - (security) the live probe never runs in CI, and `viola` reads no `CI`. **Met.**
  - (tests) `contract_ledger_probes` stamps `10 pass  0 fail` over 2.1.287 and 2.1.288 with ten literal ids; 2.1.283
    is drift-only; every dir is on exactly one list; `stamped_home` and harness `boot` still end `0 fail` on three CI
    OSes. **Met** (gate entry 10; CI green).
  - (tests) the dev-host 2.1.288 record ends `stamped 2.1.288  10 pass  0 fail`. **Met** under the plan correction:
    the W5 stamp, in `live-sessions.md` row 13.
  - (tests) the 2.1.288 set holds 4 spine payloads + 3 screens; 2.1.287 gained 3 screens with every other file
    byte-identical; no real prompt, path, user or email. **Met** (preservation guard green; hygiene contract green).
  - (obs) one start/exit pair per interactive run; the lines validate; no screen text, prompt or path. **Met**
    (`verify_with_an_instance_logs_both_spawn_pairs`).
  - (design / layouts / a11y) static `[NN/10]` lines and the summary, no SGR or path; `--help` names the folder and
    the blocker in ASCII. **Met.**
  - (budget) `live-sessions.md` lists every session, 16 of the 18 cap. **Met** under the cap rulings: the criterion's
    "at most 12" was superseded live.
  - No matrix capability claimed. **Met** (`matrix.py show --chunk`: claimed 0).
- **Gates** (`plan.md` `[[gate]]`, by `run`; final firing, implement run dir `2026-10-05T09-18-16-implement`, block
  18 green 0 red):
  - `cargo fmt --all --check` green ·
    `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green ·
    `agent-run.sh run --unit --filter 'test(/ledger::tests::|screen::tests::|cmd::verify::|run::gate::tests::|harness::run::tests::/)'`
    green (`"ok":true`) · `agent-run.sh run --unit` green · `git check-ignore -q --no-index .viola-verify-4242/` green;
  - `claude --version` (leg round) and the two record entries (leg live): the round fired twice through the tool,
    both STOPPED red at the 2.1.288 entry (`evidence/round-094211Z.txt`, `round-100046Z.txt`). Then by hand under
    the plan correction: entry 6 green; the 2.1.288 record green (exit 0, `stamped 2.1.288  10 pass  0 fail`,
    artifact fresh); the 2.1.287 record green (exit 0, `stamped 2.1.287  10 pass  0 fail`, artifact fresh)
    (`evidence/hand-entries-7-8.md`);
  - the 2.1.287/2.1.283 preservation guard green · the integration filter (cli_verify, contract_ledger_probes,
    contract_fixture_hygiene, contract_fake_agent_drift, contract_diag_schema, cli_send, cli_answer) green ·
    `agent-run.sh run` green (red once on two test-construction faults, fixed, then green);
  - smoke `cleanup` / `boot` / `status` / `cleanup` (`p-typed-smoke`) green ·
    `CI=true … --local-live` green (exit 2) · the CI-read probe green · the seam guard green · the no-ignore guard
    green · the deny guard green (exit 1, last line 0) · `agent-run.sh pre-push` green;
  - **leg operator:** hygiene `clean` after rewrites; the push `caae9ec..012fc50`; `ci.py conclusion` →
    `012fc5089498 verdict: green · checks 15/15 · wall 422 s · ci#37296910661` (`evidence/operator-pass.md`).
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran.
  - The final state is `012fc50` plus one uncommitted evidence append (the CI verdict in `operator-pass.md`).
  - The CI run at that sha is recorded in `evidence/operator-pass.md`.
  - Implement's P4 report and the session conversation are present (this wrap runs in the implementing session).
- **Process hygiene:** every process this chunk's runs started has ended:
  - session 4's `claude`, every record round's three `claude` children, nextest/cargo, and the scratch diagnostic
    builds. Re-measured at implement P4 and after each hand run: no `claude` with a probe-dir cwd, and no
    `.viola-verify-*` or `/tmp/viola-verify-*` dir.
  - The `viola run …-builder` sessions are the operator's and were left running.
