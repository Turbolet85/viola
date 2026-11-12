@row_add n=1 row=decision/cli-capability-ledger area=established-decisions rec=1
name: Per-behaviour CLI capability ledger
rule: Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_add n=2 row=decision/unverified-cli-transport-only area=established-decisions rec=1
name: Transport-only degrade on an unverified CLI
rule: On an unverified CLI version viola still types, runs the wheel and emits events, but withholds dialog answers, and drivers get unverified-cli.
@row_add n=3 row=contract/claude-cli-version-output/first-line area=established-decisions rec=1
boundary: Claude CLI --version output
item: first line
promise: Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
@row_add n=4 row=decision/exit-plan-mode-allow-with-updated-input area=established-decisions rec=1
name: ExitPlanMode answered only through PreToolUse
rule: An ExitPlanMode plan is approved only through PreToolUse as allow with updatedInput set to the tool's own input unchanged (a non-object input gives {}), and revised through deny with a message.
why: A bare allow left the plan dialog up on the live CLI.
@row_add n=5 row=decision/r8-identity-floor area=established-decisions rec=1
name: R8 parent-identity floor
rule: The measured CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
@row_add n=6 row=decision/harness-prompt-prefixes area=established-decisions rec=1
name: Harness prompts classified by raw prefix
rule: A prompt whose raw start matches a compiled harness prefix (<agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message) is filed harness; an escaped tag is typed text and never classifies harness.
why: Ratified; the escaped cross-session prefix rests on a relayed measurement that the CLI's own injection arrives escaped.
except: <\cross-session-message is a harness prefix, so a human who types that tag at a prompt's start is filed harness too.
@row_add n=7 row=decision/prompt-normalise-order area=established-decisions rec=1
name: Classify before unwrapping and un-escaping
rule: Prompt normalisation classifies harness on the raw prefix first, then removes any long-paste wrapper with its frame, then un-escapes a backslash after < before an ASCII letter or /.
why: So a typed pair is never unwrapped and delivery matching compares plain text.
@row_add n=8 row=decision/no-probe-for-unrelied-shapes area=established-decisions rec=1
name: No probe for a shape no send relies on
rule: A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only.
why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
@row_add n=9 row=decision/send-never-types-trailing-newline area=established-decisions rec=1
name: Send never types a trailing newline
rule: send never types a text that ends in a newline.
why: A pasted text's own last newline never reaches the hook.
@row_add n=10 row=decision/local-command-post-conditions area=established-decisions rec=1
name: Local commands listed with delivery post-conditions
rule: The ledger lists each known built-in local command with its delivery post-condition or none; skills are not local commands.
why: Built-in local commands fire no UserPromptSubmit, while skills expand into a prompt and fire it.
@row_add n=11 row=decision/send-waits-out-paste-hint area=established-decisions rec=1
name: Send waits out the paste hint
rule: On a verified CLI a send issued while the paste hint stands waits in the readiness gate for the input box, bounded at 8.5 s; a hint that outlasts the bound ends not-delivered / input-not-ready with nothing typed.
why: Ratified ruling.
@row_add n=12 row=contract/human-hint-lines/input-not-ready area=established-decisions rec=1
boundary: Human hint lines
item: input-not-ready
promise: Reads "<name> was not ready for input; send again, and if it repeats a human must look at the session" for a modal, a never-quiet screen, a poisoned screen or a hint past the bound; no hint line is printed for the paste-hint cause.
@row_add n=13 row=decision/verify-box-wait-for-literal area=established-decisions rec=1
name: Verify waits for an input-box literal after added turns
rule: After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding a compiled literal, a poisoned screen, or the probe deadline; a quiet screen with no literal keeps waiting.
why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
@row_add n=14 row=decision/verify-types-nothing-into-dialogs area=established-decisions rec=1
name: Verify never types into a CLI dialog
rule: No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key.
why: Ratified rulings.
@row_add n=15 row=decision/verify-child-set area=established-decisions rec=1
name: Verify's bounded child set
rule: viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80x24, and one agents --json read inside Run B, pasting only compiled texts.
why: Each widening of the child set was ratified.
@row_add n=16 row=decision/verify-plan-files-in-run-dir area=established-decisions rec=1
name: Plan-mode probe keeps plan files in its own dir
rule: The plan-mode verify run points plansDirectory at a 0700 plans dir inside its own 0700 run dir.
why: So the plan file lands inside the run's own dir and none under the user's own plans directory.
@row_add n=17 row=contract/viola-verify-output/step-line area=established-decisions rec=1
boundary: viola verify stdout
item: step line
promise: One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
@row_add n=18 row=contract/viola-verify-output/summary-line area=established-decisions rec=1
boundary: viola verify stdout
item: summary line
promise: stamped <ver>  N pass  M fail.
@row_add n=19 row=contract/viola-verify-exit-codes/1 area=established-decisions rec=1
boundary: viola verify exit codes
item: 1
promise: A relied row failed; the failing row's stamp is still written.
@row_add n=20 row=decision/verify-record-only-clean area=established-decisions rec=1
name: Fixtures recorded only from a clean verify
rule: viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
@row_add n=21 row=decision/real-cli-probe-local-only area=established-decisions rec=1
name: Real CLI probes run only locally
rule: The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
@row_add n=22 row=decision/permission-request-question-stays-null area=established-decisions rec=1
name: PermissionRequest-raised question stays null
rule: A question first raised by PermissionRequest stays null to the human.
why: The allow-with-updatedInput body that would satisfy it has no ledger row; adding one needs a 25th row, its verify probe and a re-stamp of every stamped version.
@row_none n=1 rec=2 type=9
@row_none n=2 rec=3 type=4
relied: A strict-modes refusal of the home tree or of the stamps makes viola verify exit 1 with the home pair, reading and writing nothing.
@row_add n=23 row=contract/viola-verify-refusal-codes/program-not-found area=established-decisions rec=4
boundary: viola verify process-exit refusal codes
item: program-not-found
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the program is not found.
@row_add n=24 row=contract/viola-verify-refusal-codes/pinned-hash-mismatch area=established-decisions rec=4
boundary: viola verify process-exit refusal codes
item: pinned-hash-mismatch
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the pinned copy's hash does not match.
@row_add n=25 row=contract/viola-verify-refusal-codes/version-unreadable area=established-decisions rec=4
boundary: viola verify process-exit refusal codes
item: version-unreadable
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.
@row_add n=26 row=contract/viola-verify-refusal-codes/strict-modes-failed area=established-decisions rec=4
boundary: viola verify process-exit refusal codes
item: strict-modes-failed
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.
@row_add n=27 row=contract/viola-verify-refusal-codes/unclean-fixture area=established-decisions rec=4
boundary: viola verify process-exit refusal codes
item: unclean-fixture
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
@row_add n=28 row=contract/viola-verify-refusal-codes/interrupted area=established-decisions rec=4
boundary: viola verify process-exit refusal codes
item: interrupted
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when verify is interrupted.
@row_add n=29 row=decision/session-liveness-heartbeat-and-start-time area=established-decisions rec=5
name: Heartbeat primary, pid plus start time decide gone
rule: The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows.
why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
@row_add n=30 row=decision/run-refuses-held-name area=established-decisions rec=5
name: A start refuses a held name
rule: viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
@row_add n=31 row=decision/revive-refuses-live-session area=established-decisions rec=5
name: Revive refuses a session a live claude holds
rule: viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open).
why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
@row_add n=32 row=decision/revive-resumes-in-place area=established-decisions rec=5
name: Revive resumes a gone instance in place
rule: viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch, and its first record is wheel with holder driver and cause start, as every start's.
why: Ratified: a revived start keeps the wheel at driver.
@row_add n=33 row=decision/run-start-order area=established-decisions rec=5
name: Child spawned last in the start order
rule: A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, evaluates the budget gate, writes the first snapshot, heartbeat and start events, and only then spawns the child.
why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
@row_add n=34 row=decision/exclusive-endpoint-bind area=established-decisions rec=5
name: Exclusive endpoint bind
rule: A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name.
why: So two concurrent starts cannot both win.
@row_none n=3 rec=6 type=4
relied: A frame verb against a stale instance reads instance-dead, since no live endpoint is given for a stale wrapper.
@row_add n=35 row=decision/frame-client-verifies-server area=established-decisions rec=7,25
name: Frame clients verify the serving wrapper
rule: Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
@row_none n=4 rec=8 type=4
relied: Without a live endpoint a client reads an existing instance directory as instance-dead and anything else as an unwrapped name.
@row_none n=5 rec=9 type=4
relied: Over a replayed snapshot the start's collision check decides from the heartbeat alone, refusing already-live within the stale bound.
@row_none n=6 rec=10 type=4
relied: A revive makes one snapshot replay and logs one state-recovered line.
@row_none n=7 rec=11 type=4
relied: Every start refuses a strict-modes home-tree failure with exit 1 before any log, line or child.
@row_none n=8 rec=12 type=4
relied: A failed socket-dir create and a refused check both read socket-dir-refused with exit 1, and a bind failure does not.
@row_add n=36 row=decision/thiserror-one-enum-per-crate area=established-decisions rec=13
name: thiserror, one error enum per workspace crate
rule: Each workspace crate has one thiserror error enum, and the refusal reason type lives in the core crate.
why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError.
except: Ratified exceptions: viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
@row_add n=37 row=decision/pty-error-hand-written area=established-decisions rec=13
name: Hand-written PTY error type
rule: The PTY crate's error has a hand-written Display and Error with fixed messages, keeping the portable-pty anyhow error only as a boxed source.
why: The seam's normal dependencies are exactly portable-pty, tracing and libc; an exception to the thiserror rule.
@row_add n=38 row=decision/anyhow-only-in-bin area=established-decisions rec=13
name: anyhow only in the viola binary
rule: anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter.
why: anyhow's context chains are only useful where a person reads them.
@row_add n=39 row=decision/error-chain-to-diagnostics-only area=established-decisions rec=13
name: Error chains reach only the post-mortem detail file
rule: An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr.
why: The owner's post-mortem is the one place a person reads the chain.
@row_none n=9 rec=14 type=4
relied: Program resolution has one refusal, ProgramNotFound, with a fixed message.
@row_none n=10 rec=15 type=5
@row_add n=40 row=decision/cargo-workspace-flat-crates area=established-decisions rec=16
name: Cargo workspace with flat crates
rule: The project is a Cargo workspace with a flat crates layout, the root manifest being both the viola bin package and the workspace; each product crate is created by the first piece of work that consumes it.
why: The compiler enforces that viola-pty cannot see viola-agent-claude; cargo install --path . requires the bin at the root.
@row_add n=41 row=decision/fuzz-own-workspace area=established-decisions rec=16
name: Fuzz targets in their own workspace
rule: The cargo-fuzz targets are their own cargo workspace, excluded from the root workspace.
why: So their nightly toolchain and C++-building libfuzzer-sys never enter the root graph.
@row_add n=42 row=decision/tokio-only-in-mcp-crate area=established-decisions rec=16
name: tokio only in the MCP crate
rule: Among the product crates only viola-mcp enables tokio; viola-channel lists it optional behind its tokio feature, and no other product member's normal graph reaches tokio.
guard: scripts/channel-tokio-free.sh
@row_add n=43 row=decision/no-cargo-pup area=established-decisions rec=16
name: cargo_pup not taken
rule: Module boundaries are enforced by crate separation, not by cargo_pup.
why: cargo_pup needs nightly and was rejected.
@row_add n=44 row=decision/plugin-compiled-into-binary area=established-decisions rec=17
name: Plugin compiled into the binary
rule: viola installs by cargo install --path . with the plugin's hooks, MCP and manifest files embedded in the binary and the plugin version taken from the package version; the plugin files are rewritten from the binary on every start.
why: The binary and the plugin can no longer drift apart.
@row_add n=45 row=decision/pinned-exe-copy area=established-decisions rec=17
name: Pinned exe copy keyed by version and content hash
rule: A start runs its session against a pinned copy of its own exe keyed by package version and short content hash, so a rebuild never changes what a running session calls.
why: The package version stays the same across development rebuilds, so the version alone would collide. Trade-off: a hook fix applies only to sessions started after the rebuild.
@row_add n=46 row=decision/pinned-copy-rehash area=established-decisions rec=17
name: Pinned copy re-hashed before reuse
rule: An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
@row_add n=47 row=decision/hooks-and-mcp-never-search-path area=established-decisions rec=17
name: Hooks and MCP never search PATH
rule: Hook and MCP commands are the pinned copy's absolute path substituted at start, never a PATH lookup.
why: PATH-found hooks run a different binary from the wrapper after a rebuild.
@row_add n=48 row=decision/frame-version-safety-net area=established-decisions rec=17
name: Per-frame version as second safety net
rule: Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
@row_add n=49 row=contract/plugin-mcp-json/viola area=established-decisions rec=17
boundary: The plugin's .mcp.json
item: viola
promise: Registers the server viola with command the pinned copy's absolute path and args ["mcp"].
@row_none n=11 rec=18 type=4
relied: A start pins the very bytes it read once and keyed for its home-tree check.
@row_none n=12 rec=19 type=0
what: a measured note that one source tree can build two different binaries, recorded as known with no action
@row_add n=50 row=decision/plugin-only-in-wrapped-sessions area=established-decisions rec=20
name: Plugin loads only into wrapped sessions
rule: The viola plugin loads only into sessions started by viola run, through --plugin-dir; both driver and driven run wrapped.
why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola.
except: viola verify's transient unwrapped children, all gone when verify ends.
@row_add n=51 row=decision/installed-plugin-mcp-only area=established-decisions rec=20
name: Installed plugin carries MCP only
rule: viola plugin install is optional, for the MCP verbs in an unwrapped driver; the installed plugin carries only .mcp.json and no hooks, its command the absolute path of the installing exe.
why: So a wrapped session that also has it installed never runs two hook processes for one event.
@row_add n=52 row=decision/plugin-precedence-no-ledger-row area=established-decisions rec=20
name: Plugin precedence is no ledger row
rule: Plugin precedence between the installed and the --plugin-dir plugin is not a capability-ledger row.
why: No send relies on it, the ruled limit.
@row_add n=53 row=decision/verify-transcripts-accepted-residual area=established-decisions rec=20
name: Verify's CLI transcripts are an accepted residual
rule: The CLI session transcripts and empty project dirs viola verify's runs leave under the user's Claude projects directory are an accepted residual.
@row_add n=54 row=decision/cli-human-text-json-exit-codes area=established-decisions rec=21
name: Human text by default, --json for agents
rule: CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged.
why: Agents need machine-parseable refusals.
@row_add n=55 row=decision/prompt-text-from-stdin-or-file area=established-decisions rec=21
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or --file, never from a leading-slash argument.
@row_add n=56 row=contract/viola-revive-json/error-document area=established-decisions rec=21
boundary: viola revive --json
item: error document
promise: Every refusal of the start prints {"v":1,"error":"<code>","detail":null} in place of its unable/hint pair, exit 1; a pump_child error prints no document.
@row_add n=57 row=contract/viola-revive-json/list-document area=established-decisions rec=21
boundary: viola revive --json
item: --list document
promise: <name> --list --json prints {"v":1,"ok":{"sessions":[…]}} in log order.
@row_add n=58 row=contract/viola-revive-json-error-codes/strict-modes-failed area=established-decisions rec=21
boundary: viola revive --json error codes
item: strict-modes-failed
promise: A strict-modes check refused the start.
@row_add n=59 row=contract/viola-revive-json-error-codes/already-live area=established-decisions rec=21
boundary: viola revive --json error codes
item: already-live
promise: The name is held by a live instance.
@row_add n=60 row=contract/viola-revive-json-error-codes/no-session area=established-decisions rec=21
boundary: viola revive --json error codes
item: no-session
promise: No logged session to revive.
@row_add n=61 row=contract/viola-revive-json-error-codes/session-live area=established-decisions rec=21
boundary: viola revive --json error codes
item: session-live
promise: The logged session is held by a live claude.
@row_add n=62 row=contract/viola-revive-json-error-codes/cwd-missing area=established-decisions rec=21
boundary: viola revive --json error codes
item: cwd-missing
promise: The recorded directory is missing.
@row_add n=63 row=contract/viola-revive-json-error-codes/stale-heartbeat area=established-decisions rec=21
boundary: viola revive --json error codes
item: stale-heartbeat
promise: The name is held by a stale instance.
@row_add n=64 row=contract/viola-revive-json-error-codes/pinned-hash-mismatch area=established-decisions rec=21
boundary: viola revive --json error codes
item: pinned-hash-mismatch
promise: The pinned copy no longer matches its hash.
@row_add n=65 row=contract/viola-revive-json-error-codes/squatted-name area=established-decisions rec=21
boundary: viola revive --json error codes
item: squatted-name
promise: The endpoint name is already taken at bind.
@row_add n=66 row=contract/viola-revive-json-error-codes/socket-dir-refused area=established-decisions rec=21
boundary: viola revive --json error codes
item: socket-dir-refused
promise: The socket directory was refused or could not be created.
@row_add n=67 row=contract/viola-revive-json-error-codes/internal-error area=established-decisions rec=21
boundary: viola revive --json error codes
item: internal-error
promise: A claude not on PATH or an anyhow error of the start.
@row_add n=68 row=decision/linux-only area=established-decisions rec=22
name: Linux only
rule: viola runs on Linux only: the core on any current Linux distribution, CI on ubuntu-latest; no Windows or macOS code, test, fixture or CI job stands in the tree.
@row_add n=69 row=absence/windows-and-macos-support area=established-decisions rec=22
capability: Support for Windows or macOS
@row_add n=70 row=decision/window-features-target-hyprland area=established-decisions rec=22
name: Window features target Hyprland
rule: The window features target the Hyprland compositor.
@row_add n=71 row=decision/socket-in-per-user-dir area=occupied-resources rec=23
name: Endpoint socket in a per-user 0700 directory
rule: The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found.
@row_add n=72 row=decision/no-abstract-namespace-sockets area=occupied-resources rec=23
name: No abstract-namespace sockets
rule: Linux abstract-namespace socket names are never used.
why: They carry no filesystem permissions and any local user could connect.
@row_add n=73 row=decision/connect-to-recorded-endpoint area=occupied-resources rec=23
name: Clients connect to the recorded endpoint
rule: The wrapper resolves the endpoint path once and records it in the snapshot; every other process connects to that recorded endpoint and none rebuilds the path.
why: Processes spawned inside the session may run with a different environment from the wrapper.
@row_add n=74 row=decision/stale-socket-file-left area=occupied-resources rec=24
name: Stale socket file left in place
rule: A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it.
why: A choice: a harmless file in a 0700 directory.
@row_add n=75 row=contract/cli-exit-codes/21 area=occupied-resources rec=25
boundary: CLI exit codes
item: 21
promise: instance-unreachable: the process serving the endpoint could not be verified as the snapshot's wrapper, or a check could not be made; no frame was sent.
