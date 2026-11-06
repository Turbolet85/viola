@history_item n=1 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — advisories red on RUSTSEC-2026-0441, owned by Version done-check: added the two dependency-security rows this chunk discharges (the red through `portable-pty =0.8.1` → `nix 0.25.1` → `pin-utils 0.1.0`, the remedy a move off `=0.8.1` to a release whose graph drops `pin-utils`, no ignore widened, the `[advisories]` list keeping its one entry; and the pin carrying no Windows reason, portable-pty 0.9.0's Windows garbage-read bug reaching no code viola builds). Item 12 retires both rows by measurement; the owner was the overseer's technical answer plus the operator's word, not the founder's.

@history_item n=2 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — handle inheritance without the Windows mechanism: the `CreateProcessW bInheritHandles = 0` and `inheritable(false)` mechanism was cut, and the line now reads close-on-exec on Linux «recorded, not established», with the `/proc/<child pid>/fd` measurement under the fake agent owed to a CARRY on «Version done-check» (overseer, founder-delegated, 2026-11-06 16:25Z). The ban shrank to «NEVER make channel handles inheritable». Item 15 is that measurement, and it must be taken on the moved portable-pty graph before fct-p7rzex2i is re-worded.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the IPC endpoint is a Unix socket alone: retired the Windows pipe's `inheritable(false)` and DACL controls, leaving `<dir>/viola-<h12>.sock` in `$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`, never the `/tmp` root, chmod 0600 after the bind. This is the wording of the socket-directory pair the founder's word (item 7) must confirm, and the listener is the descriptor the item 15 fd measurement must not find in the child.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — one licence exception and one deny target triple: `0BSD` now has one per-crate exception (`doctest-file`, through interprocess 2.4.4); added the rule that an exception naming a crate absent from the graph is dropped (`license-exception-not-encountered`). The `deny-sync.toml` tokio ban now runs for `x86_64-unknown-linux-gnu` alone. The portable-pty move must re-run this check: drop any exception the new graph no longer reaches, and give any new non-`allow` licence its own exception and Decisions Log entry.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — CI runs one ubuntu-latest leg: `ci.yml` on `[ubuntu-latest]` with `${{ matrix.os }}` names kept, `windows-mutants.yml` and the mutation-scratch ban retired, zizmor `concurrency-limits` at 2 low findings, and the Windows/macOS code recoverable at tag `multi-os-final` (`ecbe7ab3`). The companion «one-leg CI jobs» entry dropped «per-OS» from the `release`/`perf` lines and kept chmod-after-bind as a reason from the macOS era. Every gate green at wrap is read against this CI shape, and the item 17 remnant removal must keep it true.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — client server verification is one Linux flow: `peer_euid_admitted(observed, own)` admits only `observed == Some(own)`; `SERVER_PID_REPORTED`/`PEER_EUID_REPORTED` folded; the frame ban reads the socket-directory, pid, start-time and euid checks, none skipped, no carve-out. This is the shape the item 8 `peer_admitted` survivor record and the live driver session go through.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — child program resolution is a given path or a bare name on PATH: `resolve_program`'s one refusal is `AgentError::ProgramNotFound`, `BatchScriptChild` and the `.cmd`/`.bat` ban retired, `revive --json` codes 11 → 10, verify's refusal codes 6 → 5. After the portable-pty bump, `viola-pty` must still be handed this absolute path, and `AgentError` is one of the derive-census enums item 16 re-measures.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the ConPTY sideload and its vendored binaries retired: `pty_backend()` is always `openpty`, the `sideload_fallback` codes are gone, `PinError::StrictModes` left Error Handling, and only the pinned binary is re-hashed. This grounds the item 14 obs-plan `process-start` re-wording (`pty_backend` `openpty` alone, `sideload_fallback` gone). `PinError` is one of the item 6 divergences, so its remaining variants are what a fold or ratification rests on.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the R8 strip and the fixture scrub match names exactly: every inherited `CLAUDE*` name is stripped except `claude_env_keep`, the eleven-name identity floor always stripped, and the registry reader is retired. The live `viola run` sessions (v1-53, the overseer-drives-builder session) rest on this strip, so evidence must carry no R8-stripped value.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — verify's stop and the colour decision lose their Windows arms: the feed-panic witness now reads the child's bytes reaching the outer terminal with no OS limit, and the colour facts lost the Windows VT enable. This bears on the vt100 `catch_unwind`/passthrough property the portable-pty move must keep, and on the colour-fact carve-out, which the remnant removal must not widen.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the home strict-modes check is owner and mode alone: the Windows DACL arm and the `NotFound` exception were retired, and the board's read rule and `check_path` hold with no «every OS» qualifier, «the OPEN on the board's read rule unchanged». The founder's word on the board's read rule (item 7) lands on this as-built text and closes or keeps that OPEN.

@history_item n=12 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — the macOS carve-out's tests named in the census: introduced `tests/contract_os_gated_tests.rs` and its `GATES` table, which gives every OS-gated test a reason. v1-53's claim reads this census (39 functions, 2 gated files), and a remnant removal under item 17 must keep or justify each gate, e.g. `tests/security_negatives_socket_dir.rs` `#![cfg(unix)]`.

@history_item n=13 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — the RUSTSEC-2026-0386 supply-chain red closed: the direct precedent for item 12. An open-advisory item owned by a route entry was retired whole once an exact-pin bump in `[workspace.dependencies]` plus the lock moved the crate to a patched version: `cargo deny check` (all four families, cargo-deny 0.20.2) exited 0, `deny.toml` was untouched and no ignore was widened. RUSTSEC-2017-0008 is restated as the only ignore. A companion entry re-worded the tokio pin and re-read `channel-tokio-free: ok` and the deny probes after the bump.

@history_item n=14 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — serde_json's `unbounded_depth` banned in `deny.toml`, and the deny-probes verdict at 14/14 with its counting rule named (the `probes=(` rows of `scripts/deny-probes.sh` plus the planted `channel-tokio-edge` probe). The deny probes re-run after the portable-pty move must read this verdict. Any change in the count must re-word it with the rule beside the number.

@history_item n=15 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the tokio ban's tree form and the rmcp release-graph gate: `viola-channel` left `scripts/sync-crates.txt` and is covered by `scripts/channel-tokio-free.sh`; lint gained `scripts/rmcp-features.sh` (rmcp exactly `server` + `transport-io` in `viola`'s release graph), and supply-chain runs the tree form after the sole-root loop. Both gates must still pass over the new portable-pty transitive crates.

