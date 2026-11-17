@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (test-only crate, bins, env vars, paths) — registered the test-only crate `viola-e2e` (harness library + `viola-harness`, no-op `fake-agent` feature) and the `AGENT_RUN_` prefix for harness-only variables; the rig's candidate home (a test-only binary of `viola-e2e`) and any knob it reads fall under these registrations (decision/e2e-noop-fake-agent-feature, decision/harness-env-var-prefix, decision/test-binaries-never-released).

@history_item n=2 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** — registered the test homes (`viola-session-*/home`, `viola-test-*/home`) and test-side env vars, none read by `viola`; precedent for registering the rig's own scratch homes and keep-variables as test-only resources.

@history_item n=3 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — `FAKE_AGENT_PUMP_DELAY_MS` ratified as the one carve-out to the `VIOLA_`/`AGENT_RUN_` rule, read only under `fake-agent`, capped, configuring nothing; any new seam the rig needs on the fake agent (DEL or modal fidelity) is a boundary widening needing the founder's ratification (decision/fake-agent-pump-delay-seam, decision/test-seams-not-configuration).

@history_item n=4 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam) — `FAKE_AGENT_HOOK_PANIC` added as the second ratified seam, founder-ratified live; the seam list is closed at two (decision/fake-agent-test-seams), so a rig-driven fake-agent fidelity change should not add a third env seam.

@history_item n=5 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (ledger rows, verify's probe, stamps; hidden `hook --capture` arm) — established `viola verify` as the one stamps writer, the real CLI run only locally, `--record` writing scrubbed fixtures only at 0 fail, and the inert `hook --capture` arm; the rig's readings (whitespace trim, wake `source` field) that become rows must go through this ledger/probe path (decision/capability-ledger-gate, decision/verify-record-only-clean, decision/probe-capture-arms-inert, decision/stamps-single-writer).

@history_item n=6 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (viola-pty-watch report) — real-PTY tests stream step evidence to `<temp dir>/viola-pty-watch/` report files kept on panic or kill, removed on pass, codes and counts only; the precedent for the rig's kept evidence (decision/test-evidence-survives-kill, decision/wait-report-kept-on-failure).

@history_item n=7 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — `viola-harness run --local-live` refuses under `CI` before any build or spawn, `target/e2e-home/viola-live-<pid>/home` is the one real-`claude` verify home, homes are stamped by `viola verify`; the rig's live starts must keep this CI refusal and stamped-home model (decision/no-live-claude-in-ci, decision/real-claude-local-only, decision/harness-env-unread-by-product).

@history_item n=8 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — test homes removed through `remove_owned`, the owner record deleted last, so a removal that stops part-way keeps the record for a later sweep; the rig's scratch-root cleanup must follow the owner-record rule (decision/test-home-owner-sweep, decision/test-homes-cleaned-by-owner).

@history_item n=9 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — vt100 `=0.16.2` entered the graph through `viola-agent-claude`'s pure `screen` module, fed on `run`'s feed thread under `catch_unwind`, poisoning on a panic; the rig's choice to reuse vt100 rests on this pin and its poisoning rule (decision/vt100-screen-model, decision/vt100-readiness-gate, decision/poisoned-screen-model).

@history_item n=10 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (fuzz targets, the chaos test home) — the `viola-chaos-*` home outside `target/e2e-home`, outside G2/G4 and the secret scan, ratified as a named test-data carve-out; a rig scratch root outside the repository is a similar placement and may need the same ratification and scan story.

@history_item n=11 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the closed non-editing list) — only a closed list of focus reports, mouse reports and terminal replies never takes the wheel, every other byte does; a rig that drives viola through its own pty and answers terminal queries must write only those exact grammars or it takes the wheel (decision/closed-non-editing-byte-list, decision/byte-source-wheel-claim).

@history_item n=12 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** — the founder ratified the PTY-driven typed-input `viola verify` probe against the live installed `claude`, run locally only, no credential on a CI runner; the boundary the rig's live starts sit inside.

@history_item n=13 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled; registry rows for verify's interactive runs and screen fixtures) — compiled `SIGNATURES` (the trust and external-imports modal literals), Run A untrusted in an OS-temp dir killed with no key, Run B trusted, signature-only `Screen.<phase>.json` fixtures, `Screen::rows()` verify-only; measured that trust is inherited from a trusted parent and the imports approval keyed on the git root — the rig's trust/modal reading and its scratch root outside every repository rest on these (decision/screen-signatures-compiled, decision/screen-rows-read-only-by-verify, decision/verify-types-nothing-into-dialogs, decision/no-typing-into-trust-dialogs).

