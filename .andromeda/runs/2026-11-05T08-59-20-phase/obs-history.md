@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — §9 Platform: `ci.yml` stays the single push/PR workflow, and the scheduled `nightly.yml` sits beside it. This chunk cuts Windows and macOS jobs from both workflows, so §9's Platform line is the text the wrap's per-OS amendment re-words. `nightly.yml` stays, with Linux jobs only.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — §9 Platform: `nightly.yml` runs advisories plus the fuzz time-box. §8 item 6 admits the nightly `fuzz/artifacts/` upload on failure, from the synthetic corpus only. This chunk edits `nightly.yml` and keeps its `fuzz` job, so that upload must stay `failure()`-only and corpus-synthetic after the OS legs leave.

@history_item n=3 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — Retired the inner-allow exemption. Raw `event!` is now caught by a fail-closed grep in `scripts/lint-probes.sh`, proven both ways, which runs on the Linux lint leg, and a `lint-probes.sh` failure is a build failure (§10). That Linux lint leg is what remains after the Windows and macOS lint jobs are cut, so it must keep running the probes.

@history_item n=4 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — Admitted the `ci.yml` `supply-chain` artifact (`deny.json`, `deny-fuzz.json`, `zizmor.json`) as an unscanned upload, by content: cargo-deny and zizmor diagnostics only, no absolute path. This chunk edits `deny.toml` and the manifests, and the `cargo deny` output it changes is this artifact. The guard clause stays binding.

@history_item n=5 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — Schema default-deny is a top-level `unevaluatedProperties: false` in `schemas/diag-line.v1.json`, and `diag-detail.v1.json` is self-contained. This is the trap for any schema edit here: if `sideload_fallback` or the `conpty*` words leave the schema, a line that still carries them fails G4. Code and schema must move together, and every Linux value must be kept.

@history_item n=6 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — Introduced the `.cmd`/`.bat` refusal (`batch-script-child`, decided by program resolution before the strip plan and `pty.spawn`), its two fixed stderr lines, `env_kept`, D-34 (the R8 strip) and `PtyError`'s hand-written fixed `Display` (no thiserror in `viola-pty`). This is the origin of the Windows-only refusal text the scope makes P3 list and P4 decide. It is also the origin of `viola-pty`'s error type, which the Windows-arm removal touches.

@history_item n=7 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — The `harness-<os>` upload excludes `target/agent-run/chunk.diff`, which the scan also skips. In the Mutation row, each mutant is judged only by the legs whose `#[cfg]`s compile its line. With only the Linux leg left, the per-leg cfg rule collapses: once their gate is lifted, `cfg(unix)` twins are judged on Linux. The `harness-<os>` path exclusion must survive the matrix shrinking.

@history_item n=8 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — Added the local `viola-harness pre-push` union over the CI legs, a filter and never the verdict of record, whose document carries no absolute path. The scope removes "any pre-push or harness step that runs only for those OSes". This entry is where the pre-push gate's leg logic was first written into obs §8 and §9.

@history_item n=9 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — D-28's concurrent-append check landed for the hook files on 3 OSes (8 processes, 16 + 8 lines, none over 4 KiB). It is the §3 Logging stack per-OS row (windows-2025 / macos-latest / ubuntu-latest) that the wrap's amendment shrinks to Linux, and its Linux run must stay.

@history_item n=10 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the `perf` job's uploads) — Moved perf out of the `test` job into its own per-OS `perf` job with its own G2, G4 and scan. Its `perf-<os>` and `diag-perf-<os>` upload only on a successful scan, and a failed scan uploads `secret-scan-perf-<os>`. This is the job reduced to its Linux leg here, and its scan-gated per-OS names must still resolve.

