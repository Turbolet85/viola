# Scope — 2026-10-02-epoch-2b-cleanup · Epoch 2b cleanup

**Working entry** (`viola-0.1.0/working-route.md:66`, head of `### Epoch 3 — Windows slice II: driving verbs and live proof`):
Epoch 2b cleanup — in-repo suite red diagnosed and fixed, coverage re-taken, eight mutation survivors killed, viola-e2e
and cfg(unix) mutants scored, test tempdir leaks closed — plus nine CARRY blocks (all nine folded below; completeness
check `route.py pins` → `{run_dir}/` trail: nine `:66` rows).

**Split (P1, 2026-10-02 — overseer, founder-delegated; the founder is told at once).** Sized over one window, so at P1
the operator took the split the overseer had pre-named at the 2026-10-01 0-pending wrap: **M3** (the viola-e2e mutation
form) and **the WSL leg** (the 12 `cfg(unix)` mutants) leave this chunk. They are **owed, not dropped**: this chunk's
wrap (route-resolve) gives them their own markerless entry at the Epoch 3 head, carrying both CARRY blocks verbatim. The
WSL leg's possible return of the retired Linux mutation leg's `TMPDIR` carve-out (a widening of the `env -i` boundary)
goes to the founder live when THAT entry phases, never here. P5 stays in this chunk (the revive entry reads its result).

## What this chunk builds

### 1. M2 — the in-repo suite red, diagnosed and fixed (FIRST, in order)
- Source: code audit `.andromeda/runs/2026-10-01T09-18-50-code-audit/proposals.md` §M2; the overseer relay
  `e2b-route-adaptation.md` A (placed at the 2026-10-01 0-pending wrap; standing direction: one cleanup chunk at the head
  after every epoch's audit).
- The measured fact (audit run 2, 2026-10-01): the project coverage command failed 80 of 985 tests in the repository tree
  — 0 foreign rustc in 14 samples, CPU 8–24 %; D: held 80–83 GB free through run 2 (overseer measurement), so a near-full
  disk is ruled out for that run. First panics: "viola never exited" ×32, "the wrapped program never started" ×12,
  "wrapper builder not ready" ×7, readiness-timeout boot ×6, "timed out waiting for start" ×5. Failures concentrate in
  `viola::run_cli` 12 · `hook_fail_open` 11 · `cli_verify` 10 · `cli_version_gate` 8 · `cli_instance_state` 7 · … (16
  binaries). Contrast: the viola package passed 367/367 in cargo-mutants' gitignore-filtered copied tree; a plain in-repo
  nextest probe failed 2/2 with every inherited `CLAUDE*`/`VIOLA_*`/`ANTHROPIC*` unset; CI green on `11f135c`.
- [premise-corrected: P3 measured the suspects and found a fourth difference — research.md §Measured host facts]
  Suspects, the entry's verbatim — "(hypothesis, unmeasured): gitignored state under the tree (`target/e2e-home`,
  `target/conpty-seed`, `target/baseline-target`) or the tree's location, found by diffing what the in-repo run sees
  against the copy." All three dirs exist. `target/e2e-home/` holds 4 768 `viola-test-*` homes, none with an
  `owner.json`, so the owner sweep can never reclaim them; they are part-deleted homes whose remaining files were held
  open by a live process at drop. The copied tree also ran with `TMP`/`TEMP` pointed at a fresh scratch (the audit's
  `mut.sh`), while the in-repo run uses `%TEMP%`, which holds 23 269 leaked `.tmp*` dirs and the `viola-root-watch`
  reports. Its four measured differences are therefore `target/e2e-home`, `TMP`/`TEMP`, the build dir/profile and the
  tree path. None is yet shown to be the cause (still hypothesis, unmeasured). The top panic, "viola never exited" ×32, is
  `tests/support/verify.rs`'s bounded wait on a `viola` child (the stamped-home `viola verify` step) passing `WITHIN`
  (7 s) — testing.md 2026-09-29, re-read at `tests/support/verify.rs:70-84`.
