@answer n=1 entity=input-validation#1.1 v=no
why: the paste validator and typed-text rule are unchanged, the removed send.rs warning is Windows-only, and viola-pty passes 33 of 33 on =0.9.0.
@answer n=2 entity=input-validation#1.2 v=no
why: the dialog free-text set and its validate_paste_text call are unchanged, and the send and client changes removed only Windows-only constructs.
@answer n=3 entity=input-validation#1.3 v=no
why: the channel frame caps, closed-type checks and reply_of code-word rule are unchanged, and the client.rs change removed only Windows-only constructs.
@answer n=4 entity=input-validation#1.4 v=no
why: the hook stdin cap, instance check, 3605 s ceiling and statusline reading are unchanged, and this entity names no owner or PROVISIONAL mark that moved.
@answer n=5 entity=input-validation#1.5 v=no
why: the MCP tool input checks live in viola-mcp, which the change does not reach.
@answer n=6 entity=input-validation#1.6 v=no
why: the MCP stdin line cap lives in viola-mcp, which the change does not reach.
@answer n=7 entity=input-validation#1.7 v=no
why: the CLI argument and home checks are unchanged, and the verbatim-drive strip removed from canonical_home is Windows-only and not stated here.
@answer n=8 entity=input-validation#1.8 v=no
why: the config.json read rules and the 60 to 3600 hold bounds are unchanged, and the list.rs change removed only Windows-only code.
@answer n=9 entity=input-validation#1.9 v=no
why: the FAKE_AGENT_PUMP_DELAY_MS seam lives in src/cmd/run.rs, which the change does not reach, and its compile-out is unchanged.
@answer n=10 entity=input-validation#1.10 v=no
why: the FAKE_AGENT_HOOK_PANIC seam lives in src/cmd/hook/seam.rs, which the change does not reach, and the hook.rs change removed only Windows-only code.
@answer n=11 entity=input-validation#1.11 v=no
why: the colour facts are decided in src/human/style.rs, which the change does not reach, and the list.rs change removed only Windows-only code.
@answer n=12 entity=input-validation#1.12 v=no
why: the state-file read bounds, the stamps read with cli_verified false (re-read unmoved) and the budget.json read are unchanged, and the rate-limits refusal basis is not stated here.
@answer n=13 entity=input-validation#1.13 v=no
why: the notify tail-path rule is unchanged, and the reached viola-state files changed only Windows-only code.
@answer n=14 entity=input-validation#1.14 v=no
why: the child output caps, agents parse and --version read are unchanged, and the unverified-start witness is re-read unmoved.
@answer n=15 entity=input-validation#1.15 v=no
why: the vt100 feed, catch_unwind tee and FEED_CAPACITY are unchanged, the pump change is Windows-only, and viola-pty passes 33 of 33 on =0.9.0.
@answer n=16 entity=input-validation#1.16 v=no
why: name-based program resolution and the absolute-path handoff hold on Linux, and the removed EXE_SUFFIX join is Windows-only with no suffix cited here.
@answer n=17 entity=input-validation#1.17 v=no
why: the session-id shape and resume_args argv rule are unchanged, and the viola-agent-claude change removed only the Windows-only EXE_SUFFIX join.
@answer n=18 entity=input-validation#1.18 v=no
why: the statusline source read and default-home comparison are unchanged, and the canonical_home change removed only the Windows-only verbatim-drive strip.
@answer n=19 entity=input-validation#1.19 v=yes mode=stale
why: the statusline-rate-limits row this entity names now reads red (23 pass, 1 fail) on the unchanged CLI 2.1.287, so its stamped-on-2.1.287 reading has moved and its owner is 0.2.0.
@answer n=20 entity=input-validation#1.20 v=no
why: the entity is N/A because no upload, webhook or database surface exists, and the change adds none.
@answer n=21 entity=input-validation#2.0 v=no
why: MAX_FRAME and its census are unchanged, the census file is not reached, and the pinned-copy witness is re-read unmoved.