@history_item n=11 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's seam exemption, D-28 over-4 KiB) — G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first, in the `test` and `perf` jobs). Its one exemption is a panic at exactly `src/cmd/hook/seam.rs:<digits>`, and D-28's over-4 KiB half (8 concurrent forced panics) landed on 3 OSes. G2 must stay in both remaining Linux jobs, and the 3-OS reading of `hook_panics_append_whole_lines_over_4_kib_side_by_side` keeps only its Linux run.

@history_item n=12 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — `run`'s one pre-spawn stderr exception became every start refusal, the `.cmd`/`.bat` child included, written through `src/human.rs` `refuse` → `write_refusal` with no print macro and no `#[allow]`. If P4 retires the `.cmd`/`.bat` refusal, this writer and its §11 print-ban carve-out are where the change lands.

@history_item n=13 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (raw panic frames) — The detail-line backtrace became raw frames from `src/panic_frames.rs`: `RtlCaptureStackBackTrace` + `GetModuleHandleExW`/`GetModuleFileNameW` on Windows, `libc::backtrace` + `dladdr` on Unix, not gated by `RUST_BACKTRACE`. The reason was a Windows-runner measurement (symbolising cost 351 ms of a 403 ms hook run). Removing the Windows arm leaves the Unix capture as the one path, with the same line shape. The §7 sentence naming both is an expected wrap amendment.

@history_item n=14 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — The Mutation row left CI: obs code meets cargo-mutants at `agent-run run --mutants` at the epoch boundary and on demand, the per-leg verdict upload is retired, and §10 counts a surviving obs-code mutant as a failure. This is the Linux mutation gate that stays after `windows-mutants.yml` is deleted.

@history_item n=15 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — D-36 added `run.conpty_sideload` (Windows) to the start chain, `pty_backend` `conpty-sideload|conpty|openpty` from `viola_pty::pty_backend()`, and the additive `process-start.sideload_fallback`, closed in `diag-line.v1.json`. This is the closed vocabulary that `crates/viola-pty/src/sideload.rs`, `src/conpty.rs` and `vendor/conpty/` carried. Removing it is removing D-36's words, so per §11 Logs it needs a Decisions Log record (the scope's P3-list / P4-decide item).

@history_item n=16 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — The Decisions Log (D-01…D-36) moved verbatim to `obs-plan-amendments-archive.md` (U35). D-36's in-force items were lifted into §4 Scenario 1: `pty_backend` an open string, `sideload_fallback` a closed enum. The R8 strip's `IDENTITY_FLOOR` (D-34) was lifted into §4 E2. So a closed-word removal's "Decisions Log entry" is now an amendment record in this file, and the lifted D-36 text in §4 is what the wrap re-words.

@history_item n=17 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home until G2, G4, the secret scan and the scan-gated uploads have run, and a test home's drop goes through `remove_owned`. The Linux `test` and `perf` jobs left after the cut must keep the env set, and the `msrv` e2e-home fix must not change when homes are deleted.

@history_item n=18 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (pre-push document and mutation scratch) — The WSL clone retired with the Windows dev host, and the pre-push gate runs natively on Linux. The harness mutation scratch is Windows-host-only (`HOST_SCRATCH = cfg!(windows)`). On the Linux dev host the same-named dir is only the operator's NOCOW `TMPDIR`, and `mutants.out/` stays at the repository root. `HOST_SCRATCH` is a `cfg!(windows)` gate this chunk removes, and §8's Windows sentence is an expected amendment.

