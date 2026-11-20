# architecture · the rows that bind 2026-11-20-a-new-cli-version-stamped-automatically — picked 313 of 968 (191 by every draw) · rule any · pin 3437

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/no-daemon-disk-is-shared-truth · No daemon; the disk is the shared truth — Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.
decision/human-always-wins-wheel · The human always wins the wheel — Human keystrokes are never blocked, refused or delayed past the current atomic paste; automation is refused with a typed reason instead of competing with the human.
decision/send-confirmed-after-the-fact · Every send confirmed after the fact — A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash. · why: A turn boundary per the hooks does not prove the input box is ready. · except: A listed local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI returns `ok`, `confirmed: false`, `unconfirmable`.

## stack-and-technologies · ## Stack and Technologies
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/chrono-timestamps · chrono RFC 3339 UTC timestamps — Timestamps are RFC 3339 UTC with millisecond precision, through chrono. · why: already in the tree through rmcp
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/no-database · No database: ndjson logs and JSON snapshots — State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
decision/pinned-copy-rehash-before-reuse · Pinned copy re-hashed before every reuse — An existing pinned copy is re-hashed before every reuse.
decision/run-never-reads-screen-content · run never reads screen content — run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures and the rig-only first-screen record of whole scrubbed rows.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written

## established-decisions · ## Established Decisions
decision/browser-tools-prompt-ledger-row · Browser-tools-prompt ledger row and its readings — The browser-tools-prompt relied ledger row reads pass when some verify run's first screen is the CLI's browser-tools prompt, absent when no run met it, and fail when any first screen reads none; it is the one relied row whose absent counts toward verified.
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/confirm-window-compiled · Confirmation window compiled — The confirmation window is a compiled 10 s, never in config.json, validated per CLI version by the confirm-window ledger row; run reads no window number from the stamp. · except: A /compact send waiting for its post-condition has its own 40 s window.
decision/driver-release-deterred-not-enforced · Release from a driver deterred, not enforced — A `release` whose params carry a string `from` is refused, which deters a driver; a driver can still run CLI `viola release` outside a wrapped session, because v1 trusts every process of the same OS user. · why: `from` is self-reported, so the refusal deters rather than enforces.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/gate-timings-compiled · Readiness gate timings compiled — The quiet period (300 ms) and the maximum wait (8.5 s) are compiled built-ins, validated per CLI version by the stamped quiet-period ledger row. · why: The maximum wait covers the CLI's 8.0 s paste hint.
decision/human-controls-cli-only · Human wheel and link controls are CLI-only — `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface. · why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/local-command-context-probe · Local command context ledger row and its probe — The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window; verify probes it by Run B's eighth paste, /context, and a failing read of it is fail, never absent. · why: Ratified as a widening of verify's pastes.
decision/local-command-echo-probe · Local command echo ledger row and its probe — The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes on a new stdout tag with no UserPromptSubmit within the confirmation window; with no readable transcript the paste is skipped and the row fails. · why: Ratified as a widening of verify's pastes.
decision/ndjson-logs-and-atomic-snapshots · ndjson append logs plus atomic JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth. · why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
decision/partial-gate-on-unverified-cli · Partial readiness gate on an unverified CLI — On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
decision/pause-link-unlink-cli-only · pause, link and unlink are CLI-only — `pause`, `link` and `unlink` are offered only on the CLI, never as MCP tools. · why: They are human controls over the wheel and the link topology.
decision/permission-request-question-answered · A question first raised by PermissionRequest is answered — A question first raised by PermissionRequest with no matching armed dialog is held for the driver; the answer returns as PermissionRequest allow with updatedInput, the tool input plus answers and annotations; a human key, an expired hold or an unverified CLI leave it to the human. · why: Ratified to end the rule that left such a question null; the body now has its own relied ledger row and verify probe. · except: Unmeasured: a user argument to viola run selecting the CLI's strict session kind, which refuses a changed input; the human then sees a denied tool call and no dialog.
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/private-mode-before-write · File mode set before any byte is written — A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/release-cli-only · release is CLI-only — `release` is never an MCP tool; only the CLI offers it. · why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
decision/revive-in-place · Revive resumes in place through the start path — A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated.
decision/revive-program-resolution · Revive resolves the program from its own PATH — Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola. · why: Ratified as a boundary widening.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, on a verified CLI writes the trust record of an untrusted spawn folder, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/screen-rows-read-only-by-verify · Screen row text read only by verify — The screen model has one row-text exit, and only viola verify reads it.
decision/separate-lock-files · Locks on separate .lock files — State locks are std `File::lock` on separate `.lock` files, opened for write, so no file is both appended to and exclusively locked. · why: A layout chosen while Windows, where append plus exclusive lock fails, was a target, and kept.
decision/start-pins-the-bytes-it-checked · Start pins the very bytes it checked — A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/trust-record-ledger-row · Trust-record ledger row and its probe — The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box. · why: Ratified as a widening of verify's child set.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/verify-transcripts-accepted-residual · Verify's CLI transcripts are an accepted residual — The CLI session transcripts and empty project dirs viola verify's runs leave under the user's Claude projects directory are an accepted residual.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
contract/claude-cli-version-output/first-line · Claude CLI --version output · first line — Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
contract/send-results/unconfirmable · send results · unconfirmable — A listed local command whose post-condition is none, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
contract/viola-revive-json-error-codes/internal-error · viola revive --json error codes · internal-error — A claude not on PATH or an anyhow error of the start.
contract/viola-revive-json-error-codes/pinned-hash-mismatch · viola revive --json error codes · pinned-hash-mismatch — The pinned copy no longer matches its hash.
contract/viola-revive-json-error-codes/strict-modes-failed · viola revive --json error codes · strict-modes-failed — A strict-modes check refused the start.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|absent|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.
contract/viola-verify-refusal-codes/interrupted · viola verify process-exit refusal codes · interrupted — Written on the process-exit verify logs under a parsed VIOLA_NAME when verify is interrupted.
contract/viola-verify-refusal-codes/pinned-hash-mismatch · viola verify process-exit refusal codes · pinned-hash-mismatch — Written on the process-exit verify logs under a parsed VIOLA_NAME when the pinned copy's hash does not match.
contract/viola-verify-refusal-codes/program-not-found · viola verify process-exit refusal codes · program-not-found — Written on the process-exit verify logs under a parsed VIOLA_NAME when the program is not found.
contract/viola-verify-refusal-codes/strict-modes-failed · viola verify process-exit refusal codes · strict-modes-failed — Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
contract/viola-verify-refusal-codes/version-unreadable · viola verify process-exit refusal codes · version-unreadable — Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.
decision/chrono-utc-millisecond-timestamps · chrono timestamps, RFC 3339 UTC, milliseconds — chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision. · why: chrono is already in the tree through rmcp.
decision/compact-confirm-window · /compact confirmation window — A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency. · provisional
decision/exclusive-endpoint-bind · Exclusive endpoint bind — A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name. · why: So two concurrent starts cannot both win.
decision/explicit-wheel-release · Explicit wheel release — The wheel returns from the human to the driver only through `viola release`.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/human-interrupt-moves-wheel · Human interrupt moves the wheel — A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state. · why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
decision/human-prompt-confirms-wheel-take · Human prompt confirms the wheel take — An unsent prompt the hook filed as `human` moves the wheel before its line is appended; a `harness` prompt and the in-flight driver prompt never move it. · except: On a verified CLI, an unsent human-filed prompt with no key typed since the last human-filed prompt and a scheduled wake counted at the last turn end is a fired wake: relabelled harness, wheel kept.
decision/local-command-echo-confirmation · Local command confirmed by its stdout echo — A local command whose post-condition is a stdout echo is confirmed by counting `<local-command-stdout>` tags in the session transcript past its end recorded before the paste; only a count crosses, never content, and the read is bounded by MAX_FRAME. · why: Ratified; the read opens no symlink, only a regular file owned by the own euid.
decision/no-model-api-calls · No model API calls — The binary makes no model API calls; the model is reached only through the unmodified `claude` CLI. · why: D2, subscription only through the unmodified `claude` CLI.
decision/no-start-over-live-instance · No start over a live or stale instance — A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start.
decision/no-typing-into-trust-dialogs · viola types into no trust dialog — viola types into neither the folder trust dialog nor the external CLAUDE.md imports dialog.
decision/pinned-copy-rehash · Pinned copy re-hashed before reuse — An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
decision/pinned-exe-copy · Pinned exe copy keyed by version and content hash — At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls. · why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
decision/revive-reads-snapshot-once · Revive reads the snapshot once — A revive reads the snapshot once through the replay for both its holder check and its recorded directory, so a replayed snapshot makes one replay and logs one state-recovered line. · why: The cwd-missing revive test expects one replay and one state-recovered line in the role file.
decision/revive-refuses-live-session · Revive refuses a session a live claude holds — viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open). · why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
decision/revive-resumes-in-place · Revive resumes a gone instance in place — viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch, and its first record is wheel with holder driver and cause start, as every start's. · why: Ratified: a revived start keeps the wheel at driver.
decision/run-refuses-held-name · A start refuses a held name — viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
decision/screen-signatures-compiled · Screen signatures as compiled literals — Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when no row is a standing modal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char. · why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read. · except: A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses.
decision/send-waits-out-paste-hint · Send waits out the paste hint — On a verified CLI a send issued while the paste hint stands waits in the readiness gate for the input box, bounded at 8.5 s; a hint that outlasts the bound ends not-delivered / input-not-ready with nothing typed. · why: Ratified ruling.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/socket-dir-refused-covers-create-and-check · Socket-dir create failure and refusal read the same — A failed socket-directory create and a refused check both refuse the start as socket-dir-refused with exit 1. A failed bind does not: it reads squatted-name when another process holds the endpoint and internal-error otherwise. · why: Machine readers branch on the socket-dir-refused start document and exit line, and the cross exit-cause test pins it at exit 1 for a start.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
decision/vt100-readiness-gate · vt100 as a pre-send readiness gate only — vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no standing modal and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model. · why: CLI-native modals bypass every hook, and the screen lags the hooks. · except: The test-only live rig in viola-e2e also uses vt100 as the screen model it reads its child's trust dialog, selector and input box from.
decision/wheel-starts-with-driver · The wheel starts with the driver — The wheel starts with `driver` at every start of `viola run`, so automation is accepted until the first human editing key, a key the human types into the folder-trust dialog before the session starts included.
contract/channel-release/release-from-driver · Wrapper channel release method · release-from-driver — A `release` whose `params` carry a string `from` is refused with `-32602` and `data.reason` `release-from-driver`.
contract/not-delivered-details/input-not-ready · not-delivered refusal details · input-not-ready — A screen not ready at the readiness gate's bound is refused `not-delivered` with this detail, nothing typed.
contract/send-results/input-not-ready · send results · input-not-ready — A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a standing modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
contract/viola-revive-json-error-codes/already-live · viola revive --json error codes · already-live — The name is held by a live instance.
contract/viola-revive-json-error-codes/cwd-missing · viola revive --json error codes · cwd-missing — The recorded directory is missing.
contract/viola-revive-json-error-codes/no-session · viola revive --json error codes · no-session — No logged session to revive.
contract/viola-revive-json-error-codes/session-live · viola revive --json error codes · session-live — The logged session is held by a live claude.
contract/viola-revive-json-error-codes/socket-dir-refused · viola revive --json error codes · socket-dir-refused — The socket directory was refused or could not be created.
contract/viola-revive-json-error-codes/squatted-name · viola revive --json error codes · squatted-name — The endpoint name is already taken at bind.
contract/viola-revive-json-error-codes/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The name is held by a stale instance.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — A revive whose recorded cwd names no existing directory is refused cwd-missing.

## conventions · ## Conventions
decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
decision/cli-flag-and-subcommand-style · CLI flag and subcommand style — CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/nutype-errors-not-folded · nutype validation errors stay out of crate enums — A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`. · except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/retired-exit-codes-never-reused · Retired exit codes never reused — A retired CLI exit code is never reused or renumbered: its number stays unassigned.
decision/silent-roles-at-internal-error-catch · Run, revive, mcp and hook silent at the catch site — Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
contract/channel-methods/hook-dialog · channel methods · hook.dialog — A request with an integer JSON-RPC id, monotonic per connection; `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-json-error-document/shape · CLI --json error document · shape — One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
contract/refusal-reason/unverified-cli · RefusalReason · unverified-cli — Serialised kebab-case; its detail is null.
contract/run-start-refusals/home-tree · viola run and revive start refusals · home tree — A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
contract/run-start-refusals/socket-dir-refused · viola run and revive start refusals · socket-dir-refused — A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
contract/verify-process-exit/interrupted · viola verify process-exit detail · interrupted — The code verify's process-exit carries when stopped before its rows are checked; a failing row is no refusal and carries no code.
contract/verify-process-exit/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — The code verify's process-exit carries when the pinned copy fails its re-hash.
contract/verify-process-exit/program-not-found · viola verify process-exit detail · program-not-found — The code verify's process-exit carries when the claude CLI is not found.
contract/verify-process-exit/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — The code verify's process-exit carries on a strict-modes refusal.
contract/verify-process-exit/unclean-fixture · viola verify process-exit detail · unclean-fixture — The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
contract/verify-process-exit/version-unreadable · viola verify process-exit detail · version-unreadable — The code verify's process-exit carries when the CLI version cannot be read.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.
contract/verify/no-json · viola verify · --json — viola verify takes no --json.
decision/control-characters-refused · Control characters refused, never stripped — Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/ndjson-line-discipline · ndjson line discipline — Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
decision/one-error-enum-per-crate · One error enum per crate — Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages. · except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
decision/viola-name-instance-key · ViolaName as the instance key — The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype. · why: No database, so no UUID or serial keys.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/channel-response/refusal · channel response · refusal — A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/cli-internal-error/output · CLI internal error output · internal error line — A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
contract/code-word/format · Code word · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/refusal-order/answer · Refusal order · answer — control-character on the response's free text, then human-typing, unverified-cli, then unknown-dialog.

## standard-contracts · ## Standard Contracts
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/no-document-after-child-held-terminal · No JSON document once the child held the terminal — An error after the child has held the terminal prints no JSON document. · why: The child has held the terminal.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/stamps-merge-own-version-only · Stamps merge replaces only its own version — A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row counts — A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
contract/cli-json-output/error · CLI `--json` output · `error` — Outcomes that have no channel `result` print an `error` document.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/cli-json-output/revive-refusal · CLI `--json` output · `viola revive --json` refusal — A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
contract/cli-json-output/revive-refusal-codes · CLI `--json` output · `viola revive --json` refusal codes — `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
contract/instance-snapshot/cli-verified · instance snapshot · cli_verified — `cli_verified` comes from `run`'s version gate.
contract/instance-snapshot/cli-version · instance snapshot · cli_version — `cli_version` comes from `run`'s version gate.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
contract/instance-snapshot/spawn-order · instance snapshot · spawn order — The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/revive-json/program-not-found · viola revive --json · program not found — A program not found prints the `internal-error` document, before any child is spawned.
contract/revive-json/pump-error · viola revive --json · error after the child holds the terminal — An error while pumping the child prints no document, because the child has held the terminal.
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
contract/trust-writer-words/key-present · Trust writer code words · key-present — The config already holds a record under the key; it is left as found and nothing is written. Only viola verify's writer gives it; a start reads it as lost.
contract/trust-writer-words/removed · Trust writer code words · removed — The written trust record was removed again, every other key of the config left as found. Only viola verify gives it; a start reads it as lost.
contract/trust-writer-words/written · Trust writer code words · written — The trust record was written into the Claude config file.
decision/revive-refused-without-cwd · Revive over a replayed snapshot refused — A replayed snapshot gives no cwd, so a revive over one is refused cwd-missing.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
contract/channel-32602-cause/release-from-driver · channel -32602 causes · release-from-driver — One of the three causes a -32602 fault reads.
contract/channel-methods/plain-release · channel methods · plain release — A `release` on a driver-held wheel changes nothing.
contract/channel-methods/release-from-driver · channel methods · release from driver — A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first.
contract/cli-exit-21-document/strict-modes-failed · exit-21 CLI document · strict-modes-failed — A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line.
contract/cli-json-output/instance-unreachable · CLI `--json` output · `instance-unreachable` — `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
contract/cli-json-output/result · CLI `--json` output · channel result — Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
contract/event-data/send-refused · event data · `send-refused` — `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
contract/instance-snapshot/cwd · instance snapshot · cwd — `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its one reader is `viola revive`, which uses it as the revived child's spawn directory and as the folder its start's trust record is keyed by.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
contract/ledger-stamps/first-screen · ledger stamps envelope · measured.first_screen — For each of the six interactive verify runs, one closed word naming its first settled screen: trust, imports, browser-tools, box, none or exited; no screen text.
contract/ledger-stamps/first-settle-ms · ledger stamps envelope · measured.first_settle_ms — For each of the six interactive verify runs, the time to its first settled screen in milliseconds, a whole number.
contract/trust-writer-words/config-absent · Trust writer code words · config-absent — The Claude config file does not exist; it is never created, nothing is written and Run F starts no child.
contract/trust-writer-words/config-dir-unusable · Trust writer code words · config-dir-unusable — The kept CLAUDE_CONFIG_DIR gives no usable folder for the Claude config file; nothing is written.
contract/trust-writer-words/config-foreign · Trust writer code words · config-foreign — The Claude config file belongs to another owner; nothing is written.
contract/trust-writer-words/config-link · Trust writer code words · config-link — The Claude config path is a symbolic link; it is refused and left untouched.
contract/trust-writer-words/config-malformed · Trust writer code words · config-malformed — The Claude config file is no JSON document of the expected shape; nothing is written.
contract/trust-writer-words/config-not-regular · Trust writer code words · config-not-regular — The Claude config path is not a regular file; it is refused and left untouched.
contract/trust-writer-words/config-oversize · Trust writer code words · config-oversize — The Claude config file is over the frame cap; nothing is written.
contract/trust-writer-words/config-unreadable · Trust writer code words · config-unreadable — The Claude config file could not be read; nothing is written.
contract/trust-writer-words/home-unknown · Trust writer code words · home-unknown — No user home is known to locate the Claude config file; nothing is written.
contract/trust-writer-words/lock-held · Trust writer code words · lock-held — The config file's lock is held or cannot be made; nothing is written.
contract/trust-writer-words/lost · Trust writer code words · lost — Three attempts each failed to replace the config file or to read the change back.
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
contract/wrapper-event-appends/send-records · wrapper event appends · `send-issued` / `send-confirmed` / `send-refused` — The wrapper appends the send records with `source: wrapper` for each send.

## occupied-resources · ## Occupied Resources
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that verify reads.
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/config-bad-value-falls-back · Bad config values fall back to defaults — An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value makes the dialog hold zero or unbounded.
decision/config-falls-back-to-defaults · Unusable configuration falls back to defaults — config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line. · why: so no value makes the dialog hold zero or unbounded
decision/config-keys-match-schema · Config keys equal the config schema — The set of configuration keys the product knows equals the set of properties of the tolerant config.json schema, which admits additional properties. · guard: config_keys_are_the_config_schema_properties
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/logged-session-id-on-child-argv · Logged session id on the child's command line — A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn. · why: A ratified crossing, chosen over allowing only an id the human types.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined.
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/child-argv/caller-arguments · flags passed to the child · caller's arguments — The caller's arguments follow the viola flags.
contract/child-argv/fork-session · flags passed to the child · `--fork-session` — A revived start adds `--fork-session` after `--resume <id>` under `--fork`.
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
contract/child-argv/resume · flags passed to the child · `--resume` — A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
contract/child-environment/path · The wrapped child's environment · PATH — Prefixed by viola run with the pinned copy's folder.
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/claude-child-argv/fork-session · the claude child's argv · --fork-session — --fork-session follows --resume when --fork is given; the words after -- come last.
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it.
contract/config-json/diagnostics-level · config.json · diagnostics_level — "info" by default, or "debug"; diagnostics_level is the only source of the process-log level, and RUST_LOG has no effect.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and the caller's VIOLA_NAME for list, verify, revive --list and plugin install.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — The detail of a stopped run; a failing ledger row is no refusal and carries no code.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — A refusal code verify writes under a parsed VIOLA_NAME.
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — A refusal code verify writes under a parsed VIOLA_NAME.
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — A refusal code verify writes under a parsed VIOLA_NAME.
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude; a hidden --record-first-screens, absent from --help, is passed only by the live rig.
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
contract/viola-verify-exit-codes/1 · viola verify exit codes · 1 — The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.
decision/child-env-claude-strip · CLAUDE names stripped from the child's environment — Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it. · except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says. · except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
decision/end-to-end-homes-kept-after-each-job · Homes kept after each job's end-to-end run — Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out. · why: The homes are kept in CI for the obs gates.
decision/identity-floor-eleven-names · Identity floor of eleven names — The identity floor is CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT and CLAUDE_PID. · why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited.
decision/no-deletion-of-pinned-copies · Pinned copies and plugin folders never deleted — v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses. · why: so any later cleanup can skip copies that a live or stale instance still calls
decision/plugin-env-vars-unused · Plugin-provided environment variables unused — viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary, and reads none of those or CLAUDE_ENV_FILE and CLAUDE_PROJECT_DIR.
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
contract/child-environment/viola-bin · The wrapped child's environment · VIOLA_BIN — Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded.
contract/config-json/v · config.json · v — config.json carries v, which must be 1; the file is read with a MAX_FRAME cap.
contract/events-ndjson/byte-offsets · events.ndjson · byte offsets — The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
contract/frame-verb-exit-detail/socket-dir-refused · CLI frame verb process-exit detail · socket-dir-refused — Exit 21 with during connect, at WARN, no channel-request line; run and revive's start write it with exit_code 1.
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only.
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only.
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
contract/viola-revive-exit-codes/0 · viola revive exit codes · 0 — The revived child's wrapper ended as viola run's does.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/cli-verbs-human-stderr · Short-lived CLI verbs may use stderr — Short-lived CLI verbs may use stderr for their human output.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/config-precedence · Configuration precedence — CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/diagnostics-level-sole-log-level · diagnostics_level as the sole log level source — The config key diagnostics_level is the only source of the process-log level; RUST_LOG has no effect.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
contract/config-json/unknown-keys · config.json · unknown keys — config.json is parsed tolerantly: unknown keys are skipped and counted.
decision/agents-join-read-ungated · Agents join read not gated on a stamp — viola list and viola revive do not gate their claude agents --json read on a ledger stamp. · why: The read only enriches and its failure is never an error.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI is reported ok, confirmed false, unconfirmable.

## project-intent · ## Project Intent
decision/modular-monolith-no-daemon · Modular monolith, no daemon — The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
decision/new-capabilities-as-verbs · New capabilities as subcommands or channel verbs — New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.

## inherited-defaults · ## Inherited Defaults
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.
decision/clap-for-cli · clap parses the CLI — clap is what parses the CLI.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
decision/one-shared-persist-helper · Snapshots persisted through one shared helper — Atomic JSON snapshots are written with tempfile persist through one shared helper.
contract/channel-result/refusal · Channel result · refusal — A refusal is a typed RefusalReason in result.refusal.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.

## build-system · key of ## Infrastructure Patterns
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/serde-json-recursion-limit · serde_json keeps its recursion limit — The serde_json unbounded_depth feature is banned. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: probe row serde_json-unbounded-depth
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/github-actions-ci · GitHub Actions CI — CI runs on GitHub Actions: one push and pull-request pipeline on ubuntu, and a weekly scheduled workflow.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.

## deployment-model · key of ## Infrastructure Patterns
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.

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
