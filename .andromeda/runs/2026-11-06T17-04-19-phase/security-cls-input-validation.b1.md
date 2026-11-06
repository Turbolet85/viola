@answer n=1 entity=input-validation#1.1 v=no
why: the paste validators and typed-text rule sit in viola-core and the send handlers, which the chunk does not change (not_needed 1); the PasteHandle write only carries text already validated.
@answer n=2 entity=input-validation#1.2 v=no
why: dialog free-text validation is unchanged (not_needed 1) and no reached file holds its answer handler.
@answer n=3 entity=input-validation#1.3 v=no
why: channel framing and the wrapper's frame handlers are not changed (not_needed 3 and 4), and no indirect need names the envelope rule.
@answer n=4 entity=input-validation#1.4 v=yes
why: the dialog reply bound (DIALOG_HOLD_MAX + 5 s, the 3600 s ceiling) is the behaviour the chunk's live dialog hold reads near its ceiling (task item 10).
@answer n=5 entity=input-validation#1.5 v=no
why: MCP tool handlers and their input checks are not changed (not_needed 3).
@answer n=6 entity=input-validation#1.6 v=no
why: the MCP stdin line cap sits in the reader the chunk does not change (not_needed 3).
@answer n=7 entity=input-validation#1.7 v=yes
why: its closing sentences state the board::scan read checks and the socket-dir-refused check, the board read rule and socket-directory pair the chunk records the founder's word on (task item 15); the CLI argument rules themselves are untouched.
@answer n=8 entity=input-validation#1.8 v=yes
why: the dialog hold rule (DialogHold 60 to 3600, no config value makes it zero or unbounded) is what the chunk's live hold near the ceiling must honour (task item 10).
@answer n=9 entity=input-validation#1.9 v=yes
why: the compile-out of FAKE_AGENT_PUMP_DELAY_MS from release builds is what the scope's release check (refusing fake-agent and test-support) rules over, so the standing S4 does not drop it.
@answer n=10 entity=input-validation#1.10 v=yes
why: FAKE_AGENT_HOOK_PANIC is the one exemption that the G2 zero-panics gate the chunk must bring green names (task item 7).
@answer n=11 entity=input-validation#1.11 v=no
why: terminal colour facts are not touched (not_needed 11).
@answer n=12 entity=input-validation#1.12 v=yes
why: the board::read rule (a replayed snapshot gives a list item only within STALE_AFTER) is the board's list-row read rule the chunk records the founder's word on (task item 15).
@answer n=13 entity=input-validation#1.13 v=no
why: tailed-path notify reads are neither in the chunk's scope nor in its reached files.
@answer n=14 entity=input-validation#1.14 v=yes
why: its rule that an unwrapped claude agents row prints through escape_row and is never a ViolaName is the board read rule the chunk records the founder's word on (task item 15).
@answer n=15 entity=input-validation#1.15 v=yes
why: the vt100 feed under catch_unwind with byte passthrough unblocked is an indirect need the scope names (task item 5).
@answer n=16 entity=input-validation#1.16 v=yes
why: viola run must keep handing portable-pty an absolute child path it resolved itself after the bump, which the scope names (task item 5).
@answer n=17 entity=input-validation#1.17 v=no
why: the session id argv for revive and verify Run E is not touched (not_needed 10).
@answer n=18 entity=input-validation#1.18 v=no
why: the statusline source read at run start is not changed and no reached file holds it.
@answer n=19 entity=input-validation#1.19 v=no
why: the statusline shell-out is not changed, as no hook code changes (not_needed 2).
@answer n=20 entity=input-validation#1.20 v=no
why: upload, webhook and SQL rows are N/A for this local tool (not_needed 12).
@answer n=21 entity=input-validation#2.0 v=no
why: the MAX_FRAME cap and the bounded-reader census are not changed and no reached file holds a cap, so the census is only a re-check every change owes.
