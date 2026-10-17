# P5 review card — 2026-10-17-mcp-server-for-drivers ("MCP server for drivers")

The plan is `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/plan.md`. It has 16 steps and 26 gate entries.

The four forks were answered by the founder, live in the overseer's dialog and relayed by the overseer (`inputs#I2`):
- Q1: an eighth dated gap, with MCP `list` reading agents. He ratified this widening.
- Q2: one live `claude` 2.1.287 start.
- Q3: gap-list group 2 only.
- Q4: a new switch entry after the gap entries; `v1-33` is not claimed here.

## Goal
Build the stdio MCP server `viola mcp` (rmcp 3.4, `server` + `transport-io`).
- **Tools:** `send · wait · last · answer · list`.
- **Parity:** the same results and refusals as each CLI verb's `--json`. Refusals are code-only `isError` results.
- **Excluded:** no wheel or link tool.
- **Inputs:** re-validated server-side.
- **Bounds:** every call ends below the client's timeout.

The structure:
- The frame tools use `viola-channel`'s new Tokio client, after the liveness-only pre-check (the eighth gap).
- `list` returns CLI `list --json`'s board through a provider the root bin hands in, so no crate edge points back to
  the root.
- One live start proves that a real wrapped driver gets the five tools.

## Top acceptance criteria
- `tools/list` is exactly the five tools, and no result, body or description names `release`.
- Every tool's success and refusal equal its CLI `--json` counterpart, on the rmcp stdio E2E over Paths 2–7 and
  `list`. That includes `{"refusal":"budget-paused","detail":"five-hour"}` with no cursor (CARRY 4).
- A schema violation (target, dialog_id, behavior, control character) returns a code-only `isError` and writes no
  frame.
