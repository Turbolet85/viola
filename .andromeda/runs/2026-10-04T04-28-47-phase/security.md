# security extract

## Relevance
relevant — the chunk adds a new dependency (vt100), a new parser surface over untrusted PTY bytes, compiled screen-signature ledger rows with `viola verify` stamps, a typed-input probe that spawns `claude` under a PTY, a possible new test seam, and a fuzz target in the exempted `fuzz/` workspace.

## Constraints
- The vt100 feed must run inside `std::panic::catch_unwind`. On a panic the gate degrades to `not-delivered` / `input-not-ready`, and byte passthrough to the human terminal must continue. Per security-plan §Input Validation (row "PTY output bytes (vt100)") and §Security Anti-Patterns → Code Patterns ("NEVER feed vt100 outside `catch_unwind`").
- Input-box and modal signatures, and any screen-matching literal, must come only from compiled `viola-agent-claude` ledger rows. None may be built from runtime or upstream text (screen content, prompts, `last_assistant_message`). The screen model belongs in `viola-agent-claude`, and the feed belongs in the `src/run/` pump. Per §Security Anti-Patterns → Code Patterns (screen-signature ban) and §Input Validation (PTY output row, stack reference). R1 also applies: upstream text is content, never a command or config (§Code Patterns, first ban).
- Stamps for the new rows go through `viola verify`'s one writer, under its `.lock`, at 0600. No other process may write `ledger/stamps.json`. Per §Security Anti-Patterns → Universal (stamps writer ban) and §Data Protection (code-bearing artefacts, `ledger/stamps.json`).
  - Two readers of `ledger/stamps.json` are allowed to skip strict-modes as a ratified interim gap: `run`'s version gate and `verify`'s `update_stamps`. The gap covers those two only (§Authentication & Authorization, `~/.viola/` access control; §Input Validation, own-state-files row).
  - If the readiness gate opened a separate stamps read to decide verified/unverified, that would be a new unchecked reader, which is a widening. Whether the gate can take its verdict input from the version gate's existing read is research's question.
- vt100 0.16.2 and every crate it pulls in must pass the root `cargo deny check`:
  - licence `allow` exactly MIT / Apache-2.0 / Zlib / Unicode-3.0;
  - no new `[[licenses.exceptions]]` or `ignore` without a Decisions Log entry;
  - crates.io sources only;
  - the C-building-crate ban;
  - the sole-root tokio ban if it enters a sync crate's graph.

  The version must be pinned in `[workspace.dependencies]` and `Cargo.lock` must be committed. Per §Dependency Security (`deny.toml` additions, Pinning). Whether vt100's graph clears these is research's question.
- The chunk may need a new forcing seam for the `--vt100-panic-bytes` mode. The two seams that exist are `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`. Any other env var or seam outside `VIOLA_*` needs its own Decisions Log entry, must be `cfg(feature = "fake-agent")` only, must be absent from every release build, and must switch off no control. Under this chunk's autonomous mode that makes it a widening shown at P4 and held. Per §Security Anti-Patterns → Universal (no env/config/flag disabling a control; seam carve-outs) and §Input Validation (the two test-seam rows).
- The human always wins:
  - A gate verdict of `input-not-ready`, a parser panic, or a gate timeout is a refusal to automation only. It never blocks, delays or alters a human keystroke or the passthrough bytes.
  - Error output carries only the fixed codes (`not-delivered` / `input-not-ready`), with no screen text and no path.

  Per §Security Anti-Patterns → Universal ("NEVER let a security refusal block the human") and §Error Handling (External responses).
- Rules for the typed-input probe in `viola verify`:
  - Probe prompts must be synthetic.
  - Any `--record` output passes verify's path and username scrub, which refuses the whole recording if a path or username survives.
  - Transient captures stay 0600 under `ledger/probes/<pid>/` (0700) and are removed on every exit path.
  - Inherited `CLAUDE*` credentials must never be serialised into a capture, fixture, diagnostic or ledger entry.

  Per §Data Protection (Probe captures; Repository fixtures) and §Secret Management (Storage, R8). Whether the probe's PTY child gets the R8 strip, as `run`'s child does, is research's question.

