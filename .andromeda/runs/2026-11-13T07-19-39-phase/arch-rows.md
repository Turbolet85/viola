# architecture · the rows that bind 2026-11-13-epoch-8-cleanup — picked 349 of 987 (193 by every draw) · rule any · pin 885

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.

## stack-and-technologies · ## Stack and Technologies
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/pinned-copy-placed-once · Pinned copy placed once, never replaced — The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
decision/pinned-copy-rehash-before-reuse · Pinned copy re-hashed before every reuse — An existing pinned copy is re-hashed before every reuse.
decision/syn-test-only · syn and proc-macro2 test-only — syn and proc-macro2 are direct dependencies of the test-only E2E crate alone; no product crate depends on either.
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
decision/append-one-write-per-line · Log appends one write per line — Log appends use std append mode with one write per line. · why: crash-safe multi-process writes with no C code
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/clients-read-server-pid-euid-from-peer-creds · Clients read server pid and euid from peer credentials — Both channel clients, sync and async, read the serving process's pid and effective uid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame. · why: The CLI, hook and MCP server checks admit the euid and match the pid against the snapshot's recorded pid ahead of any frame.
decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
decision/content-hash-sha256 · Pure-Rust SHA-256 content hash — The bin/ and plugin/ version-hash keys are the exe's SHA-256 truncated to 8 bytes (16 hex), through sha2 with no cc.
decision/e2e-no-rmcp-child-transport · E2E owns the viola mcp child — The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit. · why: transport-child-process kills the server 3 s after a close
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/mcp-server-rmcp · rmcp is the MCP server — The MCP server is an rmcp stdio server with schemars tool input schemas, rmcp minor pinned, a thin adapter over viola-channel.
decision/no-database · No database: ndjson logs and JSON snapshots — State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
decision/pre-push-filter-not-verdict · Local pre-push gate is a filter, not the verdict — The local pre-push gate runs the ubuntu test job's suites natively in the working tree under a cleared environment, refuses off Linux and installs nothing; it is a pre-push filter, never the verdict of record.
decision/server-start-time-read-through-sysinfo · Server start time is read from the OS through sysinfo — The server check reads the serving pid's start time through sysinfo, with a zombie reading as gone, and matches it in whole seconds against the snapshot's recorded start time before the first frame. · why: The CLI, hook and MCP clients all verify the server this way, so a reused pid with another start time reads as a mismatch.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key

