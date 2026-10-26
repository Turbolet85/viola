# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · its claim still holds on the changed Cargo.toml and Cargo.lock (tokio stays built only in mcp, the four sole-root bans exit 0) and it names no tokio version
established-decisions#2.1 · no · portable-pty =0.8.1 still holds on the changed Cargo.lock (the delta is tokio alone), and no pty source moved
established-decisions#3.1 · no · vt100 0.16.2 still holds on the changed Cargo.lock (the delta is tokio alone), and no screen-model source moved
established-decisions#4.1 · no · its rules live in source and ledger rows the session did not change, and no reached Cargo file carries them
established-decisions#5.1 · no · the claim of no C in the build still holds on the changed Cargo.lock (tokio 1.53.3 has the same dependency set), and the std lock and ndjson rules live in source
established-decisions#6.1 · no · tempfile 3.27.0 still holds on the changed Cargo.lock (the delta is tokio alone), and the replace helper and the licence allow list are unmoved
established-decisions#7.1 · no · interprocess 2.4.4 still holds on the changed Cargo.lock (the delta is tokio alone), and the endpoint and channel code are unmoved
established-decisions#8.1 · no · rmcp 3.4.1 still holds on the changed Cargo files (its features read server and transport-io), and the MCP tool set is unmoved
established-decisions#9.1 · no · the hook transport's sync tiers and bounds live in source the session did not change, and the hook path stays tokio-free
established-decisions#10.1 · no · the fail-open hook contract lives in source the session did not change, and the hook path stays tokio-free
established-decisions#11.1 · no · the dialog-expired line is source behaviour the session did not change, and no reached Cargo file carries it
established-decisions#11.2 · no · the measured 671 s hold is a recorded runtime fact the session does not reach, and no reached Cargo file carries it
established-decisions#12.1 · no · the pending 3600 s ceiling lives in the sync hook and wrapper the session did not change, so the bump does not reach it
