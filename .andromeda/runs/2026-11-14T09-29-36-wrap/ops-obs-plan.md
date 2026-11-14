@row_set n=1 row=contract/diag-line-fields/process-start cell=promise seen=1 detector=D-obs-instrumentation
value: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, statusline-shell, hyprctl-probe), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
basis: fct-w4fzcahg
because: hyprctl-probe joined the diag-line subject enum with its emitter, so the row's closed subject list must name it.
@row_set n=2 row=contract/diag-line-fields/parse-rejected cell=promise seen=1 detector=D-obs-instrumentation
value: parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
basis: fct-w4fzcahg
because: hyprctl-json joined the parse-rejected parser enum with its emitter, so the row's parser list must name it.
@row_set n=3 row=contract/parse-rejected-detail/oversize cell=promise seen=1 detector=D-obs-instrumentation
value: channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME, hyprctl-json past MAX_FRAME.
basis: fct-woso52s6
because: a hyprctl reply past MAX_FRAME is refused as oversize with a parse-rejected line of parser hyprctl-json, a new parser that writes this detail.
@row_set n=4 row=decision/per-parser-parse-failure-counts cell=rule seen=1 detector=D-obs-instrumentation
value: Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
basis: fct-nma7kel7
because: the hyprctl reply parsers drop oversize and malformed input under their own parser word hyprctl-json, so the list of counting parsers must name them.
@row_set n=5 row=decision/viola-crates-log-at-info cell=rule seen=1 detector=D-obs-instrumentation
value: The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude, viola_mcp and viola_hyprland targets to INFO, or DEBUG via diagnostics_level.
basis: fct-stztm4lu
because: the viola log target list moved from 7 to 8 with viola_hyprland, so the row's target set must name it.
@row_set n=6 row=decision/os-apis-at-call-sites cell=rule seen=1 detector=D-obs-instrumentation
value: OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals.
basis: fct-o2wtfen7
because: the hyprctl spawn is instrumented at viola's own call site in the Hyprland seam, so the row's list of call-site-instrumented operations must name it.
@row_set n=7 row=decision/instrument-call-sites-not-os-internals cell=rule seen=1 detector=D-obs-instrumentation
value: OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
basis: fct-o2wtfen7
because: the hyprctl spawn is instrumented at viola's own call site in the Hyprland seam, so the row's list of call-site-instrumented operations must name it.
@row_set n=8 row=decision/parse-failures-unknown-counter cell=except seen=1 detector=D-obs-instrumentation
value: A hyprctl reply that fails its parse reads the closed error oversize or malformed with one parse-rejected line, never unknown.
basis: fct-woso52s6
because: the Hyprland seam surfaces a child-output parse failure as a closed error word, not as unknown, so the row must name that exception.
@row_set n=9 row=decision/parse-failures-surface-as-unknown cell=except seen=1 detector=D-obs-instrumentation
value: A hyprctl reply that fails its parse reads the closed error oversize or malformed with one parse-rejected line, never unknown.
basis: fct-woso52s6
because: the Hyprland seam surfaces a parse failure as a closed error word, not as unknown, so the row must name that exception.
@row_set n=10 row=decision/span-naming cell=rule seen=1 fact=F3 detector=D-obs-instrumentation
value: Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, statusline, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
basis: fct-woso52s6
because: the Hyprland seam's read is the manual skip_all span hyprland.read, a span area the row's closed area list does not yet name.
@row_add n=1 row=contract/diag-subject/hyprctl-probe area=log-coverage fact=F1 detector=D-obs-instrumentation
boundary: diag-line subject enum
item: hyprctl-probe
promise: Subject of the process-start and process-exit pair that the Hyprland seam writes around each hyprctl spawn.
basis: fct-w4fzcahg
because: hyprctl-probe is a new word of the closed diag-line subject enum, and each subject word stands as its own contract row.
@row_add n=2 row=contract/parse-rejected/hyprctl-json area=log-coverage fact=F2 detector=D-obs-instrumentation
boundary: parse-rejected diagnostic line
item: parser hyprctl-json
promise: A hyprctl reply that is oversize or malformed gives one parse-rejected line with parser hyprctl-json, written by the Hyprland seam and carrying codes only.
basis: fct-buxnqp7c
because: hyprctl-json is a new parser word of the parse-rejected line with its emitter, a promise of the log contract.
@row_add n=3 row=decision/compositor-reply-text-never-kept area=pii-scrubbing-compliance fact=F4 detector=D-obs-pii
name: Compositor reply text never kept
rule: No text from a compositor reply is kept: window titles, classes, names and every other reply string are dropped at the parse, so none reaches a log line or an error.
guard: contract_hyprland_canary_reaches_no_line_and_no_error
basis: fct-bwodkdui
because: the Hyprland seam reads a new untrusted source of user text, and the plan must hold that none of it crosses into a line or an error.
@row_add n=4 row=decision/hyprland-fixtures-keep-list area=1-obs-scope-summary fact=F5 detector=none
name: Hyprland fixtures scrubbed by a closed keep-list
rule: Hyprland reply fixtures are scrub-required by a closed per-request keep-list: only addresses and the version tag in digit form are kept, every other string becomes a placeholder, and the fixture hygiene walk refuses any fixture that breaks the list.
guard: hyprland_fixtures_pass_hygiene
basis: fct-w4fzcahg
because: recorded Hyprland replies are a new scrub-required fixture class with its own closed keep-list, which no standing row states.
@row_add n=5 row=decision/hyprland-seam-closed-errors area=7-error-capture-reporting fact=F6 detector=none
name: Hyprland seam errors are closed code words
rule: The Hyprland seam reports every failure as one of five closed code words with a fixed message: unsupported, unreachable, oversize, malformed and refused; an unresolved hyprctl reads unsupported and writes no line.
guard: hyprland_error_words_and_messages_are_fixed
basis: fct-woso52s6
because: the new seam brings its own closed error set whose words and line behaviour later callers must respect.
@row_add n=6 row=decision/hyprctl-deadline area=log-coverage fact=F7 detector=D-obs-instrumentation
name: hyprctl child deadline
rule: A hyprctl child past its 5 s deadline is killed and reaped and reads unreachable.
guard: read_past_the_deadline_kills_and_reaps_its_child
state: provisional
basis: fct-cfg2bcyo
because: the Hyprland seam bounds its child with a provisional 5 s deadline and a fixed outcome, which no standing row states.
