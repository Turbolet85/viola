@history_item n=1 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (send and human split by phase) split `src/human.rs` and `src/run/send.rs` into module roots over phase files, the roots re-exporting with `pub(crate) use` so every caller kept its path, each file under 800 code lines, no output, refusal, span or event moved; it is the direct precedent for this chunk's three pure moves, and the master rows that named the old paths (build-system's human writer, the [Delivery Confirmation] citations) were re-pointed at that wrap.

@history_item n=2 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (the split modules) split `src/cmd/verify/typed.rs` and `src/human/board.rs` into module roots over child files, with a `#[cfg(test)]` `test_support.rs` and the ledger's unit cases moved out via `#[cfg(test)] #[path = "ledger/test_cases.rs"] mod tests`; this is the pattern for moving `pump.rs`'s and `run.rs`'s inline test modules into their own files, and it shows the decision rows that cite a moved symbol path (`typed::turns::agents_read`) being re-pointed.

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — made `cargo modules orphans --deny` per lib/bin target (`scripts/orphans-check.sh`, with `--probe`) a CI gate, the source of `decision/no-orphan-modules`; every module the three moves create in `src/cmd/`, `src/run/` and `crates/viola-pty/src/` has to be declared in its `mod` tree or the gate goes red.

@history_item n=4 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (bounded readers) added `tests/contract_bounded_readers.rs`, the source of `decision/bounded-readers`, whose `BOUNDARIES` rows name each bounded read's file and witness (checked by `boundaries_each_name_a_bounded_read_that_stands`), and whose census matches `EXCEPTIONS` needles against the whole source line; a read moved out of `pump.rs` or `run.rs` must have its row re-pointed (the re-point named in the plan), and splitting a statement across lines can break the in-statement `take` reading.

@history_item n=5 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the sanitised error surfaces pattern) added `tests/contract_error_bodies.rs`, whose `CLI_DOCUMENTS` / `MCP_RESULTS` / `CHANNEL_FAULTS` tables name each error body's writer and witness, with the `product_error_writers_are_each_named` census and `thiserror_messages_interpolate_nothing`, which back `decision/sanitised-error-surfaces` and `decision/thiserror-messages-interpolate-nothing`; refusal writers moved out of `run.rs` (the trust and window steps) or `up.rs` must keep their table rows and `PLUMBING` lines pointing at the file that now holds them.

@history_item n=6 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the OS-gated tests census) added `tests/contract_os_gated_tests.rs`, whose `GATES` table names each gated test or file `{ file, test, gate, reason }` and which fails both ways (an unnamed gate, or a row whose gate no longer stands); a moved test module that carries a `#[cfg(...)]` gate needs its `GATES` row's `file` re-pointed.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — ([PTY]: openpty alone; the OS-gate census faults another OS; CI on one `ubuntu-latest` leg) left `viola-pty` with libc termios, `std::io::stdin()`, the pump keeping the input writer until the child exits, and child exit on the handle wait; it also made a gate naming `windows` or `target_os = "macos"` a census fault even when named, and set CI to `ubuntu-latest` with coverage floors 85/95/80; the restart module split off `pump.rs` lands in this Linux-only crate, and moved tests may not reintroduce another-OS gates.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (portable-pty pinned at =0.9.0; one-enum divergences ratified) moved `viola-pty` to portable-pty `=0.9.0` behind the unchanged `pty` seam (the source of `decision/portable-pty-behind-pty-seam`), and ratified `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused` as named exceptions beside the hand-written `PtyError`, with a census of 9 `thiserror::Error` derives outside `viola-e2e`; the pump split must keep `viola-pty` thiserror-free and agent-free, and must add no derive, or the census count moves.

@history_item n=9 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — set the per-crate sole-root tokio ban (`deny-sync.toml` over `scripts/sync-crates.txt`), which `viola-pty` is listed in; the new restart module stays on std threads and blocking I/O (`decision/tokio-containment`).

