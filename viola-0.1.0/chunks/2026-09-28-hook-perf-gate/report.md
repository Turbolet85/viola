# Report — 2026-09-28-hook-perf-gate

**Chunk:** Hook perf gate — hyperfine-gated viola hook deadlines per OS in CI via run --perf and gate --require perf, the
ratified fake-agent FAKE_AGENT_HOOK_PANIC seam with its forced-panic fail-open case on the real binary, D-28 over-4 KiB
concurrent detail line
**Date:** 2026-09-28T07:40Z
**Commits:** `5a693d6 chore(2026-09-28-hook-perf-gate): operator pre-CI commit, for the run this chunk's verdict reads`
(`git log --format='%h %s' 85ae5aa..HEAD`; its parent `85ae5aa` is the basis for every "parent commit" below). The only
later change is `evidence/operator-pass.md` (its §31–§32 sections), still uncommitted.

## Changes (structured — detectors read this)
- **Files** (`git diff --name-only 85ae5aa` = 56 paths; source, test, script and CI paths listed, run dirs and chunk docs
  omitted — `gate.py scope`: changed 12 · listed 11 · recorded 1):
  - `src/cmd/hook.rs`, `src/cmd/hook/seam.rs` (new)
  - `tests/hook_fail_open.rs`, `tests/cli_controls_not_disableable.rs` (new)
  - `crates/viola-e2e/src/harness/{run.rs,gate.rs}`, `crates/viola-e2e/src/harness/run/perf.rs` (new),
    `crates/viola-e2e/src/bin/viola-harness.rs`, `crates/viola-e2e/tests/cli.rs`
  - `scripts/g2-zero-panics.sh` (new)
  - `.github/workflows/ci.yml`
  - `Cargo.toml` (one `[[test]]` entry; `Cargo.lock` unchanged — no dependency moved)
