# architecture · the rows that bind 2026-11-18-the-2-1-325-recorded-fixture-set — picked 284 of 967 (149 by every draw) · rule any · pin 2816

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/send-confirmed-after-the-fact · Every send confirmed after the fact — A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash. · why: A turn boundary per the hooks does not prove the input box is ready. · except: A listed local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI returns `ok`, `confirmed: false`, `unconfirmable`.
decision/mechanism-not-policy · Mechanism, not policy: a general bridge — viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb. · why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.

## stack-and-technologies · ## Stack and Technologies
decision/mcp-server-rmcp · rmcp is the MCP server — The MCP server is an rmcp stdio server with schemars tool input schemas, rmcp minor pinned, a thin adapter over viola-channel.
decision/run-never-reads-screen-content · run never reads screen content — run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures and the rig-only first-screen record of whole scrubbed rows.
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/vt100-screen-model · vt100 as screen model — vt100 gives pre-send readiness and modal detection on run's feed thread, the screen of verify's interactive probe runs, and the screen the test-only live rig reads of its child.
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/e2e-no-rmcp-child-transport · E2E owns the viola mcp child — The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit. · why: transport-child-process kills the server 3 s after a close
decision/pinned-copy-placed-once · Pinned copy placed once, never replaced — The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
decision/pinned-copy-rehash-before-reuse · Pinned copy re-hashed before every reuse — An existing pinned copy is re-hashed before every reuse.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key