## Patterns to follow
- One shared closed refusal vocabulary. Gate outcomes map to the existing `not-delivered` refusal with a closed detail. They are never a free `String`, per §Security Anti-Patterns → Input (closed-enum ban) and §Error Handling (Error format, Channel).
- Fail-open containment as the existing hook panic seam does it: a fixed ASCII payload, no upstream text, and passthrough unaffected. Per §Input Validation (row "Test seam `FAKE_AGENT_HOOK_PANIC`"). It is the template for any vt100 forcing seam.
- The `fuzz/` exemption pattern:
  - a new `fuzz/fuzz_targets/` target is a test-only member of the separate `viola-fuzz` workspace, its deps pinned exactly in `fuzz/Cargo.toml`;
  - `fuzz/Cargo.lock` is audited by `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`;
  - the corpus is synthetic only, because the nightly `fuzz/artifacts/` upload is unscanned.

  Per §Dependency Security (`fuzz/` paragraph) and §Bootstrap phases (`secret-scanning-ci-gate`, fuzz upload).
- Parser-surface coverage. The vt100 feed under `catch_unwind` is a named fuzz and property surface, per §Input Validation ("Parser surfaces").
- Diagnostics vocabulary on panic is codes only (`parse-rejected{parser:"vt100-feed"}`). Screen content never reaches a log, diagnostic or event, per §Bootstrap phases (`logging-redaction-wire`, scope bullet).

## Anti-patterns to avoid
- Feeding vt100 outside `catch_unwind`, or letting a gate failure stop or alter the human terminal path (§Security Anti-Patterns → Code Patterns; → Universal).
- Deriving a signature, quiet-period value or modal set from runtime or upstream text, or writing `ledger/stamps.json` from anything but `viola verify` (§Security Anti-Patterns → Code Patterns; → Universal).
- Adding a test seam or env var, a `deny.toml` ignore, skip or exception, or a G2 exemption to reach green without a Decisions Log entry. Also linking `viola-fuzz` into `viola`, or adding `fuzz` to the root `[workspace]` (§Security Anti-Patterns → Universal; §Dependency Security; §Dependency Security CI integration, G2 bullet).
  - CARRY 4 adds one more: the harness must not gain any wipe of the Linux NOCOW `TMPDIR`, which is the operator's and is never wiped by the harness (§Security Anti-Patterns → Code Patterns, `run --mutants` scratch ban).

## Contract bindings
- security §Input Validation (PTY output row) ↔ obs `run.readiness_gate` span and `parse-rejected{parser:"vt100-feed"}`. Fields are codes only, per §Bootstrap phases `logging-redaction-wire`. The obs extract owns the field set.
- security §Input Validation (Parser surfaces) ↔ test-plan §6 Property suite and the fuzz pipeline: a `cases: 512` proptest and a `fuzz/fuzz_targets/` target with a synthetic seeded corpus.
- security §Dependency Security ↔ the CI `supply-chain` job:
  - `cargo deny check` and `scripts/deny-probes.sh` cover the vt100 addition;
  - the fuzz lockfile audit step covers any new fuzz dep.
- security §Secret Management (artifact canary scan) ↔ obs-plan §9 `viola-harness secret-scan`. Its content canary keeps screen text and prompts out of role files. Any new probe or fake-agent output lands inside the scanned capture.
- security §Input Validation (test-seam rows) and §Dependency Security (CI integration, `release-check.sh`) ↔ the tests harness. Any `--vt100-panic-bytes` seam stays behind `fake-agent`, which `release-check.sh` refuses in a release build.

## Acceptance criteria contributions
- With vt100 added, `cargo deny check` passes, `bash scripts/deny-probes.sh` still proves every ban fires, and `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` passes. The diff adds no `allow`, `skip`, `ignore`, `[[licenses.exceptions]]` or zizmor silence (per security-plan §Dependency Security).
- A grep shows every vt100 parser call site inside `std::panic::catch_unwind`. A forced-panic test reads gate outcome `input-not-ready` (refusal `not-delivered`) while the bytes passed through to the host terminal are unchanged (per security-plan §Security Anti-Patterns → Code Patterns; §Input Validation, PTY output row).
- The input-box and modal signatures, quiet period and maximum wait are compiled constants or ledger rows in `viola-agent-claude`, with no path from screen text or upstream text into them. A grep shows `ledger/stamps.json` written only through `viola verify`'s writer, with no new unchecked stamps reader beyond the two ratified interim reads (per security-plan §Security Anti-Patterns → Code Patterns / → Universal; §Authentication & Authorization, `~/.viola/` access control).
- A grep of product crates finds no new `std::env::var*` read outside `VIOLA_*` and the two existing seams. Any new seam has a Decisions Log entry, is `cfg(feature = "fake-agent")`-gated, and `scripts/release-check.sh --probe` still reads 5/5 refused with a clean control (per security-plan §Security Anti-Patterns → Universal; §Dependency Security, CI integration).