@history_item n=19 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (§1 kept current) — §1 stays a labelled verbatim copy of obs-scope.md and is kept current, so a cascade hit inside it is judged like any body amendment (the founder's ruling of 2026-10-04). The ConPTY, TerminateProcess, "on Windows" DACL, "off Unix" and macOS carve-out words in §1 are therefore expected wrap amendments too, not frozen text.

@history_item n=20 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — Named the dispatch-only, report-only `windows-mutants.yml` (the boundary audit's Windows mutation leg, which uploads nothing) in §9 Platform beside `ci.yml` and `nightly.yml`. The Mutation row gained its `run --mutants --package <member> --file …` on `windows-2025`. Founder ruling C2. This chunk deletes that workflow, so both §9 sites are expected wrap amendments.

@history_item n=21 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the chaos home outside the scans, F4) — `tests/chaos_feed_panic.rs` witnesses the feed panic, and its `TestHome::outside_scan()` home is the one carve-out from "every integration home under `target/e2e-home/`" (outside G2, G4, the scan and the upload). The founder ruled this live on 2026-10-04. This chunk edits that test's OS split and must keep its home outside the root, while the `msrv` fix keeps every other home inside it.

@history_item n=22 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — On the Linux dev host `target/e2e-home` is a link to an owner-only tmpfs directory, so G2's `find` start points read `target/e2e-home/` with the trailing slash. Without the slash, a link scope would read the fail-closed `empty scope`. This is the e2e-home backing that the `msrv` free-space cases (`e2e_home_backing_*`) exercise. The fix must keep the root and the slash semantics, and must not let a host's real free space decide the case.

@history_item n=23 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — E2's R8 strip "always removes the 11 names of the identity floor `IDENTITY_FLOOR`, measured on the Windows host", and a Linux dev-host reading stands beside it. The strip's Windows registry and case-insensitive arms leave with the `cfg(windows)` code. The Unix exact-match strip, `claude_env_keep` and the names-only `process-start` fields stay, and the "measured on the Windows host" origin is wording the wrap may re-point.

@history_item n=24 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — The Mutation row's Windows reading of `src/panic_frames.rs` is job `mutants (viola-panic-frames)` (23 tested, 14 caught, 9 left out as `#[cfg(unix)]` twins, run 38036448183). In §10, "survived" leaves out a missed mutant the host never compiles. Once `windows-mutants.yml` is deleted and the twins lose their gate, Linux compiles them, so a missed mutant there is red at `run --mutants`.

@history_item n=25 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — The `statusline-shell` spawn pair is never written on Windows. The start's settings override write (`instances/<name>/settings.json`, Unix only) carries no span and no line by design. When that Unix-only step loses its gate because its sibling is gone, it must stay span-less and line-less, and the "never on Windows" wording becomes expected-amendment text.

@history_item n=26 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — Added `interrupted` (verify stopped by Unix `SIGINT` / `SIGTERM` or Windows Ctrl-C / Ctrl-Break), with the probe dir left on a Windows close, logoff or shutdown. The Windows stop arm leaves with the code. The Unix stop handling and the closed `interrupted` word must stay, and the Windows wording in §4 and §6 is expected-amendment text.

@history_item n=27 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — The second capture arm `hook statusline --capture` is Unix only and uninstrumented by design. `verify`'s relied count `MM` was set per OS (21 Unix, 18 Windows) via `statusline::WRAPPED_HERE`. This is the start of the per-OS `MM` split that the Windows arm's removal collapses to the Unix count.

@history_item n=28 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — `MM` moved to 22 Unix / 19 Windows, and a PTY child killed at a stop carries `child_exit_status` from portable-pty's report. Per-OS `MM` text is expected-amendment wording. On Linux the PTY exit status still comes through the handle wait after the Windows `TerminateProcess` fallback leaves.

@history_item n=29 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `MM` moved to 23 Unix / 20 Windows, with `agents-join` relied on every OS. This is another step in the per-OS relied-count history that this chunk collapses to Linux's figure.

@history_item n=30 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `MM` 24 on Unix, 21 on Windows (the 24th row `local-command-compact`). This is the current figure the cascade binds: Linux keeps 24, and "21 on Windows" is expected-amendment text once `WRAPPED_HERE`'s Windows arm leaves.

@history_item n=31 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — Precedent for an OS- or surface-wide cut. The viola-ui and web-spa surfaces left 0.1.0, but by the operator's answers "the masters stay true to the tree": closed vocabulary with no emitter stayed in the catalogue and enums, marked «reserved: no emitter in 0.1.0; removed with the code by …». This is the model for deciding whether `conpty-sideload` / `conpty` / `sideload_fallback` leave with the code here or are first marked.

@history_item n=32 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the `ui` process and four events) — The precedent removal of closed words with their code: the `ui` process and four events left the enums and `schemas/diag-line.v1.json`, the third-party OFF targets shrank, and §1 was kept current. This shows how removing closed values was recorded (under the founder ruling, in this file) when the code left. It is the template for the conpty closed words.

