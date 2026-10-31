# P5 review — 2026-10-31-sanitised-error-surfaces · "Sanitised error surfaces"

Mechanical checks: pass (0 lint hits, 25 gate entries parsed, snapshot citations clean). Validation-1: aligned, `scope.md` amended by P3's closure.

## What the research changed
- **The bodies do not already hold.** viola's own text is fixed, but both clients copy a peer's `message`, `data`, `refusal` and `detail` onto the CLI document, the MCP result and the text `refused: <reason>`. This is real product work, not only proof.
- **"Exit 11" is wrong for the `viola mcp` process.** 11 is the unit test child's own code. The process exits 1 with both streams empty. The product stays; the route's PREREQ was corrected in scope, and four masters are corrected at the wrap.
- **The scope's "`error.data` a code or null" was wrong.** `-32602` carries two closed objects by contract. The rule held is closed fields and no free text. The wire form does not change, and `v` stays 1.
- **Other gaps:** `CoreError`, which the masters list, does not exist. Test-plan §6's error-body check after every E2E test was never built. Test-plan §5's sentence about an oversize `answer` printing a `--json` document is false.

## Goal
Every error a machine reader gets from CLI `--json`, MCP or the channel carries a code and a fixed message. The chunk screens what clients copy from a peer, tells the `-32602` causes apart on `channel-response`, folds `Refusal` into `AgentError`, closes the PREREQ's two test gaps, and adds a witness per surface and an enumeration with a census.

## Top acceptance criteria
- `verification-matrix.json#v1-47`, previewed below.
- A peer's free text reaches no CLI document, MCP result or `refused:` text.
- The seven wire bodies are unchanged and pinned by literals.
- `channel-response` carries `unsupported-version`, `invalid-params` or `release-from-driver`, with its level unchanged.
- `Refusal` is gone, and its outputs are byte for byte the same.
- A pre-`initialize` line past `MAX_FRAME` gives `ServeError::Initialize`, process exit 1 and one `parse-rejected` line. Each of the five `BOUNDARIES` rows names a case driving past its cap.

## Touchpoints
3 new test files (`tests/contract_error_bodies.rs`, `tests/channel_error_bodies.rs`, `tests/cli_error_bodies.rs`), 23 modified. No dependency moves and no spec master is a touchpoint.

## Test commands
25 entries: fmt, clippy on both targets, G1, lint probes, unit, two integration entries, two E2E entries, `run --perf` (the changed line sits on the timed hook paths), `cargo deny`, `channel-tokio-free`, the preservation guard, the smoke, G2, G4, `pre-push`, then hygiene, push and CI on the operator's leg. No live leg and no mutation entry.
Baselines: green on the untouched tree but for entries 8 and 11, the predicted red (their binaries do not exist yet). The guard's control read red on three outside paths (`.andromeda/runs/2026-10-31T07-21-45-phase/p5-controls.md`).

## Concretization preview — `v1-47`
- **Current:** CLI machine-readable errors, MCP errors, problem responses and channel errors carry only codes and fixed messages; no log, diagnostic, fixture, event or accessibility artifact ever holds the GUI token, cookie, launch URL or stripped parent-session values; content and drift reports reach only owner-readable per-instance detail files; a secret scan after every end-to-end test proves it.
- **Proposed:** CLI machine-readable errors, MCP errors and channel errors carry only codes and fixed messages; no log, diagnostic, fixture or event holds a stripped parent-session value; content and drift reports reach only owner-readable per-instance detail files; `viola-harness secret-scan` proves the floor after each job's end-to-end run over every kept home, the harness capture and the JUnit report. The web clauses are out per the cap's notes (the founder's ruling of 2026-10-16).
- **The one wording to read for weakening:** "after every end-to-end test" becomes "after each job's end-to-end run". The scan is job-level as built, and it reads every test's kept home except the one `viola-chaos-*` home, by the founder's ruling of 2026-10-04.

## Sampler
Forced on the operator's word (`inputs#I1` item 1).
- `sampler: architecture E carried 94 · the cascade lost 4 (occupied-resources#1.1, stack-and-technologies#1.23, stack-and-technologies#1.3, standard-contracts#11.1)`
- `sampler: security-plan E carried 84 · the cascade lost 7 (authentication-authorization#1.2, data-protection#9.1, dependency-security#2.1, dependency-security#3.1, security-anti-patterns#4.1, security-anti-patterns#4.2, threat-model-summary#13.1)`
- `sampler: design-system E carried 17 · the cascade lost 0`
- `sampler: test-plan E carried 117 · the cascade lost 7 (ci-integration#1.7, ci-integration#1.9, quality-gates-coverage-targets#3.1, test-data-fixtures#1.10, test-data-fixtures#1.2, test-data-fixtures#1.6, test-data-fixtures#2.1)`
- `sampler: obs-plan E carried 137 · the cascade lost 1 (ci-integration#2.5)`
- `sampler: a11y-plan E carried 42 · the cascade lost 1 (a11y-scope-summary#12.1)`
The plan takes each lost item, marked `(sampler)`. One changed the block: the `run --perf` entry.

## The batch agent type, on a live cascade
228 agents across the four batches and 2 on the re-spawn, each spawned `andromeda-batch`. None died, none wrote nothing, and none did the orchestrator's work. The path check was clean on all five. The classifier batch took 2602 s of the run's wall time. Two strong readers took the contract's one retry on validation rows, and both passed; 7 runs were placed by `fanout.py move`. No hard failure under `inputs#I1` item 3.

## Standing items
- **Records store (`inputs#I1` item 8):** nothing to report. Every append landed, no guard refused `.andromeda/db/` or the old ledger, and the health rows read clean at session start.
- **Host:** `target/`, the `target/e2e-home` link and `/tmp/viola-e2e-home-<uid>` were all gone. I restored the link to its operator-set form for the baselines. `df -h /tmp`: 23G free.
- **Path 1 WATCH:** a new green at `ci#40467224735`, 14.205 s on `test (ubuntu-latest)`. The wrap tallies it to 2/3.

## Beyond the plan
- **An owner for the remaining enum divergences.** `ServeError` and `StampsError` stay open as the founder's Epoch 5 batch left them, and the census adds `PinError`, `StampsReadError` and `Refused`. Proposed owner: a CARRY on the existing entry «Version done-check». The wrap writes it.