## established-decisions · ## Established Decisions
decision/browser-tools-prompt-ledger-row · Browser-tools-prompt ledger row and its readings — The browser-tools-prompt relied ledger row reads pass when some verify run's first screen is the CLI's browser-tools prompt, absent when no run met it, and fail when any first screen reads none; it is the one relied row whose absent counts toward verified.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; the fake agent and rig read trust records.
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/compact-confirm-window · /compact confirmation window — A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency. · provisional
decision/confirm-window-compiled · Confirmation window compiled — The confirmation window is a compiled 10 s, never in config.json, validated per CLI version by the confirm-window ledger row; run reads no window number from the stamp. · except: A /compact send waiting for its post-condition has its own 40 s window.
decision/every-send-confirmed · Every send is confirmed — A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted. · why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate. · except: A ledger-listed local command is confirmed by its own measured post-condition, such as a stdout echo in the session transcript, or answered ok unconfirmable where none can be read.
decision/exec-form-command-hooks · Exec-form command hooks — Hooks are exec-form `command` hooks running the pinned viola copy with `hook <event>` as arguments. · why: Exec form skips shell startup; `http` hooks need a fixed URL and so a daemon; `mcp_tool` is unavailable at SessionStart.
decision/exec-form-hooks · Exec-form command hooks — Hooks are exec-form `command` hooks naming the pinned viola copy with args `hook <event>`. · why: Exec form skips shell startup.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/gate-timings-compiled · Readiness gate timings compiled — The quiet period (300 ms) and the maximum wait (8.5 s) are compiled built-ins, validated per CLI version by the stamped quiet-period ledger row. · why: The maximum wait covers the CLI's 8.0 s paste hint.
decision/global-statusline-untouched · Global statusline config untouched — The user's global statusline config is never touched.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arm, called only by `viola verify`'s probes, reads no `VIOLA_*` and opens no channel, still exiting 0 with empty stderr.
decision/local-command-echo-confirmation · Local command confirmed by its stdout echo — A local command whose post-condition is a stdout echo is confirmed by counting `<local-command-stdout>` tags in the session transcript past its end recorded before the paste; only a count crosses, never content, and the read is bounded by MAX_FRAME. · why: Ratified; the read opens no symlink, only a regular file owned by the own euid.
decision/local-command-echo-probe · Local command echo ledger row and its probe — The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes on a new stdout tag with no UserPromptSubmit within the confirmation window; with no readable transcript the paste is skipped and the row fails. · why: Ratified as a widening of verify's pastes.
decision/local-command-post-conditions · Local commands listed with delivery post-conditions — The ledger lists each known built-in local command with its delivery post-condition or none; skills are not local commands. · why: Built-in local commands fire no UserPromptSubmit, while skills expand into a prompt and fire it.
decision/local-commands-by-ledger · Local commands classified by the ledger — A send is a local command only when its typed text is exactly a name on the ledger's local-command list, or that name, one space and a non-empty rest not starting with a space, never by a leading slash; any other slash text, skills included, is an ordinary send. · why: Local commands fire no UserPromptSubmit and would otherwise be false negatives; /clear matters most as drivers clear between skills.
decision/no-probe-for-unrelied-shapes · No probe for a shape no send relies on — A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only. · why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
decision/no-start-over-live-instance · No start over a live or stale instance — A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start.
decision/no-typing-into-trust-dialogs · viola types into no trust dialog — viola types into neither the folder trust dialog nor the external CLAUDE.md imports dialog.
decision/partial-gate-on-unverified-cli · Partial readiness gate on an unverified CLI — On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
decision/pinned-copy-never-replaced · Pinned copy never replaced — The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
decision/pinned-exe-copy · Pinned exe copy keyed by version and content hash — At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls. · why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
decision/plugin-only-in-wrapped-sessions · Plugin loads only into wrapped sessions — The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`. · why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola. · except: `viola verify`'s transient unwrapped children, all gone when verify ends.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/revive-program-resolution · Revive resolves the program from its own PATH — Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola. · why: Ratified as a boundary widening.
decision/run-refuses-held-name · A start refuses a held name — viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, on a verified CLI writes the trust record of an untrusted spawn folder, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/screen-rows-read-only-by-verify · Screen row text read only by verify — The screen model has one row-text exit, and only viola verify reads it.
decision/screen-signatures-compiled · Screen signatures as compiled literals — Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when no row holds a modal literal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char. · why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read.
decision/send-waits-out-paste-hint · Send waits out the paste hint — On a verified CLI a send issued while the paste hint stands waits in the readiness gate for the input box, bounded at 8.5 s; a hint that outlasts the bound ends not-delivered / input-not-ready with nothing typed. · why: Ratified ruling.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/trust-record-ledger-row · Trust-record ledger row and its probe — The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box. · why: Ratified as a widening of verify's child set.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/verify-box-wait-for-literal · Verify waits for an input-box literal after added turns — After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding the input box (a footer literal or the box's own rows) or a modal literal, a poisoned screen, or the probe deadline; any other quiet screen keeps waiting. · why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-never-types-into-dialog · viola verify never answers a CLI dialog — No `viola verify` run writes a byte into a CLI dialog.
decision/verify-own-trust-record · Verify writes and removes its own trust record — viola verify writes a trust record for its own temp folder only into the Claude config file its child reads, never creating an absent file, and removes that one record after. · why: Ratified as the one bounded exception to viola writing nothing into Claude's config.
decision/verify-plan-files-in-run-dir · Plan-mode probe keeps plan files in its own dir — The plan-mode verify run points plansDirectory at a 0700 plans dir inside its own 0700 run dir. · why: So the plan file lands inside the run's own dir and none under the user's own plans directory.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content. · except: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/verify-transcripts-accepted-residual · Verify's CLI transcripts are an accepted residual — The CLI session transcripts and empty project dirs viola verify's runs leave under the user's Claude projects directory are an accepted residual.
decision/verify-types-nothing-into-dialogs · Verify never types into a CLI dialog — No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key. · why: Ratified rulings.
decision/vt100-readiness-gate · vt100 as a pre-send readiness gate only — vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model. · why: CLI-native modals bypass every hook, and the screen lags the hooks. · except: The test-only live rig in viola-e2e also uses vt100 as the screen model it reads its child's trust dialog, selector and input box from.
contract/claude-cli-version-output/first-line · Claude CLI --version output · first line — Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
contract/hook-tiers/ordering-spine · Hook tiers · sync with no reply body (the ordering spine) — SessionStart, UserPromptSubmit, Stop and StopFailure each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
contract/hook-tiers/sync-spine · hook tiers · sync spine — SessionStart, UserPromptSubmit, Stop and StopFailure run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
contract/mcp-tools/answer · MCP tools · answer — One of the five MCP tools; takes `{target, dialog_id, response}`, with `response` in the channel `answer` shape.
contract/mcp-tools/last · MCP tools · last — One of the five MCP tools; takes `{target}`.
contract/mcp-tools/list · MCP tools · list — One of the five MCP tools; takes `{}` and shows the driver the sessions.
contract/mcp-tools/send · MCP tools · send — One of the five MCP tools; takes `{target, text}`, `target` a `ViolaName`.
contract/mcp-tools/wait · MCP tools · wait — One of the five MCP tools; takes `{target, after?, timeout_ms?}`.
contract/probe-capture/payload-file · Hook probe capture directory · `<PascalEvent>.<k>.json` — Each captured payload is claimed exclusively at 0600 under the first free `k`, which counts across events, so names sort in arrival order and two captures never share a name.
contract/send-results/unconfirmable · send results · unconfirmable — A listed local command whose post-condition is none, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|absent|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.
contract/viola-verify-refusal-codes/program-not-found · viola verify process-exit refusal codes · program-not-found — Written on the process-exit verify logs under a parsed VIOLA_NAME when the program is not found.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
contract/viola-verify-refusal-codes/version-unreadable · viola verify process-exit refusal codes · version-unreadable — Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/clear-confirmed-by-new-session · /clear confirmed by a new session — /clear on a verified CLI is confirmed by a session-start of cause clear whose agent session id differs from the remembered one, and by no prompt-submitted.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/compact-confirmed-by-same-session · /compact confirmed by the same session — /compact on a verified CLI is confirmed by a session-start of cause compact whose agent session id equals the remembered one, and by no prompt-submitted.
decision/hooks-and-mcp-never-search-path · Hooks and MCP never search PATH — Hook and MCP commands are the pinned copy's absolute path substituted at start, never a PATH lookup. · why: PATH-found hooks run a different binary from the wrapper after a rebuild.
decision/human-controls-cli-only · Human wheel and link controls are CLI-only — `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface. · why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/mcp-server-on-rmcp · MCP server on rmcp over stdio — The MCP server viola serves to drivers is built on the rmcp SDK over stdio, pinned to one minor version. · why: rmcp is the official SDK and tracks the MCP spec; the pinned minor limits churn from its near-weekly releases.
decision/pause-link-unlink-cli-only · pause, link and unlink are CLI-only — `pause`, `link` and `unlink` are offered only on the CLI, never as MCP tools. · why: They are human controls over the wheel and the link topology.
decision/pinned-copy-no-clobber-placement · Pinned copy placed no-clobber with the handle closed — The pinned copy's absent-copy write is never a replace: a no-clobber placement that closes the write handle before the rename and leaves an existing copy untouched. · why: A rename with the write handle still open makes a concurrent first start's exec of the copy fail ETXTBSY.
decision/pinned-copy-rehash · Pinned copy re-hashed before reuse — An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
decision/plugin-compiled-into-binary · Plugin compiled into the binary — viola is installed by `cargo install --path .`, with the plugin compiled into the binary and the plugin files rewritten from the binary on every start. · why: The binary and the plugin can no longer drift apart.
decision/poisoned-screen-model · Poisoned screen model reads not ready — A caught vt100 panic or a dropped feed message poisons the screen model until the host size changes, and a poisoned model reads input-not-ready. · why: A dropped byte desynchronises the screen.
decision/prompt-normalise-order · Classify before unwrapping and un-escaping — Prompt normalisation classifies harness on the raw prefix first, then removes any long-paste wrapper with its frame, then un-escapes a backslash after < before an ASCII letter or /. · why: So a typed pair is never unwrapped and delivery matching compares plain text.
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or --file — Prompt text comes from stdin or `--file`, never from a leading-slash argument.
decision/r8-identity-floor · R8 parent-identity floor — The measured CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
decision/release-cli-only · release is CLI-only — `release` is never an MCP tool; only the CLI offers it. · why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
decision/revive-in-place · Revive resumes in place through the start path — A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated.
decision/revive-reads-snapshot-once · Revive reads the snapshot once — A revive reads the snapshot once through the replay for both its holder check and its recorded directory, so a replayed snapshot makes one replay and logs one state-recovered line. · why: The cwd-missing revive test expects one replay and one state-recovered line in the role file.
decision/revive-refuses-live-session · Revive refuses a session a live claude holds — viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open). · why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
decision/revive-resumes-in-place · Revive resumes a gone instance in place — viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch, and its first record is wheel with holder driver and cause start, as every start's. · why: Ratified: a revived start keeps the wheel at driver.
decision/rmcp-stdio-mcp-server · rmcp stdio MCP server — rmcp, its minor pinned, is what serves MCP, over stdio. · why: The official SDK.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/start-pins-the-bytes-it-checked · Start pins the very bytes it checked — A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
decision/submitted-form-prompt-match · Prompt match in the submitted form after agent normalisation — The typed text matches prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and frame and reverses tag escaping, when equal or when non-empty and equal to the typed text less its trailing run of ASCII space, TAB, CR and LF. · why: The CLI trims only a trailing run at submit: it keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps an inner run of spaces and an inner LF.
decision/tempfile-snapshot-writer · tempfile as the snapshot writer — Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced. · why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
contract/send-results/input-not-ready · send results · input-not-ready — A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
contract/viola-verify-refusal-codes/pinned-hash-mismatch · viola verify process-exit refusal codes · pinned-hash-mismatch — Written on the process-exit verify logs under a parsed VIOLA_NAME when the pinned copy's hash does not match.
contract/viola-verify-refusal-codes/strict-modes-failed · viola verify process-exit refusal codes · strict-modes-failed — Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.

