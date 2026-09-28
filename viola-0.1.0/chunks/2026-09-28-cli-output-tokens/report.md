# Report — 2026-09-28-cli-output-tokens

**Chunk:** CLI output tokens, narrowed at P4 to live consumers: clap's own help and usage output made plain (its
`color` feature off), and `run`'s start refusals routed through one stderr refusal writer in `src/human.rs`.
**Date:** 2026-09-28
**Commits:** since last_wrap 2026-09-28T08:03:36Z: `c2dccf6 chore(2026-09-28-cli-output-tokens): operator pre-CI commit,
for the run this chunk's verdict reads` (the operator pass's only commit; basis `git log --format='%h %s' c04e332..HEAD`).

## Changes (structured — detectors read this)
- **Files:** `Cargo.toml` · `Cargo.lock` · `src/main.rs` · `src/cmd/run.rs` · new `src/human.rs` · new
  `tests/cli_output_plain.rs` (basis: `gate.py scope` over base c04e3324, `changed 6 · listed 6`).
- **Symbols / APIs:**
  - new `src/human.rs`: `pub(crate) fn write_refusal(out: &mut impl Write, unable: &str, hint: &str) -> io::Result<()>`
    writes `unable: {unable}\nhint: {hint}\n` as ONE `write_all`; `pub(crate) fn refuse(unable: &str, hint: &str)` locks
    `io::stderr()`, calls `write_refusal` and drops the result (a closed pipe is swallowed, never a panic).
  - removed: the private `fn refuse` in `src/cmd/run.rs` (was run.rs:360-364, two `writeln!`). Its five callers, all
    in `src/cmd/run.rs` and all `run` start refusals, now call `human::refuse` through `use crate::{human, obs, run}`:
    `refuse_batch_script`, `refuse_live`, `refuse_stale`, `refuse_tampered_pin`, `refuse_squatted` (basis: the phase's
    rust-plane `calls` query, 5 rows; `grep -rlE 'human::' src` → `src/cmd/run.rs` only). The refusal TEXT is unchanged.
  - `src/human.rs` is the root bin's only human-stderr writer and its only caller is `run` (hook and obs never call it).
  - No IPC method, endpoint, port, socket, event kind or env var added. No CLI verb or flag added.
- **Crates / modules:** root bin gains module `human` (`mod human;` in `src/main.rs`). No crate added or removed.
- **Dependencies:** clap stays `=4.6.7`, now `default-features = false` with `std`, `derive`, `help`, `usage`,
  `error-context`, `suggestions` listed (its default set minus `color`; basis: clap-4.6.7 `Cargo.toml` `[features]
  default`, read from the registry). `Cargo.lock` lost 8 packages and gained 0: `anstream`, `anstyle-parse`,
  `anstyle-query`, `anstyle-wincon`, `colorchoice`, `is_terminal_polyfill`, `once_cell_polyfill`, `utf8parse`
  (basis: `git diff c04e332 -- Cargo.lock | grep -E '^[-+]name = '`: 8 `-`, 0 `+`). `anstyle` itself stays
  (clap_builder depends on it without the feature). No colour or terminal crate added.
- **Schema / config:** none — no config key, schema, or redaction shape. A dependency no longer reads `CLICOLOR`,
  `CLICOLOR_FORCE`, `NO_COLOR`, `TERM`, `COLORTERM` or `CI` (anstyle-query-1.1.5 `src/lib.rs:24-134`, research F1),
  because anstyle-query left the build.
- **Spec-master edits:** none (this chunk's phase and implement touched no master).
- **Counts / qualifiers moved:** the exhaustive qualifier "the only other variables any `viola` build reads are the two
  test seams" (architecture.md:586; security-plan.md:427 restates "environment variables are not a configuration
  channel") is TRUE again after this chunk: it was false at HEAD c04e332 through clap's `color` feature (see Spec claims
  disproved). The lockfile's package count fell by 8; no master states a lockfile package count (basis:
  `grep -cE 'anstream|anstyle|colorchoice|CLICOLOR'` over the seven masters: 0 hits each).
- **Dev-tool versions:** none — no host tool installed or changed (cargo-mutants, nextest, hyperfine, the WSL distro
  all unchanged; the distro was not re-provisioned).
- **Harness / gate surface:** one new `[[test]]` entry `cli_output_plain` (`required-features = ["fake-agent"]`, uses
  `tests/support`). No harness command, CI step or status shape changed.