@history_item n=33 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§5–§11 web rows, CI steps) — §9 lost the browser suite from the `test` job, `junit-<os>` went back to nextest alone, and the stand-until paragraph left. This is the precedent for cutting CI steps and artifact contents from §9 and §8 in one wrap amendment while the scan gate and upload conditions stay as they were.

@history_item n=34 marker=2026-10-25-u55-door
**2026-10-25-u55-door** — obs-plan was split into its body and 10 category files (map `.andromeda/obs-plan.map.tsv`). The per-OS rows this chunk's expected amendments name live across `obs-scope-summary.md`, `ci-integration.md`, `log-coverage.md`, `span-trace-coverage.md`, `error-capture-reporting.md`, `pii-scrubbing-compliance.md` and `slo-invariants-telemetry-budgets.md`, as well as the body. The plan's amendment list must name each file.

@history_item n=35 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the per-OS peer-pid read) — The multi-platform trigger gained the server check's peer-pid read: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId`, macOS none (`SERVER_PID_REPORTED` false, verdict `Matches`). This is the per-OS row whose Windows and macOS halves leave with their arms. Linux's `SO_PEERCRED` comparison and `server-verify-failed` must stay byte for byte.

@history_item n=36 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (server-identity negatives) — `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`) scans every output and the `cli-<name>.ndjson` line with `digit_runs`, on Linux and Windows. Its macOS compile-out gate is one this chunk removes, and the test must keep its Linux scan.

@history_item n=37 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (captured-span cases) — Added `request_refuses_an_unverified_server_inside_its_dispatch_span` per verb and `hook_refuses_an_unverified_server_inside_hook_handle`, through the test-only `cmd::client::refused_inside`, each read red with the check removed. That helper is macOS-gated, and lifting the gate must keep every case running on Linux and still reading red.

@history_item n=38 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server check in the timed hook path) — The spine-hook budget row's timed path holds the server check, with "macOS the recorded gap". The Linux perf leg must keep timing the check, and the macOS clause is expected-amendment text.

@history_item n=39 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`server-verify-failed`: eight CLI frame verbs) — Eight CLI frame verbs write the exit-21 `server-verify-failed` line, each one's case on Linux and Windows and compiled out on macOS (the recorded gap). This adds `link` / `unlink` to the cases behind the macOS gate that this chunk lifts.

@history_item n=40 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`diagnostics/` opens behind `check_diagnostics`) — Role and detail files open only past `strict::check_diagnostics`: folder read rule (Unix: no group or other bit; Windows: no read grant to another SID), and file read rule on Unix, write rule on Windows. A refusal is a failed open (`std::io::sink`). When the Windows arm leaves, the Unix read rule must be the one rule, and the 0600/0700 product enforcement must hold.

@history_item n=41 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (catch-site reach per OS) — `viola verify`'s home-tree check reads differently per OS. On Unix a home under a regular file is a strict-modes refusal with no line. On Windows it stats `NotFound`, and the `internal-error` catch-site line stands (measured in that chunk's CI). The Windows half is expected-amendment text, and the Unix no-line refusal must stay.