- **Constraint extended (P3, the same reason as the operator's):** the `%TEMP%` `.tmp*` leftovers are an M2 suspect
  too, so they also stay until M2 has recorded its diff.
- **Constraint (operator, P1):** `target/baseline-target`, `target/e2e-home` and `target/conpty-seed` stay in place until
  M2 has diffed the in-repo run against them; no cleanup touches them before that comparison is recorded.
- It is an **open red until its cause is known** (CLAUDE.md learning): a green re-run never closes it. Done = the cause
  named with a witness, fixed (or, if the cause is outside this repository, recorded with its evidence), and the
  in-repo suite read green on the project's own command.
- Once fixed: this boundary's **coverage is re-taken** and recorded as a correction to the code-audit record (the
  Epoch 2b line value the audit could not measure; `coverage.line` was 96.77 at Epoch 2).
- Its outcome settles the host-reds CARRY (§8).

### 2. M1 — eight mutation survivors, each killed or exempted
- Source: the same audit §M1. Each survivor gets a killing test, or a recorded equivalent-mutant exemption naming why:
  - `crates/viola-pty/src/sideload.rs` `search_restricted` → `true` and → `false` (both constant returns; P1: `:33`)
  - `crates/viola-pty/src/lib.rs` `<PortablePty as Pty>::resize` → `Ok(())` (P1: `:195`)
  - `crates/viola-pty/src/lib.rs` `<HostTerminal as Drop>::drop` → `()` (P1: `:328`)
  - `crates/viola-state/src/fs.rs` `restrict` → `Ok(())` (P1: `:16`)
  - `crates/viola-state/src/pin.rs` the `pin_exe` NotFound match guard → `true` (P1: `:81`) and the `pin_companions` one
    (P1: `:171`)
  - `src/cmd/run.rs` `refuse_stale` → `()` (entry cites `:455` at `95c1a9b`; P1 at HEAD: `:454`)
- The gate is test-plan §10's (`missed == 0`, `timeout == 0`, `unviable <= caught`), judged at the boundary since the
  2026-09-28 ruling. This chunk's own evidence is a direct-file cargo-mutants run per survivor's file in its owning
  package, reading the eight caught or exempted. [intent-incomplete, P5 val-1: `run --mutants --file` adds `--in-diff`
  (`crates/viola-e2e/src/harness/run/mutants.rs:148`), so a survivor line this chunk does not touch would never be
  regenerated (testing.md 2026-09-25)]
- [premise-corrected: research.md §Files inspected / §Graph impact — the shape holds for three survivors, not all eight]
  The audit's "suspected shape" (verbatim: "Each is an effect (DLL-search state, a resize reaching the child, a
  drop-time restore, a refusal) no assertion in its unit observes."), re-derived at HEAD:
  - `search_restricted` ×2: holds. Its one caller is root `src/cmd/run.rs:291`, and no viola-pty test reads it.
  - `refuse_stale`: holds on Windows. Its only witness, `tests/cli_instance_state.rs:207-249`, is Unix-only (SIGSTOP).
  - `PortablePty::resize` and `HostTerminal::drop`: FALSIFIED. viola-pty tests already wait on both effects
    (`crates/viola-pty/src/lib.rs:749`/`:801` `size 120x40`, `:752`/`:784` `restored=true`, none `cfg`-gated). Their
    survival is unexplained. The audit drove cargo-mutants directly (`mut.sh`), not through `run --mutants`, so they are
    reproduced with a scoped `run --mutants --file` before any test is written.
  - `restrict`: an equivalent mutant on this host. Its non-Unix body is already `Ok(())` (`crates/viola-state/src/fs.rs:22-26`).
    The Unix half is witnessed by `fs_private_modes_are_explicit` (`:346-348`, `cfg(unix)`).
  - The `pin_exe` / `pin_companions` NotFound guards: the effect is observable only through an open that fails for a
    reason other than NotFound while the replace would succeed (a deny-read ACE on Windows; mode 000 on Unix). A
    directory in the file's place (`pin_companions_reports_a_directory_where_a_file_should_be`) errs on both paths alike.

### 3. Test tempdir leak closed
- Source: the same audit — "the tests leak tempfile dirs — 545 `.tmp*` dirs, about 1 GB, across the mutation units
  (overseer measurement at the boundary)".
