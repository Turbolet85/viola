@answer n=1 entity=input-validation#1.1 v=no
why: the paste-text control set and its bans are untouched and viola-core was not edited
@answer n=2 entity=input-validation#1.2 v=no
why: the dialog free-text set is the same untouched rule and viola-core was not edited
@answer n=3 entity=input-validation#1.3 v=no
why: the frame size cap, depth limit, version check and closed types are unchanged and the removed pipe endpoint and SERVER_PID_REPORTED are not named here
@answer n=4 entity=input-validation#1.4 v=no
why: the hook stdin caps, the closed events and the capture arm are unchanged and the one-attempt replace_private still writes over its own claim
@answer n=5 entity=input-validation#1.5 v=no
why: the MCP tool input checks are untouched and viola-mcp lost only SERVER_PID_REPORTED, which this entity does not name
@answer n=6 entity=input-validation#1.6 v=no
why: the MCP stdin line cap is untouched by the session
@answer n=7 entity=input-validation#1.7 v=yes mode=stale
why: the entity says the server check runs on Linux and Windows and that the home canonical form and the list read run on every OS, which the Linux-only ruling moves
@answer n=8 entity=input-validation#1.8 v=no
why: the config reads, claude_env_keep and the identity floor stay and nothing this entity names is retired
@answer n=9 entity=input-validation#1.9 v=yes mode=contradict
why: the HKCU and HKLM Environment registry name reader in src/run/env.rs is removed
@answer n=10 entity=input-validation#1.10 v=no
why: the FAKE_AGENT_PUMP_DELAY_MS seam is untouched and this entity names no fake-agent OS arm
@answer n=11 entity=input-validation#1.11 v=no
why: the FAKE_AGENT_HOOK_PANIC seam is untouched and names no OS arm
@answer n=12 entity=input-validation#1.12 v=yes mode=contradict
why: the Windows VT enable rung and its depth branch are removed, yet the entity still lists a failed Windows VT enable as a condition that yields no colour
@answer n=13 entity=input-validation#1.13 v=no
why: the state readers and their strict-modes-before-read rule still hold on the reached strict.rs and fs.rs and the entity names no Windows arm
@answer n=14 entity=input-validation#1.14 v=no
why: the notify tail rule for instance names and symlinks is untouched
@answer n=15 entity=input-validation#1.15 v=no
why: the child output caps and the claude agents parse are unchanged and the resolved-program spawn still holds
@answer n=16 entity=input-validation#1.16 v=yes mode=stale
why: the on-every-OS witness clause and the Windows-only live measurements reference move with the Linux-only ruling and the re-pointing of that route
@answer n=17 entity=input-validation#1.17 v=yes mode=contradict
why: the Windows PATHEXT search and the claude.cmd and claude.bat resolution to claude.exe with the BatchScriptChild refusal are retired with the Windows arm
@answer n=18 entity=input-validation#1.18 v=no
why: the session-id argv shape and resume_args are unchanged and the entity names no Windows or retired path
@answer n=19 entity=input-validation#1.19 v=no
why: the statusline source read at run start is unchanged and the entity names no retired arm
@answer n=20 entity=input-validation#1.20 v=yes mode=stale
why: the Unix-only and returns-None-on-Windows wording moves to plain Linux wording with shell_argv returning a fixed array, and the Windows shell-out owed line is re-pointed
@answer n=21 entity=input-validation#1.21 v=yes mode=contradict
why: the whole DLL search order and sideloaded ConPTY companions row is retired with restrict_dll_search, the sideload and pin_companions
@answer n=22 entity=input-validation#1.22 v=no
why: the entity is N/A for upload, webhook and SQL and the session does not touch that surface
@answer n=23 entity=input-validation#2.0 v=no
why: the MAX_FRAME constant and its readers are unchanged and replace_private_shared still holds its bounded compare
