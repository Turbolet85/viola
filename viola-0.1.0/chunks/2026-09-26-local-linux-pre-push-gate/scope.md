# Scope — Local Linux pre-push gate

**Marker:** `2026-09-26-local-linux-pre-push-gate` · **Version:** viola-0.1.0 · **Epoch:** Epoch 2 — Windows slice I: wrapper, events, ledger
**Working entry:** `working-route.md:36` — "Local Linux pre-push gate — WSL2 Ubuntu at CI's pins, synced Linux-filesystem clone of the working tree, Unix tests and ubuntu mutation leg before every push"

## Aim (founder ruling, relayed by the overseer 2026-09-26)
Catch Unix reds BEFORE the push. The last two chunks lost CI round-trips to Unix-only reds this Windows host
could not run: the PTY SIGHUP holder (chunk 2026-09-25-pty-wrapper-on-windows, `fix` commits c05e6e2 / 7681c73)
and the Windows/Linux mutation asymmetry (chunk 2026-09-26-ci-chunk-base-and-union-verdict). CI round-trips cost
more than the code (CARRY 1, operator route adaptation 2026-09-25).

## What this chunk builds
1. **A provisioned Linux toolchain inside WSL2 `Ubuntu`, at CI's pins, never floating.**
   - Every call is `wsl -d Ubuntu -- …`; the DEFAULT distro is `docker-desktop` (CARRY 1; overseer direction 2).
   - rustup + the `rust-toolchain.toml` channel `1.98.1` (components `rustfmt`, `clippy`); cargo-nextest `0.9.146`
     and cargo-mutants `27.1.0`, the CI pins (`ci.yml:166`, `tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0`; the
     CARRY cites `ci.yml:162`, re-verified at promotion to line 166). Installing them inside Ubuntu is in scope
     (overseer direction 3).
   - The pins are read FROM `ci.yml` / `rust-toolchain.toml` (or asserted equal to them) by the provisioning
     step, so a CI pin bump cannot leave the local leg on a stale tool silently (verified feasible: the pins sit
     at `ci.yml:33`, `:166`, `rust-toolchain.toml:2`; research §Files inspected).
   - [premise-corrected: CI's `test` job runs `run --coverage` then `gate --require coverage,doctest`
     (ci.yml:36-44, :137-140), and the harness has both forms] Whether the local Unix tests take CI's
     instrumented form or plain `run --unit --integration` is a P4 fork (research §Open questions 2).
   - [premise-corrected: `cc`/`gcc`/`make`/`pkg-config`/`jq` are absent (research M3); `sudo` needs a password, but
     `wsl.exe -d Ubuntu -u root` is uid 0 with no password (overseer measurement 2026-09-26 23:58, re-read at P5)]
     The C linker (`build-essential`) is installed as root through `wsl -u root --exec /usr/bin/apt-get`, an
     ordinary agent-run build step recorded with its version; so are rustup, the toolchain and the cargo tools.
     `git 2.53.0` is already present.
   - The Rust side of provisioning uses a pinned rustup-init with its published sha256 (host rustup is 1.29.1)
     and `cargo install --locked` for the ci.yml tool pins (arch/security extracts; research M3).
2. **A Linux-filesystem clone of the repo, synced from the Windows working tree.**
   - Lives in the Linux filesystem (e.g. under `~` in Ubuntu), never `/mnt/d` (CARRY 1; direction 4).
   - Carries the repo's git HISTORY, not only the working tree's files: `resolve_base`
     (`crates/viola-e2e/src/harness/run/mutants.rs`) derives the mutation base from the master-route
     `· complete ·` pickaxe bounded at the oldest `chore({marker}): operator pre-CI commit`'s parent; without the
     flip in history the base falls to `merge-base HEAD origin/main` and then `base-missing` (the path
     `resolve_base_needs_a_real_commit` tests) (CARRY 2).
   - Synced INCLUDING uncommitted changes — the gate runs before the operator pre-CI commit (direction 4).
   - The sync form is CHOSEN BY MEASUREMENT (speed + correctness) across candidate forms, and the plan records the
     measurement (direction 4).
   - Proven: a dirty file in the Windows tree reaches the Linux run (direction 4).
   - The sync never carries Windows `target/`, `mutants.out*`, or any other git-ignored file into the clone,
     and deletions / renames in the Windows tree propagate (a stale Linux file must not mask a Windows delete)
     — verified by measurement for the clone + fetch + temp-index-patch form: `git add -A` honours
     `.gitignore`, the patch carries deletions, `git clean -fd` (no `-x`) drops stale untracked files while
     keeping the ignored build cache (research M8).
   - Line endings: the synced bytes are the LF bytes — verified (0 CRLF files in the clone's `crates/` and
     `src/`; the patch form normalises through git by construction; research M8).
   - Measured choice (research M8): clone once (4.47 s), then per gate a Windows-built binary patch through a
     temporary index + `fetch` + `reset --hard` + `apply` ≈ 0.9 s, vs rsync 9.85 s / 2.74 s that also carries
     every ignored file not excluded by hand. No listener in either form.
   - Every WSL call is `wsl -d Ubuntu --exec …` with an explicit PATH: the `--` form re-parses argv through the
     distro shell, and the distro PATH carries the Windows PATH incl. the Windows cargo (research M2, M4).
   - The session's `CLAUDE*` values (10 present, incl. `CLAUDE_CODE_MESSAGING_TOKEN`) do not reach WSL
     (`WSLENV` forwards only `WT_*`; measured 0 inside), and the gate must not add them (research M5).