## established-decisions · ## Established Decisions
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
decision/dialog-hook-fails-open-on-server-check · Dialog hook fails open on a failed server check — A dialog hook whose server check fails sends no hook.dialog and exits 0 at once with no decision body, so the dialog renders for the human. · why: The fail-open hook contract and the server-identity negative tests rely on it, so a process that is not the recorded wrapper never answers a dialog.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/frame-client-verifies-server · Frame clients verify the serving wrapper — Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
decision/hook-never-writes-stderr · Hook logs only to diagnostics — A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
decision/hook-without-trusted-instance-exits-silently · Hook with no trusted instance exits 0 untouched — A hook whose instance check refuses or finds no wrapped instance exits 0 before reading config or stdin, and writes no line, frame or file. · why: The fail-open hook contract and the hook strict-modes tests rely on it, and the check must come before the log open that narrows the home.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arms, called only by `viola verify`'s probes, read no `VIOLA_*` and open no channel, still exiting 0 with empty stderr.
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/link-recording · How a driver-to-driven link is recorded — A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`; `viola unlink` removes it.
decision/links-are-log-events · Links are events in the log — Links are events in the log, and the snapshot's link set is derived by replaying them. · why: The log stays the single source of truth for the link set.
decision/links-recorded-as-log-events · Links recorded as log events — A link `driver → driven` is recorded when a `send` or `answer` comes from inside a wrapped session to another instance, or by `viola link`, and removed by `viola unlink`; the snapshot's link set is derived by replaying those events. · why: The log stays the single source of truth for the link set as for everything else.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/lost-endpoint-loses-event · A lost endpoint loses the event, not the turn — When a hook cannot reach its endpoint, the hook event is lost and the turn goes on.
decision/mcp-frame-tools-strict-modes-check · MCP frame tools refuse a strict-modes failure — Every MCP frame tool refuses a home or target instance directory that fails the strict-modes check with the instance-unreachable result and detail strict-modes-failed, sending no frame. · why: The handlers test expects the refusal before any snapshot read or frame, so an untrusted home never steers a frame to a wrapper.
decision/mcp-stdin-bound-in-own-reader · MCP stdin bound in viola's own reader — The MCP stdin line bound sits in the reader viola hands rmcp, never in rmcp's codec, and a cut reads as end of stream, never as an I/O error. · why: rmcp's stdio transport reads each line unbounded and its codec bounds only the writer side; end of stream keeps rmcp from writing anything of its own.
decision/mcp-stdin-line-cap · MCP stdin read under a line cap — The MCP server's stdin is read under a `MAX_FRAME` line cap, newline counted; a longer line is refused whole, never truncated or skipped, and ends the session with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it. · why: rmcp's stdio transport reads each line unbounded, so the bound sits in the reader viola hands rmcp.
decision/mcp-stdin-oversize-refused-whole · Oversize MCP stdin line refused whole — An MCP stdin line longer than the frame cap reaches rmcp as no byte and ends the session as at a closed stdin; the line is never truncated and never skipped. · why: A truncated line could dispatch a padded valid request, and a skipped one leaves the peer unanswered with no signal; follows the wrapper channel's close-on-oversize rule.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/pinned-copy-never-replaced · Pinned copy never replaced — The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
decision/pinned-copy-no-clobber-placement · Pinned copy placed no-clobber with the handle closed — The pinned copy's absent-copy write is never a replace: a no-clobber placement that closes the write handle before the rename and leaves an existing copy untouched. · why: A rename with the write handle still open makes a concurrent first start's exec of the copy fail ETXTBSY.
decision/pinned-copy-rehash · Pinned copy re-hashed before reuse — An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
decision/pinned-exe-copy · Pinned exe copy keyed by version and content hash — At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls. · why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
decision/spine-hook-loses-event-not-turn · Spine hook loses the event, never delays the turn — A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost. · why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/verify-server-before-first-frame · Server verified before the first frame — Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison. · why: A squatted or rogue server is never sent a frame.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
contract/channel-link/-32603 · Wrapper channel link recording · -32603 — A link that cannot be recorded is `-32603`, with nothing issued or the answer not taken.
contract/event-line/link · The event line · link — The driven wrapper appends a `link` line, `source: wrapper`, only when the pair is not already in its link set; a refused `send` or `answer` and a `from` equal to the wrapper's own name link nothing.
contract/hook-diagnostics/server-verify-failed · Hook diagnostics · server-verify-failed — A hook whose serving process has another euid, is not the snapshot's pid and start time, or cannot be checked sends no frame, exits 0 with no body and records `server-verify-failed`.
contract/hook-diagnostics/socket-dir-refused · Hook diagnostics · socket-dir-refused — A hook meeting a refused socket directory sends no frame, exits 0 with empty streams and records one `hook-decision` at WARN with `detail` `socket-dir-refused`.
contract/hook-exit/0-no-body · Hook exit and body · exit 0 with no body — No decision: the dialog renders for the human.
contract/hook-exit/0-with-body · Hook exit and body · exit 0 with a JSON body — A decision.
contract/hook-exit/2 · Hook exit and body · exit 2 — Never produced.
contract/mcp-stdin-oversize/parse-rejected · MCP stdin oversize process log · parse-rejected — A session ended by an oversize line writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line at WARN, holding no byte of the line.
contract/viola-revive-json-error-codes/strict-modes-failed · viola revive --json error codes · strict-modes-failed — A strict-modes check refused the start.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
contract/viola-verify-refusal-codes/pinned-hash-mismatch · viola verify process-exit refusal codes · pinned-hash-mismatch — Written on the process-exit verify logs under a parsed VIOLA_NAME when the pinned copy's hash does not match.
contract/viola-verify-refusal-codes/strict-modes-failed · viola verify process-exit refusal codes · strict-modes-failed — Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.
decision/append-heals-torn-line · The next append heals a torn line — Each event line is written in one write call; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/exit-plan-mode-allow-with-updated-input · ExitPlanMode answered only through PreToolUse — An ExitPlanMode plan is approved only through PreToolUse as allow with updatedInput set to the tool's own input unchanged (a non-object input gives {}), and revised through deny with a message. · why: A bare allow left the plan dialog up on the live CLI.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/frame-version-safety-net · Per-frame version as second safety net — Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
decision/hand-rolled-json-rpc-over-ndjson · JSON-RPC 2.0 over ndjson, hand-rolled with serde — The channel speaks a JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde; requests carry an `id`, hook events are notifications without one. · why: The same shape as MCP makes `mcp` a thin adapter; jsonrpsee is Tokio-bound and would drag a runtime into `hook`.
decision/hook-connect-deadline · Hook connect deadline — The connect deadline a hook passes to the channel client is 750 ms, kept below the spine deadline. · guard: a unit test asserting it below the spine deadline · provisional
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/self-describing-frames-no-handshake · Self-describing frames, no handshake — Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake. · why: Mixed binary versions are a measured fact, so every frame self-describes.
decision/session-end-budget-never-raised · SessionEnd budget never raised — The 1.5 s SessionEnd budget is never raised.
decision/spine-deadline · Spine deadline of 1 s — Every ordering-spine hook finishes within one built-in spine deadline of 1 s, the same for every event. · guard: the perf gate (test-plan §10)
decision/start-pins-the-bytes-it-checked · Start pins the very bytes it checked — A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
decision/sync-core-tokio-only-in-mcp · Hybrid sync core, Tokio only in mcp — The core is synchronous, and Tokio is confined to the `mcp` subcommand; among the product crates only `viola-mcp` enables `tokio`. · why: portable-pty's handles are blocking, so the pump needs threads either way; `hook` must not pay for runtime startup; rmcp and interprocess-async require Tokio. · guard: sole-root `cargo deny` `tokio` ban (`deny-sync.toml`) and a `cargo check` of the sync crates on `ubuntu-latest` · except: `viola-channel`'s optional `tokio` feature, enabled by `viola-mcp`; its graph without the feature is proven tokio-free.
decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json`.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/torn-last-line-healed · Torn log line healed by the next append — Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
contract/claude-cli-version-output/first-line · Claude CLI --version output · first line — Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
contract/event-line/unlink · The event line · unlink — `viola unlink` appends an `unlink` line with `source: wrapper` and `{driver, driven}`.
contract/hook-tiers/async · Hook tiers · async — Notification, PostToolUse and PostToolUseFailure run with `async: true` and print nothing.
contract/hook-tiers/ordering-spine · Hook tiers · sync with no reply body (the ordering spine) — SessionStart, UserPromptSubmit and Stop each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
contract/hook-tiers/session-end · Hook tiers · SessionEnd — Sync within about 1 s: the channel, else a direct try-lock append of the event, else nothing; `hooks.json` gives it no `timeout` key.
contract/hook-tiers/sync-spine · hook tiers · sync spine — SessionStart, UserPromptSubmit and Stop run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
contract/hook-tiers/sync-with-decision · Hook tiers · sync with a decision — PreToolUse with matcher `AskUserQuestion|ExitPlanMode`, and PermissionRequest, run sync and may reply with a decision.
contract/process-log/link-channel-lines · Wrapper process log · link and unlink channel lines — The `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name.
contract/viola-mcp-exit-codes/0-oversize-after-initialize · viola mcp exit codes · 0 after an oversize line once initialized — An oversize line after `initialize` completed ends `viola mcp` with exit 0, as at a closed stdin.
contract/viola-mcp-exit-codes/1-oversize-before-initialize · viola mcp exit codes · 1 on an oversize line before initialize — An oversize line before `initialize` completes ends `viola mcp` with exit 1 and both streams empty.
contract/viola-revive-json-error-codes/pinned-hash-mismatch · viola revive --json error codes · pinned-hash-mismatch — The pinned copy no longer matches its hash.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail.
contract/viola-verify-refusal-codes/interrupted · viola verify process-exit refusal codes · interrupted — Written on the process-exit verify logs under a parsed VIOLA_NAME when verify is interrupted.
contract/viola-verify-refusal-codes/program-not-found · viola verify process-exit refusal codes · program-not-found — Written on the process-exit verify logs under a parsed VIOLA_NAME when the program is not found.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
contract/viola-verify-refusal-codes/version-unreadable · viola verify process-exit refusal codes · version-unreadable — Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.

## conventions · ## Conventions
decision/fake-agent-test-seams · Fake-agent test seam variables — The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds. · why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
decision/silent-roles-at-internal-error-catch · Run, revive, mcp and hook silent at the catch site — Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
contract/channel-methods/hook-dialog · channel methods · hook.dialog — A request with an integer JSON-RPC id, monotonic per connection; `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
contract/cli-exit-21/strict-modes-failed · CLI exit 21 · strict-modes-failed — A refused strict-modes check of send, wait, last, answer, pause or release exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; nothing under the home is opened and no line is written.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/cli-json-error-document/shape · CLI --json error document · shape — One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
contract/client-server-verify/check-order · Client server verification · check order — The client's own refusals, the strict-modes instance check and the socket-directory check come before any connect; the cause has no exit code or RefusalReason of its own.
contract/mcp-frame-tools/strict-modes-failed · MCP frame tools · strict-modes-failed — A send, wait, last or answer first checks strict modes on the home and the target instance; a refusal returns the instance-unreachable isError result with detail strict-modes-failed, text refused: instance-unreachable, no frame and no line.
contract/normalised-event-kinds/link · normalised event kinds · link — `link` is a normalised event kind.
contract/normalised-event-kinds/unlink · normalised event kinds · unlink — `unlink` is a normalised event kind.
contract/protocol-version/v · protocol versioning · v — An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
contract/run-start-refusals/home-tree · viola run and revive start refusals · home tree — A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
contract/snapshot/writer · snapshot · writer — Holds the `CARGO_PKG_VERSION` of the binary that wrote the snapshot.
contract/unreachable-cause/cause-set · Unreachable cause · cause set — Exit 21 has a closed cause set (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused) written by both clients, with no serde path; it is no RefusalReason.
contract/verify-process-exit/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — The code verify's process-exit carries when the pinned copy fails its re-hash.
contract/verify-process-exit/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — The code verify's process-exit carries on a strict-modes refusal.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.
contract/viola-hook-exit-codes/0 · viola hook exit codes · 0 — Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM`, read only for SGR bytes; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`, read only.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message`, `resets_at` and `used_percentage` stay as Claude emits them.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/channel-methods/hook-event · channel methods · hook.event — A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
contract/channel-params/sender · channel params · sender — Every channel `params` carries the sending binary's `CARGO_PKG_VERSION`.
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: no live endpoint, a server that is not the recorded wrapper, a refused strict-modes check of the board, the plugin install tree or the target or driven instance, or a refused socket directory; no frame is written.
contract/cli-internal-error/output · CLI internal error output · internal error line — A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
contract/client-server-verify/server-verify-failed · Client server verification · server-verify-failed — A connected server that is not the recorded wrapper gets no frame; the verb exits 21 with the instance-unreachable document of detail server-verify-failed under --json, or the unable line and server-verify hint on stderr.
contract/exit-21-human-hints/placement · Exit 21 human hints · placement — Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.
contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — `hook statusline` emits no event.
contract/hook-to-kind-map/permissionrequest · Hook to kind map · PermissionRequest — PermissionRequest maps to `permission`, except one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`: that is the dialog's continuation and raises no event.
contract/hook-to-kind-map/pretooluse-askuserquestion · Hook to kind map · PreToolUse AskUserQuestion — PreToolUse `AskUserQuestion` maps to `question`.
contract/hook-to-kind-map/pretooluse-exitplanmode · Hook to kind map · PreToolUse ExitPlanMode — PreToolUse `ExitPlanMode` maps to `plan`.
contract/mcp-frame-tools/server-verify · MCP frame tools · server-verify — A send, wait, last or answer whose connected server is not the recorded wrapper (peer euid not this user's or unreadable, pid or OS start time not the snapshot's, or uncheckable) returns the instance-unreachable isError result, text refused: instance-unreachable, no frame and no log line.
contract/mcp-frame-tools/socket-dir-refused · MCP frame tools · socket-dir-refused — A send, wait, last or answer meeting a refused socket directory returns the instance-unreachable isError result with detail socket-dir-refused, no frame written and no log line.
contract/mcp-json-rpc-errors/32603 · MCP JSON-RPC errors · -32603 — An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is `-32603` `internal error` with `data: null`.
contract/mcp-tool-results/instance-unreachable · MCP tool results · instance-unreachable — An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
contract/mcp-tool-results/invalid-params · MCP tool results · invalid-params — A tool argument failing re-validation is an isError true result {"error":"invalid-params","detail":"<field>"}, the field one of target, text, after, timeout_ms, dialog_id, response, with no frame written.
contract/mcp-tool-results/is-error-text · MCP tool results · isError text — Every isError result's text is refused: <code>.
contract/mcp-tool-results/is-error-true · MCP tool results · isError true — A refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`.
contract/mcp-tool-results/refusal · MCP tool results · refusal — A refusal is a tool result with isError true and the channel's refusal and detail in structuredContent.
contract/mcp-tool-results/wrapper-fault · MCP tool results · wrapper-fault — A wrapper protocol fault is an isError true result {"error":"wrapper-fault","detail":…} carrying the sanitised JSON-RPC fault {code,message,data}: a fixed message, data a closed object or null.
contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
contract/run-start-refusals/socket-dir-refused · viola run and revive start refusals · socket-dir-refused — A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.

## standard-contracts · ## Standard Contracts
decision/client-strict-modes-before-frame · Strict-modes check before any client frame — Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
decision/frame-client-check-order · Frame-writing clients run their checks in one order — A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check. · why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass.
decision/hook-fails-open-on-replayed-snapshot · Hook fails open on a replayed snapshot — viola hook reads a replayed or absent snapshot as unreachable and fails open.
decision/hook-output-one-crate · Hook output produced by one crate — The Claude Code hook contract is produced only by the `viola-agent-claude` crate.
decision/link-set-changes-only-on-link-calls · Link set changes only on implicit links and link calls — The wrapper's link set changes only on an implicit link from a send or an answer and on a served link or unlink call, with each change appended to the instance log first and then mirrored to the snapshot. · why: Restoring the set at start replays only link and unlink lines, so the snapshot and the log agree only while no other path changes the set.
decision/link-set-follows-log · Link set follows the log — The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails.
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout.
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `budget-gate`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, the budget gate, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/channel-link-unlink/answer-order · channel link and unlink methods · answer order — Each answers only after its event line and the snapshot write.
contract/channel-link-unlink/error-32603 · channel link and unlink methods · -32603 — A failed record or remove of the link answers -32603.
contract/channel-link/already-linked · channel link method · already_linked — A link of a pair the set holds appends nothing and answers that pair's since with already_linked true; a new pair answers its new link line's ts as since with already_linked false.
contract/channel-methods/link · channel methods · link — A request: `link` `{driver, driven}` answers `{since, already_linked}`, sent to the driven instance's wrapper.
contract/channel-unlink/result · channel unlink method · result — Answers {} whether or not the set held the pair; an absent pair appends nothing and leaves the set unchanged.
contract/cli-exit-21-document/endpoint-lost · exit-21 CLI document · endpoint-lost — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/instance-dead · exit-21 CLI document · instance-dead — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/server-verify-failed · exit-21 CLI document · server-verify-failed — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/socket-dir-refused · exit-21 CLI document · socket-dir-refused — A refused socket directory met at connect prints the instance-unreachable document with this detail (exit 21) and one process-exit line at WARN.
contract/cli-exit-21-document/strict-modes-failed · exit-21 CLI document · strict-modes-failed — A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line.
contract/cli-exit-21-document/unwrapped-name · exit-21 CLI document · unwrapped-name — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-json-output/error · CLI `--json` output · `error` — Outcomes that have no channel `result` print an `error` document.
contract/cli-json-output/instance-unreachable · CLI `--json` output · `instance-unreachable` — `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/cli-json-output/plugin-install-strict-modes-refused · CLI `--json` output · `viola plugin install --json` strict-modes refusal — A refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` and exit 21.
contract/cli-json-output/revive-refusal · CLI `--json` output · `viola revive --json` refusal — A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
contract/cli-json-output/revive-refusal-codes · CLI `--json` output · `viola revive --json` refusal codes — `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
contract/cli-json-output/wrapper-fault · CLI `--json` output · `wrapper-fault` — `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` with exit 20; `message` is a fixed text and `data` a closed object or null.
contract/event-data/link · event data · `link` — `{driver, driven}`.
contract/event-data/unlink · event data · `unlink` — `{driver, driven}`.
contract/fault-document/invalid-params · The fault document on --json and MCP · other -32602 — Any other `-32602` fault reads `invalid params` with `data` null; every other known code reads `data` null.
contract/fault-document/known-code-message · The fault document on --json and MCP · message of a known fault code — For each of the five known fault codes, `message` is that fault's fixed text; a fault without an integer `code` reads as no fault.
contract/fault-document/release-from-driver · The fault document on --json and MCP · -32602 release-from-driver — A `-32602` fault whose `data.reason` is exactly `release-from-driver` reads the release-from-driver body.
contract/fault-document/unsupported-version · The fault document on --json and MCP · -32602 unsupported version — A `-32602` fault with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values.
contract/fault-document/wrapper-fault · The fault document on --json and MCP · wrapper fault — A fault code outside the five known codes reads `{"code": <code>, "message": "wrapper fault", "data": null}`; the literal `wrapper fault` is a fixed output word.
contract/hook-contract/async-tier · hook contract · async-tier hooks — Async-tier hooks always print nothing.
contract/hook-contract/decision · hook contract · a decision — A decision is exit 0 with a `hookSpecificOutput` body.
contract/hook-contract/no-decision · hook contract · no decision — No decision is exit 0 with an empty stdout.
contract/hook-contract/permission-request · hook contract · PermissionRequest — PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.
contract/hook-contract/pre-tool-use · hook contract · PreToolUse — PreToolUse returns `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`.
contract/hook-event/notification · channel hook.event · notification — Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}}.
contract/hook-statusline/exit · viola hook statusline · exit status — Always exits 0 with empty stderr.
contract/hook-statusline/refused-check · viola hook statusline · refused instance check — A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed, `budget.json` untouched and the home's mode left as found.
contract/hook-statusline/stdout · viola hook statusline · stdout — Prints the command's stdout unchanged, bounded by the frame cap; prints nothing when the command fails or passes the deadline, when no command is recorded, and on oversize or malformed stdin.
contract/instance-snapshot/links · instance snapshot · links — `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/mcp-list/refusal · MCP list · refusal — The board's refusal, its only one, is returned as instance-unreachable with detail strict-modes-failed.
contract/snapshot-envelope/shape · snapshot envelope · shape — A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
contract/wrapper-event-appends/link · wrapper event appends · `link` — The driven instance's wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or a served `link` call.
contract/wrapper-event-appends/unlink · wrapper event appends · `unlink` — The driven instance's wrapper appends `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes.
decision/agents-listing-never-errors · Agents listing degrades, never errors — When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
decision/replay-writes-no-file · Snapshot replay writes no file — Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/server-identity-check-before-frame · Server identity checked before the first frame — Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id, wheel and the budget gate from the log; a field no line gave is absent, never a default.
decision/statusline-check-before-log · Statusline instance check before the log — The statusline arm runs the instance check before it opens its log or reads anything under the home. · why: Opening the log sets an existing home to 0700, so a check made after it passes a home at 0770.
contract/board/agents-read-failure · The board (CLI list and MCP list) · agents read failure — If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
contract/channel-32602-cause/invalid-params · channel -32602 causes · invalid-params — One of the three causes a -32602 fault reads.
contract/channel-32602-cause/release-from-driver · channel -32602 causes · release-from-driver — One of the three causes a -32602 fault reads.
contract/channel-32602-cause/unsupported-version · channel -32602 causes · unsupported-version — One of the three causes a -32602 fault reads.
contract/channel-link-unlink/error-32602 · channel link and unlink methods · -32602 — A link or unlink answered -32602 writes no line.
contract/channel-methods/answer-from-link · channel methods · answer from link — An `answer` whose `from` names another instance records the link once every refusal has passed and its kind matched, before the answer is taken; a link that cannot be recorded answers `-32603` and the dialog stays held; others record nothing.
contract/channel-methods/budget-release-durability · channel methods · budget release durability — A `release` with `budget:true` moves no wheel and is answered only after its `budget-gate` line and the snapshot's `budget_paused` and `budget_override_until` have landed; else `-32603` and the pause stands.
contract/channel-methods/from · channel methods · from — `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`.
contract/channel-methods/hook-dialog-continuation · channel methods · hook.dialog continuation — A PermissionRequest repeating the armed PreToolUse dialog's tool with an equal `tool_input` is that dialog's continuation: no event, no new `dialog_id`, answered only from an armed `plan` revise, otherwise `null`.
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
contract/channel-methods/link-params-check · channel methods · link params check — For `link` and `unlink`, a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`.
contract/channel-methods/release-from-driver · channel methods · release from driver — A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first.
contract/channel-methods/release-type-check · channel methods · release type check — A `release` whose `from` is of a non-string type, or whose `budget` is not a bool (`null` included), is `-32602` with `data: null`; this check comes first.
contract/channel-methods/send-from-link · channel methods · send from link — A `send` whose `from` names another instance records the link before `send-issued`.
contract/channel-methods/v-and-sender · channel methods · v and sender — Every channel method's `params` carries `v` and `sender`.
contract/channel-send/from · channel send method · from — A from naming another instance records the link from that instance to this one directly before send-issued; a link that cannot be recorded answers -32603 with nothing issued or typed; no from, or the wrapper's own name, records nothing.
contract/channel-wait/wake-kinds · channel wait method · wake kinds — Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records never wake.
contract/cli-json-output/revive-list · CLI `--json` output · `viola revive <name> --list --json` — Prints `{"v":1,"ok":{"sessions":[{"ts":"…","cause":"…","id":"…"},…]}}` in log order with exit 0, or the `no-session` document with exit 1 on an empty chain.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/event-line/source · ndjson event line · source — `source` is one of `hook` · `wrapper`.
contract/hook-dialog/data · `hook.dialog` · `data` — `hook.dialog`'s `data` is the same object as its dialog event's `data`, without `dialog_id`.
contract/hook-dialog/dialog-hooks · `hook.dialog` · dialog hooks — Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog.
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
contract/hook-statusline/capture · viola hook statusline · --capture <DIR> — With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no `budget.json` write, no command run, nothing printed; without it the pass-through is unchanged.
contract/human-error-line/wrapper-fault · The human error line · wrapper fault — A fault code outside the five known codes prints the human line `error: wrapper fault  <code>`.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
contract/mcp-refusal-result/instance-unreachable · MCP refusal result · `instance-unreachable` — The MCP `isError` result for an unreachable instance is the CLI `instance-unreachable` document less `v`, its text `refused: instance-unreachable`.
contract/mcp-refusal-result/shape · MCP refusal result · result shape — `{"content":[{"type":"text","text":"refused: budget-paused"}],"isError":true,"structuredContent":{"refusal":"budget-paused","detail":"five-hour"}}`
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/revive-json/program-not-found · viola revive --json · program not found — A program not found prints the `internal-error` document, before any child is spawned.
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`; the replay writes no file.
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
contract/wrapper-channel-frames/envelope · wrapper channel frames · envelope — Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`.
contract/wrapper-channel-frames/id-less-frames · wrapper channel frames · id-less frames — Only `hook.event` may be id-less; any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`.
contract/wrapper-channel-frames/max-frame · wrapper channel frames · MAX_FRAME — A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed.
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
contract/wrapper-channel-frames/unsupported-protocol-version · wrapper channel frames · unsupported protocol version — An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.