- **Symbols / APIs:**
  - **Test seam `FAKE_AGENT_HOOK_PANIC`** (new env var read). `src/cmd/hook/seam.rs`, declared `mod seam;` in
    `src/cmd/hook.rs` with NO cfg attribute; both fns carry `#[cfg(feature = "fake-agent")]`:
    `pub(super) fn panic_if_asked()` reads `std::env::var_os("FAKE_AGENT_HOOK_PANIC")` and hands it to `fn panic_on(Option<OsString>)`,
    which panics only when the value is exactly `1` (`0`, `false`, `off`, empty, absent, `2` return). The panic payload
    is the fixed ASCII text `"forced-hook-panic ".repeat(256)` (4 608 B), carrying no upstream text, path or secret. The
    variable is named in product source in that one file only (gate `grep -rl 'FAKE_AGENT_HOOK_PANIC' src crates` → one
    line, `src/cmd/hook/seam.rs`). It configures nothing, disables no control, widens no redaction, and is compiled out
    of every build without `fake-agent` (default-feature clippy clean; `release-check.sh` → `viola only`; probe 5/5).
  - **`hook()`** (`src/cmd/hook.rs`, sole caller `cmd::dispatch()`, signature unchanged) calls `seam::panic_if_asked()`
    under `#[cfg(feature = "fake-agent")]` immediately after `viola_obs_init` and before the stdin lock and `handle`, so a
    forced panic finds `PANIC_SINK` holding the instance: one codes-only `event:"panic"` role line in
    `diagnostics/hook-<name>.ndjson` (no payload, no backtrace) and one `detail-hook.ndjson` line with `panic_payload` +
    `backtrace` array over 4 KiB; exit 0 (`exit_code(Role::Hook, _)`), empty stdout and stderr. No `hook-invoked` /
    `hook-decision` line precedes it. The panic path itself (`viola_panic_hook`, `write_panic_lines`) is unchanged.
  - **Harness `run --perf`** (new named-only selector: `Selection::perf`, neither the default nor `--all`, like
    `--browser` / `--fuzz-replay`; `RunArgs --perf` in `viola-harness.rs`, the one exhaustive `Selection` literal sets it;
    the other 12 `Selection { … }` literals spread `..` and are unchanged). Arm `crates/viola-e2e/src/harness/run/perf.rs`,
    run from `tool_arms` between fuzz replay and mutants, skipped after an earlier refusal:
    1. probe `hyperfine --version` through the `Runner` seam; non-zero → refusal `reason:"tool-missing"`,
       `detail:"hyperfine"`;
    2. `cargo build --release --workspace --features viola/fake-agent --target-dir target/perf` through the runner; failure
       → suite red naming `build`;
    3. boot one session `perf-<harness pid>`, instance `builder`, via `BootOptions { bin_dir: target/perf/release,
       build: false }` through the new `PerfSession` trait (`Live` impl = `boot::boot` / `cleanup::cleanup`; harness unit
       tests use a stand-in); failure → suite red naming `boot: <reason>`;
    4. write one synthetic payload per row (string fields only, the tests-owned content canary in content fields) to
       `target/agent-run/<session>/payload-<hook>.json`, removing each row's stale export first;
    5. per row `hyperfine -N --warmup 3 --runs 30 --input target/agent-run/<session>/payload-<hook>.json --export-json
       target/agent-run/artifacts/perf-<hook>.json "target/perf/release/viola[.exe] hook <hook>"`, cwd the repo root,
       `VIOLA_NAME=builder` and `VIOLA_DIR=<perf home>/instances/builder` on the hyperfine process; a non-zero exit fails
       that row (rows `session-start`, `user-prompt-submit`, `stop`, `session-end` — `pre-tool-use` is an unknown event
       at HEAD and is not timed; no async-tier row);
    6. zero-panic check: any `event:"panic"` line in the perf home's `diagnostics/*.ndjson` minus `detail-*` fails `panic`
       (no seam exemption);
    7. cleanup through the seam with `keep_homes` = `AGENT_RUN_KEEP_HOMES` exactly `1`; a non-ok document fails `cleanup`.
    Suite `perf`: `passed`/`failed` count the 4 rows + the panic check + cleanup (6 on green), `failures` names each,
    `artifact:"target/agent-run/artifacts"`. The deadline verdict stays with `gate`.
  - **`gate --require perf`** (`gate.rs` `perf()`, sole caller `gate()`): the four row files `perf-session-start.json`,
    `perf-user-prompt-submit.json`, `perf-stop.json`, `perf-session-end.json` are required by name (each absent one →
    `artifact-missing` naming it; the old single `perf-*.json` absence breach is gone); every present `perf-*.json` is
    still judged `results[0].max < 1.0` (`SPINE_DEADLINE_S`, unchanged), an unreadable one a breach. The row list is one
    const, `run::perf::ROWS`, re-exported as `run::PERF_ROWS` and read by `gate.rs` only.
  - **`scripts/g2-zero-panics.sh`** (new, fail-closed bash, `jq` required → `tool-missing: jq` exit 1): over
    `target/e2e-home`, empty scope → exit 1; counts `event:"panic"` lines in every `diagnostics/*.ndjson` except
    `detail-*`, exempting ONLY a line whose `panic_location` is a string of the form `<file>:<digits>` whose file part
    equals the string `src/cmd/hook/seam.rs` exactly; prints `g2: clean` (exit 0) or `g2: <n> panic line(s)` (exit 1).
    `--probe` builds `target/g2-probe/` (removed after) and requires: seam line alone → clean; `src/cmd/hook.rs:9` → red;
    `x/src/cmd/hook/seam.rs:3`, `src/cmd/hook/seam.rs.bak:3`, `src/cmd/hook/seam.rsx:3` → red; the seam path with no
    `:<line>` → red; no `panic_location` → red; a panic only in a `detail-*` file → clean; empty scope → red; prints
    `g2-probe: all cases as expected`. The seam path is one variable in the script.
- **Crates / modules:** no crate added or removed. New modules: `viola` bin `cmd::hook::seam`; `viola-e2e`
  `harness::run::perf`.