@history_item n=16 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the npm graph, the Node pin, the axum ban and the GUI scan forms left. `[[bans.features]]` held the rmcp ban alone (serde_json was added later), every remaining ban still proven live, and the canary scan's classes were narrowed. This sets the deny-ban set and the secret-scan classes CI must read green against. The 2026-10-20 Epoch 5 batch entry recorded the axum ban and the GUI scan forms as «a separate founder item».

@history_item n=17 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** — 0BSD per-crate exceptions: established the rule that `allow` stays `["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]` and that each further exception needs its own Decisions Log entry, never an `allow` addition. It was a boundary widening ratified by the overseer, founder-delegated. Any new licence the portable-pty move pulls in falls under this rule.

@history_item n=18 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — trust boundary, CI jobs, deny.toml additions, nightly.yml: named `deny.toml`'s four families plus the sole-root `deny-sync.toml` tokio ban as the supply-chain trust boundary, recorded that `scripts/deny-probes.sh` proves every ban live, and set the weekly `nightly.yml` advisory run. This is the gate set the item 12 move must keep green.

@history_item n=19 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — fuzz lockfile exemption: `fuzz/` is a separate workspace outside root `cargo deny`, a test-only exemption that holds only while it never links and never joins the root `[workspace]`, with both lockfiles committed. The follow-up entry 2026-09-24-workspace-tree-and-code-graph-planes wired the `supply-chain` step `Fuzz lockfile audit (advisories, sources)` and the nightly fuzz advisories, and added the per-OS release build through `scripts/release-check.sh` to the CI job list. CI green at wrap includes both.

