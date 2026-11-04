@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (CI setup and wired jobs; test-only crate, bins, env vars, paths) — CI installs the toolchain by `rustup toolchain install` with SHA-pinned actions and `permissions: {}`; the `test` job was born as the 3-OS harness leg. Registered `viola-fake-agent` (feature `fake-agent`), `viola-harness`, the `AGENT_RUN_` prefix and `AGENT_RUN_KEEP_HOMES`, home-level `diagnostics/` and `target/e2e-home/`. Why it matters: the three-OS matrix and its native runners are the base this chunk reads green on both Unix legs; any harness-only variable the parity suite needs falls under the `AGENT_RUN_` rule, not a new `VIOLA_*`.

@history_item n=2 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** — Named the root rstest homes `viola-test-*/home` beside the harness `viola-session-*/home`, and made `AGENT_RUN_KEEP_HOMES` read by the root test chain too. Why it matters: the test-home layout under `target/e2e-home/` that G1 to G4 read on the Unix runners rests on this split.

@history_item n=3 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — The tokio ban became `deny-sync.toml` run once per crate in `scripts/sync-crates.txt` as the sole root (an excluded-graph check false-failed under feature unification); `ci.yml` is the push/PR workflow, `nightly.yml` the weekly one. Why it matters: any code the openpty path adds stays inside a sync crate under this ban, checked for all three target triples; the chunk adds no workflow beside `ci.yml`.

@history_item n=4 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — Process-log lines carry no `v` (their version lives in the schema filename); every role's codes-only log goes to its home-level role file; `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json` registered. Why it matters: the chunk's one-schema rule holds these line schemas identical on every OS, the `v` exception included.

@history_item n=5 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — Added the `print_stdout` / `print_stderr` / `dbg_macro` bans, ripgrep 15.2.0 for G1/G3 (via `scripts/install-ripgrep.sh`), runner `jq` for G2 and jsonschema 0.57.0 for `schema-check` (G4); the `test` job follows the obs gate order with scan-gated uploads and the `lint` job wires G1/G3. Why it matters: these tools must be present and green on `ubuntu-latest` and `macos-latest` for the gates the chunk re-reads.

@history_item n=6 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — Wired coverage (cargo-llvm-cov) into the `test` job and recorded the declined `concurrency:` block with its reason. Why it matters: the per-OS coverage floors read under `gate --require coverage` and the standing no-`concurrency:` rule both bind the chunk's CI acceptance.

@history_item n=7 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — The wrapper as built: R8 became a `CLAUDE*` prefix rule (case-insensitive on Windows, `config.json` `claude_env_keep` on Unix), [PTY] recorded `HostTerminal` raw mode, explicit child cwd and viola-side program resolution, `viola-pty` deps portable-pty, windows-sys (Windows) and libc (Unix), and `PtyError` became the one ratified thiserror exception. Why it matters: the openpty leg runs behind this same seam and error type; the R8 strip and `HostTerminal` are per-OS behaviours a parity test must read per OS.

@history_item n=8 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — `FAKE_AGENT_PUMP_DELAY_MS` ratified as the one exception to the `VIOLA_` / `AGENT_RUN_` rule: read only under `cfg(feature = "fake-agent")`, capped, absent from release builds, a boundary widening ratified at wrap. Why it matters: the precedent and form for any Unix-only test seam the parity suite needs, which the scope says takes a Decisions Log entry.

@history_item n=9 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — [Session Liveness]: pid + start time decide `gone` whatever the beat, the same process `live` (beat at most 5 s) or `stale`; the start order as landed; `target/e2e-home` gained the owner record and the gone-owner sweep. Why it matters: the kill-and-revive CARRY and the `v1-40` stale-row witness both rest on this liveness rule on Unix.

