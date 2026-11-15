@row_retire n=1 row=decision/statusline-command-confined seen=2 detector=none
why: The statusline_command string no longer exists: the snapshot drops it and no viola part reads or logs it.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=2 row=decision/statusline-command-duration-exit-only seen=1 detector=none
why: Viola no longer runs the user's status-line command, so there is no shell-out to observe.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=3 row=decision/statusline-command-string-diagnostics-only seen=1 detector=none
why: The statusline_command string no longer exists in viola, so no log can carry it.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=4 row=decision/statusline-command-never-in-home-logs seen=1 detector=none
why: The statusline_command and its shell-out are gone, and shell_exit_status left the diag-line schema.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=5 row=decision/statusline-command-only-in-diagnostics seen=1 detector=none
why: The statusline_command string no longer exists in viola, so no log can carry it.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=6 row=decision/statusline-shell-out-instrumentation seen=1 detector=none
why: The viola hook statusline arm and its shell-out were removed.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=7 row=contract/hook-stdout/statusline seen=1 detector=none
why: viola hook statusline is now an unknown hook event that prints nothing.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=8 row=contract/hook-output/statusline-stdout seen=1 detector=none
why: viola hook statusline is now an unknown hook event that prints nothing.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=9 row=decision/statusline-command-never-logged seen=1 detector=none
why: Viola no longer reads a status-line source or holds a status-line command.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=10 row=decision/statusline-deadline seen=1 detector=none
why: The status-line shell-out and its STATUSLINE_DEADLINE were removed.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=11 row=decision/statusline-stdout-passthrough seen=1 detector=none
why: Viola no longer serves the status line; the wrapped status line is claude's own.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=12 row=decision/no-statusline-settings-in-telemetry seen=1 detector=none
why: run no longer passes --settings to its child and viola holds no status-line source or command; decision/child-settings-never-logged keeps the settings argument out of logs.
basis: fct-yeasicmk
because: The settings override, the status-line source read and the command were removed, so the row's subjects no longer exist.
@row_retire n=13 row=decision/rejected-statusline-stdin-runs-nothing seen=1 detector=none
why: The statusline arm, its stdin read and its shell are gone; the word is an unknown hook event that reads nothing.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=14 row=decision/statusline-settings-steps-unlogged seen=1 detector=none
why: The start no longer reads a status-line source nor writes a settings override.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=15 row=decision/statusline-silent-without-instance seen=1 detector=none
why: The statusline arm was removed; the word is an unknown hook event.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=16 row=contract/budget-span-attributes/statusline-shell-out seen=1 detector=none
why: The statusline.shell_out span and the shell_exit_status field were removed.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=17 row=contract/statusline-decision-detail/deadline seen=1 detector=none
why: The statusline arm and its deadline were removed, so it writes no hook-decision.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=18 row=contract/statusline-decision-detail/malformed-json seen=1 detector=none
why: The statusline arm was removed, so it writes no hook-decision.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=19 row=contract/statusline-decision-detail/oversize-stdin seen=1 detector=none
why: The statusline arm was removed, so it writes no hook-decision.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=20 row=contract/statusline-log-fields/hook-decision seen=2 detector=none
why: The hook_event word statusline left diag-line v1 and the arm writes no line.
basis: fct-wc3izak3
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=21 row=contract/statusline-log-fields/hook-invoked seen=1 detector=none
why: The hook_event word statusline left diag-line v1 and the arm writes no line.
basis: fct-wc3izak3
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=22 row=contract/statusline-log-fields/parse-rejected seen=1 detector=none
why: The statusline arm was removed; the word reads no stdin and writes no line.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=23 row=contract/statusline-log-fields/statusline-shell seen=1 detector=none
why: The subject statusline-shell and the field shell_exit_status left diag-line v1.
basis: fct-wc3izak3
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=24 row=contract/statusline-spans/span-chain seen=1 detector=none
why: The spans hook.statusline and statusline.shell_out were removed.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=25 row=decision/statusline-command-deadline seen=1 detector=none
why: Viola no longer runs the user's status-line command, so it has no deadline.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=26 row=contract/diag-subject/statusline-shell seen=1 detector=none
why: The subject statusline-shell left the diag-line v1 subject enum.
basis: fct-wc3izak3
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=27 row=contract/hook-decision-detail/statusline-codes seen=1 detector=none
why: viola hook statusline is an unknown hook event and writes no hook-decision.
basis: fct-vey5bl3d
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=28 row=decision/statusline-command-out-of-home-logs seen=1 detector=none
why: The statusline_command no longer exists in the snapshot or anywhere in viola.
basis: fct-yeasicmk
because: The status-line interception was removed whole, so the row's subject no longer exists.
@row_retire n=29 row=contract/replayed-snapshot/live-fields seen=1 heir=contract/replayed-snapshot/endpoint-pid-started-at detector=none
why: The snapshot no longer holds statusline_command and hook statusline serves nothing, so the item and promise name removed things.
basis: fct-yeasicmk
because: InstanceSnapshot::statusline_command and the hook statusline arm were removed, so the item cell must lose statusline_command and only a new row can carry it.
@row_set n=1 row=decision/hook-stdout-decision-body-only cell=rule seen=1 detector=none
value: viola hook stdout carries only the decision body.
basis: fct-vey5bl3d
because: The viola hook statusline arm was removed, so no hook stdout carries a status-line command's output.
@row_set n=2 row=decision/hook-instance-check-before-obs-init cell=except seen=1 detector=none
value: The --capture arm runs ahead of the check.
basis: fct-uzqm7vs5
because: The hook keeps one capture arm, hook <event> --capture, so the except names one arm.
@row_set n=3 row=decision/hook-latency-budget cell=except seen=1 detector=none
value:
basis: fct-vey5bl3d
because: viola hook statusline is gone, so the except naming it loses its subject.
@row_set n=4 row=contract/hook-stdout/capture-arms cell=promise seen=1 detector=none
value: The hidden --capture arm writes no log sink and prints nothing, except --answers' probe decision body.
basis: fct-uzqm7vs5
because: The hidden hook statusline --capture --shell probe arm was removed, leaving one capture arm.
@row_set n=5 row=decision/span-naming cell=rule seen=2 detector=none
value: Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
basis: fct-pkysvchx
because: The statusline spans were removed, so statusline leaves the closed list of span areas.
@row_set n=6 row=decision/instance-check-before-log-init cell=except seen=1 detector=none
value: The hook --capture arm runs ahead of the check.
basis: fct-uzqm7vs5
because: The hook keeps one capture arm, so the except names one arm.
@row_set n=7 row=decision/capture-arms-uninstrumented cell=rule seen=1 detector=none
value: The hidden capture arm of verify / plugin install is uninstrumented by design.
basis: fct-uzqm7vs5
because: The capture arms went from two to one, so the row states one arm.
@row_set n=8 row=decision/no-endpoint-over-replayed-snapshot cell=rule seen=1 detector=none
value: Over a replayed or absent snapshot read, endpoint lookup and the hook connect give no endpoint, so no frame is sent.
basis: fct-vey5bl3d
because: The statusline arm and its command were removed, so the clause on the statusline running no command loses its subject.
@row_set n=9 row=decision/hidden-capture-arms-uninstrumented cell=rule seen=1 detector=none
value: The hidden hook capture arm used by verify's probes reads no VIOLA_* variable, does no obs init, emits no hook lines, keeps stderr empty and always exits 0.
basis: fct-uzqm7vs5
because: The capture arms went from two to one, so the row states one arm.
@row_set n=10 row=decision/hook-capture-arms-unlogged cell=rule seen=1 detector=none
value: The hidden capture arm of viola hook (hook <event> --capture) initialises no observability and writes no lines.
basis: fct-uzqm7vs5
because: The hook statusline --capture arm was removed, leaving hook <event> --capture alone.
@row_set n=11 row=contract/span-kinds/client cell=promise seen=1 detector=none
value: The CLIENT spans are send.client, wait.block, last.client, answer.client, pause.client, release.client, cli.link, cli.unlink, channel.request (client side) and pty.spawn.
basis: fct-vey5bl3d
because: The statusline.shell_out span was removed.
@row_set n=12 row=contract/span-kinds/server cell=promise seen=1 detector=none
value: The SERVER spans are channel.dispatch (the wrapper side of every channel.request), mcp.tool_call and hook.handle.
basis: fct-vey5bl3d
because: The hook.statusline span was removed.
@row_set n=13 row=contract/child-spawn-log/subject cell=promise seen=1 detector=none
value: process-start and process-exit lines for child spawns carry a subject from a closed set.
basis: fct-vey5bl3d
because: Viola runs no shell at all, so no spawn line is a shell spawn.
@row_set n=14 row=contract/diag-line-fields/hook-events cell=promise seen=2 detector=none
value: hook_event (one of nine closed hook names), invoked_at, decision_emitted, deadline_hit, detail, duration_ms.
basis: fct-wc3izak3
because: The hook_event enum lost statusline, leaving nine closed hook names.
@row_set n=15 row=contract/diag-line-fields/process-exit cell=promise seen=1 detector=none
value: subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
basis: fct-wc3izak3
because: process-exit lost the field shell_exit_status in diag-line v1.
@row_set n=16 row=contract/diag-line-fields/process-start cell=promise seen=2 detector=none
value: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, hyprctl-probe), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
basis: fct-wc3izak3
because: The subject enum lost statusline-shell.
@row_set n=17 row=decision/sensitive-content-detail-file-only cell=rule seen=1 detector=none
value: Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
basis: fct-yeasicmk
because: The snapshot's statusline_command was removed, so it leaves the list of sensitive content.
@row_set n=18 row=contract/stdout-by-role/hook cell=promise seen=1 detector=none
value: stdout carries the decision body.
basis: fct-vey5bl3d
because: The hook statusline arm was removed, so hook stdout never carries a status-line output.
@row_set n=19 row=decision/log-schema-additive-only cell=except seen=2 fact=F4 detector=none
value: The status-line removal took the subject statusline-shell, the hook_event word statusline and the field shell_exit_status out of diag-line v1, its v staying 1.
basis: fct-wc3izak3
because: diag-line v1 lost a subject word, a hook_event word and a process-exit field with v unchanged, an exception the additive-only rule must name.
@row_add n=1 row=absence/status-line-interception area=1-obs-scope-summary fact=F1 detector=none
capability: A status-line interception: the wrapped status line is claude's own, exactly as unwrapped
basis: fct-vey5bl3d
because: Viola no longer overrides the child's settings, reads a status-line source, runs a status-line command or serves the status line, a whole capability gone.
@row_add n=2 row=absence/shell-out area=log-coverage fact=F2 detector=none
capability: A shell-out: viola runs no shell, and no diag-line subject, field or span records one
basis: fct-vey5bl3d
because: With the status-line shell-out removed viola runs no shell at all, so no log or span records one.
@row_add n=3 row=contract/hook-unknown-event/statusline area=span-trace-coverage fact=F3 detector=none
boundary: viola hook event words
item: statusline
promise: The retired word statusline is an unknown hook event: exit 0, nothing read, no line written, the home's tree left byte-equal.
basis: fct-6hiffgpi
because: The statusline arm was removed and its word now takes the unknown-event exit, a promise at the hook boundary.
@row_add n=4 row=contract/snapshot-compat/statusline-command area=span-trace-coverage fact=F5 detector=none
boundary: snapshot.json read
item: statusline_command
promise: A snapshot.json written by 0.1.0 holding statusline_command reads whole with the key skipped; the snapshot v stays 1.
basis: fct-5ykrfafm
because: The snapshot dropped statusline_command, and an older snapshot holding it must still read whole.
@row_add n=5 row=contract/ledger-stamps-compat/retired-rows area=span-trace-coverage fact=F5 detector=none
boundary: ledger stamps read
item: statusline-override, statusline-shell
promise: A ledger stamp holding the retired rows statusline-override and statusline-shell reads verified on the 21 relied rows alone; the stamp v stays 1.
basis: fct-5ykrfafm
because: The two status-line ledger rows were retired, and a stamp still holding them must read verified on the relied rows.
@row_add n=6 row=contract/replayed-snapshot/endpoint-pid-started-at area=obs-scope-summary detector=none
boundary: A snapshot.json rebuilt by replay
item: endpoint, pid, started_at
promise: A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, and run's collision check reads the heartbeat alone.
basis: fct-yeasicmk
because: The heir of contract/replayed-snapshot/live-fields, without the removed statusline_command and hook statusline.
