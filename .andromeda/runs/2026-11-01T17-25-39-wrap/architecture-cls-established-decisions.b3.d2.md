# Classified — architecture · established-decisions.b3

## Answers
established-decisions#27.1 · no · the session moves no ledger row, probe or stamp, viola-agent-claude is not reached, and the version gate it describes is unchanged
established-decisions#28.1 · no · verify's check_home_tree call and update_stamps' check_stamps order do not move; strict.rs gains only check_socket_dir and a Refused variant
established-decisions#28.2 · no · verify's Ended::Refused code list is unchanged, since verify's checks never reach check_socket_dir and the socket-dir refusal is a run and frame-verb code
established-decisions#29.1 · yes · add-to · F1: socket_dir_ready runs before any snapshot, child or Server::bind, a step the start order does not list
established-decisions#29.2 · no · the collision check's closed outcome set and its span are unchanged, and the new check enters no collision reading
established-decisions#29.3 · yes · add-to · F3, F4 and F9: each frame-writing client gains check_endpoint_dir on the recorded endpoint and a peer euid check on Linux and macOS, while the channel holds no check inside connect
established-decisions#29.4 · no · without_endpoint's no-endpoint reading is unchanged, since the new directory check runs only on a recorded endpoint and its rule lives in viola_state::liveness, which is not reached
established-decisions#29.5 · no · the collision check still takes the caller's one snapshot read, and the new start check reads the endpoint path, not the snapshot
established-decisions#29.6 · no · revive's preflight still reads the snapshot once, since the socket-dir check reads the endpoint directory's metadata and adds no replay or state-recovered line
established-decisions#29.7 · yes · add-to · F2: check_socket_dir joins the start's strict-modes checks beside check_home_tree, a sibling strict check the start now runs
established-decisions#30.1 · no · Refused gains a variant, not a derive or a new enum, so the one-enum-per-crate rule, the exception list and the owner list hold
established-decisions#30.2 · no · resolve_program's two refusals and AgentError are unchanged
established-decisions#30.3 · no · the thiserror grep count stays nine, since Refused gains a variant and no derive
established-decisions#31.1 · no · no crate added or removed, no dependency added or bumped, and the tokio edges are unchanged; the new [[test]] table adds no member
established-decisions#32.1 · no · hooks.json, .mcp.json and the pinned-copy start steps do not move, and the new check reads the endpoint directory, not the exe
established-decisions#33.1 · no · run's single exe read and its pin_bytes call hold, since the new check reads directory metadata and not the exe's bytes
