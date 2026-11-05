@answer n=1 entity=input-validation#1.1 v=no
why: paste-text control-character and empty-text rules sit on the send path, which the chunk leaves unchanged
@answer n=2 entity=input-validation#1.2 v=no
why: dialog free-text validation is the same send-side set, which the chunk does not change
@answer n=3 entity=input-validation#1.3 v=no
why: channel frame caps, depth, version and closed types are shared framing the chunk does not touch; the client verification order it needs is carried by #1.7
@answer n=4 entity=input-validation#1.4 v=yes
why: the hook statusline arm holds the cfg(unix) shell-out the chunk makes unconditional, so its stdin cap and one-object rule govern code the chunk edits
@answer n=5 entity=input-validation#1.5 v=no
why: MCP tool-input handling and schemas are not changed by the chunk
@answer n=6 entity=input-validation#1.6 v=no
why: the MCP stdin line cap sits on rmcp transport bounding that the chunk does not touch
@answer n=7 entity=input-validation#1.7 v=yes
why: the frame-verb client sequence (refuse_untrusted, peer euid decision, server check before the frame) and the canonical_home resolution are client code the chunk edits when it drops the macOS carve-out and the Windows arms
@answer n=8 entity=input-validation#1.8 v=no
why: config.json threshold, dialog hold and claude_env_keep parsing are units the scope does not name and the chunk does not change
@answer n=9 entity=input-validation#1.9 v=yes
why: the scope removes the Windows registry name read, and the identity-floor strip must still hold on Linux whatever the registry holds
@answer n=10 entity=input-validation#1.10 v=yes
why: the scope names the two fake-agent seams as staying compiled out of release builds, and FAKE_AGENT_PUMP_DELAY_MS is one of them
@answer n=11 entity=input-validation#1.11 v=yes
why: the scope names the FAKE_AGENT_HOOK_PANIC seam as the one G2 exemption and as compiled out of release builds
@answer n=12 entity=input-validation#1.12 v=no
why: the Windows VT enable leaves with its arm, and the NO_COLOR, TERM and COLORTERM reads beside it are unchanged on Linux
@answer n=13 entity=input-validation#1.13 v=yes
why: the strict-modes check must still run before the first read under the home on Linux, which the scope names for the Unix arm
@answer n=14 entity=input-validation#1.14 v=no
why: notify tailing of events.ndjson is not named by the scope and the chunk does not change it
@answer n=15 entity=input-validation#1.15 v=no
why: child stdout reads for claude agents, the version probe and verify PTY output are not named and the chunk does not change them
@answer n=16 entity=input-validation#1.16 v=yes
why: the chaos feed-panic witness carries a Windows byte reading the chunk removes, and its Unix bytes check and every-OS absence check must survive the gated-test census re-pointing
@answer n=17 entity=input-validation#1.17 v=yes
why: the .cmd/.bat refusal and the claude shim resolution leave with their arms, and the Linux PATH resolution and cwd rules beside them must keep holding
@answer n=18 entity=input-validation#1.18 v=no
why: session-id argv through resume_args and the no-session refusal are not touched by the chunk
@answer n=19 entity=input-validation#1.19 v=no
why: the statusline source read at run start is not named, and the shell-out consumes only the command already recorded
@answer n=20 entity=input-validation#1.20 v=yes
why: the statusline shell-out is the cfg(unix) arm the chunk makes unconditional, and its no-log and output rules must hold on Linux under the NEVER-log floor the scope keeps
@answer n=21 entity=input-validation#1.21 v=yes
why: the DLL search restriction and the sideloaded ConPTY companions are named for removal, so the rules over their arms bind the removal
@answer n=22 entity=input-validation#1.22 v=no
why: the scope has no upload, webhook or database surface
@answer n=23 entity=input-validation#2.0 v=yes
why: the statusline shell-out's stdout take(MAX_FRAME) and the bounded-reader census the chunk's removals must keep green
