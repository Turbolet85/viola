@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (fake agent's print-ban exemption) — the fake agent is a `[[bin]]` of the root package and gets out of the print-macro ban only through a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`, which §11 lists. Splitting that 2124-line file (item 3) has to keep the allow at the bin's crate root so the moved bodies inherit it. The fix is not a new lint table, and no other file should get its own `#[allow]`.

@history_item n=2 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** (chunk.diff out of the secret scan) — the scan applies its Critical-class patterns to `target/agent-run/*` except exactly `target/agent-run/chunk.diff`, and the harness upload drops the same file. Removing the `cookie` class from `crates/viola-e2e/src/harness/secret_scan.rs` (item 9) must keep this exact-path skip and the matching upload exclusion (decision/secret-scan-scope, decision/mutation-diff-never-uploaded).

@history_item n=3 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's exact-path seam exemption) — G2 leaves out a panic only when its `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>`, compared as a whole string. The `src/cmd/hook.rs` merge (item 2) must not move the forced-panic seam out of `src/cmd/hook/seam.rs`, or the zero-panics gate goes red (decision/zero-panics-gate, decision/g2-zero-panics-gate).

@history_item n=4 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (G2 probe's planted literal) — the probe's other-file case names a planted `src/cmd/hook.rs` location by its home (`scripts/g2-zero-panics.sh`), never as `path:N`. Standing rule: a master names a planted literal by its home. This is a trap for re-measuring hook.rs coordinates (1075↔1152, now 1709 lines) and citing them in a plan.

@history_item n=5 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's stdout, the uninstrumented capture arm) — this entry fixed verify's outcome as stdout step lines plus a `stamped <version>  N pass  N fail` summary, its refusals as the stderr pair, and the hidden `hook --capture` arm as uninstrumented. These are the verdicts `tests/cli_verify.rs` asserts, and the split must keep them byte-identical and keep every test name (contract/verify-stdout/*, decision/hidden-capture-arms-uninstrumented).

@history_item n=6 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify in CI against the fake agent) — in CI, verify runs only against the fake agent through `tests/cli_verify.rs`, harness `boot` step 4 and `stamped_home`; the real-CLI verify never runs in CI. Both split targets, `tests/cli_verify.rs` and `src/bin/viola-fake-agent.rs`, carry this decision (decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci).

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (fake-agent verify flag list) — CI's fake-agent verify passes `--screens --turn-stop --trusted-root <root> --dialogs --framing`. Splitting `src/bin/viola-fake-agent.rs` must keep every flag and replay the fake agent serves, or boot step 4 and the stamped-home fixture stop stamping.

@history_item n=8 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** (the mutation row leaves CI) — obs code meets cargo-mutants only at `agent-run run --mutants`, at the epoch-boundary audit or on demand. That is why the named survivors (item 5) close by tests written now and are read at the next boundary audit, never by a mutation run in this chunk (decision/obs-code-mutation-tested, decision/surviving-obs-mutant-fails).

@history_item n=9 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** (what counts as surviving) — a surviving obs-code mutant is the harness document's `survived`, and a missed mutant the host never compiles is left out. Unviable mutants outnumbering caught ones is also red. This sets the bar for item 5 (tests per survivor, the `stdin.rs:15` `CHUNK` mutant marked equivalent, the tap's recorded exclusion from the mutants scope) and for the dead `#[cfg(not(unix))]` constants (item 10), which a Linux host never compiles.

@history_item n=10 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** (per-leg unviable rule) — the precedent for an epoch cleanup that touches mutation-verdict wording. The streaming change added no obs event, field or sink, so no obs section moved, and `mutants.out/` stayed un-uploaded. Recording the tap's exclusion from the mutants scope must also add no obs surface.

@history_item n=11 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** (CI keeps every home: deleter renamed) — a root test home is deleted through `remove_owned`, and CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home until G2, G4, the secret scan and the uploads have read it. The new viola-made-home helper (item 6) must keep this keep-until-gates behaviour (decision/homes-kept-until-gates-run, decision/test-homes-kept-until-gates).

@history_item n=12 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the chaos home outside the scans, F4) — integration homes live under `target/e2e-home/`, except `tests/chaos_feed_panic.rs`'s one `TestHome::outside_scan()` home, which is the founder's named carve-out. A shared home helper (item 6) and a single temp root (item 8) must keep every other home under the scan root and keep this one exception (decision/test-homes-under-e2e-home, decision/test-homes-under-scan-root).

@history_item n=13 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** (G2 over a linked scope, kept homes on tmpfs) — on the Linux dev host `target/e2e-home` is a link to an owner-only tmpfs directory, so kept homes end at reboot and `/tmp` ages them out. G2 starts its walk at `target/e2e-home/` with the trailing slash, because this host's `find` does not descend into a linked start point without it. Naming the host's quota'd temp root once (item 8) sits on this setup and must not break the trailing-slash scope.

@history_item n=14 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (pre-push document and mutation scratch wording) — on the Linux dev host the same-named scratch dir is only the operator's NOCOW `TMPDIR`, which no document prints, and `mutants.out/` stays at the repository root. The pre-push document carries no absolute path. A temp root set once for the host (item 8) must stay out of every document and upload.

@history_item n=15 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** (viola-ui leaves 0.1.0) — when the GUI was cut, the secret-scan classes were explicitly kept standing. This is where the GUI-era `cookie` class was left in place, and item 9 (`v2-06`) now removes it.

@history_item n=16 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§5–§11: the GUI scan forms leave) — §8's Critical class became the `Cookie` header (the scan's case-insensitive `cookie:` form) beside `CLAUDE*` values, the GUI token and `viola_<port>` cookie dropped. §11 still says never to log a `Cookie` header. Removing the scan's `cookie` class (item 9) collides with the obs rows decision/cookie-header-never-logged, decision/no-secrets-in-log-lines, decision/secret-scan-before-upload, decision/secrets-never-logged and decision/claude-env-and-cookie-never-logged. Every other class must still be caught, and the master change belongs to the wrap.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body census, check and live witnesses; exit-2 paths; v1-47's scan reading) — `tests/contract_error_bodies.rs` holds the census (`error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`) and `NOT_ERROR_BODIES`; `tests/support/events.rs` holds `error_body_hit` / `assert_error_body_clean`. These are the verdicts behind `blanked` that the shared scanner (item 1) must keep unchanged. v1-47 fixes when the secret scan runs (after each job's end-to-end run, `if: always()`), which the cookie-class removal must not move.

@history_item n=18 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the thiserror derive census on the product sources) — the census reads 9 derives by a grep of `thiserror::Error` over `src` and `crates` outside `crates/viola-e2e`. The merges (item 2) and the dead check (item 4) must not move the count, and a scanner swap under `thiserror_messages_interpolate_nothing` must keep its verdict.

@history_item n=19 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated test census in the E2E layer) — `tests/contract_os_gated_tests.rs` (run with `run --e2e`) requires every OS-gated function or file to be named by a `GATES` row with a non-empty reason, and its self-check reads planted cases. Its scanner is the third one item 1 replaces, and the census verdicts and planted cases must hold (decision/os-gated-test-census).

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (OS target Linux only; census another-OS fault; Windows strict-modes arm retired; closed words leave with their emitter) — the census also fails a gate whose predicate names `windows` or `target_os = "macos"`. `strict.rs`'s Windows arm and `check_socket_dir`'s off-Unix arm were cut, and a closed schema word no build can emit leaves with its emitter. This is the precedent for item 10's dead `#[cfg(not(unix))]` constants in `crates/viola-e2e/src/harness/run.rs`, for the census reading of any changed gate, and for removing dead `check_present` from `crates/viola-state/src/strict.rs` with its mention (decision/linux-only-os-target, decision/unemittable-schema-words-removed).

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_diagnostics`; the hook's instance check before obs init; snapshot readers over a replayed snapshot; E5 readers) — this entry built the strict-modes family in `crates/viola-state/src/strict.rs` that `check_present` sits beside: `check_instance`, `check_home_tree`, `check_diagnostics`, each refusal writing no line. It also put the hook event and dialog paths' instance check before `read_diagnostics_level`, with the `--capture` arms ahead of it, and moved hook `connect` and the run start onto `read_snapshot_or_replay`. The `src/cmd/hook.rs` and `src/run/snapshot.rs` merges (item 2) must keep that order and the one-read-per-call `state-recovered` line.

@history_item n=22 marker=2026-10-10-session-links
**2026-10-10-session-links** (a link has no line and no span) — the start restores the link set in one pass through `viola_state::events::read_from` with no span and no line, and the replay derives links through `apply_link_line`. The `src/run/links.rs:152`↔`src/run/snapshot.rs:58` duplicate (item 2) probably sits on these unspanned paths, and the merged function must add no span or line (decision/implicit-link-untraced, decision/link-has-no-own-trace).

@history_item n=23 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (E3: the explicit methods served; link pair names in the PII table) — `link` / `unlink` open no span and write no process-log line of their own, and their names never reach a process log (`link_verbs_log_the_channel_pair_and_no_names`). Folding links.rs into snapshot.rs (item 2) must keep this (decision/link-names-never-in-process-log, decision/link-methods-untraced).

@history_item n=24 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the pre-frame server check's place in the span tree) — `src/cmd/hook.rs`'s one shared `connect` serves the event and dialog paths, and `hook_refuses_an_unverified_server_inside_hook_handle` asserts its place inside `hook.handle`. A hook.rs merge (item 2) must keep that single `connect` and the captured-span case green (contract/hook-spans/span-chain).

@history_item n=25 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`Refused::NotADirectory`'s fixed message; socket-directory captured-span cases; clean-body witnesses) — `Refused::NotADirectory` in the strict module reads a fixed, pathless message. `hook_refuses_a_loosened_socket_directory_inside_hook_handle` sits in `src/cmd/hook.rs`, and the span recorder `Seen` sits in `src/cmd/client.rs`'s `refused_seen`. The witness list adds `tests/cli_error_bodies.rs`'s socket-dir case. All of these are verdicts the strict.rs, hook.rs and error-body edits must keep (decision/not-a-directory-refusal-message-fixed).

@history_item n=26 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (`run.pin_copy` under a concurrent first start) — `pin_bytes`' `place_absent` writes through `viola_state::fs::place_private_new` and re-hashes a lost placement (`PinError::HashMismatch`, the file left as found), adding no line, field or code. The error-kind guards behind the `crates/viola-state/src/fs.rs:123` and `crates/viola-state/src/pin.rs:91` survivors (item 5) are this path, and their new tests must assert no new line (decision/pinned-copy-race-adds-no-line, decision/pinned-copy-never-replaced).

@history_item n=27 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap; D-05 `mcp-stdin`; the oversize case's canary) — `viola_mcp::serve` hands rmcp `CappedLines::new(stdin, MAX_FRAME)`. A line of at most `MAX_FRAME` bytes, newline counted, passes whole; a longer one is refused whole and ends the session. One `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` is written at WARN into `mcp.ndjson` with no byte of the line. These are exactly the behaviours the `crates/viola-mcp/src/lib.rs:95` survivor (assert the WARN line) and the `crates/viola-mcp/src/stdin.rs:84` survivor (buffer smaller than the pending line) must pin, along with the canary-free home-level files.

@history_item n=28 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (viola mcp's exit at an oversize line before initialize; the mcp role writes no self process-exit) — before `initialize`, an oversize line exits the process 1 with both streams empty and the one record. Exit 11 is only the unit child's code (`serve_child_entry`), and the `mcp` role writes no self `process-exit`. These are traps when writing the stdin and lib.rs survivor tests (contract/mcp-stdin/oversize-before-initialize).

@history_item n=29 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-identity negatives' no-number scan) — `digit_runs` is a test-support scanner with its own unit test (`digit_runs_reads_whole_numbers_only`), used across the security negatives. It is the in-tree precedent for item 1's separately tested shared scanner in `tests/support/`.

@history_item n=30 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (tracing-subscriber's dev-dependents named) — tracing-subscriber is a product dependency of the root bin only, and `viola-state` and `viola-channel` may take it only as a dev-dependency for in-memory line capture in their unit tests. This binds any line-asserting test added in `crates/viola-state` or `crates/viola-mcp` for the survivors (decision/subscriber-only-in-root-bin).

@history_item n=31 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** (the masters as rows) — the obs master became the rows this chunk is bound by, each verified against the code at 8834195b0683. Rows such as the secret-scan, cookie, mutation and test-home decisions are the current form, and any master change for `v2-06` is an amendment to those rows at the wrap.
