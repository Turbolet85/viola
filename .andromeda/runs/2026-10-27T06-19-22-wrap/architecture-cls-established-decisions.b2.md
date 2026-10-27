# Classified — architecture · established-decisions.b2

## Answers
established-decisions#17.1 · yes · stale · the session edited fs.rs (the replace_private_shared compare read), which holds the create_private_new and replace_private calls this claim names; the report re-reads replace_private's atomicity but names no re-read of create_private_new
established-decisions#18.1 · no · the dialog-expired append on the dialog-hold path is not in a reached file, and the report leaves the hook contract's code paths unchanged
established-decisions#18.2 · no · a recorded live round (a 671 s hold on claude 2.1.287) that this session neither re-runs nor changes
established-decisions#18.3 · no · the dialog server check is not in a reached file, and the report leaves the hook contract's code paths unchanged; the new mcp.ndjson line is an MCP stdin cut, not a dialog-hook line
established-decisions#18.4 · no · the instance check on the event and dialog paths (viola_state::strict::check_instance) is not in a reached file, and the report leaves the hook contract's code paths unchanged
established-decisions#19.1 · no · the session brings no live hold measurement and changes no hold code, and the ceiling's rule does not live in a reached file
established-decisions#20.1 · no · the wheel lives in src/run/wheel.rs and the wrapper, neither reached, and the report leaves the wheel's code paths unchanged; the new MCP stdin cap sits in viola mcp, outside the wrapper
established-decisions#21.1 · no · the link rule's code lives in src/run/links.rs and the wrapper, neither reached; the session's MCP change is the capped stdin reader and its wiring, and the report leaves the link code paths unchanged and no tool or result shape changed
established-decisions#21.2 · no · the explicit link verbs' checks run through client::refuse_untrusted in src/cmd/client.rs, not a reached file, and the report leaves the link code paths unchanged
established-decisions#21.3 · no · the link verbs' span and channel-request pair are not in a reached file, and the report leaves the link code paths unchanged
established-decisions#21.4 · no · the link line's ts is set by Utc::now in src/run/links.rs, not a reached file, and the report leaves the link code paths unchanged
established-decisions#22.1 · no · the budget gate lives in src/run/budget.rs, not a reached file, and the report leaves the budget governor's code paths unchanged
