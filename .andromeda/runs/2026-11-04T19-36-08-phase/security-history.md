@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (pinned actions and toolchain source) fixed `ci.yml`'s SHA-pinned action set (checkout with `persist-credentials: false`), the rustup-from-`rust-toolchain.toml` toolchain and the rule that event-payload values reach steps only through `env:`; any edit this chunk makes to the three-OS matrix's test, perf or lint legs must keep those pins and that rule.

@history_item n=2 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — made CI run obs-plan §9's artifact canary scan (`viola-harness secret-scan`) before every test-home upload, never printing matched bytes; the chunk's Unix legs must keep that scan gating their uploads (extract binding security ↔ tests CI).

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — wired the `supply-chain` job's fuzz lockfile audit and put the per-OS release build through `scripts/release-check.sh` (refuses any test-only binary) into the CI job list; making the Unix legs green must leave both running unchanged (cascade g5 binding 4, g5 n=6).

@history_item n=4 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — defined R8 (prefix strip + persistent set + 11-name identity floor, names only), the Unix pass-list `claude_env_keep` (at most 32 names, rejected whole, floor names ineffective) and child resolution to an absolute path; the openpty spawn must apply the same R8 strip with the Unix pass-list and hand portable-pty the resolved absolute path (cascade g1 binding 4, g4 n=6/n=13).

@history_item n=5 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — admitted `FAKE_AGENT_PUMP_DELAY_MS` as the first test seam (`fake-agent` only, capped at 5 000 ms, compiled out of release) and wrote the Universal NEVER against another seam or env read without its own Decisions Log entry; the Unix suite now reaches this seam, and any Unix-only seam the chunk adds falls under that rule (scope §Boundaries).

@history_item n=6 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — made `viola_state::fs::replace_private` the one writer of replaced files, its mode set on the temp file before any byte; the extract's 0600-before-any-byte constraint for every Unix test home and diagnostics file rests on it.

@history_item n=7 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — (listener hardening as landed) replaced `ListenerOptionsExt::mode(0o600)` with chmod 0600 right after the bind on every Unix OS, because `mode` returns `Unsupported` on macOS and fails the bind (ignoring it left the socket at the umask mode); the anti-pattern banning `mode` and the 0600 secondary control are what the parity suite exercises on `macos-latest` (extract anti_patterns).

@history_item n=8 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — (founder ratification of three standing boundary widenings) ratified the `FAKE_AGENT_PUMP_DELAY_MS` seam and set the standing rule that a boundary widening needs a live founder answer, an earlier direction never ratifying one; a widening this chunk finds is a P4 fork on his live word (scope §Boundaries).

@history_item n=9 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — recorded `release-check`'s refusal of `test-support` / `fake-agent` artifacts (probe 5/5); the per-OS release build on the Unix legs must keep refusing the `fake-agent` feature the Unix suite now runs (cascade g5 binding 1).

@history_item n=10 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — set the hook stdin read at `take(MAX_FRAME + 1)` and its fail-open details `oversize-stdin` · `malformed-json` · `channel-unreachable`; these closed `hook-decision` words must read the same on Linux and macOS under the one diagnostics schema (cascade g4 n=3).

@history_item n=11 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption) added the second test seam (panics only on exactly `1`, fails open) and G2's one exemption at exactly `src/cmd/hook/seam.rs:<digits>` (whole-string, `--probe` first), founder-ratified; G2 re-measured green on `ubuntu-latest` and `macos-latest` must carry exactly this exemption and no other (cascade g5 n=3, binding 1).

@history_item n=12 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (the per-OS `perf` job) put `agent-run run --perf`, G2, G4 and its own `secret-scan` in the per-OS perf job, `perf-<os>` / `diag-perf-<os>` uploading only on a passed scan; the chunk re-measures this job on the Unix legs (cascade g5 n=5 run=3).