- **Dependencies:** none (no `Cargo.toml` dependency line, no `Cargo.lock` change).
- **Schema / config:** none — no new `ObsEvent` (the existing `panic` event, `diag-line.v1.json` / `diag-detail.v1.json`
  unchanged; G4 green over the kept homes holding the new panic lines), no config key, no snapshot or event-line shape.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - CI jobs / check-runs: 8 jobs / 15 check-runs → 9 jobs / 18 check-runs (basis: `ci.py conclusion` on `5a693d6` →
    `checks 18/18`; the new `perf` job × 3 OSes). Stated at `architecture.md:556` (1 hit, `\b8 jobs\b|15 check-runs`).
  - `tests/hook_fail_open.rs`: was the 11-case matrix + the hidden-help test + the 8-process concurrent check, no forced
    panic; now also `hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line` and
    `hook_panics_append_whole_lines_over_4_kib_side_by_side` (8 processes). "As landed it holds 11 cases and no forced
    panic" is stated at `test-plan.md:1254` (basis: the file at `5a693d6`).
  - Test-seam env vars read by a `fake-agent` `viola`: one (`FAKE_AGENT_PUMP_DELAY_MS`) → two (+ `FAKE_AGENT_HOOK_PANIC`).
    "One"/"the one"/"one exception"/"one carve-out" wording at `architecture.md:107,172,581` · `security-plan.md:232
    (table row set),425,583,688` (pattern `FAKE_AGENT_PUMP_DELAY_MS|FAKE_AGENT_HOOK_PANIC`: architecture 4 lines — 107,
    172, 374, 581 · security-plan 6 — 232, 425, 583, 686, 688, 691 · test-plan 3 — 927, 1254, 1897; the 374 / 686–691 /
    927 / 1897 hits describe the pump seam itself and stay true).
- **Dev-tool versions:** hyperfine (the perf rows' timing runner, test-plan §10 "hyperfine"), dev host (Windows): absent →
  `hyperfine 1.20.0` via `cargo install --locked hyperfine@1.20.0`, installed 2026-09-28T05:45Z
  (`evidence/hyperfine-host.md`); CI's new `perf` job installs the same pin (`cargo install --locked hyperfine@1.20.0`,
  a step of its own — NOT the taiki-e `tool:` line, so `scripts/wsl-provision.sh` replays nothing new). No lockfile
  resolves it.
- **Harness / gate surface:**
  - `run --perf` (named-only) and suite `perf` in `run-summary.json` — the first writer of the closed suite value `perf`.
  - `gate --require perf` requires the four named rows.
  - `scripts/g2-zero-panics.sh [--probe]`, called as `bash scripts/g2-zero-panics.sh --probe && bash
    scripts/g2-zero-panics.sh` by the `test` job's "G2 zero panics" step (`if: always()`, `shell: bash`, same position in
    the obs-plan §9 order; replaces the inline `find … | jq` step) and by the new `perf` job.
  - New CI job `perf (${{ matrix.os }})`, `[windows-2025, macos-latest, ubuntu-latest]`, `fail-fast: false`, job
    `permissions: contents: read`, env `AGENT_RUN_KEEP_HOMES: "1"`; steps: checkout (pinned SHA, `persist-credentials:
    false`) · toolchain · rust-cache (pinned SHA) · `cargo install --locked hyperfine@1.20.0` · `jq --version` · `bash
    scripts/agent-run.sh run --perf` · G2 (probe then check, `if: always()`) · `schema-check` (`if: always()`) ·
    `secret-scan` (`id: secret-scan`, `if: always()`) · upload `perf-${{ matrix.os }}`
    (`target/agent-run/artifacts/perf-*.json`) and `diag-perf-${{ matrix.os }}` (`target/e2e-home/**/diagnostics/*.ndjson`)
    only when `always() && steps.secret-scan.outcome == 'success'` · upload `secret-scan-perf-${{ matrix.os }}` on a scan
    failure · `Gate verdict` `if: always()`: `bash scripts/agent-run.sh gate --require perf`. No new action, no
    `github.event` value; zizmor clean.
  - `test`, the pre-push and the WSL provisioning are untouched (operator P4 fork 1).
