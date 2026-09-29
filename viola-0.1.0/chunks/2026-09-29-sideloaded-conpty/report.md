# Report — 2026-09-29-sideloaded-conpty

**Chunk:** Sideloaded ConPTY — Microsoft NuGet conpty.dll and OpenConsole.exe beside the pinned viola.exe, version, hash and signature checked, loaded only from that path; H2 200-loop on windows-2025 with and without
**Date:** 2026-09-29T12:20Z
**Commits (since last_wrap 2026-09-29T08:10:56Z; basis `git log --format='%h %s' fb78ddc..HEAD`):**
- `2d83718` chore(2026-09-29-sideloaded-conpty): operator pre-CI commit, for the run this chunk's verdict reads
- `224efc4` fix(2026-09-29-sideloaded-conpty): the pump-window resize test resizes on the wrapper's claude-child start line
- `8f643f2` chore(2026-09-29-sideloaded-conpty): remove the H2 measurement

## Changes (structured — detectors read this)
- **Files** (basis `git diff --name-status fb78ddc -- . ':!.andromeda' ':!viola-0.1.0' ':!.claude'`, 27 paths):
  - added: `crates/viola-pty/src/sideload.rs` · `scripts/conpty-vendor.sh` · `src/conpty.rs` · `tests/conpty_sideload.rs` ·
    `tests/support/piped.rs` · `vendor/conpty/1.24.260710001/x64/conpty.dll` · `vendor/conpty/1.24.260710001/x64/OpenConsole.exe`
  - modified: `.gitattributes` · `.github/workflows/ci.yml` · `Cargo.toml` · `crates/viola-pty/src/lib.rs` ·
    `crates/viola-state/src/pin.rs` · `schemas/diag-line.v1.json` · `src/cmd/run.rs` · `src/main.rs` · `src/run/mod.rs` ·
    `tests/{cli_fake_agent,cli_instance_state,cli_program_resolution,cli_version_gate,contract_diag_schema,run_cli,tui_env_strip,tui_passthrough}.rs` ·
    `tests/support/{home,mod,outer_pty}.rs`
  - `crates/viola-pty/Cargo.toml`: the measurement-only `h2-measure` feature added at `2d83718`, removed at `8f643f2` —
    net unchanged against `fb78ddc` (so absent from the list above). `Cargo.lock`: unchanged (0 diff lines).
- **Symbols / APIs:**
  - `viola_pty::pty_backend() -> &'static str` replaces the const `viola_pty::PTY_BACKEND` at all 3 uses (lib.rs `pty.spawn`
    span field, `src/run/mod.rs` `log_child_start`, the `src/cmd/run.rs` span test); values `conpty-sideload` (after a
    successful preload) · `conpty` (Windows) · `openpty` (elsewhere). Entry `! grep -rn 'PTY_BACKEND' src crates tests`: 0 hits.
  - `pub mod viola_pty::sideload` (`cfg(windows)`): `restrict_dll_search() -> bool` (`SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)`,
    process-wide, one-way) · `search_restricted() -> bool` · `preload(&Path) -> Result<(), PtyError>` (`LoadLibraryExW` by absolute
    path, refuses a relative path, module never freed). The crate knows no pinned path.
  - `viola_pty::PtyError::Load(Box<dyn Error + Send + Sync>)`, fixed message `"pty load failed"` — a variant, no new enum.
  - `viola_state::pin` (`cfg(windows)`): `pub struct Companion { name, bytes, sha256_hex }` · `pub struct HeldCompanions { pub dir, _handles }` ·
    `pub fn pin_companions(&Pinned, subdir, &[Companion]) -> Result<HeldCompanions, PinError>` — write-if-absent through
    `replace_private_shared`, then each file opened with Win32 `FILE_SHARE_READ` only and re-hashed (full SHA-256); a mismatch →
    `PinError::HashMismatch`, the file left as found; `PinError` gains no variant. A held open refused with Win32 error 32
    (sharing violation) is retried up to `REPLACE_ATTEMPTS` × `REPLACE_PAUSE` (`retry_open`, `open_held_waiting`) — measured cause:
    two concurrent first starts, the refusal lasting 5-11 ms after the other start's write, 0 of 6 pairs over companions already
    written. `pin_exe` / `verify` unchanged (verify writes no companion). Private helper `digest()` now backs `file_key`.
  - Root bin: `src/conpty.rs` (`cfg(all(windows, target_arch = "x86_64"))`) — `FILES: &[Companion]` (OpenConsole.exe first, then
    conpty.dll; `include_bytes!` of the vendored files), `SUBDIR = "conpty"`, `DLL = "conpty.dll"`; the package version and the
    nupkg SHA-256 are text the vendor script parses (one textual home for all four pins).
  - `src/main.rs`: `viola_pty::sideload::restrict_dll_search()` is the second statement of `main` on Windows, for every role.
  - `src/cmd/run.rs`: new start step `conpty_sideload` (span `run.conpty_sideload`, fields `outcome` ∈ `loaded` · `hash-mismatch` ·
    `unreadable` · `load-failed` · `not-built`, `search_restricted` bool) between `pin_and_plugin` and the strip plan; the held
    handles live until `spawn_child` returns. `spawn_child` and `run::log_child_start` gain a `sideload_fallback: Option<&'static str>` parameter.
  - Remaining callers of `viola_pty::spawn` that are NOT restricted or preloaded (unchanged, out of scope per the plan): harness
    `supervise::spawn_in_pty`, root `tests/support/outer_pty.rs`, viola-pty's own tests.