@history_item n=13 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (MAX_FRAME consumers, verify's CI split) fixed that the real CLI runs only locally and `viola verify` runs in CI only against the fake agent; with no live session in this chunk, its Unix legs run verify against the fake agent alone (cascade g1 n=18).

@history_item n=14 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — added the System32-only DLL search as main's second statement, the ConPTY companions' re-hash, and the `windows-2025`-only `ConPTY vendor verification` CI step with its four pins in `src/conpty.rs`; a shared pty-seam, `main` or `ci.yml` change for Unix parity must leave all of them as they are (scope: Windows stays the live-supported target; cascade g4 n=16, g5 n=1).

@history_item n=15 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — moved the Decisions Log's in-force items into the body, among them `release-check`'s refusal of any non-`viola` executable and `test-support` / `fake-agent` artifacts, and G2's one exact-path exemption with its look-alike `--probe` (the perf arm exempting nothing); both bind the Unix legs (cascade g5 binding 1, g8 binding 3).

@history_item n=16 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — made `pre-push` run only on a Linux host, launching every child under `env -i` with HOME and PATH only, so no `CLAUDE*` value crosses; the Unix fake-agent tests the chunk extends run under this gate on the dev host (cascade g8 binding 5).

@history_item n=17 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — landed the vt100 feed under `catch_unwind` behind a tee that writes the human's bytes first, a caught panic poisoning the model with one `parse-rejected{vt100-feed, panicked}` line; the openpty pump must keep the same tee and degrade (extract constraint on PTY output bytes).

@history_item n=18 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (send's input validation as landed; the feed bound closed) made the paste one bracketed `write_all` through `viola-pty`'s `PasteHandle` and bounded the feed queue at `FEED_CAPACITY` (256, `try_send` after the human's write, never blocking passthrough); the openpty backend must keep both (extract constraint on the bounded queue; cascade g1 n=8, g4 n=1).

@history_item n=19 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — banned any test-side removal under `target/e2e-home` but of a dir the test side created and bound the two keepers to follow only an absolute, owner-only, non-link backing (on the Linux dev host the home link reaches the temp-dir backing); every Unix fake-agent test the chunk runs makes and drops its home only through those keepers. Trap recorded: a mutation run of the harness keeper with the relative-target check neutralised leaves `backing/` in its copied tree (cascade g8 n=2).

@history_item n=20 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — recorded the Linux dev-host reading of the hook's `CLAUDE*` environment (12 names, four outside the eleven-name floor, three of the floor absent) and kept the floor at eleven, the prefix rule removing the rest; the R8 strip on Linux and macOS and the scan's twelve `claude-stripped` needles rest on it, and another names probe needs its own showing.

@history_item n=21 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — named the human's terminal on `viola run`'s stdin as an attack-surface vector whose trust boundary is the closed non-editing list (F-W2), widened once by seven exact-grammar reply shapes with no reply added unmeasured; the chunk's CARRY of the cursor-position and kitty replies may add no shape under the fake agent (cascade g1 binding 6).

@history_item n=22 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (trailing CR and LF removed; an empty typed text refused) added the `not-delivered` / `empty-text` refusal beside `control-character` on both sides; the paste's closed refusal words must read the same through openpty as through ConPTY (cascade g4 n=1).

@history_item n=23 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (the revived child) ratified the snapshot's recorded `cwd` as the spawn directory, the `cwd` kept in the 0600 snapshot only, and recorded the session id as one argv element built by `resume_args`; the kill-and-revive case re-read on Unix spawns that child through openpty (scope freight; cascade g1 n=4, g4 n=14).

@history_item n=24 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (revive reads its own state) put `revive` under `check_instance` before any value is used, proven by `cfg(unix)` cases only, a Windows case then owed; precedent for the chunk's cfg audit that a gated test names why its behaviour is one OS's.

@history_item n=25 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (the statusline source and the shell-out) made `hook statusline` the only shell-out, Unix only, the command one argument of `sh -c`, stdout capped and unparsed, stderr discarded, never logged, bounded by `STATUSLINE_DEADLINE`; the Unix suite now exercises it on both Unix OSes, the Windows shell-out owed to "Windows-only live measurements" (extract patterns; cascade g4 binding 1).

@history_item n=26 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (the settings override as landed, the hook rows, the arm's check before its log) wrote `instances/<name>/settings.json` on Unix only and only for a shell-plain pinned path, and ran the arm's `check_instance` before its log because obs init narrows an existing home to 0700 (a check after it passed a 0770 home); the Unix suite must hold the override on Linux and macOS and absent on Windows, and place every check before any log.

@history_item n=27 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (the two instance checks ahead of any log) measured that a log opened before or after a refused check narrows the home to 0700 and erases the state the check refuses, and moved `revive`'s and `hook statusline`'s checks first; a trap for any Unix negative the chunk adds over a widened home.

@history_item n=28 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — made `viola list` a strict-modes entry point (whole-board refusal, no log, no line, `cfg(unix)` cases only) and its rows escaped through `escape_row`; the v1-40 stale-row witness under the outer PTY reads this board (cascade g4 binding 5).

@history_item n=29 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — moved the `windows-mutants.yml` matrix to ten items, forced by the repository's contract test that every `cfg(windows)` product file is scored on Windows; a trap if the openpty work changes which product files are `cfg(windows)`, since the matrix must follow.

@history_item n=30 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — gave Run B a probe `--settings` on Unix only and made the stamp read over a per-OS relied set (Unix counting the statusline rows, Windows not); the first documented per-OS divergence in the stamp counter, which a dialog case moved off `cfg(windows)` must assert per OS (cascade g8 n=3).

@history_item n=31 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — narrowed the canary scan's classes to the stripped `CLAUDE*` canaries, `cookie:`, the content canary and a non-`0600` diagnostics file (Unix), and set the pre-push `env -i` PATH; the Unix-only non-0600 class is the one the extract's acceptance needs passing on both Unix legs.

@history_item n=32 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added `list`'s `claude agents --json` child (R8 strip, `take(MAX_FRAME + 1)`, 5 s kill, closed `AgentsFailure`, a failed read leaving the board as read) and unwrapped names printed through `escape_row`; the v1-40 witness's `viola list` makes this read on Unix (cascade g4 n=11).

@history_item n=33 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — admitted `NO_COLOR`, `TERM`, `COLORTERM` for `viola list`'s human mode only, compared and never echoed, deciding SGR depth only, with a NEVER on any further env read without its own entry; the v1-40 witness may set them only to choose SGR bytes (cascade g4 n=9, g8 n=4).

@history_item n=34 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (MCP tool input validation and error bodies as landed) fixed MCP `isError` bodies to `invalid-params` with the closed field words and `refused: <code>` text; part of the documents the one schema must keep identical on every OS (cascade g4 n=4).

@history_item n=35 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the tokio ban's tree form and the rmcp release-graph gate) added the lint job's per-OS `rmcp features` step (`scripts/rmcp-features.sh`); the lint job re-measured on the Unix legs carries it.

@history_item n=36 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (question option text crosses into the event and `wait` payloads) widened `question` to carry `option_details` into `events.ndjson`, `hook.dialog`, CLI `wait --json` and MCP `wait`, and `background_tasks` to a count; these content-bearing fields must keep the same keys on every OS under the one schema (cascade g1 n=1 run=2).

@history_item n=37 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — set the current stamp gate at twenty-four rows, counter `/24` on Unix and `/21` on Windows (`20 pass  4 fail` Unix, `17 pass  4 fail` Windows reading `cli_verified:false`); a dialog-answering case moved off `cfg(windows)` must assert the per-OS counter, never one OS's value on the other (cascade g8 n=3, binding 6).

@history_item n=38 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — made the e2e-home keepers refuse a linked backing below the 9 GiB headroom floor (`statvfs`, a compiled constant, a fixed message naming no path); the Unix suite on the dev host meets this refusal, beside the directive's `df -h` read before a stamped-home gate.

@history_item n=39 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** — asserted the Windows pipe's remote-client rejection and DACL effect on `test (windows-2025)` (`tests/security_negatives_channel.rs`, Windows only); these Windows-only witnesses stay `cfg(windows)` with their reason named and must keep holding when a pty-seam change is shared with Unix (cascade g3 n=13/14, binding 6).

@history_item n=40 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the client's server-pid read and the Unix client euid check as built) read the server pid through `peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId`, none on macOS; the source of the per-OS split the parity suite must read, pid and start time on Linux only.

@history_item n=41 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the observed server pid and the verdict recorded as built) made `SERVER_PID_REPORTED` false on macOS so the verdict is `Matches` with no comparison there, and compiled the server-identity negatives out on macOS; the carve-out the scope keeps, which no parity test may read as a comparison (cascade g3 n=4/n=6).

@history_item n=42 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (a failed server check: no frame, and what each client reports) fixed the outputs of a failed check: CLI exit 21 with one `process-exit` WARN line, MCP no line, `hook` exit 0 with a `server-verify-failed` `hook-decision`, no pid, start time, path or endpoint anywhere; these must read the same on Linux and macOS (cascade g6 n=2).

@history_item n=43 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (`viola link` / `viola unlink`'s instance check before any log) recorded the trap that opening a role file narrows an existing home to 0700, so a check after `client::start` passes a 0770 home; any Unix negative the chunk adds must read the mode after the refusal, as `link_verbs_refuse_a_widened_instance_before_any_log` does.

@history_item n=44 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the tests holding the link verbs' controls) placed the Unix-only widened-instance case and the server-identity table's `link` / `unlink` rows on Linux and Windows, compiled out on macOS with the file; the chunk's cfg audit keeps each gate on the OSes its control holds (cascade g3 n=11).

@history_item n=45 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the link lines the served methods write) fixed `events.ndjson` link lines at `data: {driver, driven}` and the `channel-request` / `channel-response` pair at `method` with no name; the one schema must keep these fields on every OS (cascade g4 n=19/n=20, binding 6).

@history_item n=46 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`check_instance`'s set and the read rule as built) extended the Unix read rule to `diagnostics/` (`unix_read_verdict`, any group or other bit) and added the fixed `Readable` / `Symlink` refusals; every Unix test home must pass it on both Unix OSes (cascade g2 n=6).

@history_item n=47 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the Windows legs measured) recorded the one stated Unix/Windows divergence, an unstattable home refused on Unix and stat'd `NotFound` on Windows, and named `tests/security_negatives_strict_modes.rs` and `tests/chaos_snapshot_replay.rs` green on all three OSes; a test left OS-gated over that path must name its reason, and no Windows pin moves (cascade g7 binding 2).

@history_item n=48 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the six CLI frame verbs share `link` / `unlink`'s instance check) put `refuse_untrusted`'s `check_instance` first in the frame verbs, proven for the six verbs on all three OSes and for `link` / `unlink` by `cfg(unix)` cases only; a precedent for what the cfg audit finds already shared.

@history_item n=49 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`viola hook`'s event and dialog paths check the instance before obs init) canonicalised `VIOLA_DIR` and ran `check_instance` before the diagnostics level and obs init, a refused instance exiting 0 with no trace; the hook contract the Unix suite reads on both Unix OSes.

@history_item n=50 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home tree checked at `run` and `revive`'s start) added `check_home_tree` before the wrapper log, a refusal exit 1 with no log, line or child; the Unix wrapper start under openpty must hold the owner-and-mode check there (cascade g1 n=13, g2 n=7).

@history_item n=51 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the diagnostics opens refuse a symlink or a widened path) ran `check_diagnostics` before `ensure_private_dir`, a refusal ending at `std::io::sink`; every diagnostics open on Linux and macOS takes the Unix read rule (cascade g4 n=21, g7 n=15).

@history_item n=52 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the snapshot readers move onto `read_snapshot_or_replay`) moved every product snapshot reader onto the replay, a replayed snapshot giving no endpoint, the hook failing open as `channel-unreachable` and `hook statusline` running nothing; these fail-opens must read the same on every OS the unified suite runs on (cascade g6 n=4/n=5).

