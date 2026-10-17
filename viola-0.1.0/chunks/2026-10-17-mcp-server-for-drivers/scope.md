# Scope — 2026-10-17-mcp-server-for-drivers

**Working entry (Epoch 5 — Driver surface):** MCP server for drivers — stdio tools send/wait/last/answer/list,
CLI-parity results and code-only refusals, no wheel or link controls, calls under the tool-call timeout.
Five `CARRY:` blocks stand on the entry (`route.py pins`, line 150: 396 · 529 · 331 · 324 · 1425 chars); no
`PREREQ:`, `BLOCKED-ON:`, `CONTEXT:` or `WATCH:`. Each is folded below.

**Operator direction at take-up (`inputs#I1`):** the P4 forks are raised as dialogs, and three of them are the
founder's own word: a boundary widening, a cap on live `claude` starts, and the `v1-33` switch question its freight
names. A delegate's answer to any of the three is not his word.

## What this chunk builds

- **The `viola-mcp` crate.** It is the workspace's one Tokio crate. It holds an rmcp `=3.4.x` stdio server with
  features exactly `server` + `transport-io` (CLAUDE.md Modules; security.md Dependencies). `crates/viola-mcp` does
  not exist at HEAD, and `plugin/.mcp.json` holds `{"mcpServers": {}}` (read at take-up).
- **Five tools: `send · wait · last · answer · list`.** Each returns the same result and the same refusal as its CLI
  verb's `--json` form.
- **Refusals as code-only MCP error results.** Each is an `isError` result that carries a closed code and a fixed
  message, with no path, no upstream text and no anyhow chain (CLAUDE.md Critical Warnings, External errors).
- **No wheel or link control on the tool list.** No `pause`, `release`, `link` or `unlink` tool. `release` is a
  human verb that nothing driver-facing names (CLAUDE.md Session Learnings).
- **Server-side re-validation of every input,** whatever the advertised schemas say. This covers
  `ViolaName::try_new`, `validate_paste_text`, the typed-text rules and `empty-text`, the closed answer shapes, and
  `MAX_FRAME`.
- **Every call returns below the client's tool-call timeout.** `wait` is bounded inside the server.
- **The wiring** (verified at P3).
  - A hidden `viola mcp` subcommand runs the stdio server, following the precedent of `Hook`, hidden as "run by the
    plugin, never by a person" (`src/cmd/mod.rs:80-82`).
  - The embedded plugin's `.mcp.json` registers it by the `@@VIOLA_BIN@@` pinned path, substituted as for
    `hooks.json` (`crates/viola-agent-claude/src/lib.rs:179-189`), never by a PATH lookup.
  - The server's `from` is its own `VIOLA_NAME`, read as `own_name` reads it (`src/cmd/client.rs:105-108`), so a
    `send` or `answer` to another instance comes under the implicit link rule (`src/run/links.rs`).
  - The `mcp` obs role, its `mcp.ndjson` file and its schema value already stand (`src/obs.rs:29,55`;
    `schemas/diag-line.v1.json:14`).
- **The boundary question** (verified at P3, widened).
  - The MCP server is a new process that writes channel frames. security-plan §Security Anti-Patterns →
    Authentication admits seven dated exceptions, each "the frame after a liveness-only pre-check", and each closed to
    borrowing.
  - Its `send` / `wait` / `last` / `answer` frames therefore need, before "Server verification before any frame"
    lands, either that verification here or an eighth dated exception.
  - Its `list` tool, at parity with CLI `list`, also makes the `claude agents --json` read in the `mcp` process. The
    security plan reads "`mcp` spawns none (its `list` is not built)" (security-plan §Input Validation, Child process
    output), so that read is a second new crossing.
  - Both are boundary decisions. Per `inputs#I1` they are the founder's own word: a P4 fork, never a decisive lean.

## Folded freight (the five CARRY blocks)

1. **`deny-sync` coverage of `viola-channel`** (chunk 2026-09-24-supply-chain-and-workflow-gates, its
   research.md §Measured facts).
   - The measured mechanism, verbatim in substance: once a member enables `viola-channel[tokio]`, `viola-channel` as
     its own `deny-sync.toml` sole root false-fails, because `cargo metadata` reports its unified features.
     - Verified at P3: the pointer reads it at that chunk's `research.md:29`.
     - The form arm was re-derived on a scratch workspace: `cargo tree -p <crate>` resolves the named package's
       features alone (research.md, Mechanism re-derivations).
   - `[premise-corrected: no crate in scripts/sync-crates.txt has an edge into viola-channel (code-graph crate_edges)]`
     architecture's Build system says `viola-channel` "is covered through the sync roots that depend on it", but no
     sync root depends on it. Only the "form" arm of this CARRY is open.
   - The obligation: keep `viola-channel`'s own normal graph tokio-ban-checked by another sync root or another form.
     Never drop it from coverage.
   - `viola-mcp` is the first member to enable that feature.
