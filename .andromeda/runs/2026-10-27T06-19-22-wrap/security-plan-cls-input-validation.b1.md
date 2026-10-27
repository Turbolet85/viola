# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · the paste rule and validate_paste_text are unchanged and no MCP tool changed; the MCP stdin line cap is a size bound beside 1.5, not a paste rule
input-validation#1.2 · no · the dialog free-text rule and validate_paste_text are unchanged and no answer path or MCP tool changed
input-validation#1.3 · no · no channel method changed, and the never-unbounded_depth claim stands on the changed deny.toml, which now bans that feature
input-validation#1.4 · yes · stale · fs.rs is reached whole and this arm's claims name create_private_new and replace_private there; the change bounds only replace_private_shared's compare read, and the report re-reads neither the create_private_new claim nor replace_private's write
input-validation#1.5 · yes · add-to · F1, F2, F3: the viola mcp stdin line bound (CappedLines at MAX_FRAME, an over-cap line refused whole with its mcp-stdin parse-rejected line, rmcp's unbounded read_until) joins this MCP row beside the tool inputs, which stand
input-validation#1.6 · no · the CLI stdin readers, the viola mcp home step and the MCP tools it names are unchanged; the new stdio bound sits beside 1.5
input-validation#1.7 · no · config caps and the config-json reject rule are unchanged, the census alone holds the caps, and the new mcp-stdin line is not a config line
input-validation#1.8 · no · the Windows environment-name reads are untouched and no reached file holds them
input-validation#1.9 · no · the FAKE_AGENT_PUMP_DELAY_MS seam is untouched and no env var or seam changed
input-validation#1.10 · no · the FAKE_AGENT_HOOK_PANIC seam and its hook arm are untouched and no env var changed
input-validation#1.11 · no · the colour facts are read only by viola list's human mode, and no env var changed
input-validation#1.12 · no · the named own-state reads are unchanged; the compare read in replace_private_shared belongs at the Constants sites, not in this row
input-validation#1.13 · no · notify tailing of events.ndjson, its name and symlink rules are unchanged, and no reached file tails
input-validation#1.14 · no · the --version and agents pipe caps are unchanged and the census alone holds them
input-validation#1.15 · yes · add-to · F9: the chaos test now witnesses human passthrough on the outer terminal after a caught feed panic (Unix child bytes, no viola literal on any OS; the Windows reading owed)
input-validation#1.16 · no · claude resolution, the .cmd refusal and the child cwd are unchanged, and no MCP tool changed, the mcp list read included
input-validation#1.17 · no · the session-id argv rules (is_session_id, resume_args, the logged id) are unchanged and no reached file holds them
input-validation#1.18 · no · the statusline source row is untouched, per the report's untouched list for the source and shell-out rows
input-validation#1.19 · no · the statusline shell-out row is untouched: its stdout cap is unchanged and it is only listed in the enumeration
