# architecture · the rows that bind 2026-11-27-epoch-5-cleanup-2 — picked 147 of 1049 (65 by every draw) · rule any · pin 5179

## stack-and-technologies · ## Stack and Technologies
decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
decision/coverage-floors-gate · Coverage floors enforced by the harness gate — Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
decision/portable-pty-seam · portable-pty behind viola's own pty seam — The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
decision/process-liveness-signals · Heartbeat first, sysinfo and claude agents enrich — The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written

## established-decisions · ## Established Decisions
decision/child-cwd-explicit · Child cwd set explicitly — The child's cwd is set explicitly to viola's current directory. · except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
decision/child-exit-on-process-handle · Child exit detected on the process handle — Child exit is detected on the process handle, never on EOF.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/launch-pair-handed-not-verified · Launch pair handed, never verified — Every start, run, revive and adopt, and every switch, hands the claude child a model and effort pair on its command line; the pair is never read back or verified as the one the session runs on. · why: Whether a session started or resumed with the pair runs on it is unmeasured on the real CLI.
decision/lineage-label-only · Lineage is a label only — Lineage is a label only: no wheel, link, send or answer path reads the declared parent or role; viola up reads the parent only to order its re-opens. · guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
decision/liveness-heartbeat-and-pid · Heartbeat-first session liveness — The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it. · why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
decision/portable-pty-behind-pty-seam · portable-pty pinned behind viola's pty seam — portable-pty, pinned exactly at `=0.9.0`, is used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill). · why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils` (RUSTSEC-2026-0441), and `viola-pty` builds on it unchanged.
decision/revive-in-place · Revive resumes in place through the start path — A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated, and keeps the parent, the role and the model and effort pair its real snapshot records.
decision/revive-resumes-in-place · Revive resumes a gone instance in place — viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch spec but the recorded model and effort, and under the hidden --origin the recorded workspace, and its first record is wheel with holder driver and cause start, as every start's. · why: Ratified: a revived start keeps the wheel at driver.
decision/run-start-order · Child spawned last in the start order — A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record, writes the first snapshot, heartbeat and start events, and only then spawns the child. · why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
decision/session-liveness-heartbeat-and-start-time · Heartbeat primary, pid plus start time decide gone — The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows. · why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.
decision/start-verifies-new-cli-version · Start verifies a new CLI version when keyed — With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind, before the trust record and the claude child, and gates on the final verdict; the key is off by default.
decision/switch-in-place · Model and effort switched in place — A running wrapped session switches its model and effort in place: the wrapper ends only the child its own PTY handle holds, reaps it within RESPAWN_REAP_BOUND, appends `launch`, and resumes the same session id on the new pair under the same wrapper.
decision/switch-writes-no-user-settings · Switch writes nothing into the user's settings — A switch never writes to the user's own config or settings and passes no --settings; the switched pair lives only in viola's own record, which a later revive replays.
decision/up-parents-first · up re-opens parents before children — viola up re-opens its candidates one at a time, parents before children; a candidate whose parent names no other candidate is a root, and a parent cycle is taken once.
decision/up-selection-rule · up re-opens only instances gone before the boot — viola up skips a live or stale instance and takes a gone one only when its heartbeat is older than the host boot time and, unless it was named, its events log has no session-end after its last session-start; an unnamed instance ended on purpose stays down.
decision/window-features-target-hyprland · Window features target Hyprland — The window features target the Hyprland compositor.
contract/child-exit-record/exit-source · Child exit record · exit_source — The child's exit is recorded with exit_source handle-wait or kill-fallback.
decision/anyhow-only-in-bin · anyhow only in the viola binary — anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter. · why: anyhow's context chains are only useful where a person reads them.
decision/child-program-resolution · viola resolves the child program itself — viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/revive-reads-snapshot-once · Revive reads the snapshot once — A revive reads the snapshot once through the replay for both its holder check and its recorded directory, so a replayed snapshot makes one replay and logs one state-recovered line. · why: The cwd-missing revive test expects one replay and one state-recovered line in the role file.
contract/viola-revive-json-error-codes/internal-error · viola revive --json error codes · internal-error — A claude not on PATH or an anyhow error of the start.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.