2. **The rmcp feature assertion** (chunk 2026-09-24-workspace-tree-and-code-graph-planes, which moved test-plan §9
   Lint row's rmcp release-graph assertion here).
   - The assertion: `cargo tree -e features -p viola --edges normal` shows rmcp resolving to exactly `server` +
     `transport-io`.
   - It is wired as a non-vacuous lint step: it first asserts that rmcp is present. It sits beside the `deny.toml`
     `[[bans.features]]` gate (re-verified: `deny.toml:49-52` bans rmcp's `transport-streamable-http-server` and
     `auth`).
   - The operator's P4 direction there: `viola-mcp` is the assertion's true first consumer.
3. **The `mcp` process role** (chunk 2026-10-04-wait-and-last).
   - `src/main.rs` `role_of` (re-verified at `src/main.rs:92`) files every first word except `run`, `hook` and a
     leading `-` flag under `cli`. `viola mcp` would therefore read as `cli` and print `error: internal error` on a
     failure.
   - This entry names `mcp` as its own role (obs-plan §7 Main-thread catch site: `run|hook|mcp|ui` are named roles).
   - `src/obs.rs:55` already maps `ObsProcess::Mcp` to `"mcp"`.
4. **The MCP form of the `budget-paused` refusal** (chunk 2026-10-10-budget-governor, its report; Path 6 landed in
   part).
   - It is asserted nowhere yet. The channel refusal it maps is
     `{"refusal":"budget-paused","detail":"five-hour"|"seven-day"}`, with no cursor, and its hint never names
     `release`.
   - It is owed here, with the `send` tool.
5. **The switch from the prototype and `v1-33`** (re-pinned from "The board: viola list" by the review card the
   founder approved: `viola-0.1.0/chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt`, owners
   `.../inputs/I8-board-card-owners.md.txt`, placement `.../inputs/I7-relay-3.md.txt` and
   `.../inputs/I9-relay-4.md.txt`).
   - The overseer's switch from the prototype to viola is not made, and `verification-matrix.json#v1-33` is
     unclaimed.
   - `viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/gap-list.md` (re-verified present) lists
     39 items. Ten of them stop a switch, in five groups:
     1. `list` before every send (this entry);
     2. a dialog held until the overseer answers (the compiled 60 s deadline; no record of an expired hold);
     3. the screen read (CLI modals, the screen before a dialog, the context percentage);
     4. `allow <n>`;
     5. the start line and three scripts outside this repository.
   - No route entry names groups 2, 3 or 4. Two questions are the founder's at this take-up: which of them the switch
     needs, and which chunk claims `v1-33`, with its acceptance re-worded to the Linux dev host and a key typed in the
     session terminal. They are a P4 fork, his own word (`inputs#I1`).
   - The card the founder approved priced it as "not priced until he names the gap-list groups"
     (`viola-0.1.0/chunks/2026-10-11-the-board-viola-list/inputs/I8-board-card-owners.md.txt`).
   - Measured at chunk 2026-10-08 on live 2.1.287 through the product binary: the takeover by a typed key, three
     dialog kinds answered by id, and `/clear` between skills.
   - Verified at P3, the stops at HEAD:
     - Closed since the list was written:
       - group 1: `viola list` is served (`src/cmd/list.rs`);
       - item 10: resume is served (`viola revive`);
       - the terminal-replies stop that preceded all ten: terminal replies are classified non-editing
         (`src/run/wheel.rs:487-490`).
     - Still standing:
       - group 2: `DIALOG_DEADLINE` is a compiled 60 s, "PROVISIONAL, unmeasured"
         (`crates/viola-core/src/lib.rs:16-19`), and no expired-hold record exists;
       - group 3: no verb reads the screen (`src/cmd/mod.rs`, `Command`);
       - group 4: `Response` carries no permission suggestion (`crates/viola-agent-claude/src/dialog.rs:173`);
       - group 5: lies outside this repository.

## Boundaries (what this chunk does not build)

- **`viola plugin install` for an unwrapped driver, and `v1-42`** belong to "Optional plugin install".
- **Explicit `viola link` / `unlink`** belong to "Explicit session links". This chunk adds no link control.
- **Server identity before a frame** belongs to "Server verification before any frame" (Epoch 6), unless the
  founder's answer to the boundary fork puts it here.
- **The exit-cause matrix** belongs to "Exit-cause code catalogue".
- **The cross-surface sanitised-error capability `v1-47`** belongs to "Sanitised error surfaces". This chunk holds
  its own tools' errors to the code-only rule.
- **`/compact`'s post-condition row** belongs to "Compact post-condition ledger row". A `send` of `/compact` stays
  `unconfirmable`.
- **No frontend** (deferred to 0.2.0).

## Live `claude` starts

- Proving the five tools inside a wrapped driver session against the real CLI needs at least one live `claude`
  start. Verified at P3: the fake agent reads only `hooks/hooks.json` and never loads `.mcp.json`
  (`.claude/rules/verification-harness.md`, Fake agent). CI proves the server through an rmcp stdio client.
- On the dev host the bare `claude` is 2.1.296, newer than the 2.1.287 stamp. A live rig puts
  `~/.local/share/mise/installs/claude/2.1.287/` first on `PATH` (harness rule 2026-10-10).
- How many live starts this chunk may make, and whether it makes any, is a P4 fork and the founder's own word
  (`inputs#I1`).

## Capabilities in reach

- **`v1-38` "The MCP server for drivers"** (integration). Its requirement names exactly this entry's surface. It is
  the claim candidate.
- **`v1-42`** is out: "Optional plugin install".
- **`v1-33`** (manual) waits on the founder's answer to the switch fork.

## CI read at take-up (Setup 5a)

- `574ad969d6fc` (the last wrap's flip, which is HEAD): **green**, checks 15/15, `ci#39019099990` attempt 2,
  wall-clock 1151 s.
- No red to disposition.