- **Cross-project / external claims:** CI run `ci#36390764600` (push) on `5a693d6`: `verdict: green · checks 18/18 · wall
  1067 s · completed/success` (`ci.py conclusion --sha HEAD --wait 5400`; also verified by the overseer). Per job
  (`gh run view 36390764600`): `perf` on all three OSes success with `Perf rows`, `G2 zero panics` and `Gate verdict`
  success; `test` on all three OSes success with `G2 zero panics` and `Gate verdict` success. The verdict was taken on
  `5a693d6`; this wrap's commit adds only docs, run dirs and the evidence file on top.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `test-plan.md:1254` "As landed it holds 11 cases and no forced panic; … The forced-panic case on the real binary lands
    with the 'Hook perf gate' chunk" — now landed (measured: the two new `hook_fail_open` tests pass at implement gate 11
    and in CI `test` on 3 OSes; the forced-panic case took 0.414 s on the Windows debug build, under the 1 s bound).
  - `architecture.md:407` "Its hyperfine exports are the separate `perf/*.json` (obs-plan §9)" and obs-plan's
    `perf/hook-<event>.json` (`obs-plan.md:1244,1320,1321`, pattern `perf/\*\.json|perf/hook-|perf-\*\.json|perf-<hook>|target/perf`) —
    the built exports are `target/agent-run/artifacts/perf-<hook>.json`, the path `gate.rs` already read (plan's lean;
    measured: gate 15 wrote them, gate 16 read them). `architecture.md:407`'s command also reads `--features fake-agent`;
    the built form is `--features viola/fake-agent` (see Deviations).
- **Expected amendments (from plan):** (sites located by `.andromeda/runs/2026-09-28T07-37-52-wrap/sites.py`, patterns
  and per-master hit lines in its output; each carried line below names its Changes bullet)
  1. security-plan §Security Decisions Log — the `FAKE_AGENT_HOOK_PANIC` entry, ratified by the founder live 2026-09-28
     06:21, relay the Viola overseer — carried: Symbols/APIs "Test seam". Site: the Decisions Log (`security-plan.md:686`
     neighbourhood, the `2026-09-27` seam entry is the precedent; pattern seam-env, 6 hits).
  2. security-plan §Input Validation — a second test-seam row — carried: Symbols/APIs "Test seam"; site
     `security-plan.md:232` (the one seam row).
  3. security-plan §Security Anti-Patterns → Universal and §Secret Management → Storage — "the one seam" becomes two —
     carried: Counts "Test-seam env vars"; sites `security-plan.md:583` (Universal carve-out) and `:425` (Storage).
  4. security-plan §Dependency Security → CI integration — the `perf` job and its scanned uploads — carried: Harness/gate
     surface "New CI job"; site located by the obs-plan/security CI sections (pattern perf-upload: 0 security-plan hits,
     so the detector locates the CI-integration paragraph by section).
  5. architecture §Conventions "Environment variables" (`architecture.md:172`), §Established Decisions [Naming] (`:107`),
     §Occupied Resources (Environment variables → Test seam `:374` neighbourhood; `target/perf/` exports `:407`;
     `scripts/g2-zero-panics.sh`; `target/g2-probe/`; `src/cmd/hook/seam.rs`; the `perf-<pid>` harness session), and
     §Cross-cutting Patterns "Config management" (`:581`) — carried: Symbols/APIs "Test seam", "run --perf",
     "g2-zero-panics.sh"; Counts "Test-seam env vars".
  6. architecture §Infrastructure Patterns "CI/CD approach" — 9 jobs / 18 check-runs, the `perf` job — carried: Counts
     "CI jobs / check-runs"; site `architecture.md:556` (1 hit).
  7. test-plan §3 — `run --perf` built; `gate` requires the four named rows — carried: Harness/gate surface; sites
     pattern run-perf, 12 test-plan hits (435, 514, 519, 533, 541, 559, 798, 1448, 1515–1517, 1521).
  8. test-plan §5 — the Vector 6 table's interim shape (hook-path controls; verb negatives and completeness case pending)
     — carried: Symbols/APIs is silent on it, so stated here: `tests/cli_controls_not_disableable.rs` holds one rstest
     table, rows `FAKE_AGENT_HOOK_PANIC` = `0` · `false` · `off` · empty, each × {oversize stdin → `hook-decision
     {detail:"oversize-stdin"}`, malformed JSON → `detail:"malformed-json"`}, exit 0, empty streams, no panic line in the
     home; the four verb negatives and the completeness case join when `send` / `answer` land. Sites pattern vector6,
     test-plan 6 hits (336, 942, 943, 960, 963, 1254).
  9. test-plan §6 Security sweep — the forced panic landed — carried: Spec claims disproved (`test-plan.md:1254`);
     pattern forced-panic, test-plan 5 hits (331, 873, 924, 1254, 1266).
  10. test-plan §9 Perf row — as built, own per-OS job — carried: Harness/gate surface "New CI job"; pattern hyperfine,
      test-plan 22 hits (the §9 row among 1448–1454).
  11. test-plan §10 — the rows exist; `pre-tool-use` pending the dialog-tier chunk; the async tier untimed — carried:
      Symbols/APIs "run --perf" step 5; pattern pre-tool-use-row, test-plan 5 hits (706, 1084, 1091, 1517, 1527); §10
      rows at 1513–1527.
  12. obs-plan §9 — perf in its own job, not inside `test`; G2's exact-path seam exemption with its probe; the
      `perf-<os>` / `diag-perf-<os>` uploads — carried: Harness/gate surface; pattern g2, obs-plan 22 hits (§9 among
      1244–1330); pattern perf-upload, obs-plan 2 hits (1244, 1750).
  13. obs-plan §10 — status flipped to built — carried: Harness/gate surface "run --perf"; obs-plan §10 around 1316–1330
      (pattern run-perf, 1 hit at 1316).
  14. obs-plan §3 D-28 — the over-4 KiB half landed — carried: Counts "tests/hook_fail_open.rs"; pattern d28, obs-plan 6
      hits (585, 621, 663, 1142, 1684, 1725), test-plan 2 (705, 1737).
  15. obs-plan §8 item 6 — `perf-<os>` inventoried as uploaded behind the scan — carried: Harness/gate surface "New CI
      job"; pattern perf-upload, obs-plan 2 hits (1244, 1750).
