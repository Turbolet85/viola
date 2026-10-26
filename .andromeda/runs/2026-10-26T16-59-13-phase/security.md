# security extract

## Relevance
relevant — this chunk is the route entry the plan names as the owner of every strict-modes dated gap and of the home's code-bearing-file integrity.

## Constraints
- security-plan §Authentication & Authorization (`~/.viola/` access control; the detail's **Strict-modes check**) requires the home strict-modes check in every process that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`, made before the value is used, with no exception left once this entry lands. That means CLI `send`, `wait`, `last`, `answer`, `pause` and `release`, every `viola hook <event>`, `viola mcp`'s frame tools (`live_endpoint`), `viola verify`'s `update_stamps`, and on Windows x64 `run`'s `conpty/` folder and both companions. Every dated interim gap (the first through the eighth) names this entry as the one that closes it. The failure shapes are fixed: `run` and `mcp` exit 1, every `hook` event prints nothing and exits 0, and CLI client verbs report `instance-unreachable` (exit 21).
- security-plan §Authentication & Authorization (the detail's **Strict-modes check**) requires the check to run before any role-file or log open. Opening a log or role file narrows an existing home to 0700, so a check made after it passes a home at 0770. Whether `client::start` and `run` already order the check first is research's question.
- security-plan §Authentication & Authorization (Unix rule) requires a refusal when any of these is group- or world-writable or not owned by euid: the home, `instances/<name>/`, `bin/`, `plugin/`, `ledger/`, the `bin/<version>-<hash>/` and `plugin/<version>-<hash>/` folders in use (`hooks/` and `.claude-plugin/` included), or a trusted file (`snapshot.json`, `ledger/stamps.json`, the pinned `viola`). Its read rule also refuses `mode & 0o077 != 0` on the home and on `instances/<name>/`.
- security-plan §Authentication & Authorization (Windows strict-modes check) requires the owner and DACL check at the same entry points over the same list, each object checked on its own. It refuses an unreadable descriptor, a NULL DACL, a volume without `FILE_PERSISTENT_ACLS`, an owner other than the user SID or Administrators, and any allow ACE that grants write, delete, `WRITE_DAC` or `WRITE_OWNER` to another SID. Inherit-only ACEs and generic bits count. The read rule covers the home, `instances/<name>/`, `instances/<name>/diagnostics/` and each existing `events.ndjson` before it is opened. A `--home` outside `%USERPROFILE%` gets a protected user + SYSTEM DACL at creation, or is refused.
- security-plan §Data Protection (Files `~/.viola/`) requires 0700 directories and 0600 files to be set explicitly, never left to the umask. The pinned `bin/<version>-<hash>/viola` at 0700 is the one exception. Every replaced file goes through `replace_private` / `replace_private_shared`, with the mode set before any byte is written.
- security-plan §Data Protection (Code-bearing artefacts) requires these:
  - `<hash>` is the first 16 hex characters of a SHA-256 of the exe bytes (`sha2 =0.11.0`).
  - An existing `bin/<version>-<hash>/viola(.exe)` is re-hashed before reuse. A mismatch refuses to start (exit 1).
  - `plugin/<version>-<hash>/` and `instances/<name>/settings.json` are rewritten at every start.
  - The ConPTY companions are re-hashed in full through a `FILE_SHARE_READ`-only handle held until the spawn.

  Two questions are research's: whether the pinned-exe write tolerates a concurrent same-version first start (CARRY 16), and whether two builds of one tree can produce different `<hash>` values (CARRY 11).
- security-plan §Security Anti-Patterns › Universal requires that only the instance's own wrapper writes `instances/<name>/snapshot.json` and only `viola verify` writes `ledger/stamps.json`. It also requires that every exec-form `command` (`hooks.json`, `.mcp.json`, the `settings.json` statusline, `marketplace/viola/.mcp.json`) is an absolute path, never a PATH lookup.

## Patterns to follow
- `viola link` / `viola unlink` run `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so the check comes before any role file, log or snapshot value. A refusal exits 21, writes no line and leaves the mode as found. This is the order the plan cites for a verb with no dated gap (per security-plan §Authentication & Authorization).
- `run`'s version gate reads stamps through `viola_state::stamps::read_stamps_strict`, which runs `check_stamps` first. A refusal there is `cli_verified:false` with `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}`. This is the model for closing `update_stamps`' gap (per security-plan §Authentication & Authorization).
- `viola hook statusline` runs `check_instance` on the canonicalised instance directory before obs init or any open under the home. A refused instance returns exit 0 and leaves no trace: no `config.json` read, no line, no stdin read, nothing printed. This is the fail-open template for the other `hook` events (per security-plan §Authentication & Authorization; §Error Handling).
- Use `viola_state::fs::create_private_dir` for home creation, including the Windows protected DACL outside `%USERPROFILE%`. Use `replace_private` / `replace_private_shared` for every rewritten code-bearing file (per security-plan §Data Protection).

## Anti-patterns to avoid
- NEVER write a channel frame before the server is verified against a `snapshot.json` that passed the strict-modes check in the same process. Once this entry lands, no `hook.event`, `hook.dialog`, CLI frame or `viola mcp` tool frame may still borrow a dated gap (per security-plan §Security Anti-Patterns › Authentication).
- NEVER let `#[cfg(windows)]` return `Ok(())` unchecked. NEVER pass a NULL DACL, a non-persistent-ACL volume or an inherit-only `GENERIC_*` grant just because no specific-rights ACE matched. NEVER reuse `bin/` or `plugin/` contents without the re-hash or the rewrite (per security-plan §Security Anti-Patterns › Data Protection).
- NEVER let `config.json`, a `VIOLA_*` variable or a CLI flag switch off the strict-modes check. `VIOLA_DIR` is influenced by the child, so a home reached through it is checked like any other. NEVER let a refusal block the human (per security-plan §Security Anti-Patterns › Universal).

## Contract bindings
- security ↔ tests:
  - test-plan §6: the Windows `RUNNER_TEMP` negative case (CARRY 2).
  - test-plan §5: the `seed_conpty` carve-out (CARRYs 6 and 13). A home the test seeds first would skip viola's creation DACL, and whether the product must refuse such a home is owed here.
  - test-plan §5 CLI: `tests/cli_controls_not_disableable.rs`, its 0770 `--home` negative with exit 21 (CARRY 4).
  - `tests/contract_content_hash.rs` pins the SHA-256 truncation.
  - The Windows proof is CI's `windows-2025` leg.
- security ↔ obs:
  - A refused strict-modes check opens no log, writes no line and leaves the home's mode as found (obs-plan §10 exemption 7, per scope).
  - obs-plan §9's `secret-scan` fails on a non-`0600` diagnostics file on Unix (per security-plan §Secret Management).
- security ↔ arch:
  - The snapshot replay (Standard Contracts: Snapshot envelope) has exactly two product readers today, `revive`'s preflight and `viola list`, each reading after the strict-modes check (per security-plan §Threat Model Summary, filesystem vector).
  - Moving more readers onto `read_snapshot_or_replay` (CARRY 19) changes that boundary and is the founder's fork at P4.
- security ↔ Decisions Log: closing each dated gap retires its exception text in §Authentication & Authorization and in §Security Anti-Patterns › Authentication. Any new reader or refusal basis met on the way is a boundary widening, and it halts until the founder gives his live word.

## Acceptance criteria contributions
- Each of these meets a 0770 home and a group-writable or foreign-owned `instances/<name>/`:
  - Every CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) exits 21 `instance-unreachable` with no frame written.
  - `viola hook <event>` exits 0 with no body.
  - `viola mcp`'s frame tools return `instance-unreachable` with `detail: null`.
  - `viola verify`'s `update_stamps` refuses.

  In every case the home's mode is read back unchanged afterwards (per security-plan §Authentication & Authorization).
- On `windows-2025`, a case that widens an instance tree's DACL is refused by the Windows strict-modes check. One example adds a `Users` write ACE; another adds an inherit-only `(OI)(CI)(IO)` `GENERIC_ALL` read ACE (per security-plan §Authentication & Authorization).
- The code-bearing files hold up against tampering (per security-plan §Data Protection):
  - A `bin/<version>-<hash>/viola` with altered bytes makes `viola run` refuse to start (exit 1).
  - A `plugin/<version>-<hash>/` file with altered bytes holds the rendered bytes again after the next start.
- A source read finds every `read_snapshot` / `live_endpoint` / stamps read under `src/cmd/`, `src/run/` and `crates/viola-mcp/` preceded by a strict-modes check in the same process. No dated-gap exception remains in `.claude/rules/security.md` (per security-plan §Security Anti-Patterns › Authentication).
