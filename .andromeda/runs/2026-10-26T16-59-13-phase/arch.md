# arch extract

## Relevance
relevant — the chunk changes the home's file discipline (strict-modes order, single writers, the pinned copy and the plugin folder) that arch's §Occupied Resources → Filesystem, §Established Decisions and §Standard Contracts own, and it adds call sites in `viola-state` and the root bin.

## Constraints
- Every home reader runs its check before it opens a role file, log or snapshot, "because opening a log narrows an existing home" to 0700, so a check made later passes a 0770 home. Arch lists four checked readers today (`viola list`, `viola revive`, `hook statusline`, `viola link` / `viola unlink`) and counts `check_instance` call sites by the rule `check_instance(` under `src/` (three today). This chunk adds readers to that list and sites to that count, and the wrap amends both (per architecture §Occupied Resources → Filesystem, `instances/<ViolaName>/`).
- Arch records `check_instance` as proven on Unix only: "No case widens the DACL of an instance tree on Windows yet; that case is owed to the working-route entry 'Home and code-bearing file integrity'" (per architecture §Occupied Resources → Filesystem, `instances/<ViolaName>/`).
- Single writers:
  - `snapshot.json` is written only by that instance's wrapper (per architecture §Standard Contracts, Instance snapshot).
  - `ledger/stamps.json` has ONE writer, `viola_state::stamps::update_stamps`, called only by `viola verify` (per architecture §Standard Contracts, Ledger stamps envelope).
  - Readers use `read_stamps`, except `run`'s gate, which goes through `read_stamps_strict`; a refusal there is `cli_verified:false` plus one `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}` (per architecture §Occupied Resources → Filesystem, `ledger/stamps.json`).
  - Whether `update_stamps`' own read already runs the strict check is research's question.
- Every replaced file goes through `viola_state::fs::replace_private`, which sets the mode before any byte is written; no call site carries its own temp + rename. `replace_private_shared` is for files every concurrent start of one version writes the same way (the pinned copy, the plugin files, the ConPTY companions): a failed replace counts as done when the target already holds those exact bytes (per architecture §Established Decisions, [Snapshot writer]). Whether the `pin_exe` race (CARRY 16) runs on this path, and how it fails, is research's question.
- Pinned copy and plugin (per architecture §Established Decisions, [Deployment / Distribution]):
  - `run` copies its exe to `bin/<CARGO_PKG_VERSION>-<16-hex SHA-256>/` when the copy is absent. It re-hashes an existing copy before reusing it. A mismatch refuses the start (exit 1, `pinned-hash-mismatch`) and leaves the file as found.
  - `run` rewrites `plugin/<version>-<hash>/` on every start, with every exec-form `command` set to the copy's absolute forward-slash path. Hooks and MCP never search PATH.
  - A mismatched ConPTY companion never refuses the start: it is left as found and the child runs on the inbox ConPTY.
  - Whether a failed owner/DACL check on `conpty/` (CARRY 5) gets the same fail-open degrade is not settled by arch. It is a widening question for P4, not a default.