## occupied-resources · ## Occupied Resources
decision/board-refused-whole · Whole board refused on one failed check — viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line. · why: Ratified as the founder's own answer.
decision/chunk-diff-secret-scan-skip · chunk.diff is the one secret-scan skip — The mutation run's chunk.diff is the one file secret-scan skips, by exact path, and the harness artifact upload excludes it.
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on a tmpfs link — On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk. · why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
decision/e2e-home-headroom-floor · e2e-home backing headroom floor — A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key. · why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB
decision/e2e-home-owner-only-headroom · End-to-end home base checked before use — The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free.
decision/e2e-noop-fake-agent-feature · No-op fake-agent feature in the e2e crate — The test-only `viola-e2e` crate declares a no-op `fake-agent` feature. · why: So package-scoped `--features fake-agent` invocations resolve.
decision/fake-agent-feature-on-named-members · A fake-agent feature on each named member crate — The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · why: so a package-scoped --features fake-agent build (cargo-mutants) resolves · except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC, read only by a fake-agent build, makes hook panic only on the exact value 1, with a fixed ASCII payload holding no upstream text, path or secret; the seam disables no control and widens no redaction. · why: A ratified test seam, ratified together with the exact-path exemption for its panic lines.
decision/fake-agent-pump-delay-seam · FAKE_AGENT_PUMP_DELAY_MS test seam — FAKE_AGENT_PUMP_DELAY_MS is read only by a viola built with the test-only fake-agent feature, parsed as milliseconds capped at 5 000, and holds the pump back after the child starts; it disables no control and widens no redaction. · why: Ratified as a security-plan Security Anti-Patterns carve-out: a test seam, not a configuration channel.
decision/hook-missing-instance-dir-exits-silently · Hook with a missing instance folder exits 0 silently — A hook whose VIOLA_DIR names an instance folder that does not exist fails the canonical-directory step, exits 0 and writes nothing: no role file, no stdin read, no frame and no event line. · why: Claude Code runs the plugin's hooks in sessions viola does not wrap; the hook fail-open contract and its no-instance-directory test rely on it.
decision/linked-e2e-home-base-strict · Linked e2e-home base must be a private real directory — A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path.
decision/list-home-read-rule · viola list holds the home to the read rule — viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board. · except: The budget read, mcp's start and plugin install keep the write rule on the home.
decision/no-after-every-test-error-body-helper · No after-every-test error-body helper — Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built.
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/root-tests-freeze-wrappers-with-one-guard · Root tests freeze a wrapper through one SIGSTOP guard — Root tests make a wrapper read stale by stopping it with SIGSTOP through one shared guard, which waits until the process state reads stopped and sends SIGCONT on drop. · why: The list-row and instance-state tests rely on it for a stale wrapper whose pid and start time still match while its heartbeat ages.
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 7 s WITHIN bound of the watch support.
decision/secret-scan-skips-only-chunk-diff · Secret scan skips only the chunk diff — secret-scan covers every file except the harness's chunk.diff, skipped by exact path, and the harness upload excludes that file.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
decision/test-support-never-in-product · Test-support feature kept out of product builds — The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it. · guard: release-check.sh refuses a release artifact built with it
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/diagnostics-role-files/mcp · home-level diagnostics role files · mcp.ndjson — viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.
contract/e2e-home-refusal/headroom-message · e2e-home base refusal · headroom message — A base below the floor is refused with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, the two counts and no path.
contract/hook-capture-arm/exit · viola hook --capture arm · exit status — Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened.
contract/instance-unreachable-detail/strict-modes-failed · the instance-unreachable --json document · strict-modes-failed — The detail when the instance check refused the instance.
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line.
contract/viola-list-exit-codes/21 · viola list exit codes · 21 — The strict-modes check refused the home, instances/ or any one instance, and the whole board is refused.
contract/viola-mcp-exit-codes/0 · viola mcp exit codes · 0 — The session ended by stdin closing, or by a stdin line past MAX_FRAME, once initialized; both streams empty.
contract/viola-mcp-exit-codes/1 · viola mcp exit codes · 1 — The session ended before initialize, or the home or diagnostics folder failed the strict-modes check, with nothing on stdout or stderr and no role file.
contract/viola-plugin-install-exit-codes/21 · viola plugin install exit codes · 21 — The strict-modes check refused the home or an existing marketplace tree folder; nothing created, no log opened.
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
contract/viola-verify-exit-codes/1 · viola verify exit codes · 1 — The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/config-falls-back-to-defaults · Unusable configuration falls back to defaults — config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line. · why: so no value switches the budget gate off or makes the dialog hold zero or unbounded
decision/no-deletion-of-pinned-copies · Pinned copies and plugin folders never deleted — v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses. · why: so any later cleanup can skip copies that a live or stale instance still calls
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/probe-capture-arms-inert · Probe capture arms are inert — The hidden hook `--capture` arms take an absolute, existing directory only, read no `VIOLA_*` variable, start no observability, open no channel, and always exit 0 with empty stderr.
decision/refused-statusline-check-no-trace · Refused statusline check leaves no trace — A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run, nothing printed and budget.json untouched. · why: Ratified as the founder's own answer.
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined.
contract/channel-response-detail/unsupported-version · channel-response detail, a closed enum · unsupported-version — Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name.
contract/config-json/v · config.json · v — config.json carries v, which must be 1; the file is read with a MAX_FRAME cap.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/hook-decision-detail/socket-dir-refused · hook-decision detail · socket-dir-refused — One hook-decision line at WARN on a refused socket folder, with decision_emitted false.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — A refusal code verify writes under a parsed VIOLA_NAME.
contract/viola-link-exit-codes/21 · viola link and unlink exit codes · 21 — The instance check refused, no live endpoint is recorded, the recorded endpoint's folder is refused, or the serving process is not the recorded wrapper or runs under another euid.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/hook-exits-on-untrusted-instance · Hook exits at once on an untrusted instance — viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append. · except: The two --capture arms, which read no VIOLA_* variable, run ahead of the check.
decision/hook-fails-open · Hook fails open toward the human — Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts.
decision/no-other-os-test-gates · No test gate naming another OS — A test gate whose predicate names windows or macOS is a fault even when the gates table names it. · guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
decision/os-gated-tests-name-reason · OS-gated tests name their reason — Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it. · guard: contract_os_gated_tests
decision/os-gated-tests-named · OS-gated tests named with their reason — Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands. · guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/diagnostics-level-sole-log-level · diagnostics_level as the sole log level source — The config key diagnostics_level is the only source of the process-log level; RUST_LOG has no effect.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
contract/hook-statusline-output/stdout · Hook statusline output · stdout — The hook statusline pass-through carries the user's status line output on stdout and nothing else; its --capture probe arm prints nothing.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
contract/channel/framing · Channel · framing — The channel speaks JSON-RPC 2.0 over ndjson.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.