@history_item n=10 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — The Unix endpoint moved from a bare temp-dir socket to `<socket dir>/viola-<h12>.sock` in a per-user 0700 dir (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`), chmod 0600 after the bind, with a `.lock` sibling as start arbiter; the test-only `viola-test-chan-<pid>-<label>` namespace registered; libc for the Unix socket dir. Why it matters: the endpoint layout every Unix parity test meets on Linux and macOS.

@history_item n=11 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — Registered the test-only `$TMPDIR/viola-pty-watch/<test name>.report` and the `test-support` feature of viola-channel. Why it matters: the real-PTY watch report is one of the existing rigs for the Unix resize/kill legs.

@history_item n=12 marker=2026-09-27-browser-verdict-reachability
**2026-09-27-browser-verdict-reachability** — Registered `$TMPDIR/viola-root-watch/`, the root test support's `Watch` report. Why it matters: the `Wrapper::kill` / `Watch` rig (bound `WITHIN`) keeps evidence of a killed Unix test.

@history_item n=13 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second test seam; the `perf` CI job) — `FAKE_AGENT_HOOK_PANIC` ratified live by the founder as the second fake-agent seam, with G2's exact-path exemption for its panic lines; the per-OS `perf` job (hyperfine via `cargo install --locked`, `run --perf`, G2, G4, `gate --require perf`) landed. Why it matters: G2 exempts only that seam's lines by exact path, so a new Unix seam cannot ride it; the perf job is one of the three jobs read green on both Unix runners.

@history_item n=14 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap lost its `color` feature because its anstream route read `CLICOLOR` / `NO_COLOR` / `TERM` / `CI`: a dependency's env read breaks the no-env rule like a direct one. Why it matters: a dependency or feature the parity work enables must not add an env read outside the ratified sets.

@history_item n=15 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the ledger and verify; the version gate split) — `viola verify` runs in CI against the fake agent's print mode while the real CLI runs only locally; the root bin took libc `=0.2.189` (Unix) for `src/panic_frames.rs`. Why it matters: CI's `verify` leg on the Unix runners is the fake-agent path whose relied rows the chunk reads per OS.

@history_item n=16 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — CI runs no mutation job; mutation runs only through `run --mutants` at the epoch-boundary audit (founder ruling). Why it matters: the chunk must not add a mutation job to `ci.yml`.

@history_item n=17 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** — Recorded H2: a key written right after a ConPTY resize could be lost; viola never holds a key behind a resize, and a test that writes a key after a resize waits for the child to observe the new size first (0 of 200); the pty-watch dir gained the `.test.report` with `dsr-cpr` counts. Why it matters: the rule a Unix resize-then-key parity test must follow, and the rig's report it reads.

@history_item n=18 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — Homes stamped by `viola verify` against the fake agent; `wait_endpoint_gone` added to `Wrapper::stop` / `stop_keep`; `CI` read only by `viola-harness`. Why it matters: the stop path's endpoint wait the Unix suite runs, distinct from the kill path's.

@history_item n=19 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — `pty_backend()` reads `conpty-sideload` · `conpty` · `openpty`; the sideloaded ConPTY on Windows x64 fails open to inbox; harness, `OuterPty` and viola-pty tests stay inbox; the DA1 start stall recorded, viola silent. Why it matters: Unix must report `openpty`, and no sideload or ConPTY pin may move for a Unix case.

@history_item n=20 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — Test-home removals go through `remove_owned` (`tests/support/home.rs`), deleting `owner.json` last. Why it matters: `tests/support/home.rs`'s keep/remove rules must hold on both Unix runners for the kept homes G1 to G4 read.

@history_item n=21 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (pre-push native on the Linux host) — `pre-push` runs on a Linux host only, natively, every child under `env -i` with only HOME and PATH. Why it matters: the Linux dev host's local gate the chunk's Linux runs go through before CI.

@history_item n=22 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — Added the dispatch-only `windows-mutants.yml`, never a gate or a `ci.yml` dependency, and "No workflow has a `concurrency:` block". Why it matters: the chunk adds no workflow and no `concurrency:` block; `tests/contract_windows_mutation_scope.rs` keeps its file lists equal to the sources.

@history_item n=23 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — The spine deadline became the product constant `viola_core::SPINE_DEADLINE` = 1 s, the bound the perf gate reads, with the hook's `CONNECT_DEADLINE` 750 ms below it. Why it matters: the `perf` job on `ubuntu-latest` and `macos-latest` holds every hook row within this same bound.

@history_item n=24 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (fuzz targets, the chaos test home) — The test-only `$TMPDIR/viola-chaos-*/home` of `tests/chaos_feed_panic.rs` sits outside `target/e2e-home` and so outside G2, G4 and the secret scan (founder ruling). Why it matters: the one home the Unix gate re-read must leave out.

@history_item n=25 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — The `wait` feed update moved inside the append's lock to fold a macOS red where `last` read a stale turn. Why it matters: a precedent that macOS CI surfaces ordering races the other runners miss; a macOS red is folded at its cause, not gated away.

@history_item n=26 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (the dialog tier; the strict stamps read) — Perf exports became five rows with `pre-tool-use`; viola-state took libc (Unix: owner + mode) and windows-sys (Windows: owner + DACL, protected DACL outside the profile) for the strict check. Why it matters: the five exports `gate --require perf` needs on both Unix runners, and the per-OS strict-modes rule a parity test reads per OS.

@history_item n=27 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the closed non-editing list; the Windows console stdin reader) — The wheel's closed non-editing list (focus, mouse, terminal replies), a resize no stdin byte; the Windows platform fact for win32-input-mode; `viola_pty::host_stdin()` is viola's own `ReadConsoleW` on a Windows console, `std::io::stdin()` elsewhere. Why it matters: the openpty pump feeds the same observer and the Unix leg of `host_stdin` is std's; no Windows reader moves.

@history_item n=28 marker=2026-10-07-t05-47-07-wrap
**2026-10-07-t05-47-07-wrap** — H2's real-`claude` key loss after a resize and the DA1 start stall moved to "Windows-only live measurements", blocked on an interactive Windows host. Why it matters: those items stay with that owner; the parity chunk keeps its block and carries.

@history_item n=29 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — On the Linux dev host `target/e2e-home` is a link to a tmpfs dir, prepared by two keepers (`prepare_home_base`, `Workspace::ensure_e2e_home`); no CI runner has the link; a stalled-start red is a finding about the backing, never re-run for green. Why it matters: the chunk's local Linux runs go through this base, the CI runners through plain directories.

@history_item n=30 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (seven terminal reply shapes; first live on the Linux dev host) — The closed non-editing list gained seven measured reply shapes on foot 1.28.0 (standing rule: only measured shapes, each with its exact grammar and negative controls, on the founder's word); Windows became "the first target" with the first live run on the Linux dev host. Why it matters: the CARRY on `CSI ? 6 n` / kitty replies, and the live Linux reading `v1-53` is read against.

@history_item n=31 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — The `WITHIN` = 7 s root waits are counted by the pattern `Instant::now() + WITHIN` under `tests/`, not an ordinal; a wait through `EXIT_WITHIN` or a qualified path is outside the pattern. Why it matters: ungating or adding Unix waits moves this count, which carries its rule.

@history_item n=32 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — The next append heals a torn last line; the reader counts `unknown_kinds`, `unknown_fields`, `torn_lines`; the replay rebuilds an unreadable snapshot. Why it matters: these counts surface in `list --json`'s `skipped`, part of the one-schema documents.

@history_item n=33 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (kill path's endpoint wait; the snapshot's cwd; revive) — `Wrapper::kill` waits on `holder_gone` because a killed wrapper leaves its socket file on Unix (the planned wait could never end there); the snapshot gained `cwd?`; the fake agent gained `--resume` / `--fork-session`. Why it matters: the kill-and-revive CARRY's Unix reading rests on `holder_gone`, and `run_child_ends_when_its_wrapper_is_terminated` stays `cfg(windows)`.

@history_item n=34 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (as landed; four crossings ratified) — The settings override and the `sh -c` shell-out of `hook statusline` are Unix only; on Windows no override is written, no `--settings` passed and no command run, owed to "Windows-only live measurements" by the founder's cut. Why it matters: a per-OS behaviour a parity test reads per OS with its one-OS reason named, never moving the Windows side.

@history_item n=35 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log) — Opening a log sets an existing home to 0700, so a strict-modes check made after it passes a widened home; `revive` and `hook statusline` moved the check ahead of any log. Why it matters: a trap for Unix strict-modes parity tests: an oracle must read the mode before any log opens.

@history_item n=36 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `viola list` reads `instances/` and `budget.json` caller-side and contacts no wrapper; a replayed snapshot gives `live` only, never `stale`; `list` is the first surface showing the skip counts. Why it matters: the `v1-40` witness must stand on a real snapshot with a beat older than 5 s.

@history_item n=37 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — The real-PTY reports became `<test name>.<pid>.report` / `.test.report` so two test processes never share one. Why it matters: the current names of the viola-pty watch rig the Unix legs use.

@history_item n=38 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's stop handles Unix `SIGINT` / `SIGTERM` (libc `sigaction`) and Windows console events; trap recorded: a later chunk adding a `cfg(windows)` product file owes `windows-mutants.yml` its line, forced by `tests/contract_windows_mutation_scope.rs`. Why it matters: any `cfg(windows)` product source the chunk adds or moves must keep that contract green.

@history_item n=39 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (three statusline rows and the per-OS relied set) — The relied set became `LedgerRow::relied()`, filtering by `relied_on(row, statusline::WRAPPED_HERE)`, so the statusline rows are relied only on Unix and `MM` in verify's step lines differs per OS. Why it matters: the origin of the per-OS relied count the fake-agent verify cases must read per OS.

@history_item n=40 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row) — `local-command-compact` made the relied set 24 on Unix and 21 on Windows, step lines `[NN/24]` / `[NN/21]`. Why it matters: the current per-OS counts the Unix verify leg must read (twenty-four on Linux and macOS).

@history_item n=41 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — 0.1.0 is a pure CLI, the frontend deferred to 0.2.0, Tokio only in `viola-mcp`. Why it matters: the scope's boundary keeping the frontend deferred, and the Tokio containment any new pty/run code respects.

@history_item n=42 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Infrastructure Patterns) — The `test` job's `Gate verdict` runs `gate --require coverage,doctest` with no browser suite; `nightly.yml` two jobs. Why it matters: the exact gate the `test` job reads green on the Unix runners.

@history_item n=43 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts: the agents read and join) — CLI `list` makes one `claude agents --json` read; a failed, oversize or malformed read leaves the board as `board::read` gave it (every `status` `unknown`, no error). Why it matters: the `v1-40` witness's `viola list` makes this read on the Unix runners under the fake agent.

@history_item n=44 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Occupied Resources: the fake agent's agents mode) — The fake agent's `agents` mode reads files beside its own executable (no option, no env), by the founder's answer: no third fake-agent env seam. Why it matters: precedent that fake-agent behaviour the Unix suite needs goes through argv or files, never a new env seam.

@history_item n=45 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — viola-e2e's `harness::session` guard reads `AGENT_RUN_KEEP_HOMES` / `AGENT_RUN_KEEP_FAILED` in its `Drop`. Why it matters: one of the readers keeping homes for the obs gates on every runner.

@history_item n=46 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `NO_COLOR` / `TERM` / `COLORTERM` ratified as read-never-set colour names for `viola list`'s human mode; precedent: any further environment read needs its own ratification. Why it matters: the `v1-40` witness's SGR dim is decided only by these three, and the chunk adds no env read.

@history_item n=47 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the Tokio client and viola-channel's tree-form ban) — The sync crates became four (`viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`), `viola-channel` off the sole-root list with its no-feature graph proven by `scripts/channel-tokio-free.sh`. Why it matters: the current shape of the Tokio containment checked on all three OSes.

@history_item n=48 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `pending_dialog` is written before the dialog's event line, the order fix answering a macOS CI red whose cause predated the chunk; a 671 s hold read live on 2.1.287 on Linux. Why it matters: a second macOS-red precedent fixed at its cause, and an earlier live Linux reading `v1-53` may be read against.

@history_item n=49 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set) — The fake agent has thirteen argv options, the count's rule its list. Why it matters: an option the Unix suite adds joins this counted list, with no env.

@history_item n=50 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — The keepers refuse a linked base below `HEADROOM_FLOOR_MIB` (9 GiB) free, a plain-directory base (every CI runner) reading no headroom. Why it matters: the operator's `df -h /tmp` read before a stamped-home gate on the Linux dev host.

@history_item n=51 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the test-only child marker) — `MCP_SERVE_TEST_CHILD` registered as a test-only marker read only under `#[cfg(test)]`, outside `VIOLA_*`. Why it matters: precedent that even a test-only env read is registered; the chunk's acceptance allows no new seam env var outside the ratified sets.

@history_item n=52 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (both clients read the serving pid; the snapshot's pid and started_at; the diagnostics lines) — `server_pid` reads `SO_PEERCRED` on Linux, `GetNamedPipeServerProcessId` on Windows, none on macOS (`SERVER_PID_REPORTED` false), where `server_verdict` returns `Matches` with no comparison: the recorded gap; `server-verify-failed` lines carry no pid, path or endpoint. Why it matters: the macOS carve-out the parity suite must not read as a comparison, and the Linux pid arm it reads.

@history_item n=53 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the server check covers link and unlink) — The CLI frame verbs checking the server became eight with `link` / `unlink`, macOS staying the recorded gap. Why it matters: the surfaces the Unix socket-dir and euid checks reach.

@history_item n=54 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the read rule; the Windows instance-tree cases) — `strict` holds a read rule (Unix: no group or other bit; Windows: no read ACE to an untrusted SID); `check_instance` holds `events.ndjson` to the write rule on Unix and the read rule on Windows; `cfg(windows)` cases prove the DACL legs. Why it matters: strict-modes parity reads each OS's rule on its own runner, the Windows cases staying `cfg(windows)` with their reason.

