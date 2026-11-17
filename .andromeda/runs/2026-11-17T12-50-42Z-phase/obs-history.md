@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — put the weekly `cargo deny check advisories` run in a scheduled `nightly.yml` beside the single push/PR `ci.yml`. Part A has to re-read this nightly advisories run alongside the `supply-chain` job: RUSTSEC-2026-0543 reaches both, so one `deny.toml` ignore has to cover both.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — `nightly.yml` runs the advisories check and the fuzz time-box ("only" advisories was wrong), and the unscanned uploads are admitted by their content. This is the nightly surface part A re-measures, and the fuzz graph it audits is separate from the root graph.

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — admitted the ci.yml `supply-chain` artifact (`deny.json`, `deny-fuzz.json` for the `fuzz/Cargo.lock` audit, `zizmor.json`) as unscanned by content, with the guard still binding: a member that could carry a host path or log content is scanned or dropped. Part A's change to the ignore list and its comment must not put a path or log content into those uploads. The `deny-fuzz.json` audit (`cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`) is one of the configs P3 must read for this id.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one CI leg, two workflows, one deny target and one harness shim) — `cargo deny` runs on the one target `x86_64-unknown-linux-gnu`, there is one CI leg `[ubuntu-latest]`, and the workflows are `ci.yml` plus `nightly.yml`. The OS-gated census also faults any gate predicate that names `windows` or `macos`. This is the current form of rows decision/obs-deps-pass-cargo-deny, decision/os-gated-test-census and decision/linux-only-os-target. Part A's deny edit is judged on that one target, and any new kill test in B must not add an OS gate the census would fault.

@history_item n=5 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated test census in the E2E layer) — added `tests/contract_os_gated_tests.rs`. Every OS-gated test function or file under `tests/` and `crates/*/tests/` must be named in a `GATES` row with a reason. Any new test in `crates/viola-e2e/tests/` that kills the tied verify falls under this census if it is gated (e.g. `cfg(unix)`).

@history_item n=6 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp sink is always mcp.ndjson) — built `viola mcp` as the rmcp stdio server on D-09 (row decision/rmcp-stdio-mcp-server). That crate, `viola-mcp`, is the path by which `rmcp 3.4.1` and `schemars 1.2.2` bring chrono into the graph. The ignore reason part A writes names those upstreams, and the obs plan states rmcp's place.

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap and the session it ends) — recorded that rmcp 3.4.1 reads stdin with an unbounded `read_until`, so viola wraps it in `CappedLines` at `MAX_FRAME`. This is a precedent for handling an upstream rmcp shortcoming at viola's own boundary without replacing the crate, which matches part A keeping rmcp and chrono and admitting only the advisory.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the portable-pty pin at =0.9.0) — a dependency pin change was mirrored into the obs rows: the entity, the cli row and the logging-stack bullet each read the new version, and only the version moved. This is the precedent for how a dependency-side change shows up in this plan. Part A moves no version, so no row should change.

@history_item n=9 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's exact-path seam exemption) — G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first) in the test and perf jobs. It does not count only a panic whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>`. This is the basis of rows decision/zero-panics-gate, decision/g2-zero-panics-gate, decision/panic-count-home-level-only and decision/error-budget-zero-tolerance. A kill test in B must leave no panic line in a kept home, because no new exemption exists.

@history_item n=10 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the G2 probe's other-file case names its planted literal by its home, never as `path:N`. This is a standing rule for any plan prose in this chunk that cites G2's probe or planted cases.

@history_item n=11 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the G2 question answered: the chaos home outside the scans, F4) — integration homes live under `target/e2e-home/`, except the one `TestHome::outside_scan()` home of `tests/chaos_feed_panic.rs`. That is the named carve-out of rows decision/test-homes-under-e2e-home and decision/test-homes-under-scan-root. Any home that B's kill test boots, harness or root, must sit under the scan root: no second carve-out exists.

@history_item n=12 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — a vt100 panic is caught per feed on `run`'s feed thread and gives one `parse-rejected{vt100-feed, panicked}` plus the panic role line. This explains rows contract/parse-rejected-detail/panicked and decision/panic-caught-at-every-role, and it shows why the chaos home is the one exempt home.

@history_item n=13 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — no deleter may remove a home before G2, G4, the secret scan and the uploads read it: neither harness `cleanup` nor a test home's drop (`remove_owned`). CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home (rows decision/homes-kept-until-gates-run and decision/test-homes-kept-until-gates). B's "nothing of it is left" must mean no process is left, not that the kept home was deleted.

@history_item n=14 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — G2 reads `find target/e2e-home/` with the trailing slash so it descends a linked scope. On the dev host `target/e2e-home` links to an owner-only tmpfs. Trap: without the slash a local G2 reads its fail-closed `empty scope`, which matters for a local reading of B's homes.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (v1-47's secret-scan reading, provisional) — the scan runs after each job's end-to-end run, not after every test, with `AGENT_RUN_KEEP_HOMES` `1` on the job, the scan `if: always()`, and the `viola-chaos-*` home left out. B's new boots sit in that job and are scanned this way.

@history_item n=16 marker=2026-11-01T02-46-44-wrap
**2026-11-01T02-46-44-wrap** — the founder's word cleared the OPEN gap on `v1-47`'s secret-scan reading, so the after-each-job reading stands. It also made the §3 Log format copy verbatim again with test-plan's (row decision/obs-aligns-to-tests-log-format).

@history_item n=17 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — in CI `viola verify` runs only against the fake agent (harness `boot` step 4 unless `--unstamped`, `stamped_home`). The real-CLI verify runs only through `run --local-live`, which is refused under `CI`. `verify-probe` joined the spawn subjects. This is the origin of rows decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci and contract/verify-spawn-subject/verify-probe. The tied stamping verify that B kills is this boot step 4 fake-agent verify.

