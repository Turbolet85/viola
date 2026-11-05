@carried n=1 label=obs-plan/error-capture-reporting fact=fct-c5mu6ru4
why: Once the Windows frame capture leaves `src/panic_frames.rs`, the Unix capture must be the one path and keep writing raw, never-symbolised frames of the same shape, with no `RUST_BACKTRACE` gate.
quote: The backtrace is raw, never-symbolised frames from `src/panic_frames.rs` — `RtlCaptureStackBackTrace` + `GetModuleHandleExW`/`GetModuleFileNameW` on Windows, `libc::backtrace` + `dladdr` on Unix — up to 62 strings `0x<ip> <module path> base=0x<base> +0x<offset>` (or `0x<ip> ?`), enough for an offline resolver against the pinned copy. It is **not** gated by `RUST_BACKTRACE`, because env vars are not a configuration channel.
@carried n=1 run=2 label=obs-plan fact=fct-yn6razq4
quote: When an instance resolves, it also writes the payload and a raw, never-symbolised backtrace as one line to `detail-<process>.ndjson`, not gated by `RUST_BACKTRACE`, because env vars are not a configuration channel.
@carried n=2 label=obs-plan/error-capture-reporting fact=fct-bprhan45
why: When the Windows write-rule arm leaves the diagnostics strict-modes check, the Unix read rule, the symlink refusal and the failed open to `std::io::sink` must stand unchanged.
quote: a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes. A refusal is exactly a failed open: the role writer ends at `std::io::sink`, and the detail line is dropped.
@carried n=2 run=2 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
quote: A symlink, a folder another user may read (Unix: a group or other bit; Windows: a read grant to another SID) or a file another user may read (Unix: a group or other bit; Windows: the write rule) refuses the open as a failed open: the writer ends at `std::io::sink` and the detail line is dropped, so High-class content is never written through a symlink or into a folder another user may read (unit cases in `src/obs.rs`).
@carried n=3 label=obs-plan/error-capture-reporting fact=fct-vfnyc5f5
why: The Windows half of the catch-site reach names code the chunk removes, so the Unix strict-modes refusal with no line must be the one reading and the Windows sentence one of the expected amendments.
quote: **Catch-site reach:** on Unix a home under a regular file is a strict-modes refusal with no line — `viola verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair — so it never reaches the `cli` catch site; on Windows such a path stats `NotFound`, the check skips it as absent, and the catch-site line `process-exit{detail:"internal-error"}` stands.
@carried n=4 label=obs-plan/error-capture-reporting fact=fct-vfnyc5f5
why: The chunk edits `Cargo.toml` and the workflows, and G3 must still find no abort panic strategy in them so every catch site keeps unwinding.
quote: **Unwinding is required:** every Cargo profile keeps `panic = "unwind"`. `panic = "abort"` turns every `catch_unwind` into an abort, so `hook` would die by SIGABRT instead of exiting 0. Gate G3 (§9) enforces this.
@carried n=4 run=2 label=obs-plan fact=fct-sue3fxzz
quote: every Cargo profile keeps `panic = "unwind"`, enforced by G3.
@carried n=5 label=obs-plan/error-capture-reporting fact=fct-vfnyc5f5
why: The scope leaves the `.cmd` / `.bat` refusal to P3 and P4, so the `batch-script-child` start refusal, its `--json` document and its human pair must stay together or leave together, and every other start refusal must keep its document.
quote: **Start refusals under `--json`:** `refused_with` (`src/cmd/run.rs`) prints the exit-1 document in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and the `internal-error` document for a program not found, which has no human pair, all before `spawn_child`; `run` passes `Launch`'s `json` false and `revive` passes its `--json`.
@carried n=5 run=2 label=obs-plan fact=fct-26ufyipy
quote: writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`
@carried n=6 label=obs-plan/error-capture-reporting fact=fct-j5jj3gux
why: If the `.cmd` / `.bat` refusal leaves, its `BatchScriptChild` variant and the derive census must be re-read with it, and every remaining message must stay fixed.
quote: The product's `thiserror::Error` derives read 9: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (as measured at chunk 2026-10-31-sanitised-error-surfaces, a grep of `thiserror::Error` over `src` and `crates`). `AgentError`'s variants include `BatchScriptChild` and `ProgramNotFound`, their two messages fixed.
@carried n=7 label=obs-plan/error-capture-reporting fact=fct-l5yuzjd3
why: The Windows byte reading names the retired route entry «Windows-only live measurements», which the expected amendments re-point, and the Unix byte assertion must stay in the test once its OS split leaves.
quote: The same test also witnesses the passthrough after the forced feed panic: none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal on any OS, and on Unix the child's bytes `e4 b8 ad` reach it; the Windows byte reading is owed to the route entry "Windows-only live measurements", since ConPTY re-renders a wide character on a 1-column screen.
@carried n=8 label=obs-plan/error-capture-reporting fact=fct-pto2t3ps
why: The chunk edits the OS split of `tests/chaos_feed_panic.rs`, and its home must stay outside `target/e2e-home` as G2's single seam exemption, however the e2e-home rule reads for a changed test.
quote: Witnessed at run level by `tests/chaos_feed_panic.rs` (chunk 2026-10-04-confirmed-send-with-cl-1-records): exactly one `parse-rejected{vt100-feed, panicked}` and one `event:"panic"` line, both asserted present, then `viola send` refused `send-refused{detail:"input-not-ready"}` (exit 13) while a typed key still reaches the child. G2's single seam exemption is unchanged: that test's home lives outside `target/e2e-home` (`TestHome::outside_scan()`), so G2, G4 and the secret scan do not read it (the founder's ruling, live, 2026-10-04, relayed by the overseer).
@carried n=9 label=obs-plan/error-capture-reporting fact=fct-mgjsgjd2
why: The socket-directory check is Unix code whose gate the chunk may drop, and its MCP refusal must keep `socket-dir-refused`, no frame and no line.
quote: MCP rejects on a refused socket directory: `frame` runs `check_endpoint_dir` on the recorded endpoint after `live_endpoint`; a refusal returns `instance-unreachable` with `detail: "socket-dir-refused"` through `UnreachableCause::SocketDir`, which no `RefusalReason` carries, and writes no frame and no line, the `isError` result its only record.
@carried n=10 label=obs-plan/error-capture-reporting fact=fct-25dvifnk
why: The macOS carve-out in the peer check leaves with the macOS arm, and a refused peer or a failed server verification must still map to `ServerVerify` on Linux.
quote: a refused `peer_admitted` or a failed `server_verified` is `ServerVerify`
@carried n=10 run=2 label=obs-plan fact=fct-pqz5mgd5
quote: or a frame tool's refused euid or server-verification mismatch (Linux, Windows and macOS, on macOS an euid mismatch alone) — is logged by no line, its outcome kept inside `mcp.tool_call`, the `isError` code its only record.
@carried n=11 label=obs-plan/error-capture-reporting fact=fct-3qk5e3jo
why: The scope keeps every Linux closed word and exit code, so the six `UnreachableCause` words and the one exit-21 document per cause must survive the removal of the Windows and macOS peer arms unchanged.
quote: Exit 21 prints one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`; `detail` is one of the six closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`.
@carried n=12 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The per-OS `test` and `perf` jobs shrink to their Linux leg, and their detail-file uploads must keep the synthetic-input basis and the secret-scan gate.
quote: It covers two artifacts: the `test` job's `diag-<os>` and the per-OS `perf` job's `diag-perf-<os>` (`target/e2e-home/**/diagnostics/*.ndjson`), whose inputs are the per-row synthetic hook payloads (string fields, the tests-owned canary in content fields) and the `fake-agent` seam's fixed ASCII panic payload (in `detail-hook.ndjson` only; the home-level `panic` line stays payload-free). The same job's `perf-<os>` (hyperfine timings and repo-relative command lines only) and `diag-perf-<os>` upload only when `always() && steps.secret-scan.outcome == 'success'` — the same channel, scan gate and synthetic-input basis as `diag-<os>`, not a new crossing.
@carried n=12 run=2 label=obs-plan fact=fct-tcjwyf2p
quote: **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload; the canary never appears in a home-level `diagnostics/*.ndjson`; a detail-file upload is admissible only because every input is synthetic, so a non-synthetic source first drops `detail-*.ndjson` and `target/agent-run/` from the upload paths; on a scan failure only the hit report is uploaded.
@carried n=13 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The Windows leg of the server-identity negative test leaves with its macOS compile-out, and the test must still scan every output and the `cli-<name>.ndjson` line with `digit_runs` on Linux.
quote: A server-verification refusal carries no pid, start time, path or endpoint: `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs` (whole numbers only, itself tested by `digit_runs_reads_whole_numbers_only`), on Linux and Windows (the file compiles out on macOS).
@carried n=14 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The strict-modes negative test loses its macOS and Windows legs, and its Linux reading must still find no line and no pid, path or endpoint.
quote: `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and finds no line, on Linux, macOS and Windows.
@carried n=15 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The socket-directory negative test is `cfg(unix)` with no Windows sibling, so if its gate drops it must keep reading every line and document with `assert_names_nothing` and `digit_runs`.
quote: A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed `ViolaName`: `tests/security_negatives_socket_dir.rs` reads them with `assert_names_nothing` and `digit_runs`, on Linux and macOS (the file is `cfg(unix)`, with no case on Windows).
@carried n=16 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: `HOST_SCRATCH = cfg!(windows)` is a gate the chunk removes, and `mutants.out/` must stay at the repository root, gitignored, with no document printing a scratch path and nothing of it uploaded.
quote: On a Windows host it sits in the harness's host mutation scratch `<repo parent>/viola-mutants-scratch/` (`HOST_SCRATCH = cfg!(windows)`, inert elsewhere), outside the repository, whose path is never printed (documents carry only the byte count `scratch_bytes`). On the Linux dev host the same-named directory is only the operator's NOCOW `TMPDIR` for cargo-mutants' copy trees — an environment fact, not harness code — and no document prints it either; `mutants.out/` there stays at the repository root (gitignored).
@carried n=16 run=2 label=obs-plan fact=fct-onsbpcol
quote: Unscanned uploads are admissible by content only: a member that could carry a host path or log content is scanned before upload, or dropped; `mutants.out/`, `target/run-archive/` and the local `viola-harness pre-push` document are never uploaded, and a non-synthetic fuzz corpus seed first drops the `fuzz/artifacts/` upload.
@carried n=17 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The chunk edits `nightly.yml` and keeps its `fuzz` job, whose `fuzz/artifacts/` upload must stay on `failure()` only, mutated from the committed synthetic corpus.
quote: the nightly `fuzz` job's `fuzz/artifacts/` (on `failure()`) holds crash inputs the fuzzer mutated from the committed synthetic `fuzz/corpus/<target>/`. A non-synthetic corpus seed must first drop that upload.
@binding n=1
**error-capture-reporting ↔ pii-scrubbing-compliance**: the content sink's strict-modes check must keep one Unix read rule for the role and detail files once the Windows arm leaves, so a refusal is still a failed open and High-class content never lands in a folder another user may read (obs-plan/error-capture-reporting fct-bprhan45) (obs-plan/pii-scrubbing-compliance fct-ydc7ueix).
@binding n=2
**error-capture-reporting ↔ the scope's `.cmd` / `.bat` fork**: the `batch-script-child` `--json` document, the `BatchScriptChild` variant and the fixed `unable` / `hint:` pair must move as one, whichever way P4 decides (obs-plan/error-capture-reporting fct-vfnyc5f5) (obs-plan/error-capture-reporting fct-j5jj3gux) (obs-plan fct-26ufyipy).
@binding n=3
**error-capture-reporting ↔ the closed-word contract**: removing the macOS carve-out and the Windows peer arms must leave the six `UnreachableCause` words, the `ServerVerify` mapping and the `socket-dir-refused` record unchanged on Linux (obs-plan/error-capture-reporting fct-3qk5e3jo) (obs-plan/error-capture-reporting fct-25dvifnk) (obs-plan/error-capture-reporting fct-mgjsgjd2) (obs-plan fct-pqz5mgd5).
@binding n=4
**pii-scrubbing-compliance ↔ ci-integration**: the Linux `test` and `perf` jobs that remain must keep their `diag-<os>`, `diag-perf-<os>` and `perf-<os>` uploads behind the secret scan, in the §9 step order (obs-plan/pii-scrubbing-compliance fct-ydc7ueix) (obs-plan fct-cstqzpm2) (obs-plan fct-hmk2eyfw).
@binding n=5
**error-capture-reporting and pii-scrubbing-compliance ↔ the wrap's expected amendments**: the Windows and macOS sentences of the panic-frame capture, the catch-site reach, the feed-panic byte reading owed to «Windows-only live measurements», the per-OS negative tests and `HOST_SCRATCH` must be named in the plan as amendments the wrap writes, since the phase loop does not edit the obs-plan (obs-plan/error-capture-reporting fct-c5mu6ru4) (obs-plan/error-capture-reporting fct-l5yuzjd3) (obs-plan fct-7rmyxbi7).
@binding n=6
**error-capture-reporting ↔ ci-integration (G2, G3)**: the edited manifests and workflows must still pass G3's no-abort check, and `tests/chaos_feed_panic.rs` must keep its home outside `target/e2e-home` as G2's single seam exemption (obs-plan/error-capture-reporting fct-vfnyc5f5) (obs-plan/error-capture-reporting fct-pto2t3ps) (obs-plan fct-2rxslar4).