- Crate placement (per architecture §Infrastructure Patterns → Crate dependency direction):
  - The strict check lives in `viola-state`'s `strict` module: libc for owner and mode on Unix, windows-sys for owner and DACL on Windows.
  - `fs::create_private_dir` gives the topmost folder it creates outside the user profile the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` DACL.
  - `viola-agent-claude` stays pure, with no I/O.
  - `viola-mcp` reaches the check only through `viola-state`.
- Tokio only in `viola-mcp`. New checks on the `run`, `hook` and CLI-verb paths are sync std code, and the sync crates stay tokio-free (per architecture §Established Decisions, [Concurrency / Backend Framework]).
- Refusal surfaces (per architecture §Conventions, CLI exit codes / `viola hook` exit codes / MCP):
  - Exit 21 is "instance unreachable". Among strict-modes refusals it names only `viola list`, `viola plugin install`, `viola link` and `viola unlink` today.
  - `viola hook` always exits 0.
  - MCP refusals are `isError` results shaped like CLI `--json`.
  - Adding the frame verbs' refused check to exit 21 amends this row. A new exit-cause code belongs to "Exit-cause code catalogue", not this chunk.
- Snapshot read and replay (per architecture §Standard Contracts, Snapshot envelope):
  - `replay::read_snapshot_or_replay` treats `endpoint`, `pid` and `started_at` as absent, and its two product readers run only after `check_instance`.
  - `read_snapshot` has three product callers by the rule `read_snapshot(` under `src/` outside test modules. The CLI frame verbs and `viola mcp` read the snapshot inside `liveness::live_endpoint`.
  - Arch records the readers' move as owed to this entry (per architecture §Established Decisions, [Message Broker / IPC]). The scope holds it for the founder's P4 fork, so the plan neither includes nor excludes it.

## Patterns to follow
- `src/cmd/link.rs`'s `link_call` order: `check_instance` on the home and `instances/<driven>` before `client::start`, then the liveness pre-check (`live_endpoint`), then the server check, then one frame. A refusal is exit 21 with no line written and the mode left as found. This is the template for `send`, `wait`, `last`, `answer`, `pause` and `release` (per architecture §Established Decisions, [Links]).
- `hook statusline`'s arm order: canonicalise the instance directory, `check_instance`, then open the log. A refusal exits 0 at once with nothing read, opened or written. This is the fail-open template for `viola hook`'s nine events (per architecture §Standard Contracts, Hook contract).
- `run`'s version gate reads stamps through `read_stamps_strict` (strict check first), and a refusal degrades instead of aborting. This is the model for `update_stamps`' read (per architecture §Occupied Resources → Filesystem, `ledger/stamps.json`).
- `viola mcp` and `viola plugin install` run `strict::check_path` on the home (and `diagnostics/` or the `marketplace/` tree dirs) before any role file opens or any write (per architecture §Occupied Resources → Binary, subcommands and exit codes).
- `liveness::live_endpoint` returns `LiveEndpoint` from one snapshot read, and that read is the one place the check must come before. Where the check sits relative to it, inside `viola-state` or at each caller, is a placement decision for the plan (per architecture §Infrastructure Patterns → Crate dependency direction).

## Anti-patterns to avoid
- A strict-modes check made after a role file, log or `create_private_dir` call has opened or narrowed the home. It passes a 0770 home: measured on `revive` and the statusline arm, and fixed there (per architecture §Occupied Resources → Filesystem, `instances/<ViolaName>/`).
- A second writer of `snapshot.json` or `ledger/stamps.json`, or a call site with its own temp + rename beside `replace_private` / `replace_private_shared` (per architecture §Established Decisions, [Snapshot writer]).
- A plugin `command` (in `hooks.json` or `.mcp.json`) that names anything but the pinned copy's absolute path, such as a PATH lookup or the build-tree exe (per architecture §Established Decisions, [Deployment / Distribution]).

## Contract bindings
- architecture §Occupied Resources → Filesystem (the checked-reader list and the `check_instance(` call-site count) ↔ security-plan §Authentication & Authorization (the dated strict-modes exceptions for `hook.event`, stamps, ConPTY, `send`, `wait`/`last`, `answer`, `pause`/`release` and `mcp` frames, closed by this entry). The wrap amends both together.
- architecture §Conventions (CLI exit codes: exit 21 for a refused check; hook always 0) ↔ test-plan §5 CLI (`tests/cli_controls_not_disableable.rs`, its 0770 `--home` negative, exit 21) and obs-plan §10 exemption 7 (a refused check writes no line).
- architecture §Standard Contracts, Snapshot envelope (`read_snapshot_or_replay`) ↔ test-plan §6 E5 steps 1–3 / `verification-matrix.json#v1-41`, conditional on the founder's P4 fork.
- architecture §Infrastructure Patterns → CI/CD approach (the `windows-2025` leg of `ci.yml`'s matrix) ↔ test-plan §6 (the Windows `RUNNER_TEMP` negative case and the `seed_conpty` carve-out, CARRYs 6 and 13). That leg is this chunk's only Windows proof.

## Acceptance criteria contributions
- On every entry point the chunk widens to (`send`, `wait`, `last`, `answer`, `pause`, `release`, `viola hook`'s nine events, `viola mcp`'s frame tools, `verify`'s stamps read, `run`), a home at 0770 is refused before any role file or log opens, and the home reads 0770 afterwards, never narrowed to 0700. Each site is counted by the stated `check_instance(` / strict-call rule under `src/` (per architecture §Occupied Resources → Filesystem, `instances/<ViolaName>/`).
- On `windows-2025`, a case that widens an instance tree's DACL is refused by `check_instance` (per architecture §Occupied Resources → Filesystem, `instances/<ViolaName>/`).
- A pinned copy whose bytes no longer match its key refuses `run` with exit 1 `pinned-hash-mismatch` and is left byte-identical. The plugin files are rewritten at every start, and each exec-form `command` is the pinned copy's absolute forward-slash path (per architecture §Established Decisions, [Deployment / Distribution]).
- One product writer each, read back from source: `snapshot.json` written only by the wrapper, and `ledger/stamps.json` written only through `viola_state::stamps::update_stamps`, called only from `viola verify` (per architecture §Standard Contracts, Ledger stamps envelope / Instance snapshot).
