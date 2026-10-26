# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · a link carries no typed text, so the paste-text rule (send.text, validate_paste_text, empty-text) is Not needed
input-validation#1.2 · no · the chunk carries no dialog free text, so the answer free-text rule is Not needed
input-validation#1.3 · yes · the channel link and unlink params (driver, driven) take ViolaName::try_new, the frame stays under MAX_FRAME with serde_json's default depth, a driven that is not the serving wrapper's name or a driver equal to driven is -32602 before any append, and the raw-channel name negative in tests/channel_links.rs tests it
input-validation#1.4 · no · hook stdin is Not needed, since no hook arm changes
input-validation#1.5 · yes · with link and unlink off the tool list, an MCP call to either meets this entity's unknown-tool refusal (JSON-RPC -32602), the result the chunk's v1-13 link half states
input-validation#1.6 · yes · the verbs' driver and driven arguments take parse_viola_name, their home takes resolved_home with canonicalisation before any snapshot read, and their snapshot read needs a dated gap in the closed interim list or the check_instance check
input-validation#1.7 · no · the chunk reads no config.json key and names no threshold, dialog-hold or start read among its units
input-validation#1.8 · no · the Windows persistent-environment name read is Not needed, since the verbs spawn no child
input-validation#1.9 · no · the seam is hold_pump_start in src/cmd/run.rs, a unit the chunk does not edit, and its link tests name no FAKE_AGENT_PUMP_DELAY_MS use
input-validation#1.10 · no · the seam is in src/cmd/hook/seam.rs, which the chunk does not reach, and its link tests name no FAKE_AGENT_HOOK_PANIC use
input-validation#1.11 · no · the colour facts are read only by viola list's human mode, and the chunk names no styled output and reaches no style module
input-validation#1.12 · yes · the link and unlink event lines are read back through the one events reader and apply_link_line, the logged names are re-parsed on display (the second CARRY, against this rule's no-reparse wording), the snapshot links write is the wrapper's alone, and the verbs' live_endpoint read needs a dated gap or the check_instance check
input-validation#1.13 · no · no reached file tails a path, and the board shows no link
input-validation#1.14 · no · the claude agents --json read is Not needed, since list is not changed, and the verbs spawn no child
input-validation#1.15 · no · PTY output and the vt100 feed are Not needed, and the run.rs edit is the channel dispatch unit only
input-validation#1.16 · no · the verbs spawn no child and the link method only records a pair, so child executable resolution is not reached
input-validation#1.17 · no · the session-id argv shape and resume_args are Not needed, since the chunk builds no resume argv
input-validation#1.18 · no · the statusline source read at run start is Not needed, since no statusline path changes
input-validation#1.19 · no · the statusline shell-out is Not needed, since no statusline path changes
input-validation#1.20 · no · the DLL-search restriction and the sideloaded ConPTY companions are Not needed
input-validation#1.21 · no · the chunk adds no upload, webhook or database surface
