# security extract

## Relevance
partial. Items 1-7 (diagnostic sinks, detail files, panic-payload routing, the `config.json` read, the harness `logs` merge) fall under the NEVER-log floor and the file-permission rules. Item 8 (the `.claude/settings.json` agent write guard) is dev tooling that security-plan does not cover, so this plan sets no constraint on it.

## Constraints
- security-plan §Bootstrap phases → `logging-redaction-wire` (the consolidated NEVER-log floor) requires the following:
  - User content, tool `input`, `statusline_command` and serde_path_to_error drift reports go only to `instances/<name>/diagnostics/`.
  - None of that may reach stderr log lines or any home-level output.
  - This is the security basis for a payload-free home-level panic line and for sending the panic payload and backtrace only to `detail-run.ndjson`. The same applies to every future line in the home-level `<home>/diagnostics/<role>.ndjson` sinks: operational metadata only (pids, `agent_session_id`, `started_at` are allowed).
- security-plan §Bootstrap `logging-redaction-wire`, §Secret Management ("What counts as secret") and §Anti-Patterns → Secrets require that no sink ever writes the following, including `process-start`'s service-identity fields and any env/context capture:
  - `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value
  - the GUI token, the launch URL or the `.url` contents
  - the `Cookie` header
  - GUI request query strings
- security-plan §Authentication & Authorization (`~/.viola/` access control row), §Data Protection → At rest (Logs) and §Anti-Patterns → Data Protection / Logging require the following on Unix:
  - Every directory is created 0700, and every file is created with `OpenOptionsExt::mode(0o600)`. That covers both `<home>/diagnostics/` role files and `instances/<name>/diagnostics/detail-<process>.ndjson`.
  - The mode is never left to the umask.
  - `diagnostics/` files are never readable by other users.
  - On Windows, `instances/<name>/diagnostics/` is covered by the strict-modes read rule; whether this chunk's lazy dir creation relies on the inherited home DACL is research's question.
  - Whether the existing walking-skeleton home creation already sets 0700/0600 is also research's question.
- security-plan §Anti-Patterns → Input requires that any `instances/<name>` join use a string that came out of `ViolaName::try_new`. This covers:
  - the detail-file path built from the `OnceLock<ProcessCtx>` instance
  - the file names of the `hook-<name>`/`run-<name>`/`cli-<name>` role sinks
  - the harness `logs` enumeration of instance dirs and its `--instance` filter
- security-plan §Anti-Patterns → Universal (config/env cannot switch off controls) and §Input Validation (Configuration values row) require the following:
  - The `diagnostics_level` read is a tolerant parse of `config.json` with `v` checked.
  - The level is taken only from `config.json`, never from `RUST_LOG`, `VIOLA_*` or env.
  - It must never widen the field allow-list, and the NEVER-log floor is a control that config must not disable.
- security-plan §Dependency Security (Pinning) and §Anti-Patterns → Logging require tracing-subscriber `>=0.3.20` in `Cargo.lock` (RUSTSEC-2025-0055, ANSI escape injection). They also require that `cargo deny check` keeps passing with obs's `veil` / `tracing-subscriber` feature bans in `deny.toml`. Whether the current lock already resolves `>=0.3.20` is research's question.
- security-plan §Error Handling (Internal logging) and §Anti-Patterns → Logging require that `viola hook` never writes to stderr and never exits non-zero. The sink open path that this chunk builds for the `hook-<name>` role must therefore fail silently: on any open or permission failure it makes no stderr write and never panics to stderr, even though no hook producer exists yet.

## Patterns to follow
- Explicit-mode creation (per security-plan §Authentication & Authorization, `~/.viola/` access control row): pass `OpenOptionsExt::mode(0o600)` with append for files, create dirs 0700, and discard the write if the mode cannot be set, rather than falling back.
- Newtype-gated paths (per security-plan §Input Validation, Tailed paths row): parse through `ViolaName::try_new`, ignore symlinks via `symlink_metadata`, and skip names that fail to parse. The harness merge's walk over `instances/*/diagnostics/` should follow this.
- Bounded line reads (per security-plan §Input Validation, Own state files on read row, and the §Constants `MAX_FRAME` entry): read each ndjson line with a `MAX_FRAME` cap and count an over-long line as torn. This matches the merge's `{"torn":true,"offset":n}` shape.
- Escaping control characters (per security-plan §Bootstrap `logging-redaction-wire` and the RUSTSEC-2025-0055 class): keep `with_ansi(false)` on the JSON layer, and escape C0/C1 controls (keeping `\n`/`\t`) in any human-mode terminal rendering of log content.

## Anti-patterns to avoid
- NEVER create `diagnostics/` files readable by other users, and NEVER create any file under the home with the default umask (per security-plan §Anti-Patterns → Logging / Data Protection).
- NEVER write drift reports, anyhow chains carrying serde sources, or panic payloads that quote upstream text to stderr or to a home-level line. They go only to `instances/<name>/diagnostics/` (per security-plan §Anti-Patterns → Logging; §Bootstrap `error-sanitization-wire`).
- NEVER use tracing-subscriber `<0.3.20`, and NEVER let an env var or `config.json` key change what the floor lets through (per security-plan §Anti-Patterns → Logging / Universal).

## Contract bindings
- obs ↔ security: obs-plan §3 (field allow-list, file location) must satisfy security-plan §Bootstrap `logging-redaction-wire`. Security owns the floor and the 0600/0700 contract, and obs owns the format and vocabulary. `diagnostics_level` changes volume only (scope item 4) and is the obs side of the §Anti-Patterns → Universal constraint.
- tests ↔ security: the harness `logs` merge and the `diag-line.v1.json` schema check run in CI. Any fixtures or golden files they add must hold synthetic content only, with no home paths or usernames (security-plan §Data Protection → Repository fixtures). `cargo deny check` stays the CI supply-chain gate (security-plan §Dependency Security, CI integration).

## Acceptance criteria contributions
- On Unix, a test asserts that `<home>/diagnostics/` and `instances/<name>/diagnostics/` have mode 0700, and that the role file and `detail-run.ndjson` have mode 0600, independent of the process umask (per security-plan §Authentication & Authorization, `~/.viola/` access control row).
- A forced panic with an instance resolved leaves the payload and backtrace only in `instances/<name>/diagnostics/detail-run.ndjson`. The home-level `run-<name>` line contains no payload text, and stderr carries none of it (per security-plan §Bootstrap `logging-redaction-wire`).
- `cargo deny check` passes, and `Cargo.lock` resolves tracing-subscriber `>=0.3.20` (per security-plan §Dependency Security, Pinning).
- Setting `RUST_LOG=trace` (or any `VIOLA_*` var) does not change the emitted level or fields. Only the `config.json` key `diagnostics_level` does, and `"debug"` adds no field outside the allow-list (per security-plan §Anti-Patterns → Universal).

## Relevant amendment history
- 2026-09-24-three-os-ci-headless-harness-skeleton, rejected Decisions-Log entries for an interim `--home` fix and the R8 strip. The walking skeleton does not canonicalise or strict-modes-check `--home`, and on Windows it does not set the protected DACL. That gap belongs to the route entries "Home and code-bearing file integrity" and "CLI machine contract — global --home" (CARRY pins), not to this chunk. This chunk creates `diagnostics/` files under that home without owning the strict-modes check. The R8 `CLAUDE*` strip is also not applied yet, so any env or context capture in `process-start` must stay away from those values.
- 2026-09-24-supply-chain-and-workflow-gates, `deny.toml` additions. The arch-bans bullet now records that `scripts/deny-probes.sh` proves every ban live, and that `deny.toml` carries obs's `veil` / `tracing-subscriber` feature bans. If this chunk adds or changes tracing-subscriber features, it must satisfy those bans and the probes.
