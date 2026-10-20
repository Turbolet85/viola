# arch extract

## Relevance
relevant: the chunk re-measures and re-stamps four capability-ledger rows. Those rows, their probes, the stamp verdict and the recorded fixtures are architecture-owned decisions.

## Constraints
- Every `claude` behaviour viola relies on must be a measured, version-stamped ledger row. On a version without a passing stamp, viola degrades to transport-only, and no send is presumed delivered. A 2.1.302 behaviour change may therefore not be stamped through on assumption (per architecture §Design Philosophy, "Measured, never assumed").
- A version reads verified only when its stamp holds `pass` for every row of `LedgerRow::relied()`. On Unix that is all twenty-four members of `LedgerRow::ALL`, in order; on Windows it is twenty-one. "Full stamp" means that relied set. The ledger is a closed set, so adding a 25th row needs its own probe and a re-stamp of every stamped version (per architecture §Established Decisions [CLI Version Compatibility]).
- Each row's oracle is fixed in the plan:
  - `tag-escaping`: the CLI inserts `<\` into a typed `pasted_content` tag, and `viola-agent-claude` reverses it.
  - `local-command-clear`: SessionEnd `clear`, then a SessionStart with source `clear` and a new `session_id`, and no UserPromptSubmit.
  - `paste-newline`: `pasted(&probes.trusted, PROBE_NEWLINE_PASTE)`. An LF inside a bracketed paste stays inside the one prompt submitted.
  - `local-command-compact`: `local_command_compacted`. It needs a SessionStart with source `compact` under Run B's first id, no UserPromptSubmit carrying `/compact`, and `compact_latency_ms` within `COMPACT_CONFIRM_WINDOW`.

  A correction to a probe or an oracle must keep the behaviour each row gates (per architecture §Established Decisions [CLI Version Compatibility]).
- Run B is fixed at six compiled pastes, in this order: `PROBE_PROMPT`, `PROBE_LONG_PASTE`, `PROBE_TAG_PASTE`, `PROBE_NEWLINE_PASTE`, `PROBE_COMPACT_COMMAND`, `PROBE_LOCAL_COMMAND`. Each added paste goes only into rows that hold the input-box literal and no modal literal, and is bounded by `PROBE_DEADLINE`. A failed wait ends the added pastes, so one row's fix can move the rows after it in the sequence. One verify is six starts, and no run or start may be added (per architecture §Established Decisions [CLI Version Compatibility]).
- `send` confirmation is an exact match of the typed text against the normalised `prompt-submitted` `text`. The wrapper-side matcher never sees a Claude shape. `/clear` and `/compact` are confirmed only by their ledger post-conditions, never by a leading slash. A change in how the CLI submits a paste or runs a local command therefore lands in `viola-agent-claude`'s normalisation or in `LOCAL_COMMANDS`, not in `src/run/send/` (per architecture §Established Decisions [Delivery Confirmation]).
- Claude-specific shapes (hook parsing, ledger rows, the stamp merge, the `verified` verdict, the fixture scrub) live only in `viola-agent-claude`, which stays pure: no I/O and no `viola-state` dependency. The bounded spawns and the stamps file I/O stay in the root bin and in `viola_state::stamps` (per architecture §Established Decisions [Agent Coverage]).
- The real `claude` probe and `--record` run only locally. CI runs `verify` against the fake agent's modes (`--screens --turn-stop --trusted-root --dialogs --framing`), so any changed probe must keep a fake-agent replay path (per architecture §Established Decisions [CI/CD]).

## Patterns to follow
- Earlier ledger-row chunks follow one shape: a compiled probe text and a post-condition check in `viola_agent_claude::ledger`, `check(row, &Probes)` over Run B's captures, a live round on the Linux dev host that ends in `stamped <ver>  N pass  M fail`, and Windows legs owed to "Windows-only live measurements" (per architecture §Cross-cutting Patterns, capability ledger as the single gate).
- Normalisation order in `prompt-submitted` is fixed:
  1. Classify `harness` on the raw prefix.
  2. Remove the same-id `pasted_content` wrapper together with its exact frame.
  3. Un-escape `<\` before an ASCII letter or `/`.

  Re-measured paste and tag shapes are absorbed inside this order (per architecture §Established Decisions [CLI Version Compatibility], tag escaping and the long-paste wrapper).
- Stamps go through the one writer, `viola_state::stamps::update_stamps`. A merge replaces only its own version's entry and keeps unknown versions and fields. A failing row still writes its stamp, and verify exits 1 (per architecture §Standard Contracts, Ledger stamps envelope).
- Fixtures are recorded only by `viola verify --record` into `fixtures/claude/<cli-version>/`, and only when 0 rows fail. Each is scrubbed, refused whole when unclean (closed codes only), hygiene-walked, and replayed by the fake agent (per architecture §Occupied Resources, Repository).

## Anti-patterns to avoid
- Hard-coding a 2.1.302 shape as an unconditional assumption, or passing a row whose gated behaviour no longer holds. A behaviour the product can no longer rely on stays a failing row (per architecture §Cross-cutting Patterns, capability ledger as the single gate).
- Putting a Claude-specific shape into the wrapper's matcher, `classify`, or any crate other than `viola-agent-claude`, or matching local commands by a leading-slash test (per architecture §Established Decisions [Delivery Confirmation]).
- Having `run` read a `measured` field, or a window number, from the stamp. `COMPACT_CONFIRM_WINDOW` and `GATE_MAX_WAIT` stay compiled, and the stamp only validates them (per architecture §Standard Contracts, Ledger stamps envelope).

## Contract bindings
- arch ↔ security: the live round's home matters. A verify without `--home` writes `~/.viola/ledger/stamps.json`, which is strict-modes-read by `run`'s version gate. The bounds on Run B's pastes and children (no key typed into a dialog, compiled literals only) are security-ratified (per architecture §Occupied Resources, Filesystem; per architecture §Established Decisions [CLI Version Compatibility]).
- arch ↔ tests: `contract_ledger_probes` (its `ROW_IDS`, the twenty-four ids), the committed `2.1.287` set (34 files by `ls fixtures/claude/2.1.287 | wc -l`), and the fake agent's `--framing` replay bind any changed oracle to the CI replay (per architecture §Occupied Resources, Repository).
- arch ↔ obs: a malformed or strict-refused stamp reads `cli_verified:false` with one `parse-rejected{parser:"ledger-stamps"}` line (per architecture §Standard Contracts, Ledger stamps envelope).

## Acceptance criteria contributions
- `viola verify` on claude 2.1.302 on the Linux dev host prints 24 `[NN/24]` step lines and `stamped 2.1.302  24 pass  0 fail`. `LedgerRow::ALL` keeps exactly its twenty-four members, in order (per architecture §Established Decisions [CLI Version Compatibility]).
- Any changed paste, tag or local-command handling lives in `viola-agent-claude` (`hook` normalisation, `ledger`, `LOCAL_COMMANDS`), and no Claude shape is added under `src/run/send/`. Whether today's code already absorbs 2.1.302's shapes is a question for research and the live round (per architecture §Established Decisions [Agent Coverage]).
- The committed `fixtures/claude/2.1.287` set still stamps `24 pass  0 fail` under `contract_ledger_probes` against the changed probes. A `fixtures/claude/2.1.302/` set, if one is added, comes only from `viola verify --record` with 0 failing rows and passes the hygiene walk (per architecture §Occupied Resources, Repository).
- After the live round, `ledger/stamps.json` in the chosen home holds an updated 2.1.302 entry. Every other version's entry is kept unchanged (per architecture §Standard Contracts, Ledger stamps envelope).