@history_item n=42 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`sideload_fallback` gains `strict-modes-failed`) — `sideload_fallback` and `run.conpty_sideload.outcome` gained `strict-modes-failed` (the `conpty/` companion's owner/DACL check, `PinError::StrictModes`), closing the schema enum at five. This is the last change to the closed property this chunk's code removal orphans, and its schema enum edit must be decided with D-36's words.

@history_item n=43 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (refusal telemetry: two strict-modes lines) — A strict-modes failure has a line in two places: the `ledger-stamps` read and, on Windows x64, a refused `conpty/` companion (`sideload_fallback` `strict-modes-failed`). The OS-API reason names `check_instance`, `check_home_tree` and `check_diagnostics` as call sites. Removing the companion leaves the Vector 7 trigger and §1 naming a line site that no longer exists, an expected amendment.

@history_item n=44 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (six frame verbs' instance check) — Added `client::refuse_untrusted` for the frame verbs. `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` on three OSes. Its macOS and Windows legs leave, and its Linux reading must still find no line and no pid, path or endpoint.

@history_item n=45 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check in the timed path) — "The instance check ahead of logger init (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads) is in the timed path." The five rows had max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms, and the overhead gate is vacuous unless the timed instance passes the check. The Linux perf leg keeps this, and the "on Windows" reads are expected-amendment text.

@history_item n=46 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the `human::refuse` census, 15 to 18) — Precedent for re-deriving the census by its rule (`grep -rcE 'human::refuse\(' src`) whenever a refusal site moves. If P4 retires the `.cmd`/`.bat` refusal, `run`'s count must be re-derived the same way.

@history_item n=47 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the feed-panic test's passthrough half) — `chaos_feed_panic_refuses_send_and_keeps_passthrough`: none of the refusal words reaches the outer terminal on any OS, and on Unix the child's bytes `e4 b8 ad` do. The Windows byte reading was owed to the route entry «Windows-only live measurements», which is now retired. This chunk edits the test's OS split, keeps the Unix byte assertion, and the §7 sentence's owner is among the 39 re-pointed master hits.

@history_item n=48 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`v1-47`'s secret-scan reading) — The scan runs after each job's end-to-end run, with `AGENT_RUN_KEEP_HOMES` `1` on the job (`.github/workflows/ci.yml`), the scan `if: always()` and the `viola-chaos-*` home out. The remaining Linux jobs in the edited `ci.yml` must keep exactly this reading.

@history_item n=49 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the nine thiserror derives) — The product's `thiserror::Error` derives read 9, and `AgentError`'s variants include `BatchScriptChild` and `ProgramNotFound` with fixed messages. If P4 retires the `.cmd`/`.bat` refusal, `BatchScriptChild` and this census must be re-read with it.

