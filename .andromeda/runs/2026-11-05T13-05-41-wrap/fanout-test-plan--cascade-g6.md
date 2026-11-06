@proposal n=1 detector=D-retired severity=warning file=test-data-fixtures anchor=fct-bxlupfu3 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 7. Test Data & Fixtures
change: | On-disk product state (`events.ndjson`, `snapshot.json`, `heartbeat`, `plugin/`, `settings.json`) | Produced only by running real `viola run` / `hook` / CLI verbs against `viola-fake-agent` in a fresh home, never hand-written. Three exceptions: the chaos tests, which corrupt files the product already wrote; the server-identity impostor cases (below); and the MCP handler suite's own snapshots (`crates/viola-mcp/tests/handlers.rs`, `record_live` below). A home made with `create_dir_all` before viola is refused (0755) by the read rule. A case that needs a home viola created starts from `stamped_home`, as the `hook_statusline` cases that run the arm's whole course do. | per-test fresh (TempDir) |
sidecar: §7 On-disk product state — the Windows x64 `seed_conpty` exception and the windows-2025 `Writable` reading of a plain home retired with the ConPTY sideload and the Windows strict-modes arm (Linux only, founder rulings 2026-11-04; code at tag `multi-os-final`).
rationale: the report's Retired bullet names the ConPTY sideload and the Windows home owner/DACL strict-modes arm; Symbols / APIs: `tests/support/home.rs` loses `seed_conpty`.
@proposal n=2 detector=D-retired severity=warning file=test-data-fixtures anchor=fct-oanlcr47 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Test Data & Fixtures
change: remove the row «A widened seeded home (Windows x64)» whole: `seed_conpty`, its shared `target/conpty-seed/<key>/` companion and the `icacls` grant it measured are retired.
sidecar: §7 — the widened-seeded-home row (Windows x64) retired with `seed_conpty` and the `icacls` arms.
rationale: report Symbols / APIs: `tests/support/home.rs` loses `seed_conpty` and the `icacls` arms; Retired: the ConPTY sideload with `target/conpty-seed/`.
@proposal n=3 detector=D-cascade severity=warning file=test-data-fixtures anchor=fct-652w7iir basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Test Data & Fixtures
change: in the statusline-source row, «`Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child» reads «`Wrapper::boot_as_user`, which sets `HOME` on that one child»; the rest of the row unchanged.
sidecar: §7 statusline source — the Windows `USERPROFILE` arm of the default-home child dropped (Linux only).
rationale: report Symbols / APIs: test support loses the `USERPROFILE` arm; Counts `os-target` class: a per-OS arm on a Linux-only product is stale wording.
@proposal n=4 detector=D-retired severity=warning file=test-data-fixtures anchor=fct-jfjmhkwu basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Test Data & Fixtures
change: | A test-owned socket root | `tests/support/home.rs`, under `/tmp`: `SOCKET_ROOT_VAR` (`XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`, the last there and not in `tests/support/start.rs` | per-test |
sidecar: §7 socket root — `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR` alone; the macOS `TMPDIR` arm retired.
rationale: report Symbols / APIs: `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR`; Retired: the macOS `$TMPDIR/viola/` socket dir.
@proposal n=5 detector=D-retired severity=warning file=test-data-fixtures anchor=fct-t5v77dkt basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Test Data & Fixtures
change: | Widened test homes (the strict-modes negatives) | Set by the test on the real file, never by viola: `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes). | per-test |
sidecar: §7 widened test homes — the Windows `icacls` arm of the four helpers retired.
rationale: report Symbols / APIs: `tests/support/home.rs` loses the `icacls` arms of `widen` · `grant_read` · `unwiden` · `still_widened`.
@proposal n=6 detector=D-cascade severity=warning file=test-data-fixtures anchor=fct-myaghpwm basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Test Data & Fixtures
change: in the recorded-hook-payloads row, «and on Unix Run B's probe status line as `StatusLine.default-1.json`» reads «and Run B's probe status line as `StatusLine.default-1.json`»; the rest of the row unchanged.
sidecar: §7 recorded hook payloads — the «on Unix» qualifier on Run B's status-line recording dropped (the pass-through runs on every build).
rationale: report Symbols / APIs: `WRAPPED_HERE` folded and `statusline::shell_argv` returns `[&str; 3]` (was `Option`), so the status line path is no longer Unix-bound; the Counts `os-target` class.
@proposal n=7 detector=D-retired severity=warning file=test-data-fixtures anchor=fct-rgx56qfp basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 7. Test Data & Fixtures
change: - The server-identity impostor cases (`served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves, recording a held live process's pid (the fake agent's holder, below) beside an endpoint the test binds in a private directory (`viola_state::fs::create_private_dir`, below), so that a server that is not the recorded wrapper answers the frame-writing clients.
sidecar: §7 impostor cases — the «Linux and Windows» qualifier dropped; the cases run unconditionally.
rationale: report deviation 11: the hook-fail-open and handlers' impostor cases and `security_negatives_server_identity.rs` now run unconditionally; Retired: the Windows named-pipe endpoint.
@proposal n=8 detector=D-retired severity=warning file=test-data-fixtures anchor=fct-wmieczsn basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Test Data & Fixtures
change: - `crates/viola-mcp/tests/handlers.rs` `record_live(instance, endpoint, pid)` writes an `InstanceSnapshot` recording an endpoint no one ever bound, for the unserved row of `handlers_frame_tools_name_the_cause_of_an_unreachable_instance`.
sidecar: §7 `record_live` — the Windows `Serving` reason (a pipe living in the accept thread) retired with `server/win.rs`.
rationale: report Symbols / APIs: `viola-channel`'s `server/win.rs` and the named-pipe arms are removed, so the reason «a Windows `Serving` has no `Drop`» names no code; the owner confirms the row's unbound endpoint still serves it.
@proposal n=9 detector=D-retired severity=warning file=test-data-fixtures anchor=fct-ubb4ig4c basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 7. Test Data & Fixtures
change: in the home-made-before-viola item, «A home planted under a regular file takes a per-OS oracle: on Unix it is a strict-modes refusal, while on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`).» reads «A home planted under a regular file is a strict-modes refusal (`verify_under_a_regular_file_refuses_the_home`).»; the rest of the item unchanged.
sidecar: §7 — the per-OS oracle of a home planted under a regular file narrows to its Unix strict-modes refusal; the Windows `NotFound` arm retired.
rationale: report Symbols / APIs: `strict.rs`'s Windows owner/DACL arm and the off-Unix arm of `check_socket_dir` removed, the Unix body unconditional; Retired: the Windows home owner/DACL strict-modes arm.
@proposal n=10 detector=D-retired severity=warning file=mocking-stubbing-discipline anchor=fct-pwscsozw basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 8. Mocking & Stubbing Discipline
change: | Fake servers | interprocess 2.4.4 server API (squatter listener; impostor listener); `viola-fake-agent` as the fake CLI | The squatter binds the computed endpoint before `run`; in the server-verification negatives an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener: `tests/security_negatives_server_identity.rs`, `tests/hook_fail_open.rs`'s `Impostor`; the MCP case is `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`), and the hook unit impostor varies the recorded start time instead (a second early, or an unparseable `started_at`), a root-bin unit test having no portable second process; a test server's endpoint directory is made through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()`, whose mode `tempfile` 3.27 leaves to the umask (0755 on a default one) and which the client's socket-directory check refuses ahead of the server check (`crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs`, `refused_inside` in `src/cmd/client.rs`); no HTTP fake servers |
sidecar: §8 Fake servers — windows-sys `CreateNamedPipeW` control pipes, the listener-admission negatives and `ImpersonateAnonymousToken` retired with the named pipe; the «Linux and Windows» qualifier dropped.
rationale: report Retired: the Windows named-pipe endpoint, its SDDL / SQOS / `accept_remote(false)` controls and the `windows-sys` dependency; Files: `tests/security_negatives_channel.rs` and `tests/channel_sqos_open.rs` deleted.
@proposal n=11 detector=D-retired severity=warning file=mocking-stubbing-discipline anchor=fct-zsbxg2jw basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 8. Mocking & Stubbing Discipline
change: | Filesystem faults | none (a real tempfile home, real modes) | A widened or read-granted home tree is made on the real file by the §7 widened-test-home helpers and refused by the product's own check, never a stubbed one; a loosened socket directory is made on the real file too, at 0755 or as a symlink to 0700 (`Form` in `tests/security_negatives_socket_dir.rs`), refused by the product's own check and read back as found, and the pure `socket_dir_verdict` takes a foreign owner's uid by injection at the unit tier |
sidecar: §8 Filesystem faults — the ACL wording, the windows-2025 grants, `windows_read_verdict`'s synthetic descriptors and the widened-seeded-home clause retired.
rationale: report Symbols / APIs: `windows_read_verdict`, the Windows owner/DACL arm and `seed_conpty` removed; Retired: the Windows home owner/DACL strict-modes arm and the ConPTY sideload.
@proposal n=12 detector=D-cascade severity=warning file=mocking-stubbing-discipline anchor=fct-6xaoh2kl basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 8. Mocking & Stubbing Discipline
change: | Peer euid decisions | none (a parameter of the decision or of the private function under test) | `peer_euid_admitted(observed, own)` takes its euids as arguments (admitting `observed == Some(own)`; an unreadable euid refused) and the private `serve_admitting` takes its decision as an `Admission` argument (`crates/viola-channel/src/server.rs`): dependency injection, with no env, flag or feature seam |
sidecar: §8 Peer euid decisions — `peer_euid_admitted` loses its `reported` parameter (`PEER_EUID_REPORTED` folded to its Linux value).
rationale: report Symbols / APIs: the constants `SERVER_PID_REPORTED` and `PEER_EUID_REPORTED` removed, `peer_euid_admitted(observed, own)` is `observed == Some(own)`, no seam added.
@proposal n=13 detector=D-retired severity=warning file=ci-integration anchor=fct-pk32flwi basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: in the Lint stage, the rmcp step reads «(after `Lint probes`; chunk 2026-10-17-mcp-server-for-drivers)» without «every OS,»; the two dev-host paragraphs (the check-only Windows-target clippy `--target x86_64-pc-windows-msvc` and the macOS-target clippy `--target aarch64-apple-darwin`) are removed; **Parallel:** reads «one-leg matrix `[ubuntu-latest]` (the job name keeps its `${{ matrix.os }}` form)»; Runs' other clauses and Cache unchanged.
sidecar: §9 Lint — the every-OS wording, the per-OS matrix and the dev-host Windows- and macOS-target clippy recipes retired (deny targets one triple; matrix `[ubuntu-latest]`).
rationale: report Harness / gate surface: `lint` keeps `strategy.matrix.os: [ubuntu-latest]`, `Lint probes` loses its `if: runner.os`; Counts `deny-triples`: the two test-plan ci-integration lint lines naming the Windows / macOS targets are stale.
@proposal n=14 detector=D-new-detail severity=warning file=ci-integration anchor=fct-3gkvok74 fact=F4 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-swb6jipj
section: ## 9. CI Integration
change: add under the Supply-chain stage, in its shape: «  - **Recorded red:** the `supply-chain` job reads red on one advisory, RUSTSEC-2026-0441 (`pin-utils` unmaintained, through `portable-pty =0.8.1` → `nix 0.25.1`); a CARRY on «Version done-check» owns it, its remedy moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils`; the ignore list is not widened.»
sidecar: §9 Supply-chain — the recorded RUSTSEC-2026-0441 red and its owner, the CARRY on «Version done-check» (no ignore widened).
rationale: report Cross-project claims: ci#42030025050 `supply-chain` failure on RUSTSEC-2026-0441 alone, owned by the CARRY on «Version done-check» (`inputs#I3`, `inputs#I4`), remedy `portable-pty` off `=0.8.1`, no ignore-list widening.
@proposal n=15 detector=D-retired severity=warning file=ci-integration anchor=fct-4otx4vxd basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: in the Unit stage, **Runs:** reads «`scripts/agent-run.sh run --unit`»; the pwsh `scripts/agent-run.ps1` clause removed; Parallel and Cache unchanged.
sidecar: §9 Unit — the pwsh `agent-run.ps1` arm retired; the harness is `scripts/agent-run.sh` alone.
rationale: report Harness / gate surface: `scripts/agent-run.ps1` deleted, the harness is `scripts/agent-run.sh` alone; Retired: the `{sh,ps1}` shim pair.
@proposal n=16 detector=D-retired severity=warning file=ci-integration anchor=fct-ijfdjbph basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: in the E2E stage, **Runs:** reads «`agent-run run --e2e` on `ubuntu-latest`» and **Parallel:** reads «one-leg matrix `[ubuntu-latest]`»; Cache unchanged.
sidecar: §9 E2E — «all 3 OSes» and the per-OS matrix narrowed to `ubuntu-latest` (the windows-2025 and macos-latest legs retired).
rationale: report Retired: the `windows-2025` and `macos-latest` CI legs; Counts: the CI matrix 3 OS legs → `[ubuntu-latest]`, `os-target` class «three OSes» stale.
@proposal n=17 detector=D-retired severity=warning file=ci-integration anchor=fct-sd3lvsrr basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: in the Coverage report stage, **Runs:** reads «`agent-run run --coverage` (cargo-llvm-cov 0.9.1 under nextest). In CI this is the `test` job (matrix `[ubuntu-latest]`). Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the `test` job uses `coverage,doctest`, and the Perf, Fuzz replay and MSRV jobs use `perf`, `fuzz-replay` and `nextest-unit`»; **Parallel:** reads «one-leg matrix `[ubuntu-latest]`»; Cache unchanged.
sidecar: §9 Coverage report — the windows-2025 `ConPTY vendor verification` step, its probe and the `conpty_sideload` binary retired; the per-OS job is the one-leg `test` job.
rationale: report Harness / gate surface: the `ConPTY vendor verification` step leaves `ci.yml`, `test` keeps `[ubuntu-latest]`; Files: `tests/conpty_sideload.rs`, `scripts/conpty-vendor.sh` and `vendor/conpty/` deleted.
@proposal n=18 detector=D-retired severity=warning file=ci-integration anchor=fct-ixlw7x3t basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: in the Mutation stage, **Runs:** reads «no push or pull-request job and no gate since 2026-09-28: mutation testing runs at the epoch boundary through `/andromeda-code-audit`, and on demand through `agent-run run --mutants` [`--file`] (§3 `run` step 4, §10 Mutation gate)»; the Windows-leg sentence (`windows-mutants.yml`, its ten `mutants (<label>)` jobs on `windows-2025`) removed; Parallel and Cache unchanged.
sidecar: §9 Mutation — the boundary audit's Windows leg, `windows-mutants.yml`, retired.
rationale: report Retired: `windows-mutants.yml` (the boundary audit's Windows leg) with `viola-mutants-scratch` / `HOST_SCRATCH`; Harness / gate surface: `windows-mutants.yml` deleted.
@proposal n=19 detector=D-tests-count-literals severity=warning file=ci-integration anchor=fct-k7t43itf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 9. CI Integration
change: in the Quality-gates item, both workflow lists read «(`ci.yml`, `nightly.yml`)»; the sentence «`windows-mutants.yml` installs `tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0` with the same action, and `tests/contract_windows_mutation_scope.rs` asserts those two pins equal the `test` job's `tool:` line» removed; the zizmor clause reads «(2 low, one per workflow; zizmor 1.30.1, chunk 2026-11-05-linux-only-code-and-ci)»; the rest unchanged.
sidecar: §9 — workflows 3 → 2 (`windows-mutants.yml` and `contract_windows_mutation_scope.rs` retired); zizmor `concurrency-limits` declined findings 3 → 2.
rationale: report Counts: workflows 3 → 2 (`ci.yml`, `nightly.yml`), zizmor `concurrency-limits` suppressions 3 → 2 (implement entry 22, `(2 suppressed)`); Files: `tests/contract_windows_mutation_scope.rs` deleted.
@proposal n=20 detector=D-tests-count-literals severity=warning file=ci-integration anchor=fct-7d3yogoz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 9. CI Integration
change: in the `ci` kill-line item, «over 27 binaries» reads «over 26 binaries»; the rest unchanged.
sidecar: §9 `ci` kill line — `binary(…)` names 27 → 26 (`binary(conpty_sideload)` gone).
rationale: report Counts: nextest `ci` kill line `binary(…)` names 27 → 26 (`.config/nextest.toml` line 24), test-plan/ci-integration fct-7d3yogoz «over 27 binaries» named the stale site.
@proposal n=21 detector=D-retired severity=warning file=ci-integration anchor=fct-fumv4pch basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: remove the item «The per-OS JUnit reader» whole: `scripts/junit-os-sets.py` and the three `junit-<os>` artifacts it compared are retired.
sidecar: §9 — the per-OS JUnit reader (`scripts/junit-os-sets.py`) retired with the windows-2025 and macos-latest legs.
rationale: report Retired: `scripts/junit-os-sets.py` and the three `junit-<os>` artifacts; Files: `scripts/junit-os-sets.py` deleted.
@proposal n=22 detector=D-retired severity=warning file=ci-integration anchor=fct-5iefujrn basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 9. CI Integration
change: remove the item «The per-OS JUnit reading» (the dated three-OS case counts at ci#41918694243): its reader and two of its three legs are retired, and the reading is history held at the tag `multi-os-final` and the amendments sidecar.
sidecar: §9 — the dated per-OS JUnit reading retired as current (three-OS history; Linux only).
rationale: report Counts `win-ci` class: a dated measurement on `windows-2025` / `macos-latest` that records history is re-worded or retired by its owner, never left as current; Retired: `scripts/junit-os-sets.py`.
@proposal n=23 detector=D-retired severity=warning file=body anchor=fct-54gkcbby dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 7. Test Data & Fixtures
change: - **On-disk product state:** produced only by real `viola` runs against the fake agent in a fresh home, never hand-written — except the chaos tests' corruption of files the product already wrote and the server-identity impostors' test-built `snapshot.json`; a case that needs a home viola created starts from `stamped_home`, and a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: a plain home is one the product refuses by the read rule.
sidecar: §7 seed strategies — the Windows x64 `seed_conpty` exception and the windows-2025 `Writable` reading retired.
rationale: the broad line test-data-fixtures#1.1 details; report Retired: the ConPTY sideload and the Windows home owner/DACL strict-modes arm.
@proposal n=24 detector=D-retired severity=warning file=body anchor=fct-hkmaq6ye dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 8. Mocking & Stubbing Discipline
change: - Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
sidecar: §8 Filesystem — the synthetic SDDL descriptor stubs for foreign SIDs and non-persistent-ACL volumes retired with the Windows strict-modes arm.
rationale: the broad line mocking-stubbing-discipline#2.6 details; report Symbols / APIs: `windows_read_verdict`, `Refused::NullDacl`, `Refused::NoPersistentAcls` removed.
@proposal n=25 detector=D-retired severity=warning file=body anchor=fct-346brcpe dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: - **Lint:** fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, on `ubuntu-latest`; the `--acyclic` review is not a gate.
sidecar: §9 Lint — «per OS» and the dev-host Windows- and macOS-target clippy runs retired.
rationale: the broad line ci-integration#1.1 details; report Harness / gate surface: `lint` keeps `[ubuntu-latest]`; Counts `deny-triples`.
@proposal n=26 detector=D-retired severity=warning file=body anchor=fct-onifqrhg dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: - **Coverage report:** `agent-run run --coverage` is the `test` job, on `ubuntu-latest`; its one instrumented nextest run replaces the unit, integration and E2E rows, and each job ends in `viola-harness gate --require <suites>`.
sidecar: §9 Coverage report — the windows-2025 ConPTY vendor verification retired; the test job runs on `ubuntu-latest`.
rationale: the broad line ci-integration#1.6 details; report Harness / gate surface: the `ConPTY vendor verification` step leaves `ci.yml`.
@proposal n=27 detector=D-retired severity=warning file=body anchor=fct-dk3rsyuy dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: remove the build-failure item «ConPTY vendor errors (`windows-2025`)» whole.
sidecar: §9 build failure conditions — ConPTY vendor errors retired with `scripts/conpty-vendor.sh`.
rationale: the failure condition ci-integration#1.6's vendor step details; report Retired: `scripts/conpty-vendor.sh` and the `ConPTY vendor verification` CI step.
@proposal n=28 detector=D-retired severity=warning file=body anchor=fct-fo2372wn dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: - **Mutation:** no push or pull-request job and no gate — mutation runs at the epoch boundary through `/andromeda-code-audit` and on demand through `run --mutants`.
sidecar: §9 Mutation — the Windows leg `windows-mutants.yml` retired.
rationale: the broad line ci-integration#1.7 details; report Retired: `windows-mutants.yml`.
@proposal n=29 detector=D-tests-count-literals severity=warning file=body anchor=fct-3x7gvt6f dependent_of=D-tests-count-literals basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 9. CI Integration
change: **Platform:** reads «GitHub Actions (one `ci.yml` on push and PR and a scheduled `nightly.yml` for fuzz and weekly advisories). There is no deploy stage in v1.»; the **Caches:** line unchanged.
sidecar: §9 Platform — workflows 3 → 2; the dispatch-only `windows-mutants.yml` retired.
rationale: the broad line ci-integration#1.12's workflow list details; report Counts: workflows 3 → 2 (`ci.yml`, `nightly.yml`).
@proposal n=30 detector=D-retired severity=warning file=body anchor=fct-2ymlhohm dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 9. CI Integration
change: - `mutants.out/` is never uploaded (no push or pull-request job runs mutation since 2026-09-28): its `outcomes.json` holds absolute argv paths and its `log/` holds test output.
sidecar: §9 test report format — the `windows-mutants.yml` clause of the `mutants.out/` rule retired.
rationale: the upload rule ci-integration#1.7's report-only Windows leg detailed; report Retired: `windows-mutants.yml`.
@proposal n=31 detector=D-retired severity=warning file=body anchor=fct-shyrfs6v dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 9. CI Integration
change: - OS: `ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs), the one leg of `strategy.matrix.os` in `test`, `perf`, `lint` and `release` (every job and artifact name keeps its `${{ matrix.os }}` form); the Windows and macOS code is recoverable at the tag `multi-os-final`. `fail-fast: false`.
sidecar: §9 Matrix builds — OS legs 3 → `[ubuntu-latest]`; windows-2025 and macos-latest retired.
rationale: the matrix ci-integration#1.5's per-OS E2E details; report Harness / gate surface: `test`, `perf`, `lint`, `release` keep `strategy.matrix.os: [ubuntu-latest]`; Retired: the `windows-2025` and `macos-latest` legs.
@proposal n=32 detector=D-retired severity=warning file=body anchor=fct-q4zg6vp5 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 9. CI Integration
change: - Any unit / integration / E2E / contract / property test fails
sidecar: §9 build failure conditions — «(on any of the 3 OSes)» dropped (Linux only).
rationale: the failure condition of ci-integration#1.5's E2E stage; report Counts `os-target` class: «three OSes» stale.
@proposal n=33 detector=D-retired severity=warning file=body anchor=fct-6iojik4n dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 9. CI Integration
change: - Coverage falls below threshold (Section 10)
sidecar: §9 build failure conditions — «on any OS» dropped (the coverage job is the one-leg `test` job).
rationale: the failure condition of ci-integration#1.6's coverage job; report Counts `os-target` class: an every-OS clause is stale wording on a Linux-only product.
@remark
Not this group's, same retirements: §9 body Perf fct-3q2uf724 «per-OS job» (no picked Perf entity); integration-test-strategy §5 seed_conpty carve-out, DA1 answer and Windows DACL read-back; e2e-test-strategy §6 skipped list and chaos .cmd case; quality-gates §10 per-OS gating; key bootstrap-phases-derive-for-route-setup-project fct-2j6rht5l still lists conpty_sideload (kill-line 26); architecture/security-plan carry the same zizmor and workflow counts.
