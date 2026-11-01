# Classified — architecture · established-decisions.b3

## Answers
established-decisions#27.1 · no · the ledger, its probes and its stamps are untouched per the Untouched list, and the socket check adds nothing to the version gate
established-decisions#28.1 · no · check_home_tree and its order do not move in strict.rs, and verify's home-tree and stamps checks are not reached
established-decisions#28.2 · no · verify's Ended::Refused code set is unchanged, since the socket-dir refusal sits in no verify path
established-decisions#29.1 · yes · add-to · F1: the socket-directory check runs before any snapshot, child or Server::bind, a step the start order does not list
established-decisions#29.2 · no · collision_check's read, lines and span outcome are unchanged; the socket check is a separate step with its own code and reads no snapshot
established-decisions#29.3 · yes · add-to · F2, F3, F4 and F5: frame-writing clients check the endpoint directory first, and the peer euid joins the pid and start-time check on Linux and macOS, with the wrapper's accept loop dropping refused euids
established-decisions#29.4 · no · without_endpoint and its position behind the strict-modes check at every caller are unchanged; the new endpoint check runs only on a recorded endpoint, after it
established-decisions#29.5 · no · the collision check's snapshot read and its heartbeat decision are unchanged; the socket check reads no snapshot
established-decisions#29.6 · no · revive's one preflight read and its one state-recovered line are unchanged; the socket check adds no replay
established-decisions#29.7 · no · check_home_tree's list, its order and its refusal form do not move; the socket check is not a home-tree item
established-decisions#30.1 · no · Refused gains a fixed-message variant with no derive, so the owner rule and its divergence census hold
established-decisions#30.2 · no · resolve_program and its two AgentError refusals are not reached
established-decisions#30.3 · no · the nine thiserror derives stay nine, since Refused gains a variant, not a derive
established-decisions#31.1 · yes · add-to · F9: no check sits in viola_channel::Client::connect, since viola-channel has no viola-state edge, so the checks live at its callers
established-decisions#32.1 · no · the pinned copy, its hash refusal and the plugin rewrite are not changed; the socket check adds no copy, key or hook
established-decisions#33.1 · no · pin_key, pin_bytes and pin_exe are not changed, and the socket check reads no exe bytes
