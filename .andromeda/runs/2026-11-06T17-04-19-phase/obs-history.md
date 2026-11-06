@history_item n=1 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the closed words that left `diag-line.v1.json` with their emitter) `process-exit` `detail` closed at 19, `os` closed to `linux`, the schema item reads `pty_backend` a string; §6 gained the rule that a closed word no build emits leaves the schema with its emitter. Kept: the log-coverage `process-start` row (fct-yb5jpaot) still names `conpty-sideload|conpty` and `sideload_fallback` because its edit dropped cell pipes the store refused. It was routed to a CARRY on «Version done-check» with the target text (`pty_backend` `openpty` alone, `sideload_fallback` gone). This is CARRY 14's origin. The trap is the store's cell-pipe refusal, so the edit must go through the whole-fact form and keep the header's 2 cells. The 19-value count is what G4 checks against.

@history_item n=2 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the ConPTY sideload and the Windows kill fallback retired; openpty the one PTY backend) The `viola-pty` entity now reads portable-pty `=0.8.1` alone, and the OS-API entity reads openpty, Unix socket, locks and rename, sysinfo. `pty.spawn`'s `pty_backend` reads `openpty`, span-trace's `process-start` carries no `sideload_fallback`, and the strict-modes line sites are down to two. This is the wording CARRY 12's portable-pty move must re-pin (fct-t2iihkln), and the span-trace side CARRY 14's log-coverage row must agree with. `pty_backend()` always returns `openpty`.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the `.cmd`/`.bat` start refusal retired; the refusal writer's site counts) `AgentError`'s named variants now read `ProgramNotFound` alone, and the `human::refuse(` census reads `run` 7, `verify` 5. Kept: the typed-error census still reads 9 derives, though the report read 10 both before and after the chunk. That drift was not that chunk's, so it was routed to a CARRY on «Version done-check». This is CARRY 16's origin: re-measure the count by its grep of `thiserror::Error` over `src` and `crates`, then re-word the count in error-capture (fct-j5jj3gux) and the platform pick (fct-nwrbbhk2).

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (one CI leg, two workflows, one deny target and one harness shim) Platform now reads `ci.yml` beside `nightly.yml`, matrix `[ubuntu-latest]`, gates on that leg only, and `cargo deny` runs on the one target `x86_64-unknown-linux-gnu`. The OS-gated census gained its another-OS fault class: a predicate naming `windows` or `target_os = "macos"` is a fault even when a row names it. `logs` names `scripts/agent-run.sh` alone. This is the gate set «every gate green on Linux» rests on. The census fault class bears directly on CARRY 17's remnant removals and keeps, and on v1-53's census claim.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the OS target, Linux only) The tier values gained the Linux-only OS target, with the Windows and macOS code recoverable at the tag `multi-os-final` (`ecbe7ab3`). The multi-platform signal now reads one file sink and line schema on Linux (0600/0700, `O_APPEND`, Unix socket, openpty), and the concurrent-append check runs on `ubuntu-latest`. A dated three-OS measurement stays as history, never as current. This is the target CARRY 17's remnant removal must leave the tree at. The multi-platform-exporter-compat trigger stands, stated for Linux.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the perf job on its one leg, PID reuse without its OS) The perf gates and `diag-perf-<os>` are now read in the one `perf` job. Kept as true: `exit_source:"kill-fallback"` (`ExitSource::KillFallback` stands in `viola-pty`) and the `junit-<os>` upload name. The PTY seam's kill-fallback line must therefore survive the portable-pty bump. The `<os>`-form artifact names are not Windows remnants to remove under CARRY 17.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows home owner/DACL strict-modes arm retired) `diagnostics/` files now read 0600 in 0700 directories, `check_diagnostics` holds the read rule with no group or other bit, and init step 4 creates the home at 0700. The live `viola run` sessions' acceptance (files 0600) and the secret scan's 0600 check rest on this. Any remnant left in the strict-modes code is CARRY 17's to remove.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the named-pipe endpoint and the macOS no-peer-pid carve-out retired; the server check on Linux) `peer_admitted` now refuses another euid and an unreadable one, with `peer_euid_admitted(observed, own)` as `observed == Some(own)`. The span recorder item no longer names the macOS-gated `refused_inside`. This is the predicate behind CARRY 8's always-admit survivor at the three client seams.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows host arms of the panic capture, the env strip, verify and the statusline retired) This entry retired the panic backtrace's Windows arm, the case-insensitive strip and the registry reader, verify's console-control stop, and `WRAPPED_HERE`, which was folded to true. It is the code family whose leftover comments and keys CARRY 17 names, such as `crates/viola-agent-claude/src/lib.rs:2` and the `COVERAGE_*` comments. The masters already read Linux-only, so removing a remnant should move no obs text.