## build-system · key of ## Infrastructure Patterns
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh
decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
contract/cli-json/internal-error · CLI --json output · internal-error — Under `--json` an internal error prints the `internal-error` document on stdout, with stderr empty.
contract/human-stderr/internal-error · Human stderr · internal error — An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/release-build-viola-only · Release build yields viola only — The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent. · why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables. · guard: scripts/release-check.sh
decision/uploads-gated-on-secret-scan · Uploads gated on the secret scan — CI artifacts are uploaded only when the secret scan succeeds; on a scan failure only the secret-scan report is uploaded. · guard: secret-scan step
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps, budget and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.
decision/e2e-test-only · viola-e2e is test-only — viola-e2e is test-only, and no product crate depends on it.

## deployment-model · key of ## Infrastructure Patterns
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.

## project-directory-structure · key of ## Infrastructure Patterns
decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes
decision/release-carries-viola-only · Release build carries viola only — The release build carries the viola binary only and no test-only feature. · guard: release-check in the CI release job
decision/channel-json-rpc-local-sockets · JSON-RPC over local sockets for the channel — The channel speaks JSON-RPC 2.0 as newline-delimited JSON over interprocess local sockets.

# architecture · its absences, names and overviews, whole

## stack-and-technologies · ## Stack and Technologies
absence/model-api-calls — Calling any model API; viola drives the unmodified claude CLI on the user's subscription
overview/stack-and-technologies — viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.