@history_item n=10 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — (Established Decisions: 5 row(s)) added `decision/switch-in-place`, under which the wrapper ends only the child its own PTY handle holds, reaps it within `RESPAWN_REAP_BOUND`, appends `launch` and resumes the same session id; it also added `decision/switch-writes-no-user-settings` and the `send-never-queued` turn-running reading while a switch stands. This is the respawn code that moves out of `pump.rs` (the audit's M3 and `Pumping::run`'s cognitive 17), and its order of reap, then `launch`, then respawn must survive the move unchanged.

@history_item n=11 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — (Standard Contracts: 4 row(s), 2 row(s); Occupied Resources: 3 row(s)) set the respawn's outward promises: the old child's late `session-end` of the switched id is dropped once (`contract/hook-event/appended-line`); `launch` is appended once per switch after the reap and before the respawn, and `window` is re-appended after it; `child_pid` is rewritten for the respawned child; and the respawn's argv carries `--resume` with the switched id (`contract/child-argv/resume`, `contract/claude-child-argv/resume`). A pure move of the restart code must keep every one of these byte-for-byte.

@history_item n=12 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — (Established Decisions: 5 row(s); Standard Contracts: 1 row(s); the ratified entries of switching, viola-up and placing) set `decision/switch-in-place`, `decision/up-parents-first` and `decision/up-writes-no-state` standing on the operator's own word and ratified the delegate-word rows of the switch, `viola up` and `viola open` chunks; the decisions this chunk's moves rest on are now standing, so the moves may change no wording or behaviour of them.

@history_item n=13 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — (Established Decisions, Conventions, Standard Contracts, Occupied Resources) landed `viola up` with `decision/up-selection-rule`, `decision/up-parents-first` (parents before children, a cycle taken once), `decision/up-writes-no-state`, `contract/cli-verbs/up`, exit codes 0/1/21, the `boot-time-unread` and `not-live` / `terminal-not-found` details, `cli-<VIOLA_NAME>.ndjson` as its log, `up` as the parent's only lineage reader, and `revive --origin`; splitting `upped` into a plan step (refusal, order, skip) and an act step (spawn, wait, line) must keep each rung, the order and the exit mapping as they are.

@history_item n=14 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — (Established Decisions: 2 row(s)) added `decision/lineage-label-only`, held by the source census `lineage_is_read_by_no_wheel_link_send_or_answer_path`, and `viola-up` later named `up` as the one reader allowed to order by parent; the census reads source files, so moving `upped`'s ordering code to a new file must keep that file inside the census's allowed reader.

@history_item n=15 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — (Standard Contracts: 7 row(s); Occupied Resources: 5 row(s)) added the wrapper's `window` line (appended after `launch`, only under `--origin`), the snapshot's `window` record and its replay rule, and the hidden `run --origin` / `--place` flags; these are the window step whose `open_window` call leaves `src/cmd/run.rs` for a home beside `src/run/window.rs`, and the step's place after `launch` in the start must stay.

@history_item n=16 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** — (Standard Contracts: 1 row(s), twice) widened `decision/hyprctl-direct-spawn` so a dispatch is one Lua element, `hl.dsp.focus` or `hl.dsp.window.move`, on a closed address or an i32 workspace id; the window step only moves, and the scope bars any change to the window code's logic.

@history_item n=17 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Infrastructure Patterns: 1 row(s); Standard Contracts: 6 row(s)) made `viola-hyprland` the one compositor seam (sync, tokio-free, depending on `viola-core` alone) with its closed error set `unsupported` · `unreachable` · `oversize` · `malformed` · `refused`, and dropped «depends on all members» from `decision/root-bin-consumes-normalised-events`; the moved window step stays a caller of the seam's closed request set.

@history_item n=18 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (Established Decisions, Standard Contracts, Occupied Resources, Cross-cutting Patterns, Conventions) gave the run and revive start its trust step: on a verified CLI it writes an untrusted spawn folder's trust record. The record is keyed by the spawn directory (`decision/spawn-directory-confined-to-snapshot`; `contract/instance-snapshot/cwd`, revive using the recorded `cwd`); `key-present` / `removed` read as `lost` to a start, and the start reads a kept `CLAUDE_CONFIG_DIR`'s value. This is the trust step that moves out of `run.rs`, and it must keep its place, its word mapping and its env read.

