# security extract

## Relevance
partial — no new trust boundary, input surface, secret or dependency; the chunk restates and tests `viola verify`'s Run B driver (`src/cmd/verify/typed.rs`), whose PTY children and probe pastes rest on founder-ratified rulings that a restatement must leave unchanged, and any test-only seam it adds touches the release-build and env-var bans.

## Constraints
- A behaviour-preserving restatement of Run B leaves unchanged everything security-plan §Threat Model Summary (Child process spawning vector, the `viola verify` entry point) names: the four compiled-literal bracketed pastes and their order, a paste only into rows that hold the input-box literal and no modal literal, the wait up to `PROBE_DEADLINE` from that turn's Stop, the Ctrl-C → Ctrl-C after 500 ms → kill ending, and no byte typed into any dialog. These are founder live rulings, so any departure is a widening for P4, not a restatement.
- Per security-plan §Threat Model Summary (Child process spawning vector), each product PTY child stays a direct spawn (never through a shell) under the R8 strip at a fixed 80×24. A `Pty` double seam must not change how the product path spawns or strips. Whether `SharedPty` / `Run` already isolate the spawn from the driver logic is research's question.
- Per security-plan §Input Validation (Child process output row), verify's PTY output stays capped at `MAX_FRAME` into a `Screen` fed under `catch_unwind`, and it is read only against the compiled signature literals. A restated framing wait (`framing_turns`, `wait_pump`) keeps that cap and that catch on the product path.
- Per security-plan §Input Validation (the two Test seam rows), a seam is compiled out of every release build and configures nothing. Per security-plan §Secret Management (Storage), environment variables are not a configuration channel, and the only non-`VIOLA_*` variables are the two `FAKE_AGENT_*` seams. So a `Pty` double or a test hook lives behind `cfg(test)` (or the existing `fake-agent` feature) and reads no new environment variable. A new env seam would need a Decisions Log entry.
- Per security-plan §Dependency Security (CI integration, the per-OS `release` job), `scripts/release-check.sh` refuses any artifact built with a test-only feature. A seam reachable outside `cfg(test)` / `fake-agent` would put test code into the shipped `viola`.
- Per security-plan §Data Protection (Interactive probe dirs), Run B's `<cwd>/.viola-verify-<pid>/` is 0700 and removed on every exit path by a drop guard, and viola writes nothing under `~/.claude`. A test that builds a `Run` or reaches `finish` / `end_by_ctrl_c` must not leave a `.viola-verify-*` dir in the cwd or write under `~/.claude`.
- Per security-plan §Security Anti-Patterns → Data Protection, a test that starts a real non-`claude` PTY child spawns a real executable directly, never a `.cmd`/`.bat` and never through `sh`/`bash`/`cmd`. Per security-plan §Security Anti-Patterns → Code Patterns, `statusline_command` stays the only shell-out.

## Patterns to follow
- The landed test-seam form, per security-plan §Input Validation (Test seam rows): the seam is named only in its own file under a `cfg` gate, compiled out of release, and holds closed values. Apply it to any `Pty` double or framing-wait hook.
- Per security-plan §Security Anti-Patterns → Code Patterns, a test-side removal takes only a dir the test itself created (its own `TempDir` drop). This applies to any scratch cwd a `Run`-building test uses.
- Per security-plan §Input Validation (Constants), `Read::take(MAX_FRAME)` applies on every child-output reader. A test double that hands bytes to the driver goes through the same bounded path, not around it.

## Anti-patterns to avoid
- Per security-plan §Security Anti-Patterns → Code Patterns, never feed vt100 outside `catch_unwind`, and never build screen signatures from runtime text. A test double supplies screen bytes, never the signature literals the product matches.
- Per security-plan §Security Anti-Patterns → Data Protection, never spawn a `.cmd`/`.bat` PTY child through portable-pty.
- Per security-plan §Security Anti-Patterns → Universal, no `config.json` key, `VIOLA_*` variable or flag (a test-seam variable included) may switch off a control.

## Contract bindings
- security §Dependency Security (the `release` job's `release-check.sh` refusal of test-only features) ↔ test-plan §10 / CI `release` job: the gate that proves a `cfg(test)` seam stays out of the shipped binary.
- security §Threat Model Summary (verify's ratified Run B behaviour) ↔ test-plan §10 Mutation gate: the survivors' disposal records must not claim a restatement that changed a ratified probe behaviour.

## Acceptance criteria contributions
- Any seam the chunk adds sits under `#[cfg(test)]` or the existing `fake-agent` feature. The product source reads no new environment variable, and `scripts/release-check.sh` still passes on the release build (per security-plan §Input Validation, Test seam rows, and §Dependency Security).
- Outside `typed.rs`'s `mod tests`, the diff changes none of Run B's compiled paste literals, `PROBE_DEADLINE`, the paste order, the input-box/modal guard or the Ctrl-C → 500 ms → kill ending (per security-plan §Threat Model Summary, Child process spawning vector).
- Any test PTY child is a direct spawn of a real non-`claude` executable: no shell, no `.cmd`/`.bat` (per security-plan §Security Anti-Patterns → Data Protection).
- After the chunk's tests run, no `.viola-verify-*` dir is left in the cwd and nothing new is written under `~/.claude` (per security-plan §Data Protection, Interactive probe dirs).
