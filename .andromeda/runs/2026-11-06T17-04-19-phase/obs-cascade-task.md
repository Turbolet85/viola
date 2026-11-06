@task_prose part=chunk
Closes viola-0.1.0: every gate green on Linux (fmt, clippy, check, test, cargo deny, the deny and lint probes, the tokio-free and release checks, CI's seven jobs), the CI red `supply-chain` read at Setup closed against its log.
Moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441), with no ignore-list widening.
Removes or keeps-with-reason the Windows remnants left in code, re-verifying their coordinates at HEAD, the masters following.
Re-measures and re-words master facts: the obs-plan log-coverage `process-start` row (`pty_backend` `openpty` alone, `sideload_fallback` gone), the `thiserror::Error` derive census (10 measured where obs-plan error-capture states 9), the one-enum-per-crate divergences (fold or ratify), and the channel-handle no-leak property on Linux.
Runs at most two live sessions on the headless pty rig (v1-53's live `viola run` on Linux plus one spare), and rules the 17 unclaimed capabilities as proved or deferred on the founder's word.
@task_file n=1 path=Cargo.toml mark=edits
what: the `portable-pty` dependency pin
@task_file n=2 path=Cargo.lock
@task_file n=3 path=crates/viola-pty/Cargo.toml
@task_file n=4 path=deny.toml
@task_file n=5 path=tests/cli_output_plain.rs mark=edits
what: the Windows remnants at lines 104 and 116
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the Windows remnant at line 2
@task_file n=7 path=crates/viola-e2e/src/harness/session.rs mark=edits
what: the Windows remnant at line 52
@task_file n=8 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=9 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=10 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=11 path=crates/viola-e2e/src/harness/gate.rs mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=12 path=crates/viola-e2e/src/harness/run/perf.rs mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=13 path=crates/viola-e2e/src/harness/run/coverage.rs mark=edits
what: the `COVERAGE_FLOORS` per-OS comment and the `COVERAGE_IGNORE` Windows-path comments
@task_file n=14 path=src/cmd/mod.rs mark=edits
what: `plain_drive`
@task_file n=15 path=crates/viola-e2e/src/harness/run/mutants/host.rs mark=edits
what: the `windows` test-host key
@task_file n=16 path=tests/contract_os_gated_tests.rs mark=re-reads
what: the census of OS-gated tests (39 functions, 2 gated files) that v1-53's claim reads
@task_file n=17 path=.github/workflows/ci.yml mark=re-reads
what: CI's seven jobs, `supply-chain` among them, all green at the wrap
@task_item n=1 part=needs
The obs-plan log-coverage `process-start` row (fct-yb5jpaot): its `pty_backend` values cut to `openpty` alone and `sideload_fallback` removed, the row held to the header's two cells, landed through the store's whole-fact form, then its CARRY retired.
@task_item n=2 part=needs
The obs-plan error-capture platform pick and its count of thiserror-derived error types (stated 9, measured 10, rule rul-in3bj4pn): re-measured at HEAD and re-worded together with the architecture and conventions statements of the same count.
@task_item n=3 part=needs
The one-enum-per-crate divergences named in the error-capture typed-error list (`ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError`, `Refused`): each folded or ratified where the masters-agree check runs; a fold changes that list and the fixed-Display scrubbing layer it feeds.
@task_item n=4 part=needs
The obs-scope-summary entity row for `viola-pty`, which names portable-pty `=0.8.1`: the version bump makes it stale; §1 is kept current as a verbatim obs-scope copy and an edit there is judged like any body amendment.
@task_item n=5 part=needs
Indirect, through the `portable-pty` bump: the supply-chain trigger (security Vector 9) holds the new graph Tokio-free in the sync crates, free of C-building crates and passing `cargo deny`; the per-module level table keeps the `portable_pty` target OFF, with failures captured at the viola seam.
@task_item n=6 part=needs
Indirect, through the `portable-pty` bump: the PTY seam boundary wrapper still gives `process-start{subject:"claude-child"}` on spawn, the child's `process-exit` from the handle-wait thread (never reader EOF), `exit_source:"kill-fallback"` on the kill fallback, and paste as `text_bytes` only; the killed-wrapper reading of item 11 rests on that exit path.
@task_item n=7 part=needs
The CI gates re-run green at the wrap: G1 bare `#[instrument]`, G2 zero panics, G3 no abort panic strategy, G4 schema conformance against `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json`, the secret scan, and the gated `diag-<os>` / `harness-<os>` / `junit-<os>` uploads in their step order.
@task_item n=8 part=needs
The build failure conditions the done-check must clear: clippy `disallowed-macros` / `print_stdout` / `print_stderr` / `dbg_macro`, `scripts/lint-probes.sh`, `cargo deny` (Tokio, C-crate, veil `toggle`, banned obs crates), and the zero error budget (0 panic lines, 0 secret-scan hits, 0 schema failures).
@task_item n=9 part=needs
v1-53's claim reads the `ubuntu-latest` nextest JUnit alone: the `junit-<os>` artifact, uploaded only when the secret scan passes, carries the nextest JUnit and is itself scanned.
@task_item n=10 part=needs
The live sessions (v1-53's `viola run`, the overseer driving a builder): `run` writes nothing to the terminal while the child runs and logs to `diagnostics/run-<name>.ndjson`; its evidence is read through `agent-run logs` / `status` and `jq`, never by human review.
@task_item n=11 part=needs
Indirect, through a live real-`claude` session: its diagnostics are non-synthetic, so any upload of them first drops `detail-*.ndjson` and `target/agent-run/`, and no `CLAUDE*` value, send text or `last_assistant_message` reaches a home-level file.
@task_item n=12 part=needs
The dialog hold near the 3600 s ceiling read live: the dialog → `answer` scenario's lines (a dialog hook's `hook-invoked` written after the `hook.dialog` response, question, answer and tool input never logged) are the read's evidence.
@task_item n=13 part=needs
The multi-platform-exporter-compat trigger and the CI platform matrix `[ubuntu-latest]`: one file sink and one line schema on Linux; the remnant removal leaves no OS-specific exporter, artifact or gate branch behind.
@task_item n=14 part=needs
The socket-directory refusal pair's wording (a founder item of Epoch 7): the §3 harness contract's start refusals and the `socket-dir-refused` detail code it carries, as that wording is ratified or re-worded.
@task_item n=15 part=needs
The `host.rs` `windows` test-host key sits in the mutation harness: a surviving mutant in obs code at `run --mutants` is a build failure, and the mutation audit's one survivor by construction (`peer_admitted`) keeps its dated note and untestable zone.
@task_standing n=1
convention: every integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it
home: ci-integration
@task_standing n=2
convention: every log line goes through `obs_event!` with typed fields named in the §6 catalog only, and conforms to `schemas/diag-line.v1.json` (default-deny); no raw `event!` or level macro outside `viola_core::obs`
home: log-coverage
@task_standing n=3
convention: no retry-once policy for panics or flaky telemetry assertions; `retries = 0`, and any nonzero error-budget count makes the chunk red until fixed within it
home: slo-invariants-telemetry-budgets
@task_standing n=4
convention: every CI gate is its own `run:` step with `shell: bash` on the `ubuntu-latest` leg; G2, G4 and the secret scan run `if: always()`; uploads are gated on the secret-scan outcome and Actions are pinned by commit SHA
home: ci-integration
@task_standing n=5
convention: every test input and fixture is synthetic, and no `CLAUDE*` value, `Cookie` header or content canary may reach a home-level `diagnostics/` file
home: pii-scrubbing-compliance
@task_standing n=6
convention: no `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of product crates
home: obs-anti-patterns
@task_item n=1 part=not_needed
Metric coverage: the chunk adds no derived metric, metric name or grouping key.
@task_item n=2 part=not_needed
The budget governor and statusline scenario: no change touches the statusline shell-out, `budget.json` or `budget-gate`.
@task_item n=3 part=not_needed
Trace-context propagation and the `corr` / `conn` join: the chunk adds no channel method, verb or cross-surface call.
@task_item n=4 part=not_needed
Heartbeat ticks and the liveness signal: nothing in the chunk changes the heartbeat file or its stale/gone flip.
@task_item n=5 part=not_needed
Edge flows E5 (snapshot replay) and the torn-append heal: no state-store reader or appender is touched.
@task_item n=6 part=not_needed
Operational retention and rotation of `events.ndjson` and `diagnostics/`: unchanged, none added.
@task_item n=7 part=not_needed
The obs tier, its justification, and the not-triggered compliance-audit-trail and self-observing-inversion triggers: not revisited by this chunk.
@task_item n=8 part=not_needed
Any frontend telemetry scenario: the 0.2.0 frontend is out of this chunk.
@task_item n=9 part=not_needed
Span naming and span-kind conventions: the chunk declares no new span.