@history_item n=55 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (three Windows readings for tests) — A path under a regular file stats `NotFound` on Windows (catch-site line) but is a strict-modes refusal on Unix, so such a test needs a per-OS oracle; inheritable grants reach through `seed_conpty`'s hard links. Why it matters: a direct trap for tests the chunk ungates or adds.

@history_item n=56 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (diagnostics opens; replay readers; home-tree check) — Role and detail opens check `diagnostics/` (read rule on Unix files, write rule on Windows files); `run`'s collision check over a replayed snapshot decides from the heartbeat alone; the start's home-tree check refuses before any log. Why it matters: per-OS open rules and the liveness reading the `v1-40` witness and kill-and-revive case meet.

@history_item n=57 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (no-clobber placement; runner ACLs differ by host) — An exec of a pinned copy still open for write failed `ETXTBSY` on Linux (4 of 24 boots), fixed by `place_private_new`; `windows-2025` runners of one image differ in inherited temp-dir ACLs, so an oracle reads a grant by its right, not one SID. Why it matters: a Unix-only race shape and a runner-dependent oracle trap for parity tests.

@history_item n=58 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers; [PTY] OPEN) — Every product reader of external bytes is bounded in its statement or an `EXCEPTIONS` row of `tests/contract_bounded_readers.rs`; `tests/chaos_feed_panic.rs` asserts the panicking bytes on Unix and only the literal absence on Windows, the Windows reading owed to "Windows-only live measurements". Why it matters: any reader the openpty path adds is censused, and that test's per-OS split keeps its reason.