3. **A bounded Linux build cache.** The WSL vhdx lives on C: (199.1 GB free and a 1.48 GB vhdx, re-measured
   2026-09-26, research M6); the Linux `target/` stays bounded and its size is REPORTED by the gate (direction 2).
   - [inferred] "Bounded" is a stated cap the gate enforces (a cold clean of the clone's `target/` when above
     it), not only a printed size; a vhdx never shrinks on delete, so the cap bounds the PEAK (research M6).
     The cap's value is P4's.
4. **An agent-run gate entry** that, from Windows, syncs the clone and runs inside Ubuntu: the Unix tests + the
   ubuntu mutation leg (`run --mutants --leg ubuntu` or its harness equivalent) against the DERIVED base
   (direction 5; CARRY 1).
   - The leg's printed `base` should equal the Windows and CI legs' base (CARRY 2).
   - The leg is judged by its VERDICT — 0 missed, 0 timeout, unviable ≤ caught — never by caught/unviable counts,
     which differ run to run and from CI's (local 73/6 and 71/8 on one tree, CI 74/5) (CARRY 2).
   - A red there STOPS the push (direction 5).
   - The entry surfaces a missing/unprovisioned distro or tool as a typed, named failure (not a silent skip and
     not a green) — verified pattern: `Refusal { reason, detail }` / `tool-missing` (run.rs:93-105).
   - Where it lives: an INTERNAL `viola-harness` subcommand (`pre-push`, like `gate` / `secret-scan`), forwarded
     by one shim arm each — verified shape (bin/viola-harness.rs:56-69, agent-run.sh:74); the 5-command agent
     surface is untouched; test-plan §3 Internal harness subcommands + Closed enums gain it at wrap.
   - [premise-corrected: under `--leg` a run defers every survivor (run.rs:122), and `gate --mutants-legs
     ubuntu-latest` alone judges a `#[cfg(windows)]` body by the ubuntu leg (gate.rs:144-149)] "Judged by its
     verdict" needs a union reading, not the leg's own exit: either the local union of both legs or new
     Linux-only judging code — a P4 fork (research §Open questions 1). The leg must be NAMED `ubuntu-latest`
     for the unix+linux family (cfg_legs.rs:24).
   - The printed `base` equals the Windows-local derivation by construction: the clone's HEAD is the Windows HEAD
     (fetched sha), its history holds the flip (`a69c5ef` at promotion) and its tree holds the pending record
     (research M8).
   - [premise-corrected: re-derived at P4 — HEAD a69c5ef IS the last flip (`git log -1 -G ' · complete · ' HEAD
     -- .andromeda/master-route.md` → a69c5ef) and HEAD's tree holds 0 pending records, so before the pre-CI
     commit `pre_ci_parent` finds none, the bound is HEAD, and `chunk_flip` takes its "flip is HEAD → HEAD^"
     step (mutants.rs:80-82) → base acd08c7; after the pre-CI commit CI derives a69c5ef] CARRY 2's "the leg's
     printed base should equal the Windows and CI legs' base" does NOT hold for a gate run before the pre-CI
     commit while HEAD is the wrap flip (the normal state at an operator pass): the local base is HEAD^ and the
     local diff adds the previous wrap commit's delta. /implement's local Windows leg has the same skew. How
     the local base reaches CI's (a base-rule change, or the gate after the commit) is a P4 fork.