@history_item n=19 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (Established Decisions: 2 row(s), 1 row(s); Standard Contracts: 1 row(s)) put `decision/start-verifies-new-cli-version` into the start and rewrote `decision/run-start-order` to: gate the version, bind the endpoint, run any automatic verify, and on a verified verdict write the trust record, before the first snapshot and the spawn. `cli_verified` became the start's final verdict. Moving the trust step out of `run::start` must keep it after the automatic verify, on the final verdict.

@history_item n=20 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (Standard Contracts: 9 row(s), 7 row(s)) added the trust writer's closed code words (`written`, `removed`, `config-absent` … `lock-held`, `lost`), which reach the stamps and which the start's trust step reports; the moved step must keep the same writer and the same words.

@history_item n=21 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the start's refused socket directory, exit 1) added `socket_dir_ready` in `src/cmd/run.rs`, which runs before any snapshot, child or `Server::bind` (exit 1, or the `socket-dir-refused` document under `revive --json`); the start-order rows (`contract/viola-run-start-exit-codes/1`) cite this order, and the split of `run.rs` must leave it where it runs.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree check before the start's and verify's log) placed `checked_exe` / `refuse_tree` (`check_home_tree`) before the wrapper log opens at every start, a refusal writing no line. This is the source of `decision/strict-check-before-log-open`, `decision/strict-modes-check-before-log` and `decision/refused-check-writes-no-line`; no code moved out of `run.rs` may come to open a log or read under the home ahead of that check.

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`viola revive --json` documents every refusal of its start) added `Launch`'s `json: bool` and `refused_with` printing `refused_document` for the start's refusals before `spawn_child`, with an anyhow error of the start printing the `internal-error` document. No document is printed after the child held the terminal (`contract/viola-revive-json/error-document`, `decision/no-document-after-child-held-terminal`). The steps moved out of `run.rs` sit inside this refusal path.

@history_item n=24 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — registered the `fake-agent` test seam `FAKE_AGENT_PUMP_DELAY_MS`, read only under the feature in `hold_pump_start` in `src/cmd/run.rs` (`decision/fake-agent-test-seams`); the split of `run.rs` must keep the seam feature-gated, and any master text citing its file must be re-pointed if it moves.

@history_item n=25 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — (Established Decisions: 2 row(s); Occupied Resources) added `decision/launch-pair-handed-not-verified` and `contract/child-argv/launch-pair` (`--model` and `--effort` after `--plugin-dir`, before the caller's words), with the flag literals kept in the agent crate (`decision/claude-flag-literals-in-agent-crate`); the respawn moved out of `pump.rs` builds this argv, so its literals stay in `viola-agent-claude` and `viola-pty` knows no agent.

@history_item n=26 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (the replay's first product reader; revive as the second way a wrapper starts) made a passing revive go through the same start as `run` (`decision/revive-in-place`, `decision/revive-resumes-in-place`), with the child's cwd taken from the snapshot (`decision/child-cwd-explicit`'s except); `run.rs` is the shared start of `run`, `revive` and `adopt` (and through them `open` and `up`), so the boot path the scope says the harness smoke trio covers is this one.

@history_item n=27 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — added the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and the tracing-level `disallowed-macros`, held by `tests/contract_lints.rs` and `scripts/lint-probes.sh` (`decision/no-print-macros`, `decision/obs-event-only-emitter`, `decision/clippy-print-and-macro-bans`); moved code keeps writing through `obs_event!` and the `human` module (`decision/single-human-writer`), and any lint list naming a moved file needs re-pointing.

@history_item n=28 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — moved mutation testing out of CI into the harness `run --mutants`, kept for the epoch-boundary audit (`decision/mutation-at-epoch-boundary`); this chunk plans no mutation run, but a move changes the sites the next audit's diff mutants read (the root crate's 326 caught · 0 missed at `328c22d8`), so every moved function must keep its tests with it.
