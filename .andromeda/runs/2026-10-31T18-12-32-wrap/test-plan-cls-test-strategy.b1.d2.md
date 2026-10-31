# Classified — test-plan · test-strategy.b1

## Answers
test-strategy#1.1 · yes · stale · the root bin gains unit cases, and the macOS-gated server-check cases sit in reached `client.rs`, `last.rs`, `pause.rs` and `link.rs`, whose gating the report does not show holds on the changed `pause.rs`
test-strategy#1.2 · no · the change adds root test rows and no contract surface, so each standard contract keeps its suite on real endpoints, real homes and assert_cmd
test-strategy#1.3 · no · the seven critical paths and four expansion scenarios are unchanged; the session adds E2E rows, not paths or scenarios
test-strategy#1.4 · no · the six proptest parser surfaces and their inputs are untouched; the reached `viola-core` change adds an error enum, not a parser
test-strategy#1.5 · yes · stale · reached `viola_state/src/liveness.rs` gains `without_endpoint` and holds `classify`, whose pure reading the report does not show holds on the changed file (the hyperfine rows stay unmoved)
test-strategy#1.6 · no · the fault sources (set_len truncation, torn append, SIGSTOP freeze, no-EOF child, mockall seam) are unchanged; the `chaos_wait_vanish` kill moved to a helper but is still a std process kill
test-strategy#1.7 · no · the kill-line change is on the `ci` profile only (24 to 25 operands), and a site naming the kill line without a number stays true; the default profile still has none
test-strategy#1.8 · no · nothing under `fixtures` or the Claude schemas moved and no fake-agent drift test changed; `diag-line.v1.json` is a different schema
test-strategy#2.1 · yes · add-to · F22: `tests/support/start.rs` now shares `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper` across root test files, so the shared-helper list gains `start.rs`
test-strategy#3.1 · no · reached `security_negatives_server_identity.rs` still exits 21 with no frame and no line for a not-recorded wrapper, per the report, and `channel_links.rs` raw `-32602` frames are untouched
test-strategy#3.2 · no · `viola-core` gains an error enum (F18), not a paste surface; the V2 proptest and Scenario 2 are unchanged
test-strategy#3.3 · no · `src/cmd/hook.rs` is not in the change; hook stdin parsing and fail-open stand
test-strategy#3.4 · no · the MCP E2E rmcp-client layer and Scenarios 2, 5, 7 stand; the `isError` body change is not what this row states
test-strategy#3.5 · no · the CLI unit and integration layers and their §4 and §5 homes are unchanged; only exit 21 documents moved, and no CLI surface was added
test-strategy#3.6 · no · reached `security_negatives_strict_modes.rs` still shows exit 21, no frame and no line for the six frame verbs, and the `instance-unreachable` document stays; the new `strict-modes-failed` detail contradicts nothing here
test-strategy#3.7 · no · the child-spawn process-boundary layer and Scenario E2 stand; `run` passes `json` false to `Launch`, so its human pairs are byte-identical
test-strategy#3.8 · no · no CI step, workflow, script or deny rule moved; the `Cargo.toml` change adds a test binary, not a supply-chain gate
test-strategy#3.9 · no · the six proptest and fuzz parser surfaces are not in the change
test-strategy#3.10 · no · the chaos suite layer mapping stands; `chaos_wait_vanish.rs` still exists and only swaps its kill for the shared helper
test-strategy#3.11 · no · the hyperfine `max` gate, its rows, run rule and bounds are untouched
test-strategy#3.12 · no · the cross-surface E2E layer and Scenarios 2 and E3 stand; this row maps the trigger to E2E only, and the parity change is not stated here
test-strategy#3.13 · no · the CI matrix and build stages are unchanged; no CI step changed
test-strategy#3.14 · no · `viola-agent-claude`, the fixtures and their unit and contract replays are untouched
test-strategy#3.15 · no · nothing under `fixtures` moved and no contract sandbox replay changed
test-strategy#3.16 · yes · stale · reached `contract_error_bodies.rs` moved its writer enumeration (each new word a row, the `NOT_ERROR_BODIES` post-preflight row removed), and the reached `cli_error_bodies.rs` and `mcp_tools.rs` witnesses are part of the same surface
