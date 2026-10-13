# security extract

## Relevance
partial — no new trust boundary, input or listener; the chunk touches security-bearing code (the Windows strict-modes check behind §2's red, `replace_private`, the board's check, the statusline argv, the mutation scratch guard, send's validation order, human-mode escaping) and the supply-chain gates (§4's cross-target clippy, a possible `windows-mutants.yml` edit), where the work must leave each control exactly as strong as it is.

## Constraints
- §2's Windows baseline red is closed by removing its cause, never by loosening the check. security-plan §Authentication & Authorization (`~/.viola/` access control row, "Windows strict-modes check") requires the instance check to refuse an unreadable descriptor, a NULL DACL, a volume without persistent ACLs, an owner other than the user SID or Administrators, and any allow ACE granting write, delete, `WRITE_DAC` or `WRITE_OWNER` to another SID, with inherit-only ACEs and generic bits counted. Whether the cargo-mutants scratch dir's owner or DACL on `windows-2025` is what `trusted(&instance)` refuses is research's question, to be closed against run 38215104120's log. If that hypothesis holds, the fix belongs in the test's or harness's lay-down of the tree (for example an explicit protected user + SYSTEM DACL), never in `check_instance`.
- §3's split of `src/run/send.rs` must keep the order security-plan §Input Validation (Paste text row) requires: `validate_paste_text` runs over decoded `char`s of the text as received and rejects rather than strips; `typed_text` is taken once after it passes and is the only text used afterwards; and `control-character` then `empty-text` are refused before the first wheel read. Whether the current code already follows this order at each phase boundary is research's question. A split must leave it the same.
- §3's split of `src/human.rs` must keep human-mode `wait`/`last` escaping C0/C1 (keeping `\n` and `\t`), and `list` rows going through `escape_row`, which escapes every control as `\xHH` so each row stays one line, per security-plan §Bootstrap phases (`logging-redaction-wire`). Escaping must stay on every writer path after the move.
- Any test or restatement for the `viola-state` survivors keeps two rules from security-plan §Authentication & Authorization (`~/.viola/` access control row):
  - `replace_private` (`fs.rs:281/285/286`) is the one helper for replaced files, and it sets the temp file's mode (0600, or 0700 for the pinned `viola`) before any byte is written.
  - `board::scan` / `check_if_present` (`board.rs:45/67`) let an absent home or `instances/` pass but refuse an unstattable one. A `NotFound`-guard survivor is disposed only by a case that proves a non-`NotFound` error is refused.
- The `shell_argv` survivors (`statusline.rs:110`) are disposed only in a way that keeps the shape security-plan §Input Validation (Statusline shell-out row) requires: exactly `/bin/sh -c <command>` with the command as one argument on Unix, `None` for every input on Windows, and still the only shell-out.
- The `scratch.rs` `prepare` survivors (`:48`, `:54`) and any `windows-mutants.yml` change sit under the scratch-wipe ban in security-plan §Security Anti-Patterns › Code Patterns (`run --mutants` row). The wipe:
  - runs only behind the pure guard (final component exactly `viola-mutants-scratch`, the repository neither that dir nor inside it, a root refused `scratch-refused`, a failed removal `scratch-wipe-failed`);
  - never falls back to `%TEMP%` or the repository;
  - never prints the path;
  - runs only when `HOST_SCRATCH = cfg!(windows)`.
- §4's cross-target clippy gate comes under security-plan §Dependency Security (Pinning; `deny.toml` additions):
  - it uses the `rust-toolchain.toml` 1.98.1 toolchain through rustup;
  - it adds no C-building crate and no Windows SDK dependency to the graph (the arch C-crate ban in `deny.toml` stays);
  - `pre-push`'s `tools` stage checks the host against CI's pins and installs nothing, so the `x86_64-pc-windows-msvc` std is host preparation, never a harness install;
  - no gate launches anything as uid 0.

## Patterns to follow
- Every ban gets a probe that proves it fires: `scripts/deny-probes.sh`, G2's `--probe`, `release-check.sh --probe`, `conpty-vendor.sh --probe`, all read red before the check runs (security-plan §Dependency Security, CI integration). A new gate entry follows the same shape.
- The strict-modes check is proven by its own cases (`cfg(unix)` today, with a Windows case owed to "Home and code-bearing file integrity"), per security-plan §Authentication & Authorization (`~/.viola/` access control row). A Windows reproduction of §2 on the ordinary `windows-2025` CI leg lays down its tree with a DACL the check accepts.
- One home per control: `viola_state::fs::replace_private` for every replaced file, and `viola_agent_claude::statusline::shell_argv` for the one shell-out (security-plan §Input Validation, Statusline shell-out row). The split moves call sites but never duplicates them.
- External errors use fixed messages and closed codes with no path (security-plan §Error Handling). Any refusal a new test asserts is matched on its code, never on a path or payload.

## Anti-patterns to avoid
- Never make `#[cfg(windows)]` strict-modes return `Ok(())` unchecked. Never pass a NULL DACL, a volume without persistent ACLs, or an inherit-only `GENERIC_*` grant just to turn the baseline green (security-plan §Security Anti-Patterns › Data Protection).
- Never add an env var, flag or `config.json` key that switches off the home strict-modes check, for example a "mutants scratch" bypass. Any new test seam needs its own Decisions Log entry (security-plan §Security Anti-Patterns › Universal).
- Never reference an Action by a mutable ref, widen workflow permissions, add `inputs:`, a secret, a cache or an upload to `windows-mutants.yml`, and never dispatch it inside this chunk (security-plan §Dependency Security, CI integration; §Security Anti-Patterns › Code Patterns).

## Contract bindings
- tests ↔ security: survivor disposal (a failing assertion under the mutant, a restatement, or an accepted-equivalent record the next boundary audit reads) is test-plan territory. The security plan only requires that a disposal of a guard survivor (`board.rs`, `fs.rs`, `scratch.rs`, `shell_argv`) preserves the control the guard enforces.
- arch ↔ security: §4's gate relies on the arch C-building-crate ban in `deny.toml` and the per-sync-crate `deny-sync.toml` tokio ban (three target triples) staying as they are (security-plan §Dependency Security, `deny.toml` additions).
- obs ↔ security: §3's `human.rs` split carries the `logging-redaction-wire` escaping floor (security-plan §Bootstrap phases), which obs-plan's human-output rules also read.

## Acceptance criteria contributions
- No change to the refusal conditions of `viola_state::strict::check_instance` / the Windows owner + DACL read in the chunk diff. §2's fix lands in test or harness lay-down code, with the cause named against run 38215104120's log (per security-plan §Authentication & Authorization).
- `cargo deny check` and `bash scripts/deny-probes.sh` pass, and `Cargo.lock` gains no crate from §4's gate (per security-plan §Dependency Security).
- If `.github/workflows/windows-mutants.yml` is edited, `zizmor .github/workflows/` passes, and the file keeps `permissions: {}`, job `contents: read`, SHA-pinned `uses:`, no `inputs:` and no upload (per security-plan §Dependency Security).
- After the §3 split, the existing `validate_paste_text` / `control-character` / `empty-text` ordering cases and the `wait`/`last`/`list` escaping cases pass unchanged (per security-plan §Input Validation and §Bootstrap phases).