@history_item n=53 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (the pinned `viola` placed by `place_private_new`) placed the pinned copy at 0700 by a no-clobber placement with its write handle closed first; the Unix modes the suite reads (0700 pinned copy, 0600 files) rest on it (cascade g4 n=18 run=3).

@history_item n=54 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (the first-start race on the pinned copy closed) measured `ETXTBSY` (raw 26 on Linux) on two-instance boots and closed it by placing with the handle closed, a lost placement re-hashed; a Unix-specific failure mode any multi-instance case in the Unix suite can meet.

@history_item n=55 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the vt100 passthrough after a caught feed panic) added `chaos_feed_panic_refuses_send_and_keeps_passthrough`, reading the child's bytes on the outer terminal on Unix and only the absence of error words on every OS, the Windows byte reading owed to "Windows-only live measurements"; its Unix witness must stay green on both Unix legs (cascade g4 n=12).

@history_item n=56 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader census) added `tests/contract_bounded_readers.rs`, requiring every product reader to be bounded in its statement or named in `EXCEPTIONS`; any reader the Unix PTY path adds must pass it (cascade g4 n=17).

@history_item n=57 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (channel-response's closed detail word) closed `channel-response`'s `detail` to three words in `schemas/diag-line.v1.json`, written on both server and client lines; the one-schema rule requires it identical on every OS (cascade g6 n=8, g4 n=2).

@history_item n=58 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the clients' screen of a peer's reply) made `sanitised_fault` and `is_code_word` the only path a peer's reply reaches a CLI `--json` document or MCP result; part of the closed-words floor the extract's error-handling acceptance holds on Linux and macOS.