## conventions · ## Conventions
decision/mcp-tools-list-held-to-recorded-exchange · MCP tools/list held to a recorded claude exchange — The served tools/list result is held to a recorded claude exchange's result key and type shape, its tool names in order. · guard: mcp_recorded_exchange (viola-e2e)
decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/hook-to-kind-map/sessionstart · Hook to kind map · SessionStart — SessionStart maps to `session-start`; `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
contract/hook-to-kind-map/stop · Hook to kind map · Stop — Stop maps to `turn-ended`.
contract/hook-to-kind-map/stopfailure · Hook to kind map · StopFailure — StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine TurnEndCause words, classified from the CLI's error word inside the Claude adapter.
contract/mcp-list-results/cachescope · MCP list results · cacheScope — The tools/list and server/discover results each carry cacheScope private, because claude drops every tool of a list result without it.
contract/mcp-list-results/ttlms · MCP list results · ttlMs — The tools/list and server/discover results each carry ttlMs 0, because claude drops every tool of a list result without it.
contract/normalised-event-kinds/session-start · normalised event kinds · session-start — `session-start` is a normalised event kind.
contract/normalised-event-kinds/turn-ended · normalised event kinds · turn-ended — `turn-ended` is a normalised event kind.
contract/ok-payload/unconfirmable · ok payload · unconfirmable — unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured, or is a stdout echo with no readable session transcript.
contract/turn-end-cause/cause-set · Turn-end cause · cause set — `turn-ended` has a closed cause set, TurnEndCause, in viola-core beside UnreachableCause, of nine words (rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error) with no Claude-specific name; a `turn-ended` cause is one of its words.
contract/verify-process-exit/interrupted · viola verify process-exit detail · interrupted — The code verify's process-exit carries when stopped before its rows are checked; a failing row is no refusal and carries no code.
contract/verify-process-exit/unclean-fixture · viola verify process-exit detail · unclean-fixture — The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/timestamp-field-names · Timestamp field names — Every instance event, and the `hook.event` frame, carries its own instant in the field `ts`; request frames carry no instant, process-log and detail lines carry `timestamp`, and other instants use `*_at` fields, except the start instant a link record holds.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/channel-methods/hook-event · channel methods · hook.event — A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}, transcript?}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
contract/cli-exit-codes/0 · CLI exit codes · 0 — ok.
contract/cli-exit-codes/10 · CLI exit codes · 10 — The `human-typing` refusal.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-exit-codes/13 · CLI exit codes · 13 — The `not-delivered` refusal.
contract/hook-to-kind-map/posttooluse · Hook to kind map · PostToolUse — PostToolUse maps to `activity`, which is log-only and never wakes `wait`.
contract/hook-to-kind-map/sessionend · Hook to kind map · SessionEnd — SessionEnd maps to `session-end`.
contract/hook-to-kind-map/userpromptsubmit · Hook to kind map · UserPromptSubmit — UserPromptSubmit maps to `prompt-submitted`.
contract/normalised-event-kinds/activity · normalised event kinds · activity — `activity` is a normalised event kind.
contract/normalised-event-kinds/permission · normalised event kinds · permission — `permission` is a normalised event kind.
contract/normalised-event-kinds/plan · normalised event kinds · plan — `plan` is a normalised event kind.
contract/normalised-event-kinds/prompt-submitted · normalised event kinds · prompt-submitted — `prompt-submitted` is a normalised event kind.
contract/normalised-event-kinds/question · normalised event kinds · question — `question` is a normalised event kind.
contract/normalised-event-kinds/session-end · normalised event kinds · session-end — `session-end` is a normalised event kind.
contract/verify-process-exit/program-not-found · viola verify process-exit detail · program-not-found — The code verify's process-exit carries when the claude CLI is not found.
contract/verify-process-exit/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — The code verify's process-exit carries on a strict-modes refusal.
contract/verify-process-exit/version-unreadable · viola verify process-exit detail · version-unreadable — The code verify's process-exit carries when the CLI version cannot be read.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.
contract/verify/no-json · viola verify · --json — viola verify takes no --json.
contract/viola-hook-exit-codes/0 · viola hook exit codes · 0 — Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.

## standard-contracts · ## Standard Contracts
decision/api-error-turn-end-clears-turn · API-error turn end clears the running turn — An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted. · guard: path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/mcp-list-reads-agents · MCP list reads claude agents on each call — The MCP server's `list` tool makes the same one `claude agents` read as CLI `list` on each call. · why: A ratified widening.
decision/mcp-over-stdio · MCP over stdio only — MCP speaks JSON-RPC 2.0 over stdio; the MCP server is stdio only.
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
decision/stamps-merge-own-version-only · Stamps merge replaces only its own version — A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row counts — A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
contract/event-data/session-start · event data per kind · session-start — `session-start` data is `{cause, agent_session_id}`.
contract/event-data/turn-ended-background-tasks · event data per kind · turn-ended background_tasks — `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array.
contract/event-data/turn-ended-cause · event data per kind · turn-ended cause — `cause` is present only on an API-error turn end, one of the nine TurnEndCause words, and absent on a Stop end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`.
contract/event-data/turn-ended-last-assistant-message · event data per kind · turn-ended last_assistant_message — `turn-ended` data is `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
contract/event-data/turn-ended-scheduled-wakes · event data per kind · turn-ended scheduled_wakes — `scheduled_wakes` counts the session's scheduled wakes: the Stop payload's `session_crons` array length with no element read or carried, `null` when the key is absent, `null` or not an array, and on an API-error turn end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait` only.
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
contract/hook-event/transcript · channel hook.event · transcript — An optional string param beside `event`, never inside it, sent only for a SessionStart whose transcript path passes the shape check against its own session id; the wrapper re-validates it for a session-start line only, drops any other, and it never enters event data.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/first-screen · ledger stamps envelope · measured.first_screen — For each of the six interactive verify runs, one closed word naming its first settled screen: trust, imports, browser-tools, box, none or exited; no screen text.
contract/ledger-stamps/first-settle-ms · ledger stamps envelope · measured.first_settle_ms — For each of the six interactive verify runs, the time to its first settled screen in milliseconds, a whole number.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
contract/trust-writer-words/removed · Trust writer code words · removed — The written trust record was removed again, every other key of the config left as found. Only viola verify gives it; a start reads it as lost.
contract/trust-writer-words/written · Trust writer code words · written — The trust record was written into the Claude config file.
decision/dialog-logged-exactly-once · Every dialog logged exactly once — Every dialog appears exactly once in the event log: on receipt the wrapper assigns the `dialog_id` and appends the dialog event before it can wake `wait` or reply, even when it replies `null` at once. · except: A matched continuation appends nothing, being the same dialog.
decision/replayed-snapshot-gives-no-live-endpoint · Replayed snapshot gives no live endpoint — A replayed snapshot carries no endpoint, pid or start time, so its readers treat the instance as unreachable and a start's collision check judges liveness from the heartbeat alone, refusing already-live within the stale bound. · why: Clients, hooks and the MCP tools find no endpoint through the replay, and the snapshot-replay chaos test expects a fresh beat to refuse already-live.
decision/replayed-snapshot-liveness · Replayed snapshot liveness rests on the heartbeat — For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id and wheel from the log; a field no line gave is absent, never a default.
decision/upstream-text-is-content · Upstream text is content — Upstream text carried in event `data` is content, never commands.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
contract/instance-snapshot/agent-session-id · instance snapshot · agent_session_id — `agent_session_id` is the one from the newest `session-start` event and exists only to join `claude agents --json` rows to this instance.
contract/instance-snapshot/cli-verified · instance snapshot · cli_verified — `cli_verified` comes from `run`'s version gate.
contract/instance-snapshot/cli-version · instance snapshot · cli_version — `cli_version` comes from `run`'s version gate.
contract/instance-snapshot/cwd · instance snapshot · cwd — `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its one reader is `viola revive`, which uses it as the revived child's spawn directory and as the folder its start's trust record is keyed by.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/trust-writer-words/config-absent · Trust writer code words · config-absent — The Claude config file does not exist; it is never created, nothing is written and Run F starts no child.
contract/trust-writer-words/config-dir-unusable · Trust writer code words · config-dir-unusable — The kept CLAUDE_CONFIG_DIR gives no usable folder for the Claude config file; nothing is written.
contract/trust-writer-words/config-foreign · Trust writer code words · config-foreign — The Claude config file belongs to another owner; nothing is written.
contract/trust-writer-words/config-link · Trust writer code words · config-link — The Claude config path is a symbolic link; it is refused and left untouched.
contract/trust-writer-words/config-malformed · Trust writer code words · config-malformed — The Claude config file is no JSON document of the expected shape; nothing is written.
contract/trust-writer-words/config-not-regular · Trust writer code words · config-not-regular — The Claude config path is not a regular file; it is refused and left untouched.
contract/trust-writer-words/config-oversize · Trust writer code words · config-oversize — The Claude config file is over the frame cap; nothing is written.
contract/trust-writer-words/config-unreadable · Trust writer code words · config-unreadable — The Claude config file could not be read; nothing is written.
contract/trust-writer-words/home-unknown · Trust writer code words · home-unknown — No user home is known to locate the Claude config file; nothing is written.
contract/trust-writer-words/key-present · Trust writer code words · key-present — The config already holds a record under the key; it is left as found and nothing is written. Only viola verify's writer gives it; a start reads it as lost.
contract/trust-writer-words/lock-held · Trust writer code words · lock-held — The config file's lock is held or cannot be made; nothing is written.
contract/trust-writer-words/lost · Trust writer code words · lost — Three attempts each failed to replace the config file or to read the change back.