- **Cross-project / external claims:**
  - CI ci#36404931982 on `c2dccf60c7db` (the pre-CI commit, the final HEAD before this wrap): verdict green, checks
    18/18, wall 285 s (read through `ci.py conclusion --sha HEAD --wait 5400`; verified by the overseer).
  - CI's mutation union on that run: `mutants (ubuntu-latest)` caught `src/cmd/run.rs:373:5 replace refuse_stale
    with ()`, missed `refuse_batch_script`; `mutants (windows-2025)` caught `refuse_batch_script`, missed
    `refuse_stale`; `mutants-verdict` `{"ok":true,"breaches":[]}` (job logs 108871272680 / 108871272499 / 108872675418).
  - `cli_output_plain`'s four cases PASS in `test (ubuntu-latest)`, `test (windows-2025)`, `test (macos-latest)`.
  - clap-4.6.7 and anstyle-query-1.1.5 registry sources (feature table; env reads).
- **Reverted / negative API facts:** none shipped. The step-6 guard runs neutralised two sites temporarily (`"color"`
  back in the clap features; `write_refusal` as two `writeln!`) and restored them byte-exact (evidence/guards.md).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. architecture.md:586 (Cross-cutting Patterns → Config management) and security-plan.md:427 (Secret Management,
     restating it) assert the only env vars outside `VIOLA_*` any `viola` build reads are the two test seams. At
     HEAD c04e332 it was false: clap's default `color` feature pulled anstream → anstyle-query, which reads
     `CLICOLOR`, `CLICOLOR_FORCE`, `NO_COLOR`, `TERM`, `COLORTERM`, `CI`; `CLICOLOR_FORCE=1 viola --help` into a pipe
     emitted 25 ESC bytes (research F1). This chunk removes the route, so the sentence is true again; the claim's
     disposition is an amendment naming the closed dependency route (Expected amendment 2).
  2. plan.md Test Commands entry `bash scripts/agent-run.sh run --mutants --file src/cmd/run.rs` expects exit 0 (0
     missed) from a Windows-host scoped run. Measured false by construction: the diff regenerates `refuse_stale`'s
     mutant, whose only killer `tests/cli_instance_state.rs::run_refuses_a_stale_name` is `#[cfg(unix)]` (SIGSTOP)
     — 5 tested, 4 caught, 1 missed (archive 169). The union of both legs kills it (local pre-push and CI). Operator
     word (implement, overseer): "Surface; union is verdict … Record gate 17 as a plan-entry premise gap, have the
     wrap reconcile the entry, and confirm the union in CI." (evidence/gate-17-surfaced.md). A plan artifact, not a
     spec master.
- **Expected amendments (from plan):**
  - architecture §Stack and Technologies — clap 4.6.7 without `color`: **carried** (Dependencies bullet). Sites:
    `grep -n clap architecture.md` → 6 hits (:16 stack table "clap 4.6.7 (derive)", :140, :148, :456, :476, :626).
  - architecture §Cross-cutting Patterns → Config management — the "only other variables" sentence holds again, the
    clap `color` route named closed: **carried** (Counts / qualifiers + Spec claims disproved 1). Sites: `grep -n
    'only other variables'` → architecture.md:586 (1 hit; 0 elsewhere); security-plan.md:427 restates the channel rule
    (`grep -n FAKE_AGENT_PUMP_DELAY_MS security-plan.md` → 5 hits: :232, :427, :688, :690, :693).
  - architecture §Infrastructure Patterns → Build system / Project directory structure — `src/human.rs` as the root
    bin's human-output module and its refusal writer as the only human-stderr site: **carried** (Symbols / Crates
    bullets). Sites: `grep -cE 'human\.rs|human-output'` → 0 in all seven masters; the tree lives at
    architecture.md:458 (`**Project directory structure**`), `src/main.rs` at :476.
  - design-system §Surface: cli → Toolkit — clap without `color`, every human byte's styling viola's own:
    **carried** (Dependencies). Sites: `grep -n clap design-system.md` → 3 hits (:686 Toolkit "clap 4.6.7 (derive)",
    :810, :915).
  - test-plan §5 CLI — the plain-output witness `tests/cli_output_plain.rs`: **carried** (Coverage below). Sites:
    `grep -c cli_output_plain` → 0 in all masters; test-plan §5 at :908, root `tests/cli_*.rs` rows at :943-963.