@history_item n=14 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D, `plansDirectory` pointed inside the run's own 0700 dir so nothing lands under `~/.claude/plans`; precedent for keeping every CLI write inside the rig's private scratch (decision/verify-plan-files-in-run-dir, decision/verify-child-set).

@history_item n=15 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's pastes go only into rows holding the input-box literal and no modal; measured that 2.1.287's footer holds no input-box literal for 8.0 s after a long paste; a trap for the rig's screen reads after a paste (decision/verify-box-wait-for-literal).

@history_item n=16 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `target/e2e-home` linked to `/tmp/viola-e2e-home-<uid>` (tmpfs, 0700), the test side creating and deleting outside the working directory, a boundary widening ratified by the founder; standing rule: a stalled-start red is a finding about the backing, never re-run for green — governs where the rig's scratch root lives (decision/test-side-removal-outside-workdir).

@history_item n=17 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (paste frame; cross-session form; identity floor's Linux reading) — earlier live readings came from scratch probes, not verify probes, so their rows stayed owed; the identity floor stays eleven names and hook-process `CLAUDE*` names were read names-only; the rig's own readings and its isolated child environment follow the same names-only and owed-row rules (decision/r8-identity-floor, decision/identity-floor-eleven-names, decision/env-strip-names-only).

@history_item n=18 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the gate waits up to 8.5 s for the input box; trap: any case reading a verified literal-less screen through a clock that never advances waits forever — matters for a rig screen model replaying screens (decision/gate-timings-compiled, decision/send-waits-out-paste-hint).

@history_item n=19 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (seven terminal reply shapes; the live readings on 2.1.287) — seven reply shapes added after a live CLI took the wheel 237 ms after start; standing rule the list grows only by measured shapes with negative controls; also the first live send readings — the rig's terminal must answer the CLI's queries and a reading it makes may add a shape only by measurement.

@history_item n=20 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text drops trailing CR and LF; empty-text) — typed text drops trailing CR/LF; endings read live before tests pinned them; context for the carried whitespace-trim reading (decision/typed-text-normalisation, decision/send-text-line-endings).

@history_item n=21 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — inner CR/CRLF typed as one LF, measured live on 2.1.287 with listed unmeasured shapes; the rig's whitespace reading on the input box extends this measured set.

@history_item n=22 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's dirs removed on pass, fail or a SIGINT/SIGTERM stop, a stopped run killing and reaping its child with no key, guards removing only what that run created; SIGKILL and a crash leave dirs — precedent and limit for the rig's "ends what it starts on every exit path" hygiene.

@history_item n=23 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode) — the founder chose files beside the fake agent's exe over a third env seam; precedent for how fake-agent fidelity (DEL, the trust dialog's rows) carried on this entry is configured.

@history_item n=24 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (plugin install as built) — a live round took none of its readings because both wrapped starts in a fresh git repo were held by the folder-trust dialog; the trap this rig's trust-dialog reading exists to close.

@history_item n=25 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe leaves an empty CLI project dir) — verify's runs leave transcripts and empty project dirs under `~/.claude/projects/`, an accepted residual by the founder's word; the rig's "no write under the real `~/.claude` left unexplained" must account for this class (decision/verify-transcripts-accepted-residual).

@history_item n=26 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — 2.1.302 hides `for agents` after 15 s, so the second literal `(shift+tab to cycle)` was added after one scratch footer reading; the fake agent's `--long-turn-screen` reproduces a stop without a live start — screen literals drift by CLI version, which the rig's screen reads must survive.

@history_item n=27 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the e2e-home keepers refuse a linked base below a 9 GiB headroom floor; a rig scratch root on the same tmpfs inherits this quota constraint.