@history_item n=50 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (every refusal of `revive --json`'s start) — `refused_with` (`src/cmd/run.rs`) prints the exit-1 document for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, plus `internal-error` for a program not found. The `batch-script-child` document, its human pair and its variant must move as one, whichever way P4 decides.

@history_item n=51 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`process-exit` `detail` enum's count) — `schemas/diag-line.v1.json` closes `process-exit` `detail` by the rule "the enum's members", measured per chunk (19 here). Any word this chunk removes from the enum, for example `batch-script-child`, needs the count re-measured by that rule, not copied.

@history_item n=52 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (Founder Direction 4's codes and named carve-outs) — Named the carve-outs outside the distinct-code rule, macOS's missing server check among them, by the founder's word (`inputs#I2` Q4). This is the origin of the macOS carve-out sentence that leaves with the macOS arm as an expected amendment.

@history_item n=53 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — §3's Log format copy was made verbatim to test-plan §3 again, and `v1-47`'s secret-scan reading was ratified with its OPEN gone. Any schema or closed-word change this chunk forces in §3 must keep the copy matching test-plan's, since the two move together.

@history_item n=54 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's closed causes, five to six) — `viola_core::UnreachableCause` holds six words, `socket-dir-refused` added, with no serde path. `refused_with` also prints for `socket-dir-refused`. The scope keeps all six unchanged on Linux.

@history_item n=55 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused` in the log catalogue) — `process-exit` `detail` moved 19 to 20 and `hook-decision` 7 to 8, each counted from the enum's members. These are the current schema counts that this chunk's schema decisions must re-measure, keeping every Linux value.

@history_item n=56 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`server-verify-failed` reaches macOS) — The multi-OS trigger gained the peer euid: Unix `peer_creds`, Windows none (`PEER_EUID_REPORTED` false, DACL plus pid the controls). On macOS `server-verify-failed` became the named carve-out, "an euid mismatch alone", owned by no route entry, and the "Linux, Windows and macOS" parentheticals spread across §1, §2, §4, §5 and §6. This chunk removes that carve-out's code, so every site of the parenthetical is an expected amendment. Linux's pid-and-euid meaning stays.

@history_item n=57 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (Founder Direction 4: macOS named and unowned) — The socket-directory carve-out retired, leaving "macOS comparing no peer pid" as the one named carve-out, owned by no route entry. It leaves with the macOS arm, and the Founder Direction 4 note and row are expected-amendment text.

@history_item n=58 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the OS call sites) — The OS-APIs entity gained peer-euid (Unix), `check_socket_dir` ("viola's one `symlink_metadata`, off Unix no read") and `accept_loop`'s euid read with no line. The "off Unix" clause names code this chunk removes. The `cfg(unix)` checks made unconditional must keep their lines and their silences.

@history_item n=59 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`human::refuse` census) — Census `run` 8 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1, re-derived at this chunk by grep. This is the current figure a `.cmd`/`.bat` retirement would move.

@history_item n=60 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the hook's timed path holds both new checks) — The spine row's timed path became the socket-directory check, the peer euid decision on Unix, then the server check, with macOS "no pid comparison" and the band kept as measured. The Linux perf leg must keep all three checks timed under `SPINE_DEADLINE`.

@history_item n=61 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the wrapper's drop of a refused peer) — `accept_loop` drops a refused-euid stream before any frame, with no span and no line. If this Unix arm is ungated, Linux must keep the drop and its silence, and no seam may force a refusing decision.

@history_item n=62 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (socket-directory PII reading) — `tests/security_negatives_socket_dir.rs` is `cfg(unix)` (Linux and macOS, no Windows case), and the server-identity negatives stay compiled out on macOS. Both gates are among those this chunk lifts or removes, and both tests keep their Linux readings.

@history_item n=63 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client euid check's place) — `peer_admitted` refuses where `PEER_EUID_REPORTED` (`cfg!(unix)`), and on Windows no euid is read. When the Windows arm leaves, the const may fold away, but the Linux refusal must hold byte for byte.

@history_item n=64 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (captured-span cases for the socket directory) — The span recorder `Seen` moved to the test module `refused_seen` of `src/cmd/client.rs`, out of the macOS-gated `refused_inside`, so the two socket-directory cases run on macOS. Lifting the macOS gate merges these homes, and every captured-span case must still run on Linux.

@history_item n=65 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the checks' order by span scenario) — Each scenario's parenthesis became "the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone". These are the four scenario sites (send, wait / last, dialog with answer, pause / release) whose macOS and Windows words leave as one expected amendment.

@history_item n=66 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the per-OS JUnit reading) — Added `scripts/junit-os-sets.py --run <id>`, an operator-pass reader of the three `junit-<os>` artifacts of one run, never a CI step, with output sets `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`. With two OSes gone it has nothing to compare, so P3 reads whether it and its §9 row are retired or re-pointed.

@history_item n=67 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated test census) — Added the §9 row for `tests/contract_os_gated_tests.rs` (`run --e2e`): every OS-gated test function and file is named by a `GATES` row with a reason, it fails on an unnamed gate or a row that no longer stands, and its self-check reads eight planted cases. The scope re-points or retires it with the gates it counted, and its §9 row moves with that decision.