- **Coverage of new surfaces:**
  - `FAKE_AGENT_HOOK_PANIC` read (fake-agent builds only) → validation closed-value `== "1"`✓ · instrumentation the
    existing panic role + detail lines✓ · PII payload fixed synthetic text, detail file only, role line payload-free✓ ·
    tests unit (rstest 6 non-firing values + `should_panic`) + integ (forced-panic case, concurrent check, controls
    table) + remove-the-guard run (`evidence/seam-guard.md`)✓ · a11y n/a · tokens n/a
  - `run --perf` / suite `perf` → validation n/a (named flag, fixed argv, repo-relative paths) · instrumentation n/a
    (harness) · PII payloads carry only the tests-owned content canary, allowed in the harness capture (`secret_scan`
    CONTENT class)✓ · tests unit through stand-ins (`perf.rs` 11 tests) + live gate 15 + CI 3 OSes✓ · a11y n/a · tokens n/a
  - `gate --require perf` four rows → tests unit (zero, three, four rows, a `max` of 1.0, an unreadable file)✓
  - `scripts/g2-zero-panics.sh` → tests its `--probe` (9 cases) as a listed gate + CI both jobs✓
  - CI `perf` job → zizmor clean✓ · uploads gated on the secret scan✓

## Deviations from intent
- **Perf build feature spelling:** plan step 10.2 (and `architecture.md:407`) write `--features fake-agent` over
  `--workspace`; the arm uses `--features viola/fake-agent`, the spelling `boot.rs`'s `cargo_build` uses for the same
  workspace build. Justification: one form for the one workspace build; surfaced at implement P4.
- **G2 probe:** one case beyond the plan's list — the seam path without a `:<line>` suffix reads red (the exemption
  requires the `<file>:<digits>` form). Stricter than asked, never looser.