- [premise-corrected: research.md §Measured host facts — the leak is in `%TEMP%` and `target/e2e-home`, far past 545]
  The dirs land in `%TEMP%` (the mutation scratch when cargo-mutants sets `TMP`/`TEMP`): 23 269 `.tmp*` dirs. Of the
  4 377 dated 10-01, 3 832 are viola-e2e harness self-tests' throwaway git repos that keep only their read-only git
  object files (Windows refuses to delete them, so `TempDir`'s drop leaves the dir; `crates/viola-e2e/src/harness/run.rs:500-530`,
  `crates/viola-e2e/src/harness/run/mutants/base.rs`). Another 399 are throwaway cargo projects and 132 are empty dirs. A
  second leak sits in `target/e2e-home/`: the 4 768 ownerless part-deleted homes (§1). Done = one suite run leaves no new
  `.tmp*` dir and no new ownerless home, by a before/after count, with the keep-on-failure contracts (`viola-root-watch`,
  `viola-pty-watch`, `AGENT_RUN_KEEP_*`) intact.

### 4. Evolve L4 — crate sources through `cargo metadata`
- Source: Epoch 2b evolve diagnosis `.andromeda/runs/2026-10-01T09-05-15-evolve-diagnose/proposals.md` §L4 (project
  half): "crate sources are located through `cargo metadata`, never `CARGO_HOME` (a host-win32.md or gotchas line)".
- One line in `.claude/rules/host-win32.md` Session Additions or `.claude/docs/gotchas.md`.

### 5. Evolve P6 — the "unbuilt selector" test
- Source: the same diagnosis §P6: `crates/viola-e2e/tests/cli.rs` `unbuilt_selectors_and_unknown_commands_are_usage`
  (P1: `:59`) went red in 3 chunks as each selector got built; it "becomes build-independent or moves into the e2e
  build's set".

### 6. Evolve P2 — the deadline-below-kill-line rule becomes a project lint
- Source: the same diagnosis §P2 (project half): testing.md's 2026-09-24 entry (extended 2026-09-27 twice: a test's
  assertion deadline sits strictly below the nextest profile's kill line) recurred; it becomes a lint or test that
  asserts each test deadline constant sits below the kill line.