## conventions · ## Conventions
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight, open, focus, up and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document; also an up whose lines are not all successes.
contract/cli-verbs/up · CLI verbs · up — `viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow the fleet and a name not scanned reads not-taken; CLI-only, no MCP tool and no channel method.
contract/viola-up-exit-codes/0 · viola up exit codes · 0 — Every line reads live, already-live, stale-heartbeat or would-open.
contract/viola-up-exit-codes/1 · viola up exit codes · 1 — Any other line; or a refusal with a document under --json: boot-time-unread, strict-modes-failed, pinned-hash-mismatch or terminal-not-found.
contract/viola-up-exit-codes/21 · viola up exit codes · 21 — The board's strict refusal (instance-unreachable, strict-modes-failed), with no line printed.
decision/fake-agent-test-seams · Fake-agent test seam variables — The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds. · why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/standard-rust-naming · Standard Rust naming — Modules and files are snake_case, types UpperCamelCase, functions and variables snake_case, and constants SCREAMING_SNAKE_CASE, as standard Rust.
decision/viola-name-instance-key · ViolaName as the instance key — The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype. · why: No database, so no UUID or serial keys.
contract/cli-verbs/open · CLI verbs · open — `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude`.
contract/cli-verbs/switch · CLI verbs · switch — `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` takes at least one of `--model` and `--effort`, else a usage error, exit 2; prints `<name>  switched  <model> <effort>` on stdout and exits 0, 10, 12, 13, 14, 20 or 21.
contract/exit-21-human-hints/placement · Exit 21 human hints · placement — Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.

## standard-contracts · ## Standard Contracts
decision/no-shell-out · No shelling out — Nothing shells out through `sh` or `bash`; child processes are spawned directly.
decision/spawn-directory-confined-to-snapshot · Spawn directory confined to the snapshot — The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
decision/up-writes-no-state · up writes no state of its own — viola up writes no snapshot, event line or stamp, leaving each to the revive it spawns; its lines, hint and --json document hold instance names and closed words only, never a window address, pid or workspace; under --dry-run it spawns nothing.
contract/channel-methods/switch · channel methods · switch — A request: `switch` `{model?, effort?}`, at least one, reads no `from` and answers `{model, effort}` or a refusal human-typing, unverified-cli, or not-delivered with turn-running, dialog-pending, no-session, child-not-ended or resume-failed; a bad or missing half is -32602, data null.
contract/hook-event/appended-line · channel hook.event · appended line — A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}; after a switch's respawn the old child's late `session-end` of the switched id is dropped once while the new child's pid is alive.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — `child_pid` is the pid of the current agent child, rewritten when a switch respawns it, and exists only to join `claude agents --json` rows to this instance.
contract/instance-snapshot/cwd · instance snapshot · cwd — `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its readers are `viola revive`, which uses it as the revived child's spawn directory and its trust record's folder, and `viola up`, opening each terminal there.
contract/instance-snapshot/launch · instance snapshot · launch — `launch` is `{model, effort}`, the pair the start or the latest switch handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line with a non-string member reading none.
contract/instance-snapshot/spawn-order · instance snapshot · spawn order — The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
contract/wrapper-event-appends/launch · wrapper event appends · `launch` — The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`, and once at every switch, after the old child is reaped and before the respawn; it is log-only and never wakes `wait`.
decision/hyprctl-direct-spawn · hyprctl spawned directly with a closed argv — hyprctl is spawned directly from a caller-resolved path, with no shell and no PATH lookup, stdin null and a closed argv; a dispatch is one Lua element, hl.dsp.focus or hl.dsp.window.move, whose only inserted values are a closed window address or an i32 workspace id, never reply text. · why: Hyprland 0.56.2 runs a dispatch's arguments as Lua, so only the measured hl.dsp.focus and hl.dsp.window.move forms are accepted.
decision/no-document-after-child-held-terminal · No JSON document once the child held the terminal — An error after the child has held the terminal prints no JSON document. · why: The child has held the terminal. · except: viola adopt --json's not-resumed end prints its error document after the child released the terminal; nothing of run or revive.
contract/event-data/launch · event data · `launch` — `{model, effort}`, both strings, with `source: wrapper`.
contract/hyprland-seam-errors/malformed · Hyprland seam errors · malformed — The reply could not be parsed for its request; the fixed message hyprctl reply not understood interpolates nothing.
contract/hyprland-seam-errors/oversize · Hyprland seam errors · oversize — The reply passed the frame cap and was refused whole before the exit was read; the fixed message hyprctl reply over the frame cap interpolates nothing.
contract/hyprland-seam-errors/refused · Hyprland seam errors · refused — hyprctl refused a dispatch; the fixed message hyprctl refused the dispatch interpolates nothing.
contract/hyprland-seam-errors/unreachable · Hyprland seam errors · unreachable — hyprctl could not start, exited non-zero or was killed at its deadline; the fixed message hyprctl did not answer interpolates nothing.
contract/hyprland-seam-errors/unsupported · Hyprland seam errors · unsupported — No hyprctl program was given, so nothing is spawned and no line is written; the fixed message hyprctl not found interpolates nothing.
contract/instance-snapshot/pid-and-started-at · instance snapshot · pid and started_at — `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
contract/instance-snapshot/started-at · instance snapshot · started_at — `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
contract/instance-snapshot/window · instance snapshot · window — `window` is `{address?, workspace?, placement, origin}`, the wrapper's window record, omitted when absent; a replay rebuilds it from the log, the last `window` line after the last `launch` line winning and a later `launch` line clearing it.
contract/instance-snapshot/writer · instance snapshot · writer — `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
contract/ledger-stamps/content · ledger stamps envelope · content — Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/session-liveness/gone · session liveness · `gone` — A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
contract/session-liveness/live · session liveness · `live` — A session whose process is running and whose beat is at most 5 s old is `live`.
contract/session-liveness/stale · session liveness · `stale` — A session whose process is running and whose beat is older than 5 s or absent is `stale`.
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
contract/viola-open-exit-codes/0 · viola open exit codes · 0 — The session went live: the human line `<name>  live  <placement>`, or under --json `{"v":1,"ok":{"name","parent","window","placement"}}`, window `{address, workspace}` or null.
contract/viola-open-exit-codes/1 · viola open exit codes · 1 — A refusal: strict-modes-failed before any log, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error, each under --json the `{"v":1,"error":"<code>","detail":null}` document.
contract/wrapper-event-appends/window · wrapper event appends · `window` — The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin, and again after a switch's `launch` line when the snapshot holds a window; it is log-only and never wakes `wait`.

