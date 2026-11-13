@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — verify's agent-readable outcome set: exit 0/1, one stdout step line per row and the last line `stamped <version>  N pass  N fail`, refusals as the stderr `unable:` + `hint:` pair; the hidden `hook --capture` arm uninstrumented by design (§10 exemption 5). This is the stdout/stderr contract (contract/verify-stdout/*, decision/capture-arms-uninstrumented) that the proof run's "every row pass" reading rests on; a readiness fix must not change it.

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — verify logs its spawns at the call site as `process-start`/`process-exit` pairs (`verify-probe` added to the closed `subject` enum by D-35), each exit with `child_exit_status` + `duration_ms`; `run_bounded` stays unlogged; CI verify runs only against the fake agent, the real-`claude` verify only locally and refused under `CI`. Precedent for decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the live proof run is local, CI replays the fake agent's screens.

@history_item n=3 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed thread (`src/run/gate.rs`) catches a panic per feed/resize, poisons the model and writes one `parse-rejected{parser:"vt100-feed", detail:"panicked"}` at WARN; `send-refused{detail:"input-not-ready"}` follows from the gate's verdict. Bears on the scope's Out boundary (the run pump's readiness gate): if research finds it shares the footer literal, this is the obs shape of that gate (decision/vt100-panics-contained, decision/deterministic-emission's readiness-gate exception).

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gains `strict-modes-failed` (run's version gate reading the stamps through `read_stamps_strict`, read as unverified, `cli_verified:false`); CI `viola verify` moves to the recorded 2.1.287. Origin of contract/run-log-parse-rejected/ledger-stamps and decision/unreadable-ledger-is-unverified; precedent that a new recorded CLI version for the fake-agent replay is stated in the master.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY runs (Run A, Run B) each log one `verify-pty-probe` pair (new `subject` value); no line holds row text, a prompt or a path; CI drives the fake agent with `--screens --turn-stop --trusted-root`. Run B is the typed run this chunk changes; its spawn pair and the fake agent's `--screens` replay are what the readiness change runs inside (contract/verify-spawn-subject/verify-pty-probe, decision/verify-lines-carry-no-content).

@history_item n=6 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — six verify spawn pairs (Runs A–D), CI adds `--dialogs`; the capture arm's `--answers` decision body. Step in the count history of verify's pairs; a readiness change adds no spawn and must keep the count.

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify adds `--framing` (boot step 4 and the stamped-home fixture stamp the rows through the framing replay); verify still logs six pairs and no row text, prompt or path. Directly the framing pastes (`framing_turns`) whose readiness this chunk re-keys, and the replay the new footer-notice fixtures extend; the flag list is bound to test-plan §3 boot step 4.

@history_item n=8 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` includes the readiness gate's wait, up to `GATE_MAX_WAIT` (8.5 s on a delivered send, founder's ruling). Relevant only if research widens the change to `viola run`'s gate (the scope's Out clause): the gate's wait is already observed this way, no new field.

@history_item n=9 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — a live Linux dev-host reading on `claude` 2.1.287 stands beside the Windows origin of the R8 identity floor and moves nothing in the logged fields (`env_stripped_count`, `env_stripped_known`, `env_kept`, names only). Precedent for a live-CLI reading on the dev host recorded without changing the strip's names-only rows (decision/r8-strip-logged-as-names-only, contract/claude-child-process-start/env-*), which the verify spawns still carry.

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `process-exit` detail `interrupted` for a verify stopped by SIGINT/SIGTERM: no further spawn, every opened pair closed, stderr stop pair, empty stdout, no stamp; the probe dir removed on pass, fail and stop. Origin of contract/verify-process-exit/interrupted and contract/verify-stop-stderr/unable-hint-pair; a new wait for the input box in Run B must stay stoppable under this rule.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout is one `[NN/MM]` step line per relied row, `MM` the relied count, the summary summing to it; Run B's `verify-pty-probe` pair carries one probe `--settings`, no line carries the probe command, a payload or a path; the statusline capture arm uninstrumented. Origin of contract/verify-stdout/step-line, summary-line and decision/no-statusline-settings-in-telemetry; the proof's "every row pass but `statusline-rate-limits`" is read on these lines.

@history_item n=12 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — seven verify spawns (Run E, the `--resume` of Run B's first session id); a child of an interactive PTY run killed at a stop carries `child_exit_status` and `duration_ms` (measured). Origin of contract/verify-process-exit/child-exit-status-and-duration; Run B's session id feeds Run E, so a Run B readiness change has a downstream run.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify logs eight spawn pairs, the eighth an `agents-probe` pair around Run B's single agents read, made once its first turn has settled and before the framing pastes; no row text, session id, `cwd` or path; `MM` 23/20. Places the agents read inside Run B's sequence exactly between the turn-settle and the pastes this chunk re-keys (contract/verify-spawn-subject/agents-probe, contract/diag-subject/agents-probe).

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the sixth Run B paste (`/compact`, row `local-command-compact`, founder's answer), `MM` 24 on Unix; Run B's six pastes add no spawn pair or subject (eight pairs). The `/compact` paste is one of the pastes the scope names; after it a footer notice is most likely to stand, and it adds no obs line.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check and the catch site's narrowed reach) — verify runs `check_home_tree` (home, `ledger/`, `bin/`, `plugin/`) before its `cli` log opens, a refusal exit 1 with the home pair and no line; `update_stamps` runs `check_stamps` before and under its lock, a refusal mapped to the home pair, exit 1 through verify's own `process-exit`. Origin of contract/viola-verify/home-tree-refused, contract/viola-verify-refusal/* and decision/refused-home-tree-writes-no-line; the proof's stamp write goes through this path.

@history_item n=16 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap; D-05 `mcp-stdin`) — rmcp 3.4.1 reads each stdin line with an unbounded `read_until`, so `viola_mcp::serve` hands it `CappedLines::new(stdin, MAX_FRAME)`; a longer line ends the session with one `parse-rejected{parser:"mcp-stdin", detail:"oversize"}`, no rmcp line (rmcp target OFF). Trap for the supply-chain item: an rmcp bump past `<3.5` must keep `CappedLines` as the only stdin reader and keep contract/mcp-stdin/max-frame, contract/mcp-stdin/oversize-line and decision/rmcp-tracing-target-off true.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`viola mcp`'s exit at an oversize line before initialize; the mcp role writes no self process-exit) — before `initialize` the cut gives `ServeError::Initialize`, process exit 1, both streams empty, one `parse-rejected`; the `mcp` role writes no self `process-exit`. These exits rest on rmcp's `ServeError` and serve loop; an rmcp version bump must be re-read against them (decision/mcp-log-sink, decision/mcp-stdout-frames-only).

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — under a parsed `VIOLA_NAME` verify's `process-exit` carries `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture` or `interrupted`; a refused `update_stamps` reads `strict-modes-failed`; a failing row carries no code; verify takes no `--json`. Origin of contract/process-exit-detail/verify-refusal and contract/process-exit-detail/strict-modes-failed; a readiness timeout in Run B is a failing row, not a new refusal code, unless a Decisions Log entry adds one (decision/event-values-closed).

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — one CI leg (`ubuntu-latest`), `ci.yml` beside `nightly.yml`, `cargo deny` on one target `x86_64-unknown-linux-gnu` (decision/obs-deps-pass-cargo-deny); verify's relied count reads all 24 rows of `ALL`, stops by `SIGINT`/`SIGTERM`; closed words that no build can emit leave `diag-line.v1.json` with their emitter. The supply-chain job this chunk turns green runs on that one target; the relied count bounds the proof's pass reading.

@history_item n=20 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `nightly.yml` beside `ci.yml` runs the weekly `cargo deny check advisories`. The advisories red this chunk closes (`df9af357` nightly, then `supply-chain` on push) is read on both; a reasoned `deny.toml` ignore or the rmcp/rustix bump must hold in both workflows.

@history_item n=21 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — the ci.yml `supply-chain` artifact (`deny.json`, `deny-fuzz.json`, `zizmor.json`) is admitted unscanned by content: cargo-deny/zizmor diagnostics only, no absolute path; a member that could carry a host path or log content is scanned or dropped. Any change to the `supply-chain` job's outputs while fixing it stays under this guard.

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the portable-pty pin at =0.9.0) — a third-party pin move is recorded in the master's entity, row and logging-stack bullet with only the version moved, the target's OFF status and seam capture unchanged. Precedent for how an rmcp version bump is reconciled on the obs side (decision/rmcp-stdio-mcp-server, decision/third-party-targets-off).

@history_item n=23 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (1077 rows), each verified against the code at 8834195b and re-read at 2d378825, fates by the founder. Every picked row of this chunk is from that landing; a disagreement found while planning is a row amendment, not a text edit.
