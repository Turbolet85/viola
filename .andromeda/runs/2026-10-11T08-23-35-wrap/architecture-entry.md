
## 2026-10-11-the-board-viola-list — viola list as landed: a caller-side board, wrapped rows only
**Section:** §Standard Contracts (Channel methods, Session liveness, Snapshot envelope); §Established Decisions ([API Style], [Database / State Store]); §Occupied Resources (Binary, subcommands and exit codes; Filesystem: `instances/`, `budget.json`, `config.json`, `diagnostics/`); §Conventions (exit `1`, exit `21`); §Infrastructure Patterns → Project directory structure
**Change:**
- `list` as landed reads `<home>/instances/` (`viola_state::board::scan`, then `board::read`) and `budget.json` (through `read_budget_stamped`), contacts no wrapper, and reads, spawns and parses no `claude agents --json`: every item is `wrapped: true`, `status: "unknown"`, no unwrapped row. Was: `list` reads `claude agents --json` itself; that read, the unwrapped rows and idle/busy STATUS are now owed to "Unwrapped sessions on the board", the join-field ledger row and probe to "Paste newline ledger row".
- The replay fallback has two product readers (was one): `viola revive`'s preflight and `board::read`, which gives a `live` item for a replayed snapshot only with a beat at most `STALE_AFTER` (5 s) old, never `stale`.
- `events::read_from` gains `board::log_pass`; `viola list` is the first surface that shows the three skip counts (was: no surface shows them, owed to the board entry), summed over every scanned instance, gone ones included.
- Three readers check the instance files (was two): `list`'s scan runs `check_path` on the home and `instances/` and `check_instance` per instance in name order, before any open and before its log; a failure refuses the whole board, exit 21, no log, no line, the home's mode as found.
- `budget.json` has one checked read, `read_budget_stamped` (`{reading, read_at}`), with two product paths: the gate's `read_budget`, which wraps it, and `list`. `config.json`'s thresholds gain `list` as a second reader through `read_thresholds`.
- `diagnostics/`: `list` is a `cli` producer under `VIOLA_NAME` only (self pair, `state-entry`, `state-recovered`, `config-json`); none on a refused scan.
- Exits: `list [--json]` → 0 / 21 / 1 / 2; exit 21 also names `list`'s refusal; exit 1 covers an unlistable `instances/` and a failed log start, a per-instance read error giving no row.
- The tree: `viola-state` gains `board.rs` and `read_budget_stamped`; `src/human.rs` gains `escape_row`, the two state pairs and the board writers; `cmd/list.rs`.
- The link/unlink hold moved to "Explicit session links".
**Why:** the chunk landed the cut the operator directed; the review card the founder approved names every held part's owner. The whole-board refusal and its no-line shape are the operator's answer at planning, not ruled by the founder; the exit-1 paths are implement's decisions, accepted by the operator at this wrap.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/
