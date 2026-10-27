# Classified — tests · test-strategy.b1

## Answers
test-strategy#1.1 · yes · the chunk's own unit witnesses per property and boundary (the root bin's send and answer refusal-ordering cases, the validate_paste_text cases) live in inline mod tests as rstest matrices on all three OSes, including the exit-code rows for empty-text (13) and unknown (14)
test-strategy#1.2 · yes · the chunk's integration cases (a frame over MAX_FRAME gives -32600 and a close, the wrapper-channel control-character case, channel_links.rs, the torn-line count, the assert_cmd hook and CLI cases) run over a real interprocess endpoint, a real tempfile home and CARGO_BIN_EXE_viola
test-strategy#1.3 · yes · the Path 1 run-start test tallied on CI reads and the hook fail-open rows of the section 6 security sweep, both run against a harness-booted viola run through the outer PTY
test-strategy#1.4 · yes · the scope names the property suite (hook stdin, statusline and claude agents --json parsers at cases 512, the validate_paste_text property, the vt100 feed property under catch_unwind) and the fuzz corpus replay for any parser the chunk lands
test-strategy#1.5 · yes · the oversize-stdin fail-open must return within the spine Instant bound, the hook deadline that the hyperfine rows gate on max
test-strategy#1.6 · yes · the chaos witness is the forced vt100 panic (chaos_feed_panic.rs: send refused input-not-ready while passthrough continues) and the over-long ndjson line fault
test-strategy#1.7 · yes · the per-chunk remove-the-guard rule: each new guard whose neutralised form could start a product process (ViolaName before a path join, the pinned key, revive --id) needs a bounded Running and WITHIN witness; the mutation gate itself stays at the epoch boundary
test-strategy#1.8 · no · the contract tier replays the fake agent's default output against recorded fixtures and checks fixture schemas; the chunk adds no fixture or recorded set and changes no default fake-agent output
test-strategy#2.1 · yes · the scope's child-spawn witness plants the fake agent through tests/support/fake.rs claude_bin under spawn_on's test-scoped PATH, and its events.ndjson reads go through tests/support/ndjson.rs
test-strategy#3.1 · yes · the IPC coverage trigger: frame codec cases and the MAX_FRAME property at each side of the limit, the -32600 close, and the raw link and unlink name validation in channel_links.rs
test-strategy#3.2 · yes · the paste-injection trigger: ESC, C0, DEL and C1 refused not-delivered / control-character at the client and the wrapper, LF, CR, TAB and multi-byte UTF-8 accepted, an empty typed text refused empty-text
test-strategy#3.3 · yes · the hook stdin trigger: oversize and malformed stdin fail open (exit 0, empty stderr, no body) in the security-sweep matrix, with the assert_cmd hook cases
test-strategy#3.4 · yes · the MCP trigger: a non-ViolaName target, a non-u64 dialog_id, an open-string behavior and control characters in text refused as invalid-params with code-only payloads and no frame written, carried by the rmcp-client layer and the sweep
test-strategy#3.5 · yes · the CLI and env trigger: path-traversal names refused by ViolaName before a path join (CLI positionals, VIOLA_NAME, revive --id, a pinned key), the oversize answer JSON, and the closed config values
test-strategy#3.6 · yes · the filesystem trigger: size caps on config.json, snapshot.json, events.ndjson, budget.json, ledger/stamps.json, statusline-source.json and settings.json, the events reader's three skip counts, and the planted oversize state files
test-strategy#3.7 · yes · the child-spawn trigger: oversize or malformed claude agents --json read as unknown through the fake agent's agents_mode, the claude --version and PTY output caps, and the forced vt100 panic
test-strategy#3.8 · no · supply-chain gates run on every change; the chunk adds no dependency and lands no new parser, so no deny graph or fuzz/ lockfile moves
test-strategy#3.9 · yes · the property-test trigger: proptest at cases 512 for the hook stdin, statusline and agents parsers, the MAX_FRAME and paste properties, and the vt100 feed property
test-strategy#3.10 · yes · the chaos-test trigger: the forced vt100 panic in tests/chaos_feed_panic.rs and the over-long ndjson torn-line fault
test-strategy#3.11 · yes · the hook deadline that the oversize-stdin fail-open must stay inside is the bound the perf rows gate on max
test-strategy#3.12 · yes · cross-surface parity: the paste rule must refuse the same way at CLI send and at MCP send (not-delivered, control-character, invalid-params)
test-strategy#3.13 · no · the CI matrix build runs on every change and the chunk adds no OS-only check; its unit and integration witnesses run on all three OSes through 1.1 and 1.2
test-strategy#3.14 · yes · the bounded claude agents --json parser, its closed status enum and the hook payload bounds stay in viola-agent-claude, so every recorded CLI version's unit fixture replay must still hold
test-strategy#3.15 · no · the contract-against-sandbox tier compares the fake agent's default output with recorded fixtures; the chunk adds no fixture and changes no default output, and agents_mode is a child-spawn fault seam owed under 3.7
test-strategy#3.16 · yes · the chunk's refusals and config parse-rejected lines must carry codes only (MCP invalid-params, control-character, one line per key), which the sanitisation and log rule rules over; the per-E2E scan itself is standing S9
