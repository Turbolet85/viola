# security extract

## Relevance
relevant. This chunk builds the ledger stamp, which is what switches on dialog answering (a code-execution gate). It also adds the only writer of an integrity-sensitive trusted file, a child spawn plus output parse, committed fixtures that fall under the NEVER-log floor, and the first `cli`-process catch site.

## Constraints
- **Sole writer of `ledger/stamps.json`.** Per security-plan §Data Protection (At rest → Code-bearing artefacts), §Bootstrap phases (`auth-scaffolding-baseline`, the `viola run` / `viola-state` item) and §Security Anti-Patterns → Universal:
  - `viola verify` is the only process that writes `ledger/stamps.json`.
  - It writes under the `.lock` sibling, at 0600, through `viola_state::fs::replace_private`, with the mode set on the temp file before any byte is written.
  - Test homes get no other writer: no helper, no fixture copy, no direct write in `stamped_home`.
- **Dialog decisions need a stamp.** Per §Security Anti-Patterns → Universal ("NEVER answer a `hook.dialog` with a non-`null` decision unless…") and §Threat Model Summary (Data classification → config; Accepted risks → the `unverified-cli` gate):
  - A non-`null` dialog decision requires a stamp for the child's measured CLI version.
  - An unlisted version, an unparseable one, or one without a stamp degrades to transport-only (decision withheld, `null`).
  - Whether any hook path returns a non-`null` decision today is research's question.
- **Strict-modes before trusting the stamps file.** Per §Authentication & Authorization (`~/.viola/` access control, Unix and Windows strict-modes lists) and §Input Validation (own state files on read):
  - `ledger/stamps.json` and `ledger/` are in the strict-modes set.
  - `run` may trust the stamps it reads only after the home strict-modes check has passed in the same process. On failure `run` exits 1.
  - The hook's dated interim exception (Decisions Log `2026-09-28`, hook.event) may not be borrowed by `run` or `verify`.
  - Whether a strict-modes check exists in `run`/`viola-state` today is research's question. If it does not, that is a P4 fork (land it here, or pin it to Epoch 6 with a ratified gap), not a silent skip.
- **The `--version` probe and the probe child.** Per §Input Validation (Child process output; Child executable resolution) and §Security Anti-Patterns → Data Protection / Code Patterns:
  - Output is read with `Read::take(MAX_FRAME)` and parsed tolerantly. A failure maps to unknown/unlisted.
  - The child is resolved through the existing resolver: on Windows PATHEXT only, and `claude.cmd` maps to the sibling `claude.exe`. It is never a `.cmd`/`.bat`, never spawned through sh/bash/cmd, and upstream output is never treated as a command.
  - Hook stdin that `verify` observes keeps the `MAX_FRAME + 1` cap.
- **Largest-hook-payload row.** Per §Input Validation (Constants) and the Decisions Log `2026-09-23` (Amendment 7; Phase 3.5 round 1), plus §Bootstrap phases ("Amendment 7 … before the `viola verify` chunk"):
  - The row records the byte size of each hook event kind and is stamped per CLI version.
  - A stamped maximum within a factor of 4 of `MAX_FRAME` is the revisit signal.
  - Amendment 7 must already be folded into arch §CLI Version Compatibility before implementation.
- **Recorded fixtures and stamps stay clean.** Per §Data Protection (Repository fixtures), §Security Anti-Patterns → Data Protection ("NEVER commit `fixtures/claude/*`…") / Secrets, §Secret Management (Storage: "never serialised into events, snapshots, diagnostics or the ledger") and §Bootstrap phases (`logging-redaction-wire`):
  - Probe prompts are synthetic.
  - Home paths and usernames are scrubbed before commit.
  - No GUI token, `?t=`, cookie, `CLAUDE_CODE_MESSAGING_*` or other R8-stripped `CLAUDE*` value reaches a fixture or `stamps.json`.
  - Whether `verify`'s own child spawn applies the R8 strip is P3/P4's question.
- **No bypass of the stamp gate.** Per §Security Anti-Patterns → Universal (config/env/flag clause; test-seam carve-outs) and §Input Validation (test-seam rows):
  - No `config.json` key, `VIOLA_*` var or CLI flag may force `cli_verified` or switch on dialog answering without a stamp.
  - Pointing `verify` at the fake agent must not add a new non-`VIOLA_*` env read to `viola`. A new seam needs its own Decisions Log entry and must be `cfg(feature = "fake-agent")` only.

