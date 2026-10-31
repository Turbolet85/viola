# security-plan · picked accounting · group 2 of 2

## Picked
data-protection#1.1 · not touched · the report's edits on the run path are the `AgentError` matches in `src/cmd/run.rs` and `src/run/mod.rs` plus cap tests, and it keeps `PinError::HashMismatch` as truly named, so the home, pin and code-bearing rules still hold
data-protection#9.1 · not touched · the report's only edit on `src/cmd/verify.rs` is the `AgentError` rename, its outputs unchanged, so the fixture scrub and its closed refusal codes still hold
data-protection#10.0 · not touched · the report moves no probe run, transcript, retention or plugin-install behaviour, `src/cmd/verify.rs` moving by the `AgentError` rename alone
bootstrap-phases#1.1 · not touched · `ProtocolError::ReleaseFromDriver` still stands with the seven bodies unchanged, `fault_of` is the only client change the report names, and no manifest edge moved
secret-management#1.1 · not touched · the report adds no config key or env var, `src/run/env.rs` gains only `env_keep_keeps_no_key_past_the_frame_cap`, and `secret-scan` reads no `claude-stripped` line, so the strip and the floor still hold
error-handling#1.1 · not touched · `src/cmd/hook/statusline.rs` gains only test lines by the report, so no hook path, stderr or exit moved
error-handling#1.4 · not touched · `src/cmd/hook/statusline.rs` gains only test lines by the report, so the replayed-snapshot path still runs nothing and prints nothing
error-handling#1.5 · not touched · the report moves only `of_reply`'s reading of a peer's reply, with no line added and no public signature moved, so the failed server check's unreachable result still holds
error-handling#1.6 · not touched · the report moves only `of_reply`'s reading of a peer's reply, with no line added and no public signature moved, so the refused check's unreachable result still holds
error-handling#1.7 · proposed · its `-32603` and exit rules hold, and the wrapper fault's now-closed fields and the reply screen land beside it as facts and on the broad MCP line
error-handling#1.8 · proposed · its `--json` refusal holds as written, and the fixed-stderr exception for `answer` and `send` lands beside it as F11
error-handling#1.9 · not touched · the edits to `src/cmd/run.rs` and `src/cmd/verify.rs` are `AgentError` matches only, and the report keeps `StampsError::StrictModes` as truly named, so the start's refusals still hold
error-handling#1.10 · proposed · its fixed messages hold, and the enum census, its gate, the open divergences and the retired `CoreError` on the broad list land beside it
security-anti-patterns#1.1 · not touched · `src/cmd/client.rs` moved no public signature and the report changes only reply reading, so the verify-before-frame ban still holds
security-anti-patterns#4.1 · not touched · the agent crate's change is the `Refusal` fold alone and the report moves no ledger row, stamp or `unverified-cli` path, so the ban still holds
