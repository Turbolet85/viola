@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — set the supply-chain trust boundary: `deny.toml` with its four families, the sole-root `deny-sync.toml` tokio ban, the ubuntu `supply-chain` job, and the weekly advisory run as its own `nightly.yml` workflow (schedule + dispatch, no cache). This is the map of every config and job that reads advisories, which part A must re-measure (root graph, `deny-sync.toml` runs, nightly advisories).

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — made `fuzz/` a separate test-only workspace whose `fuzz/Cargo.lock` sits outside `cargo deny`. The exemption holds only while it is never linked into a shipped binary and never joins the root workspace, and both lockfiles are committed. This is the ground of the fuzz-workspace-exemption rows. The chrono ignore must not assume that root `deny.toml` changes reach the fuzz graph's families.

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — wired the fuzz lockfile audit: the `supply-chain` step `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` and weekly `nightly.yml` `cargo deny --manifest-path fuzz/Cargo.toml check advisories`. Both run from the repo root, where cargo-deny resolves the root `deny.toml`. Trap for part A: the root ignore list is read by the fuzz runs too, so whether the fuzz graph reaches chrono decides whether the ignore is needed or stays unused there. The entry also added the release-check refusal of test-only binaries to the CI job list.

@history_item n=4 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** — admitted two per-crate `0BSD` licence exceptions, never through `allow`, and set the rule that each further exception needs its own entry. This is the precedent for keeping the RUSTSEC-2026-0543 ignore to one id with its own reason, and no wider than that.

@history_item n=5 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (founder ratification of three standing boundary widenings) — admitted the `FAKE_AGENT_PUMP_DELAY_MS` seam and the unscanned `supply-chain` artifact (`deny.json`, `deny-fuzz.json`, `zizmor.json`). It also set the standing rule that a boundary widening halts for a live founder answer, and that an earlier delegated direction never ratifies one. The widened unmaintained-advisory rule rests on the founder's own relay, and the wrap records it as his.

@history_item n=6 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — added the `FAKE_AGENT_PUMP_DELAY_MS` test-seam row: `fake-agent` only, capped at 5 000 ms, compiled out of release, configuring nothing. It also added the Universal NEVER: no other seam without its own Decisions Log entry. Part B's tie and census must use existing seams. A new env or feature seam to drive booted-process teardown would be a widening.

