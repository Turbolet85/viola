# Classified — test-plan · test-strategy.b1

## Answers
test-strategy#1.1 · no · the inline unit tests stay in viola-mcp and viola-state, both among the six crates, the exit-code table is unchanged (exit 11 sits in 10 to 14), and CI is green on three OSes, so the claim holds on the reached stdin.rs and fs.rs
test-strategy#1.2 · no · every Standard Contract keeps its crate-level tests/ suite (viola-mcp handlers.rs is untouched) and the new stdin cases are inline unit cases or viola-e2e cases, not integration suites
test-strategy#1.3 · no · the seven critical paths and four expansion scenarios are unchanged, and the reached chaos and mcp_tools cases stay E2E cases that add no path or expansion scenario
test-strategy#1.4 · yes · add-to · the property-test trigger now also covers MCP stdin framing at MAX_FRAME plus or minus one, held by literal unit cases in stdin::tests and not by a proptest, which the six-surface proptest list must carry
test-strategy#1.5 · no · the hyperfine hook deadlines and the heartbeat classify are unchanged, and the new 5 s and 8 s kill deadlines are test bounds, not performance budgets
test-strategy#1.6 · no · the feed-panic test keeps its fault source (a forced vt100 panic that the fault list never named) and the session adds only an outer-terminal passthrough assertion, which the level's fault-source claim does not describe
test-strategy#1.7 · no · the remove-the-guard rule and the Running-bounded G4 and G10 witnesses are unchanged, and the new stdin cap's witness is bounded by its own 8 s kill
test-strategy#1.8 · no · the Contract level's fixture comparisons and jsonschema checks are unchanged, and the new contract_bounded_readers runs in the E2E layer under run --e2e and replays no fixture
test-strategy#2.1 · no · the root contract prefix and the viola-e2e mcp prefix already admit the new contract_bounded_readers binary and the viola-e2e mcp_tools file, and the entity names no binary
test-strategy#3.1 · no · the IPC vector's named locations (the channel and MCP impostor cases, the channel negatives and the hook impostor rows) are unchanged, and the MCP stdin text the session changed is not described by this row
test-strategy#3.2 · no · validate_paste_text in viola-core and Scenario 2 are untouched by the session
test-strategy#3.3 · no · the hook stdin boundary and its Security sweep rows are unchanged, and the new Security sweep negative is an MCP stdin line, not a hook input
test-strategy#3.4 · no · the E2E layer and the Security sweep location still hold the new MCP stdin negative, which mcp_tools drives inside the viola-e2e MCP suite
test-strategy#3.5 · no · no CLI flag, env var or exit code was added, so the CLI and env vector's locations are unchanged
test-strategy#3.6 · no · replace_private_shared keeps its outcome for an equal, longer, shorter, different or absent target, so the filesystem claim holds on the reached fs.rs and the strict-modes negatives are unchanged
test-strategy#3.7 · yes · add-to · the child-spawn vector's passthrough witness now sits in the chaos suite, the child's bytes arriving on the outer terminal after a forced feed panic, outside this row's section 5 and Scenario E2 locations
test-strategy#3.8 · no · cargo deny and the ban probes still run as the row names, the reached deny.toml and deny-probes.sh still hold the sole-root tokio ban and prove each ban fires, and the new serde_json unbounded_depth ban is one more ban they prove
test-strategy#3.9 · yes · add-to · the property-test trigger's MAX_FRAME plus or minus one framing now reaches the MCP stdin reader, held by literal unit cases in stdin::tests and not by a proptest, which this row's section 4 proptest and section 9 fuzz locations do not carry
test-strategy#3.10 · no · the chaos suite named as the location holds the reached chaos_feed_panic test, whose new outer-terminal passthrough assertion sits inside it, and the row names no assertion
test-strategy#3.11 · no · the performance budgets and hyperfine gates are unchanged, and the new kill deadlines are test bounds rather than budgets
test-strategy#3.12 · no · no Scenario 2, Scenario E3 or cross-surface parity step changed, and the MCP stdin negative is single-surface
test-strategy#3.13 · no · the CI matrix, workflows and caches are unchanged, and the new cases ran green on all three CI OSes
test-strategy#3.14 · no · viola-agent-claude and the CLI-version fixtures are untouched by the session
test-strategy#3.15 · no · the sandbox contract tests still sit in the section 6 Contract suite and section 7, and the new contract_bounded_readers runs in the E2E layer without a sandbox, so the row's layer and location hold
test-strategy#3.16 · no · the canary check sits in the named section 6 error-sanitization location, and the new mcp-stdin parser value is a log-shape change the section 3 logs location still covers, with the new line holding no byte of the over-long line