@history_item n=59 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the pattern; `channel-response` detail; `v1-47`) — Error bodies carry codes and fixed messages only, held by `tests/contract_error_bodies.rs`; `channel-response`'s `detail` closed to three words in `schemas/diag-line.v1.json`; `v1-47`'s reading is after each job's end-to-end run with `AGENT_RUN_KEEP_HOMES` `"1"` and `if: always()`, the chaos home out. Why it matters: the closed words and the kept-homes rule the Unix jobs must hold.

@history_item n=60 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — `UnreachableCause` became exit 21's closed cause set written by `client::unreachable_document`; `stale-heartbeat` is a stale holder's own exit-1 code; `revive --json` documents every start refusal; `server-verify-failed` was claimed off macOS only, with `v1-18`'s carve-out; `process-exit` `detail` enum gained six words. Why it matters: the closed document words the one-schema rule holds identical on every OS.

@history_item n=61 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — The Epoch 6 delegate marks cleared by the founder's word: `v1-47`'s reading plain, [PTY]'s Windows reading after a feed panic staying OPEN with its owner. Why it matters: the standing of the kept-homes rule and the chaos-test split the chunk relies on.

@history_item n=62 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused`; the start's and the clients' refusals) — `socket-dir-refused` became exit 21's sixth cause and `revive --json`'s eleventh code; `run` / `revive` refuse through `socket_dir_ready` (exit 1) and every client runs `check_endpoint_dir` before its connect (CLI exit 21, hook fail-open `hook-decision`, MCP `isError`). Why it matters: the Unix-only refusal whose closed word must read the same on Linux and macOS.

@history_item n=63 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the peer euid on both sides; macOS's server check) — `peer_euid_admitted` is `!reported || observed == Some(own)`, `PEER_EUID_REPORTED` `cfg!(unix)`, injected only through the private `serve_admitting` with no env, flag or feature seam; on macOS `server-verify-failed` means an euid mismatch alone, the no-peer-pid gap owned by no route entry. Why it matters: the scope's invariant that no seam forces a refusing euid decision, and the macOS carve-out it keeps.

@history_item n=64 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`check_socket_dir`; the human pair) — `check_socket_dir` takes one `symlink_metadata` and refuses a symlink, non-directory, foreign owner or group/other bit, never chmodding; no check sits inside `Client::connect`; the socket-dir `unable:` / `hint:` pair names only the `ViolaName`, its wording owed to the founder at Epoch 7's boundary. Why it matters: the check the parity suite reads on both Unix runners, and a sanitised surface it must not widen.

@history_item n=65 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a force-killed wrapper's socket file) — A force-killed wrapper's socket file stays until the next bind of that name takes it over; harness `endpoint_gone` and `holder_gone` read a refused or `NotFound` connect as gone. Why it matters: the Unix kill-and-revive case must let the revive's bind take the name over and never assert the file removed.

@history_item n=66 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list`'s one home rule) — `board::scan` holds the home to the read rule on every OS, so a 0750 home refuses the whole board (`ScanRefused::Home`). Why it matters: the `v1-40` witness's test home must stand at the read rule.

@history_item n=67 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the test-owned socket roots) — `tests/support/home.rs` `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on` give a test its own socket root, `SOCKET_ROOT_VAR` `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere. Why it matters: the per-OS root the Unix socket-dir tests take on both runners.
