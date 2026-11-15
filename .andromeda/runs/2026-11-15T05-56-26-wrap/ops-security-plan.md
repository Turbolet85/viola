@row_add n=1 row=absence/status-line-interception area=threat-model-summary fact=F1 detector=none
capability: Status-line interception: a settings override, --settings on the child, a status-line source read or a command run
basis: fct-vey5bl3d
because: The chunk removed the status-line interception whole, so its absence is a whole-capability fact the threat model now keeps.
@row_add n=2 row=decision/no-shell-out area=input-validation fact=F2 detector=none
name: No shell-out
rule: Every child viola starts is spawned directly from an argv, never through /bin/sh -c or any other shell.
basis: fct-vey5bl3d
because: With the status-line shell-out removed viola runs no shell at all, and no row of the plan states that ban.
@row_add n=3 row=decision/snapshot-holds-no-command area=input-validation fact=F4 detector=none
name: Snapshot holds no command
rule: The instance snapshot holds no command; a snapshot carrying a statusline_command key reads whole, the key skipped, and nothing runs from it.
guard: snapshot_holding_the_retired_command_reads_whole_skipping_it
basis: fct-5ykrfafm
because: The snapshot dropped statusline_command and an older snapshot holding it reads whole with the key skipped, a contract of the trusted state file.
@row_set n=1 row=decision/hook-unknown-event-silent-exit cell=rule seen=1 fact=F3 detector=none
value: The hook event argument maps into a closed set of nine events and no other word; an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing read or written anywhere.
basis: fct-uzqm7vs5
because: The statusline word is no longer taken apart and now takes the unknown-event exit, so the nine events stand alone.
@row_set n=2 row=decision/replayed-snapshot-yields-nothing cell=rule seen=1 detector=none
value: A snapshot replayed from the log carries no endpoint, pid or start time, so it yields no frame.
basis: fct-wc3izak3
because: The snapshot holds no statusline command any more, so the replayed snapshot's lack of one is no longer a property it has.
@row_set n=3 row=contract/cli-state-root/home-flag cell=promise seen=1 detector=none
value: Chooses which state tree, snapshot, endpoint and ledger stamps are used.
basis: fct-wc3izak3
because: The snapshot's statusline_command is gone, so the home no longer selects one.
@row_set n=4 row=contract/cli-state-root/viola-dir cell=promise seen=1 detector=none
value: Chooses which state tree, snapshot, endpoint and ledger stamps are used.
basis: fct-wc3izak3
because: The snapshot's statusline_command is gone, so the home no longer selects one.
@row_set n=5 row=decision/strict-modes-check cell=rule seen=1 detector=none
value: Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission.
basis: fct-yeasicmk
because: statusline_command is no longer read by any process, so the check's list of trusted reads loses it.
@row_set n=6 row=decision/capture-arms-isolated cell=rule seen=1 detector=none
value: The hidden capture arm reads no VIOLA_* variable, opens no channel and always exits 0.
basis: fct-uzqm7vs5
because: The capture arms went from two to one when the statusline capture arm was removed.
@row_set n=7 row=decision/home-strict-modes-before-read cell=rule seen=1 detector=none
value: --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
basis: fct-yeasicmk
because: No statusline command is read any more, so the reads the check precedes lose it.
@row_set n=8 row=decision/hook-instance-check-first cell=rule seen=1 detector=none
value: Hook event and dialog paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
basis: fct-4xv6dfgb
because: The hook statusline path was removed, leaving the event and dialog arms alone under the check.
@row_set n=9 row=decision/max-frame-shared-cap cell=rule seen=1 detector=none
value: One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps.
basis: fct-pkysvchx
because: The statusline source and output reads were removed, so the cap's consumer list loses them.
@row_set n=10 row=decision/replayed-snapshot-fallbacks cell=rule seen=1 detector=none
value: Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, and the collision check judges liveness by the heartbeat alone.
basis: fct-vey5bl3d
because: The hook statusline arm is gone, so no statusline fallback over a replayed snapshot remains.
@row_set n=11 row=decision/replayed-snapshot-inert cell=rule seen=1 detector=none
value: A snapshot that fails to parse or has a newer version is replayed from the log and carries no endpoint, pid or start time, so no frame is written from it.
basis: fct-wc3izak3
because: The snapshot holds no statusline command any more, so the replayed one's lack of it says nothing.
@row_set n=12 row=decision/code-bearing-files-rewritten-each-start cell=rule seen=1 detector=none
value: The plugin files are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy.
basis: fct-vey5bl3d
because: The per-session settings.json override is no longer written, so the plugin files are the only code-bearing files left.
@row_set n=13 row=decision/fixture-scrub-and-refusal cell=except seen=1 detector=none
value:
basis: fct-tls6mbvr
because: The StatusLine fixtures left and no fixture holds a workspace.repo, so the approved card retires the except clause.
@row_set n=14 row=decision/plugin-files-rewritten-each-start cell=rule seen=1 detector=none
value: The plugin files are rewritten at every start.
basis: fct-vey5bl3d
because: The settings override is no longer written at any start.
@row_set n=15 row=decision/user-content-only-to-diagnostics cell=except seen=1 detector=none
value: events.ndjson, wait and last results and hook.dialog carry content by design
basis: fct-wc3izak3
because: snapshot.json no longer holds statusline_command, so it no longer carries content by design.
@row_set n=16 row=decision/hook-silent-exit-zero cell=rule seen=1 detector=none
value: The hook verb and its hidden capture arm never write to stderr and always exit 0; a failing security check ends in exit 0 with no body.
basis: fct-uzqm7vs5
because: The capture arms went from two to one.
@row_set n=17 row=decision/hook-strict-modes-first cell=except seen=1 detector=none
value: The capture arm runs ahead of that check.
basis: fct-uzqm7vs5
because: The capture arms went from two to one.
@row_set n=18 row=decision/absolute-viola-path-in-commands cell=rule seen=1 detector=none
value: Every exec-form command in the embedded hooks.json or .mcp.json and in plugin install's .mcp.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH.
basis: fct-pkysvchx
because: No statusline command is written into a settings.json any more, so the rule's command list loses it.
@row_set n=19 row=decision/fixture-hygiene cell=except seen=1 detector=none
value:
basis: fct-tls6mbvr
because: The StatusLine fixtures left and no fixture holds a workspace.repo, so the approved card retires the except clause.
@row_set n=20 row=decision/snapshot-written-only-by-wrapper cell=why seen=1 detector=none
value: the endpoint in it decides where clients connect
basis: fct-wc3izak3
because: statusline_command left the snapshot and hook statusline is gone, so the reason loses them.
@row_retire n=1 row=decision/statusline-command-integrity seen=1 detector=none
why: viola runs no statusline command, so its integrity has no subject.
basis: fct-vey5bl3d
because: The hook statusline shell-out and the snapshot's statusline_command were removed.
@row_retire n=2 row=decision/statusline-only-shell-out seen=1 heir=decision/no-shell-out detector=none
why: The statusline shell-out and the settings override are gone and viola runs no shell at all.
basis: fct-vey5bl3d
because: The only shell-out was removed, so the rule becomes the ban on any shell-out.
@row_retire n=3 row=decision/statusline-sources-untrusted-read seen=1 detector=none
why: viola reads no status-line source any more.
basis: fct-vey5bl3d
because: The status-line source read was removed whole.
@row_retire n=4 row=decision/no-statusline-command-in-logs seen=1 detector=none
why: No statusline command exists in viola to be logged.
basis: fct-vey5bl3d
because: The status-line command and its log subject were removed.
@row_retire n=5 row=decision/single-shell-out seen=1 heir=decision/no-shell-out detector=none
why: The recorded command's shell-out is removed and viola runs no shell at all.
basis: fct-vey5bl3d
because: With the hook statusline shell-out gone the single exception to direct spawns is gone.
@row_retire n=6 row=decision/statusline-capture-arm seen=1 detector=none
why: The hidden statusline capture arm and its shell verdict were removed.
basis: fct-vey5bl3d
because: The hook keeps one capture arm, hook event --capture.
@row_retire n=7 row=decision/statusline-command-trust seen=1 detector=none
why: statusline_command no longer exists in the snapshot.
basis: fct-wc3izak3
because: The snapshot dropped statusline_command, so its trust has no subject.
@row_retire n=8 row=decision/statusline-deadline seen=1 detector=none
why: No statusline command is run, so its deadline is gone.
basis: fct-vey5bl3d
because: The 5 s statusline deadline was removed with the shell-out.
@row_retire n=9 row=decision/statusline-payload-fails-silent seen=2 detector=none
why: The hook statusline arm that read the payload was removed.
basis: fct-vey5bl3d
because: The statusline word now takes the unknown-event exit with no payload read.
@row_retire n=10 row=decision/statusline-shell-out seen=1 heir=decision/no-shell-out detector=none
why: The recorded statusline command is no longer run through /bin/sh -c.
basis: fct-vey5bl3d
because: The shell-out was removed and viola runs no shell at all.
@row_retire n=11 row=decision/statusline-source-read-only seen=1 detector=none
why: viola takes no statusline command from any source.
basis: fct-vey5bl3d
because: The status-line source read was removed whole.
@row_retire n=12 row=decision/statusline-source-selection seen=1 detector=none
why: viola run reads no status-line source any more.
basis: fct-vey5bl3d
because: The status-line source read was removed whole.
@row_retire n=13 row=decision/settings-override-plain-path seen=1 detector=none
why: No settings override is written any more.
basis: fct-vey5bl3d
because: The per-session settings.json override was removed.
@row_retire n=14 row=decision/statusline-override-plain-path-only seen=1 detector=none
why: No statusline override is written and no --settings is passed to the child.
basis: fct-vey5bl3d
because: The per-session settings.json override and the child's --settings were removed.
@row_retire n=15 row=decision/statusline-only-from-strict-home seen=1 detector=none
why: No statusline_command exists to be run.
basis: fct-wc3izak3
because: The snapshot dropped statusline_command and hook statusline is gone.
