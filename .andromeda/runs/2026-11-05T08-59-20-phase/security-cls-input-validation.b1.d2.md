@answer n=1 entity=input-validation#1.1 v=no
why: the chunk changes no send or answer input path, so the shared validate_paste_text rule is not reached (task not-needed 1)
@answer n=2 entity=input-validation#1.2 v=no
why: dialog free-text validation is shared code the chunk does not touch (task not-needed 1)
@answer n=3 entity=input-validation#1.3 v=no
why: channel frame limits, closed types and the -32602 faults are unchanged, and the chunk's client edit is the Unix server verification this rule does not rule over
@answer n=4 entity=input-validation#1.4 v=no
why: hook stdin cap, event argument and capture arm are not edited; the statusline shell-out next to them is covered by #1.20
@answer n=5 entity=input-validation#1.5 v=no
why: MCP tool-input handlers and schemas are not changed by the chunk (task not-needed 2)
@answer n=6 entity=input-validation#1.6 v=no
why: the MCP stdin line cap is not edited and no MCP transport change is in scope
@answer n=7 entity=input-validation#1.7 v=yes
why: the frame verbs' client path (checks ordered before connect and before the frame, peer euid decision, Linux server check) is what the chunk edits, and its Linux ordering must survive removal of the Windows and macOS arms
@answer n=8 entity=input-validation#1.8 v=no
why: config.json parsing of thresholds, dialog hold and claude_env_keep is not edited and no config read changes
@answer n=9 entity=input-validation#1.9 v=yes
why: the Windows registry read of persistent-environment names in src/run/env.rs leaves with its cfg(windows) arm (task item 6), and the identity-floor strip this entity states must hold after the removal
@answer n=10 entity=input-validation#1.10 v=yes
why: the scope names the FAKE_AGENT_PUMP_DELAY_MS seam and requires it to stay compiled out of release builds (task item 20)
@answer n=11 entity=input-validation#1.11 v=yes
why: the scope names the FAKE_AGENT_HOOK_PANIC seam and its panic_location in the G2 exemption, and requires it to stay compiled out (task items 20 and 23)
@answer n=12 entity=input-validation#1.12 v=yes
why: the Windows VT enable inside the colour decision in src/human/style.rs leaves with the cfg(windows) arms, and the Linux Depth rules this entity states must hold in the edited decide function
@answer n=13 entity=input-validation#1.13 v=yes
why: the strict-modes check before the first read under the home is the rule the chunk's Unix strict-modes arm must keep (task item 4), and this entity states that read ordering
@answer n=14 entity=input-validation#1.14 v=no
why: notify tailing of events.ndjson is not edited and no tail path changes
@answer n=15 entity=input-validation#1.15 v=no
why: child output reads for claude agents, the version probe and the verify PTY runs are not changed by the chunk
@answer n=16 entity=input-validation#1.16 v=no
why: the vt100 feed under catch_unwind and the screen poisoning are not changed (task not-needed 7), and the witness's Linux assertion is untouched
@answer n=17 entity=input-validation#1.17 v=yes
why: resolve_program is edited to drop the Windows .cmd/.bat refusal and claude shim resolution (task item 6), and its Linux PATH and absolute-path rules must survive the edit
@answer n=18 entity=input-validation#1.18 v=no
why: session-id argv validation and resume_args are not edited and have no Windows-only part
@answer n=19 entity=input-validation#1.19 v=yes
why: the statusline command that the unconditional shell-out runs is decided by this source read, and the chunk must give that shell-out no new reach beyond it (task item 5)
@answer n=20 entity=input-validation#1.20 v=yes
why: the statusline shell-out is one of the cfg(unix) arms made unconditional (task item 5), and its Windows return-None arm in shell_argv leaves
@answer n=21 entity=input-validation#1.21 v=yes
why: the DLL search restriction and the sideloaded ConPTY companions this entity states live in sideload.rs and main.rs, which the chunk removes (task items 6 and 7)
@answer n=22 entity=input-validation#1.22 v=no
why: there is no upload, webhook or database surface to validate
@answer n=23 entity=input-validation#2.0 v=yes
why: the chunk edits product readers, and the bounded-reader census this entity names must keep passing for every reader the chunk keeps or adds