@history_item n=28 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the tap, the `Mcp.` class; the test-only child marker) — `viola-mcp-tap` landed as a `[[bin]]` of `viola-e2e` recording a live CLI exchange through scrub and screen-fault refusals, and the test-only `MCP_SERVE_TEST_CHILD` marker kept outside `VIOLA_*`; the closest precedent for a standing rig binary in `viola-e2e` (decision/viola-env-namespace-product-only).

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (home-tree check before the start's and verify's log) — `run` and `verify` refuse a widened home tree before any log; a rig-made home must be created owner-only or every start refuses (decision/verify-refuses-untrusted-home-or-stamps, contract/run-start-refusals/home-tree).

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a force-killed wrapper's socket file; the test-owned socket roots) — a force-killed wrapper leaves its socket file, harness cleanup reads a refused or absent connect as gone, test socket roots are test-owned; the rig's kill path and process census meet these (decision/cleanup-reads-refused-or-missing-socket-gone).

@history_item n=31 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated tests census; the shared Frozen guard) — every OS-gated test names its reason in `tests/contract_os_gated_tests.rs`, and `tests/support/frozen.rs` is the one freeze guard; the rig's tests and any process control it shares land under these.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (test support and harness; verify's stop; the root `WITHIN` waits) — Linux only, harness behind `scripts/agent-run.sh` alone, verify's stop `SIGINT`/`SIGTERM` alone, `Frozen` ungated; the rig is a Linux-only test-side artifact (decision/linux-only).

@history_item n=33 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — readiness reads the input box by its own rule and prompt rows beside the footer literals; recorded screen fixtures keep box rows as rule/prompt chars alone and refuse a prompt row holding a home path, username or email — the rig's kept screens must meet the same scrub (decision/screen-signatures-compiled, decision/screen-fixtures-signature-rows-only, decision/verify-box-wait-for-literal).

@history_item n=34 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Occupied Resources: 2 rows) — `viola-hypr-tap` registered as a test-only binary of `viola-e2e`; a new rig binary is an occupied name the registry must hold the same way.

@history_item n=35 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — viola no longer touches any status line, no `--settings` at start, `--settings` read by verify alone; the rig must leave the founder's status line and global config untouched and nothing of them may reach a record (absence/status-line-interception, decision/global-statusline-untouched, decision/claude-flag-literals-in-agent-crate).

@history_item n=36 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — a key typed into the folder-trust dialog before session start takes the wheel; at the dialog hold's end the CLI's dialog stays drawn — both read live, both shape how the rig reads and answers the trust dialog (decision/wheel-starts-with-driver, decision/dialog-hold-bound).

@history_item n=37 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — the shared root wait bound `WITHIN` is 12 s and test channel sockets bind under a short `/tmp` test root, never `TMPDIR`; the scope's directive that a test takes a bound and a socket under the short root rests here (decision/root-waits-share-one-bound, decision/test-sockets-under-short-tmp-root).

@history_item n=38 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — the delivery match accepts the typed text less its trailing ASCII-whitespace run, the trim set an unread assumption; the post-gate section runs under one flight lock, closing the race only for a turn the wrapper has learned of — the two carried readings (trim set, hook-latency window) measure these (decision/submitted-form-prompt-match, decision/send-flight-lock, contract/hook-event/driver-relabel).

@history_item n=39 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance, a leading trim waiting for the live reading carried on «Standing live rig with a screen model»; StopFailure ratified with no ledger row, its live reading owned by «The 2.1.325 recorded fixture set» — this chunk owns the leading-trim reading.

@history_item n=40 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `wait` wakes by kind alone, a harness-origin prompt-submitted never wakes; the background for reading whether a fired scheduled wake's prompt is harness-classified (contract/channel-methods/wait-wake-kinds, contract/channel-wait/wake-kinds).

@history_item n=41 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** — `turn-ended` gained `scheduled_wakes` from the Stop payload's `session_crons` length, claimed only on measured `0` and synthetic counts; the rig's carried reading «a real wake reads 1» verifies this row (contract/event-data/turn-ended-scheduled-wakes).

@history_item n=42 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the scheduled-wake-count rows; their live reading (a wake reading 1, the fired wake's `source` field) still rides this chunk, and its result decides whether «A fired scheduled wake keeps the wheel» stands (decision/harness-prompt-prefixes, decision/harness-prompts-not-human).

@history_item n=43 marker=2026-11-16-survey-key-keeps-the-wheel
**2026-11-16-survey-key-keeps-the-wheel** — the founder ruled the survey exemption out: a key answering the CLI's session-feedback survey takes the wheel; the rig reading CLI modals must treat the survey as a modal whose key moves the wheel (decision/closed-non-editing-byte-list).