- **Coverage of new surfaces:**
  - `viola --help` / clap usage errors (now plain) → validation n/a · instrumentation n/a (no new event; obs-plan §11) ·
    PII n/a · tests integ (`tests/cli_output_plain.rs`: forced colour into a pipe ×2, help on a terminal; 3 OSes in CI)
    · a11y ✓ (no colour-only meaning, no SGR bold/underline on a terminal; a11y-plan §6 CLI equivalent) · tokens n/a
    (no SGR emitted)
  - `human::write_refusal` / `human::refuse` (run's start-refusal stderr) → validation n/a (fixed text, no upstream
    input) · instrumentation n/a (the refusal's `process-exit` line is `refused()`'s, unchanged) · PII n/a (no path, no
    pid; the instance name only) · tests unit (3 inline cases incl. one-write and error return) + integ
    (`cli_instance_state` LIVE/SQUATTED/TAMPERED/stale constants, `cli_program_resolution` REFUSAL) · a11y ✓ (`hint:`
    last, a11y-plan §8) · tokens n/a

## Deviations from intent
- Plan step 3 says the callers call `crate::human::refuse`; they call `human::refuse` through `use crate::{human,
  obs, run}` — `run.rs`'s own idiom for `obs::` / `run::`. The `grep -rlE 'human::' src` entry reads the same.
- Plan step 5's `help_is_plain_on_a_terminal` runs "with no colour variable set": `tests/support/outer_pty.rs`'s
  `OuterPty::spawn` exposes no env removal (`SpawnSpec.env_remove` is built empty) and tests/support is outside the
  research lists, so the case inherits the runner's environment. It still reads red with `color` restored (the
  terminal case failed on Windows ConPTY with `[0, 1, 4, 22, 24, …]`, evidence/guards.md).
- `tests/cli_output_plain.rs` carries a fourth case beyond the plan's three, `sgr_attributes_reads_bold_and_underline_but_not_colour_values`
  — a literal self-check of the test's own SGR parser (in-file, in-intent).
- The lockfile lost 8 packages where the plan named 4 — the other 4 are the ones "only they pulled in" (research
  Files to modify).
- Gate `run --mutants --file src/cmd/run.rs` red, surfaced with the operator's word (Spec claims disproved 2).
- scope record: none — `gate.py scope` clean (`scope: clean — changed 6 · listed 6 · recorded 0 (companion 0 ·
  mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 45`), 0 recorded.

## Decisions & corrections
- Operator (overseer) at implement P2: gate 17's red is a plan-entry premise gap; the union of both mutation legs is
  the chunk's mutation verdict (since chunk 7); the wrap reconciles the entry; CI confirms the union (confirmed).
- Operator at implement P4: drive the operator pass (pre-push → hygiene → pre-CI commit → guarded push → `ci.py
  conclusion`), reporting the final hygiene verdict line verbatim.
- Operator at implement start: a transient Bash-classifier outage (no verdict) is waited out and retried, never
  worked around.
- Learning candidate: a scoped `run --mutants --file` on the Windows host misses a cross-platform function whose only
  killing test is `#[cfg(unix)]` (and the Linux leg misses one whose killer is `#[cfg(windows)]`, e.g. the `.cmd`/`.bat`
  refusal) — the union, not the scoped inner loop, is the verdict; a plan entry expecting 0 missed from a scoped
  host run needs every regenerated mutant's killer to compile on that host.
- Learning candidate: a ConPTY-hosted `--help` under clap `color` emits bold/underline SGR (`1`/`4`), so an SGR-parameter
  parse is a valid Windows red oracle — the literal-absence oracle need not be a text search.
- Sweep hazard: `grep -rlE 'human::' src` matches the `use crate::{human, …}` form only through the call sites
  (`human::refuse(`), not the `use` line — the entry's `last line src/cmd/run.rs` depends on the call-site form.