@history_item n=10 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the OS-gated test census in the E2E layer) This entry added `tests/contract_os_gated_tests.rs` (run through `run --e2e`). Every OS-gated test function and whole gated file must be named by a `GATES` row with a reason, and the census fails on an unnamed gate or a stale row. v1-53's claim reads this census, plus the `ubuntu-latest` JUnit. Its function and file counts must be re-measured after CARRY 17's removals.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the nine thiserror derives, CoreError retired) The platform pick's thiserror list was rewritten to `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`. The count 9 was measured by a grep of `thiserror::Error` over `src` and `crates`, after `Refusal` folded into `AgentError` (10 to 9). This is the measurement CARRY 16 re-takes, by the same grep rule. A fold under CARRY 6 moves this list and its count together.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`Refused::NotADirectory`'s fixed message) The new variant has a fixed message naming no path. As a variant of `Refused` with no derive of its own, it adds nothing to the 9, and that chunk's re-count still read 9. This is the counting precedent for CARRY 16: variants are not derives, so only `#[derive(thiserror::Error)]` sites count.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the error-body census, check and live witnesses) This entry added `tests/contract_error_bodies.rs`, including `thiserror_messages_interpolate_nothing`, plus the `assert_error_body_clean` check and the live witnesses. Any fold or ratification of the one-enum-per-crate divergences (CARRY 6) must keep every derive's `Display` fixed and non-interpolating. This census must stay green.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (viola mcp's exit at an oversize line before initialize) A cut before `initialize` exits with process exit 1 (`ServeError::Initialize`; `src/main.rs` `exit_code` maps `Role::Mcp` + `Failed` to 1), and 11 is the unit child's own code. This bears on CARRY 6: a fold of `ServeError` beside `McpError` must keep this exit path and re-word the variant the fact names (fct-iy5crjhq).

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the mcp role writes no self process-exit) `internal_error_exit_line` writes for `Run` and `Cli` only. The Structured Logs and Errors rows carve out «the `mcp` role writes no self `process-exit`». A `ServeError` / `McpError` fold must keep this carve-out true.

@history_item n=16 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — (env_kept, D-34 supersedes D-14, batch-script-child from resolution, run refusal stderr) §7 records `PtyError`'s fixed `Display` as hand-written, because `viola-pty` takes no thiserror. That keeps `PtyError` out of CARRY 16's derive census. The portable-pty move must keep `viola-pty` thiserror-free or the census changes. `env_kept` and the names-only strip (D-34) also bind the live sessions' child `process-start`.

@history_item n=17 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (D-36 sideloaded ConPTY telemetry) This entry introduced `pty_backend` `conpty-sideload|conpty|openpty` and the additive `process-start` field `sideload_fallback`, closed in `diag-line.v1.json` under §8 default-deny. These are exactly the words CARRY 14 removes from the fct-yb5jpaot row. Removing them touches obs-added fields only, never a tests log-format field.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`sideload_fallback` gains `strict-modes-failed`) `sideload_fallback` grew to five words, the last from `PinError::StrictModes` at `pin_companions`, and the schema closed it to those five. This is the long D-36 tail still quoted in the fct-yb5jpaot row that CARRY 14 cuts. The schema side already left with the emitter at 2026-11-05.

@history_item n=19 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (registry migration (U35): the obs-plan Decisions Log leaves the body) This entry lifted D-11, D-23 and D-24 into §3 Logging stack: OFF third-party targets with failures captured at the seam (spawn `Err` becomes `internal-error`, handle-wait gives `exit_source`), and the seam-gap path, never through `tracing-log` / `LogTracer`. It also lifted D-36's `pty_backend` open string / `sideload_fallback` closed enum, and D-35's closed `subject` enum. These are the rules the portable-pty move must hold, and the subject enum CARRY 14 leaves unchanged.

@history_item n=20 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (the reserved web vocabulary leaves: the `ui` process and four events) This entry set the third-party OFF targets to `rmcp`, `portable_pty`, `sysinfo`, `interprocess` (fct-7e4jvssp). The portable-pty bump must keep the `portable_pty` target OFF and capture failures at the viola seam.