- [premise-corrected: research.md §Files inspected — the population is test-side `Duration` constants against their package's kill line]
  The kill lines are `.config/nextest.toml`'s `mutants` profile at 5 s × 2 = 10 s and its `package(viola-e2e)` override
  at 15 s × 2 = 30 s; cargo-mutants' floor is 20 s. Test-side named deadlines found by
  `grep -rn -E 'const [A-Z_]+: Duration = Duration::from_(secs|millis)\(' src crates tests`: `tests/support/watch.rs`
  `WITHIN` 7 s, `crates/viola-pty/src/lib.rs` `CHILD_WITHIN` 7 s, `tests/tui_passthrough.rs` `RESIZE_WITHIN` 8 s,
  `crates/viola-state/src/heartbeat.rs` test `WITHIN` 4 s, and `tests/hook_fail_open.rs` `SPINE_BOUND` 1 s. Product
  constants (`PROBE_DEADLINE` 120 s, `VERSION_DEADLINE` 5 s, the harness's 20 s boot deadlines run by viola-e2e under
  its 30 s kill) are a different class, which the lint's stated scope must name. It reads the kill lines from the file,
  never a copied literal.

### 7. P5 — can a `claude` child outlive its wrapper
- Source: `refs/session-memory-options.md` §5 row P5 (overseer, founder-delegated, at the 2026-10-01 0-pending wrap:
  "measure it now").
- Measure: when the Windows Terminal tab closes or the wrapper is killed, does the `claude` child die, and does its
  session keep a `claude agents --json` row — **first the fake agent under `viola run`, then the real CLI** with
  `Stop-Process` on the wrapper, then `Get-Process -Id <child_pid>` + the agents rows.
- The study's mechanism claim, verbatim: "The inner ConPTY close should end attached clients `[H]`, but with no job
  object (M9) a grandchild could survive." — and "the study names a kill-on-close job object, hypothesis". Re-derived at
  HEAD: "no job object" holds (`grep -rn -i 'JobObject|CreateJobObject|KILL_ON_JOB_CLOSE' src crates Cargo.toml` finds
  0). Whether the ConPTY close ends the child is exactly what P5 measures, so it stays a hypothesis.
- Other `viola.exe` sessions of the operator's run on this host (research.md). P5 stops only the exact pids it spawned
  itself, never a process by name.
- **This chunk measures; its wrap routes any fix.** A surviving child is "a 0.1.0 defect whether or not revive ships"
  (a driverless `claude` with no wheel, against "the human always wins"). The revive entry reads this result rather than
  re-measuring. The tab-close leg is founder-attended (the study's §5 marks P5's tab close as FA); the `Stop-Process` legs
  are agent-runnable, by exact pid.

### 8. CARRY — host reds (moves with the head unless retired here)
- Source: chunk 2026-09-29-sideloaded-conpty (operator's word at its wrap, overseer agreeing), re-read by
  2026-09-29-fake-agent-drift-contract: the local integration suite is red on this host on pre-chunk trees too, one tree
  grading differently run to run; the standing method is a same-day control (reverse-order standalone pairs, then the
  unmatched binaries alone on both trees) with CI as the acceptance leg
  (`viola-0.1.0/chunks/2026-09-29-sideloaded-conpty/evidence/entry-6-not-this-chunk.md`,
  `…/2026-09-29-fake-agent-drift-contract/evidence/host-reds-two-sided.md`).
- "The chunk that establishes the cause or reads the local suite green retires it." M2 (§1) is that attempt: if M2 lands
  the cause and the suite reads green, this CARRY retires at this chunk's wrap; otherwise it moves on with the head.
- Until M2 closes, this chunk's own gate judgements use the same-day-control method.

### 9. CARRY — WSL `--install-deps` hardening (moves with the head)
- Source: chunk 2026-09-27-browser-verdict-reachability (overseer live ratification, operator-only): before the WSL
  distro is next re-provisioned, `scripts/wsl-provision.sh --install-deps` stops running user-writable code as root (root
  runs only `apt-get install` over an allowlisted dry-run list).
- This chunk re-provisions nothing (the WSL leg left with the split) → it moves on with the first markerless entry;
  `wsl-provision.sh` and ci.yml's `test`-job tool line stay byte-unchanged. If any step here finds it must re-provision,
  this CARRY is taken in full first.

## Excluded — owed to the split-off entry (minted at this chunk's wrap)
- **M3** — verbatim CARRY: viola-e2e (727 mutants) is unscored at both boundaries — 32 of 246 harness tests fail in
  cargo-mutants' copied tree because they spawn the root package's `viola-fake-agent.exe`, which a `--package=viola-e2e`
  build never produces; the boundary tier takes a viola-e2e form with the root bins in each copied tree (the project's
  `run --mutants` prebuild + `--copy-target` form, its copy size measured first, or a test seam that builds what it
  spawns).
- **The WSL leg** — verbatim CARRY: 13 mutants never compiled on this Windows host — 12 `cfg(unix)` (viola-pty
  `HostTerminal::enter` ×4 and `host_size`; viola-channel `open_by`, `host_socket_dir`, `listen`, `Guard::drop`;
  `src/panic_frames.rs` `raw_frames` and `module_of` ×2) and 1 `cfg(all(windows, not(target_arch = "x86_64")))`
  (`src/cmd/run.rs` `sideload_outcome`); the boundary mutation tier gains a WSL Ubuntu leg measuring the 12; the 13th is
  recorded not measurable (no Windows non-x86_64 host), in the form M1's exemptions take.
- If §1's diagnosis touches the harness mutation path, it records the M3-relevant facts it measured for that entry.

## Boundaries
- No route reorder or removal here; the split's new entry is the wrap's write.
- No control retired or relaxed (security.md, the `env -i` WSL boundary, the release-check, the G2 exemption list). A
  fix that would widen a boundary is shown to the founder at P4, not first at the wrap (operator, P1).
- M2's diff evidence and the coverage correction live in `chunks/2026-10-02-epoch-2b-cleanup/evidence/`.

## CI read at Setup (last wrap's flip `093bffb` → HEAD)
| sha | verdict | wall |
|---|---|---|
| `e0fbc724390a` | green · 15/15 · ci#36863595260 | 263 s |
| `95c1a9b5fc59` | green · 15/15 · ci#36834171359 | 254 s |
| `4a3062d71804` | untested — pushed under a later tip (docs commit `docs: session memory raise/revive options`; its tip `95c1a9b` green) | — |
| `11f135c450e6` | green · 15/15 · ci#36590311253 | 274 s |
| `093bffb24c7f` | green · 15/15 · ci#36587273249 | 273 s |

No red and no `not green` → nothing to disposition.
