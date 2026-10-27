# security · picked accounting · group 2 of 3

## Picked
input-validation#1.1 · carried · the chunk re-verifies the paste rule at CLI `send`, MCP `send` and the wrapper's `send`, all reached files
input-validation#1.2 · carried · answer free text gets the same check in `viola answer` and the wrapper's `answer` handler, `src/cmd/answer.rs` reached
input-validation#1.3 · carried · the channel frame cap, depth, `v` check and closed param types rule over `frame.rs`, `server.rs`, `client.rs` and `tokio_client.rs`, all edited
input-validation#1.4 · carried · hook stdin's `MAX_FRAME + 1` cap, closed `HookEvent`, one-object rule and the capture arms' bounds rule over `src/cmd/hook.rs`, `hook/statusline.rs` and the agent crate's hook and dialog parsers, all edited
input-validation#1.5 · carried · the MCP handlers' raw-argument checks are a boundary the scope names, `crates/viola-mcp/src/tools.rs` edited
input-validation#1.6 · carried · CLI names through `ViolaName::try_new`, `send` / `answer` input caps, `revive --id` shape and home resolution are scope items over edited `send.rs`, `answer.rs`, `revive.rs` and `mod.rs`
input-validation#1.7 · carried · the `config.json` capped tolerant read into `Percent` / `DialogHold` is a scope item, `src/run/budget.rs` edited
input-validation#1.12 · carried · the own-state-file readers' line cap, torn count and order after strict-modes rule over edited `events.rs`, `snapshot.rs`, `stamps.rs` and `budget.rs`
input-validation#1.13 · carried · the notify path filter is a named-before-path-join site the scope lists
input-validation#1.14 · carried · the `claude agents --json` and `--version` reads' caps and closed failure rule over edited `agents.rs` and `version_gate.rs`
input-validation#1.15 · carried · the vt100 feed's `catch_unwind` degrade and bounded queue are the check the chunk re-reads in `src/run/gate.rs` and `screen.rs`
input-validation#1.16 · carried · its untrusted-output clause (bounded by `MAX_FRAME`, a row's `name` never a `ViolaName` or `target`) rules over the edited agents read
input-validation#1.17 · carried · `revive --id` and the session id argv value are a closed-shape validator the scope names, `src/cmd/revive.rs` edited
input-validation#1.18 · carried · the statusline source read's `take(MAX_FRAME)` and command shape are a scope item, `src/cmd/run.rs` edited
input-validation#1.19 · carried · the shell-out stdout cap and bounded wait are a scope item, `src/cmd/hook/statusline.rs` edited
input-validation#2.0 · carried · `MAX_FRAME` is the one const in `crates/viola-core/src/lib.rs` every bound the chunk adds reads
data-protection#1.1 · carried · its pinned key shape (`<VERSION>-<16 hex>`, truncated SHA-256) is the closed shape the scope requires before the `bin/` / `plugin/` join
data-protection#6.1 · not in scope · states what a served `link` / `unlink` appends to `events.ndjson`, a log-content rule over `src/run/links.rs`, which the chunk does not reach
data-protection#7.1 · not in scope · states the link verbs' process-log and output content in `src/cmd/link.rs` / `unlink.rs`, files the chunk does not reach
dependency-security#2.1 · carried · `tokio_client.rs` is edited, so its bound must stay behind the `tokio` feature with `viola-channel`'s normal graph tokio-free
bootstrap-phases#1.1 · carried · its client-reads-only-after-strict-modes clause is the ordering every bounded read the chunk adds or moves must keep
secret-management#1.1 · carried · the R8 strip the agents read spawns under, and the identity floor `claude_env_keep` cannot override, are values the scope names