## Outcome
- **Acceptance criteria (re-asserted against the diff):**
  - (design) plain `--help` / usage error under `CLICOLOR_FORCE` into a pipe, no bold/underline on a terminal, three
    OSes — **met**: `cli_output_plain` green locally (archive 162), red with `color` restored (archive 161), PASS on
    ubuntu/windows/macos in ci#36404931982.
  - (arch/security) the four colour-stack packages absent, none gained, deny + deny-probes green, no colour/terminal
    crate — **met**: lockfile probe `exit 1 · last line 0`; 8 removed / 0 added; `cargo deny check` and
    `deny-probes.sh` green.
  - (design/a11y) every start refusal exactly two lines, hint last, one write — **met**: integration green
    (`cli_instance_state`, `cli_program_resolution`), `write_refusal_writes_the_pair_in_one_write` green and red at
    two writes (archives 163/164); the real smoke printed `unable: builder is already live\nhint: viola list\n`, 49 B,
    exit 1, empty stdout.
  - (obs/security) both clippy forms, no `#[allow]`, no print macro, error returned without panic, `hook_fail_open`
    green, `human::` only from run — **met** (`grep -cE 'print!|println!|eprint|#\[allow' src/human.rs` → 0).
  - (tests) unit/integration/G2/G4/secret-scan green; scoped mutants over `src/human.rs` and `src/cmd/run.rs` 0 missed;
    pre-push union green; coverage floors — **met except one clause**: `src/human.rs` scoped green; `src/cmd/run.rs`
    scoped **UNMET on the Windows host by construction** (1 missed, `refuse_stale`, killer `#[cfg(unix)]`), killed in
    the union (local pre-push ×2 and CI `mutants-verdict` breaches []). Unlinked to the matrix → P2 escalation, with
    the operator's word already recorded.
  - (ops) CI on the pushed HEAD green — **met**: ci#36404931982 on c2dccf6, 18/18.
- **Gates** (implement run 2026-09-28T09-07-16; operator pass after it):
  - `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
    green · `cargo clippy --workspace --all-targets -- -D warnings` green · `bash scripts/lint-probes.sh` green ·
    `bash scripts/orphans-check.sh` green
  - `grep -cE '^name = "(anstream|anstyle-query|anstyle-wincon|colorchoice)"$' Cargo.lock` green (exit 1 · last line 0)
  - `cargo deny check` green · `bash scripts/deny-probes.sh` green · `grep -rlE 'human::' src` green (last line
    src/cmd/run.rs)
  - `bash scripts/agent-run.sh run --unit` green · `run --integration --filter 'binary(=cli_output_plain)'` green ·
    `AGENT_RUN_KEEP_HOMES=1 … run --integration` green (artifact fresh) · `g2-zero-panics.sh` green (g2: clean) ·
    `schema-check` green · `secret-scan` green
  - `run --mutants --file src/human.rs` green (scoped)
  - `run --mutants --file src/cmd/run.rs` **red · exit 0** (exit 1; `"verdict":"scoped"` held) — surfaced; operator word:
    the union is the verdict (evidence/gate-17-surfaced.md)
  - smoke `cleanup` → `boot` → `status` → `cleanup` green (processes_gone, endpoint_gone)
  - `bash scripts/release-check.sh` green (release-check: viola only) · MSRV `RUSTUP_TOOLCHAIN=1.96 … cargo check`
    green · `bash scripts/agent-run.sh pre-push` green (union)
  - operator leg entries (never fired by the tool; recorded in evidence/operator-pass.md): `gate.py hygiene` green
    (final `hygiene: clean — read 37 (runs 29 · evidence 8) · trails 12 not read · binary 0 not read by P1`) ·
    `bash scripts/agent-run.sh pre-push` green (ok:true, stage union, legs counted, breaches []) · the guarded push
    exit 0 (`c04e332..c2dccf6`) · `ci.py conclusion --sha HEAD --wait 5400` green (`c2dccf60c7db verdict: green ·
    checks 18/18 · wall 285 s · runs ci#36404931982 completed/success`)
  - smoke (hand-driven, P3): boot ready, status ready, a second `viola run builder` on the live name refused
    byte-exact, cleanup processes_gone + endpoint_gone.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran — its commit list is `c2dccf6` alone (no fix commits); the final HEAD's CI
  run is ci#36404931982, recorded in `evidence/operator-pass.md`; implement's P4 report (this conversation) for the
  deviations, the guard readings and the census.
- **Process hygiene:** implement's census — sessions `p-cot-smoke`, `p-cot-hand` terminated (processes_gone; pids
  43800/35988 gone); the pre-push WSL VM terminated (`vm.terminated: true`); cargo/nextest/cargo-mutants terminated.
  Re-measured at this wrap: no process from this repository's `target/` is running; 3 `viola.exe` from
  `additional/viola-lab/prototype/target/debug/` run (not this chunk's; the count moves between 3 and 4 across
  readings, operator-owned).
