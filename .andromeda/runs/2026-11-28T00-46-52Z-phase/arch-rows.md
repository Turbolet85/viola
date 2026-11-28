# architecture · the rows that bind 2026-11-28-console-scaffold — picked 276 of 1049 (149 by every draw) · rule any · pin 5229

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
decision/human-always-wins-wheel · The human always wins the wheel — Human keystrokes are never blocked, refused or delayed past the current paste's in-flight chunk and closing marker; automation is refused with a typed reason instead of competing with the human.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.
decision/send-confirmed-after-the-fact · Every send confirmed after the fact — A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash. · why: A turn boundary per the hooks does not prove the input box is ready. · except: A listed local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI returns `ok`, `confirmed: false`, `unconfirmable`.

## stack-and-technologies · ## Stack and Technologies
decision/cargo-deny-dependency-policy · cargo-deny dependency policy — cargo-deny enforces advisories, licences, sources and bans: C crates, telemetry crates, feature bans, and tokio banned per sync crate.
decision/chrono-timestamps · chrono RFC 3339 UTC timestamps — Timestamps are RFC 3339 UTC with millisecond precision, through chrono. · why: already in the tree through rmcp
decision/clap-cli-parser · clap as CLI parser, without colour — clap (derive) parses the CLI with its default features minus color. · why: help and usage output stays plain and no colour or terminal crate is in the graph
decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/hybrid-concurrency · Std threads everywhere, Tokio only inside mcp — Every CLI verb runs on std threads; Tokio is built only inside mcp. · why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
decision/libc-only-os-binding · libc is the one OS-binding crate — libc is the one OS-binding crate of the Linux-only build, a plain dependency; no crate depends on windows-sys.
decision/portable-pty-seam · portable-pty behind viola's own pty seam — The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/msrv-floor · Declared MSRV floor 1.96 — The workspace declares rust-version 1.96 and CI builds against that floor without touching the toolchain pin. · why: the security toolchain floor · guard: msrv CI job
decision/no-database · No database: ndjson logs and JSON snapshots — State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
decision/no-message-broker · No message broker — Request and response go over per-wrapper local sockets as JSON-RPC 2.0 over ndjson, with no broker or daemon.
decision/no-unbounded-json-depth · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned. · why: every parse of external bytes keeps serde_json's default recursion limit · guard: deny.toml
decision/run-never-reads-screen-content · run never reads screen content — run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures and the rig-only first-screen record of whole scrubbed rows.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written
decision/wrapper-ipc-local-socket · interprocess local sockets for wrapper IPC — Each viola run has one endpoint, a Unix domain socket through interprocess's sync local_socket API, chmod 0600 after bind in a per-user directory; the Tokio flavour is used only by the MCP client.

