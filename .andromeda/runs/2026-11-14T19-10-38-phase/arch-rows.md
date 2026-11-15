# architecture · the rows that bind 2026-11-14-status-line-interception-removed — picked 115 of 965 (72 by every draw) · rule any · pin 1206

## design-philosophy · ## Design Philosophy
decision/capability-ledger · Version-stamped capability ledger — Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/no-tmux-or-shell-dependency · No dependence on tmux or a shell — No design choice may depend on tmux or a shell.

## stack-and-technologies · ## Stack and Technologies
decision/chrono-timestamps · chrono RFC 3339 UTC timestamps — Timestamps are RFC 3339 UTC with millisecond precision, through chrono. · why: already in the tree through rmcp
decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
decision/versioned-records-no-migrations · Versioned records instead of migrations — Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log. · except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
decision/run-never-reads-screen-content · run never reads screen content — run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures.
decision/vt100-screen-model · vt100 as screen model — vt100 gives pre-send readiness and modal detection on run's feed thread and the screen of verify's interactive probe runs.

## established-decisions · ## Established Decisions
decision/chrono-utc-millisecond-timestamps · chrono timestamps, RFC 3339 UTC, milliseconds — chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision. · why: chrono is already in the tree through rmcp.
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
decision/cli-capability-ledger · Per-behaviour CLI capability ledger — Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/global-statusline-untouched · Global statusline config untouched — The user's global statusline config is never touched.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arms, called only by `viola verify`'s probes, read no `VIOLA_*` and open no channel, still exiting 0 with empty stderr.
decision/personal-v1-no-accounts · Personal v1 without accounts — v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively. · why: The later public version for individual Claude Code subscribers must not need a rewrite.
decision/reader-counts-skipped-lines · Event reader counts what it skips — The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no statusline command, and no frame is written and nothing runs.
decision/statusline-wrapped-per-session · Statusline wrapped only per viola session — The statusline is wrapped only for sessions started by `viola run`, through a per-session settings override that runs the user's own statusline command unchanged. · why: The user's global statusline config is never touched.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
decision/versioned-tolerant-own-formats · Own formats versioned-tolerant — Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields. · why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
contract/list-json/skipped · viola list --json · skipped — skipped is {unknown_kinds, unknown_fields, torn_lines}, each summed over every scanned instance, gone ones included.
contract/probe-capture/statusline-shell-file · Hook probe capture directory · `StatusLine.<k>.shell` — Holds exactly `bin-sh` when the `--shell` word equals `/bin/sh` byte for byte, else `other`; the word itself is written nowhere.
contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
decision/hook-without-trusted-instance-exits-silently · Hook with no trusted instance exits 0 untouched — A hook whose instance check refuses or finds no wrapped instance exits 0 before reading config or stdin, and writes no line, frame or file. · why: The fail-open hook contract and the hook strict-modes tests rely on it, and the check must come before the log open that narrows the home.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/verify-box-wait-for-literal · Verify waits for an input-box literal after added turns — After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding the input box (a footer literal or the box's own rows) or a modal literal, a poisoned screen, or the probe deadline; any other quiet screen keeps waiting. · why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
decision/verify-child-set · Verify's bounded child set — viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts. · why: Each widening of the child set was ratified.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
contract/viola-verify-exit-codes/1-relied-row-failed · viola verify exit codes · 1 — A relied ledger row failed; the failing row's stamp is still written.
contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.

## conventions · ## Conventions
decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`. · except: Removing the budget gate's fields and event kind kept every own-format `v` at 1 (snapshot, channel params, event line, list document); readers skip the removed keys and kind.
contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — `hook statusline` emits no event.
contract/protocol-version/v · protocol versioning · v — An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
decision/missing-equals-null-on-read · Missing and null read the same — Readers treat a missing field and a `null` field the same.
decision/silent-roles-at-internal-error-catch · Run, revive, mcp and hook silent at the catch site — Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
decision/timestamp-format · Timestamp format — Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/viola-hook-exit-codes/0 · viola hook exit codes · 0 — Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.

## standard-contracts · ## Standard Contracts
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly. · except: `hook statusline` runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin.
decision/statusline-check-before-log · Statusline instance check before the log — The statusline arm runs the instance check before it opens its log or reads anything under the home. · why: Opening the log sets an existing home to 0700, so a check made after it passes a home at 0770.
decision/statusline-deadline · Statusline command deadline — The statusline command is bounded by a 5 s deadline. · provisional
decision/statusline-hook-exempt · Statusline wrapper exempt from hook output rules — `viola hook statusline` prints the user's statusline command's stdout unchanged and always exits 0 with empty stderr. · why: It is the statusline wrapper, not a Claude Code hook, so the hook decision rules do not apply.
decision/statusline-only-shell-out · Statusline command is the only shell-out — The user's statusline command is the only shell-out in viola: one argument of `/bin/sh -c`, with the same stdin, environment and working directory unchanged, stderr discarded, bounded by a deadline. · why: The command is a user-written shell string; a ratified boundary widening. · guard: ledger row statusline-shell
decision/statusline-source-in-viola-home · Statusline command source — The user's statusline command is read at start from one named source that viola only reads: the statusline source file in the viola home whenever it exists, else only in the default home the user's Claude settings, else none. · why: Two files deciding a command viola later runs is a ratified boundary widening, choosing the file in the viola home.
decision/statusline-wrapper-not-a-hook · Statusline wrapper is not a hook event — `viola hook statusline` is a hidden arm of `hook` taken on the word `statusline` before the event is mapped, so the hook event set keeps its nine events and the arm is exempt from the hook rules. · why: The statusline arm is the statusline wrapper, not a Claude Code hook.
decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row passes — A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
contract/hook-statusline/capture · viola hook statusline · --capture <DIR> — With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no command run, nothing printed; without it the pass-through is unchanged.
contract/hook-statusline/exit · viola hook statusline · exit status — Always exits 0 with empty stderr.
contract/hook-statusline/refused-check · viola hook statusline · refused instance check — A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed and the home's mode left as found.
contract/hook-statusline/stdout · viola hook statusline · stdout — Prints the command's stdout unchanged, bounded by the frame cap; prints nothing when the command fails or passes the deadline, when no command is recorded, and on oversize or malformed stdin.
contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
contract/instance-snapshot/statusline-command · instance snapshot · statusline_command — `statusline_command` is the user's statusline command that `run` read from the named source at start, omitted when the source gave none; no event carries it, and `hook statusline` reads it and never changes the snapshot.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/malformed-envelope · ledger stamps envelope · malformed envelope — A malformed envelope makes `run`'s gate record `cli_verified:false`.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail".
contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`; the replay writes no file.
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
contract/statusline-source/command · The statusline source settings · statusLine command — A command is taken only from a `statusLine` of type `command`, non-empty, with no NUL.
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id and wheel from the log; a field no line gave is absent, never a default.
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
contract/instance-snapshot/spawn-order · instance snapshot · spawn order — The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
contract/ledger-stamps/measured · ledger stamps envelope · measured — The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.

## occupied-resources · ## Occupied Resources
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that run and verify read.
decision/per-session-settings-override · Per-session settings override for the status line — Every start, a revived one included, rewrites the instance's settings.json whole at 0600 before the spawn, so the child's status line is viola's wrapper. · why: A code-bearing file handed to the CLI at every start is a ratified boundary widening.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene
decision/refused-statusline-check-no-trace · Refused statusline check leaves no trace — A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run and nothing printed. · why: Ratified as the founder's own answer.
decision/screen-fixtures-signature-rows-only · Screen fixtures keep only signature rows — A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording.
decision/settings-override-safe-path-only · Settings override only for a safe pinned path — The settings override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings is passed.
decision/statusline-source-precedence · Source of the user's statusline command — When the home holds statusline-source.json it is the source of the user's statusline command; otherwise, only in the default home, the statusLine command of the user's Claude settings is taken; viola writes neither and reads both under the frame bound.
contract/child-argv/settings · flags passed to the child · `--settings` — `--settings <instance dir>/settings.json` follows when the start wrote a settings override.
contract/claude-child-argv/settings · the claude child's argv · --settings — --settings <instance dir>/settings.json follows when the start wrote a settings override; the caller's arguments come after.
contract/instance-settings-json/statusline · instance settings.json · statusLine — {"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}, the pinned path absolute.
contract/settings-override/plain-path-only · per-session settings override · plain-character pinned path — The override is written only for a pinned path of plain characters; otherwise no override is written and no `--settings` is passed.
contract/settings-override/status-line · per-session settings override · `statusLine` — `instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn.
contract/statusline-capture-shell-file/verdict · StatusLine.<k>.shell capture file · verdict — Holds exactly bin-sh or other; a missing shell word is other, and the word is written nowhere.
decision/probe-capture-arms-inert · Probe capture arms are inert — The hidden hook `--capture` arms take an absolute, existing directory only, read no `VIOLA_*` variable, start no observability, open no channel, and always exit 0 with empty stderr.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/child-argv/caller-arguments · flags passed to the child · caller's arguments — The caller's arguments follow the viola flags.
contract/child-argv/fork-session · flags passed to the child · `--fork-session` — A revived start adds `--fork-session` after `--resume <id>` under `--fork`.
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
contract/child-argv/resume · flags passed to the child · `--resume` — A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
contract/claude-child-argv/fork-session · the claude child's argv · --fork-session — --fork-session follows --resume when --fork is given; the words after -- come last.
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/hook-capture-arm/exit · viola hook --capture arm · exit status — Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened.
contract/hook-capture-arm/payload-file · viola hook --capture arm · payload file — Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — The detail of a stopped run; a failing ledger row is no refusal and carries no code.
contract/verify-exit-detail/version-unreadable · viola verify process-exit detail · version-unreadable — A refusal code verify writes under a parsed VIOLA_NAME.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/capability-ledger-gate · Capability ledger as the single gate for CLI behaviour — Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption. · why: A ruled limit sets the exception. · except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
decision/hook-exits-on-untrusted-instance · Hook exits at once on an untrusted instance — viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append. · except: The two --capture arms, which read no VIOLA_* variable, run ahead of the check.
decision/mixed-version-tolerance · Mixed-version tolerance — Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole, and a stamp holding a retired ledger row verifies on the relied rows alone.
decision/statusline-source-follows-home · Statusline source follows the home — The statusline source is the home's statusline-source.json whenever it exists; only the default home falls back to the user's Claude settings, so a home named by --home or reached through VIOLA_DIR never reads the user's settings.
contract/hook-statusline-output/stdout · Hook statusline output · stdout — The hook statusline pass-through carries the user's status line output on stdout and nothing else; its --capture probe arm prints nothing.
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/hook-fails-open · Hook fails open toward the human — Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
decision/no-environment-configuration · Environment variables are no configuration channel — No environment variable configures viola; the VIOLA_* variables are wrapper-to-child plumbing set by viola run only, and a dependency or feature that reads an environment variable breaks the rule like a direct read does. · except: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.

## project-intent · ## Project Intent
decision/wrap-unmodified-claude · Wrap the unmodified claude CLI — viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude.
contract/timestamps/format · Timestamps · format — Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.

## build-system · key of ## Infrastructure Patterns
decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh

## ci-cd-approach · key of ## Infrastructure Patterns
decision/fuzz-replay-ci · Fuzz corpus replayed in CI — A CI job replays the fuzz corpus on every push and pull request. · guard: gate --require fuzz-replay
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin. · why: So the crate stays pure.

## deployment-model · key of ## Infrastructure Patterns
decision/run-pins-self-copy · run pins a copy of itself and its plugin — viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.

## project-directory-structure · key of ## Infrastructure Patterns
decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, statusline_command, pid, started_at or links.

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
