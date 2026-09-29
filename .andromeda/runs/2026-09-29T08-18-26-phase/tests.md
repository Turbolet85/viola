# tests extract

## Relevance
relevant — the chunk changes the Windows Module ↔ PTY boundary, adds pinned-bin-dir contents with hash/signature negatives, and its acceptance is a CI measurement on the windows-2025 leg.

## Constraints
- OS-branch code is verified only on its own runner: the sideload, load-path control, signature check and fallback are `#[cfg(windows)]` behaviour proven on the `windows-2025` leg (the ConPTY leg), while the macOS/Linux legs must stay green and unchanged; a host-only or Linux-only green is no proof (per test-plan §9 Matrix builds; §11 Test Strategy; §1 multi-os-compat trigger).
- The Module ↔ PTY boundary is tested with a real ConPTY spawn of `viola-fake-agent` through the `viola-pty` seam; the existing resize (incl. the spawn→pump window), no-EOF exit-on-handle and `.cmd`-refusal tests must hold under whichever backend `viola run` selects, and the H2 limit recorded in that row is the fact this chunk's measurement updates (per test-plan §5 Module ↔ PTY).
- Zero-flakiness: nextest `retries = 0`, no `#[ignore]` parking, a flake keeps the chunk red until its root cause is fixed — so an H2 key-after-resize loop that can lose keys cannot land as a gating test; it runs measurement-only, and the gating red test keeps its "key only after the child reports the new size" discipline (per test-plan §10 Zero-flakiness budget; §5 Module ↔ PTY).
- Coverage is gated per OS job at lines 85 / functions 95 / regions 80, so new Windows-only code (hash, signature, pre-load, fallback, backend recording) must be covered on the windows leg; the `--ignore-filename-regex` may not be widened to reach green (per test-plan §10 Coverage thresholds / Stack adjustments; §11 CI).
- On-disk state is never mocked: the pinned bin dir and its two new files are exercised in a real tempfile home under `target/e2e-home/`, produced by the real `viola run` pin path, never hand-written; faults are real file operations (per test-plan §8 What to mock / Filesystem; §7 Seed strategies; §5 Setup / teardown lifecycle).
- The pinned-code negatives parallel the pinned exe's: a tampered pinned file is a required filesystem-vector negative; for the sideload the expected verdict is the scope's degrade to inbox ConPTY with the backend recorded, not an exit — whether the pin writer already covers extra files is research's question (per test-plan §1 Vector 7 trigger; §6 Path 1 step 5).
- Tests make no real network calls: no test or harness step may download the NuGet package; the pinned bytes must already sit beside the test binaries (CI Windows legs and the `pre-push` host `windows-tests` stage) before the suites run (per test-plan §11 Universal; §3 `pre-push`; §9 E2E row).

## Patterns to follow
- A pinned official download verified by sha256 against pins that live once in ci.yml's workflow `env:` and are parsed from the file text — `scripts/install-node.sh` / `NODE_PIN_*` — so each tool has exactly one version source (per test-plan §9 tool-install paragraph).
- The pinned-exe tamper negative: overwrite one byte of the pinned file, rerun `viola run`, assert the product's verdict — the template for sideload hash-mismatch, absent and signature-rejected cases (per test-plan §6 Path 1 steps 5-6).
- Hash pins asserted against literal oracles, never the product's own constant (`tests/contract_content_hash.rs` style) (per test-plan §6 Contract suite; §11 Unit).
- The H2 precedent: a measurement-only CI loop added, read, then removed so ci.yml returns byte-identical, with the run id and loss count recorded as evidence (per test-plan §5 Module ↔ PTY, ci#36527891850 / ci#36529038462).
- Windows-only cases as `#[cfg(windows)]` tests on the windows-2025 `test` leg (the `.cmd` refusal, `tests/channel_sqos_open.rs`) (per test-plan §5 Module ↔ PTY; §6 Security control negatives).

## Anti-patterns to avoid
- Gating the H2 loop's loss rate with retries, a retry-once budget or `#[ignore]` (per test-plan §11 Quality; §11 CI; §10 Zero-flakiness budget).
- Doubling the Windows loader or WinVerifyTrust at runtime without a seam trait viola defines — fault the check with real files (a tampered byte, an unsigned or foreign-signed file), not a monkey-patch (per test-plan §11 Mocking; §8 Anti-monkey-patching).
- Asserting the backend by parsing the rendered child screen; verdicts come from events, diagnostics, the fake-agent receipt and exit codes (per test-plan §11 E2E).

## Contract bindings
- tests ↔ obs: the backend recorded on the degrade (the `pty.spawn` `pty_backend` field or its successor) is the tests' oracle for "which ConPTY ran"; G2 zero-panics and the secret scan still gate the Windows homes (per test-plan §3 Log format; §9 E2E row / Test report format).
- tests ↔ security: `scripts/release-check.sh` judges the release build's own artifact records and must still end `release-check: viola only`; how two non-crate binaries relate to it and to `cargo deny` is security's/arch's call (per test-plan §9 Release build row / Build failure conditions).
- tests ↔ WSL CARRY: `scripts/wsl-provision.sh` parses ci.yml's `test`-job `tool:` line, and `pre-push` refuses off-pin tools; if the chunk changes that line or requires re-provisioning, the `--install-deps` CARRY activates (per test-plan §9 tool-install paragraph; §3 `pre-push`).
- tests ↔ arch: the harness `boot` bin dir and `viola run`'s pinned `bin/<version>-<hash>/` must both carry the sideload for E2E to exercise it — whether the pin path copies it is research's question (per test-plan §3 5-command implementation; §1 config and code-bearing state).

## Acceptance criteria contributions
- The H2 200-iteration loop ran on `windows-2025` twice, once with and once without the sideload, each run id and loss count recorded as chunk evidence, and ci.yml carries no gating loop afterwards (per test-plan §5 Module ↔ PTY; §10 Zero-flakiness budget).
- `#[cfg(windows)]` tests in a real tempfile home prove that an absent, hash-mismatched and signature-rejected sideload each still spawn the fake agent on the inbox ConPTY with the backend recorded, with hash pins asserted as literals (per test-plan §1 Vector 7 trigger; §8 Filesystem; §11 Unit).
- The existing Module ↔ PTY tests (resize incl. the pump-start window, no-EOF exit on handle, `.cmd` refusal) pass on `windows-2025` with the sideload in place (per test-plan §5 Module ↔ PTY).
- CI green on all three OSes, with each `test` job ending in `viola-harness gate --require coverage,doctest,playwright` at 85/95/80 per OS and the coverage ignore regex unchanged (per test-plan §9 Coverage report row; §10 Coverage thresholds).