## occupied-resources · ## Occupied Resources
decision/claude-flag-literals-in-agent-crate · Claude flag literals in the agent crate — The Claude agent crate is the one product home of the --resume, --fork-session, --model and --effort flag literals, and holds the program name and the --settings flag that verify reads. · except: viola adopt's fixed not-resumed hint names claude --resume <id> as words for the human to type, never as an argument viola passes.
contract/child-argv/launch-pair · flags passed to the child · launch pair — `--model <M> --effort <E>` follows `--plugin-dir` and precedes the caller's arguments at every start and every resume.
contract/child-argv/resume · flags passed to the child · `--resume` — A revived or adopted start, and a switch's respawn, launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
contract/claude-child-argv/resume · the claude child's argv · --resume — A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it; an adopted start adds --resume with the operator-typed id, in the session-id shape; a switch's respawn adds --resume with the switched id.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME for list, verify, revive --list, up and plugin install.
contract/process-exit-detail/boot-time-unread · process-exit detail enum · boot-time-unread — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola up: its exit 1 when the host boot time cannot be read.
contract/process-exit-detail/not-live · process-exit detail enum · not-live — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
contract/process-exit-detail/terminal-not-found · process-exit detail enum · terminal-not-found — A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
contract/viola-cli-arguments/revive-origin · viola CLI arguments · revive --origin — viola revive --origin <cli|agent> is a hidden flag, conflicting with --list and with no --place: under it the revive places its window on the workspace a real snapshot recorded, else none; without it there is no window step.
decision/env-strip-names-only · Environment strip reads names only — The environment strip and the logs read variable names only, never a value. · except: viola verify, and the run, revive and adopt start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/env-strip-reads-names-only · Environment strip reads names only — The child-environment strip and its logs handle variable names only and never read a value. · except: viola verify, and the run, revive and adopt start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
contract/child-argv/caller-arguments · flags passed to the child · caller's arguments — The caller's arguments follow the viola flags.
contract/child-argv/plugin-dir · flags passed to the child · `--plugin-dir` — `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
contract/claude-child-argv/plugin-dir · the claude child's argv · --plugin-dir — --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
contract/config-json/claude-env-keep · config.json · claude_env_keep — The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm and viola adopt's start included.
contract/profraw-census-exit-codes/0 · profraw census exit codes · 0 — Every run passed and, in the default form, the profile count equals the run count with no third or short profile; in the case form, no profile was refused.
contract/profraw-census-exit-codes/1 · profraw census exit codes · 1 — Any other count.
contract/profraw-census-exit-codes/2 · profraw census exit codes · 2 — A usage error, no instrumented binary, or (case form) no llvm-profdata.
contract/viola-cli-arguments/run-origin · viola CLI arguments · run --origin — viola run --origin <WORD> is a hidden flag naming the window's origin, an origin word; any other word is a usage error, exit 2.
contract/viola-cli-arguments/run-place · viola CLI arguments · run --place — viola run --place <N> is a hidden flag naming the workspace the window is moved to; --place without --origin is a usage error, exit 2.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/run-terminal-child-only · Run writes only the child's output to the terminal — While the child runs, viola run writes nothing to the terminal except the child's own output. · why: The terminal carries the child's screen.
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.

## project-intent · ## Project Intent
decision/modular-monolith-no-daemon · Modular monolith, no daemon — The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records
decision/anyhow-only-root-bin · anyhow only in the root binary — anyhow is used only in the root binary; CLI exit codes are typed.

## build-system · key of ## Infrastructure Patterns
decision/fmt-and-clippy-gate · Formatting and clippy with warnings denied — Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh
decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/coverage-floors · Coverage floors — The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest. · guard: gate --require coverage,doctest
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/hyprland-seam-crate · viola-hyprland is the one compositor seam — Compositor queries and dispatches go only through viola-hyprland's closed request set; the crate is sync and tokio-free and depends on viola-core alone among viola crates, never on viola-agent-claude, viola-state or viola-channel. · guard: the cargo tree check of viola-hyprland and the per-sync-crate tokio ban
decision/pty-knows-no-agent · viola-pty depends on no viola crate and knows no agent — viola-pty depends on no other viola crate, knows no agent, and takes no thiserror.
decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds private config;UserPromptSubmit/StopFailure key names,source word,error word 13|other,2 lengths,cachedChromeExtensionInstalled name;VH key names,needle;fake agent/rig trust records

## project-directory-structure · key of ## Infrastructure Patterns
decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes
decision/pty-seam-portable-pty · portable-pty is the PTY seam — The PTY seam is built over portable-pty, with host raw mode through libc termios.

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