- **Gate test renamed:** `gate_perf_gates_the_max_sample_against_the_spine_deadline` →
  `gate_perf_requires_every_row_and_gates_the_max_sample_against_the_spine_deadline` (zero · three · four rows, a max of
  1.0, an unreadable row).
- **Seam test module attribute:** `#[cfg(test)]` and `#[cfg(feature = "fake-agent")]` stacked as two attributes, not
  `#[cfg(all(test, …))]` — cargo-mutants mutated a test fn inside the combined form (a MISSED mutant at implement).
- Scope record — `gate.py scope` (P1 of this wrap): `scope: clean — changed 12 · listed 11 · recorded 1 (companion 1 ·
  mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 44`:
  - companion: `crates/viola-e2e/tests/cli.rs` · serves `crates/viola-e2e/src/bin/viola-harness.rs` · self — its
    unbuilt-selector usage case named `run --perf`; now `run --e2e` (still unbuilt).

## Decisions & corrections
- Operator P4 forks (phase, with the overseer's notes): perf in its own per-OS job (test, pre-push, WSL untouched; CARRY 3
  does not fire and moves on); G2 exempts the seam's exact file path with a known-positive probe; the controls file lands
  now with the hook-path controls, test-plan §5 recording the interim shape.
- Operator pass driven by /implement on the operator's word ("run the operator pass now, entries 29-32, then fold any CI
  red into this chunk"); no red reached the pass. Hygiene refused the phase run's `dryrun.txt` / `dryrun2.txt` (absolute
  host paths); on the operator's word, widened with the overseer's ("my line-2-only was too narrow"), lines 2, 4, 35 and
  38 of each were rewritten to placeholders (`bash.exe` · `<OS temp>/…` · `{tools_dir}/…`), every other byte kept.
