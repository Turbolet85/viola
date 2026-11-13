@word proposal=2026-11-12T03-23-49 of=0922fbab47d987b4ab01de0ef016e67560c5ba4a195010dc7b3dabd5b8abea8f
by: the founder, by dialog 2026-11-12 ≈06:18Z (the lists file, rows-2026-11-12T03-23-49-lists.md, md5 02e8f838935ee291cec8a44cf02826dd) and ≈08:20Z (the holds file, rows-2026-11-12T03-23-49-holds.md, md5 c268c6e2b7aac66684d02fcab5d64035), on viola-overseer's read against the code at 03fb886f (word-reads/READS.md, md5 fd427122b7fa5d8d38cc47a6586f6778)

@fate n=1 list=gone master=design-system row=decision/event-tape-played-in-order v=drop
@fate n=2 list=gone master=design-system row=contract/readback-box/open v=drop
@fate n=3 list=gone master=design-system row=contract/readback-box/rb v=drop
@fate n=4 list=gone master=design-system row=contract/readback-box/unable v=drop
@fate n=5 list=gone master=design-system row=decision/monochrome-palette-ladder v=drop
@fate n=6 list=gone master=design-system row=decision/blue-only-for-transfer-markers v=drop
@fate n=7 list=gone master=design-system row=decision/handoff-blue-lightened v=drop
@fate n=8 list=gone master=design-system row=contract/color-roles/primary v=drop
@fate n=9 list=gone master=design-system row=contract/color-roles/secondary v=drop
@fate n=10 list=gone master=design-system row=contract/borders/subtle v=drop
@fate n=11 list=gone master=design-system row=contract/borders/standard v=drop
@fate n=12 list=gone master=design-system row=contract/borders/emphasis v=drop
@fate n=13 list=gone master=design-system row=contract/borders/focus v=drop
@fate n=14 list=gone master=design-system row=contract/borders/handoff v=drop
@fate n=15 list=gone master=design-system row=contract/colour-palette/primary v=drop
@fate n=16 list=gone master=design-system row=contract/colour-palette/secondary v=drop
@fate n=17 list=gone master=design-system row=contract/colour-palette/handoff v=drop
@fate n=18 list=gone master=design-system row=contract/border-intensity/subtle v=drop
@fate n=19 list=gone master=design-system row=contract/border-intensity/standard v=drop
@fate n=20 list=gone master=design-system row=contract/border-intensity/emphasis v=drop
@fate n=21 list=gone master=design-system row=contract/border-intensity/focus v=drop
@fate n=22 list=gone master=design-system row=contract/border-intensity/handoff v=drop
@fate n=23 list=gone master=security-plan row=decision/tail-only-valid-instance-logs v=drop
@fate n=24 list=gone master=security-plan row=decision/notify-stays-on-8-2 v=drop
@fate n=25 list=gone master=security-plan row=decision/zipsign-key-ci-secret-only v=drop
@fate n=26 list=gone master=security-plan row=contract/config-json/port v=drop
@fate n=27 list=gone master=security-plan row=decision/tailed-paths-valid-instance-logs v=drop
@fate n=28 list=gone master=architecture row=decision/terminal-fleet-console v=keep
because: Viola 0.2.0's route plans the console in its Epoch 6 (Console scaffold · Fleet console view · Console actions): a decision of the version in flight, not built yet, so its absence from the code is expected.
@fate n=29 list=gone master=architecture row=decision/hook-tier-map-verified v=drop
@fate n=30 list=gone master=architecture row=decision/snapshots-check-mode-only v=stale
cell: rule
value: The agent crate's decision-body snapshots are only checked by tests, never written by them; the root integration tests take no snapshots.
at: crates/viola-agent-claude/src/dialog.rs:687
@fate n=31 list=gone master=test-plan row=decision/one-test-per-recorded-hook-payload v=stale as=decision/recorded-hook-payloads-walked-at-run-time
cell: name
value: Recorded hook payloads walked by one test
cell: rule
value: One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.
at: tests/contract_ledger_probes.rs:4
at: tests/contract_fixture_hygiene.rs:5
@fate n=32 list=gone master=test-plan row=decision/mcp-paused-tokio-clock v=drop
@fate n=33 list=gone master=test-plan row=decision/host-unreachable-mutants-owed v=stale as=decision/host-unreachable-mutants-excluded
cell: name
value: Host-unreachable mutants recorded as host-excluded
cell: rule
value: A missed mutant the host never compiled is recorded host-excluded: kept out of the survivors and never counted as caught.
at: crates/viola-e2e/src/harness/run/mutants.rs:46
@fate n=34 list=gone master=test-plan row=decision/mock-instant-behind-clock-seam v=stale as=decision/test-clocks-behind-clock-seam
cell: name
value: Test clocks behind the clock seam
cell: rule
value: Time in the sync crates is faked through the core Clock seam with hand-written test clocks; no mocking crate stands behind it.
at: crates/viola-core/src/lib.rs:31
at: src/run/send/test_support.rs:24
@fate n=35 list=gone master=obs-plan row=decision/liveness-logged-on-change v=drop
@fate n=36 list=gone master=obs-plan row=contract/detail-line/corr v=drop
@fate n=37 list=gone master=obs-plan row=contract/detail-line/conn v=drop
@fate n=38 list=gone master=obs-plan row=decision/exit-code-panic-evidence v=drop
@fate n=39 list=gone master=obs-plan row=contract/parse-rejected-detail/control-character v=stale
cell: promise
value: Listed in the diagnostics schema for the paste-text parser, but never emitted: a control character in a paste is refused as send-refused not-delivered.
at: schemas/diag-line.v1.json:86
at: src/run/send.rs:337
@fate n=40 list=gone master=obs-plan row=contract/derived-metrics/viola-hook-duration-ms v=drop
@fate n=41 list=gone master=obs-plan row=contract/derived-metrics/viola-channel-duration-ms v=drop
@fate n=42 list=gone master=obs-plan row=contract/derived-metrics/viola-send-confirm-ms v=drop
@fate n=43 list=gone master=obs-plan row=contract/derived-metrics/viola-send-outcome v=drop
@fate n=44 list=gone master=obs-plan row=contract/derived-metrics/viola-wait-outcome v=drop
@fate n=45 list=gone master=obs-plan row=contract/derived-metrics/viola-state-recovered v=drop
@fate n=46 list=gone master=obs-plan row=contract/derived-metrics/viola-parse-skipped v=drop
@fate n=47 list=gone master=obs-plan row=contract/derived-metrics/viola-parse-rejected v=drop
@fate n=48 list=gone master=obs-plan row=contract/derived-metrics/viola-compat-unknown v=drop
@fate n=49 list=gone master=obs-plan row=contract/derived-metrics/viola-child-exit v=drop
@fate n=50 list=gone master=obs-plan row=contract/derived-metrics/viola-panic v=stale as=decision/panic-lines-stay-zero
cell: name
value: Panic lines stay at zero
cell: rule
value: Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.
at: scripts/g2-zero-panics.sh:29
at: .github/workflows/ci.yml:56
@fate n=51 list=gone master=obs-plan row=contract/span-kinds/client v=keep
because: All 11 client spans the row lists stand in the code (src/cmd/send.rs:231 · src/cmd/wait.rs:88 · crates/viola-channel/src/client.rs:67 · src/cmd/hook/statusline.rs:195 · crates/viola-pty/src/lib.rs:127); the checker searched for an otel.kind field, and the kind is the document's grouping of them.
@fate n=52 list=gone master=obs-plan row=contract/span-kinds/server v=keep
because: All 4 server spans the row lists stand in the code (crates/viola-channel/src/server.rs:230 · crates/viola-mcp/src/tools.rs:248 · src/cmd/hook.rs:163 · src/cmd/hook/statusline.rs:99); the kind is the document's grouping of them.
@fate n=53 list=gone master=obs-plan row=contract/span-kinds/internal v=stale
cell: promise
value: The INTERNAL spans are run.*, state.* and pty.paste_write; a hook's decision is the decision_emitted field, not a span.
at: crates/viola-pty/src/pump.rs:74
at: src/cmd/hook.rs:393
@fate n=54 list=gone master=obs-plan row=decision/derived-metric-naming v=drop
@fate n=55 list=gone master=obs-plan row=contract/derived-metrics/viola-compat-v-rejected v=drop
@fate n=56 list=guards master=test-plan row=decision/acyclic-boundary-review v=absent
@fate n=57 list=guards master=test-plan row=decision/cross-user-at-peer-check-boundary v=stands
guard: peer_euid_admitted_takes_only_the_owning_user
at: crates/viola-channel/src/client.rs:517
@fate n=58 list=guards master=test-plan row=decision/leading-slash-never-a-prompt v=stands
guard: send_leading_slash_argument_is_a_usage_error
at: tests/cli_send.rs:1004
@fate n=59 list=guards master=test-plan row=decision/peer-euid-admission v=stands
guard: serve_drops_a_refused_peer_before_any_frame, peer_euid_admitted_takes_only_the_owning_user
at: crates/viola-channel/src/server.rs:1033
at: crates/viola-channel/src/client.rs:517
@fate n=60 list=guards master=obs-plan row=decision/source-allow-list-logging v=stands
guard: disallowed-macros; raw_event_hits; G1 bare instrument
at: clippy.toml:4
at: scripts/lint-probes.sh:125
at: .github/workflows/ci.yml:294
@fate n=61 list=guards master=obs-plan row=decision/heartbeat-liveness v=absent
@fate n=62 list=guards master=obs-plan row=decision/no-log-crate-bridge v=stands
guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
at: tests/tui_pty_seam.rs:30
@fate n=63 list=guards master=obs-plan row=decision/hook-single-log-open v=stands
guard: SPINE_BOUND
at: tests/hook_fail_open.rs:26
at: tests/hook_fail_open.rs:319
@fate n=64 list=divergences master=layout-templates row=decision/board-vs-strip v=code
cell: rule
value: `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
@fate n=65 list=divergences master=layout-templates row=decision/shared-captions-fixed v=rule
work: The planned fleet console (0.2.0, Epoch 6) reuses the six board captions, their words and slot order (src/human/board/rows.rs:42).
@fate n=66 list=divergences master=design-system row=decision/stable-session-slots v=code
cell: rule
value: Every session keeps a slot in the strip bay stable across state changes: wrapped sessions in name order, then unwrapped ones in name order; nothing is re-sorted by state, and a slot shifts only when a session arrives or leaves.
@fate n=67 list=divergences master=design-system row=decision/two-hues-one-role-each v=none
because: The second hue is the web page's handoff blue, and the web palette leaves the product in 0.2.0; amber keeps its own row.
@fate n=68 list=divergences master=design-system row=decision/list-is-the-board v=code
cell: rule
value: `viola list` is the board, `verify` and `plugin install` act on no strip, and every other verb a person runs acts on one strip.
@fate n=69 list=divergences master=a11y-plan row=decision/fixed-copy-from-design v=code
cell: rule
value: The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
@fate n=70 list=divergences master=a11y-plan row=decision/refusal-ends-with-hint v=rule
work: Every refusal reason prints a hint, unverified-cli and answer's other not-delivered details included (src/cmd/send.rs:85 · src/human/mirror.rs:65-67).
@fate n=71 list=divergences master=a11y-plan row=decision/codes-beside-plain-word v=code
cell: rule
value: A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it.
@fate n=72 list=divergences master=a11y-plan row=decision/no-screen-content-parsing v=code
cell: name
value: No screen-content parsing beyond signatures
cell: rule
value: The rendered `claude` TUI screen is parsed only for compiled input-box and modal signatures, never for other content, and screenshots are never taken as a11y evidence (R7).
@fate n=73 list=divergences master=security-plan row=decision/cli-tool-version-floors v=code as=decision/cli-tool-exact-pins
cell: name
value: Exact pins for external CLI tools
cell: rule
value: External CLI and audit tools are installed at exact pinned versions, in CI and by the ripgrep install script, and CI actions are pinned by commit SHA; no tool is expressed as a minimum version floor.
cell: why
value: CI installs each tool at its pin itself, and no drift check compares a locally installed build against a pin.
@fate n=74 list=divergences master=security-plan row=decision/nutype-input-validation v=code
cell: rule
value: Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers and channel link and unlink params.
@fate n=75 list=divergences master=security-plan row=decision/client-exit-13-first v=code as=decision/client-refusals-first
cell: rule
value: A CLI frame verb's strict-modes refusal comes first, then its own exit-13 refusals, before its liveness and server checks.
@fate n=76 list=divergences master=security-plan row=decision/exec-commands-absolute-pinned-path v=code
cell: name
value: Exec commands use an absolute path
cell: rule
value: Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive and verify, and the installing exe's canonical path in plugin install's `.mcp.json`.
@fate n=77 list=divergences master=security-plan row=decision/fixture-scrub-and-refusal v=rule
work: The email check runs on every kind of committed fixture and at the record, not on screens alone (crates/viola-agent-claude/src/ledger.rs:1327-1334).
@fate n=78 list=divergences master=security-plan row=decision/exact-third-party-pins v=rule
work: The chunk that closes V182 settles rmcp's version (Cargo.toml:223, held to >=3.4.1, <3.5), and its wrap writes the rmcp exception in the form that proves right.
@fate n=79 list=divergences master=security-plan row=decision/cli-names-through-name-check v=rule
work: run parses its name through the shared parse_name rather than its own copy of send's (src/cmd/run.rs:50).
@fate n=80 list=divergences master=security-plan row=decision/budget-json-checked-read v=none
because: The budget gate and its budget.json reading leave the product in 0.2.0 (census row F-08).
@fate n=81 list=divergences master=security-plan row=decision/no-rust-cache-in-release v=holds
because: No release workflow stands (.github/workflows holds only ci.yml and nightly.yml), so none restores rust-cache; ci.yml's release job (.github/workflows/ci.yml:317-338) is a build check that ships nothing, and the row holds as written, vacuously.
@fate n=82 list=divergences master=security-plan row=decision/no-stored-secrets v=code
cell: except
value: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory
@fate n=83 list=divergences master=security-plan row=decision/strict-modes-check v=rule
work: The trusted-state checks refuse a symbolic link too, as the diagnostics and socket-directory checks do: check_instance, check_stamps, check_path and check_read stop following one (crates/viola-state/src/strict.rs:240).
@fate n=84 list=divergences master=architecture row=decision/no-tmux-or-shell-dependency v=rule
work: The status-line shell-out leaves (crates/viola-agent-claude/src/statusline.rs:124): 0.2.0's entry «Status-line interception removed» (census F-09), already on the route.
@fate n=85 list=divergences master=architecture row=decision/tolerant-external-payloads v=code
cell: rule
value: Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only.
@fate n=86 list=divergences master=architecture row=decision/thiserror-per-crate-anyhow-in-bin v=code
cell: rule
value: Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin.
@fate n=87 list=divergences master=architecture row=decision/nutype-errors-fold-into-crate-enum v=code as=decision/nutype-errors-not-folded
cell: name
value: nutype validation errors stay out of crate enums
cell: rule
value: A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
@fate n=88 list=divergences master=architecture row=decision/env-var-viola-prefix v=code
cell: except
value: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only.
@fate n=89 list=divergences master=architecture row=decision/harness-env-var-prefix v=code
cell: name
value: Test-harness knobs carry the AGENT_RUN_ prefix
cell: rule
value: Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
@fate n=90 list=divergences master=architecture row=decision/timestamp-field-names v=code
cell: rule
value: Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
@fate n=91 list=divergences master=architecture row=decision/percent-newtype v=none
because: Percent's only users are the budget-gate thresholds, and the budget gate leaves the product in 0.2.0.
@fate n=92 list=divergences master=architecture row=decision/fake-agent-feature-on-every-member v=code as=decision/fake-agent-feature-on-named-members
cell: rule
value: The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves.
cell: name
value: A fake-agent feature on each named member crate
@fate n=93 list=divergences master=architecture row=decision/no-env-config-channel v=none
because: a duplicate of decision/no-environment-configuration
@fate n=94 list=divergences master=architecture row=decision/no-environment-configuration v=code
cell: except
value: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.
@fate n=95 list=divergences master=architecture row=decision/budget-gate-reads-hook-written-file v=none
because: The budget gate and its budget.json reading leave the product in 0.2.0 (census row F-08).
@fate n=96 list=divergences master=architecture row=decision/thiserror-one-enum-per-crate v=code
cell: rule
value: Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin.
@fate n=97 list=divergences master=test-plan row=decision/workflows-audited-and-sha-pinned v=holds
because: zizmor runs over the workflows and gates on its exit code (.github/workflows/ci.yml:355-388), and all 28 `uses:` lines are pinned to a 40-hex sha; no step of its own asserts the pinning, approved as today.
@fate n=98 list=divergences master=test-plan row=decision/machine-parseable-test-output v=code
cell: rule
value: Every test layer produces machine-parseable output, and harness commands other than logs print one JSON document on stdout.
@fate n=99 list=divergences master=test-plan row=decision/no-test-retries-or-sleeps v=code
cell: rule
value: nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
@fate n=100 list=divergences master=test-plan row=decision/insta-snapshots-for-decision-bodies v=code
cell: rule
value: Decision bodies are pinned by insta snapshots of the body text.
@fate n=101 list=divergences master=test-plan row=decision/pty-exit-from-wait-not-eof v=code
cell: rule
value: viola-pty reports a child's exit from try_wait on the handle, never from reader EOF; consumers are tested with a mockall double of the seven-method seam whose reader never returns EOF while try_wait returns an exit status.
@fate n=102 list=divergences master=test-plan row=decision/wrapper-channel-integration-driver v=code
cell: rule
value: Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly.
@fate n=103 list=divergences master=test-plan row=decision/on-disk-state-tested-on-real-files v=code
cell: rule
value: Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
@fate n=104 list=divergences master=test-plan row=decision/pty-tested-on-real-openpty v=rule
work: The needless key at tests/tui_pty_seam.rs:64 goes; the size receipt follows a resize on its own.
@fate n=105 list=divergences master=test-plan row=decision/child-holds-no-viola-paths v=code as=decision/child-env-names-viola-instance
cell: rule
value: The agent child is given VIOLA_NAME, VIOLA_DIR (its instance directory) and VIOLA_BIN (the pinned copy path).
cell: name
value: Child given its viola name, instance dir and binary
@fate n=106 list=divergences master=test-plan row=decision/proptest-strategies v=code
cell: rule
value: proptest strategies are what generate property-test data.
@fate n=107 list=divergences master=test-plan row=decision/product-state-from-real-runs v=code
cell: except
value: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
@fate n=108 list=divergences master=test-plan row=decision/stamps-written-only-by-verify v=code
cell: except
value: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
@fate n=109 list=divergences master=test-plan row=decision/injected-clock v=code
cell: rule
value: Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes.
cell: except
value: A link line's since is stamped from the wall clock and asserted by its shape only.
@fate n=110 list=divergences master=test-plan row=decision/fixture-hygiene-walk v=rule
work: The email check runs on the hook, agents and statusline fixture walks too, not only on the Mcp and Screen walks (tests/contract_fixture_hygiene.rs:72) — one work with security-plan · decision/fixture-scrub-and-refusal.
@fate n=111 list=divergences master=test-plan row=decision/no-test-owned-real-time v=code
cell: rule
value: Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket.
@fate n=112 list=divergences master=test-plan row=decision/fd-test-reads-live-child-table v=rule
work: The self-dump fallback goes and the test always reads /proc/<pid>/fd, Viola being Linux-only (tests/tui_channel_fds.rs:25).
@fate n=113 list=divergences master=test-plan row=decision/no-fd-count-assertion v=rule
work: The fds.len() <= 5 count assertion goes (tests/tui_channel_fds.rs:34-37); the /proc table assertions stay.
@fate n=114 list=divergences master=test-plan row=decision/property-tests-with-proptest v=code
cell: rule
value: Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline and agents suites.
@fate n=115 list=divergences master=test-plan row=decision/concurrent-append-whole-lines v=code
cell: rule
value: Separate hook processes appending at once to the hook role file and the detail file, including detail lines larger than 4 KiB, must leave every line parsing as one JSON object; every diagnostics file appends through those same two paths, and a torn or interleaved line fails the test.
@fate n=116 list=divergences master=test-plan row=decision/time-faked-through-seams v=code
cell: rule
value: Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument.
@fate n=117 list=divergences master=test-plan row=decision/utc-now-through-clock-seam v=code as=decision/utc-now-read-directly
cell: name
value: Wall clock read directly, not through the clock seam
cell: rule
value: chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision such as the budget gate's takes; the clock seam carries monotonic Instants only.
cell: except
value: The link line's since is stamped directly, so tests assert since by shape only.
@fate n=118 list=divergences master=test-plan row=decision/fake-agent-replays-recorded-bytes v=code
cell: rule
value: The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id.
@fate n=119 list=divergences master=test-plan row=decision/fixtures-walked-for-hygiene v=rule
work: Emails are walked for in every kind of committed fixture, not only MCP exchanges and screens (tests/support/hygiene.rs:10) — one work with security-plan · decision/fixture-scrub-and-refusal.
@fate n=120 list=divergences master=test-plan row=decision/fixtures-recorded-by-live-verify v=code
cell: rule
value: The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap.
@fate n=121 list=divergences master=test-plan row=decision/fuzz-nightly-and-corpus-gate v=code as=decision/fuzz-weekly-and-corpus-gate
cell: rule
value: cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands.
cell: name
value: Fuzzing weekly with a corpus replay gate
@fate n=122 list=divergences master=test-plan row=decision/tests-never-read-host-sessions v=code
cell: rule
value: A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
@fate n=123 list=divergences master=test-plan row=decision/paste-injection-test-layers v=holds
because: All three layers stand: a unit property test (crates/viola-core/src/lib.rs:982), an integration test (tests/channel_paste_validation.rs:47) and an E2E test that sends ESC-bearing text through MCP and the CLI and reads the control-character refusal (crates/viola-e2e/tests/mcp_tools.rs:323-347); the checker missed the E2E layer.
@fate n=124 list=divergences master=test-plan row=decision/hook-unknown-fields-tolerated v=code
cell: rule
value: Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry.
@fate n=125 list=divergences master=test-plan row=decision/one-global-tracing-install v=code
cell: rule
value: The binary installs a global tracing default in exactly one place, its observability init; a unit test installs one only as the one global-subscriber test of its process.
@fate n=126 list=divergences master=test-plan row=decision/diagnostics-checked-before-narrowing v=code
cell: rule
value: A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; a failed role open installs no subscriber and a detail line is dropped.
@fate n=127 list=divergences master=test-plan row=decision/no-nested-harness-in-nextest v=code
cell: name
value: No nested cargo run inside nextest
cell: rule
value: No nextest test shells out to agent-run, a test calls the viola-harness binary only on paths that never build, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
@fate n=128 list=divergences master=test-plan row=decision/members-declare-fake-agent-feature v=code
cell: rule
value: Every workspace member but viola-core declares a fake-agent feature, a no-op where it has none.
@fate n=129 list=divergences master=test-plan row=decision/ci-keeps-test-homes v=code
cell: rule
value: In CI, AGENT_RUN_KEEP_HOMES=1 is set job-wide in the test and perf jobs (only run --mutants overrides it to 0), so every rstest home and every harness session home of those jobs stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run.
@fate n=130 list=divergences master=test-plan row=decision/ci-uploads-only-after-secret-scan v=code
cell: rule
value: On CI failure the home's diagnostics, events and harness artifacts leave CI only in uploads gated on a passed secret scan; the only ungated uploads (the supply-chain reports among them) carry no home content.
@fate n=131 list=divergences master=test-plan row=decision/ci-tools-version-pinned v=code
cell: rule
value: Every tool CI installs is version-pinned; binaries the runner image ships, such as jq and python3, run unpinned.
@fate n=132 list=divergences master=test-plan row=decision/no-secrets-in-logs v=code
cell: rule
value: No log line and nothing in diagnostics/ holds a Cookie header or a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600.
@fate n=133 list=divergences master=test-plan row=decision/integration-binary-surface-prefix v=code
cell: name
value: Root integration binaries prefixed by surface
cell: rule
value: Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix.
@fate n=134 list=divergences master=test-plan row=decision/fixture-chain-exists-twice v=code as=decision/fixture-chain-exists-once
cell: name
value: Fixture chain exists once
cell: rule
value: The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session.
@fate n=135 list=divergences master=obs-plan row=decision/tracing-json-line-file-pipeline v=holds
because: agent-run's status and logs stand (scripts/agent-run.sh:66) and hand to the harness's logs reader (crates/viola-e2e/src/harness/logs.rs:46-51), which reads the JSON-per-line role files under diagnostics/; the checker missed them.
@fate n=136 list=divergences master=obs-plan row=decision/agents-probe-parse-failure-unknown v=code
cell: rule
value: A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified.
@fate n=137 list=divergences master=obs-plan row=decision/hook-logger-init-no-blocking-io v=code
cell: rule
value: Logger initialisation in viola hook reads the diagnostics level, runs the strict-modes checks, creates or narrows the home and diagnostics/ to 0700 and opens the role file once for append at 0600, adding no other I/O.
@fate n=138 list=divergences master=obs-plan row=decision/mcp-log-file-sink v=code
cell: rule
value: The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing, and the log writer is set explicitly.
@fate n=139 list=divergences master=obs-plan row=decision/one-log-file-per-role v=code
cell: rule
value: Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened.
@fate n=140 list=divergences master=obs-plan row=decision/run-threads-inside-catch-unwind v=code
cell: rule
value: The two run PTY pump threads each run inside catch_unwind, and the child's exit is polled inline on run's main thread under main's catch_unwind; a worker panic that leaves main running with no process-exit is forbidden.
@fate n=141 list=divergences master=obs-plan row=decision/payload-types-derive-redact v=rule
work: The hook and MCP payload types derive veil Redact too; today only the channel Request does (crates/viola-channel/src/frame.rs:51).
@fate n=142 list=divergences master=obs-plan row=decision/correlation-propagation v=code
cell: rule
value: Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
@fate n=143 list=divergences master=obs-plan row=decision/deterministic-emission v=code
cell: except
value: The readiness gate reads the vt100 screen before a paste, and its verdict can refuse a send as input-not-ready.
@fate n=144 list=divergences master=obs-plan row=decision/build-fails-on-any-panic v=code
cell: name
value: Build fails on any panic line
cell: rule
value: The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code.
@fate n=145 list=divergences master=obs-plan row=decision/closed-event-and-corr-on-every-line v=code
cell: name
value: Closed event enum on every line, corr where correlated
cell: rule
value: Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
@fate n=146 list=divergences master=obs-plan row=decision/diag-lines-schema-checked-in-unit-tests v=code
cell: name
value: Emitted lines schema-checked in integration and E2E tests
cell: rule
value: Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
@fate n=147 list=divergences master=obs-plan row=decision/run-worker-panic-ends-run v=code
cell: rule
value: A panic in either PTY pump thread is sent to main, which kills the child and takes run's exit path, exit 1 internal-error; the child's exit is polled on main, not on a handle-wait thread; a worker panic leaving main running with no process-exit is forbidden.
@fate n=148 list=divergences master=obs-plan row=decision/hook-lines-required v=code
cell: rule
value: Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither.
@fate n=149 list=divergences master=obs-plan row=decision/mcp-log-sink v=code
cell: rule
value: viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes; a file that cannot be opened exits 1 printing nothing, with no stderr fallback.
@fate n=150 list=divergences master=obs-plan row=decision/hook-fail-open-logged-to-hook-file v=code
cell: rule
value: Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
@fate n=151 list=divergences master=obs-plan row=decision/per-parser-parse-failure-counts v=code
cell: rule
value: Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
@fate n=152 list=divergences master=obs-plan row=decision/content-sink-behind-strict-check v=code
cell: rule
value: The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open as a failed open and the detail line is dropped.
@fate n=153 list=divergences master=obs-plan row=decision/panic-count-home-level-only v=code
cell: rule
value: The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded.
@fate n=154 list=divergences master=obs-plan row=decision/cli-log-only-under-viola-name v=code
cell: rule
value: A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves: for list, plugin install and revive --list under a parsed VIOLA_NAME (with no name no log), for the frame verbs under the named target (link and unlink the driven) instance.
@fate n=155 list=divergences master=obs-plan row=decision/mcp-explicit-log-writer v=code
cell: rule
value: The viola mcp log writer is always explicit: the mcp.ndjson file, with no stderr fallback.
@fate n=156 list=divergences master=obs-plan row=decision/structured-logs-primary-signal v=holds
because: agent-run logs (scripts/agent-run.sh:66) merges the role-file lines, the detail lines and events.ndjson (crates/viola-e2e/src/harness/logs.rs:46-51); the checker missed the merge.
@fate n=157 list=divergences master=obs-plan row=decision/panic-caught-at-every-role v=code
cell: rule
value: A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the two run PTY pump threads and the vt100 feed turn a panic into one record.
@fate n=158 list=divergences master=obs-plan row=decision/instance-source-per-role v=code
cell: rule
value: run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only.
@fate n=159 list=divergences master=obs-plan row=decision/private-diagnostics-files v=rule
work: The diagnostics open refuses a symlink and narrows the mode on the open file, closing the race after the earlier lstat (crates/viola-state/src/fs.rs:44-51).
@fate n=160 list=divergences master=obs-plan row=decision/failed-log-open-discards v=code
cell: rule
value: When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error.
@fate n=161 list=divergences master=obs-plan row=decision/span-carried-into-tasks-and-threads v=code
cell: name
value: Span not carried into tasks and threads
cell: rule
value: Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields.
@fate n=162 list=divergences master=obs-plan row=decision/g1-bare-instrument-gate v=code
cell: rule
value: Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step.
@fate n=163 list=divergences master=obs-plan row=decision/no-sleep-based-log-waits v=holds
because: Every log wait polls a readiness condition to a deadline (crates/viola-e2e/src/harness/boot.rs:477-492); the sleep there is the poll interval, never the wait, and the checker counted it.
@fate n=164 list=stale master=layout-templates row=contract/mcp-tool-results/success v=code
cell: promise
value: Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
@fate n=165 list=stale master=design-system row=contract/flight-progress-strip/wheel-human v=none
because: The flight-progress strip and its weight-600 type belong to the strip-bay web page, which leaves the product. The CLI WHEEL cell prints `human` as plain text.
@fate n=166 list=stale master=design-system row=contract/colour-palette/accent v=code
cell: promise
value: The only attention colour: the `DIALOG` word of a pending dialog in `viola list` human output, as truecolor or its 256- and 16-colour fallbacks; never used for errors, warnings or budget.
@fate n=167 list=stale master=architecture row=overview/project-intent v=code
cell: text
value: viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary and a Claude Code plugin of hooks and a stdio MCP server that calls that binary; a keyboard-driven terminal fleet console is planned, not built. It wraps the unmodified claude CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around core, and no daemon.
@fate n=168 list=stale master=architecture row=contract/mcp-list-results/ttlms v=keep
because: tools_result sets ttlMs 0. Viola's ServerHandler does not override discover, so rmcp 3.4.1's default answers server/discover with a DiscoverResult whose ttl_ms is fixed at 0 and always serialized. The recorded live exchange with viola mcp shows ttlMs 0 on both results. (fixtures/claude/2.1.302/Mcp.tools-list-1.json:70)
@fate n=169 list=stale master=architecture row=contract/mcp-list-results/cachescope v=keep
because: tools_result sets cacheScope Private. rmcp 3.4.1's default discover, which viola's ServerHandler does not override, builds a DiscoverResult with cache_scope Private. The recorded live exchange shows cacheScope private on both server/discover and tools/list. (fixtures/claude/2.1.302/Mcp.tools-list-1.json:71)
@fate n=170 list=stale master=test-plan row=contract/harness-stdout/document v=code
cell: promise
value: Every harness command but logs and supervise prints exactly one JSON document on stdout, starting with v 1, cmd naming the command and ok, in that key order; logs streams ndjson lines unless it fails, and supervise prints nothing.
@fate n=171 list=stale master=test-plan row=contract/cli-json/one-document-per-verb v=code
cell: promise
value: Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
@fate n=172 list=stale master=test-plan row=contract/harness-boot/readiness-failure v=code
cell: promise
value: Exits non-zero with a JSON reason on stdout; a run that refuses to start is surfaced as reason run-exited with its instance and the exit code of its process-exit line, never its own text.
@fate n=173 list=stale master=obs-plan row=absence/pii-scrubbing-regime v=code
cell: capability
value: A PII-scrubbing regime for diagnostics lines beyond the NEVER-log floor
@fate n=174 list=stale master=obs-plan row=contract/log-levels/error v=code
cell: promise
value: Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
@fate n=175 list=stale master=obs-plan row=contract/ipc-invocation-log/fields v=code
cell: promise
value: Channel-request and channel-response lines carry method, conn (srv_conn when the client sent none) and the corr of a frame with an id; a request line adds sender, a response line result class and an error's error code but never sender.
@fate n=176 list=stale master=obs-plan row=contract/viola-verify/home-tree-refused v=code
cell: promise
value: A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
@fate n=177 list=stale master=obs-plan row=contract/budget-log-line/required-fields v=none
because: The budget log line belongs to the budget gate, which leaves the product in 0.2.0.
@fate n=178 list=stale master=obs-plan row=contract/budget-log-line/budget-paused v=none
because: The budget-paused refusal belongs to the budget gate, which leaves the product in 0.2.0.
@fate n=179 list=stale master=obs-plan row=contract/channel-response/internal-error v=code
cell: promise
value: A -32603 is logged at ERROR with result_class error, error_code -32603 and no detail; the fixed text internal error is only the reply's message, never a log field.
@fate n=180 list=stale master=obs-plan row=contract/claude-child-process-log/handle-wait-thread v=code as=contract/claude-child-process-log/handle-wait-exit
cell: promise
value: process-start is logged once the child is spawned and process-exit once the pump returns; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
@fate n=181 list=stale master=obs-plan row=contract/dialog-spans/span-chain v=code
cell: promise
value: hook.handle, its channel.request(hook.dialog) on a thread of its own → channel.dispatch › run.dialog_register › run.dialog_await; the decision is the hook-decision line inside hook.handle, not a span; driver side answer.client › channel.request(answer) → channel.dispatch.
@fate n=182 list=stale master=obs-plan row=contract/detail-line/fields v=code
cell: promise
value: Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
@fate n=183 list=stale master=obs-plan row=contract/diagnostics-line/instance v=code
cell: promise
value: Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
@fate n=184 list=stale master=obs-plan row=contract/agent-run-logs/filters v=code
cell: promise
value: The merged output is filterable by --instance, --kind and --process.
@fate n=185 list=stale master=obs-plan row=contract/send-line/wrapper-side v=code
cell: promise
value: send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.
@fate n=186 list=stale master=obs-plan row=contract/diagnostics-files/mcp v=code
cell: promise
value: Role mcp writes its role file mcp.ndjson in the home's diagnostics folder, which never carries user content; a role file that cannot be opened ends the server with exit 1 and nothing on stderr.
@fate n=187 list=relied master=design-system rec=fct-mfvo2or6@1 v=keep as=decision/list-refuses-board-on-strict-modes-failure
name: Strict-modes failure refuses the whole board
rule: A failed strict-modes check of the home or any instance refuses the whole list board with exit 21, in the fixed unable and hint pair or, under --json, the instance-unreachable document with detail strict-modes-failed.
why: The MCP list tool reads the same refusal document through the shared board reader, and the list tests pin exit 21 and the document byte for byte.
at: src/cmd/list.rs:43
@fate n=188 list=relied master=security-plan rec=fct-cw3l7aky@1 v=keep as=decision/send-text-is-one-bracketed-paste
name: Send text typed as one bracketed paste
rule: Send text is written into the PTY as one bracketed paste followed by Enter, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes.
why: The control-character refusal exists to keep the paste closed. It runs in the wrapper's send, in CLI send and answer, and in the MCP send and answer tools.
at: crates/viola-pty/src/pump.rs:79
@fate n=189 list=relied master=security-plan rec=fct-a6h3q7dw@2 v=cut
because: Duplicates line 16, which keeps the same check's verdicts: an absent path passes and an unstattable one is refused.
@fate n=190 list=relied master=security-plan rec=fct-ohbg3pqy@1 v=keep as=decision/one-socket-dir-check-for-bind-and-clients
name: One socket-directory check for bind and clients
rule: The wrapper's bind and the three client seams share one check on the endpoint's parent directory before they bind or connect. The three seams are the CLI frame verbs, the hooks and the MCP frame tools.
why: Both sides of the socket read the same verdict, so a directory refused to a client is refused to a start, and the socket-dir-refused detail means one thing.
at: crates/viola-state/src/strict.rs:154
@fate n=191 list=relied master=security-plan rec=fct-6oinutja@1 v=keep as=decision/start-checks-socket-dir-before-bind
name: Socket directory checked before any bind, snapshot or child
rule: A start, run or revive alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict.
why: When the directory is refused, the start negative test expects no child, no bind, no snapshot and the directory as found.
at: src/cmd/run.rs:510
@fate n=192 list=relied master=security-plan rec=fct-pf2l572y@1 v=keep as=decision/refused-peer-dropped-before-any-frame
name: Refused peer dropped before any frame
rule: A peer the server refuses is dropped before its thread starts and before any frame is read, so it reads end-of-file, and no line and nothing on the terminal is written.
why: The server-side euid admission rests on it, and the serve test expects a refused peer's request to end closed with nothing dispatched.
at: crates/viola-channel/src/server.rs:145
@fate n=193 list=relied master=security-plan rec=fct-mhqlyrp7@2 v=keep as=decision/start-checks-home-tree-before-log-opens
name: Start checks the home tree before its log opens
rule: Run and revive's start check the whole home tree before the wrapper log opens: the home first, then the instance, ledger, bin and plugin folders and the pinned copy's own. A refusal exits 1 with no line or child.
why: Opening the log narrows the home to its owner, which would erase the state the check refuses; the revive and start tests expect the widened folder left as found.
at: src/cmd/run.rs:169
@fate n=194 list=relied master=security-plan rec=fct-gfc5nchi@2 v=cut
because: Duplicates line 9, kept as the rule that reading a connected stream's peer pid or euid writes and logs nothing.
@fate n=195 list=relied master=security-plan rec=fct-zpaxdj2d@2 v=keep as=decision/unreadable-peer-euid-is-refused
name: Unreadable peer euid refused, read silently
rule: A peer whose euid cannot be read is refused, and reading a connected stream's peer euid or pid writes and logs nothing.
why: Server admission and the client server check both rest on it, and the negative tests expect no pid or uid in any output or line.
at: crates/viola-channel/src/client.rs:139
@fate n=196 list=relied master=security-plan rec=fct-5ryfhirx@2 v=keep as=decision/only-matches-verdict-lets-a-frame-out
name: Only a Matches server verdict lets a frame out
rule: Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a client write a frame; a mismatched pid or start time, or an unverifiable server, is refused before any frame.
why: The CLI verbs, the hooks and the MCP frame tools rely on it to reach only the recorded wrapper; server-verify-failed names the refusal.
at: src/cmd/client.rs:180
@fate n=197 list=relied master=security-plan rec=fct-p7bllbag@2 v=keep as=decision/frame-verb-check-order
name: Frame verb check order, link and unlink included
rule: Link and unlink take the same order as every CLI frame verb: the strict-modes check, the liveness pre-check, then the socket-directory, euid and server checks before their one frame.
why: The socket-directory negative test runs link and unlink alongside the other six frame verbs and expects exit 21 with no frame on either side.
at: src/cmd/link.rs:94
@fate n=198 list=relied master=architecture rec=fct-y6frgi47@1 v=keep as=decision/replayed-snapshot-gives-no-live-endpoint
name: Replayed snapshot gives no live endpoint
rule: A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound.
why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
at: src/cmd/run.rs:441
@fate n=199 list=relied master=architecture rec=fct-xcjb7ry2@1 v=keep as=decision/plugin-install-rewrites-marketplace-whole
name: Plugin install rewrites the marketplace tree whole
rule: The plugin marketplace tree is rewritten whole on every plugin install, any other entry there is left as found, and viola never reads the tree back.
why: A rerun re-points the MCP server at the current exe and home, the rerun test expects the same tree, and only its folders need the strict-modes check.
at: src/cmd/plugin.rs:115
@fate n=200 list=relied master=architecture rec=fct-q7tym3q2@1 v=keep as=decision/silent-roles-at-internal-error-catch
name: Run, revive, mcp and hook silent at the catch site
rule: Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there.
why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
at: src/main.rs:144
@fate n=201 list=relied master=architecture rec=fct-dhyimyqh@1 v=keep as=decision/connect-failure-dead-closed-stream-lost
name: Failed connect is instance-dead, lost stream endpoint-lost
rule: A failed connect reads as the instance-dead cause and a closed or broken stream as the endpoint-lost cause, on the CLI and in the MCP tools alike.
why: The exit 21 detail words, the process-exit during field and the MCP instance-unreachable detail all rest on the split.
at: src/cmd/client.rs:196
@fate n=202 list=relied master=architecture rec=fct-kbitf2mh@2 v=keep as=decision/socket-dir-check-verdicts
name: Socket-directory check verdicts, never chmods
rule: The socket-directory check passes an absent path and never chmods, so a refused directory stays as found. It refuses a symlink, a non-directory, another owner, any group or other bit, and an unstattable path.
why: A start makes an absent directory itself, and the negative tests for start, frame verbs and hooks expect a loosened or linked directory left as found.
at: crates/viola-state/src/strict.rs:137
@fate n=203 list=relied master=architecture rec=fct-kan7g7fj@1 v=keep as=decision/mcp-frame-tools-strict-modes-check
name: MCP frame tools refuse a strict-modes failure
rule: Every MCP frame tool refuses a home or target instance directory that fails the strict-modes check with the instance-unreachable result and detail strict-modes-failed, sending no frame.
why: The handlers test expects the refusal before any snapshot read or frame, so an untrusted home never steers a frame to a wrapper.
at: crates/viola-mcp/src/tools.rs:339
@fate n=204 list=relied master=architecture rec=fct-au3pdba5@1 v=keep as=decision/mcp-unreachable-result-for-every-cause
name: MCP frame tools report every unreachable cause one way
rule: Every unreachable cause an MCP frame tool meets surfaces as the instance-unreachable result with the cause in detail and no line of its own; every cause but endpoint-lost is found before any frame is sent.
why: MCP drivers branch on the one instance-unreachable error and its closed detail; in the MCP process the result is the only record of the cause.
at: crates/viola-mcp/src/tools.rs:342
@fate n=205 list=relied master=architecture rec=fct-vwfyd52k@1 v=keep as=decision/dialog-hook-fails-open-on-server-check
name: Dialog hook fails open on a failed server check
rule: A dialog hook whose server check fails sends no hook.dialog and exits 0 at once with no decision body, so the dialog renders for the human.
why: The fail-open hook contract and the server-identity negative tests rely on it, so a process that is not the recorded wrapper never answers a dialog.
at: src/cmd/hook.rs:318
@fate n=206 list=relied master=architecture rec=fct-4qhayrrs@1 v=keep as=decision/hook-without-trusted-instance-exits-silently
name: Hook with no trusted instance exits 0 untouched
rule: A hook whose instance check refuses or finds no wrapped instance exits 0 before reading config or stdin, and writes no line, frame or file.
why: The fail-open hook contract and the hook strict-modes tests rely on it, and the check must come before the log open that narrows the home.
at: src/cmd/hook.rs:116
@fate n=207 list=relied master=architecture rec=fct-vag5jbae@1 v=keep as=decision/verify-refuses-untrusted-home-or-stamps
name: Verify refuses an untrusted home tree or stamps
rule: A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written.
why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
at: src/cmd/verify.rs:67
@fate n=208 list=relied master=architecture rec=fct-2iyz7ner@1 v=cut
because: Duplicates line 23: a stale wrapper gives no live endpoint, so under that rule its existing directory reads instance-dead.
@fate n=209 list=relied master=architecture rec=fct-e4yo5lcc@1 v=keep as=decision/no-live-endpoint-dead-or-unwrapped
name: No live endpoint reads instance-dead or unwrapped-name
rule: Without a live endpoint, whether the wrapper is stale, gone or replayed, a client reads an existing instance directory as instance-dead and anything else as an unwrapped name.
why: The CLI frame verbs and the MCP frame tools take their exit 21 detail and hint from it, and the cross exit-cause tests pin both words.
at: crates/viola-state/src/liveness.rs:110
@fate n=210 list=relied master=architecture rec=fct-zgxijmsq@1 v=cut
because: Duplicates line 12, whose kept rule already holds the heartbeat-alone collision check and its already-live refusal.
@fate n=211 list=relied master=architecture rec=fct-5kgumnrt@1 v=keep as=decision/revive-reads-snapshot-once
name: Revive reads the snapshot once
rule: A revive reads the snapshot once through the replay for both its holder check and its recorded directory, so a replayed snapshot makes one replay and logs one state-recovered line.
why: The cwd-missing revive test expects one replay and one state-recovered line in the role file.
at: src/cmd/revive.rs:210
@fate n=212 list=relied master=architecture rec=fct-wwpgfv5r@1 v=cut
because: Duplicates line 7, whose kept rule already holds the exit 1 refusal with no line or child before the log opens.
@fate n=213 list=relied master=architecture rec=fct-mefdhndu@1 v=keep as=decision/socket-dir-refused-covers-create-and-check
name: Socket-dir create failure and refusal read the same
rule: A failed socket-directory create and a refused check both refuse the start as socket-dir-refused with exit 1. A failed bind does not: it reads squatted-name when another process holds the endpoint and internal-error otherwise.
why: Machine readers branch on the socket-dir-refused start document and exit line, and the cross exit-cause test pins it at exit 1 for a start.
at: src/cmd/run.rs:518
@fate n=214 list=relied master=architecture rec=fct-5gkabtaw@2 v=cut
because: Callers discard the error and word their own refusal, run as internal-error and verify as program-not-found, so nothing reads the variant or its message.
@fate n=215 list=relied master=architecture rec=fct-5ijnay4v@1 v=keep as=decision/start-pins-the-bytes-it-checked
name: Start pins the very bytes it checked
rule: A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked.
why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
at: src/cmd/run.rs:201
@fate n=216 list=relied master=architecture rec=fct-nbf2wdvs@2 v=keep as=decision/verify-run-e-resumes-run-b-first-session
name: Verify's Run E resumes Run B's first session
rule: Verify's Run E resumes the session id taken from Run B's first SessionStart, only when that id has a session id's shape; with no such id Run E starts no child, so Run E depends on Run B running before it.
why: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the five runs execute in order A to E.
at: crates/viola-agent-claude/src/ledger.rs:884
@fate n=217 list=relied master=architecture rec=fct-2oafusjr@1 v=keep as=decision/hook-missing-instance-dir-exits-silently
name: Hook with a missing instance folder exits 0 silently
rule: A hook whose VIOLA_DIR names an instance folder that does not exist fails the canonical-directory step, exits 0 and writes nothing: no role file, no stdin read, no frame and no event line.
why: Claude Code runs the plugin's hooks in sessions viola does not wrap; the hook fail-open contract and its no-instance-directory test rely on it.
at: src/cmd/hook.rs:116
@fate n=218 list=relied master=architecture rec=fct-wdtionxy@2 v=cut
because: Does not hold beyond the hook: run, the frame verbs, verify and mcp propagate the refused role-file open and exit 1 as an internal error. The failed-open and dropped-detail part is kept under item 51.
@fate n=219 list=relied master=architecture rec=fct-vs5zumck@2 v=cut
because: Duplicates lines 7 and 21, kept as the start's and verify's home-tree checks before any log opens (the reader's keep corrected by the overseer).
@fate n=220 list=relied master=architecture rec=fct-sbmskf35@2 v=keep as=decision/path-under-a-file-is-a-strict-modes-refusal
name: A path under a regular file is a strict-modes refusal
rule: A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair.
why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
at: crates/viola-state/src/strict.rs:184
@fate n=221 list=relied master=architecture rec=fct-flit34us@1 v=keep as=decision/cleanup-reads-refused-or-missing-socket-gone
name: Cleanup reads a refused or missing socket as gone
rule: Harness cleanup reads the endpoint as gone when a connect is refused or the path is absent, and removes no leftover socket file, so cleanup reads ok after a force-killed wrapper.
why: A killed wrapper removes nothing and the next bind takes its leftover; the force-killed-wrapper lifecycle test asserts ok and the file still standing.
at: crates/viola-e2e/src/harness/cleanup.rs:136
@fate n=222 list=relied master=architecture rec=fct-w6vpbrjt@2 v=keep as=decision/root-tests-freeze-wrappers-with-one-guard
name: Root tests freeze a wrapper through one SIGSTOP guard
rule: Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop.
why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
at: tests/support/frozen.rs:10
@fate n=223 list=relied master=architecture rec=fct-vp2lxvsd@2 v=keep as=decision/clients-read-server-pid-euid-from-peer-creds
name: Clients read server pid and euid from peer credentials
rule: Both channel clients, sync and async, read the serving process's pid and effective uid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame.
why: The CLI, hook and MCP server checks admit the euid and match the pid against the snapshot's recorded pid ahead of any frame.
at: crates/viola-channel/src/client.rs:132
@fate n=224 list=relied master=architecture rec=fct-e6lqqdby@1 v=keep as=decision/server-start-time-read-through-sysinfo
name: Server start time is read from the OS through sysinfo
rule: The server check reads the serving pid's start time through sysinfo, with a zombie reading as gone, and matches it in whole seconds against the snapshot's recorded start time before the first frame.
why: The CLI, hook and MCP clients all verify the server this way, so a reused pid with another start time reads as a mismatch.
at: crates/viola-state/src/liveness.rs:157
@fate n=225 list=relied master=architecture rec=fct-ekks4dfz@1 v=keep as=decision/stamps-writer-checks-before-and-under-lock
name: Stamps writer checks strict modes before and under its lock
rule: The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none.
why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
at: crates/viola-state/src/stamps.rs:61
@fate n=226 list=relied master=architecture rec=fct-fr2pme2q@1 v=keep as=decision/link-set-changes-only-on-link-calls
name: Link set changes only on implicit links and link calls
rule: The wrapper's link set changes only on an implicit link from a send or an answer and on a served link or unlink call, with each change appended to the instance log first and then mirrored to the snapshot.
why: Restoring the set at start replays only link and unlink lines, so the snapshot and the log agree only while no other path changes the set.
at: src/run/links.rs:130
@fate n=227 list=relied master=architecture rec=fct-ng4l52uh@1 v=cut
because: Does not hold: list builds its own exit-21 document (its refused_document) beside the client's shared unreachable_document.
@fate n=228 list=relied master=architecture rec=fct-a6y4blve@2 v=keep as=decision/frame-client-check-order
name: Frame-writing clients run their checks in one order
rule: A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check.
why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass.
at: crates/viola-mcp/src/tools.rs:339
@fate n=229 list=relied master=architecture rec=fct-4yfg5mra@1 v=cut
because: Duplicates line 23, kept as decision/no-live-endpoint-dead-or-unwrapped (the reader's keep corrected by the overseer).
@fate n=230 list=relied master=architecture rec=fct-ikwybol4@2 v=cut
because: Duplicates line 9, kept as decision/unreadable-peer-euid-is-refused under the same key (the reader's keep corrected by the overseer).
@fate n=231 list=relied master=test-plan rec=fct-njspb5nv@3 v=keep as=decision/child-exit-read-on-the-process-handle
name: Child exit is read on the process handle, never on EOF
rule: The pump detects the child's exit by polling the process handle and never by output EOF, then drains the output still in flight for a bounded time.
why: The wrapper's exit code relies on it, and the pump tests use a reader that never ends so only the handle can end the pump.
at: crates/viola-pty/src/pump.rs:169
@fate n=232 list=relied master=test-plan rec=fct-xq6vpu6l@1 v=cut
because: A post-mortem aid inside one test helper's readiness wait; only a person reads the stall lines after a failure, and no other part relies on them.
@fate n=233 list=relied master=test-plan rec=fct-pb23i77m@1 v=keep as=decision/harness-boot-creates-stamps-and-starts
name: Harness boot makes the home, stamps it and starts wrappers
rule: The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready.
why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
at: crates/viola-e2e/src/harness/boot.rs:149
@fate n=234 list=relied master=test-plan rec=fct-wok64pug@1 v=keep as=decision/cleanup-removes-home-record-and-supervisor
name: Cleanup removes the home, session record and supervisor
rule: Per session, harness cleanup stops the supervisor and kills it and the recorded processes if needed, removes the session home unless homes are kept, and drops the session record.
why: Boot refuses a session id whose record exists, and cleanup of all sessions finds sessions only by their records.
at: crates/viola-e2e/src/harness/cleanup.rs:89
@fate n=235 list=relied master=test-plan rec=fct-7tiauhj7@1 v=keep as=decision/error-body-check-reports-place-and-class
name: Error-body check reports place and class, never bytes
rule: The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read.
why: A failing witness must print no canary into JUnit, which the secret scan reads.
at: tests/support/events.rs:45
@fate n=236 list=relied master=obs-plan rec=fct-xpt2kzwy@1 v=keep as=decision/panic-before-role-file-leaves-no-record
name: A panic before the role file opens leaves no record
rule: A panic before any role file is opened, in argument parsing or the home and instance checks, leaves no log record, because the panic hook has no sink yet and returns without writing.
why: Strict-modes refusals rely on nothing being created under the home before their check; the panic hook set first keeps the default panic text off stderr.
at: src/main.rs:161
@fate n=237 list=relied master=obs-plan rec=fct-bprhan45@2 v=keep as=decision/refused-diagnostics-path-is-a-failed-open
name: A refused diagnostics path is a failed open
rule: A refused diagnostics folder or file is exactly a failed open: the role file does not open, and a detail line is dropped with nothing written and nothing on stderr.
why: The hook carries on with no subscriber, the other roles end as an internal error, and no line is ever written through a widened folder or a symlink.
at: src/obs.rs:71
@fate n=238 list=relied master=obs-plan rec=fct-dujo6hv4@1 v=keep as=decision/pre-log-strict-refusals-write-no-line
name: Strict-modes refusals before the log opens write no line
rule: Refusals of the instance and home-tree checks that run before a role file opens write no line in any log and open nothing under the home.
why: Opening a log narrows the home and would erase the refused state; the strict-modes negative tests assert that no diagnostics folder appears.
at: src/cmd/client.rs:24
@fate n=239 list=relied master=obs-plan rec=fct-25dvifnk@1 v=keep as=decision/mcp-unreachable-carries-one-cause-word
name: Each MCP unreachable result carries one cause word
rule: Each MCP unreachable outcome (a refused check, a dead or unwrapped instance, a lost endpoint, an unverified server or a refused socket directory) carries exactly one UnreachableCause word as its detail.
why: The MCP process writes no exit line for it, so the result's cause word is the cause's only record for the calling agent.
at: crates/viola-mcp/src/result.rs:30
@fate n=240 list=relied master=obs-plan rec=fct-wmupioom@1 v=keep as=decision/not-a-directory-refusal-message-fixed
name: NotADirectory refusal message is fixed and pathless
rule: The NotADirectory strict-modes refusal reads the fixed message "a trusted state path is not a directory" and names no path.
why: The security plan's fixed-message rule keeps paths out of error text, and the fixed-messages unit test pins every refusal's text.
at: crates/viola-state/src/strict.rs:28
@fate n=241 list=relied master=obs-plan rec=fct-4gke2rkl@1 v=keep as=decision/clients-screen-peer-replies-to-code-words
name: Clients screen peer replies to code words and a fixed fault
rule: The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line.
why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
at: src/cmd/client.rs:68
@fate n=242 list=relied master=obs-plan rec=fct-474ly2yg@1 v=keep as=decision/conn-in-params-joins-cross-process-logs
name: Conn in request params joins logs across processes
rule: Every channel client request carries its conn label in params, and the server logs that conn on its lines (or its own srv label when the conn is missing or malformed) without handing it to the wrapper's answer.
why: Joining a client's request line to the wrapper's handling lines across processes relies on the shared label.
at: crates/viola-channel/src/client.rs:176
@fate n=243 list=relied master=obs-plan rec=fct-rar73ens@1 v=keep as=decision/flat-log-lines-and-one-level-gate
name: Log lines are flat and Targets is the only level gate
rule: The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment.
why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
at: src/obs.rs:122