## established-decisions · ## Established Decisions
decision/cargo-workspace-flat-crates · Cargo workspace with flat crates — The project is a Cargo workspace with a flat crates layout, the root manifest being both the viola bin package and the workspace; each product crate is created by the first piece of work that consumes it. · why: The compiler enforces that viola-pty cannot see viola-agent-claude; cargo install --path . requires the bin at the root.
decision/channel-blocks-never-polls · Nothing polls the channel — `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
decision/chrono-utc-millisecond-timestamps · chrono timestamps, RFC 3339 UTC, milliseconds — chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision. · why: chrono is already in the tree through rmcp.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/flat-cargo-workspace · Flat Cargo workspace with the bin at the root — One Cargo workspace with a flat `crates/` layout, the root `Cargo.toml` being both the `viola` bin package and the workspace; `fuzz/` is its own workspace. · why: The compiler enforces that `viola-pty` cannot see `viola-agent-claude`; `cargo install --path .` needs the bin at the root.
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/local-command-context-probe · Local command context ledger row and its probe — The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window; verify probes it by Run B's eighth paste, /context, and a failing read of it is fail, never absent. · why: Ratified as a widening of verify's pastes.
decision/local-command-echo-confirmation · Local command confirmed by its stdout echo — A local command whose post-condition is a stdout echo is confirmed by counting `<local-command-stdout>` tags in the session transcript past its end recorded before the paste; only a count crosses, never content, and the read is bounded by MAX_FRAME. · why: Ratified; the read opens no symlink, only a regular file owned by the own euid.
decision/local-command-echo-probe · Local command echo ledger row and its probe — The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes on a new stdout tag with no UserPromptSubmit within the confirmation window; with no readable transcript the paste is skipped and the row fails. · why: Ratified as a widening of verify's pastes.
decision/no-automatic-reverify-of-red · No automatic verify of an earlier red version — An automatic verify runs only for a version whose stamps entry is short of a relied row; an entry holding every relied row that did not verify is never verified again automatically. · why: So a red version does not loop.
decision/paste-bound · Paste cut at a compiled paste bound — A send's paste not finished within the compiled 5 s paste bound, counted from the call, is cut: the writer stops after its in-flight chunk of at most 4096 bytes and writes the closing paste marker with no Enter; a paste still queued at the deadline is cut with nothing written. · why: So a paste never blocks past its bound, even while queued behind human keys.
decision/paste-cut-mark · Cut mark refuses sends until the human acts — A cut sets an in-memory mark at the wheel's human-key count; while the count is unchanged the next send is refused not-delivered paste-cut with no cursor; a human key read, a prompt-submitted of any origin, a session-start or a session-end clears it; a release with no key or a turn-ended leaves it.
decision/portable-pty-behind-pty-seam · portable-pty pinned behind viola's pty seam — portable-pty, pinned exactly at `=0.9.0`, is used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill). · why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils` (RUSTSEC-2026-0441), and `viola-pty` builds on it unchanged.
decision/screen-rows-read-only-by-verify · Screen row text read only by verify — The screen model has one row-text exit, and only viola verify reads it.
decision/send-flight-lock · Send post-gate section under the wrapper flight lock — A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes the lock. · why: Closes the race between the readiness gate and a turn the wrapper learns of, while the human's keys never wait.
decision/send-never-queued · A send mid-turn or mid-send is refused, never queued — A `send` while a turn runs, while another `send` is in flight, or while a switch stands, from its request until the new child's `session-start`, is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
decision/send-time-bound · Send time bound — send blocks at most the readiness gate's maximum wait plus the compiled 5 s paste bound plus the window; on the MCP surface the MCP call deadline bounds the driver's wait, never the send itself.
decision/sync-core-tokio-only-in-mcp · Hybrid sync core, Tokio only in mcp — The core is synchronous, and Tokio is confined to the `mcp` subcommand; among the product crates only `viola-mcp` enables `tokio`. · why: portable-pty's handles are blocking, so the pump needs threads either way; `hook` must not pay for runtime startup; rmcp and interprocess-async require Tokio. · guard: sole-root `cargo deny` `tokio` ban (`deny-sync.toml`) and a `cargo check` of the sync crates on `ubuntu-latest` · except: `viola-channel`'s optional `tokio` feature, enabled by `viola-mcp`; its graph without the feature is proven tokio-free.
decision/terminal-fleet-console · Terminal fleet console as the one view — The one view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/tokio-only-in-mcp-crate · tokio only in the MCP crate — Among the product crates only viola-mcp enables tokio; viola-channel lists it optional behind its tokio feature, and no other product member's normal graph reaches tokio. · guard: scripts/channel-tokio-free.sh
decision/typed-text-normalisation · Typed text normalisation — send types its text with every CR LF pair and every other CR as one LF and without trailing CR and LF characters, nothing else changed, and uses that typed text for classification, matching, text_bytes and the paste. · why: The CLI drops a pasted text's last newline and submits a CR as LF, so a text typed with them failed the exact match.
decision/verify-box-wait-for-literal · Verify waits for an input-box literal after added turns — After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding the input box (a footer literal or the box's own rows) or a modal literal, a poisoned screen, or the probe deadline; any other quiet screen keeps waiting. · why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
decision/verify-never-types-into-dialog · viola verify never types into a CLI dialog — No `viola verify` run writes a byte into a CLI dialog.
decision/verify-types-nothing-into-dialogs · Verify never types into a CLI dialog — No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key. · why: Ratified rulings.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
decision/wait-blocks-nothing-polls · wait blocks, nothing polls — wait blocks until the next driver-relevant event and returns it, so nothing polls and a driver parked in wait learns that a dialog is pending.
contract/events-ndjson/send-confirmed-order · events.ndjson · send-confirmed order — The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts; a send confirmed by a stdout echo has no such line and its submitted_at is the instant the echo was read.
contract/not-delivered-details/no-prompt-submitted · not-delivered refusal details · no-prompt-submitted — A send with no matching `prompt-submitted` within the compiled confirmation window is refused `not-delivered` with this detail.
contract/not-delivered-details/paste-cut · not-delivered refusal details · paste-cut — A send whose paste was cut at its bound, with that send's cursor, or refused by a standing cut mark, with no cursor, is refused `not-delivered` with this detail, exit 13.
contract/send-refusal/manual-pause · Send refusal details · manual-pause — After `viola pause`, automated `send` is refused `human-typing` with detail `manual-pause`.
contract/send-refusal/order · Send refusal details · order — `control-character` → `empty-text` → `human-typing` → `turn-running` → the readiness gate's wait → `human-typing` and `turn-running` again → `paste-cut` of a standing cut mark → `paste-cut` of a paste cut at its bound.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|absent|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.
decision/anyhow-only-in-bin · anyhow only in the viola binary — anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter. · why: anyhow's context chains are only useful where a person reads them.
decision/atomic-bracketed-send · Atomic bracketed-paste send — A bridge send is written as one bracketed-paste write plus CR, and human bytes arriving during it are held and passed on straight after. · except: A paste not finished within the compiled paste bound is cut: the writer stops after its in-flight chunk and writes the closing paste marker with no CR.
decision/browser-tools-prompt-ledger-row · Browser-tools-prompt ledger row and its readings — The browser-tools-prompt relied ledger row reads pass when some verify run's first screen is the CLI's browser-tools prompt, absent when no run met it, and fail when any first screen reads none; it is the one relied row whose absent counts toward verified.
decision/cross-session-prefix-ledger-row · Cross-session-prefix ledger row and its readings — The cross-session-prefix relied ledger row reads pass only when a print-mode sender's message to verify's Run B, started with the crossSessionInbound accept settings, arrives opening with the CLI's cross-session harness prefix, else fail, never on the sender's words; no runtime dir runs no turn.
decision/every-send-confirmed · Every send is confirmed — A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted. · why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate. · except: A ledger-listed local command is confirmed by its own measured post-condition, such as a stdout echo in the session transcript, or answered ok unconfirmable where none can be read.
decision/human-key-never-held-behind-resize · Human key never held behind a resize — viola never holds, queues or re-orders a human key behind a resize. · why: On Linux, real claude kept a key typed 0, 20 or 100 ms after a resize of its own pty in every measured round, so viola adds no wait; the key-loss reading is a Windows-ConPTY measurement.
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/ipc-interprocess-local-socket · interprocess local_socket for IPC — Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls. · why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
decision/lineage-label-only · Lineage is a label only — Lineage is a label only: no wheel, link, send or answer path reads the declared parent or role; viola up reads the parent only to order its re-opens. · guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
decision/link-recording · How a driver-to-driven link is recorded — A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`; `viola unlink` removes it.
decision/links-are-log-events · Links are events in the log — Links are events in the log, and the snapshot's link set is derived by replaying them. · why: The log stays the single source of truth for the link set.
decision/links-recorded-as-log-events · Links recorded as log events — A link `driver → driven` is recorded when a `send` or `answer` comes from inside a wrapped session to another instance, or by `viola link`, and removed by `viola unlink`; the snapshot's link set is derived by replaying those events. · why: The log stays the single source of truth for the link set as for everything else.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/no-cargo-pup · cargo_pup not taken — Module boundaries are enforced by crate separation, not by cargo_pup. · why: cargo_pup needs nightly and was rejected.
decision/no-da1-answer · viola never answers DA1 — viola never answers a DA1 query itself.
decision/no-probe-for-unrelied-shapes · No probe for a shape no send relies on — A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only. · why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
decision/partial-gate-on-unverified-cli · Partial readiness gate on an unverified CLI — On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
decision/permission-request-question-ledger-row · Permission-request-question ledger row and its probe — The permission-request-question relied ledger row is probed by verify's Run C with a compiled prompt raising a question through PermissionRequest, answered by the closed probe id permission-question-first-option; it passes only when that request is raised and the answer reaches the tool's answers.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/send-never-types-trailing-newline · Send never types a trailing newline — send never types a text that ends in a newline. · why: A pasted text's own last newline never reaches the hook.
decision/send-text-line-endings · Send text line-ending normalisation — A `send` types its text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters.
decision/separate-lock-files · Locks on separate .lock files — State locks are std `File::lock` on separate `.lock` files, opened for write, so no file is both appended to and exclusively locked. · why: A layout chosen while Windows, where append plus exclusive lock fails, was a target, and kept.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/start-verifies-new-cli-version · Start verifies a new CLI version when keyed — With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind, before the trust record and the claude child, and gates on the final verdict; the key is off by default.
decision/submitted-form-prompt-match · Prompt match in the submitted form after agent normalisation — The typed text matches prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and frame and reverses tag escaping, when equal or when non-empty and equal to the typed text less its trailing run of ASCII space, TAB, CR and LF. · why: The CLI trims only a trailing run at submit: it keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps an inner run of spaces and an inner LF.
decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
decision/trust-record-ledger-row · Trust-record ledger row and its probe — The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box. · why: Ratified as a widening of verify's child set.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, one agents --json read inside Run B and one print-mode sender, verify-sender, inside Run B's cross-session turn, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
contract/channel-results/cursor · channel results · cursor — Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
contract/human-hint-lines/paste-cut · Human hint lines · paste-cut — Reads "<name> did not take the whole text in time; a human must look at the session before the next send".
contract/send-results/input-not-ready · send results · input-not-ready — A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a standing modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
contract/send-results/unconfirmable · send results · unconfirmable — A listed local command whose post-condition is none, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written. A probe run whose first screen read the trust or imports form is no failed row but the untrusted-folder refusal, which writes no stamp.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
contract/viola-verify-refusal-codes/untrusted-folder · viola verify process-exit refusal codes · untrusted-folder — Written on the process-exit verify logs under a parsed VIOLA_NAME when a probe run's first screen read the trust or imports form; no stamp is written.

## conventions · ## Conventions
decision/cli-flag-and-subcommand-style · CLI flag and subcommand style — CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
decision/crate-naming · Crate naming — Crates are named `viola-<area>`, kebab-case.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
decision/four-interfaces-one-binary · Four interfaces from one binary — The binary exposes four interfaces: CLI verbs, MCP tools over stdio, the hook stdin/stdout contract, and the wrapper channel; there is no REST write surface, GraphQL, gRPC or tRPC.
decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`. · except: ledger/verify.lock, the automatic verify's attempt lock, guards no file of its own name and stands apart from stamps.json.lock.
decision/silent-roles-at-internal-error-catch · Run, revive, adopt, mcp and hook silent at the catch site — Run, revive's start, adopt's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
contract/code-word/format · Code word · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/refusal-order/first-only · Refusal order · several refusals — When several refusals apply, the wrapper returns only the first in the fixed order for the verb.
contract/refusal-order/send · Refusal order · send — control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, paste-cut of a standing cut mark, paste-cut of a paste cut at its bound, then no-prompt-submitted.
contract/send-ok-payload/unconfirmable · send ok payload · unconfirmable — Only an `ok` payload's detail, with `confirmed: false`, never a refusal detail.
contract/send/paste-failure-after-issue · send refusals · paste failing after send-issued — A paste that fails after send-issued is refused as input-not-ready with that send's cursor, except a paste cut at its bound, refused as paste-cut with that send's cursor.
decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
decision/anyhow-only-in-root-bin · anyhow only in the root bin crate — `anyhow` appears only in the root bin crate.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
decision/one-error-enum-per-crate · One error enum per crate — Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages. · except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
decision/retired-exit-codes-never-reused · Retired exit codes never reused — A retired CLI exit code is never reused or renumbered: its number stays unassigned.
decision/standard-rust-naming · Standard Rust naming — Modules and files are snake_case, types UpperCamelCase, functions and variables snake_case, and constants SCREAMING_SNAKE_CASE, as standard Rust.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight, open, focus, up and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document; also an up whose lines are not all successes.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/cli-internal-error/output · CLI internal error output · internal error line — A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
contract/normalised-event-kinds/send-confirmed · normalised event kinds · send-confirmed — `send-confirmed` is a normalised event kind.
contract/normalised-event-kinds/send-issued · normalised event kinds · send-issued — `send-issued` is a normalised event kind.
contract/normalised-event-kinds/send-refused · normalised event kinds · send-refused — `send-refused` is a normalised event kind.
contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
contract/refusal-reason/human-typing · RefusalReason · human-typing — Serialised kebab-case; its detail is null or manual-pause.
contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut, dialog-pending, no-session, child-not-ended, resume-failed.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.

## standard-contracts · ## Standard Contracts
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/stamps-merge-own-version-only · Stamps merge replaces only its own version — A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row counts — A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
contract/channel-methods/send · channel methods · send — A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
contract/channel-methods/send-from-link · channel methods · send from link — A `send` whose `from` names another instance records the link before `send-issued`.
contract/channel-send/from · channel send method · from — A from naming another instance records the link from that instance to this one directly before send-issued; a link that cannot be recorded answers -32603 with nothing issued or typed; no from, or the wrapper's own name, records nothing.
contract/channel-send/result · channel send method · result — {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
contract/event-data/send-confirmed · event data · `send-confirmed` — `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record.
contract/event-data/send-issued · event data · `send-issued` — `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor.
contract/event-data/send-refused · event data · `send-refused` — `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
decision/api-error-turn-end-clears-turn · API-error turn end clears the running turn — An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted. · guard: path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `lineage`, `launch`, `window`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`. · why: They are how the audit trail and a replay learn the wheel holder, each send's outcome and the link set. · except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
contract/channel-methods/hook-event-driver-origin · channel methods · hook.event driver origin — A `prompt-submitted` that is the in-flight `send`'s typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with `origin:"driver"`, its text kept as submitted, and only then settles that send.
contract/event-data/prompt-submitted-text · event data per kind · prompt-submitted text — `prompt-submitted` data is `{text, origin}`; `text` is the submitted prompt as typed, the long-paste wrapper and its frame removed and tag escaping reversed.
contract/event-prompt-submitted/origin · prompt-submitted event data · origin — driver, human or harness; the hook files only human or harness; the wrapper sets driver only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake harness.
contract/hook-event/driver-relabel · channel hook.event · driver relabel — A prompt-submitted whose text is the in-flight send's typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with origin driver, its text kept as submitted, and only then settles that send.
contract/instance-snapshot/started-at · instance snapshot · started_at — `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
contract/ledger-stamps/first-screen · ledger stamps envelope · measured.first_screen — For each of the six interactive verify runs, one closed word naming its first settled screen: trust, imports, browser-tools, box, none or exited; no screen text.
contract/ledger-stamps/first-settle-ms · ledger stamps envelope · measured.first_settle_ms — For each of the six interactive verify runs, the time to its first settled screen in milliseconds, a whole number.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/wrapper-event-appends/link · wrapper event appends · `link` — The driven instance's wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or a served `link` call.
contract/wrapper-event-appends/send-records · wrapper event appends · `send-issued` / `send-confirmed` / `send-refused` — The wrapper appends the send records with `source: wrapper` for each send.

## occupied-resources · ## Occupied Resources
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/colour-env-decides-only-sgr · Colour variables decide only SGR output — Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read. · why: ratified: the colour variables are to be read
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
decision/harness-env-unread-by-product · Harness variables never read by viola — AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED, AGENT_RUN_RIG and CI are test-harness variables that viola itself never reads.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
decision/test-support-never-in-product · Test-support feature kept out of product builds — The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it. · guard: release-check.sh refuses a release artifact built with it
decision/verify-probe-dirs-drop-guarded · verify probe dirs removed by drop guard — Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing. · why: its captures are transient, content-bearing raw hook payloads
decision/verify-removes-only-own-dirs · verify removes only the probe folders it created — viola verify removes its probe folders whole, and its own trust record from the Claude config file, when it passes, fails or is stopped by Ctrl-C or TERM, only what that run created, never by a pattern or a listing. · except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand; a SIGKILL also leaves verify's own one-key trust record.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/viola-channel-tokio-free-by-default · viola-channel tokio-free without its feature — The viola-channel dependency graph without its tokio feature carries no Tokio; only a crate that enables the feature, viola-mcp, gets the Tokio client. · guard: channel-tokio-free.sh
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
contract/config-json/diagnostics-level · config.json · diagnostics_level — "info" by default, or "debug"; diagnostics_level is the only source of the process-log level, and RUST_LOG has no effect.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/human-output-colour-env/colorterm · Human output colour environment · COLORTERM — truecolor or 24bit picks truecolor.
contract/human-output-colour-env/no-color · Human output colour environment · NO_COLOR — Present, an empty value included, switches colour off.
contract/human-output-colour-env/term · Human output colour environment · TERM — Exactly dumb switches colour off; a value containing 256color picks 256 colours.
decision/config-bad-value-falls-back · Bad config values fall back to defaults — An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value makes the dialog hold zero or unbounded.
decision/config-falls-back-to-defaults · Unusable configuration falls back to defaults — config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line. · why: so no value makes the dialog hold zero or unbounded
decision/fake-agent-feature-on-named-members · A fake-agent feature on each named member crate — The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · why: so a package-scoped --features fake-agent build (cargo-mutants) resolves · except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness, viola-mcp-tap and viola-live-rig are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
decision/verify-run-e-resumes-run-b-first-session · Verify's Run E resumes Run B's first session — Verify's Run E resumes the session id taken from Run B's first SessionStart, only when that id has a session id's shape; with no such id Run E starts no child, so Run E depends on Run B running before it. · why: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the six runs execute in order A to F.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME for list, verify, revive --list, up and plugin install.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/diagnostics-role-files/mcp · home-level diagnostics role files · mcp.ndjson — viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm and viola adopt's start included.
contract/viola-cli-arguments/link-unlink · viola CLI arguments · link and unlink — viola link and viola unlink each take <driver> <driven> [--json], the verb's instance being the driven one.
contract/viola-cli-arguments/list · viola CLI arguments · list — viola list takes [--json] and nothing else.
contract/viola-cli-arguments/plugin-install · viola CLI arguments · plugin install — viola plugin install takes [--json] and nothing else; viola plugin alone is a usage error.
contract/viola-cli-arguments/revive · viola CLI arguments · revive — viola revive takes <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>], or <name> --list, each with [--json].
contract/viola-cli-arguments/revive-origin · viola CLI arguments · revive --origin — viola revive --origin <cli|agent> is a hidden flag, conflicting with --list and with no --place: under it the revive places its window on the workspace a real snapshot recorded, else none; without it there is no window step.
contract/viola-cli-arguments/run-effort · viola CLI arguments · run --effort — viola run --effort <EFFORT> names the effort handed to the child, one of the five effort words; any other value is a usage error, exit 2; with none given the effort is the config.json default, else the compiled default.
contract/viola-cli-arguments/run-model · viola CLI arguments · run --model — viola run --model <MODEL> names the model handed to the child, a model word; a value failing that shape is a usage error, exit 2; with none given the model is the config.json default, else the compiled default.
contract/viola-cli-arguments/run-origin · viola CLI arguments · run --origin — viola run --origin <WORD> is a hidden flag naming the window's origin, an origin word; any other word is a usage error, exit 2.
contract/viola-cli-arguments/run-parent · viola CLI arguments · run --parent — viola run --parent <NAME> declares the instance's parent; a parent equal to the instance's own name is a usage error, exit 2, before anything is read; with no --parent the inherited VIOLA_NAME is taken as parent only, dropped when it equals the own name.
contract/viola-cli-arguments/run-place · viola CLI arguments · run --place — viola run --place <N> is a hidden flag naming the workspace the window is moved to; --place without --origin is a usage error, exit 2.
contract/viola-cli-arguments/run-role · viola CLI arguments · run --role — viola run --role <WORD> declares the instance's role, a code word.
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude; a hidden --record-first-screens, absent from --help, is passed only by the live rig.
contract/viola-verify-exit-codes/1 · viola verify exit codes · 1 — The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/clap-without-color-feature · clap built without its color feature — clap is built without its color feature, so viola's own colour layer is the only reader of colour environment variables. · why: The feature's anstream and anstyle-query route read CLICOLOR, CLICOLOR_FORCE, NO_COLOR, TERM, COLORTERM and CI.
decision/cli-verbs-human-stderr · Short-lived CLI verbs may use stderr — Short-lived CLI verbs may use stderr for their human output.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/diagnostics-level-sole-log-level · diagnostics_level as the sole log level source — The config key diagnostics_level is the only source of the process-log level; RUST_LOG has no effect.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/serde-json-unbounded-depth-banned · serde_json unbounded depth banned — The serde_json unbounded-depth feature is banned in the dependency policy. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/config-precedence · Configuration precedence — CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI is reported ok, confirmed false, unconfirmable.
contract/config-json/unknown-keys · config.json · unknown keys — config.json is parsed tolerantly: unknown keys are skipped and counted.

## project-intent · ## Project Intent
decision/modular-monolith-no-daemon · Modular monolith, no daemon — The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.

## inherited-defaults · ## Inherited Defaults
decision/clap-for-cli · clap parses the CLI — clap is what parses the CLI.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/dual-mit-apache-license · MIT OR Apache-2.0 license — Every workspace crate inherits the license MIT OR Apache-2.0 from the workspace package.
decision/no-crates-io-publish · Nothing published to crates.io — Every workspace crate, the root bin included, sets publish false; nothing is published to crates.io in v1.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.

## build-system · key of ## Infrastructure Patterns
decision/banned-features · Banned dependency features — The features veil/toggle, rmcp auth and transport-streamable-http-server, and tracing-subscriber/env-filter are banned. · guard: cargo deny check
decision/crates-io-only · Sources from crates.io only — Every dependency comes from crates.io only. · guard: cargo deny check
decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
decision/licence-allowlist · Licence allowlist — Dependency licences allowed are exactly MIT, Apache-2.0, Zlib and Unicode-3.0. · why: The 0BSD exception was ratified. · guard: cargo deny check · except: 0BSD only through the per-crate exception for doctest-file, never through the allowlist.
decision/no-c-building-crates · No C-building crates — C-building crates (cc, libsqlite3-sys, openssl-sys) are banned from the dependency graph. · guard: cargo deny check · except: The fuzz workspace's libfuzzer-sys builds C++ via cc, a ratified test-only exemption: never linked into viola, and fuzz never joins the root workspace.
decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh
decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
decision/no-telemetry-crates · No telemetry crates — Telemetry crates (opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender) are banned. · guard: cargo deny check
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
decision/sync-crates-tokio-free · Sync crates are tokio-free — Tokio is banned anywhere, direct or transitive, in the normal dependency graph of viola-core, viola-pty, viola-state and viola-agent-claude, checked once per crate as the sole root. · why: A direct-parent allowlist would let Tokio arrive through a third-party feature; excluding over the workspace false-fails under feature unification. · guard: deny-sync.toml per crate in scripts/sync-crates.txt
decision/workspace-dependency-pins · Third-party versions pinned in one place — Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it.
contract/human-stderr/internal-error · Human stderr · internal error — An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.
decision/deny-single-target · Dependency policy for one target triple — The dependency policy is evaluated for the one target triple x86_64-unknown-linux-gnu.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/release-build-viola-only · Release build yields viola only — The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent. · why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables. · guard: scripts/release-check.sh
decision/sync-crates-compile-without-tokio · Sync crates compile without tokio — Each listed sync crate, the hook path included, is checked to compile on its own with no tokio; an empty list or an absent package fails.
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
decision/weekly-advisories · Weekly advisory audit — A weekly job audits advisories over both the root and the fuzz lockfile. · why: The advisory database moves without code changes.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/core-depends-on-no-viola-crate · viola-core depends on no viola crate — viola-core depends on no other viola crate.
decision/pty-knows-no-agent · viola-pty depends on no viola crate and knows no agent — viola-pty depends on no other viola crate, knows no agent, and takes no thiserror.
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/e2e-test-only · viola-e2e is test-only — viola-e2e is test-only, and no product crate depends on it.

## deployment-model · key of ## Infrastructure Patterns
decision/local-only-deployment · Local-only deployment — viola is deployed local-only.
decision/single-binary-on-path · Single binary on PATH — viola ships as a single binary installed on PATH.

## project-directory-structure · key of ## Infrastructure Patterns
decision/cargo-deny-bans · cargo-deny supply-chain policy — cargo-deny checks advisories, licences and sources, and bans C dependencies, telemetry and listed features. · guard: deny-probes negative probe per ban with a clean control
decision/no-tokio-in-sync-crates · No tokio in sync crates — Every crate on the single sync-crate list must have no tokio in its graph, each checked as the sole root. · guard: cargo-deny tokio ban over the sync-crate list
decision/pty-seam-portable-pty · portable-pty is the PTY seam — The PTY seam is built over portable-pty, with host raw mode through libc termios.
decision/release-carries-viola-only · Release build carries viola only — The release build carries the viola binary only and no test-only feature. · guard: release-check in the CI release job
decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage

## stack-and-technologies · ## Stack and Technologies
absence/model-api-calls — Calling any model API; viola drives the unmodified claude CLI on the user's subscription
overview/stack-and-technologies — viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.

## established-decisions · ## Established Decisions
absence/daemon — A daemon
absence/graphical-frontend — A web or graphical frontend, page or graphical control surface
absence/mobile-view — A mobile or phone view
absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command
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
name/binary/viola-hypr-tap — viola-hypr-tap
name/binary/viola-live-rig — viola-live-rig
name/binary/viola-mcp-tap — viola-mcp-tap
name/command/adopt — adopt
name/command/answer — answer
name/command/focus — focus
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/open — open
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/switch — switch
name/command/unlink — unlink
name/command/up — up
name/command/verify — verify
name/command/wait — wait
name/crate/viola — viola
name/crate/viola-agent-claude — viola-agent-claude
name/crate/viola-channel — viola-channel
name/crate/viola-core — viola-core
name/crate/viola-e2e — viola-e2e
name/crate/viola-fuzz — viola-fuzz
name/crate/viola-hyprland — viola-hyprland
name/crate/viola-mcp — viola-mcp
name/crate/viola-pty — viola-pty
name/crate/viola-state — viola-state
name/other/agent-run-chunk-base — AGENT_RUN_CHUNK_BASE
name/other/agent-run-keep-failed — AGENT_RUN_KEEP_FAILED
name/other/agent-run-keep-homes — AGENT_RUN_KEEP_HOMES
name/other/agent-run-rig — AGENT_RUN_RIG
name/other/fake-agent-hook-panic — FAKE_AGENT_HOOK_PANIC
name/other/fake-agent-pump-delay-ms — FAKE_AGENT_PUMP_DELAY_MS
name/other/foot-spawns-ndjson — foot-spawns.ndjson
name/other/harness-tied-test-child — HARNESS_TIED_TEST_CHILD
name/other/hook-notification — hook notification
name/other/hook-permission-request — hook permission-request
name/other/hook-post-tool-use — hook post-tool-use
name/other/hook-post-tool-use-failure — hook post-tool-use-failure
name/other/hook-pre-tool-use — hook pre-tool-use
name/other/hook-session-end — hook session-end
name/other/hook-session-start — hook session-start
name/other/hook-stop — hook stop
name/other/hook-stop-failure — hook stop-failure
name/other/hook-user-prompt-submit — hook user-prompt-submit
name/other/ledger-verify-lock — ledger/verify.lock
name/other/mcp-serve-test-child — MCP_SERVE_TEST_CHILD
name/other/mcp-tool-answer — answer
name/other/mcp-tool-last — last
name/other/mcp-tool-list — list
name/other/mcp-tool-send — send
name/other/mcp-tool-wait — wait
name/other/root-tied-test-child — ROOT_TIED_TEST_CHILD
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