## established-decisions · ## Established Decisions
absence/daemon — A daemon
absence/graphical-frontend — A web or graphical frontend, page or graphical control surface
absence/mobile-view — A mobile or phone view
absence/windows-and-macos-support — Support for Windows or macOS

## conventions · ## Conventions
absence/database — A database

## standard-contracts · ## Standard Contracts
absence/graphical-interface — A web or graphical surface
absence/polling-endpoints — Polling endpoints (v1)
absence/websocket — WebSocket endpoints (v1)

## occupied-resources · ## Occupied Resources
absence/listening-network-port — A listening network port of any kind: no database port, no broker port, no listening TCP port
name/binary/viola — viola
name/binary/viola-fake-agent — viola-fake-agent
name/binary/viola-harness — viola-harness
name/binary/viola-mcp-tap — viola-mcp-tap
name/command/answer — answer
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/verify — verify
name/command/wait — wait
name/crate/viola — viola
name/crate/viola-agent-claude — viola-agent-claude
name/crate/viola-channel — viola-channel
name/crate/viola-core — viola-core
name/crate/viola-e2e — viola-e2e
name/crate/viola-fuzz — viola-fuzz
name/crate/viola-mcp — viola-mcp
name/crate/viola-pty — viola-pty
name/crate/viola-state — viola-state
name/other/agent-run-chunk-base — AGENT_RUN_CHUNK_BASE
name/other/agent-run-keep-failed — AGENT_RUN_KEEP_FAILED
name/other/agent-run-keep-homes — AGENT_RUN_KEEP_HOMES
name/other/fake-agent-hook-panic — FAKE_AGENT_HOOK_PANIC
name/other/fake-agent-pump-delay-ms — FAKE_AGENT_PUMP_DELAY_MS
name/other/hook-notification — hook notification
name/other/hook-permission-request — hook permission-request
name/other/hook-post-tool-use — hook post-tool-use
name/other/hook-post-tool-use-failure — hook post-tool-use-failure
name/other/hook-pre-tool-use — hook pre-tool-use
name/other/hook-session-end — hook session-end
name/other/hook-session-start — hook session-start
name/other/hook-statusline — hook statusline
name/other/hook-stop — hook stop
name/other/hook-user-prompt-submit — hook user-prompt-submit
name/other/mcp-serve-test-child — MCP_SERVE_TEST_CHILD
name/other/mcp-tool-answer — answer
name/other/mcp-tool-last — last
name/other/mcp-tool-list — list
name/other/mcp-tool-send — send
name/other/mcp-tool-wait — wait
name/other/viola — viola
name/other/viola-at-viola — viola@viola
name/other/viola-bin — VIOLA_BIN
name/other/viola-dir — VIOLA_DIR
name/other/viola-h12-sock — viola-<h12>.sock
name/other/viola-name — VIOLA_NAME
name/other/viola-test-chan-pid-label — viola-test-chan-<pid>-<label>
name/other/viola-verify-probe — viola-verify-probe
name/server/viola — viola

## cross-cutting-patterns · ## Cross-cutting Patterns
absence/secrets — Secrets of any kind in v1 configuration

## project-intent · ## Project Intent
absence/web-or-graphical-frontend — A web or graphical frontend
overview/project-intent — viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary and a Claude Code plugin of hooks and a stdio MCP server that calls that binary; a keyboard-driven terminal fleet console is planned, not built. It wraps the unmodified claude CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around core, and no daemon.

## inherited-defaults · ## Inherited Defaults
absence/containers-and-hosting — Containers or hosting of any kind

## ci-cd-approach · key of ## Infrastructure Patterns
absence/deploy-stage — A deploy stage

## deployment-model · key of ## Infrastructure Patterns
absence/container-or-serverless-deployment — Container, orchestration or serverless deployment (Docker, Compose, Kubernetes, serverless)