- **Crates / modules:** added `viola-pty::sideload` (module) · root `conpty` (module). No crate added or removed.
- **Dependencies:** none added or bumped (`Cargo.lock` 0 diff lines). windows-sys features unchanged (`Win32_System_LibraryLoader`
  was already in the workspace list). Non-crate binaries added: Microsoft `conpty.dll` 109 920 B sha256 `39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8`
  and `OpenConsole.exe` 1 066 296 B sha256 `b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160`, from
  `Microsoft.Windows.Console.ConPTY` 1.24.260710001 (nupkg sha256 `175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e`, licence MIT),
  committed under `vendor/conpty/1.24.260710001/x64/` (`.gitattributes`: `vendor/conpty/** binary`; `git check-attr` reads text unset).
- **Schema / config:**
  - `schemas/diag-line.v1.json` `process-start` properties: new `sideload_fallback` `{ "enum": ["hash-mismatch", "unreadable", "load-failed", "not-built"] }`.
    `pty_backend` stays `{ "type": "string" }` (new value `conpty-sideload`). `diag-detail.v1.json` unchanged (carries no `process-start` field).
  - `process-start{subject:"claude-child"}` carries `pty_backend = pty_backend()` and, when the Windows outcome is not `loaded`, `sideload_fallback`.
  - New home files: `<home>/bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (write-if-absent, never overwritten).
  - No config key, env var or CLI flag added.
- **Spec-master edits:** none (implement authored none).
- **Counts / qualifiers moved:** the Windows `pty_backend` catalog gains one value (`conpty-sideload`); the start sequence gains one
  step (`run.conpty_sideload`); obs-plan has `pty_backend` 4 lines, `run.pin_copy` 3 lines (basis: `sites.py` regex per master, this
  run's scratch). The H2 inbox baseline 13/200 (ci#36527891850) gains a with/without pair: sideload 0/200, inbox 14/200.
- **Dev-tool versions:** none — no host tool installed or bumped. The vendor script uses the host's curl 8.18.0 (Schannel), unzip,
  sha256sum and `powershell.exe Get-AuthenticodeSignature` (read, unchanged).
- **Harness / gate surface:**
  - `scripts/conpty-vendor.sh` — default (re-vendor) · `--verify` (fetch outside the tree, byte-compare, signer check) · `--probe`
    (4 refusals: nupkg sha before extraction, per-file sha, unsigned file, TLS failure on an unreachable host; + control), summary line
    `conpty-vendor probe: <n>/<n> refused, control clean`. Pins parsed from `src/conpty.rs` text.
  - `ci.yml` `test` job: new `windows-2025`-only step `ConPTY vendor verification` (`--verify`, `--probe`) before the coverage step —
    stays. The `H2 loop (measurement only)` step was added at `2d83718` and removed at `8f643f2`; `ci.yml` against `fb78ddc` now
    differs by the vendor step only (entry `git diff fb78ddc -- .github/workflows/ci.yml | grep -c '^[-+].*tool:'` → 0).
  - Root tests: `Cargo.toml` `[[test]] conpty_sideload` (required-features `fake-agent`).
  - Shared test support: `tests/support/piped.rs` `Piped` (spawn with 3 piped streams · answers every DA1 `ESC[c` on stdout with
    `ESC[?1;0c` on stdin · `write` · `exited` · `stdout_eof` · `wait` · `finish` → `Output` · kills viola on drop if still running);
    `tests/support/home.rs` `seed_conpty(home)` (Windows x64: `<home>/bin/<key>/conpty/` hard-linked, copy fallback, from one per-run
    copy under `target/conpty-seed/<key>/`), called by `Wrapper::boot` and every piped / outer-PTY `viola run` start except those that
    must leave the home uncreated (`run_viola_unseeded`) and `tests/conpty_sideload.rs` (the product's write path); `OuterPty::spawn_in`
    (given cwd).
- **Cross-project / external claims:**
  - nuget.org `Microsoft.Windows.Console.ConPTY` 1.24.260710001 (research facts 1-3; re-verified on the runner:
    `conpty-vendor: verified 1.24.260710001`, `conpty-vendor probe: 4/4 refused, control clean`, ci#36563179341 on `2d83718`).
  - microsoft/terminal `winconpty.cpp` @ main (research facts 4-5), not the release tag.
  - CI runs this chunk's gates read: ci#36563179341 on `2d83718` → failure (14/15; `test (windows-2025)` coverage 974/975,
    `tui_host_resize_in_the_pump_start_window_reaches_the_child`) · ci#36563868040 on `224efc4` → success, 15/15, wall 1301 s (the H2
    measurement run) · **ci#36566391084 on `8f643f2` → success, 15/15, wall 320 s** (the final HEAD; also verified by the overseer).
- **Reverted / negative API facts:**
  - Concurrent companion pinning (one thread per file, `std::thread::scope`) was written and reverted to the serial loop: measured no
    gain (run_cli start median 6.11-6.41 s concurrent vs 6.21-6.38 s serial, 3 rounds each, local).
  - The `h2-measure` feature, `tests::h2_race{,_inbox,_sideload}` and the CI loop: measurement only, removed at `8f643f2` (entry
    `! grep -rnE 'h2-measure|h2_race|h2-loop' crates .github Cargo.toml` → no output).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - Plan implementation note (plan.md "Implementation notes", "The first start of a version writes 1 176 216 B … inside the 7 s
    `booted_wrapper` bound and the 1.0 s spine gate"): on this host under a parallel suite a first start's two companion writes cost
    ~1.2 s (`pin_companions` median 1193 ms; each `replace_private_shared` ~0.55 s: temp create 78 ms, `sync_all` 78 ms, rename 301 ms
    medians over 24 writes), which pushed first-start tests past their 7 s waits locally. CI's perf and test jobs are green
    (ci#36566391084); the tests now seed their homes (test-side), the product path is unchanged. Owner: a note for arch
    [Deployment / Distribution] / test-plan (first-start cost), no acceptance changed.
  - Plan implementation note ("If `OpenConsole.exe` emits a different spawn preamble, the zero-viola-literals case still holds by
    construction"): the preamble differs (inbox `ESC[?9001h ESC[?1004h`; sideload `ESC[1t ESC[c ESC[?1004h ESC[?9001h`) and it carries
    a DA1 query that holds the child's start ~3 s when unanswered (3.54 s vs 0.54 s answered vs 0.48 s pre-chunk; piped probe, this
    host) and makes the child's start wait for viola's pump (the answer travels through it). The zero-viola-literals half holds.
    Evidence `evidence/da1-stall.md`. **Product finding surfaced to the wrap (overseer), owner: the route entry that first runs
    viola headless.** Not fixed in the product: viola stays silent; the test-side piped driver answers DA1.
  - test-plan §Test data / testing.md ("Every test owns a fresh home: a not-yet-existing `home` … so viola creates it"): on Windows x64
    the seeded tests now pre-create `<home>/bin/<key>/conpty/` before viola runs (the home dir is created by the test; viola sets no
    DACL of its own on Windows — `viola-state` has no DACL write). Needs a test-plan carve-out or a ruling.
- **Expected amendments (from plan)** (site search: `sites.py` — one regex per anchor over the seven masters, hit counts per master):
  - architecture [PTY] (the sideload mechanism, the System32 restriction, the H2 counts beside 13/200) — **carried**: Symbols/APIs +
    Counts bullets. Sites: `\[PTY\]` architecture 1 (line 47); `\bH2\b` architecture 1; `13/200` 0 hits in all seven (the baseline is
    worded otherwise in the [PTY] line).
  - architecture [Session Liveness] (`run.conpty_sideload` after the pinned copy) — **carried**: Symbols/APIs (`src/cmd/run.rs`). Sites: architecture 2.
  - architecture [Deployment / Distribution] (embedded companions, write-if-absent) — **carried**: Symbols/APIs + Dependencies. Sites: architecture 1.
  - architecture §Occupied Resources → Filesystem `bin/<version>-<hash>/conpty/{…}` and → Repository `vendor/conpty/<version>/x64/` —
    **carried**: Schema/config + Dependencies. Sites: `Occupied Resources` architecture 3 · security-plan 12 · test-plan 3;
    `bin/<version>-<hash>|bin/<key>` architecture 3 · security-plan 13 · test-plan 1.
  - security-plan Decisions Log — the vendored MIT Microsoft binaries (founder live 2026-09-29 10:41:12, relay the Viola overseer) —
    **carried**: Dependencies. Sites: `Decisions Log` security-plan 33.
  - security-plan Decisions Log — the third dated interim gap (load from `bin/` under a held-handle re-hash before Epoch 6's
    owner/DACL check), same founder answer — **carried**: Symbols/APIs (`pin_companions`). Sites: as above.
  - security-plan §Input Validation + §Security Anti-Patterns → Universal — the DLL search-order planting vector and its control —
    **carried**: Symbols/APIs (`restrict_dll_search`, `main`). Sites: `Input Validation` security-plan 5; `Security Anti-Patterns`
    security-plan 9 · architecture 1; `LoadLibrary|DLL search` 0 hits in all seven (new text). Never the Threat Model Summary (verbatim copy).
  - security-plan §Data Protection → Code-bearing artefacts — the two companions join the trusted-file list — **carried**:
    Schema/config. Sites: `Code-bearing artefacts` security-plan 2.
  - obs-plan §6 — `pty_backend` gains `conpty-sideload`; additive `sideload_fallback` with its 4 codes; §4 Scenario 1 —
    `run.conpty_sideload` (`outcome`, `search_restricted`); §12 — D-36 — **carried**: Schema/config + Symbols/APIs. Sites:
    `pty_backend` obs-plan 4; `run.pin_copy` obs-plan 3; `D-35` obs-plan 1. §1 stays verbatim.
  - test-plan §5 Module ↔ PTY (the with/without counts and run id) and §9 (the `ConPTY vendor verification` step; the
    `conpty_sideload` binary) — **carried**: Counts + Harness/gate. Sites: `Module ↔ PTY` test-plan 4; `\bH2\b` test-plan 5 (line 927
    is the §5 row); `conpty` test-plan 12.
  - a11y-plan §3 — only if the sideloaded preamble differs: **carried** — it differs (Spec claims disproved, second bullet; the
    measured preamble in `evidence/da1-stall.md`). Sites: `conpty` a11y-plan 4; `preamble` 0 in a11y-plan (test-plan 3). §1 stays verbatim.
- **Coverage of new surfaces:**
  - `run.conpty_sideload` start step → validation SHA-256 pin per file ✓ · instrumentation span `run.conpty_sideload` {outcome,
    search_restricted} ✓ · PII n/a (codes only; `assert_nothing_logged` finds no path/host/hash in any home-level line) · tests
    unit + integ (`conpty_sideload` 5 cases, span-order test) · a11y n/a · tokens n/a
  - `process-start.sideload_fallback` field → validation closed enum in `diag-line.v1.json` ✓ · instrumentation ✓ · PII n/a · tests integ ✓ · a11y n/a · tokens n/a
  - DLL search restriction (`main`, every role) → validation n/a · instrumentation `search_restricted` on the span ✓ · PII n/a ·
    tests unit (`restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load`, two-sided) + integ (planted case); guard-removal run: red when removed · a11y n/a · tokens n/a
  - `<home>/bin/<key>/conpty/` companions → validation hash + held handles ✓ · instrumentation via the span outcome ✓ · PII n/a ·
    tests unit (6 pin cases incl. the held-writer denial) + integ (tamper cases) · a11y n/a · tokens n/a
  - `scripts/conpty-vendor.sh` (network fetch, dev/CI only) → validation nupkg sha before extraction + per-file sha + Authenticode
    signer ✓ · instrumentation n/a · PII n/a · tests `--probe` 4/4 + control (local and CI) · a11y n/a · tokens n/a
  - the sideloaded host's terminal preamble (a child-screen change, not a viola byte) → a11y `--` zero-viola-literals oracle holds on both
    backends (tampered case + existing a11y case) ✓ · DA1 behaviour surfaced (finding above)

## Deviations from intent
- **Companion pinning retries a sharing violation** (not in plan step 5): justified by measurement (two concurrent first starts → the
  second fell back `unreadable`, Win32 error 32, lasting 5-11 ms; 0 of 6 pairs over pre-written companions). Bounded like `fs.rs`'s
  existing replace retry; still fails open. Unit tests force the window with a test-held writer released by the injected pause.
- **Unit tests use `tempfile::tempdir()` homes**, not `target/e2e-home` (plan step 8 wording): the neighbouring `pin_exe` / viola-pty
  tests do; the root integration cases use `target/e2e-home`.
- **The piped-driver tests' homes are pre-seeded** with the companions (test-side, Windows x64), which pre-creates the home (see Spec
  claims disproved, third bullet). Overseer's word; the product path is unchanged and `conpty_sideload` never seeds.
- **The piped test driver answers DA1** (test-side) — overseer's word: "the piped driver answers DA1 the way a terminal would, and viola stays silent".
- **The pump-window resize test resizes on the wrapper's `process-start{claude-child}` line** instead of the child's `start` receipt
  (the chunk broke its premise: the child's start now waits for the DA1 round-trip through the pump).
- **Hygiene control renamed** `.andromeda/runs/2026-09-29T08-18-26-phase/baseline/control/net.rs` → `net.rs.txt` (bytes kept;
  the operator's convention); the plan's entry-9 `baseline` note still names `net.rs`.
- **The hash-compare remove-the-guard run was not run**: the permission classifier refused it; per the overseer it was not routed
  around and not handed over; the tampered-copy tests stand as the witness (`evidence/guard-runs.md`).
- **Scope record** (P1 `gate.py scope`: `scope: clean — changed 27 · listed 14 · recorded 13 (companion 3 · mechanical 0 · in-intent 10 · widening 0) · absorbed 0 · excluded 47`), all authority `self`:
  - companion: `schemas/diag-line.v1.json` (serves src/run/mod.rs) · `tests/run_cli.rs` (serves src/run/mod.rs — the pinned
    `pty_backend` literal) · `tests/support/mod.rs` (serves tests/support/piped.rs)
  - in-intent: `tests/support/outer_pty.rs` (serves tests/conpty_sideload.rs) · `tests/support/home.rs` (serves tests/run_cli.rs) ·
    `tests/support/piped.rs` (serves step 8) · `tests/cli_fake_agent.rs` · `tests/cli_instance_state.rs` ·
    `tests/cli_program_resolution.rs` · `tests/cli_version_gate.rs` · `tests/contract_diag_schema.rs` (each serves tests/support/piped.rs) ·
    `tests/tui_env_strip.rs` · `tests/tui_passthrough.rs` (each serves tests/support/home.rs)
  - mechanical: none · widening: none

## Decisions & corrections
- Overseer rulings this chunk: (1) entries 6 and 18 recorded `red — not this chunk's` on the two-sided `fb78ddc` basis, CI is the
  acceptance leg; any timing red the baseline shares gets the same record, any it does not share is this chunk's and gets fixed.
  (2) The hash-compare demo: not routed around the classifier, not run by the operator — recorded not run, the tamper tests witness.
  (3) Serial pinning kept (concurrency reverted). (4) First-start cost handled test-side by a per-run seed; no bound raised.
  (5) DA1: test-side answer, viola silent, the ~3 s stall surfaced as a product finding owned by the first headless-viola entry.
  (6) The three test fixes live in shared `tests/support/`, applied to every piped-spawn binary. (7) Do not chase the host (D: is a
  ReFS Dev Drive, C: NTFS — the overseer's measurement; cause unestablished). (8) Leaked processes stopped by exact executable path,
  never the prototype's. (9) The baseline worktree is removed with `git worktree remove` when done. (10) Probe homes under
  `target/e2e-home/probe-*` ride the founder desk item that deletes e2e-home.
- Classification flagged to the overseer: `conpty_sideload`'s 5 pre-push failures (a binary the control cannot run) were recorded
  under entry 6's basis because all 5 fail in the shared `stamped_home` fixture's `viola verify` step (`viola never exited`,
  `tests/support/verify.rs:81`), where the control fails 22 and 25 tests with the identical message.
- Sweep hazards met: `viola never exited` comes from `tests/support/verify.rs:81` (a `viola verify` in the stamped-home fixture), not
  from a wrapper; a grep for a `zz` timer prefix matched harness `fuzz_` identifiers (read the hits); the literal `13/200` has 0 hits in
  the masters (the H2 baseline is worded otherwise); nextest `LEAK` means a child held the test's stdio after it ended.
- Host mechanics met: the Bash guard refuses a `cat` heredoc with a file target (scripts go through the Write tool) and any command
  carrying a doubled backslash; a git worktree cannot move across drives; `rm -rf` in target/e2e-home was denied at the prompt.
- The test-home convention needs a ruling for seeded homes (Spec claims disproved, third bullet).

## Outcome
- **Acceptance criteria** (re-asserted against the diff):
  1. (founder) H2 200 + 200 on windows-2025 in one CI run, counts, run id and image recorded, no rate — **met**:
     ci#36563868040, image `windows-2025-vs2026` 20260828.587, sideload 200 · 0 losses, inbox 200 · 14 losses
     (`evidence/h2-with-without.md`); the overseer's relay: "sideload 0/200 vs inbox 14/200".
  2. (tests) after the removal no H2 loop / `h2-measure` / `h2_race`, final HEAD CI green 15/15 — **met**: entry 27 no output;
     ci#36566391084 on `8f643f2` 15/15.
  3. (security) planted copies never run; tampered pinned copies → `System32\conhost.exe`; restricted bare-name load → error 126 —
     **met** (`conpty_sideload` planted + tamper cases; viola-pty unit, two-sided; guard-removal run red → proven).
  4. (security) a mismatched companion never loaded, left as found, child on the inbox ConPTY; held handles deny a concurrent write —
     **met** (tamper cases; `pin_companions_holds_every_file_against_a_writer`); the hash-compare guard-removal run not run (classifier).
  5. (security) `--verify` / `--probe` green, `cargo deny check` and `deny-probes.sh` green, no new crate — **met** (entries; CI step).
  6. (arch) start order pin → sideload → spawn; child hosted by `bin/<key>/conpty/OpenConsole.exe`; viola-pty depends on no viola crate;
     portable-pty `=0.8.1` — **met** (span-order test; default integration case; the two probe entries).
  7. (obs) `pty_backend` tells the backends apart, `sideload_fallback` on a degrade, G4 green, no home-level path/hash, no
     `process-exit{exit_code:1}` / panic on a degrade — **met** (integration asserts; CI G4 green in ci#36566391084).
  8. (design / layouts / a11y) no viola byte on a fallback; zero-viola-literals on both backends; `viola list` captions unchanged by
     construction — **met** (tampered case asserts no literal; no `list` source in the diff).
  9. (tests) `bash scripts/agent-run.sh run` `"ok":true` on this Windows host — **UNMET only by a `red — not this chunk's` gate** →
     owner the P5 pin (below); CI coverage/doctest/playwright gates per OS — **met** (CI green); coverage ignore regex unchanged — **met**.
  10. No verification-matrix capability claimed — **met** (`matrix.py show`: claimed 0); v1-45's P5 `notes` line stands (phase run).
- **Gates** (the chunk's `[[gate]]` entries by `run`; implement's gate run `2026-09-29T08-52-03-implement`, final call `--only
  1,2,4,5,7-26`):
  - `cargo fmt --all --check` — green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green
  - `cargo clippy -p viola-pty --all-targets --features h2-measure -- -D warnings` — leg operator, fired once by hand at implement:
    exit 0 (`evidence/h2-measure-clippy.md`); valid only until the removal commit
  - `bash scripts/agent-run.sh run --unit` — green · `… run --integration --filter 'binary(conpty_sideload)'` — green
  - `bash scripts/agent-run.sh run` — **`red — not this chunk's: the same failures on the fb78ddc tree on this host today (45/51 of
    199 vs 54/53 of 204; the same binaries) → owner: this wrap's P5 pin`** (`evidence/entry-6-not-this-chunk.md`)
  - `bash scripts/conpty-vendor.sh --probe` — green (`4/4 refused, control clean`) · `… --verify` — green (`verified 1.24.260710001`)
  - the no-network grep, the `PTY_BACKEND` grep, the `wsl-provision.sh` diff, the `tool:` line count (exit 1, last line 0), the
    `coverage.rs` diff, the viola-pty `^viola-` grep, the `portable-pty = "=0.8.1"` count — green
  - `cargo deny check` — green · `bash scripts/deny-probes.sh` — green
  - `bash scripts/agent-run.sh pre-push` — **`red — not this chunk's`** on its windows-tests stage only (917/975; 53 in control-failing
    binaries + 5 `conpty_sideload` at the shared verify-fixture step, same basis and owner); linux-tests green (coverage 938/938,
    playwright 1/1, gate no breaches), vm-release terminated clean
  - smoke `cleanup --session p-sc-smoke` · `boot --session p-sc-smoke --instance builder` · `logs --session p-sc-smoke --process run`
    (contains `"pty_backend":"conpty-sideload"`) · `cleanup` (`processes_gone`, `endpoint_gone`) — all green
  - `gate.py hygiene` — leg operator: `refused 1` (P3 `net.rs`) → renamed → `clean` (`evidence/operator-pass.md`)
  - `git diff --quiet && … && git push origin HEAD` (×2) — leg operator: pushed `fb78ddc..2d83718`, later `224efc4..8f643f2`
  - `ci.py conclusion --sha HEAD --wait 5400` — leg operator: red ci#36563179341 (fixed at `224efc4`), then green ci#36563868040
  - `gh run view <id> --log --job … | grep 'h2-loop:'` — leg operator: exit 0, both atoms held
  - `! grep -rnE 'h2-measure|h2_race|h2-loop' …` — leg operator: exit 0, no output
  - `ci.py conclusion --sha HEAD --wait 1800` — leg operator: **green, ci#36566391084 on `8f643f2`, 15/15** (overseer-verified)
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran — `2d83718` (pre-CI) → `224efc4` (fix, CI red read) → `8f643f2` (removal); the verdicts
  rest on the final HEAD's CI run ci#36566391084 and the evidence files in `chunks/2026-09-29-sideloaded-conpty/evidence/`
  (`operator-pass.md`, `entry-6-not-this-chunk.md`, `h2-with-without.md`, `da1-stall.md`, `guard-runs.md`, `h2-measure-clippy.md`);
  implement's P4 report (in this session's conversation) for the rest.
- **Process hygiene** (implement P4 census; re-measured at this wrap: 0 of this repo's processes running — `census.ps1` by exact
  executable path under `D:\dev\projects\viola\`, the prototype excluded):
  | process | started by | final state |
  |---|---|---|
  | gate runs, harness smoke session, pre-push | this chunk's runs | terminated (smoke cleanup `processes_gone`, `endpoint_gone`) |
  | leaked `viola.exe` / `viola-fake-agent.exe` / sideloaded `OpenConsole.exe` from red local runs | this chunk's test runs | terminated by exact path (34 + 12 + 12 + 20 + 35 across the cleanups); 0 remain |
  | baseline worktree `target/baseline-wt` | this chunk | removed (`git worktree remove`) |
  | `additional/viola-lab/prototype` `viola.exe` (6, later 8) · other sessions' processes | other sessions | left running — not this chunk's |
  - Left on disk for the operator: `target/baseline-target` (the baseline's build dir; its `rm` was denied) · `target/e2e-home/probe-*`
    (ride the founder desk item that deletes e2e-home) · `target/conpty-seed/` (the per-run seed, regenerated on demand).