@history_item n=18 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome on stdout, the uninstrumented `hook --capture` arm, the `boot` readiness owner) — set verify's agent-readable outcome (exit code, stdout step lines, the `stamped` summary). It made the hidden capture arm uninstrumented, so a capture-arm process leaves no role line. It also assigned the `boot` step-4 and readiness ownership to the harness. This is the context for what a killed stamping verify leaves.

@history_item n=19 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by SIGINT/SIGTERM ends `process-exit{detail:"interrupted"}` after closing every spawn pair it opened, and it removes the probe dir on a pass, a fail or a stop. SIGKILL and a crash leave the probe dir. This is the main trap for B's "kill the tied stamping verify and read that nothing of it is left": a SIGKILL leaves the probe dir and writes no `interrupted` line. The proof has to say which kill it sends and what "nothing left" covers (processes, not the probe dir).

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms of the panic capture, the env strip, verify and the statusline retired) — a verify stop now reads `SIGINT` / `SIGTERM` only, and SIGKILL and a crash leave the probe dir. This is the current wording of the kill semantics B's kill test relies on.

@history_item n=21 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify's interactive PTY runs became five `verify-pty-probe` pairs (row contract/verify-spawn-subject/verify-pty-probe). A PTY child killed at a stop carries `child_exit_status` and `duration_ms`. A print-mode or `--version` child killed at the stop was not measured. This bounds what B may assert about the children of a killed verify.

@history_item n=22 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `verify-pty-probe` to the closed `subject` enum, one pair per interactive PTY run, and CI's fake-agent verify drives its interactive runs. This is the origin of row contract/verify-spawn-subject/verify-pty-probe, covering the child processes a tied verify owns.

@history_item n=23 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify logs eight child spawns, the eighth being one `agents-probe` pair around Run B's single agents read (row contract/verify-spawn-subject/agents-probe). B's kill proof has to account for every child the stamping verify can hold, this probe included.

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check and the catch site's narrowed reach) — verify runs `strict::check_home_tree` before its `cli` log opens, and a refusal exits 1 with the home pair and writes no line. A refused `update_stamps` exits through verify's own `process-exit`. This is the origin of row contract/viola-verify/home-tree-refused. A verify killed in B before or after this check leaves different traces.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check of `run` and `viola revive`'s start, before the wrapper log) — a refused home tree opens no log, starts no child and exits 1. This is the origin of row decision/home-tree-check-before-log-init. A booted supervisor in B that is killed must leave modes as found.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the pre-init window and the panic exemptions gain the chunk's checks) — exemption 8 and exemption 1's pre-init window cover verify's home tree and `run`'s home tree. These are the bounded exemptions named in rows decision/zero-unlogged-panics and decision/one-line-panic-record.

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — under a parsed `VIOLA_NAME` verify writes its refusal code on `process-exit`, beside `interrupted`. A refused `update_stamps` reads `strict-modes-failed`. This is part of row contract/viola-verify/home-tree-refused, and of what a stamping verify's log shows when it stops normally rather than by SIGKILL.

@history_item n=28 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — retired bounded exemption 6 but kept its number as a retired marker, so 7 and the citations keep their numbers. Rows decision/zero-unlogged-panics and decision/one-line-panic-record cite these exemptions, and any citation in the plan must use the standing numbering.

@history_item n=29 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's socket-directory step, no span and one line on a refusal) — a refused socket directory is one `process-exit{exit_code:1, detail:"socket-dir-refused"}` before any snapshot, child or bind, on `revive`'s start too. A bind failure keeps its own outcome (row contract/run-start-failure-detail/socket-dir-refused). This bears on B's boundary that a test socket goes under the short `/tmp` root: a booted supervisor whose socket directory is refused ends with this line, not a hang.

@history_item n=30 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — only `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]`, and the fake agent is a root-package bin with a crate-level print allow. This is the lint context for B's edits to `crates/viola-e2e/tests/harness_lifecycle.rs` and `crates/viola-e2e/tests/cli.rs` and the 20 s wait-bound lint.

@history_item n=31 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — `agent-run status` reads `state: degraded` when a booted instance's item is `stale` or absent. A supervisor that outlives a killed test, or a kill that leaves a stale instance, shows up through this reading.

@history_item n=32 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — default-deny is enforced by a top-level `unevaluatedProperties: false`, and `diag-detail.v1.json` is self-contained with the event enum inlined. This is the origin of rows decision/schema-top-level-unevaluated-properties and decision/detail-schema-self-contained, which G4 holds over every line a booted or killed process in B writes.

@history_item n=33 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — made `corr` required on the corr-bearing events in `diag-line.v1.json` and never on `hook-invoked`. This explains row contract/process-log-line/corr, "required only on the events whose rules always define it".

@history_item n=34 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the reserved web vocabulary leaves) — `process` is `run|hook|mcp|cli` (four roles), the role files are `run-<name>`, `hook-<name>`, `mcp` and `cli-<name>`, and the third-party OFF targets include `rmcp`. This is the origin of rows contract/log-line/process, contract/diag-line/process and contract/process-log-line/process.

@history_item n=35 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (3. Observability Harness Contract, `decision/obs-aligns-to-tests-log-format`) — narrowed the rule to "must not rename them or remove one the product still emits", by operator Q2. This is the current text of the picked row. B changes no field, but a test-plan log-format move would bind here.

@history_item n=36 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — every area of the master became rows, each verified against the code. The picked rows this chunk rests on (deps, panic, CI gates, verify subjects, test homes) take their current wording from this migration and the entries after it.
