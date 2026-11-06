@answer n=1 entity=input-validation#1.1 v=no
why: the paste-text validator is unchanged and the chunk adds no send surface (not_needed 1), so no rule here needs the chunk's code or tests to act.
@answer n=2 entity=input-validation#1.2 v=no
why: the dialog free-text validator is unchanged and the chunk answers no dialog (not_needed 1).
@answer n=3 entity=input-validation#1.3 v=no
why: the channel frame parameter rules belong to the wrapper's handlers, which the chunk neither changes nor extends with a method (not_needed 3 and 4; item 16 concerns the MCP tool list, not these params).
@answer n=4 entity=input-validation#1.4 v=yes
why: the dialog reply read bound (DIALOG_HOLD_MAX + 5 s, 3605 s at the ceiling) is the value the chunk's live hold reading near 3600 s re-measures.
@answer n=5 entity=input-validation#1.5 v=no
why: the MCP tool-input handlers are not changed (not_needed 3).
@answer n=6 entity=input-validation#1.6 v=no
why: the MCP stdin line cap and its reader are not changed (not_needed 3).
@answer n=7 entity=input-validation#1.7 v=yes
why: the frame verbs' check order (check_instance, live_endpoint, check_endpoint_dir, peer_admitted, server check) is the channel's existing peer and server-verification checks the live driver runs through, and peer_admitted is the seam item 13 names.
@answer n=8 entity=input-validation#1.8 v=yes
why: the dialog hold read (a whole number of 60 to 3600 s, never zero or unbounded, read not reconfigured) is the rule the chunk's hold reading near the ceiling tests.
@answer n=9 entity=input-validation#1.9 v=yes
why: FAKE_AGENT_PUMP_DELAY_MS is a test-only env marker at HEAD, which the scope's test-only marker re-check (item 12) rules over: compiled out of release and switching off no control.
@answer n=10 entity=input-validation#1.10 v=yes
why: FAKE_AGENT_HOOK_PANIC is the one exemption the G2 zero-panics gate names (item 7), and this entity's fail-open panic rule is what that exemption rests on.
@answer n=11 entity=input-validation#1.11 v=no
why: the terminal colour facts in list's human mode are not touched (not_needed 11).
@answer n=12 entity=input-validation#1.12 v=no
why: the own-state readers (snapshot, events, stamps, budget) are unchanged, and the live run's version-gate read is used by the chunk, not re-measured.
@answer n=13 entity=input-validation#1.13 v=no
why: no tailed path or notify surface is named by the scope or changed by the chunk.
@answer n=14 entity=input-validation#1.14 v=no
why: the claude agents --json parser is not touched (not_needed 10), verify's PTY runs are not run (not_needed 6), and the --version probe is a used gate the chunk neither changes nor re-measures.
@answer n=15 entity=input-validation#1.15 v=yes
why: the portable-pty bump reaches the PTY output path, and the scope requires the vt100 feed to stay under catch_unwind with byte passthrough unblocked (item 5).
@answer n=16 entity=input-validation#1.16 v=yes
why: the bump must keep viola run handing portable-pty an absolute child path it resolved itself (item 5), and the live run uses the real claude resolved from the calling PATH (item 8).
@answer n=17 entity=input-validation#1.17 v=no
why: the session id as a child argv value for revive and verify's Run E is not touched (not_needed 10).
@answer n=18 entity=input-validation#1.18 v=no
why: the statusline source read at run start is unchanged, its default-home unit in src/cmd/mod.rs is not the plain_drive unit the chunk edits, and live runs use their own --home (standing 1), so no user settings are read.
@answer n=19 entity=input-validation#1.19 v=no
why: the statusline shell-out is unchanged, and no scope item reads or re-measures it.
@answer n=20 entity=input-validation#1.20 v=no
why: there is no upload, webhook or SQL surface (not_needed 12).
@answer n=21 entity=input-validation#2.0 v=no
why: the MAX_FRAME constant and the bounded-readers census are checks every change owes through the suite; the chunk adds no reader and changes no constant.