## occupied-resources · ## Occupied Resources
decision/child-env-claude-strip · CLAUDE names stripped from the child's environment — Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it. · except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says. · except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that verify reads.
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/logged-session-id-on-child-argv · Logged session id on the child's command line — A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn. · why: A ratified crossing, chosen over allowing only an id the human types.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene · except: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/verify-probe-dirs-drop-guarded · verify probe dirs removed by drop guard — Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing. · why: its captures are transient, content-bearing raw hook payloads
decision/verify-removes-only-own-dirs · verify removes only the probe folders it created — viola verify removes its probe folders whole, and its own trust record from the Claude config file, when it passes, fails or is stopped by Ctrl-C or TERM, only what that run created, never by a pattern or a listing. · except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand.
decision/verify-run-e-resumes-run-b-first-session · Verify's Run E resumes Run B's first session — Verify's Run E resumes the session id taken from Run B's first SessionStart, only when that id has a session id's shape; with no such id Run E starts no child, so Run E depends on Run B running before it. · why: The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the six runs execute in order A to F.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/hook-capture-arm/payload-file · viola hook --capture arm · payload file — Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
contract/session-id/shape · Claude session id · shape — 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere.
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — The detail of a stopped run; a failing ledger row is no refusal and carries no code.
contract/verify-exit-detail/program-not-found · viola verify process-exit detail · program-not-found — A refusal code verify writes under a parsed VIOLA_NAME.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — A refusal code verify writes under a parsed VIOLA_NAME.
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude; a hidden --record-first-screens, absent from --help, is passed only by the live rig.
contract/viola-verify-exit-codes/1 · viola verify exit codes · 1 — The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.
decision/colour-env-decides-only-sgr · Colour variables decide only SGR output — Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read. · why: ratified: the colour variables are to be read
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on a tmpfs link — On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk. · why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
decision/e2e-home-headroom-floor · e2e-home backing headroom floor — A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key. · why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB
decision/e2e-home-owner-only-headroom · End-to-end home base checked before use — The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free.
decision/e2e-noop-fake-agent-feature · No-op fake-agent feature in the e2e crate — The test-only `viola-e2e` crate declares a no-op `fake-agent` feature. · why: So package-scoped `--features fake-agent` invocations resolve.
decision/fake-agent-feature-on-named-members · A fake-agent feature on each named member crate — The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · why: so a package-scoped --features fake-agent build (cargo-mutants) resolves · except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
decision/harness-env-unread-by-product · Harness variables never read by viola — AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED, AGENT_RUN_RIG and CI are test-harness variables that viola itself never reads.
decision/mcp-subcommand-hidden · Hidden mcp subcommand — The viola mcp subcommand is hidden from viola --help and takes no argument but the global --home. · why: The plugin runs it, never a person.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/pinned-binary-copy · Pinned copy of the binary — viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
decision/plugin-carries-no-hooks · Installed plugin carries no hooks — The plugin `viola plugin install` adds registers the MCP server `viola` in its `.mcp.json` and carries no hooks.
decision/probe-capture-arms-inert · Probe capture arms are inert — The hidden hook `--capture` arm takes an absolute, existing directory only, reads no `VIOLA_*` variable, starts no observability, opens no channel, and always exits 0 with empty stderr.
decision/run-pins-checked-bytes · Run pins the bytes it checked — viola run pins the very executable bytes its start check examined.
decision/screen-fixtures-signature-rows-only · Screen fixtures keep only signature rows — A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording. · except: The hidden rig-only --record-first-screens keeps a failed verify's first screens as whole scrubbed rows, kept locally only.
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
contract/child-argv/resume · flags passed to the child · `--resume` — A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it.
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded.
contract/hook-capture-arm/exit · viola hook --capture arm · exit status — Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened.
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only.
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — A refusal code verify writes under a parsed VIOLA_NAME.
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line.
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
contract/viola-revive-json/list · viola revive --json output · --list — viola revive <name> --list --json prints the ok.sessions document, or the no-session document with exit 1.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/hook-exits-on-untrusted-instance · Hook exits at once on an untrusted instance — viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append. · except: The --capture arm, which reads no VIOLA_* variable, runs ahead of the check.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI is reported ok, confirmed false, unconfirmable.
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/hook-fails-open · Hook fails open toward the human — Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.
contract/mcp-output/stdout · MCP server output · stdout and stderr — viola mcp writes only MCP frames to stdout; when its role file cannot be opened it exits 1 and prints nothing.

## project-intent · ## Project Intent
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; the fake agent and rig read trust records.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.
decision/rmcp-for-mcp · rmcp serves MCP — rmcp is what serves MCP.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; the fake agent and rig read trust records.

## deployment-model · key of ## Infrastructure Patterns
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.

## project-directory-structure · key of ## Infrastructure Patterns
decision/zero-panics-gate · Zero panic lines in end-to-end runs — No panic role line may appear under the end-to-end home, checked fail-closed. · guard: obs G2 zero-panics gate · except: A panic whose location is the hook seam module.

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