@history_item n=7 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC`) — added the second fake-agent-only seam, closed value `1`, and widened the seam ban to name any other seam or G2 exemption as needing its own entry. This is the constraint behind decision/fake-agent-hook-panic-seam and test-seams-compiled-out.

@history_item n=8 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — made verify's probe captures transient 0600 files under the 0700 `ledger/probes/<pid>/`, removed whole by a drop guard. It added the hidden `hook --capture` arm (no `VIOLA_*`, no channel, exit 0) and moved the CI line to: `verify` runs in CI only against the fake agent, its real-CLI probe only locally. Part B's kill of the tied stamping verify is a fake-agent CI test, and its "nothing left" read touches these probe dirs.

@history_item n=9 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — removed the CI and pre-push mutation legs on the founder's ruling, so mutation belongs to the epoch-boundary audit (decision/mutation-testing-at-audit). This chunk's boundary says no mutation run.

@history_item n=10 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — recorded release-check's refusal of `test-support` / `fake-agent` artifacts (probe 5/5). This is the guard behind decision/release-build-no-test-binaries: any tie or census code part B adds must stay test-feature-only, or the release build refuses it.

@history_item n=11 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — moved the in-force Decisions Log items into the body. `scripts/release-check.sh` refuses any executable other than `viola` and any `test-support` / `fake-agent` artifact, and `test-support` is enabled only by the root package's `[dev-dependencies]`. Part B's harness changes must keep that feature placement.

@history_item n=12 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — set verify's interactive children as direct spawns under the R8 strip at 80×24, pasting only compiled literals, with the probe dirs 0700 and removed on every exit path. This is the origin of decision/verify-children-direct-spawn. A test that kills the tied verify must find these children and dirs gone, and must not drive the real CLI.

@history_item n=13 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — added the NEVER: a test-side removal under `target/e2e-home` takes only dirs the test side created (owner record, own `TempDir`, `viola-session-` name). The two keepers follow a linked base only to an absolute owner-only real directory, and `viola` never touches the backing. It also flagged a mutation trap in the keeper. Part B's cleanup of booted processes and their homes must stay within these removal rules.

@history_item n=14 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — made verify's probe dirs removed on pass, fail or SIGINT/SIGTERM through `cmd::verify::stop`, with its guards removing only what the run created, and no remover of an earlier run's dirs. SIGKILL, a crash or a failed handler install leave the dirs to be removed by hand. Key constraint for part B: a SIGKILLed tied verify is an accepted leftover case, so "nothing of it is left" must be scoped to processes, or the test owns the dirs it reads.

@history_item n=15 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added Run E to verify's child set (resumes only Run B's first SessionStart session id; none spawns nothing). This is the origin of decision/verify-run-e-resume-target, part of what a killed tied verify may have running.

@history_item n=16 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — narrowed the audit tool list to cargo-deny, zizmor and the deferred cargo-audit. `[[bans.features]]` now holds the rmcp ban alone, and nightly runs `advisories` and `fuzz`. Every remaining ban is proven live by `deny-probes.sh`. This is the current shape of the gates part A touches.

@history_item n=17 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (tokio ban tree form and rmcp release-graph gate) — brought in rmcp and exact pins tokio `=1.53.1` and schemars `=1.2.2` through `viola-mcp`, scoped rmcp's features to `server` + `transport-io` (checked by `scripts/rmcp-features.sh`), and moved `viola-channel` out of the sync-crate list. rmcp 3.4.1 and schemars 1.2.2 are the upstream crates that bring chrono in, so the ignore's reason must name them.

@history_item n=18 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — landed the CLI home resolution (`--home`, else the `VIOLA_NAME`/`VIOLA_DIR` instance home, else `~/.viola`, canonicalised, nothing created), unratified at the time. This is the origin of decision/cli-home-resolution-order and contract/cli-state-root rows, which pick the home a booted test drives.

@history_item n=19 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — the founder ratified the `VIOLA_DIR` step and canonicalisation, `mcp` included. decision/cli-home-resolution-order now stands as ratified.

@history_item n=20 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the e2e-home keepers also refuse a linked base below the 9 GiB headroom floor, with a fixed message naming no path. This completes decision/e2e-home-link-guard, which booted tests rely on for their homes.

@history_item n=21 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — closed an advisory red on `supply-chain` (RUSTSEC-2026-0386, tokio) by a lock bump with `deny.toml` untouched and no ignore widened, and retired the open-advisory item. This is the precedent for a safe-upgrade advisory. Part A differs: chrono has no safe upgrade, hence the founder's widened ignore rule.

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the `serde_json` `unbounded_depth` row to `deny.toml`'s `[[bans.features]]`, proven by a `deny-probes.sh` row and a test reading `deny.toml`, and set the deny-probes verdict at `14/14` with its counting rule. These are the last `deny.toml` edits before this chunk. Any test or probe that reads `deny.toml` text must stay green after the ignore and comment edit.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log) — `update_stamps` checks before creating `ledger/` and again under its lock, and `verify` checks the home, `ledger/`, `bin/` and `plugin/` before its log. This is the origin of decision/stamps-checked-under-lock and verify-home-tree-check, which govern the stamping verify that part B kills mid-run.

@history_item n=24 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — the pinned `viola` is placed once by `place_private_new`, never replaced. A copy another start placed is re-hashed and, on mismatch, left as found, never overwritten or deleted. This backs decision/pinned-artefacts-never-deleted: cleanup after a killed booted wrapper or verify must not remove pinned binary or plugin folders.

@history_item n=25 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — built the socket-directory check at the bind and the three client seams (one shared `check_endpoint_dir`, which never chmods), and the start's check before any snapshot, child or bind. It placed the socket in the per-user 0700 dir, never the tmp root. A killed wrapper's leftover socket file is kept by choice: nothing removes it, and `harness/cleanup.rs` `unconnectable` removes no leftover file. It added the NEVER on an env, flag or feature seam forcing a peer-euid refusal. Part B's killed wrappers leave socket files that its "nothing left" read must expect. Test sockets go under the short `/tmp` root inside such a dir.

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — CI became one `ubuntu-latest` leg, and the endpoint a Unix socket alone in `$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`. A licence exception naming a crate absent from the graph is dropped, never kept, and `deny-sync.toml` runs for one triple. An unmaintained advisory (RUSTSEC-2026-0441, pin-utils via portable-pty) read red and was owned by a CARRY with no ignore widened. Part A has the same class (unmaintained, transitive) and resolves it the other way, under the founder's widened rule.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution) — `viola run` resolves a given path as given and a bare name on PATH to an absolute path, else `ProgramNotFound`. This is the origin of decision/run-resolves-absolute-program, which applies to the fake agent that booted tests spawn.

@history_item n=28 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — `[advisories]` reads `ignore = []`, and a new ignore follows the Critical CVE response SLA with a `reason` naming the disabled feature. RUSTSEC-2026-0441 was closed by portable-pty `=0.9.0`, and the CVE SLA's remedy became bump the crate (no swap path). Part A's ignore is the first under a non-disabled-feature ground, so the comment must state both grounds: the SLA wording speaks only of a disabled feature.

@history_item n=29 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Dependency Security: 5 rows) — the ignore list went from empty to three reasoned rmcp ignores (decision/rmcp-advisory-ignores, ratified by the founder's F1 «Утвердить ignore»). decision/ci-cargo-deny-gate now fails on any advisory not on its reasoned ignore list. decision/cargo-deny-gate-clean and no-ignored-advisories were retired, and the rustix floor was added. These are the rows part A extends. The current `deny.toml` comment above the ignores states only the disabled-feature ground; the widened unmaintained ground becomes a new row or a `~` of rmcp-advisory-ignores / ci-cargo-deny-gate at the WRAP.

@history_item n=30 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — widened decision/test-removal-own-dirs-only with a fourth way: another build's stamp dir whose lock it takes with `try_lock` (landed provisional). Any sweep part B adds for leftover booted-test state must fit one of the four listed ways.

@history_item n=31 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Security Anti-Patterns: decision/test-removal-own-dirs-only state) — the founder ratified the stamp-dir `try_lock` removal, so the four-way removal rule stands. A fifth way of removing test-side dirs would need its own ruling.
