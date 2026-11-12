@row_add n=1 row=decision/recorded-fixtures-scrubbed area=occupied-resources rec=1
name: Recorded hook fixtures scrubbed and hygiene-walked
rule: Every fixture written by viola verify --record is recorded only when no ledger row fails, scrubbed (the home becomes ~, the user word becomes <user>), refused whole when unclean, and walked by the fixture hygiene contract.
guard: contract_fixture_hygiene
@row_add n=2 row=decision/screen-fixtures-signature-rows-only area=occupied-resources rec=1
name: Screen fixtures keep only signature rows
rule: A recorded screen fixture keeps only the rows that hold a compiled signature literal; every other row is an empty string.
@row_add n=3 row=contract/unclean-fixture-refusal/message area=occupied-resources rec=1
boundary: unclean fixture refusal
item: message
promise: An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
@row_add n=4 row=contract/unclean-fixture-refusal/home-path area=occupied-resources rec=1
boundary: unclean fixture refusal
item: home-path
promise: The code given when a recording holds a home path.
@row_add n=5 row=contract/unclean-fixture-refusal/absolute-path area=occupied-resources rec=1
boundary: unclean fixture refusal
item: absolute-path
promise: The code given when a recording holds an absolute path.
@row_add n=6 row=contract/unclean-fixture-refusal/username area=occupied-resources rec=1
boundary: unclean fixture refusal
item: username
promise: The code given when a recording holds the username, including one split across a screen row seam.
@row_add n=7 row=contract/unclean-fixture-refusal/email area=occupied-resources rec=1
boundary: unclean fixture refusal
item: email
promise: The code given when a recording holds an email-shaped token.
@row_add n=8 row=decision/fixture-keeps-own-repository-remote area=occupied-resources rec=1
name: Fixture keeps this repository's own remote
rule: A workspace.repo naming this repository's own remote is kept in a fixture rather than scrubbed.
why: ratified
@row_add n=9 row=decision/chunk-diff-secret-scan-skip area=occupied-resources rec=2
name: chunk.diff is the one secret-scan skip
rule: The mutation run's chunk.diff is the one file secret-scan skips, by exact path, and the harness artifact upload excludes it.
@row_add n=10 row=decision/test-home-owner-sweep area=occupied-resources rec=3
name: Test homes removed by owner record only
rule: A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last.
why: a removal that stops part-way keeps the record for a later sweep
@row_add n=11 row=decision/linked-e2e-home-base-strict area=occupied-resources rec=3
name: Linked e2e-home base must be a private real directory
rule: A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path.
@row_add n=12 row=decision/e2e-home-headroom-floor area=occupied-resources rec=3
name: e2e-home backing headroom floor
rule: A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key.
why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB
@row_add n=13 row=contract/e2e-home-refusal/headroom-message area=occupied-resources rec=3
boundary: e2e-home base refusal
item: headroom message
promise: A base below the floor is refused with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, the two counts and no path.
@row_add n=14 row=decision/dev-host-e2e-home-on-tmpfs area=occupied-resources rec=3
name: Dev-host test homes on a tmpfs link
rule: On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk.
why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
@row_add n=15 row=decision/test-side-removal-outside-workdir area=occupied-resources rec=3
name: Test-side removals outside the working directory
rule: Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made.
why: a boundary widening, ratified
@row_add n=16 row=decision/e2e-homes-kept-after-each-job area=occupied-resources rec=4
name: E2E homes kept after every job's end-to-end run
rule: Each CI job keeps its test homes after its end-to-end run, whatever the outcome, the chaos-test home excluded.
@row_add n=17 row=decision/profraw-census-dev-host-only area=occupied-resources rec=5
name: Profraw census is a dev-host witness only
rule: The profraw census script is a dev-host witness for the corrupt-profile mechanism, and no CI job, pre-push stage or harness command runs either of its forms.
@row_add n=18 row=contract/profraw-census-exit-codes/0 area=occupied-resources rec=5
boundary: profraw census exit codes
item: 0
promise: Every run passed and, in the default form, the profile count equals the run count with no third or short profile; in the case form, no profile was refused.
@row_add n=19 row=contract/profraw-census-exit-codes/1 area=occupied-resources rec=5
boundary: profraw census exit codes
item: 1
promise: Any other count.
@row_add n=20 row=contract/profraw-census-exit-codes/2 area=occupied-resources rec=5
boundary: profraw census exit codes
item: 2
promise: A usage error, no instrumented binary, or (case form) no llvm-profdata.
@row_add n=21 row=decision/mutation-scratch-nocow area=occupied-resources rec=6
name: Mutation scratch directory is NOCOW
rule: On the Linux dev host every mutation run's TMPDIR is a sibling directory outside the repository marked NOCOW, an environment fact and not harness code.
why: /tmp is a quota tmpfs too small for the copied tree, and a COW btrfs reflink copy drops the fake agent's exec bit; NOCOW makes the copy fall back to a mode-keeping copy
@row_add n=22 row=decision/rust-toolchain-pin area=stack-and-technologies rec=7
name: Rust toolchain pinned exactly
rule: Rust is the language of one native binary, pinned exactly by rust-toolchain.toml with rustfmt and clippy, edition 2024.
@row_add n=23 row=decision/msrv-floor area=stack-and-technologies rec=7
name: Declared MSRV floor 1.96
rule: The workspace declares rust-version 1.96 and CI builds against that floor without touching the toolchain pin.
why: the security toolchain floor
guard: msrv CI job
@row_add n=24 row=decision/fuzz-nightly-toolchain area=stack-and-technologies rec=7
name: Fuzz workspace alone on nightly
rule: Only the separate fuzz workspace builds on a pinned nightly toolchain.
@row_add n=25 row=decision/hybrid-concurrency area=stack-and-technologies rec=8
name: Std threads everywhere, Tokio only inside mcp
rule: Every CLI verb runs on std threads; Tokio is built only inside mcp.
why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
@row_add n=26 row=decision/clap-cli-parser area=stack-and-technologies rec=9
name: clap as CLI parser, without colour
rule: clap (derive) parses the CLI with its default features minus color.
why: help and usage output stays plain and no colour or terminal crate is in the graph
@row_add n=27 row=decision/portable-pty-seam area=stack-and-technologies rec=10
name: portable-pty behind viola's own pty seam
rule: The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
@row_add n=28 row=decision/libc-only-os-binding area=stack-and-technologies rec=11
name: libc is the one OS-binding crate
rule: libc is the one OS-binding crate of the Linux-only build, a plain dependency; no crate depends on windows-sys.
@row_add n=29 row=decision/vt100-screen-model area=stack-and-technologies rec=12
name: vt100 as screen model
rule: vt100 gives pre-send readiness and modal detection on run's feed thread and the screen of verify's interactive probe runs.
@row_add n=30 row=decision/run-never-reads-screen-content area=stack-and-technologies rec=12
name: run never reads screen content
rule: run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures.
@row_add n=31 row=decision/wrapper-ipc-local-socket area=stack-and-technologies rec=13
name: interprocess local sockets for wrapper IPC
rule: Each viola run has one endpoint, a Unix domain socket through interprocess's sync local_socket API, chmod 0600 after bind in a per-user directory; the Tokio flavour is used only by the MCP client.
@row_add n=32 row=decision/socket-dir-strict-check area=stack-and-technologies rec=13,44
name: Socket directory checked before bind and connect
rule: The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
@row_add n=33 row=decision/no-database area=stack-and-technologies rec=14
name: No database: ndjson logs and JSON snapshots
rule: State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
@row_add n=34 row=decision/versioned-records-no-migrations area=stack-and-technologies rec=15
name: Versioned records instead of migrations
rule: Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log.
except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
@row_add n=35 row=decision/atomic-replaced-state-files area=stack-and-technologies rec=16
name: Replaced files through one atomic helper
rule: Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt.
why: crash-safe multi-process writes with no C code
@row_add n=36 row=decision/pinned-copy-placed-once area=stack-and-technologies rec=16
name: Pinned copy placed once, never replaced
rule: The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
@row_add n=37 row=decision/separate-lock-files area=stack-and-technologies rec=16
name: Locks on separate .lock files
rule: State locking uses std file locks on separate .lock files, opened for write.
why: crash-safe multi-process writes with no C code
@row_add n=38 row=decision/append-one-write-per-line area=stack-and-technologies rec=16
name: Log appends one write per line
rule: Log appends use std append mode with one write per line.
why: crash-safe multi-process writes with no C code
@row_add n=39 row=decision/content-hash-sha256 area=stack-and-technologies rec=17
name: Pure-Rust SHA-256 content hash
rule: The bin/ and plugin/ version-hash keys are the exe's SHA-256 truncated to 8 bytes (16 hex), through sha2 with no cc.
@row_add n=40 row=decision/pinned-copy-rehash-before-reuse area=stack-and-technologies rec=17
name: Pinned copy re-hashed before every reuse
rule: An existing pinned copy is re-hashed before every reuse.
@row_add n=41 row=decision/sha2-optimized-in-dev area=stack-and-technologies rec=17
name: sha2 optimised in dev builds
rule: The dev profile builds sha2 at opt-level 3.
why: dev and test builds hash the debug exe at release speed
@row_add n=42 row=decision/serde-serialization area=stack-and-technologies rec=18
name: serde for frames, logs and snapshots
rule: serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
@row_add n=43 row=decision/preserve-key-order area=stack-and-technologies rec=18
name: Printed documents keep declared key order
rule: serde_json's preserve_order feature is on, so a printed document keeps its declared key order.
@row_add n=44 row=decision/no-unbounded-json-depth area=stack-and-technologies rec=18
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned.
why: every parse of external bytes keeps serde_json's default recursion limit
guard: deny.toml
@row_add n=45 row=decision/tracing-process-logs area=stack-and-technologies rec=19
name: tracing for one-line JSON process logs
rule: Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
@row_add n=46 row=decision/frame-params-redacted area=stack-and-technologies rec=19
name: Frame params never rendered in Debug
rule: A frame's params never reach a Debug render, through veil's derived redaction with its toggle feature banned.
@row_add n=47 row=decision/nutype-domain-newtypes area=stack-and-technologies rec=20
name: nutype domain newtypes
rule: Bounded domain values are nutype newtypes: ViolaName (charset and length cap), Percent (0 to 100), DialogHold (whole seconds 60 to 3600).
@row_add n=48 row=decision/chrono-timestamps area=stack-and-technologies rec=21
name: chrono RFC 3339 UTC timestamps
rule: Timestamps are RFC 3339 UTC with millisecond precision, through chrono.
why: already in the tree through rmcp
@row_add n=49 row=decision/typed-errors-per-crate area=stack-and-technologies rec=22
name: thiserror per crate, anyhow at the bin edge
rule: Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only.
except: viola-pty's error type is hand-written
@row_add n=50 row=decision/error-context-detail-file-only area=stack-and-technologies rec=22
name: Error context chains only in the detail file
rule: Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
@row_add n=51 row=decision/no-message-broker area=stack-and-technologies rec=23
name: No message broker
rule: Request and response go over per-wrapper local sockets as JSON-RPC 2.0 over ndjson, with no broker or daemon.
@row_add n=52 row=decision/process-liveness-signals area=stack-and-technologies rec=24
name: Heartbeat first, sysinfo and claude agents enrich
rule: The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
@row_add n=53 row=decision/mcp-server-rmcp area=stack-and-technologies rec=25
name: rmcp stdio MCP server
rule: viola mcp is a stdio MCP server built on rmcp with schemars tool input schemas, minor version pinned.
@row_add n=54 row=decision/rmcp-release-features area=stack-and-technologies rec=25
name: rmcp release features exactly server and transport-io
rule: The release graph's rmcp features are exactly server and transport-io.
guard: scripts/rmcp-features.sh
@row_add n=55 row=decision/e2e-no-rmcp-child-transport area=stack-and-technologies rec=25
name: E2E owns the viola mcp child
rule: The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit.
why: transport-child-process kills the server 3 s after a close
@row_add n=56 row=contract/mcp-tools/send area=stack-and-technologies rec=25
boundary: MCP tools
item: send
promise: One of the five tools the MCP server offers.
@row_add n=57 row=contract/mcp-tools/wait area=stack-and-technologies rec=25
boundary: MCP tools
item: wait
promise: One of the five tools the MCP server offers.
@row_add n=58 row=contract/mcp-tools/last area=stack-and-technologies rec=25
boundary: MCP tools
item: last
promise: One of the five tools the MCP server offers.
@row_add n=59 row=contract/mcp-tools/answer area=stack-and-technologies rec=25
boundary: MCP tools
item: answer
promise: One of the five tools the MCP server offers.
@row_add n=60 row=contract/mcp-tools/list area=stack-and-technologies rec=25
boundary: MCP tools
item: list
promise: One of the five tools the MCP server offers.
@row_add n=61 row=absence/model-api-calls area=stack-and-technologies rec=26
capability: Calling any model API; viola drives the unmodified claude CLI on the user's subscription
@row_add n=62 row=absence/mobile-view area=stack-and-technologies rec=27
capability: A mobile or phone view
@row_add n=63 row=decision/local-install-no-hosting area=stack-and-technologies rec=28
name: Local-only install, no hosting
rule: v1 is installed locally with cargo install and has no container, hosting or deployment.
@row_add n=64 row=decision/plugin-embedded-in-binary area=stack-and-technologies rec=28
name: Claude Code plugin compiled into the binary
rule: The Claude Code plugin is compiled into the binary and written out by viola run.
@row_add n=65 row=decision/ci-ubuntu-fake-agent area=stack-and-technologies rec=29
name: CI on ubuntu against the fake agent
rule: CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
@row_add n=66 row=decision/pre-push-filter-not-verdict area=stack-and-technologies rec=29
name: Local pre-push gate is a filter, not the verdict
rule: The local pre-push gate runs the ubuntu test job's suites natively in the working tree under a cleared environment, refuses off Linux and installs nothing; it is a pre-push filter, never the verdict of record.
@row_add n=67 row=decision/clippy-print-and-macro-bans area=stack-and-technologies rec=30
name: clippy deny warnings with print and dbg bans
rule: clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
@row_add n=68 row=decision/cargo-deny-dependency-policy area=stack-and-technologies rec=30
name: cargo-deny dependency policy
rule: cargo-deny enforces advisories, licences, sources and bans: C crates, telemetry crates, feature bans, and tokio banned per sync crate.
@row_add n=69 row=decision/workflow-lint-zizmor area=stack-and-technologies rec=30
name: GitHub workflows linted by zizmor
rule: GitHub workflows are linted by zizmor.
@row_add n=70 row=decision/coverage-floors-gate area=stack-and-technologies rec=30
name: Coverage floors enforced by the harness gate
rule: Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
@row_add n=71 row=decision/fuzz-workspace-deny-exemption area=stack-and-technologies rec=30
name: Fuzz workspace outside the cargo-deny graph
rule: Fuzzing runs in CI only, in a separate fuzz workspace with its own lockfile outside the root cargo-deny graph.
why: a test-only exemption
@row_add n=72 row=decision/syn-test-only area=stack-and-technologies rec=30
name: syn and proc-macro2 test-only
rule: syn and proc-macro2 are direct dependencies of the test-only E2E crate alone; no product crate depends on either.
@row_add n=73 row=decision/release-tooling-later area=stack-and-technologies rec=31
name: Release tooling deferred past v1
rule: Public signed releases and installers come with dist and cargo-auditable, and self_update later, once distribution is in scope, not in v1.
@row_none n=1 rec=32 type=4
relied: Both channel clients read the serving process's pid and euid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame.
@row_none n=2 rec=33 type=4
relied: The serving process's start time read through sysinfo is the observed side of the server check each frame-writing client makes before its first frame.
@row_add n=74 row=contract/list-item/budget-paused area=standard-contracts rec=34
boundary: list payload item
item: budget_paused
promise: The instance's own budget gate from its snapshot; false after a release --budget override, absent for an unwrapped session.
@row_add n=75 row=contract/list-item/wrapped area=standard-contracts rec=34
boundary: list payload item
item: wrapped
promise: false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, budget_paused, cli_version or cli_verified.
@row_add n=76 row=contract/list-item/liveness area=standard-contracts rec=34
boundary: list payload item
item: liveness
promise: live or stale; gone instances are omitted.
@row_add n=77 row=contract/list-item/status area=standard-contracts rec=34
boundary: list payload item
item: status
promise: idle, busy or unknown.
@row_add n=78 row=contract/list-item/wheel area=standard-contracts rec=34
boundary: list payload item
item: wheel
promise: human or driver; absent when the session is unwrapped.
@row_add n=79 row=contract/list-payload/budget area=standard-contracts rec=34
boundary: list payload
item: budget
promise: {five_hour, seven_day, read_at, paused} or "unknown"; each window is {used_percentage, resets_at}, a field "unknown" when unparseable and a window "unknown" when omitted; paused only says the newest reading crosses a threshold.
@row_add n=80 row=contract/snapshot-envelope/shape area=standard-contracts rec=35
boundary: snapshot envelope
item: shape
promise: An atomically replaced JSON document {v, written_at, writer, data}.
@row_add n=81 row=contract/snapshot-envelope/v area=standard-contracts rec=35
boundary: snapshot envelope
item: v
promise: Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
@row_add n=82 row=decision/snapshot-replay-log-fields-only area=standard-contracts rec=35
name: Snapshot replay recovers only log-derived fields
rule: A snapshot rebuilt by replay recovers only links, agent_session_id, wheel and the budget gate from the log; a field no line gave is absent, never a default.
@row_add n=83 row=decision/replayed-snapshot-liveness area=standard-contracts rec=35
name: Replayed snapshot liveness rests on the heartbeat
rule: For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
@row_add n=84 row=decision/replay-writes-no-file area=standard-contracts rec=35
name: Snapshot replay writes no file
rule: Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
@row_add n=85 row=decision/hook-fails-open-on-replayed-snapshot area=standard-contracts rec=35
name: Hook fails open on a replayed snapshot
rule: viola hook reads a replayed or absent snapshot as unreachable and fails open.
@row_add n=86 row=decision/revive-refused-without-cwd area=standard-contracts rec=35
name: Revive over a replayed snapshot refused
rule: A replayed snapshot gives no cwd, so a revive over one is refused cwd-missing.
@row_add n=87 row=contract/ledger-stamps/writer area=standard-contracts rec=36
boundary: ledger stamps envelope
item: writer
promise: The literal "verify", not a binary version; the file is written only by viola verify.
@row_add n=88 row=contract/ledger-stamps/rows area=standard-contracts rec=36
boundary: ledger stamps envelope
item: rows
promise: Maps each ledger row id to "pass" or "fail".
@row_add n=89 row=contract/ledger-stamps/measured area=standard-contracts rec=36
boundary: ledger stamps envelope
item: measured
promise: The probe's readings (largest hook payload per spine event, typed_probe timings, dialog_probe); additions need no v bump, and run reads no field of it.
@row_add n=90 row=decision/stamps-merge-own-version-only area=standard-contracts rec=36
name: Stamps merge replaces only its own version
rule: A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
@row_add n=91 row=decision/version-verified-all-relied-rows area=standard-contracts rec=36
name: A CLI version is verified only when every relied row passes
rule: A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
@row_add n=92 row=decision/malformed-stamps-unverified area=standard-contracts rec=36
name: Malformed or untrusted stamps read unverified
rule: A non-v1 or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified false with one parse-rejected line.
@row_none n=3 rec=37 type=4
relied: The stamps writer runs the strict-modes check before creating the ledger directory and again under its lock, and a refusal reads and writes nothing.
@row_add n=93 row=contract/instance-snapshot/links area=standard-contracts rec=38
boundary: instance snapshot
item: links
promise: The set derived by replaying link and unlink events, as {driver, driven, since}, since being the link line's ts.
@row_add n=94 row=decision/link-set-follows-log area=standard-contracts rec=38
name: Link set follows the log
rule: The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails.
@row_none n=4 rec=39 type=4
relied: The link set changes only on an implicit link and on a served link or unlink call.
@row_add n=95 row=contract/channel-link/already-linked area=standard-contracts rec=40
boundary: channel link method
item: already_linked
promise: A pair the set holds appends nothing and answers that pair's since with already_linked true; a new pair answers its new link line's ts as since with already_linked false.
@row_add n=96 row=contract/channel-unlink/result area=standard-contracts rec=41
boundary: channel unlink method
item: result
promise: Answers {} whether or not the set held the pair; an absent pair appends nothing and leaves the set unchanged.
@row_add n=97 row=contract/channel-link-unlink/error-32602 area=standard-contracts rec=42
boundary: channel link and unlink methods
item: -32602
promise: A link or unlink answered -32602 writes no line.
@row_add n=98 row=contract/channel-link-unlink/error-32603 area=standard-contracts rec=42
boundary: channel link and unlink methods
item: -32603
promise: A failed record or remove of the link answers -32603.
@row_add n=99 row=decision/server-identity-check-before-frame area=standard-contracts rec=43
name: Server identity checked before the first frame
rule: Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame.
@row_add n=100 row=decision/client-strict-modes-before-frame area=standard-contracts rec=44
name: Strict-modes check before any client frame
rule: Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
@row_add n=101 row=contract/channel-send/result area=standard-contracts rec=45
boundary: channel send method
item: result
promise: {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
@row_add n=102 row=contract/channel-send/budget-paused area=standard-contracts rec=45
boundary: channel send method
item: budget-paused
promise: A closed budget gate read at arrival refuses with budget-paused, detail five-hour or seven-day, before send-issued, with no cursor, nothing typed and no flight reserved.
@row_add n=103 row=contract/channel-send/from area=standard-contracts rec=45
boundary: channel send method
item: from
promise: A from naming another instance records the link from that instance to this one directly before send-issued; a link that cannot be recorded answers -32603 with nothing issued or typed; no from, or the wrapper's own name, records nothing.
@row_add n=104 row=contract/channel-wait/result area=standard-contracts rec=46
boundary: channel wait method
item: result
promise: {after?, timeout_ms?, from?} answers {event, cursor}, cursor the end offset of that event's line, or {timed_out:true}.
@row_add n=105 row=contract/channel-wait/params area=standard-contracts rec=46
boundary: channel wait method
item: after and timeout_ms
promise: Each absent or a non-negative integer fitting u64; null, a negative, a fraction or a string is -32602; unknown fields are ignored.
@row_add n=106 row=contract/channel-wait/start-offset area=standard-contracts rec=46
boundary: channel wait method
item: start offset
promise: The start is after, else the log's end at the call; a line counts only when it starts at or after it, and a pending dialog at or after the start is returned at once.
@row_add n=107 row=contract/channel-wait/wake-kinds area=standard-contracts rec=46
boundary: channel wait method
item: wake kinds
promise: Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records never wake.
@row_add n=108 row=contract/channel-link-unlink/from area=standard-contracts rec=47
boundary: channel link and unlink methods
item: from
promise: link and unlink read no from; a from of any shape changes nothing.
@row_add n=109 row=contract/channel-link-unlink/answer-order area=standard-contracts rec=47
boundary: channel link and unlink methods
item: answer order
promise: Each answers only after its event line and the snapshot write.
@row_add n=110 row=contract/channel-32602-cause/unsupported-version area=standard-contracts rec=48
boundary: channel -32602 causes
item: unsupported-version
promise: One of the three causes a -32602 fault reads.
@row_add n=111 row=contract/channel-32602-cause/invalid-params area=standard-contracts rec=48
boundary: channel -32602 causes
item: invalid-params
promise: One of the three causes a -32602 fault reads.
@row_add n=112 row=contract/channel-32602-cause/release-from-driver area=standard-contracts rec=48
boundary: channel -32602 causes
item: release-from-driver
promise: One of the three causes a -32602 fault reads.
@row_add n=113 row=contract/hook-event/notification area=standard-contracts rec=49
boundary: channel hook.event
item: notification
promise: Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}}.
@row_add n=114 row=contract/hook-event/kind area=standard-contracts rec=49
boundary: channel hook.event
item: kind
promise: One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
@row_add n=115 row=contract/hook-event/appended-line area=standard-contracts rec=49
boundary: channel hook.event
item: appended line
promise: A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
@row_add n=116 row=contract/hook-event/driver-relabel area=standard-contracts rec=49
boundary: channel hook.event
item: driver relabel
promise: A prompt-submitted whose text equals the in-flight send's typed text is appended with origin driver and only then settles that send.
@row_add n=117 row=decision/list-needs-no-wrapper area=standard-contracts rec=50
name: list is not a channel method
rule: list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
@row_add n=118 row=contract/list-payload/ok area=standard-contracts rec=50
boundary: list payload
item: ok
promise: CLI --json and MCP list return {items, budget, skipped}.
@row_add n=119 row=contract/list-payload/agents-read-failure area=standard-contracts rec=50
boundary: list payload
item: agents read failure
promise: A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item.
@row_add n=120 row=contract/mcp-list/refusal area=standard-contracts rec=50
boundary: MCP list
item: refusal
promise: The board's only refusal returns instance-unreachable with detail strict-modes-failed.
@row_add n=121 row=contract/event-prompt-submitted/text area=standard-contracts rec=51
boundary: prompt-submitted event data
item: text
promise: The submitted prompt as typed, with any long-paste wrapper and its frame removed and tag escaping reversed; a pasted text that ended in a newline is one byte short of it.
@row_add n=122 row=contract/event-prompt-submitted/origin area=standard-contracts rec=51
boundary: prompt-submitted event data
item: origin
promise: driver, human or harness; the hook files only human or harness, and driver is set only by the wrapper's relabel of the in-flight send's own prompt.
@row_add n=123 row=contract/cli-link-json/ok area=standard-contracts rec=52
boundary: viola link and unlink --json output
item: ok
promise: link prints {"v":1,"ok":{"since":…,"already_linked":…}} and unlink prints {"v":1,"ok":{}}, each with exit 0.
@row_add n=124 row=contract/cli-exit-21-document/strict-modes-failed area=standard-contracts rec=53,52,54
boundary: exit-21 CLI document
item: strict-modes-failed
promise: A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line.
@row_add n=125 row=contract/cli-exit-21-document/instance-dead area=standard-contracts rec=53
boundary: exit-21 CLI document
item: instance-dead
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=126 row=contract/cli-exit-21-document/unwrapped-name area=standard-contracts rec=53
boundary: exit-21 CLI document
item: unwrapped-name
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=127 row=contract/cli-exit-21-document/endpoint-lost area=standard-contracts rec=53
boundary: exit-21 CLI document
item: endpoint-lost
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=128 row=contract/cli-exit-21-document/server-verify-failed area=standard-contracts rec=53
boundary: exit-21 CLI document
item: server-verify-failed
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_add n=129 row=contract/cli-exit-21-document/socket-dir-refused area=standard-contracts rec=53,52,54
boundary: exit-21 CLI document
item: socket-dir-refused
promise: A refused socket directory met at connect prints the instance-unreachable document with this detail (exit 21) and one process-exit line at WARN.
@row_none n=5 rec=55 type=4
relied: Every exit-21 CLI document is written by one writer.