## Patterns to follow
- `viola_state::fs::replace_private`, with its `.lock` sibling, for every `stamps.json` write, the same as `snapshot.json` and `ui/<port>.url` (§Authentication & Authorization, `~/.viola/` access control).
- Reuse the landed resolver `viola-agent-claude::resolve_program` (with root `src/run/mod.rs::resolve_program`) for the `claude` that `verify` and the `run` version gate spawn (§Input Validation, Child executable resolution).
- Error messages are fixed:
  - `AgentError` / `StateError` `Display` use fixed messages with no path or payload fields.
  - The `verify` catch site prints only `error: internal error`, through `src/human.rs`.
  - The full chain goes only to `instances/<name>/diagnostics/` when an instance resolves.
  - Per §Error Handling and §Bootstrap phases (`error-sanitization-wire`).
- Ledger rows and their values are closed and compiled in. Screen signatures, harness prefixes and local-command lists come only from compiled `viola-agent-claude` rows. Row ids and decisions are closed enums, not free strings (§Security Anti-Patterns → Code Patterns / Input).
- `stamps.json` is parsed as viola's own versioned format: tolerant, capped at `MAX_FRAME`, read after strict-modes (§Input Validation, own state files on read).

## Anti-patterns to avoid
- Any writer of `ledger/stamps.json` other than `viola verify`. This includes a test helper that "stamps the fake agent's version" directly, and `run` writing a stamp as a side effect (§Security Anti-Patterns → Universal).
- Any of these reaching a committed fixture: real prompts, unscrubbed home paths or usernames, R8-stripped `CLAUDE*` values (§Security Anti-Patterns → Data Protection / Secrets).
- Building a signature, prefix or local-command list from runtime text, recorded payloads or `stamps.json` values instead of compiled rows. Also: letting a flag, env var or config key switch on dialog answering without a stamp (§Security Anti-Patterns → Code Patterns / Universal).

## Contract bindings
- **security ↔ tests.** test-plan :220 records a conflict: arch lets a test home "stamp the fake agent's version", while this plan allows `viola verify` as the only writer. It resolves as `viola verify --home <home>` run against the fake agent. The `stamped_home` seam (`tests/support/home.rs`) and the fixture hygiene walk (`tests/support/hygiene.rs`) enforce §Data Protection's fixture review and the NEVER-log floor.
- **security ↔ obs.** CI's `viola-harness secret-scan` (obs-plan §9, §Secret Management → Secret scanning in CI) covers the test homes that fake-agent `verify` runs produce. The catch-site `error: internal error` line (obs-plan §7) is §Error Handling's stderr sanitization.
- **security ↔ arch.**
  - Amendment 7 becomes the ledger row in arch §CLI Version Compatibility.
  - `cli_verified` in `snapshot.json` stays wrapper-written only (§Security Anti-Patterns → Universal, snapshot writer).
  - The strict-modes check this chunk needs at `run`'s stamps read ties to the Epoch 6 entry "Home and code-bearing file integrity".
- **security ↔ CI.**
  - The real `claude` and live `verify` run only locally, and CI uses the fake agent (§Threat Model Summary → Infrastructure CI/CD).
  - `scripts/release-check.sh` must still judge `viola only`, with no `fake-agent`-featured artifact.
  - Any new crate passes `cargo deny check` (§Dependency Security).
- **security ↔ WSL (conditional CARRY 4).** This applies only if this chunk re-provisions the distro. In that case `--install-deps` must first narrow to an allowlisted `apt-get install` over the dry-run list, launched by the operator only (Decisions Log `2026-09-27` browser-verdict-reachability, Conditions; §Secret Management → Development).

## Acceptance criteria contributions
- **Sole writer.** A grep over `crates/`, `src/` and `tests/` finds no writer of `stamps.json` outside `viola verify`'s path through `replace_private`. After a fake-agent `viola verify --home <home>`, `ledger/stamps.json` exists at 0600 on Unix and a `.lock` sibling is used. (per security-plan §Data Protection, At rest → Code-bearing artefacts; §Security Anti-Patterns → Universal)
- **Transport-only degrade.** Each of these cases gets the same result: a home without a stamp, a child reporting an unlisted version, and a child reporting unparseable `--version` output. The snapshot shows `cli_verified:false`, and any dialog path yields a `null` decision (the human sees the dialog). No flag, env var or config key changes that outcome. (per security-plan §Security Anti-Patterns → Universal; §Input Validation, Child process output)
- **Clean fixtures and stamps.** Every committed `fixtures/claude/<ver>/*.json` passes the hygiene walk: no absolute home path, no username, no `CLAUDE_CODE_MESSAGING_*` or other identity-floor value, no `?t=`/cookie. `stamps.json` from a CI `verify` run holds none of these either, and CI `secret-scan` stays green. (per security-plan §Data Protection, Repository fixtures; §Bootstrap phases `logging-redaction-wire`)
- **Sanitized internal failure.** An induced internal failure in `viola verify` prints exactly `error: internal error` on stderr, with no path, no serde source and no upstream text, and exits non-zero. The `--version` read is capped with `Read::take(MAX_FRAME)`. (per security-plan §Error Handling; §Input Validation, Child process output)