- Sweep / tooling hazards met this chunk:
  - cargo-mutants skips only a module marked exactly `#[cfg(test)]`; inside `#[cfg(all(test, feature = "…"))]` it mutates
    the test fns themselves (a `replace <test fn> with ()` MISSED in the scoped loop and both pre-push legs).
  - A harness CLI test pinning a not-yet-built selector as `usage` runs the real arm the moment the selector lands: the
    old `run --perf` case built, booted and ran hyperfine inside nextest and read exit 0.
  - On one tree, the host's scoped mutants loop graded two `perf.rs` mutants unviable that both pre-push legs graded
    missed / caught (the known run-to-run grading variance).
  - The Bash guard refuses a `cat >> file <<EOF` document write and any doubled backslash (already in host-win32.md).

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - `run --perf` writes the four `perf-<hook>.json` from hyperfine 1.20.0 `-N --warmup 3 --runs 30`, adds a green `perf`
    suite, and `gate --require perf` exits 0 on this host — MET (gates 15/16; host max samples 73.0 · 72.8 · 73.1 ·
    73.7 ms, `evidence/perf-argv.md`).
  - `gate --require perf` breaches `artifact-missing` per absent row, `perf` on `max >= 1.0` or unreadable; unit table
    covers zero, three, four rows — MET (`gate.rs` test).
  - `run --perf` refuses `tool-missing`/`hyperfine`; failed boot, hyperfine failure, planted panic line, non-ok cleanup
    each red and named, through stand-ins — MET (`perf.rs` tests).
  - CI `perf` job on three OSes with the pinned hyperfine, ending in `gate --require perf`, uploads behind a green scan,
    green on the pushed sha; zizmor passes — MET (ci#36390764600 on `5a693d6`; gate `zizmor .github/workflows/`).
  - The built `viola hook stop` with `FAKE_AGENT_HOOK_PANIC=1` exits 0 < 1 s, empty streams, one role panic line at
    `src/cmd/hook/seam.rs` without payload/backtrace, one detail line > 4 096 B with payload + backtrace, both
    schema-valid — MET (integration; CI `test` × 3).
  - N concurrent forced panics → N whole role lines and N whole detail lines > 4 096 B, in `test` on three OSes — MET.
  - `cli_controls_not_disableable.rs` seam row, `0`/`false`/`off`/empty, same verdicts, no panic line — MET.
  - Variable named in product source only in `seam.rs`, panic compiled only under `fake-agent`; release-check probe 5/5
    and `viola only`; default-feature clippy clean — MET.
  - `g2-zero-panics.sh` passes over kept homes holding forced-panic lines; `--probe` all cases; both CI jobs probe then
    check — MET.
  - Coverage (85/95/80 per OS) and the two-leg mutation union over the diff — MET (pre-push: Linux and Windows coverage
    gates ok, union `breaches: []`, legs counted 52/52, 0 missed; CI `test` gates green).
- Gates (the plan's `[[gate]]` entries by `run`, implement run `2026-09-28T05-42-35-implement`, final block):
  `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green ·
  `cargo clippy --workspace --all-targets -- -D warnings` green · `bash scripts/lint-probes.sh` green · `bash
  scripts/orphans-check.sh` green · `zizmor .github/workflows/` green · `hyperfine --version` green (`contains hyperfine
  1.20.0`) · `grep -rl 'FAKE_AGENT_HOOK_PANIC' src crates` green (`last line src/cmd/hook/seam.rs`) · `bash
  scripts/g2-zero-panics.sh --probe` green · `bash scripts/agent-run.sh run --unit` green · `AGENT_RUN_KEEP_HOMES=1 bash
  scripts/agent-run.sh run --integration` green · `bash scripts/g2-zero-panics.sh` green · `bash scripts/agent-run.sh
  schema-check` green · `bash scripts/agent-run.sh secret-scan` green · `bash scripts/agent-run.sh run --perf` green ·
  `bash scripts/agent-run.sh gate --require perf` green · the four `run --mutants --file …` scoped loops green
  (`"verdict":"scoped"`) · the smoke four (`cleanup` · `boot` · `status` · `cleanup` of `p-perf-smoke`) green · `bash
  scripts/release-check.sh --probe` green · `bash scripts/release-check.sh` green · `RUSTUP_TOOLCHAIN=1.96
  CARGO_TARGET_DIR=target/msrv cargo check --workspace --all-targets --features viola/fake-agent` green (Checking viola,
  viola-e2e on rustc 1.96.1) · `bash scripts/agent-run.sh pre-push` green. Operator legs (recorded in
  `evidence/operator-pass.md`): `gate.py hygiene` clean · `pre-push` green · the guarded push `85ae5aa..5a693d6` · `ci.py
  conclusion --sha HEAD --wait 5400` green (`verdict: green`). No `defer`, no deferral.
- Earlier reds, all this chunk's and fixed at implement: the viola-e2e `cli.rs` unbuilt-selector case (companion); a
  MISSED mutant on the seam's test fn (attribute split); a MISSED mutant `delete field artifact` in `perf.rs` (an
  `artifact` assertion added). Remove-the-guard: seam neutralised → both forced-panic tests red (2 vs 1 and 16 vs 8 role
  lines), restored → green (`evidence/seam-guard.md`).
- Smoke: boot-path changed (`viola hook`); `boot` ok · `status` `ready` · `cleanup` `processes_gone:true`,
  `endpoint_gone:true`; a kept-home `run --perf` read 132 `hook-decision` lines with no fallback `detail` and 33 wrapper
  events per row: every timed sample took the channel path, never the no-instance no-op or the unreachable fallback.
- Watches: none folded.
- Outcome basis: the operator pass ran — `5a693d6` is the final HEAD and its CI run `ci#36390764600` is green 18/18
  (`evidence/operator-pass.md`); implement's P4 report and gate trail (`.andromeda/runs/2026-09-28T05-42-35-implement/`)
  for what only it holds; the post-pass artifact is `evidence/operator-pass.md` §31–§32.
- Process hygiene: implement's census — none of this chunk's processes left running; the host list at this wrap holds
  only the `viola-lab` prototype's three `viola.exe` (not this chunk's) and the operator's `claude.exe` sessions; the
  pre-push's WSL VM `terminated:true`.
