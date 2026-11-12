@row_add n=1 row=decision/test-binaries-never-released area=occupied-resources rec=1
name: Test-only binaries kept out of release builds
rule: viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
@row_add n=2 row=name/binary/viola-fake-agent area=occupied-resources rec=1
class: binary
name: viola-fake-agent
@row_add n=3 row=name/binary/viola-harness area=occupied-resources rec=1
class: binary
name: viola-harness
@row_add n=4 row=name/binary/viola-mcp-tap area=occupied-resources rec=1
class: binary
name: viola-mcp-tap
@row_add n=5 row=name/command/run area=occupied-resources rec=2
class: command
name: run
@row_add n=6 row=name/command/revive area=occupied-resources rec=2
class: command
name: revive
@row_add n=7 row=name/command/send area=occupied-resources rec=2
class: command
name: send
@row_add n=8 row=name/command/wait area=occupied-resources rec=2
class: command
name: wait
@row_add n=9 row=name/command/last area=occupied-resources rec=2
class: command
name: last
@row_add n=10 row=name/command/list area=occupied-resources rec=2
class: command
name: list
@row_add n=11 row=name/command/answer area=occupied-resources rec=2
class: command
name: answer
@row_add n=12 row=name/command/hook area=occupied-resources rec=2
class: command
name: hook
@row_add n=13 row=name/command/mcp area=occupied-resources rec=2
class: command
name: mcp
@row_add n=14 row=name/command/verify area=occupied-resources rec=2
class: command
name: verify
@row_add n=15 row=name/command/pause area=occupied-resources rec=2
class: command
name: pause
@row_add n=16 row=name/command/release area=occupied-resources rec=2
class: command
name: release
@row_add n=17 row=name/command/link area=occupied-resources rec=2
class: command
name: link
@row_add n=18 row=name/command/unlink area=occupied-resources rec=2
class: command
name: unlink
@row_add n=19 row=name/command/plugin-install area=occupied-resources rec=2
class: command
name: plugin install
@row_add n=20 row=decision/mcp-subcommand-hidden area=occupied-resources rec=2
name: Hidden mcp subcommand
rule: The viola mcp subcommand is hidden from viola --help and takes no argument but the global --home.
why: The plugin runs it, never a person.
@row_add n=21 row=contract/viola-mcp-exit-codes/0 area=occupied-resources rec=2
boundary: viola mcp exit codes
item: 0
promise: The session ended by stdin closing, or by a stdin line past MAX_FRAME, once initialized; both streams empty.
@row_add n=22 row=contract/viola-mcp-exit-codes/1 area=occupied-resources rec=2
boundary: viola mcp exit codes
item: 1
promise: The session ended before initialize, or the home or diagnostics folder failed the strict-modes check, with nothing on stdout or stderr and no role file.
@row_add n=23 row=contract/viola-cli-arguments/verify area=occupied-resources rec=2
boundary: viola CLI arguments
item: verify
promise: viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude.
@row_add n=24 row=contract/viola-cli-arguments/list area=occupied-resources rec=2
boundary: viola CLI arguments
item: list
promise: viola list takes [--json] and nothing else.
@row_add n=25 row=contract/viola-cli-arguments/plugin-install area=occupied-resources rec=2
boundary: viola CLI arguments
item: plugin install
promise: viola plugin install takes [--json] and nothing else; viola plugin alone is a usage error.
@row_add n=26 row=contract/viola-cli-arguments/revive area=occupied-resources rec=2
boundary: viola CLI arguments
item: revive
promise: viola revive takes <name> [--id <ID>] [--fork] [-- <child args>], or <name> --list, each with [--json].
@row_add n=27 row=contract/viola-list-exit-codes/0 area=occupied-resources rec=2
boundary: viola list exit codes
item: 0
promise: The board was printed in one stdout write.
@row_add n=28 row=contract/viola-list-exit-codes/21 area=occupied-resources rec=2
boundary: viola list exit codes
item: 21
promise: The strict-modes check refused the home, instances/ or any one instance, and the whole board is refused.
@row_add n=29 row=contract/viola-list-exit-codes/1 area=occupied-resources rec=2
boundary: viola list exit codes
item: 1
promise: An internal error, through the cli catch site.
@row_add n=30 row=contract/viola-list-exit-codes/2 area=occupied-resources rec=2
boundary: viola list exit codes
item: 2
promise: A clap usage error.
@row_add n=31 row=contract/viola-plugin-install-exit-codes/0 area=occupied-resources rec=2
boundary: viola plugin install exit codes
item: 0
promise: The result was printed in one stdout write.
@row_add n=32 row=contract/viola-plugin-install-exit-codes/21 area=occupied-resources rec=2
boundary: viola plugin install exit codes
item: 21
promise: The strict-modes check refused the home or an existing marketplace tree folder; nothing created, no log opened.
@row_add n=33 row=contract/viola-plugin-install-exit-codes/1 area=occupied-resources rec=2
boundary: viola plugin install exit codes
item: 1
promise: An internal error, through the cli catch site.
@row_add n=34 row=contract/viola-plugin-install-exit-codes/2 area=occupied-resources rec=2
boundary: viola plugin install exit codes
item: 2
promise: A clap usage error.
@row_add n=35 row=contract/viola-revive-exit-codes/0 area=occupied-resources rec=2
boundary: viola revive exit codes
item: 0
promise: The revived child's wrapper ended as viola run's does.
@row_add n=36 row=contract/viola-revive-exit-codes/1 area=occupied-resources rec=2
boundary: viola revive exit codes
item: 1
promise: A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
@row_add n=37 row=contract/viola-revive-exit-codes/2 area=occupied-resources rec=2
boundary: viola revive exit codes
item: 2
promise: A clap usage error, a malformed --id included.
@row_add n=38 row=contract/viola-revive-json/error-document area=occupied-resources rec=2
boundary: viola revive --json output
item: error document
promise: A refused strict-modes check and every refusal of the start print the exit-1 document {"v":1,"error":"<code>","detail":null}, the code one of the ten standard codes, with stderr empty.
@row_add n=39 row=contract/viola-revive-json/list area=occupied-resources rec=2
boundary: viola revive --json output
item: --list
promise: viola revive <name> --list --json prints the ok.sessions document, or the no-session document with exit 1.
@row_add n=40 row=contract/viola-cli-arguments/link-unlink area=occupied-resources rec=3
boundary: viola CLI arguments
item: link and unlink
promise: viola link and viola unlink each take <driver> <driven> [--json], the verb's instance being the driven one.
@row_add n=41 row=contract/viola-link-exit-codes/0 area=occupied-resources rec=3
boundary: viola link and unlink exit codes
item: 0
promise: One result line, or under --json {"v":1,"ok":<ok>}, link's ok holding since and already_linked and unlink's ok empty.
@row_add n=42 row=contract/viola-link-exit-codes/21 area=occupied-resources rec=3
boundary: viola link and unlink exit codes
item: 21
promise: The instance check refused, no live endpoint is recorded, the recorded endpoint's folder is refused, or the serving process is not the recorded wrapper or runs under another euid.
@row_add n=43 row=contract/viola-link-exit-codes/20 area=occupied-resources rec=3
boundary: viola link and unlink exit codes
item: 20
promise: A wrapper error, a -32602 included, such as driver equal to driven.
@row_add n=44 row=contract/viola-link-exit-codes/1 area=occupied-resources rec=3
boundary: viola link and unlink exit codes
item: 1
promise: A refusal reply, or an ok of another shape.
@row_add n=45 row=contract/frame-verb-exit-codes/21 area=occupied-resources rec=4
boundary: send, wait, last, answer, pause and release exit codes
item: 21
promise: The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout.
@row_add n=46 row=contract/instance-unreachable-detail/strict-modes-failed area=occupied-resources rec=4
boundary: the instance-unreachable --json document
item: strict-modes-failed
promise: The detail when the instance check refused the instance.
@row_add n=47 row=contract/instance-unreachable-detail/socket-dir-refused area=occupied-resources rec=4
boundary: the instance-unreachable --json document
item: socket-dir-refused
promise: The detail when the recorded socket folder was refused, past the instance check.
@row_add n=48 row=contract/instance-unreachable-detail/server-verify-failed area=occupied-resources rec=4
boundary: the instance-unreachable --json document
item: server-verify-failed
promise: The detail when the server runs under another euid.
@row_add n=49 row=contract/viola-verify-exit-codes/1 area=occupied-resources rec=4
boundary: viola verify exit codes
item: 1
promise: The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.
@row_add n=50 row=contract/viola-run-start-exit-codes/1 area=occupied-resources rec=4
boundary: viola run and revive start exit codes
item: 1
promise: The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
@row_add n=51 row=contract/hook-capture-arm/exit area=occupied-resources rec=5
boundary: viola hook --capture arm
item: exit status
promise: Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened.
@row_add n=52 row=contract/hook-capture-arm/payload-file area=occupied-resources rec=5
boundary: viola hook --capture arm
item: payload file
promise: Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
@row_add n=53 row=contract/statusline-capture-shell-file/verdict area=occupied-resources rec=5
boundary: StatusLine.<k>.shell capture file
item: verdict
promise: Holds exactly bin-sh or other; a missing shell word is other, and the word is written nowhere.
@row_add n=54 row=contract/claude-child-argv/plugin-dir area=occupied-resources rec=6
boundary: the claude child's argv
item: --plugin-dir
promise: --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
@row_add n=55 row=contract/claude-child-argv/settings area=occupied-resources rec=6
boundary: the claude child's argv
item: --settings
promise: --settings <instance dir>/settings.json follows when the start wrote a settings override; the caller's arguments come after.
@row_add n=56 row=contract/claude-child-argv/resume area=occupied-resources rec=6
boundary: the claude child's argv
item: --resume
promise: A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it.
@row_add n=57 row=contract/claude-child-argv/fork-session area=occupied-resources rec=6
boundary: the claude child's argv
item: --fork-session
promise: --fork-session follows --resume when --fork is given; the words after -- come last.
@row_add n=58 row=contract/session-id/shape area=occupied-resources rec=6
boundary: Claude session id
item: shape
promise: 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere.
@row_add n=59 row=decision/logged-session-id-on-child-argv area=occupied-resources rec=6
name: Logged session id on the child's command line
rule: A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn.
why: A ratified crossing, chosen over allowing only an id the human types.
@row_add n=60 row=decision/claude-flag-literals-in-agent-crate area=occupied-resources rec=6
name: Claude flag literals in the agent crate
rule: The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that run and verify read.
@row_none n=1 rec=7 type=4
relied: verify's Run E resumes the session id taken from Run B's first SessionStart, so Run E depends on Run B.
@row_add n=61 row=decision/per-session-settings-override area=occupied-resources rec=8
name: Per-session settings override for the status line
rule: Every start, a revived one included, rewrites the instance's settings.json whole at 0600 before the spawn, so the child's status line is viola's wrapper.
why: A code-bearing file handed to the CLI at every start is a ratified boundary widening.
@row_add n=62 row=decision/settings-override-safe-path-only area=occupied-resources rec=8
name: Settings override only for a safe pinned path
rule: The settings override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings is passed.
@row_add n=63 row=contract/instance-settings-json/statusline area=occupied-resources rec=8
boundary: instance settings.json
item: statusLine
promise: {"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}, the pinned path absolute.
@row_add n=64 row=contract/child-environment/viola-dir area=occupied-resources rec=9
boundary: the claude child's environment
item: VIOLA_DIR
promise: Set by viola run to the absolute path of the instance's state folder; hook, mcp and every CLI verb in the session derive the viola home as its grandparent.
@row_add n=65 row=decision/session-home-from-viola-dir area=occupied-resources rec=9
name: Session home resolved from VIOLA_DIR
rule: Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing.
why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
@row_none n=2 rec=10 type=4
relied: A hook whose VIOLA_DIR names an instance folder that does not exist exits 0 and writes nothing.
@row_add n=66 row=decision/child-env-claude-strip area=occupied-resources rec=11
name: CLAUDE names stripped from the child's environment
rule: Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it.
except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
@row_add n=67 row=decision/identity-floor-eleven-names area=occupied-resources rec=11
name: Identity floor of eleven names
rule: The identity floor is CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT and CLAUDE_PID.
why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited.
@row_add n=68 row=decision/env-strip-names-only area=occupied-resources rec=11
name: Environment strip reads names only
rule: The environment strip and the logs read variable names only, never a value.
@row_add n=69 row=contract/process-start-claude-child/env-stripped-known area=occupied-resources rec=11
boundary: process-start line of the claude child
item: env_stripped_known
promise: Names the CLAUDE* variables stripped from the child's environment, names only.
@row_add n=70 row=contract/process-start-claude-child/env-kept area=occupied-resources rec=11
boundary: process-start line of the claude child
item: env_kept
promise: Names the CLAUDE* variables kept through claude_env_keep, names only.
@row_add n=71 row=decision/harness-env-unread-by-product area=occupied-resources rec=12
name: Harness variables never read by viola
rule: AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED and CI are test-harness variables that viola itself never reads.
@row_add n=72 row=contract/harness-environment/agent-run-chunk-base area=occupied-resources rec=12
boundary: test harness environment
item: AGENT_RUN_CHUNK_BASE
promise: Overrides the diff base of viola-harness run --mutants; without it the harness derives the base from git history.
@row_add n=73 row=contract/harness-environment/agent-run-keep-homes area=occupied-resources rec=12
boundary: test harness environment
item: AGENT_RUN_KEEP_HOMES
promise: The value 1 keeps the test and session homes for the CI gates.
@row_add n=74 row=contract/harness-environment/agent-run-keep-failed area=occupied-resources rec=12
boundary: test harness environment
item: AGENT_RUN_KEEP_FAILED
promise: The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command.
@row_add n=75 row=decision/no-live-claude-in-ci area=occupied-resources rec=12
name: Real claude never runs in CI
rule: When CI is present, viola-harness run --local-live returns ok false with reason live-in-ci and exit 2 before any build, spawn or suite.
@row_add n=76 row=decision/fake-agent-pump-delay-seam area=occupied-resources rec=13
name: Pump delay test seam
rule: FAKE_AGENT_PUMP_DELAY_MS is read only by a fake-agent build, capped at 5000 ms, and holds the pump back after the child starts; the seam disables no control and widens no redaction.
why: A ratified carve-out from the security anti-patterns: a test seam, not a configuration channel.
@row_add n=77 row=decision/fake-agent-hook-panic-seam area=occupied-resources rec=13
name: Hook panic test seam
rule: FAKE_AGENT_HOOK_PANIC, read only by a fake-agent build, makes hook panic only on the exact value 1, with a fixed ASCII payload holding no upstream text, path or secret; the seam disables no control and widens no redaction.
why: A ratified test seam, ratified together with the exact-path exemption for its panic lines.
@row_add n=78 row=contract/config-json/v area=occupied-resources rec=14
boundary: config.json
item: v
promise: Must be 1; the file is read under a MAX_FRAME cap.
@row_add n=79 row=contract/config-json/five-hour area=occupied-resources rec=14
boundary: config.json
item: five_hour
promise: A budget threshold, a JSON whole number from 0 to 100, default 90; any other value takes the default with one parse-rejected config-json line, the other key standing.
@row_add n=80 row=contract/config-json/seven-day area=occupied-resources rec=14
boundary: config.json
item: seven_day
promise: A budget threshold, a JSON whole number from 0 to 100, default 85; any other value takes the default with one parse-rejected config-json line, the other key standing.
@row_add n=81 row=contract/config-json/diagnostics-level area=occupied-resources rec=14
boundary: config.json
item: diagnostics_level
promise: "info" by default, or "debug".
@row_add n=82 row=contract/config-json/claude-env-keep area=occupied-resources rec=14
boundary: config.json
item: claude_env_keep
promise: The R8 pass-list, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole and an identity-floor name in it has no effect.
@row_add n=83 row=contract/config-json/dialog-hold-s area=occupied-resources rec=14
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line.
@row_add n=84 row=decision/config-bad-value-falls-back area=occupied-resources rec=14
name: Bad config values fall back to defaults
rule: An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value switches the budget gate off or makes the dialog hold zero or unbounded.
@row_add n=85 row=contract/viola-run-start-refusals/pinned-hash-mismatch area=occupied-resources rec=15
boundary: viola run start refusals
item: pinned-hash-mismatch
promise: A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
@row_add n=86 row=contract/diagnostics-role-files/run-name area=occupied-resources rec=16
boundary: home-level diagnostics role files
item: run-<name>.ndjson
promise: The wrapper's codes-only process lines, viola revive's start arm included.
@row_add n=87 row=contract/diagnostics-role-files/hook-name area=occupied-resources rec=16
boundary: home-level diagnostics role files
item: hook-<name>.ndjson
promise: The hook's hook-invoked, hook-decision and parse-rejected lines.
@row_add n=88 row=contract/diagnostics-role-files/mcp area=occupied-resources rec=16
boundary: home-level diagnostics role files
item: mcp.ndjson
promise: viola mcp's codes-only lines, with no self process-exit line.
@row_add n=89 row=contract/diagnostics-role-files/cli-name area=occupied-resources rec=16
boundary: home-level diagnostics role files
item: cli-<name>.ndjson
promise: The CLI verbs' lines, <name> being the target instance for the frame verbs and the caller's VIOLA_NAME for list, verify, revive --list and plugin install.
@row_add n=90 row=contract/diag-line/format area=occupied-resources rec=16
boundary: diagnostics line
item: format
promise: Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
@row_add n=91 row=decision/codes-only-role-logs area=occupied-resources rec=16
name: Codes-only role logs
rule: Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
@row_add n=92 row=decision/refused-check-writes-no-line area=occupied-resources rec=16,24
name: Refused check writes no line
rule: A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
@row_add n=93 row=contract/frame-verb-exit-detail/instance-dead area=occupied-resources rec=16
boundary: CLI frame verb process-exit detail
item: instance-dead
promise: Exit 21 with during connect, at WARN.
@row_add n=94 row=contract/frame-verb-exit-detail/server-verify-failed area=occupied-resources rec=16
boundary: CLI frame verb process-exit detail
item: server-verify-failed
promise: Exit 21 with during connect when the serving process is not the recorded wrapper or runs under another euid; no channel-request line.
@row_add n=95 row=contract/frame-verb-exit-detail/unwrapped-name area=occupied-resources rec=16
boundary: CLI frame verb process-exit detail
item: unwrapped-name
promise: Exit 21 with during connect.
@row_add n=96 row=contract/frame-verb-exit-detail/endpoint-lost area=occupied-resources rec=16
boundary: CLI frame verb process-exit detail
item: endpoint-lost
promise: Exit 21 with during call, for a wrapper lost with the call open.
@row_add n=97 row=contract/frame-verb-exit-detail/wrapper-fault area=occupied-resources rec=16
boundary: CLI frame verb process-exit detail
item: wrapper-fault
promise: Exit 20.
@row_add n=98 row=contract/frame-verb-exit-detail/socket-dir-refused area=occupied-resources rec=16
boundary: CLI frame verb process-exit detail
item: socket-dir-refused
promise: Exit 21 with during connect, at WARN, no channel-request line; run and revive's start write it with exit_code 1.
@row_add n=99 row=contract/process-exit-detail/stale-heartbeat area=occupied-resources rec=16
boundary: process-exit detail enum
item: stale-heartbeat
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json.
@row_add n=100 row=contract/hook-decision-detail/socket-dir-refused area=occupied-resources rec=16
boundary: hook-decision detail
item: socket-dir-refused
promise: One hook-decision line at WARN on a refused socket folder, with decision_emitted false.
@row_add n=101 row=contract/channel-response-detail/unsupported-version area=occupied-resources rec=17
boundary: channel-response detail, a closed enum
item: unsupported-version
promise: Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name.
@row_add n=102 row=contract/channel-response-detail/invalid-params area=occupied-resources rec=17
boundary: channel-response detail, a closed enum
item: invalid-params
promise: Written at WARN on both the server's and the client's line, the client reading it from a -32602 error's data.
@row_add n=103 row=contract/channel-response-detail/release-from-driver area=occupied-resources rec=17
boundary: channel-response detail, a closed enum
item: release-from-driver
promise: Written at WARN on both the server's and the client's line.
@row_add n=104 row=contract/verify-exit-detail/program-not-found area=occupied-resources rec=18
boundary: viola verify process-exit detail
item: program-not-found
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_add n=105 row=contract/verify-exit-detail/pinned-hash-mismatch area=occupied-resources rec=18
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_add n=106 row=contract/verify-exit-detail/version-unreadable area=occupied-resources rec=18
boundary: viola verify process-exit detail
item: version-unreadable
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_add n=107 row=contract/verify-exit-detail/strict-modes-failed area=occupied-resources rec=18
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line.
@row_add n=108 row=contract/verify-exit-detail/unclean-fixture area=occupied-resources rec=18
boundary: viola verify process-exit detail
item: unclean-fixture
promise: A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
@row_add n=109 row=contract/verify-exit-detail/interrupted area=occupied-resources rec=18
boundary: viola verify process-exit detail
item: interrupted
promise: The detail of a stopped run; a failing ledger row is no refusal and carries no code.
@row_add n=110 row=contract/diag-line/state-recovered area=occupied-resources rec=19
boundary: diagnostics line
item: state-recovered
promise: A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay.
@row_none n=3 rec=20 type=4
relied: A refused diagnostics folder or file silences the log rather than failing the process, and a detail line is dropped.
@row_add n=111 row=decision/events-log-never-truncated area=occupied-resources rec=21
name: Events log never rotated or truncated
rule: The instance's events.ndjson is never rotated or truncated, and any size bound must keep existing offsets valid.
why: wait's after and send's cursor are byte offsets into the file.
@row_add n=112 row=decision/strict-check-before-log-open area=occupied-resources rec=21,24
name: Strict-modes check before any log opens
rule: Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read.
why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
@row_add n=113 row=decision/board-refused-whole area=occupied-resources rec=21
name: Whole board refused on one failed check
rule: viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line.
why: Ratified as the founder's own answer.
@row_add n=114 row=decision/refused-statusline-check-no-trace area=occupied-resources rec=21
name: Refused statusline check leaves no trace
rule: A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run, nothing printed and budget.json untouched.
why: Ratified as the founder's own answer.
@row_add n=115 row=decision/strict-read-rule area=occupied-resources rec=22
name: Strict-modes read rule
rule: The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
@row_none n=4 rec=23 type=4
relied: run's and revive's starts and verify check the home tree, refusing at home or instance level, before using it.
@row_none n=5 rec=25 type=4
relied: A path under a regular file is a strict-modes refusal: verify exits 1 with the home pair, the frame verbs exit 21 with the instance pair.
@row_add n=116 row=decision/list-home-read-rule area=occupied-resources rec=26,27
name: viola list holds the home to the read rule
rule: viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board.
except: The budget read, mcp's start and plugin install keep the write rule on the home.
@row_add n=117 row=contract/budget-json/v area=occupied-resources rec=28
boundary: budget.json
item: v
promise: Must be 1, or the file gives no reading.
@row_add n=118 row=contract/budget-json/five-hour area=occupied-resources rec=28
boundary: budget.json
item: five_hour
promise: The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
@row_add n=119 row=contract/budget-json/seven-day area=occupied-resources rec=28
boundary: budget.json
item: seven_day
promise: The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
@row_add n=120 row=contract/budget-json/read-at area=occupied-resources rec=28
boundary: budget.json
item: read_at
promise: An RFC 3339 UTC timestamp with milliseconds; read as "unknown" when absent or no string.
@row_add n=121 row=decision/budget-json-single-writer area=occupied-resources rec=28
name: budget.json written only by the statusline hook
rule: budget.json holds the newest rate_limits reading across wrapped sessions, last writer wins, written whole only by hook statusline and only when the payload holds a rate_limits object.
@row_add n=122 row=decision/budget-gate-reads-hook-written-file area=occupied-resources rec=28
name: Budget gate refuses on a hook-written file
rule: The wrapper may refuse a driver on budget.json, read behind the strict-modes check on the home and the file and failing open: every failure is no reading, with no log line and nothing created.
why: A ratified boundary widening, ratified as built.
@row_add n=123 row=decision/stamps-single-writer area=occupied-resources rec=29
name: Ledger stamps have one writer
rule: ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
@row_add n=124 row=decision/version-gate-strict-stamps-read area=occupied-resources rec=29
name: Version gate reads stamps strictly
rule: run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
@row_add n=125 row=decision/verify-removes-only-own-dirs area=occupied-resources rec=30,31
name: verify removes only the probe folders it created
rule: viola verify removes its probe folders whole when it passes, fails or is stopped by Ctrl-C or TERM, and only the folders that run created, never by a pattern or a listing.
except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand.
@row_add n=126 row=decision/root-waits-share-one-bound area=occupied-resources rec=32
name: Root test waits share one bound
rule: Root test waits on a child share the one 7 s WITHIN bound of the watch support.
@row_add n=127 row=decision/wait-report-kept-on-failure area=occupied-resources rec=32
name: Wait report kept on failure
rule: A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes.
why: A test the runner kills keeps its evidence.
@row_none n=6 rec=33 type=4
relied: Harness cleanup reads ok after a force-killed wrapper, since a refused or missing socket reads gone and no leftover file is removed.
@row_add n=128 row=decision/no-after-every-test-error-body-helper area=occupied-resources rec=34
name: No after-every-test error-body helper
rule: Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built.
@row_none n=7 rec=35 type=4
relied: Root tests freeze a wrapper with SIGSTOP through one shared guard so that it reads stale, resuming it on drop.
@row_none n=8 rec=36 type=5
@row_add n=129 row=decision/e2e-home-owner-only-headroom area=occupied-resources rec=37
name: End-to-end home base checked before use
rule: The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free.
@row_none n=9 rec=38 type=5