@history_item n=21 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (seven verify spawn pairs; a killed PTY child's exit status) It was measured that a child of an interactive PTY run killed at the stop carries `child_exit_status`, because portable-pty reports a code for a signalled child. This was measured on portable-pty `=0.8.1` (fct-f3gi2vhc). After the bump the behaviour must be re-measured on the new release, or the line re-worded.

@history_item n=22 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (§1 kept current) Under the founder's 2026-10-04 ruling, §1 stays a labelled verbatim copy but is kept current: a cascade hit inside it is judged like any body amendment. So the §1 `viola-pty` row's `=0.8.1` pin is re-worded as a body amendment under the portable-pty move, not left stale.

@history_item n=23 marker=2026-10-25-u55-door
**2026-10-25-u55-door** — (obs-plan: split into its body and 10 category files) The master was split into a body and category files, mapped in `.andromeda/obs-plan.map.tsv`. CARRY 14's row (log-coverage, fct-yb5jpaot), CARRY 16's census (error-capture-reporting, fct-j5jj3gux) and the `viola-pty` row (obs-scope-summary) live in these category facts. They are edited through the store, never by hand during the phase run.

@history_item n=24 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — (obs_event! field set, schema default-deny keyword) Default-deny is enforced by a top-level `unevaluatedProperties: false`, not by per-event `additionalProperties`. Dropping `sideload_fallback` from §6 (CARRY 14) must leave `schemas/diag-line.v1.json` and the §6 catalog in step under this keyword, with G4 at zero conformance failures.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `process-exit` `detail` enum's count) The count was set at 19, counted from the enum's members, with the history of earlier counts dropped. Its count rule is the one the 2026-11-01 entry (19 to 20) and the 2026-11-05 entry (back to 19) both follow. Any code-word change under the socket-directory ruling (CARRY 7) moves this count by the same rule.

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (Founder Direction 4's codes and named carve-outs) The socket-directory cause's lack of a code was opened as an OPEN, PROVISIONAL item owned by «Unix endpoint and home hardening», which later closed it. This is the history behind the `socket-dir-refused` code whose pair wording CARRY 7 brings to the founder.

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory refusal's human pair) The pair is named only by pointer to design-system §Surface: cli (plain ASCII, `hint:` last, no SGR, the parsed `ViolaName` alone). Its wording is a planning form approved by the delegate (`inputs#I3`, PROVISIONAL), with the founder's wording owed at Epoch 7's boundary batch. That is CARRY 7's first item. Kept: the wording itself stays out of obs-plan, so a ratification or re-wording moves design-system, not obs text.

@history_item n=28 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (exit 21's closed causes, five to six) `UnreachableCause::SocketDir` became the sixth closed exit-21 word, and `refused_with` prints the exit-1 document for `socket-dir-refused`. The answer card's confirm was the delegate's, PROVISIONAL, superseded by the founder's own word at Epoch 7's boundary. That is part of CARRY 7's «151 proposals inside Q1 and Q2».

@history_item n=29 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (MCP's frame tools: the socket-directory and euid refusals) `frame` runs `check_endpoint_dir` after `live_endpoint`. It returns `socket-dir-refused` through `SocketDir`, or `server-verify-failed` on a refused `peer_admitted` / `server_verified`, with no frame and no line. The mark is PROVISIONAL and owed at Epoch 7's boundary (CARRY 7). This is also one of CARRY 8's three seams.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused` in the log catalogue, `process-exit` 19 to 20, `hook-decision` 7 to 8) The code joined the exit-1 list, the exit-21 list and the hook fail-open codes, and both schema enums. The mark is PROVISIONAL and owed at Epoch 7's boundary (CARRY 7). Any re-wording must keep the one code consistent across these lists and the schema's enum.

@history_item n=31 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`server-verify-failed` reaches macOS, as an euid mismatch alone) The euid reach was set on Linux, Windows and macOS, a PROVISIONAL card confirm owed at Epoch 7's boundary (CARRY 7). Its macOS half was later retired by 2026-11-05. Only the Linux reading still stands for the founder's batch.

@history_item n=32 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory is no strict-modes failure) `check_socket_dir` takes the strict-modes read verdict yet writes under its own `detail`, with one line per surface (run/revive start, eight CLI frame verbs, hook; none at MCP `frame`). The mark is PROVISIONAL and owed at Epoch 7's boundary (CARRY 7). It is the basis of fct-wrd7y3z2 and fct-mivk5eld.

@history_item n=33 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the frame verbs' socket-directory refusal and its line) A frame verb meeting a refused socket directory writes one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN inside its dispatch span, with no channel line. The check sits in `cmd::client::connect` and has 8 callers. The mark is PROVISIONAL and owed at Epoch 7's boundary (CARRY 7).

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`run`'s start refusals and the `human::refuse` census) The start refusals gained the socket directory that `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`), and `run` went 7 to 8. This is the fct-26ufyipy text that CARRY 7's pair ruling reaches. It is also the census rule (`grep -rcE 'human::refuse\(' src`) that any CARRY 17 edit in `src/cmd/` must keep counted.

@history_item n=35 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the OS call sites the new checks add) The OS-APIs entity gained the peer-euid read and `check_socket_dir`, with its refusal written under `socket-dir-refused` and MCP writing no line. This is the fct-f3waaflx text, a PROVISIONAL card confirm owed at Epoch 7's boundary (CARRY 7). Kill-fallback use stays a listed call site through the portable-pty bump.

@history_item n=36 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (Founder Direction 4: the socket-directory carve-out retired, macOS's named and unowned) The OPEN closed: the socket-directory cause has its one code whatever failed (a refused check, a failed create, a lost race). That one-code rule is a P5 lean (`inputs#I3`, PROVISIONAL), part of CARRY 7's «that chunk's P5 word and its leans» (fct-n7xftqkr).

@history_item n=37 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the hook's timed path holds both new checks, no perf value moved) The socket-directory and peer-euid checks sit in the hook's timed path under `SPINE_DEADLINE`, with the band max 2.5 to 3.3 ms kept. The perf job green at the wrap rests on this row. The card confirm is PROVISIONAL and owed at Epoch 7's boundary (CARRY 7).

@history_item n=38 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the wrapper's drop of a refused peer writes no line) `accept_loop` drops a peer of a refused euid with no span and no line. This is a P5 lean (`inputs#I3`, PROVISIONAL), one of CARRY 7's leans owed the founder's word.

@history_item n=39 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the start's socket-directory step, no span and one line on a refusal) This defines the start's `socket-dir-refused` exit-1 line, one `process-exit{subject:"self", exit_code:1}` before any snapshot, child or bind (fct-h2mbnimu), from a fourth start step with no span. The mark is PROVISIONAL and owed at Epoch 7's boundary (CARRY 7).

@history_item n=40 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory at `viola hook`) This defines one `hook-decision{detail:"socket-dir-refused"}` at WARN inside `hook.handle`; on the dialog path `hook-invoked{corr:null}` comes first. The hook `connect` chain gains `check_endpoint_dir` and the euid decision. The one line is a P5 lean (`inputs#I3`, PROVISIONAL), part of CARRY 7.

@history_item n=41 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory refusal's PII reading and clean-body witnesses) `tests/security_negatives_socket_dir.rs` reads no path, pid or uid in the refusal's lines and documents, and its human pair names only the `ViolaName`. The live witnesses went from three to four. A re-worded pair under CARRY 7 must still pass these.

@history_item n=42 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the checks' order before any frame, by span scenario) This set the CLI order before any frame: `check_endpoint_dir`, connect, `peer_admitted(client.server_euid())`, `verify_server`, for all 8 callers. It is a PROVISIONAL card confirm owed at Epoch 7's boundary (CARRY 7). It is also the order CARRY 8's survivor sits in.

@history_item n=43 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the client euid check's place at the three seams) The three seams (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) read the euid through `server_euid`, which writes and logs nothing. Each seam's wiring was read only through forced refusals, since one account can make no foreign peer. This is exactly CARRY 8's mutation survivor by construction (fct-jvpve25b).

@history_item n=44 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory refusal's captured-span cases) This entry added `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, and moved the span recorder `Seen` to `refused_seen`, out of the then macOS-gated `refused_inside`. A pair re-wording or a remnant edit must keep these cases green.

@history_item n=45 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (OTel SDK init: the `cli` chain after the log opens) The `cli` anchor's chain now reads: liveness pre-check, socket-directory check (refused: the exit-21 `socket-dir-refused` line), peer euid decision, server check. This is a PROVISIONAL card confirm owed at Epoch 7's boundary (CARRY 7).

@history_item n=46 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — (the marks are his; §3's Log format copy matches test-plan's) This is the precedent for discharging delegate marks on the founder's own word. The PROVISIONAL records were superseded in this file's history with their other claims standing and none pruned, and the run's `marks-discharged.md` named each heading. CARRY 7's Epoch 7 batch discharges the 2026-11-01-unix-endpoint-and-home-hardening marks the same way.

@history_item n=47 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — (the founder's Epoch 5 batch: every refused check's no-line shape his) This is the earlier precedent of an epoch-boundary founder batch ratifying delegate marks. Each superseded entry is listed by heading, its provisional mark gone, and its other claims stand. It shows the form CARRY 7's batch takes in obs-plan.

@history_item n=48 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (G2's exact-path seam exemption, the D-28 over-4 KiB half, the perf rows built) G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first, in the `test` and `perf` jobs). It exempts exactly `src/cmd/hook/seam.rs:<digits>`, as the founder ratified live. The error budget's «0 panic lines» is read as G2 counts them. A remnant edit must not move the seam's file, or the exemption breaks.

@history_item n=49 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (the G2 question answered: the chaos home outside the scans (F4)) `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home sits outside G2, G4, the secret scan and the upload: the second named test-data carve-out, with no second G2 exemption. The zero-panic read at the wrap relies on this carve-out staying in place.

@history_item n=50 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — (G2's start point descends a linked scope, and a kept home's diagnostics on the dev host) G2's `find` start points read `target/e2e-home/` with a trailing slash, because on the dev host it is a link to tmpfs. Without the slash a local G2 reads `empty scope`. That is the trap for any local gate re-run at the done-check. Kept homes' diagnostics end at a reboot.

@history_item n=51 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (the G2 probe's other-file case carries no line citation) The standing rule: a master names a planted literal by its home, never as `path:N`. This matters for this chunk's master edits and citation sweeps around G2 and the census.

@history_item n=52 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — (CI keeps every home: deleter renamed) Neither harness `cleanup` nor a test home's drop (`remove_owned`) deletes a home before G2, G4, the secret scan and the uploads. CI's `AGENT_RUN_KEEP_HOMES=1` guarantees this. CARRY 17's `EXE_SUFFIX` harness edits must keep it honoured.

@history_item n=53 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (v1-47's secret-scan reading, provisional) The secret scan runs after each job's end-to-end run, with `AGENT_RUN_KEEP_HOMES` `1`, `if: always()`, and the `viola-chaos-*` home out. The OPEN was later discharged by 2026-11-01-t02-46-44-wrap. This is the zero-hit reading the wrap re-reads.

@history_item n=54 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — (supply-chain artifact admitted by content (operator ratification)) The `supply-chain` artifact (`deny.json`, `deny-fuzz.json`, `zizmor.json`) uploads unscanned, admissible by content (0 absolute paths as measured). The guard binds: a member that could carry a host path or log content is scanned or dropped. CARRY 12 re-runs this job green on the new portable-pty graph, and the artifact must stay inside this rule.

@history_item n=55 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — (Platform: nightly.yml beside the single push/PR ci.yml) The weekly `cargo deny check advisories` runs in `nightly.yml`, beside `ci.yml`. RUSTSEC-2026-0441 reads red in both `cargo deny check advisories` and CI's `supply-chain`, so CARRY 12's remedy must turn both green with no ignore-list widening.

@history_item n=56 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — (raw-tracing ban: level-macro path ban plus a fail-closed raw-`event!` grep) The `clippy.toml` `disallowed-macros` path ban and the fail-closed raw-`event!` grep in `scripts/lint-probes.sh` were added, and a lint-probes failure became a build failure. These are lint-stage gates in «every gate green on Linux» (fct-xpak76xj, fct-5kn2jpbt). Remnant and dependency edits must keep them passing.

@history_item n=57 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — (where `state-recovered` is written and how often; tracing-subscriber's dev-dependents named) tracing-subscriber is a product dependency of the root bin only. `viola-channel` and `viola-state` name the workspace pin under dev-dependencies. The Cargo.lock move under the portable-pty bump must leave the locked tracing-subscriber at the Vector 9 floor `>=0.3.20`, with the sync crates' graphs Tokio-free.

@history_item n=58 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — (the obs-code mutation reading on Windows is its own job; "missed" reads after the host exclusion) Red at a `run --mutants` is a timed-out obs-code mutant, a missed one not left out as host-excluded, or unviable mutants outnumbering caught ones. A missed mutant the host never compiles is left out (fct-r4tkrl6z, fct-o5c3uxju). CARRY 17's `host.rs` `windows` test-host key decides what is host-excluded, so removing or keeping it must not widen that set.

@history_item n=59 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — (the mutation row leaves CI; the leg verdict upload and pre-push scratch counts retire) Obs code meets cargo-mutants only at `agent-run run --mutants` (the epoch-boundary audit or on demand), never in CI. CARRY 8's survivor stands as a dated note, not a CI gate. The code audit is out of this chunk's scope.

@history_item n=60 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (E5: the other snapshot readers, five to three) This set the standing rule that a count carried from a plan is re-measured at the wrap, never copied. That rule governs CARRY 16's derive count (10 vs 9) and the census function/file counts v1-53 reads.

@history_item n=61 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — (the expiry is the wrapper's: `dialog-expired` and `dialog-answered{deadline_hit:true}`; the start reads `dialog_hold_s`) `deadline_ms` is the slot's hold, `dialog_hold_s` in ms (60 000 to 3 600 000). At the hold's end the wrapper writes `dialog-expired` and `dialog-answered{deadline_hit:true}`, and the hook writes `hook-decision{decision_emitted:false, deadline_hit:false}`. The hook's own `deadline_hit:true` is only its 3605 s backstop. This is the evidence model for CARRY 2's live hold read near the 3600 s ceiling, which stays PROVISIONAL until that read.

@history_item n=62 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (`dialog-answered`'s `from` / `from_trust` only on an answer that carried a `from`) This was measured false live and re-worded: a `viola answer` without `VIOLA_NAME` carried no `from`. It is the precedent that a live session's lines re-word the master when they disprove it. That bears on this chunk's live `viola run` and driver sessions.

@history_item n=63 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (verify-probe joins the spawn subjects; boot readiness checks the start records) The real-`claude` verify runs only through the local `run --local-live`, which is refused under `CI`. The live sessions' verified branch (`cli_verified:true`) needs a CLI stamped this way. The `subject` enum, which CARRY 14 leaves unchanged, gained `verify-probe` here.

@history_item n=64 marker=2026-10-10-session-links
**2026-10-10-session-links** — (a link has no line and no span) The implicit link is recorded inside an issued `send` or a taken `answer` whose `from` is another name. It has no process-log line, no `event` value and no span: its trace is the channel pair, the product `link` line directly before `send-issued`, and the snapshot's `links`. A CARRY 4 proof of v1-13 / v1-36's link clauses reads this trace.

@history_item n=65 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (E3: the explicit methods served, the hold retired) The wrapper serves `link` / `unlink` with no span or line of its own. The trace is the channel pair holding neither name, the product line and the snapshot's `links`. Bad names give `-32602`. This bears on CARRY 4's dispositions of v1-13 (MCP `link` result) and v1-36 (the board's link set).

@history_item n=66 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (run.budget_eval in the start chain, the gate's log surface) `run.budget_eval` sits between `channel.bind` and `state.snapshot_write` in the start chain v1-53's live run walks. A `release` with `budget: true` leaves the wheel and appends one `budget-gate` record only when a pause stood, with no span or line of its own. This is v1-13's budget-release clause, which CARRY 4 proves or defers.

@history_item n=67 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (strict-modes-failed on the stamps read; five perf rows) This set the five hyperfine rows (`pre-tool-use` included, on an unstamped session), each required by name by `gate --require perf`. It also added `parse-rejected{ledger-stamps, strict-modes-failed}`, one of the two strict-modes line sites still standing. The `perf` job, one of CI's seven re-read green at the wrap, rests on these rows.

@history_item n=68 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the `diagnostics/` opens behind `check_diagnostics`) A role or detail file opens only past `check_diagnostics`, so the product holds the 0600-in-0700 rule at open, and not only the secret scan. A refusal is a failed open with no panic sink. The live sessions' 0600 acceptance and the scan's 0600 check rest on this.

@history_item n=69 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — (the text_bytes gloss follows the typed text's inner CR rule) `text_bytes` is the typed text's length (CR LF and lone CR as LF, trailing CR/LF stripped), read live on 2.1.287. Paste through the bumped PTY seam must keep logging `text_bytes` only, never text. The live driver session's send lines read this field.
