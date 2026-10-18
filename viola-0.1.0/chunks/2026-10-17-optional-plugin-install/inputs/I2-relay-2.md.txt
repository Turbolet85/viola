# P4 forks — 2026-10-17-optional-plugin-install (the dialog came back unanswered: "No answer from the driver within 58 min", held by viola's dialog hook)

Each fork is the founder's own word (`inputs#I1`). The recommendation is marked.

## Q1 — Install path (boundary widening; playbook pattern "Boundary widening")
How does viola's MCP-only plugin reach Claude Code? On claude 2.1.287 a local marketplace is added with
`claude plugin marketplace add <dir>` and installed with `claude plugin install viola@viola`; both write a Claude Code
settings scope under `~/.claude`.
1. **Write + print (Recommended)** — `viola plugin install` writes `<home>/marketplace/` (the manifest plus a hook-free
   plugin whose `.mcp.json` is `["--home", <home>, "mcp"]` on the canonical `current_exe`) and prints the CLI's two
   commands for the human to run. viola spawns no `claude` child and writes nothing under `~/.claude`, so no boundary
   moves. The cost: two manual steps after the verb.
2. **Spawn the CLI** — viola also runs the two commands as bounded direct `claude` children (R8 strip, `MAX_FRAME`, 5 s
   kill, user scope). One step for the user, but the CLI then writes under `~/.claude` on viola's call: a new crossing
   that viola today causes only through verify's transcript residual. It also adds two child failure modes and refusal
   codes.

## Q2 — Precedence (a departure from architecture's "is a capability-ledger row")
When a wrapped session also has the installed plugin, which `viola mcp` answers? The reading comes from which home's
`diagnostics/mcp.ndjson` gains a `process-start`.
1. **One live round (Recommended)** — measured once on 2.1.287 on this host, recorded in the chunk's `evidence/`, and
   written into architecture as a measured fact, its Windows leg owed on "Windows-only live measurements". No 24th
   `LedgerRow`, by CLAUDE.md's ruled limit "a shape no send relies on needs no probe": no send relies on which viola
   server answers, and the per-frame `v` + sender version guards a version skew. The cost: a CLI upgrade does not
   re-measure it.
2. **24th row + verify Run F** — a `plugin-precedence` `LedgerRow` stamped by a new verify run that installs the plugin
   at local scope in its own 0700 probe dir and removes it on every verify. This adds one live start and a write under
   `~/.claude` to every verify (a widening of verify's child set), a fake-agent model of installed plugins, and every row
   literal (23 → 24). Every home stamped before reads `cli_verified:false` until it is re-verified.
3. **Defer the measurement** — build the verb and the no-hooks guarantee only, and mint an owner route entry for the
   precedence reading. v1-42 then stays unclaimed, because its acceptance says the winner is measured.

## Q3 — Round writes (boundary widening)
The live round has to install the plugin into the real Claude Code configuration. May the round run
`claude plugin marketplace add` + `install`, then `uninstall` + `marketplace remove`, recording a before/after listing
of `~/.claude/plugins`?
1. **Local scope in a rig (Recommended)** — all four commands at `--scope local` from a rig dir under `target/` (the
   settings land in `<rig>/.claude/settings.local.json`), both removed after the reading, and what stays under
   `~/.claude/plugins` (likely the plugin cache) recorded as a measured residual. The smallest footprint the CLI offers.
2. **User scope** — the same round at the default user scope, written to the user's own settings and removed after.
   Closer to what a real unwrapped driver does, but it touches the user-wide settings for the round's length.
3. **Founder runs them** — the session writes the marketplace and prints the commands; the founder runs add/install and
   later uninstall/remove at the desk, and the session only drives the starts and reads the homes.

## Q4 — Start cap (live claude start cap)
How many live claude 2.1.287 starts may this chunk make? Start 1 is the precedence reading: a wrapped `viola run` with
the installed plugin present. Start 2 is the exit-1 reading: an unwrapped start whose registered `viola mcp` refuses
its home (0770) and exits 1, asking whether the CLI's start blocks. If start 2 blocks, it is recorded as a finding and
the fix is brought back to the founder before any code changes.
1. **Two starts (Recommended)** — one per reading, nothing spare; a red on either reading is recorded, and any re-run
   comes back to the founder.
2. **Three starts** — one per reading plus one spare re-run on a fixed build.
3. **One start** — the precedence reading only; the exit-1 reading stays owed (its CARRY moves to an owner entry).

## Answer
The founder's own words, live in the overseer's dialog at 2026-10-17T17:50Z, relayed verbatim by the Viola overseer:

"The founder's own words on your four P4 questions, live in the overseer's dialog at 2026-10-17T17:50Z: Q1 «Записать + напечатать» (write the marketplace and print the two CLI commands; no claude child, nothing under ~/.claude) · Q2 «Один live-раунд» (measured once, recorded as a fact; no 24th row) · Q3 «Local scope в стенде» (all four commands at --scope local from a rig dir under target/, removed after; residual measured) · Q4 «Два старта» (cap 2; a red is recorded, never re-fired). Re-read your phase run dir from disk first (this session was compacted while idle), snapshot this answer as an input, then continue P4 → P5."

Read: Q1 → option 1 (Write + print) · Q2 → option 1 (One live round, no 24th `LedgerRow`) · Q3 → option 1 (Local
scope in a rig under `target/`) · Q4 → option 1 (Two starts; a red recorded, never re-fired).
Authority (gate-contract §Scope, *Whose word*): the founder, 2026-10-17, relayed verbatim by the Viola overseer — his
own word, so the Q3 boundary widening is ratified for what the question shown to him named (the round's four
`claude plugin` commands at `--scope local` from a rig dir under `target/`, removed after, the residual under
`~/.claude/plugins` measured), and the start cap is two.