- CARRY 1: `viola-channel` stays tokio-free, checked by a `cargo tree -p` form (measured to resolve the named
  package's features alone). `viola-channel` leaves `sync-crates.txt`.
- CARRY 2: `scripts/rmcp-features.sh` in CI's lint job.
- CARRY 3: `role_of` → `Role::Mcp`, printing no `error: internal error`.
- The live leg (`evidence/live-mcp.md`):
  - a wrapped 2.1.287 driver's `viola mcp` started;
  - its default `wait` was answered `timed-out` between 50 and 60 s;
  - the start count is 1.
- CI green on the final HEAD.

## Touchpoints
- **New:**
  - `crates/viola-mcp/{Cargo.toml, src/lib.rs, src/tools.rs, src/result.rs, tests/handlers.rs}`
  - `crates/viola-channel/src/tokio_client.rs`
  - `src/cmd/mcp.rs`
  - `crates/viola-e2e/tests/mcp_tools.rs`
  - `scripts/channel-tokio-free.sh`, `scripts/rmcp-features.sh`
  - `evidence/`
- **Modified:**
  - `Cargo.toml` / `Cargo.lock`
  - `crates/viola-channel/{Cargo.toml, src/lib.rs}`
  - `crates/viola-state/src/liveness.rs` (`live_endpoint` moved in) and `src/cmd/client.rs`
  - `src/cmd/{list,mod}.rs`, `src/main.rs`
  - `plugin/.mcp.json`, `crates/viola-agent-claude/src/lib.rs`
  - `crates/viola-e2e/Cargo.toml`
  - `.github/workflows/ci.yml`
  - `scripts/{sync-crates.txt, deny-probes.sh}`
  - `tests/{contract_lints,cli_output_plain}.rs`
  - `.config/nextest.toml` (conditional)

## Test commands (26 entries)
- **Lint ×6:** fmt, clippy, the Windows-target clippy, G1, `lint-probes`, `rmcp-features`.
- **Probes:** `channel-tokio-free`, `cargo deny check`, the sole-root loop, `deny-probes`.
- **Unit ×2:** whole, and new cases by filter.
- **Integration ×2** and **E2E ×2:** by name, then whole.
- **Booted smoke ×4:** cleanup, boot, status, cleanup.
- **Probes:** G2, `schema-check`, `pre-push`.
- **Operator:** hygiene, push, `ci.py conclusion --wait 1800`.

Baselines on the untouched tree:
- 9 green: `deny`, the loop, `deny-probes`, integration 373, E2E 165, the smoke entries.
- 5 red, each on input this chunk builds: the two scripts are absent (exit 127); `package(viola-mcp)` and
  `binary(mcp_tools)` match nothing.

The scripts' must-pass and must-fail controls are implement's one-shot run, recorded in
`evidence/instrument-controls.md`.

The live leg is an attended rig step, not an entry:
- the 2026-10-16 rig, copied byte for byte (its record shows every session stopped through the rig, none killed);
- an unstamped home and a fake-agent `builder`;
- one `claude --model haiku --allowedTools mcp__plugin_viola_viola`;
- no dialog answered and no key typed. A permission dialog is recorded as the falsifier of the tool-name prefix.

## Concretization preview — `v1-38` "The MCP server for drivers" (integration)
**Current acceptance:** "A wrapped driver session gets send, wait, last, answer and list as MCP tools with the same
results and refusals as the CLI, refusals as error results carrying codes only; no wheel or link control appears on
the tool list; inputs are re-validated server-side regardless of advertised schemas; every call returns below the
client's tool-call timeout."

**Proposed concrete form** (the outcome kept whole, with the means named):

The current sentence, plus the following.
1. On the three CI OSes, `crates/viola-e2e/tests/mcp_tools.rs` runs an rmcp 3.4 client spawning `viola mcp --home
   <home>` over fake-agent instances.
   - It reads `tools/list` as exactly `send`, `wait`, `last`, `answer`, `list`.
   - Each tool's `structuredContent` equals the CLI `--json` document less `v` for the same channel result. A success
     has `isError` false. A refusal or fault has `isError` true, and a refusal's text is `refused: <reason>`.
   - Covered: send read back; send `human-typing`, `budget-paused`/`five-hour`, `not-delivered`
     `control-character` / `empty-text`; `instance-unreachable`; wait woken and timed out; last; answer taken,
     `unknown-dialog` and `unverified-cli`; list.
   - No body or text holds `release`, a home path or a `Caused by`.
2. A non-`ViolaName` target, a non-`u64` dialog_id, an open-string behavior and a control character in text each
   return a code-only `isError` and log no `send-issued`.
3. `wait`'s `timeout_ms` is clamped to 50 000 ms, which is also its default.
4. On the Linux dev host one wrapped `claude` 2.1.287 driver session (`evidence/live-mcp.md`) started viola's MCP
   server, and its default `wait` returned between 50 and 60 s.

Never weaker: every clause of the current text stays, and "below the client's tool-call timeout" is carried by the
cap plus the live reading.

**Not claimed:**
- `v1-33`: the founder's Q4, `inputs#I2`. After the word, one dated `notes` line records "not claimed by this chunk;
  its claimer is the new entry 'Overseer switch to viola', after 'Dialog hold for the driver'".
- `v1-42`: owned by "Optional plugin install".

## For your confirmation (leans the plan takes)
Each lean below rests on an artifact.
1. **`wait` cap and default of 50 000 ms.**
   - It sits below the 60 000 ms client default read in the 2.1.287 bundle and above `send`'s 18.5 s.
   - The stdio default is a hypothesis (`So=1e8`).
   - It closes arch [MCP]'s open item.
2. **Schema violations return `{"error":"invalid-params","detail":<field>}`.** This is a new code beside
   `instance-unreachable` / `wrapper-fault`; arch names no code for it.
3. **An internal handler fault is a JSON-RPC `-32603` `internal error`** with data null, rather than an `isError`
   body.
4. **`viola mcp` is hidden from `--help`,** per the `Hook` precedent, so the `HELP` literal is unchanged.
5. **A start-time home strict-modes check exits 1** with no log (security-plan's `mcp` row). Whether a registered
   server exiting 1 blocks the CLI's start stays unmeasured: the founder capped the chunk at one start.
6. **The Tokio client behind `viola-channel[tokio]`** (architecture ×3), over test-plan §5's "sync channel client"
   sentence, which the wrap re-words.
7. **`--model haiku` on the live start,** the rig's own precedent.

## Warnings
- **Size:** 461 lines of plan prose (642 total, fence 181) against ~400. That is over the template and under the
  >50% band, and it follows the chunk's surface (a new crate, a client, five tools, three test tiers, a live leg).
- **No mutation run is listed:** the code audit at the Epoch 5 boundary grades `crates/viola-mcp`.

## Beyond the plan
1. **Route insertion** (`inputs#I2` Q3): a new markerless entry at the end of Epoch 5, after "Compact post-condition
   ledger row":
   "Dialog hold for the driver — a longer or configurable dialog hold and a record of an expired hold (gap-list
   group 2)."
2. **Route insertion** (`inputs#I2` Q4): a new markerless entry directly after row 1:
   "Overseer switch to viola — the overseer drives through viola instead of the prototype; claims v1-33 with its
   acceptance re-worded to the Linux dev host and a key typed in the session terminal."
3. **Owner for a part left out:** whether a registered `viola mcp` that exits 1 blocks the CLI's start, unmeasured
   here by the founder's one-start cap. Proposed as a CARRY on the existing entry "Optional plugin install", which
   holds the plugin-registration measurements.
4. **Owner for the eighth dated gap's closure:** a CARRY on the existing entries "Server verification before any
   frame" and "Home and code-bearing file integrity", which close the other seven.

Apply? (yes / review / cancel)
