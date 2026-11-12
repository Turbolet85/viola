@row_add n=1 row=overview/input-validation area=input-validation rec=1
text: Viola validates untrusted data at every point where it enters. Two validation regimes apply: tolerant parsing for external payloads and versioned-tolerant parsing for viola's own formats, with the security floors holding in both. Shared validators live in one core crate, so the CLI, the MCP server and the wrapper all call the same function.
@row_add n=2 row=decision/shared-validators-in-core area=input-validation rec=1
name: Shared validators in the core crate
rule: Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one.
why: so the CLI, MCP and the wrapper apply one identical check
@row_add n=3 row=decision/paste-text-character-set area=input-validation rec=2
name: Paste text character set
rule: Paste text allows LF, CR and TAB and refuses every other C0 control, DEL and every C1 character, checked over decoded characters.
@row_add n=4 row=decision/refused-text-never-stripped area=input-validation rec=2
name: Refused text is refused whole
rule: A text holding a refused character is refused whole, never stripped, and the typed text is the only text a send uses.
@row_add n=5 row=contract/send-refusal/empty-text area=input-validation rec=2
boundary: send refusal words
item: empty-text
promise: An empty paste text is refused with the word empty-text.
@row_add n=6 row=decision/wrapper-checks-authoritatively area=input-validation rec=2,3
name: Wrapper re-checks text authoritatively
rule: Paste text and dialog free text are checked client-side first and again, authoritatively, by the wrapper before anything is typed or answered.
@row_add n=7 row=decision/dialog-free-text-character-set area=input-validation rec=3,2
name: Dialog free text uses the paste text set
rule: Dialog free text (permission and plan messages, question answer values) is held to the same exact character set as paste text.
@row_add n=8 row=contract/answer-refusal/control-character area=input-validation rec=3
boundary: answer refusal words
item: control-character
promise: Dialog free text holding a refused character is refused with the word control-character before any frame is sent.
@row_add n=9 row=decision/channel-frame-cap-before-read area=input-validation rec=4
name: Channel frame cap before reading
rule: A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
@row_add n=10 row=decision/json-default-depth-limit area=input-validation rec=4
name: Default JSON depth limit
rule: Channel frames are parsed with serde_json's default depth limit, never with unbounded depth.
@row_add n=11 row=contract/channel-errors/32600 area=input-validation rec=4
boundary: channel error codes
item: -32600
promise: A frame over the MAX_FRAME cap is answered -32600 and the connection is closed.
@row_add n=12 row=contract/channel-errors/32602 area=input-validation rec=4
boundary: channel error codes
item: -32602
promise: A method whose params fail the wrapper's check is answered -32602.
@row_add n=13 row=decision/invalid-hook-frames-dropped area=input-validation rec=4
name: Invalid hook frames dropped
rule: An invalid hook.dialog frame is not registered and an invalid hook.event frame is not appended.
@row_add n=14 row=decision/hook-stdin-never-truncated area=input-validation rec=5
name: Hook stdin over-limit detected
rule: Hook stdin is capped at MAX_FRAME and an over-limit payload is detected, never silently truncated.
@row_add n=15 row=decision/hook-security-fields-closed area=input-validation rec=5
name: Hook security fields in closed enums
rule: Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
@row_add n=16 row=decision/hooks-fail-open area=input-validation rec=5
name: Hooks fail open
rule: Every hook failure fails open: exit 0 with no body.
@row_add n=17 row=decision/budget-file-single-writer area=input-validation rec=5
name: Budget file has one writer
rule: The budget file is written by the statusline hook alone.
@row_add n=18 row=decision/capture-arms-isolated area=input-validation rec=5
name: Hidden capture arms isolated
rule: The hidden capture arms read no VIOLA_* variable, open no channel and always exit 0.
@row_add n=19 row=decision/mcp-handlers-check-raw-args area=input-validation rec=6
name: MCP handlers check raw arguments
rule: Each MCP tool handler checks its raw arguments itself; the published schemas are advisory only.
why: rmcp does not enforce the schemars schemas
@row_add n=20 row=contract/mcp-errors/invalid-params area=input-validation rec=6
boundary: MCP tool errors
item: invalid-params
promise: A tool input that fails its check returns invalid-params with a closed field word, and no frame is sent.
@row_add n=21 row=decision/mcp-stdin-line-cap area=input-validation rec=7
name: MCP stdin line cap
rule: Each MCP stdin line is capped at MAX_FRAME, its newline counted; a longer line is refused whole, never truncated or skipped, and ends the session like a stdin close.
why: rmcp reads stdin lines unbounded, so viola hands it a capped reader
@row_add n=22 row=contract/mcp-stdin/parse-rejected area=input-validation rec=7
boundary: MCP stdin refusal log
item: parse-rejected
promise: An over-long MCP stdin line is recorded as one parse-rejected line holding no byte of it.
@row_add n=23 row=decision/names-through-validated-type area=input-validation rec=8
name: Names through the validated name type
rule: Every instance name taken from CLI arguments passes the validated name constructor.
@row_add n=24 row=decision/home-strict-modes-first area=input-validation rec=8,13
name: Home strict-modes check first
rule: The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
@row_add n=25 row=decision/stamps-checked-under-lock area=input-validation rec=8
name: Stamps checked before and under lock
rule: The ledger stamps file is checked before the ledger directory is created and again under its lock.
@row_add n=26 row=decision/cli-input-checked-before-frame area=input-validation rec=8
name: CLI input checked before any frame
rule: answer and send input is read under MAX_FRAME and parsed into closed enums or through the paste text validator before any frame is sent.
@row_add n=27 row=contract/revive-json/output-fields area=input-validation rec=8
boundary: revive --json output
item: output fields
promise: revive --json prints codes, closed words and session-shaped ids only, never a path or pid.
@row_add n=28 row=decision/config-bad-value-handling area=input-validation rec=9
name: Bad config values
rule: Configuration is parsed tolerantly under MAX_FRAME with its version checked; a bad value takes its key's default or is rejected whole, and is reported as parse-rejected.
@row_add n=29 row=decision/config-cannot-disable-controls area=input-validation rec=9
name: Configuration cannot disable controls
rule: No configuration value switches the budget gate off or makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
@row_add n=30 row=decision/test-seams-compiled-out area=input-validation rec=10,11
name: Test seams compiled out of release
rule: Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
@row_add n=31 row=decision/colour-env-only-compared area=input-validation rec=12
name: Terminal colour variables only compared
rule: NO_COLOR, TERM and COLORTERM are only compared, never echoed or set, decide only whether and how deep SGR bytes are written, and reach no log line, error or refusal.
@row_add n=32 row=decision/state-readers-never-write area=input-validation rec=13
name: State file readers never write
rule: A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
@row_add n=33 row=decision/replayed-snapshot-inert area=input-validation rec=13
name: Replayed snapshot is inert
rule: A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid, start time or statusline command, so no frame is written and no command runs from it.
@row_add n=34 row=decision/statusline-command-trust area=input-validation rec=13
name: Statusline command trusted via home check
rule: The recorded statusline command is trusted only because the home passes the strict-modes check.
why: write access to the recorded command is code execution
@row_add n=35 row=decision/failed-budget-read-no-reading area=input-validation rec=13
name: Failed budget read is no reading
rule: Every failed read of the budget file counts as no reading.
@row_add n=36 row=decision/tail-only-valid-instance-logs area=input-validation rec=14
name: Tail only valid instance logs
rule: The notifier tails only an instance event log whose name passes the validated name check, under the canonical home, ignoring symlinks and names that fail parsing.
@row_add n=37 row=decision/child-output-bounded area=input-validation rec=15
name: Child output bounded and tolerant
rule: Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
@row_add n=38 row=decision/verify-reads-only-signatures area=input-validation rec=15
name: Verify reads PTY output by signatures
rule: verify's PTY output is read only against the compiled signature literals.
@row_add n=39 row=decision/unwrapped-name-never-target area=input-validation rec=15
name: Unwrapped agent names are no targets
rule: An unwrapped agent row's name is never a validated viola name and never a valid target.
@row_add n=40 row=decision/vt100-feed-isolated area=input-validation rec=16
name: Screen model feed isolated from passthrough
rule: PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
@row_add n=41 row=decision/claude-resolved-from-path area=input-validation rec=17
name: claude resolved from PATH
rule: The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded.
why: a same-user trust and accepted risk
@row_add n=42 row=decision/revive-cwd-spawn-only area=input-validation rec=17
name: Recorded cwd used only to spawn
rule: The recorded working directory of a revive is read after the strict-modes check and used only as the spawn directory.
@row_add n=43 row=decision/session-id-argv-no-shell area=input-validation rec=18
name: Session id as one argv element
rule: The session id after --resume has the closed session-id shape and is one argv element of a direct spawn, never passed through a shell.
@row_add n=44 row=contract/revive-refusal/no-session area=input-validation rec=18
boundary: revive refusal words
item: no-session
promise: An --id the log does not hold is refused no-session before any spawn, and no refusal line or hint holds the recorded directory, a pid or a logged id.
@row_add n=45 row=decision/statusline-source-read-only area=input-validation rec=19
name: Statusline source read only
rule: A statusline command is taken only from a non-empty statusLine of type command with no NUL, read once and capped; viola writes neither source.
@row_add n=46 row=decision/single-shell-out area=input-validation rec=20
name: Statusline hook is the only shell-out
rule: The recorded statusline command runs only from the statusline hook via /bin/sh -c as one argument, its output capped and printed unparsed, its wait bounded, and it is the only shell-out.
@row_add n=47 row=decision/no-statusline-command-in-logs area=input-validation rec=20
name: Statusline command never logged
rule: No log line or span carries the statusline command.
@row_add n=48 row=absence/file-upload area=input-validation rec=21
capability: File upload
@row_add n=49 row=absence/webhook area=input-validation rec=21
capability: Webhooks
@row_add n=50 row=absence/database area=input-validation rec=21,33
capability: A database
@row_add n=51 row=decision/parser-fuzz-scope area=input-validation rec=22
name: Parser surfaces in fuzz scope
rule: Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser and the vt100 feed are in scope for fuzz and property coverage.
@row_add n=52 row=decision/max-frame-cap area=input-validation rec=23
name: One external read cap
rule: MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read.
except: viola's reads of its own executable are no external input
@row_add n=53 row=decision/max-frame-revisit-signal area=input-validation rec=23
name: Frame cap revisit signal
rule: A stamped largest hook payload within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
@row_none n=1 rec=24 type=S
@row_none n=2 rec=25 type=S
@row_add n=54 row=decision/data-protection-minimal-tier area=data-protection rec=26
name: Data protection at Minimal tier, adapted
rule: Data protection follows the Minimal tier with file-permission controls kept in scope.
why: viola stores sensitive user content; HTTPS enforcement does not apply
@row_none n=3 rec=27 type=S
@row_add n=55 row=decision/no-tls-local-transport area=data-protection rec=28
name: No TLS on local transports
rule: No TLS is used: the IPC channel is local Unix sockets and MCP is stdio, so confidentiality in transit rests on the socket-directory and peer checks.
@row_none n=4 rec=29 type=0
what: a not-applicable control line for HSTS
@row_none n=5 rec=30 type=0
what: a not-applicable control line for certificate pinning
@row_add n=56 row=decision/no-handle-inheritance area=data-protection rec=31
name: No channel handle inheritance
rule: Channel handles never leak into the claude child: every inherited descriptor is closed in the child.
@row_none n=6 rec=32 type=S
@row_add n=57 row=decision/state-files-os-permissions area=data-protection rec=34
name: State files protected by OS permissions
rule: State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
@row_add n=58 row=decision/settings-override-plain-path area=data-protection rec=34
name: Settings override only for plain paths
rule: The settings override is written only for a pinned path of plain characters.
why: a shell runs the override unquoted
@row_add n=59 row=decision/plugin-files-rewritten-each-start area=data-protection rec=34
name: Plugin files rewritten every start
rule: The plugin files and the settings override are rewritten at every start.
@row_add n=60 row=decision/pinned-binary-rehashed area=data-protection rec=34
name: Pinned binary re-hashed before use
rule: The pinned viola binary is re-hashed before use.
@row_add n=61 row=decision/probe-artifacts-cleanup area=data-protection rec=35
name: Probe artifacts owner-only and removed
rule: Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing.
except: a kill or a crash leaves them to be removed by hand
@row_add n=62 row=decision/logs-owner-only area=data-protection rec=36
name: Logs owner-only
rule: The event log and diagnostics files are 0600 inside 0700 directories.
@row_add n=63 row=decision/drift-reports-diagnostics-only area=data-protection rec=36
name: Drift reports only in diagnostics
rule: Deserialisation drift reports are written only to the diagnostics directory.
why: drift reports can quote tool input
@row_add n=64 row=decision/fixture-recording-scrubbed area=data-protection rec=37
name: Fixture recordings scrubbed
rule: verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
@row_add n=65 row=decision/fixtures-reviewed-before-commit area=data-protection rec=37
name: Fixtures reviewed before commit
rule: Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
@row_add n=66 row=decision/no-keys-at-rest area=data-protection rec=38
name: No keys at rest
rule: Key management is omitted for the Minimal tier: there are no keys at rest.
@row_add n=67 row=decision/log-rotation-keeps-permissions area=data-protection rec=38
name: Log rotation keeps permissions
rule: Any rotation scheme for the event log must keep 0600/0700 permissions and valid offsets.
@row_add n=68 row=decision/verify-trusted-runs-risk area=data-protection rec=39
name: Verify trusted runs accepted risk
rule: verify's trusted runs B, C and D are real interactive turns whose transcripts the CLI persists and for which the user's global hooks run.
why: accepted risk
@row_add n=69 row=decision/viola-writes-nothing-in-claude-dir area=data-protection rec=39,41
name: viola writes nothing in Claude's config
rule: viola itself writes nothing under the user's Claude Code config directory or its global config file.
@row_add n=70 row=decision/scratch-footer-reading-residual area=data-protection rec=40
name: Scratch footer reading is no standing permission
rule: A scratch footer reading of a newer CLI is one tmpfs-home run start, no key typed, only scrubbed footer rows kept; it is no product path, and another such reading needs its own answer.
why: accepted residual
@row_add n=71 row=decision/live-plugin-install-round area=data-protection rec=41
name: Live plugin install round
rule: A live plugin install round runs the CLI's four plugin commands at local scope outside the work tree, passing folder trust by keys into its own pty only, never a desktop key.
why: accepted residual
@row_add n=72 row=decision/verify-never-answers-import area=data-protection rec=42
name: Verify needs a pre-trusted folder
rule: verify against the real claude runs from a user-trusted folder whose external CLAUDE.md import is already answered; verify never answers it.
@row_add n=73 row=decision/event-log-growth-risk area=data-protection rec=43
name: Event log growth accepted
rule: Unbounded growth of the event log may exhaust local disk until retention is resolved.
why: accepted risk: availability impact only, no confidentiality or integrity impact
@row_none n=7 rec=44 type=S
@row_none n=8 rec=45 type=S
@row_none n=9 rec=46 type=S
@row_none n=10 rec=47 type=S
@row_add n=74 row=decision/rmcp-feature-set area=api-security rec=48
name: rmcp feature set
rule: The release graph enables rmcp features exactly server and transport-io; the test-only e2e crate takes client only, never transport-child-process.
why: all five 2026 rmcp advisories sit in unused features
guard: scripts/rmcp-features.sh in CI's lint job
@row_add n=75 row=decision/rmcp-banned-features area=api-security rec=48
name: rmcp HTTP and auth features banned
rule: rmcp's transport-streamable-http-server and auth features are banned.
why: all five 2026 rmcp advisories sit in unused features
guard: cargo-deny bans.features
@row_none n=11 rec=49 type=S
@row_none n=12 rec=50 type=S
@row_none n=13 rec=51 type=S
@row_none n=14 rec=52 type=S
@row_add n=76 row=decision/cargo-deny-audit area=dependency-security rec=53,52
name: cargo-deny as audit tool
rule: cargo-deny 0.20.2 or newer audits advisories, bans, licences and sources.
@row_add n=77 row=decision/zizmor-workflow-audit area=dependency-security rec=54,52
name: zizmor for workflows
rule: zizmor 1.30.1 or newer audits the GitHub Actions workflows.
@row_add n=78 row=decision/cargo-audit-not-gated area=dependency-security rec=55
name: cargo-audit not gated
rule: cargo-audit is not gated; it is reserved for binary scans of cargo-auditable release binaries.
why: it would duplicate cargo-deny advisories on the same lockfile