5. **Wired into the operator pass BEFORE the push entry** (direction 5): every chunk's operator pass runs this gate
   ahead of the `git push` operator entry, and a red there stops the push.
   - [inferred] The wiring's home: the project's own operator-pass contract (test-plan §3 / §10, `.claude/docs/
     workflow.md`, `rules/verification-harness.md`) — the Andromeda skill's `plan-template.md` lives outside this
     repo; whether the "plan template" in direction 5 means the project's docs or the pipeline skill is a P4 fork
     (research §Open questions 3; unverifiable at HEAD — the operator's word decides it).

6. **The Linux red the gate found, folded in** (operator ruling at implement, 2026-09-26, relayed by the overseer:
   "fold, don't split — a red found now goes into this chunk"; `evidence/linux-red-investigation.md`).
   `pre-push` went red 3/3 at `linux-tests` on `tui_host_resize_reaches_the_child` while 68 isolated reproductions
   stayed green. The order is binding, so the cause is MEASURED, not inferred from a green:
   - [inferred] HYPOTHESIS (read from code, unmeasured): the resize lands between the spawn sizing
     (`src/cmd/run.rs:63`) and the pump's second size read (`crates/viola-pty/src/lib.rs:293`), so the pump's
     baseline already equals the new size and nothing is ever forwarded.
   - First, instrument: a one-shot, temporary stamp of the spawn read, the pump's baseline read and each changed
     poll (time and value), and a permanent failure dump in the test of the child's `size` receipts with the times
     the test sent the resize and observed each receipt. Run `pre-push` with it before any fix.
   - Only if the dump confirms the window: the pump starts from the spawn size instead of a second read, a test
     forces the resize into that window deterministically (not a timing bet), and `pre-push` reads green three
     times running. If the dump does not confirm it: stop and report what it shows; no fix applied blind.
   - This is a product defect a real user can hit (a terminal resized while `viola run` starts), whatever the
     gate's environment adds.
7. **Entry 6 carries `apt-get update`** (operator ruling, same relay): measured at implement, exit 100 on three 404s
   under stale package lists (`evidence/provision.md`).

## Witnesses (direction 6)
- A PLANTED Unix-only failure (e.g. a `#[cfg(unix)]` test that fails) goes RED locally through the gate, before
  any push; removing the plant returns it green.
- The gate's own guard takes its remove-the-guard runs as usual, including a mutation witness (CARRY 1: "that
  gate's own guard takes a mutation witness as usual").
- The dirty-file sync proof (item 2).
- Standing: tests with mutations; fold, do not carry (direction 6).

## Boundaries
- [premise-corrected: operator ruling at implement, item 6] Product code changes in exactly one place, the
  `viola-pty` pump's resize baseline (and its caller in `src/cmd/run.rs`), and only if the instrument confirms the
  window; everything else stays test-side / harness / scripts / docs (`viola-e2e`, `scripts/`, the tui test,
  spec amendments at wrap).
- CI stays the verdict of record; the local gate is a pre-push filter, not a replacement for the CI legs.
- [premise-corrected: item 4's base bullet; operator ruling at P4 "Fix the base rule"] The derived-base
  algorithm changes in ONE place only: `chunk_flip`'s "flip is HEAD → HEAD^" step is skipped while the working
  tree's master-route carries a pending record that HEAD's tree lacks (a promoted, uncommitted chunk). Every
  other base rule (override, pre-CI bound, merge-base fallback, `base-missing`) stays as it is.
- Operator rulings at P4 (2026-09-26): the local verdict is the UNION of both legs (ubuntu-latest in WSL +
  windows-2025 on the host, CI's own `gate --mutants-legs`); the Unix tests take the CI test-job form
  (`run --coverage` + `gate --require coverage,doctest`); the wiring is project-side, and the Andromeda
  `plan-template.md` change is a PROPOSAL recorded in this chunk's evidence for the overseer to relay (a project
  chunk never edits Andromeda).
- macOS reds are out of scope (no local macOS).

## Surfaces / contracts touched
- `scripts/agent-run.{sh,ps1}` + `crates/viola-e2e/src/harness/` (the gate entry, its JSON output and typed exit).
- `.github/workflows/ci.yml` + `rust-toolchain.toml` as pin SOURCES (read, not changed).
- test-plan §3 (harness commands) / §10 (quality gates) / §12, `.claude/docs/{workflow,commands}.md`,
  `rules/verification-harness.md`, `rules/host-win32.md` (WSL recipes) — amended at wrap.

## CI verdict read at Setup (5a)
- a69c5ef (the last wrap's flip = HEAD): 13/15 completed success; `lint (windows-2025)` and `test (macos-latest)`
  in progress — **verdict not yet available**; no completed red to disposition.
- Update at P5 (relayed by the overseer): a69c5ef finished success, run 36272899441, all 15.