@history_item n=20 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — pinned actions and toolchain source: set the SHA-pinned action set and `rustup toolchain install` from `rust-toolchain.toml`, and ruled that event-payload values reach steps only through `env:`. The later entry 2026-09-28-mutation-testing-to-the-epoch-boundary cut the set to four (`download-artifact` removed) and left one unscanned upload (nightly `fuzz/artifacts/`). These are the CI constraints that must hold while gates are driven green.

@history_item n=21 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — chunk.diff out of the canary scan and the harness upload; `ci.yml` reads no event value. This is the exact-path skip the `secret-scan` step still carries, and the reason the test job's canary scan passes on repository source text.

@history_item n=22 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — CI runs the obs artifact canary scan before uploads: `viola-harness secret-scan` checks every test-home upload against the NEVER-log floor and never prints matched bytes. The chunk's CI run, and any committed live-session evidence, must pass it.

@history_item n=23 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — test-only feature proven absent: `viola-channel`'s `test-support` feature, and `release-check`'s refusal of `test-support`/`fake-agent` artifacts (probe `5/5`). The 2026-09-29-t15-07-57-wrap registry migration later put this rule, and G2's one exact-path exemption, in the body. The release job must still read 5/5 refused with a clean control after the remnant removal.

@history_item n=24 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption, ratified by the founder: `src/cmd/hook/seam.rs:<digits>` is compared as a whole string and its `--probe` runs before every check. Any other seam or G2 exemption needs its own Decisions Log entry. A companion entry added the `perf` job with its own `secret-scan`. The remnant removal and test edits must leave the seam in its own file and G2 at one exemption.

@history_item n=25 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — `FAKE_AGENT_PUMP_DELAY_MS` carve-out: the first test seam, `fake-agent`-only and capped at 5 s, and the Universal NEVER against another seam or env read without a Decisions Log entry. `PTY_SIDELOAD_TEST_CASE` (item 3) is judged against this rule.

@history_item n=26 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the terminal colour facts NO_COLOR / TERM / COLORTERM join the admitted env set: a boundary widening ratified by the founder, and the third carve-out beside the two seams. The extract names it as one of the only carve-outs, so no remnant edit may add another env read.

@history_item n=27 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — founder ratification of three standing boundary widenings: set the standing rule that a boundary widening needs a live answer, never an earlier direction, and that a widening halts the wrap. It also admitted the `supply-chain` artifact (`deny.json`, `deny-fuzz.json`, `zizmor.json`) unscanned, by content. This binds any widening the portable-pty move or the live sessions would bring.

@history_item n=28 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — registry migration (U35): the security-plan Decisions Log moved verbatim to `security-plan-amendments-archive.md`, its in-force items standing in the body (the release check, G2's one exemption). This is why «its own Decisions Log entry» now lands as a live amendment record (fct-lnwynz4l) rather than a body log line.

@history_item n=29 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the product error enums, by census: made the `thiserror` derive census nine (`AgentError` · `ChannelError` · `StateError` · `PinError` · `StampsError` · `StampsReadError` · `Refused` · `McpError` · `ServeError`, read 10 → 9 with `Refusal` folded), retired `CoreError`, and set the fixed-message `Display` list. Item 16 re-measures this count (10 again at HEAD), and item 6 changes the enum set it lists.

@history_item n=30 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the open enum divergences and their owner: `ServeError`, `StampsError`, `PinError`, `StampsReadError`, `Refused` are recorded as open, owned by a CARRY on «Version done-check». The companion entry «viola_agent_claude::Refusal folded into AgentError» is the fold precedent (`StampError` before it), with a trap: a bare `Refusal` token also matches `RefusalReason` and `revive`'s private enum, so search for the backticked name and its variants. The census entry (`tests/contract_error_bodies.rs`, `thiserror_messages_interpolate_nothing`) must stay green after any fold.

@history_item n=31 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the Epoch 6 delegate marks are his: the five divergences' owner lost its «PROVISIONAL, owed the founder's own word», and the CARRY on «Version done-check» stays. The 2026-10-26-explicit-session-links halt confirm stays PROVISIONAL, since the question shown to the founder did not name it. That confirm still needs his word, which bears on the v1-13/v1-36 link clauses.

@history_item n=32 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the socket-directory check, built at the bind and the client seams: built `check_socket_dir`/`check_endpoint_dir`, added `Refused::NotADirectory` with a fixed message, and kept the `thiserror` derives at nine (`Refused` gained a variant, not a derive). This bears on the `Refused` fold/ratification and on the census count.

@history_item n=33 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the client's peer euid check, the wrapper's drop of another euid, `socket-dir-refused`, a refused directory at a hook and at a start, and the leftover socket file: each row passed on the overseer's confirm as delegate, PROVISIONAL, «superseded by the founder's own word in the batch at Epoch 7's boundary». Item 7's owed batch is that supersession, and the socket-directory pair's wording is among these rows.

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the tests holding the directory and peer checks, and no seam to drive them: no test drives a foreign euid; the decisions are held by injected values; and a new ban forbids any env, flag or feature seam to force a refusing peer-euid decision. Each client seam's `peer_admitted` wiring is «the named weakest proof», and an always-admit mutant survives by construction. This is the item 8 survivor, recorded and not reached.

@history_item n=35 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the board's one rule on the home: `board::scan` became `check_read` on the home and `check_path` on `instances/`, so a 0750 home is refused as `ScanRefused::Home`, closing the CARRY of 2026-10-26-home-and-code-bearing-file-integrity («`viola list`'s home-check inconsistency»). The row carries «OPEN — the board's read rule stands on a delegate's answer · owner: the Epoch 7 boundary batch», which item 7's founder word closes or keeps.

