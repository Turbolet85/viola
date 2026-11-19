# architecture · the rows that bind 2026-11-19-a-live-stopfailure-payload — picked 162 of 967 (102 by every draw) · rule any · pin 3058

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.

## stack-and-technologies · ## Stack and Technologies
decision/serde-serialization · serde for frames, logs and snapshots — serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
decision/no-unbounded-json-depth · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned. · why: every parse of external bytes keeps serde_json's default recursion limit · guard: deny.toml
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key

## established-decisions · ## Established Decisions
decision/browser-tools-prompt-ledger-row · Browser-tools-prompt ledger row and its readings — The browser-tools-prompt relied ledger row reads pass when some verify run's first screen is the CLI's browser-tools prompt, absent when no run met it, and fail when any first screen reads none; it is the one relied row whose absent counts toward verified.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented. · except: StopFailure is registered with no ledger row and no verify probe.
decision/human-interrupt-moves-wheel · Human interrupt moves the wheel — A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state. · why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
decision/no-model-api-calls · No model API calls — The binary makes no model API calls; the model is reached only through the unmodified `claude` CLI. · why: D2, subscription only through the unmodified `claude` CLI.
decision/no-probe-for-unrelied-shapes · No probe for a shape no send relies on — A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only. · why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
decision/no-typing-into-trust-dialogs · viola types into no trust dialog — viola types into neither the folder trust dialog nor the external CLAUDE.md imports dialog.
decision/plugin-only-in-wrapped-sessions · Plugin loads only into wrapped sessions — The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`. · why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola. · except: `viola verify`'s transient unwrapped children, all gone when verify ends.
decision/r8-identity-floor · R8 parent-identity floor — The measured CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-never-types-into-dialog · viola verify never answers a CLI dialog — No `viola verify` run writes a byte into a CLI dialog.
decision/verify-own-trust-record · Verify writes and removes its own trust record — viola verify writes a trust record for its own temp folder only into the Claude config file its child reads, never creating an absent file, and removes that one record after. · why: Ratified as the one bounded exception to viola writing nothing into Claude's config.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content. · except: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
decision/verify-types-nothing-into-dialogs · Verify never types into a CLI dialog — No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key. · why: Ratified rulings.
contract/claude-cli-version-output/first-line · Claude CLI --version output · first line — Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
contract/hook-tiers/ordering-spine · Hook tiers · sync with no reply body (the ordering spine) — SessionStart, UserPromptSubmit, Stop and StopFailure each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
contract/hook-tiers/sync-spine · hook tiers · sync spine — SessionStart, UserPromptSubmit, Stop and StopFailure run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|absent|fail, MM being the relied row count.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/global-statusline-untouched · Global statusline config untouched — The user's global statusline config is never touched.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arm, called only by `viola verify`'s probes, reads no `VIOLA_*` and opens no channel, still exiting 0 with empty stderr.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/permission-request-question-stays-null · A question first raised by PermissionRequest stays null — A question first raised by PermissionRequest stays null to the human. · why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a further relied row, its viola verify probe and a re-stamp of every stamped version.
decision/private-mode-before-write · File mode set before any byte is written — A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
decision/reader-counts-skipped-lines · Event reader counts what it skips — The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
decision/send-flight-lock · Send post-gate section under the wrapper flight lock — A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes the lock. · why: Closes the race between the readiness gate and a turn the wrapper learns of, while the human's keys never wait.
decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
contract/viola-verify-refusal-codes/version-unreadable · viola verify process-exit refusal codes · version-unreadable — Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.

## conventions · ## Conventions
decision/env-var-viola-prefix · Environment variables carry the VIOLA_ prefix — Every environment variable the product reads or sets has the `VIOLA_` prefix, SCREAMING_SNAKE_CASE. · except: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/unparseable-external-values-unknown · Unparseable external values become unknown — An unparseable external value becomes the string `"unknown"` in viola's own records, never an error. · except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
contract/hook-to-kind-map/stopfailure · Hook to kind map · StopFailure — StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine TurnEndCause words, classified from the CLI's error word inside the Claude adapter.
contract/normalised-event-kinds/turn-ended · normalised event kinds · turn-ended — `turn-ended` is a normalised event kind.
contract/turn-end-cause/cause-set · Turn-end cause · cause set — `turn-ended` has a closed cause set, TurnEndCause, in viola-core beside UnreachableCause, of nine words (rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error) with no Claude-specific name; a `turn-ended` cause is one of its words.
contract/verify-process-exit/unclean-fixture · viola verify process-exit detail · unclean-fixture — The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
decision/wire-enum-values-kebab-case · Enum string values on the wire in kebab-case — Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
contract/channel-methods/hook-event · channel methods · hook.event — A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}, transcript?}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/hook-to-kind-map/stop · Hook to kind map · Stop — Stop maps to `turn-ended`.
contract/hook-to-kind-map/userpromptsubmit · Hook to kind map · UserPromptSubmit — UserPromptSubmit maps to `prompt-submitted`.
contract/verify-process-exit/version-unreadable · viola verify process-exit detail · version-unreadable — The code verify's process-exit carries when the CLI version cannot be read.

## standard-contracts · ## Standard Contracts
decision/api-error-turn-end-clears-turn · API-error turn end clears the running turn — An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted. · guard: path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row counts — A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
contract/event-data/turn-ended-background-tasks · event data per kind · turn-ended background_tasks — `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array.
contract/event-data/turn-ended-cause · event data per kind · turn-ended cause — `cause` is present only on an API-error turn end, one of the nine TurnEndCause words, and absent on a Stop end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`.
contract/event-data/turn-ended-last-assistant-message · event data per kind · turn-ended last_assistant_message — `turn-ended` data is `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
contract/event-data/turn-ended-scheduled-wakes · event data per kind · turn-ended scheduled_wakes — `scheduled_wakes` counts the session's scheduled wakes: the Stop payload's `session_crons` array length with no element read or carried, `null` when the key is absent, `null` or not an array, and on an API-error turn end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait` only.
contract/event-line/data · ndjson event line · data — `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
contract/hook-event/kind · channel hook.event · kind — One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
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
contract/trust-writer-words/removed · Trust writer code words · removed — The written trust record was removed again, every other key of the config left as found. Only viola verify gives it; a start reads it as lost.
contract/trust-writer-words/written · Trust writer code words · written — The trust record was written into the Claude config file.
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
decision/stamps-merge-own-version-only · Stamps merge replaces only its own version — A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
contract/channel-methods/last · channel methods · last — A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
contract/channel-methods/last-after-seen-line · channel methods · last after a seen line — A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start.
contract/event-line/shape · ndjson event line · shape — Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
contract/event-line/source · ndjson event line · source — `source` is one of `hook` · `wrapper`.
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
contract/hook-event/notification · channel hook.event · notification — Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}, transcript?}.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/ledger-stamps/writer · ledger stamps envelope · writer — `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.

## occupied-resources · ## Occupied Resources
decision/child-env-claude-strip · CLAUDE names stripped from the child's environment — Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it. · except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says. · except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
decision/cleanup-reads-refused-or-missing-socket-gone · Cleanup reads a refused or missing socket as gone — Harness cleanup reads the endpoint as gone when a connect is refused or the path is absent, and removes no leftover socket file, so cleanup reads ok after a force-killed wrapper. · why: A killed wrapper removes nothing and the next bind takes its leftover; the force-killed-wrapper lifecycle test asserts ok and the file still standing.
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value. · except: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/harness-env-unread-by-product · Harness variables never read by viola — AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED, AGENT_RUN_RIG and CI are test-harness variables that viola itself never reads.
decision/identity-floor-eleven-names · Identity floor of eleven names — The identity floor is CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT and CLAUDE_PID. · why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene · except: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/test-binaries-never-released · Test-only binaries kept out of release builds — viola-fake-agent, viola-harness, viola-mcp-tap and viola-live-rig are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
decision/test-seams-not-configuration · Test seams are not a configuration channel — Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
decision/verify-probe-dirs-drop-guarded · verify probe dirs removed by drop guard — Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing. · why: its captures are transient, content-bearing raw hook payloads
decision/verify-removes-only-own-dirs · verify removes only the probe folders it created — viola verify removes its probe folders whole, and its own trust record from the Claude config file, when it passes, fails or is stopped by Ctrl-C or TERM, only what that run created, never by a pattern or a listing. · except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
decision/viola-env-namespace-product-only · VIOLA_ environment namespace is product plumbing — Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/process-start-claude-child/env-kept · process-start line of the claude child · env_kept — Names the CLAUDE* variables kept through claude_env_keep, names only.
contract/process-start-claude-child/env-stripped-known · process-start line of the claude child · env_stripped_known — Names the CLAUDE* variables stripped from the child's environment, names only.
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
contract/viola-cli-arguments/verify · viola CLI arguments · verify — viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude; a hidden --record-first-screens, absent from --help, is passed only by the live rig.
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that verify reads.
decision/e2e-noop-fake-agent-feature · No-op fake-agent feature in the e2e crate — The test-only `viola-e2e` crate declares a no-op `fake-agent` feature. · why: So package-scoped `--features fake-agent` invocations resolve.
decision/end-to-end-homes-kept-after-each-job · Homes kept after each job's end-to-end run — Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out. · why: The homes are kept in CI for the obs gates.
decision/fake-agent-feature-on-named-members · A fake-agent feature on each named member crate — The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves. · why: so a package-scoped --features fake-agent build (cargo-mutants) resolves · except: viola-channel's fake-agent feature enables its Tokio client, so its package-scoped mutation build compiles and tests that client.
decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC, read only by a fake-agent build, makes hook panic only on the exact value 1, with a fixed ASCII payload holding no upstream text, path or secret; the seam disables no control and widens no redaction. · why: A ratified test seam, ratified together with the exact-path exemption for its panic lines.
decision/fake-agent-pump-delay-seam · FAKE_AGENT_PUMP_DELAY_MS test seam — FAKE_AGENT_PUMP_DELAY_MS is read only by a viola built with the test-only fake-agent feature, parsed as milliseconds capped at 5 000, and holds the pump back after the child starts; it disables no control and widens no redaction. · why: Ratified as a security-plan Security Anti-Patterns carve-out: a test seam, not a configuration channel.
decision/probe-capture-arms-inert · Probe capture arms are inert — The hidden hook `--capture` arm takes an absolute, existing directory only, reads no `VIOLA_*` variable, starts no observability, opens no channel, and always exits 0 with empty stderr.
decision/session-home-from-viola-dir · Session home resolved from VIOLA_DIR — Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing. · why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/child-environment/path · The wrapped child's environment · PATH — Prefixed by viola run with the pinned copy's folder.
contract/child-environment/viola-bin · The wrapped child's environment · VIOLA_BIN — Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
contract/child-environment/viola-dir · The wrapped child's environment · VIOLA_DIR — Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/harness-environment/agent-run-chunk-base · test harness environment · AGENT_RUN_CHUNK_BASE — Overrides the diff base of viola-harness run --mutants; without it the harness derives the base from git history.
contract/harness-environment/agent-run-keep-failed · test harness environment · AGENT_RUN_KEEP_FAILED — The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command.
contract/harness-environment/agent-run-keep-homes · test harness environment · AGENT_RUN_KEEP_HOMES — The value 1 keeps the test and session homes for the CI gates.
contract/hook-capture-arm/exit · viola hook --capture arm · exit status — Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened.
contract/hook-capture-arm/payload-file · viola hook --capture arm · payload file — Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — A refusal code verify writes under a parsed VIOLA_NAME.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.

## project-intent · ## Project Intent
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.
decision/tolerant-serde-external · Tolerant serde for external payloads — External payloads are deserialised tolerantly.

## build-system · key of ## Infrastructure Patterns
decision/serde-json-recursion-limit · serde_json keeps its recursion limit — The serde_json unbounded_depth feature is banned. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: probe row serde_json-unbounded-depth

## ci-cd-approach · key of ## Infrastructure Patterns
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/release-build-viola-only · Release build yields viola only — The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent. · why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables. · guard: scripts/release-check.sh

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.
decision/e2e-test-only · viola-e2e is test-only — viola-e2e is test-only, and no product crate depends on it.

## project-directory-structure · key of ## Infrastructure Patterns
decision/release-carries-viola-only · Release build carries viola only — The release build carries the viola binary only and no test-only feature. · guard: release-check in the CI release job

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
