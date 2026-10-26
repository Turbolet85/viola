# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · prompt and reply text in events.ndjson is stored data the chunk does not change, and its budget test captures only the send-refused line
threat-model-summary#1.2 · no · dialog and tool-input content sent over hook.dialog is outside the chunk's listener, frames and tests; the admission it needs is #2.1's
threat-model-summary#1.3 · yes · config.json holds the budget thresholds the budget-paused send test runs under and the chunk must keep; the statusline, plugin and binary items are not reached
threat-model-summary#1.4 · no · the R8-stripped CLAUDE* credentials are not reached, and the NEVER-log floor for them is Not needed for this chunk
threat-model-summary#1.5 · yes · the squatted-name refusal the chunk edits (refuse_squatted) must keep cwd and pid off its error body and log, and budget.json's usage reading is what the budget-paused test relies on
threat-model-summary#2.1 · yes · the Windows admission the chunk asserts: the protected DACL for the user SID and SYSTEM, remote-client rejection, first-instance on a held name, non-inheritable handles, and the squatted-name refusal
threat-model-summary#3.1 · no · send framing and text validation are unchanged (paste-text validation is Not needed), and the budget test changes only its log capture
threat-model-summary#4.1 · no · the wheel and the stdin classifier are unchanged, and the squatted refusal is a start refusal, not a block in a running wrapper
threat-model-summary#5.1 · no · hook stdin parsing, the capture arms and the fail-open contract are Not needed, and the chunk changes no hook path
threat-model-summary#6.1 · no · the MCP tool list and inputs are unchanged, and the chunk adds no channel method
threat-model-summary#7.1 · yes · --home and VIOLA_DIR pick the endpoint that the squatted-name and first-instance tests aim at, and the chunk adds no flag or env seam that could disable the DACL
threat-model-summary#8.1 · yes · the home's OS permissions are the trust boundary, so the chunk's test-side second listener and principal stay out of the product home with relaxed modes; home access control itself is Not needed
threat-model-summary#9.1 · no · run's child resolution, its --version probe and PATH placement are unchanged, and the chunk spawns nothing new
threat-model-summary#10.1 · yes · any windows-sys feature or dev-dependency the read-back adds is pinned in workspace.dependencies and passes cargo deny, and no ignore is added for the tokio red
threat-model-summary#11.1 · yes · the windows-2025 leg, the ubuntu supply-chain job and the per-OS release build (which refuses test-only binaries) are where the chunk's assertions and gates run
