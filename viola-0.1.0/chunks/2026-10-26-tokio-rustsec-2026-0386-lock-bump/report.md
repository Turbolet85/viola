# Report — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

**Chunk:** the locked tokio moved to a version RUSTSEC-2026-0386 lists as patched, so supply-chain reads green with no ignore widened
**Date:** 2026-10-26T06:30Z
**Commits:** `948f7f01 chore(2026-10-26-tokio-rustsec-2026-0386-lock-bump): operator pre-CI commit, for the run this chunk's verdict reads` (the one commit since `last_wrap` 2026-10-26T01:58:54Z beside that wrap's own `b1249fb1`; `git log --format='%h %s' b1249fb1..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `Cargo.toml` (line 242) · `Cargo.lock` (lines 1633, 1635) — basis `gate.py scope`: `scope: clean — changed 2 · listed 2 · recorded 0`, and the new-text listing below. No source, crate manifest, `deny.toml`, `deny-sync.toml`, lint, toolchain or workflow file changed (the guard entry: `guard: allowed 2 — Cargo.lock, Cargo.toml · companions excluded 0 · changed 2 · excluded 0` / `guard: clean`).
- **Symbols / APIs:** none — no symbol, IPC method, endpoint, export, port, socket or env var added or changed. viola calls no `AsyncFd` constructor (research.md Sweeps: `grep -rn AsyncFd --include=*.rs` over `crates src tests` 0 hits); the four constructors tokio 1.53.3 deprecates (`AsyncFd::{new, with_interest, try_new, try_with_interest}`, `inputs#I2`, `inputs#I4`) reach no viola call site, and clippy `-D warnings` on the host and the `x86_64-pc-windows-msvc` target read green with no `deprecated` line.
- **Crates / modules:** none added, removed or changed. Tokio stays built only in `viola-mcp` (`rt`), `viola-channel`'s optional `tokio` feature (`io-util`, dev `rt`) and the test-only `viola-e2e` (`rt` + `process`): the four sole-root `deny-sync.toml` bans over `scripts/sync-crates.txt` exit 0, `scripts/channel-tokio-free.sh` ends `channel-tokio-free: ok`, `scripts/deny-probes.sh` ends `deny-probes: 13/13 banned, control clean`.
- **Dependencies:** bumped — `tokio` `=1.53.1` → `=1.53.3` (the `[workspace.dependencies]` exact pin, `default-features = false` kept; `Cargo.toml:242`); the lock's `tokio` entry `version = "1.53.1"` → `"1.53.3"` and its `checksum` (`Cargo.lock:1633,1635`), written by `cargo update -p tokio --precise 1.53.3` (exit 0, one line `Updating tokio v1.53.1 -> v1.53.3`). Lock delta against `b1249fb1`: 4 lines, the tokio entry alone (gate entry `git diff -U0 b1249fb1… -- Cargo.lock | grep -c '^[-+][^-+]'` → `last line 4`); `tokio-macros` 2.7.2, `tokio-stream` 0.1.19, `tokio-util` 0.7.20 and every other crate unmoved. tokio 1.53.3 declares rust-version 1.71 and the same dependency set as 1.53.1 (`inputs#I3`); the MSRV floor 1.96 `cargo check --workspace` reads green. Added: none.
- **Schema / config:** none.
- **Spec-master edits:** none (this chunk's steps wrote no master; the plan's expected amendments are the wrap's, below).
- **Retired:** the open advisory RUSTSEC-2026-0386 against the locked tokio — `cargo deny check` (all four families, cargo-deny 0.20.2, the CI pin) exits 0 and its output lacks `RUSTSEC-2026-0386`; the security-plan `deps-supply-chain` group's open-advisory line that names this entry as its owner (`.andromeda/security-plan-detail/dependency-security.md:58`) states an open fact this chunk closed. No version-census row (no `census.md` in `viola-0.1.0/`).
- **Counts / qualifiers moved:**
  - tokio's version `1.53.1` → `1.53.3` — pattern `tokio-ver`: 8 standing master sites (`.andromeda/architecture.md:442`, `.andromeda/architecture-detail/stack-and-technologies.md:11`, `.andromeda/security-plan-detail/dependency-security.md:14`, `.andromeda/security-plan-detail/dependency-security.md:58`, `.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8`, `.andromeda/test-plan-detail/mocking-stubbing-discipline.md:17`, `.andromeda/test-plan-detail/test-data-fixtures.md:21`, `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:24`) · 8 changed (each states the locked/pinned tokio version as current truth) · 3 leaf rows (`.claude/docs/stack.md:10`, `.claude/docs/stack.md:47`, `.claude/docs/services/viola-mcp.md:11` — the cascade's leaf step).
  - the open advisory RUSTSEC-2026-0386 → closed — pattern `rustsec-0386`: 1 standing master site, `.andromeda/security-plan-detail/dependency-security.md:58` ×2 · 1 changed (the open-advisory line retired, closed by this chunk) · 0 leaf.
  - `supply-chain` red on every CI run since the 2026-10-20 wrap → green on ci#40278510123 (sha `948f7f01`) — pattern `sc-red`: 0 master sites — basis the refusal line `control: sc-red, owner-generic never fired over the seven masters' pre-pass text, bodies and sets (948f7f01)`; the fact stands only inside the line `rustsec-0386` names (`dependency-security.md:58`).
  - Owner title «tokio RUSTSEC-2026-0386 lock bump» — pattern `owner-title`: 1 standing master site, `.andromeda/security-plan-detail/dependency-security.md:58` · 1 changed (the owner it names is this chunk; retired with the line).
  - Sweep reach: 3 patterns · 3 moved facts, 3 with a pattern (the report's own count: the version, the advisory, the CI red) · owner pattern 1 standing · by line 0 lines · 0 offsets (basis `cascade.py sweep --pre-pass --pattern tokio-ver rustsec-0386 owner-title`, trail `.andromeda/runs/2026-10-26T06-27-48-wrap/cascade-2026-10-26-tokio-rustsec-2026-0386-lock-bump-pre-pass.json`: `total (3 patterns) · 13 rows over 8 files`).
- **Dev-tool versions:** none — cargo-deny re-read at 0.20.2 and zizmor at 1.30.1 (the CI pins, both unchanged); `tokio` is a lockfile-resolved crate, the Dependencies bullet's subject, not this line's.
- **Harness / gate surface:** none changed. Read on CI for the first time since the 2026-10-20 red: job `supply-chain`'s steps `9 Fuzz lockfile audit (advisories, sources)`, `10 Tokio ban per sync crate (sole root)`, `11 Ban probes`, `12 Workflow lint (zizmor)` — each `success` on ci#40278510123 (job 121426373523), after `8 Dependency policy (cargo deny, all four families)` `success`.
- **Cross-project / external claims:**
  - CI: ci#40278510123 · sha `948f7f015ba913b112447f1fe7ceb45e11d21e24` · event push · `run_attempt` 1 · `completed/success` · `verdict: green · checks 15/15 · wall 778 s` (`ci.py conclusion --sha HEAD --wait 2400`; `gh api repos/Turbolet85/viola/actions/runs/40278510123`). The verdict was taken on `948f7f01`; this wrap's commit adds the records on top.
  - CI before: ci#40257114709 · sha `b1249fb1` · failure on `supply-chain` alone (one `error[unsound]` RUSTSEC-2026-0386 on tokio 1.53.1; scope.md, CI since the last wrap).
  - `inputs.py verify`: `inputs: 7 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 3 · unparsed 0`. No DRIFT.
    - I1 · message (the operator, relaying overseer1's call, 05:14Z) · copy · n/a — cited scope.md, research.md, plan.md.
    - I2 · `~/.cargo/advisory-dbs/advisory-db-3157b0e258782691:crates/tokio/RUSTSEC-2026-0386.md` · pointer committed@21d909e4 · unchanged — the advisory, patched `>= 1.51.6, < 1.52.0` and `>= 1.53.3`.
    - I3 · `~/.cargo/registry/…/tokio-1.53.3/Cargo.toml` · copy · unchanged — rust-version 1.71, dependencies identical to 1.53.1's.
    - I4 · `~/.cargo/registry/…/tokio-1.53.3/CHANGELOG.md` · copy · unchanged — the 1.53.2 and 1.53.3 notes.
    - I5 · message (delegate overseer, founder-delegated, relayed by the operator, 06:03Z) · copy · n/a — cited here: `inputs#I5`, the P5 word yes on `.andromeda/runs/2026-10-26T05-14-53-phase/p5-card-1.md` (delegate overseer, founder-delegated, the founder asleep). The card's `## Beyond the plan` rows, verbatim: `none`.
    - I6 · message (the operator, live, 06:05Z, the /andromeda-implement arguments) · copy · n/a — cited here: `inputs#I6`, the operator's word that the implementer runs the operator pass and the CI read.
    - I7 · message (the operator, live, 06:27Z, the /andromeda-wrap-session arguments) · copy · n/a — cited here: `inputs#I7`, the wrap run whole as an ordinary chunk, no forced sample; the 85 % and 90 % context alarms expected, stop only at 95 %.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none — the masters' tokio `1.53.1` lines and the open-advisory line were true at HEAD before this chunk; this chunk changed the facts they state (Counts / qualifiers moved), it falsified none.
- **Expected amendments (from plan):**
  - architecture §Stack and Technologies — the Backend framework row's "Tokio 1.53.1 built only inside `mcp`" → 1.53.3: carried — Dependencies; `tokio-ver` 1 hit, `.andromeda/architecture-detail/stack-and-technologies.md:11`.
  - architecture §Inherited Defaults — "Tokio 1.53.1 only in `mcp`" → 1.53.3: carried — Dependencies; `tokio-ver` 1 hit, `.andromeda/architecture.md:442`.
  - security-plan §Dependency Security — the `deps-supply-chain` open-advisory line retired, closed by this chunk: carried — Retired, Counts / qualifiers moved; `rustsec-0386` ×2 and `owner-title` 1 on `.andromeda/security-plan-detail/dependency-security.md:58` (also `tokio-ver` 1 on that line).
  - security-plan §Dependency Security — the `deps` group's "tokio `=1.53.1`" → `=1.53.3`: carried — Dependencies; `tokio-ver` 1 hit, `.andromeda/security-plan-detail/dependency-security.md:14`.
  - test-plan §8 Mocking & Stubbing Discipline — "tokio 1.53.1 `test-util`" twice → 1.53.3, the "in no manifest yet" qualifier kept: carried — Dependencies; `tokio-ver` 2 hits, `.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8,17`.
  - test-plan §7 Test Data & Fixtures — "tokio `=1.53.1`" → `=1.53.3`: carried — Dependencies; `tokio-ver` 1 hit, `.andromeda/test-plan-detail/test-data-fixtures.md:21`.
  - test-plan §3 Test Harness Contract — the Bootstrap phases key file's tokio version: carried — Dependencies; `tokio-ver` 1 hit, `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:24` (a key file of test-plan).
  - The Tier-3 docs `.claude/docs/stack.md:10,47` and `.claude/docs/services/viola-mcp.md:11`: carried — Dependencies; `tokio-ver` 3 leaf rows (the cascade's leaf step, not a master).
- **Coverage of new surfaces:** none — no new external surface, hot-path op or UI element.

## Deviations from intent
- none against the plan's steps and acceptance. P3 smoke was recorded from the P2 block rather than re-driven by hand: the plan's `role = 'smoke'` session (`agent-run.sh cleanup` → `boot --session p-trb-smoke --instance builder` → `status` → `g2-zero-panics.sh` → `schema-check` → `cleanup`) ran green inside the gate block; status read `"state":"ready"` with `builder` live / idle / `cli_verified:true` (implement's outcome, `.andromeda/runs/2026-10-26T06-05-40-implement/outcome-2026-10-26-tokio-rustsec-2026-0386-lock-bump.md`).
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 2 · listed 2 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 367`).

## Decisions & corrections
- The operator's word for this chunk's operator pass: "please run implement per the skill on 2026-10-26-tokio-rustsec-2026-0386-lock-bump, through its operator pass and the CI read; supply-chain is expected to turn green with it. Journal each long run in the run dir before you wait on it." — the operator, live, 2026-10-26 (`inputs#I6`). The implementer fired the pass (`.claude/rules/ci.md` §The operator pass: "The operator fires it, or the implementer does on the operator's word given for that chunk"); each long run was journaled before its wait (`.andromeda/runs/2026-10-26T06-05-40-implement/journal.md`).
- The operator's word for this wrap: "please run the wrap per the skill on 2026-10-26-tokio-rustsec-2026-0386-lock-bump, whole, as an ordinary chunk with no forced sample. The 85 % and 90 % context alarms are expected; finish the wrap, stop only at 95 %." — the operator, live, 2026-10-26 (`inputs#I7`).
- A standing learning recurred and held: `.claude/rules/ci.md` 2026-10-13 (write nothing into the tree between the pre-CI commit and the push). The evidence file's sections 3–5 (commit, push, CI read) and the journal's push line were written after the push landed; the push entry's `git diff --quiet` guard passed (`history moved: refs/remotes/origin/build/viola-0.1.0 b1249fb1→948f7f01`). Evidence `chunks/2026-10-26-tokio-rustsec-2026-0386-lock-bump/evidence/operator-pass.md`.
- An ordering met in the operator pass: the hygiene re-read "right before the commit" (ci.md step 2) was taken after `evidence/operator-pass.md` sections 1–2 were written, so its own line could not be in the commit without a further read; it was recorded after the push, the file saying so ("Sections 3–5 of this file were written after the push"). Evidence: the two hygiene lines (`read 356 … evidence 0` then `read 357 … evidence 1`) in that file.
- P3 smoke recorded, not re-driven: the plan's smoke entries are ordinary (non-`leg`) entries of the P2 block, so a hand `run` + `status` would have booted a second session for the same check (implement P3, "a `leg` entry ran at P2 — P3 does not re-drive it", read by analogy for in-block smoke entries). Evidence: implement's outcome, Deviations.
- Implement's gate header read `walk-class rust 4 (…) · uncommitted 360`; no deferral was in play (the rust delta `Cargo.toml` + `Cargo.lock` ran every gate), so the walk-class void had nothing to void.

## Outcome
- Acceptance, re-asserted against the diff (`Cargo.toml:242`, `Cargo.lock:1633,1635`):
  - (security) `cargo deny check` exits 0 and its output lacks `RUSTSEC-2026-0386`; the ignore list unchanged (`deny.toml` untouched, the guard) — MET.
  - (arch) the pin reads `tokio = { version = "=1.53.3", default-features = false }` (entry `last line 1`), the lock resolves `version = "1.53.3"`, the lock delta is 4 lines — MET.
  - (arch) tokio containment: the four sole-root bans exit 0, `channel-tokio-free: ok`, `deny-probes: 13/13 banned, control clean` — MET.
  - (tests) the fuzz lockfile audit exit 0, zizmor exit 0, `rmcp-features: server transport-io` — MET.
  - (tests) fmt, clippy host and `x86_64-pc-windows-msvc`, `RUSTUP_TOOLCHAIN=1.96 cargo check --workspace` green — MET.
  - (tests) `agent-run.sh run --unit` `"ok":true`; `agent-run.sh pre-push` `"ok":true` at stage `linux-tests` (coverage suite passed 3006 · failed 0; gate `breaches []`) — MET.
  - (obs) `g2-zero-panics.sh` and `agent-run.sh schema-check` `"ok":true` on the booted smoke home and the kept homes — MET.
  - (tests) the guard reads `guard: clean` — MET.
  - (tests) CI on the pushed HEAD `verdict: green` over the full row, `supply-chain` included — MET, ci#40278510123.
  - No verification-matrix capability claimed (`matrix.py show --chunk …` → `claimed by …: 0`; `v1-01` pooled) — MET.
- Gates (implement run `.andromeda/runs/2026-10-26T06-05-40-implement/`, summary `entries 27 · green 23 · red 0 · recorded 1 · timeout 0 · not-run 3`):
  - `df -h /tmp` — recorded, exit 0: `tmpfs 32G 6.7G 25G 22% /tmp`.
  - `cargo fmt --all --check` — green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` — green · `RUSTUP_TOOLCHAIN=1.96 cargo check --workspace` — green.
  - `grep -c '^tokio = { version = "=1.53.3", default-features = false }$' Cargo.toml` — green, `last line 1` · `grep -A1 '^name = "tokio"$' Cargo.lock` — green, `contains version = "1.53.3"` · `git diff -U0 b1249fb1… -- Cargo.lock | grep -c '^[-+][^-+]'` — green, `last line 4`.
  - `cargo deny check` — green, `exit 0` · `lacks RUSTSEC-2026-0386` · `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` — green · the sole-root ban loop over `scripts/sync-crates.txt` — green · `bash scripts/channel-tokio-free.sh` — green · `bash scripts/deny-probes.sh` — green · `bash scripts/rmcp-features.sh` — green · `zizmor --format=json .github/workflows/` — green.
  - `bash scripts/agent-run.sh run --unit` — green, `contains "ok":true`.
  - `gate.py guard … --allow '(Cargo\.toml|Cargo\.lock)' -- src crates tests …` — green, `last line guard: clean`.
  - smoke: `agent-run.sh cleanup --session p-trb-smoke` · `boot --session p-trb-smoke --instance builder` · `status --session p-trb-smoke` — green; `bash scripts/g2-zero-panics.sh` · `bash scripts/agent-run.sh schema-check` — green; `cleanup` — green, `processes_gone:true`, `endpoint_gone:true`.
  - `bash scripts/agent-run.sh pre-push` — green (90.89 s).
  - `gate.py hygiene` (`leg = 'operator'`) — the operator pass: `hygiene: clean — read 356 …`, re-read `read 357 … evidence 1` (`evidence/operator-pass.md` §2).
  - `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — the operator pass: green, `history moved: refs/remotes/origin/HEAD b1249fb1→948f7f01; refs/remotes/origin/build/viola-0.1.0 b1249fb1→948f7f01` (§4).
  - `ci.py conclusion --sha HEAD --wait 2400` (`leg = 'operator'`) — the operator pass: `948f7f015ba9 verdict: green · checks 15/15 · wall 778 s · runs ci#40278510123 completed/success` (§5).
  - Deferrals: none. Smoke: boot-path booted and torn down clean (above).
- Watches: none folded.
- Outcome basis: the operator pass ran — Setup 4's commit list `948f7f01` (the pre-CI commit, no fix commit) and the final HEAD's CI run ci#40278510123, recorded in `chunks/2026-10-26-tokio-rustsec-2026-0386-lock-bump/evidence/operator-pass.md`; the pass's `pre-push` re-run green (88.05 s) on the uncommitted tree before the commit. Implement's P4 report is held whole in this window and persisted as `.andromeda/runs/2026-10-26T06-05-40-implement/outcome-2026-10-26-tokio-rustsec-2026-0386-lock-bump.md` (the one `*-implement/` dir of this marker).
- Process hygiene: implement P4's census — the `p-trb-smoke` wrapper (pid 194411 at status) and its fake agent, started by a P2 entry, terminated by the cleanup entry; the cargo / nextest children of the block terminated. Re-measured here: `gate.py census --args projects/viola/target/debug/viola --args p-trb-smoke --comm cargo-nextest --comm fake-agent --comm viola-harness` → `census: none running` (this run dir's trail).
## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: b1249fb1 (the parent of the oldest pre-CI commit 948f7f01) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 2 line(s) in 2 range(s)
added: 1633 · 1635
### Cargo.toml — added 1 line(s) in 1 range(s)
added: 242