@history_item n=36 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the Unix socket's place in the threat model: corrected the socket file from `$TMPDIR/viola-<h12>.sock` to `<dir>/viola-<h12>.sock` in the per-user 0700 directory, and Networking to «never the `/tmp` or `$TMPDIR` root». The threat-model and networking text must agree with the founder's wording of the socket-directory pair.

@history_item n=37 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — budget.json ratified; the board's refusal and the statusline arm's no-trace shape his: the whole-board refusal and its no-line shape became the founder's own (2026-10-13, superseding the operator's answer of 2026-10-11-the-board-viola-list). A companion entry records a delegate card confirm as PROVISIONAL, superseded by his batch word. These are the precedents for how item 7's batch answers are recorded against the board's read rule.

@history_item n=38 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch: the card confirms his, and revive's no-line refusal his. The companion entry lists each superseded provisional record without editing those entries (append-only). This is the template for recording item 7's Epoch 7 batch (the socket-directory pair, the board's read rule, the P5 word, the 151-proposal answer-card confirm).

@history_item n=39 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — the served `link` / `unlink` methods behind the IPC controls: named the nine served methods plus `hook.event` (the record the Universal ban asks for a new channel method), with no `from` read. Its PROVISIONAL halt confirm is still open. The v1-13/v1-36 dispositions must add no channel method and keep this surface exactly.

@history_item n=40 marker=2026-10-10-session-links
**2026-10-10-session-links** — the link-set restore reads the log: recorded that no refusal or permission rests on `from` or the link set. At the time the explicit verbs were not built and their boundary widening awaited the founder's own word. This is background for v1-36's board link-set clause, deferred or re-worded, never built here.

@history_item n=41 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the MCP-over-stdio threat vector bounded at its stdin lines: kept the clause that leaving `release`, `pause`, `link` and `unlink` off the MCP tool list is an affordance, not an enforcement. v1-13's MCP `link` result is settled by deferral or re-wording, never by adding those tools.

@history_item n=42 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the seventh dated gap: pause and release frames. It added the driver-originated `release` refusal (`release-from-driver`, the founder's ruling F-W1), and 2026-10-10-budget-governor added that the string-`from` refusal comes before `budget` is acted on, so it lifts no pause. Both constrain how v1-13's budget-release clause can be disposed.

@history_item n=43 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — a new attack-surface vector: the human's terminal on `viola run`'s stdin. It set the closed non-editing list (F-W2), widened once by seven exact-grammar reply shapes on the founder's word, with «no reply added unmeasured». Item 10's `CSI ? 6 n` / kitty-graphics hypothesis joins the list only by the same live measurement and his word. The headless rig sessions widen nothing by themselves.

@history_item n=44 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — two scratch measurements across the credential boundary, ratified by the founder. Standing rule: the answers are spent, and another peer message or names probe needs its own showing. The 2026-10-19-newer-claude-cli-rows footer reading repeats it: «no product path and no standing permission». Each of this chunk's live sessions must rest on the directive's own word, never on these.

@history_item n=45 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the live round's rig outside the work tree, its trust keys and residual: keys go only into the rig's own pty, never a desktop key; `viola release <name>` comes before the driver's send; and the residual is the CLI's own `~/.claude` writes. This is the precedent for the headless pty rig the overseer-drives-builder and v1-53 sessions run on.

@history_item n=46 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's typed-input PTY probe: Repository fixtures keep only signature rows, and the refusal `a recorded fixture is not clean: <file> <code>` uses closed codes `home-path` · `absolute-path` · `username` · `email`. Later entries extended the same scrub: 2026-10-13-owed-ledger-rows (the kept `workspace.repo`) and 2026-10-20-epoch-5-cleanup-ii (the `viola-mcp-tap` `Mcp.` exchanges). Any fixture this chunk's live sessions record must pass it.

@history_item n=47 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — one tree's release builds can key two pinned folders. Measurement B (six cold builds, 3 unloaded sections unequal) is recorded with no cause claimed, as a founder item and a CARRY on «Version done-check». The companion «first-start race closed» entry keeps the SHA-256 re-hash of a present copy before reuse (`PinError::HashMismatch` on different bytes, left as found). Item 5 is recorded only, and acting on it is the founder's.

@history_item n=48 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` validated into `DialogHold`, with the dialog reply bound at the ceiling: 60–3600 s, no env var or flag, the hook's reply read bounded at `DIALOG_HOLD_MAX + 5 s` (3605 s), a widening the founder answered himself. Item 2's live hold near the ceiling is read inside these bounds, never by reconfiguring them.

@history_item n=49 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 degrade as landed: the feed runs under `catch_unwind` with the human's bytes first, a panic poisons the model to `input-not-ready`, the panic sizes are measured, and `host_size` never yields zero. 2026-10-04-confirmed-send-with-cl-1-records bounded the feed queue (`FEED_CAPACITY` 256, `try_send`, passthrough never blocked). `crates/viola-pty` is touched by the portable-pty move, which must keep both.

@history_item n=50 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — child resolution as built, and `PtyError`'s `Display` hand-written (the other enums `thiserror`), with fixed messages. A `PtyError` change forced by the new portable-pty release must keep a hand-written fixed `Display` free of upstream text and paths.

@history_item n=51 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — test-side removals reach the dev host's tmpfs backing through the home link: added the ban that harness `cleanup` removes only `viola-session-` dirs it created, and the keepers' link checks (`Workspace::ensure_e2e_home`). The trap: with the relative-target check neutralised, a keeper makes `backing/` in the working tree. 2026-10-20-epoch-5-cleanup added the 9 GiB headroom refusal. Item 17's `session.rs:52` and `EXE_SUFFIX` edits in `crates/viola-e2e/src/harness/` must leave both intact.

@history_item n=52 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the loopback GUI leaves 0.1.0: API Security holds the stub «The frontend is deferred to 0.2.0; its technology is not chosen.» (the founder's ruling). A cap whose acceptance names a frontend surface is unprovable in 0.1.0 and reads deferred only on his word.

@history_item n=53 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `ledger/stamps.json` checked before `ledger/` is made: gave `StampsError` (`StrictModes` · `State`) its fixed message. The companion «`check_instance`'s set and the read rule» entry gave `Refused` its `Readable`/`Symlink` fixed messages. These are the fixed-message facts an item 6 fold of `StampsError` beside `StateError`, or of `Refused`, must keep.

@history_item n=54 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the frame ban met in full by every frame: the eight dated strict-modes gaps closed, with no frame taking its reference values from an unchecked snapshot. This is the frame-before-verification ban (fct-izf47ccx) the live driver session and the survivor record stand on.
