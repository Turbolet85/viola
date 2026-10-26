# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · the task's Not-needed paste-text validation (the chunk changes nothing typed into the PTY); send.rs is reached for its home check before the frame, which leaves the text check as it is
input-validation#1.2 · no · the task's Not-needed answer free text; answer.rs is reached for its frame gap (liveness and server check), not the text check
input-validation#1.3 · no · channel frame bounds and params are the wrapper's receiving rules in viola-channel, which the chunk does not reach; it changes only what runs before a frame
input-validation#1.4 · yes · the event path's new strict check in hook.rs and the reached statusline arm must keep the hook's fail-open rule (exit 0, no body, no stderr or line on refusal)
input-validation#1.5 · yes · the MCP frame tools' new instance check joins target into instances/<name>, so the ViolaName::try_new parse and the no-frame invalid-params refusal come before it
input-validation#1.6 · yes · it names the interim gaps this chunk closes (hook events, the stamps read, the CLI send/wait/last/answer/pause/release and mcp snapshot reads) and the home resolution that feeds the check
input-validation#1.7 · no · config.json value rules sit in units the chunk does not edit (the budget and dialog readers, obs's diagnostics_level read beside the edited open_role_file), and its check-before-read order is the chunk's own Need
input-validation#1.8 · no · the task's Not-needed Windows persistent-environment name read (src/run/env.rs is not reached)
input-validation#1.9 · yes · compiled out of every release build (no fake-agent feature), the invariant the chunk's viola-only release-build measurement holds under release-check's rules
input-validation#1.10 · yes · compiled out of every release build (no fake-agent feature), the invariant the chunk's viola-only release-build measurement holds under release-check's rules
input-validation#1.11 · no · the task's Not-needed terminal colour facts (list's human mode; src/human/style.rs is not reached)
input-validation#1.12 · yes · live_endpoint's snapshot read takes the strict check first, update_stamps reads ledger/stamps.json only under strict-modes, and every reader stays on read_snapshot until the replay fork
input-validation#1.13 · no · the chunk adds or changes no tailing, and no tail unit is in its reach
input-validation#1.14 · no · verify's PTY output and the claude agents read are the task's Not-needed (revive's preflight, list, verify runs B–E), and no reached unit is the --version probe
input-validation#1.15 · no · the task's Not-needed vt100 feed under catch_unwind and PTY output passthrough
input-validation#1.16 · no · child program resolution and revive's recorded cwd sit in unreached units (src/run/mod.rs, viola-agent-claude, src/cmd/revive.rs); run.rs is reached only at start, pin_and_plugin and the collision check
input-validation#1.17 · no · the task's Not-needed revive session-id argv and --resume shape; verify reaches only its update_stamps call
input-validation#1.18 · no · the task's Not-needed statusline source read; run.rs's reached units are start, pin_and_plugin and the collision check, not the source functions, and the source's non-trusted status is the chunk's own Need
input-validation#1.19 · yes · the statusline arm reached whole keeps its order (canonicalise, instance check, then log, stdin, budget.json, snapshot read, shell-out) and statusline_command never runs from a failing home; the shell mechanics are the task's Not-needed