@history_item n=59 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the error bodies' census and what it does not drive) added `tests/contract_error_bodies.rs` and the `absolute-path` · `caused-by` · `serde-path` · `planted` error-body check, and recorded that no `-32603` is driven in a scanned home; any error writer the schema alignment adds must be named there, and the Unix suite's scanned homes must drive no `-32603` (cascade g6 binding 5, g7 n=12/n=14).

@history_item n=60 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the product error enums, by census; `Refusal` folded into `AgentError`) fixed the nine `thiserror` enums with fixed messages and made `resolve_program`'s refusals `AgentError::BatchScriptChild` / `ProgramNotFound`; the Unix wrapper's resolution refuses only through them (cascade g7 n=9/n=11).

@history_item n=61 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21 names its cause in detail) made exit 21's `detail` a closed set written only by the two clients (`UnreachableCause`, no serde path); the closed words every OS must share, no OS-only word added (cascade g6 n=12, g7 n=2).

@history_item n=62 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the no-endpoint reading) added `without_endpoint`, `instance-dead` for an instance directory and `unwrapped-name` otherwise, opening nothing; one more closed reading that must match across OSes (cascade g3 n=9).

@history_item n=63 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site's document under --json; the exit-cause table; verify's refusal codes; revive --json documents) added the `internal-error` document, `tests/cross_exit_causes.rs` (`required-features = ["fake-agent"]`), verify's closed `process-exit` codes and revive's closed exit-1 codes; the cause table runs in the Unix suite, and each document must read the same on every OS (cascade g7 n=8/n=13, g4 n=5, g4 n=22).

