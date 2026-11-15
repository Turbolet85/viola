@row_set n=1 row=contract/viola-hook/exit-code cell=promise seen=1 detector=D-tests-count-literals
value: viola hook <event> always exits 0.
basis: fct-yeasicmk
because: The viola hook statusline arm is removed and its word now takes the unknown-event exit, so the row must not name it as a current entry point.
@row_set n=2 row=contract/viola-hook/stdout cell=promise seen=1 detector=D-tests-count-literals
value: stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body.
basis: fct-vey5bl3d
because: The status-line hook arm and its passthrough of the user's command output are retired, so the stdout promise loses that clause.
@row_set n=3 row=decision/fake-agent-contract-suite cell=rule seen=1 detector=D-tests-count-literals
value: A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations.
basis: fct-vey5bl3d
because: The StatusLine fixtures and the fake-agent and drift status-line cases are removed, so the status line variant family leaves the list.
@row_set n=4 row=decision/fixtures-recorded-by-live-verify cell=rule seen=2 detector=D-tests-count-literals
value: The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
basis: fct-pkysvchx
because: The StatusLine fixtures and verify's --record status-line variants are removed, so the status line family leaves the recorded fixture list.
@row_set n=5 row=decision/bounded-readers-keep-the-cap cell=rule seen=3 detector=D-tests-count-literals
value: The version gate drain, the hyprctl reader and the dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command.
basis: fct-vey5bl3d
because: The status-line shell-out is removed with its bounded-readers row, so it no longer stands among the capped readers.
@row_set n=6 row=contract/viola-hook/exit-and-stderr cell=promise seen=1 fact=F3 detector=none
value: Every hook event exits 0 with empty stderr, and an unknown hook event word exits 0 reading and writing nothing, the home's tree left byte-equal.
basis: fct-6hiffgpi
because: The retired hook word now maps to no hook event and takes the unknown-event exit with nothing read or written, which the row states only for known events.
@row_set n=7 row=decision/hook-deadline-driver cell=rule seen=1 detector=D-tests-count-literals
value: viola hook is driven by std::process::Command with an Instant deadline over recorded fixture stdin.
basis: fct-vey5bl3d
because: The hook statusline arm and its tests are removed, so the driver row must not name it.
@row_set n=8 row=contract/generated-plugin/hook-commands cell=promise seen=1 detector=D-tests-count-literals
value: A restart rewrites plugin/ with absolute hook commands, never a PATH lookup.
basis: fct-vey5bl3d
because: The per-session settings.json override is retired, so a restart no longer rewrites it.
@row_set n=9 row=decision/hook-instance-check-before-config cell=except seen=1 detector=D-tests-count-literals
value: The hook <event> --capture arm stays ahead of the check.
basis: fct-uzqm7vs5
because: The hidden hook statusline --capture --shell probe arm is removed, so the capture arms go from two to one.
@row_set n=10 row=decision/fake-agent-children-own-process-group cell=rule seen=1 detector=D-tests-count-literals
value: On Unix every command the fake agent spawns, each hook, runs in a process group of its own.
basis: fct-yeasicmk
because: The fake agent's status-line command execution is removed, so it no longer spawns a status line command.
@row_set n=11 row=decision/fixture-hygiene-walk cell=except seen=2 detector=D-tests-count-literals
value:
basis: fct-tls6mbvr
because: The StatusLine variants that held workspace.repo are deleted, so the except loses its subject, as the approved P5 card row says.
@row_set n=12 row=contract/fake-agent-receipt/kind cell=promise seen=1 detector=D-tests-count-literals
value: Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, step.
basis: fct-tqnlmjlb
because: The fake agent's receipt kinds lose statusline, so the closed list must not name it.
@row_set n=13 row=decision/no-production-data-in-tests cell=except seen=2 detector=D-tests-count-literals
value:
basis: fct-tls6mbvr
because: No fixture holds a workspace.repo once the StatusLine variants leave, so the except loses its subject, as the approved P5 card row says.
@row_retire n=1 row=decision/plugin-rewritten-on-start seen=1 heir=decision/plugin-dir-rewritten-on-start detector=D-tests-count-literals
why: A start no longer writes an instance settings.json, so the row's name and rule pairing plugin and settings are false.
basis: fct-vey5bl3d
because: The per-session settings.json override is retired, leaving only the plugin rewrite, which the heir states.
@row_retire n=2 row=decision/statusline-command-needs-strict-home seen=1 detector=D-tests-count-literals
why: viola runs no status-line command at all, so there is nothing for the strict-home gate to hold back.
basis: fct-vey5bl3d
because: The status-line hook arm and its shell-out are removed whole.
@row_retire n=3 row=contract/viola-hook-statusline/passthrough seen=2 detector=D-tests-count-literals
why: viola hook statusline is no longer an entry point; its word is an unknown hook event.
basis: fct-vey5bl3d
because: The status-line interception is retired whole, its passthrough with it.
@row_retire n=4 row=decision/statusline-passthrough seen=1 detector=D-tests-count-literals
why: The statusline hook is removed; a wrapped session's status line is exactly as unwrapped.
basis: fct-vey5bl3d
because: The status-line hook arm is retired whole.
@row_retire n=5 row=decision/statusline-shell-out-frame-cap seen=1 detector=D-tests-count-literals
why: The statusline shell-out and its frame-cap test are removed.
basis: fct-vey5bl3d
because: The /bin/sh -c shell-out of the status-line arm is retired with its tests.
@row_retire n=6 row=contract/instance-settings/statusline-override seen=1 detector=D-tests-count-literals
why: No instance settings file is written any more.
basis: fct-vey5bl3d
because: The per-session settings.json override is retired.
@row_retire n=7 row=contract/harness-boot/statusline-echo seen=2 detector=D-tests-count-literals
why: viola-harness boot no longer takes --statusline-echo.
basis: fct-tqnlmjlb
because: The boot option and its planted status-line source are removed from the harness.
@row_retire n=8 row=decision/statusline-command-after-strict-modes seen=1 detector=D-tests-count-literals
why: viola runs no configured status-line command, so there is nothing to gate.
basis: fct-vey5bl3d
because: The status-line source read, the snapshot's statusline_command and the shell-out are retired.
@row_retire n=9 row=contract/hook-statusline/passthrough seen=2 detector=D-tests-count-literals
why: The viola hook statusline arm is removed.
basis: fct-vey5bl3d
because: The status-line interception is retired whole, its passthrough with it.
@row_retire n=10 row=decision/statusline-command-never-runs-unsafe seen=1 detector=D-tests-count-literals
why: The user status-line command is never run by viola at all.
basis: fct-vey5bl3d
because: The status-line arm, its shell-out and the snapshot's statusline_command are retired.
@row_retire n=11 row=decision/statusline-command-shell-and-bound seen=1 detector=D-tests-count-literals
why: The /bin/sh -c shell-out and its 5 s bound are removed; viola runs no shell.
basis: fct-vey5bl3d
because: The shell-out and STATUSLINE_DEADLINE are retired.
@row_retire n=12 row=contract/statusline-hook/stdout seen=1 detector=D-tests-count-literals
why: The wrapped statusline arm no longer exists.
basis: fct-vey5bl3d
because: The status-line hook arm is retired whole.
@row_retire n=13 row=contract/statusline-settings-source/source-file seen=1 detector=D-tests-count-literals
why: viola reads no status-line source any more.
basis: fct-vey5bl3d
because: The status-line source read is retired.
@row_retire n=14 row=contract/viola-run-start/settings-override seen=1 heir=contract/viola-run-start/settings detector=D-tests-count-literals
why: A start writes no settings.json and hands its child no --settings.
basis: fct-6hiffgpi
because: The settings override write is removed; the heir states what a start now does with settings.
@row_retire n=15 row=decision/recorder-scrub seen=2 heir=decision/recorder-scrubs-paths-and-usernames detector=D-tests-count-literals
why: The recorder no longer records status line variants, and the workspace.repo except has lost its subject.
basis: fct-tls6mbvr
because: The StatusLine fixtures and their recording are removed, so both the rule's variant family and the except are false.
@row_retire n=16 row=decision/statusline-source-read-only seen=1 detector=D-tests-count-literals
why: No status-line source exists to read or plant.
basis: fct-vey5bl3d
because: The status-line source read and the harness's planted source are retired.
@row_retire n=17 row=contract/fake-agent-hooks/status-line-command seen=1 detector=D-tests-count-literals
why: The fake agent no longer runs a status line command.
basis: fct-yeasicmk
because: The fake agent's statusline-echo mode, --statusline-stdin and status-line command execution are removed.
@row_retire n=18 row=decision/statusline-source-written-by-test seen=1 detector=D-tests-count-literals
why: No test plants a status-line source any more.
basis: fct-vey5bl3d
because: The status-line source read and the harness's planted source are retired.
@row_retire n=19 row=contract/fake-agent-receipt/statusline seen=1 detector=D-tests-count-literals
why: The fake agent's statusline receipt kind is removed.
basis: fct-tqnlmjlb
because: The receipt kinds lose statusline.
@row_add n=1 row=absence/status-line-interception area=1-test-scope-summary fact=F1 detector=none
capability: Status-line interception: no settings override, source read, hook arm or shell-out; the status line is as unwrapped
basis: fct-vey5bl3d
because: The status-line interception is retired whole, a whole capability the project no longer has.
@row_add n=2 row=decision/no-shell-out area=test-scope-summary fact=F2 detector=none
name: No shell out
rule: viola runs no shell at all: no product path spawns a system shell.
basis: fct-vey5bl3d
because: With the status-line shell-out removed, no product path spawns /bin/sh, which future work must keep.
@row_add n=3 row=decision/plugin-dir-rewritten-on-start area=1-test-scope-summary detector=D-tests-count-literals
name: Plugin rewritten on start
rule: The plugin directory is rewritten on every start.
basis: fct-vey5bl3d
because: The plugin rewrite stands while the settings half of the retired row is gone.
@row_add n=4 row=contract/viola-run-start/settings area=6-e2e-test-strategy fact=F4 detector=none
boundary: viola run start
item: settings
promise: A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal, and no default home is made under --home.
basis: fct-6hiffgpi
because: The settings override is removed and a start now adds no settings layer, which the tests hold.
@row_add n=5 row=decision/retired-snapshot-key-read-skipped area=unit-test-strategy fact=F5 detector=none
name: Retired snapshot key read past
rule: A snapshot holding a retired key reads whole with that key skipped, a written snapshot holds no retired key, and a removed snapshot key or ledger row id leaves the format version v at 1.
why: An exception to the version-bump rule: a removal of a field or row id keeps every own-format v at 1.
guard: snapshot_holding_the_retired_command_reads_whole_skipping_it
basis: fct-5ykrfafm
because: The snapshot drops statusline_command and the stamps two row ids with the envelope v kept at 1, a compat contract older homes rely on.
@row_add n=6 row=decision/recorder-scrubs-paths-and-usernames area=7-test-data-fixtures detector=D-tests-count-literals
name: Recorded hook payloads scrubbed of paths and users
rule: The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads before writing, and the test-only viola-mcp-tap and viola-hypr-tap apply the same scrub.
basis: fct-tls6mbvr
because: The scrub stands without the status line variants and the workspace.repo except, which left with the StatusLine fixtures.
@row_add n=7 row=contract/fake-agent-cli/settings area=7-test-data-fixtures fact=F6 detector=none
boundary: viola-fake-agent command line
item: --settings
promise: --settings <file> is accepted, its value consumed and ignored.
basis: fct-yeasicmk
because: The fake agent keeps accepting --settings, which verify's Runs C and D still pass, while its status-line options are removed.