@history_item n=64 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory check, built at the bind and the client seams) built `check_socket_dir` / `check_endpoint_dir` (one `symlink_metadata` read, never a chmod, off Unix `Ok(())`) at the bind and the three client seams, none inside `Client::connect`; the controls the parity suite now reads on both Unix OSes (cascade g2 n=1).

@history_item n=65 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused`, the sixth cause and the eleventh code) added the closed word to exit 21's causes and `revive --json`'s codes, its refusal naming only the `ViolaName`; founder-ratified, its rows PROVISIONAL on the overseer's confirm until the Epoch 7 boundary batch, which this chunk precedes (cascade g6 n=12, g7 n=3/n=4).

@history_item n=66 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory at a hook and at a start) made the hook fail open with one WARN `socket-dir-refused` `hook-decision` and the start exit 1 with one `process-exit` before any snapshot, child or bind (`socket_dir_ready`); the Unix suite reads both on Linux and macOS (cascade g6 n=3, g7 n=7).

@history_item n=67 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a killed wrapper's leftover socket file) chose to leave a killed wrapper's socket file until a next bind takes it over, nothing removing it; the kill-and-revive case on Unix must expect it (cascade g2 n=2).

@history_item n=68 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the wrapper's drop of a peer of another euid) built the drop in `accept_loop` on the accept thread before any frame, the peer reading end-of-file with no line; a Unix test must witness it by end-of-file, never a log line (cascade g6 n=9, binding 3).

@history_item n=69 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the client's peer euid check and the order at a seam) built `peer_admitted` at the three client seams (an unreadable euid refused, `PEER_EUID_REPORTED = cfg!(unix)`) and fixed the order: directory check, connect, euid decision, server verification; a refused euid reads `server-verify-failed` (cascade g3 n=5, g7 n=1).

@history_item n=70 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the board's one rule on the home) made `board::scan` hold the home to `check_read` on every OS, proven on all three, under an OPEN marker owned by the Epoch 7 boundary batch; the v1-40 witness's `viola list` runs on a home that must pass it (cascade g2 n=8).

@history_item n=71 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (macOS's absent pid comparison, a named carve-out) made macOS's missing pid and start-time check the only frame-ban exception, owned by no route entry, `server-verify-failed` there an euid mismatch alone, the server-identity negatives `#![cfg(not(target_os = "macos"))]`, and recorded `macos-latest` reading `socket-dir-refused` over a 0755 or symlinked directory; the scope's macOS boundary rests on this entry.

@history_item n=72 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the tests holding the directory and peer checks, and no seam to drive them) put `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`) on both Unix legs, held foreign euid and owner decisions by injected values only, and banned any env, flag or feature seam forcing a refusing peer-euid decision; the scope's no-seam boundary and the Unix suite's security negatives (cascade g3 n=3/n=12, g8 n=5).

@history_item n=73 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the Unix socket's place in the threat model) placed the socket at `<dir>/viola-<h12>.sock` in the verified per-user 0700 directory, never the temp-dir root, refused when another user owns, loosens or links it; the extract's anti-pattern on socket placement (cascade g1 n=5).
