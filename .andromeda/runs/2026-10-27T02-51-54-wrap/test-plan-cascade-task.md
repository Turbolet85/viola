# Task — test-plan · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The session
The pinned copy's absent-copy write in `viola_state::pin::pin_bytes` is now a no-clobber placement of a closed temp file (`viola_state::fs::place_private_new`, returning `Placed::Written` or `Placed::Present`), never a replace; a concurrent first start that finds the copy already placed re-hashes it (equal is reused, different is `PinError::HashMismatch`, the file left as found).
The race it fixes was read by its errno on the product run: the fake agent's `hook` receipt now carries `spawn_os_error`, and 4 of 24 two-instance boots failed with raw 26 (`ETXTBSY`) before, 0 of 24 after.
New unit witnesses landed: five `fs::tests::place_private_new_*` cases (a forced-window exec case among them, red without the guard on the dev host) and two `pin::tests::place_absent_*` cases.
Retired the test-side race workaround: `mcp_tools_path7_unverified_cli_answer` (Path 7's MCP leg) now boots the default `overseer` + `builder` pair unstamped instead of `builder` alone.
Folded CI red 2: the Windows plain-home oracle now reads the inherited untrusted write grant by its right, whichever SID holds it, because the measured refused entry on one runner was Authenticated Users, not Users; red 1 (Path 1's timeout) changed nothing and is runner variance under a WATCH.
Measured that one tree builds to two different release files whose loaded sections are all equal (cause not explained, the founder's item); CI `ci#40439188053` green 15/15.

## Reached files
- crates/viola-state/src/fs.rs
- crates/viola-state/src/pin.rs
- crates/viola-state/src/strict.rs
- src/bin/viola-fake-agent.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/preconditions.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-unstamped.py
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-before.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-after.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/red-green.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-builds.sh
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/operator-pass.md

## Touches
- §7 fake agent receipts (the `data-fake-agent` group): the `hook` receipt's description now lacks the new `spawn_os_error` field, set on a failed spawn of an absolute `command`; the `statusline` receipt beside it is unmoved.
- §7 Seed strategies, on-disk product state (test-plan.md body line about a plain home): its Windows reading names the refused entry as an inherited `S-1-5-32-545` write ACE, which measurement disproved; the case now reads any inherited untrusted write grant by its right.
- The same plain-home claim in the detail: the integration Windows read-back group, the test-data-fixtures values group and the viola-state unit group each state the Users SID as the refused entry and are stale.
- A plain-home claim that names no SID, or names Users only as an example of an untrusted SID, is unmoved.
- §6 Path 7: its MCP leg now runs on the default two-instance pair unstamped; Path 7 headings and true claims that name no instance count are unmoved, and the Path 7 procedure's CLI leg step (`agent-run boot --session p7 --unstamped --instance builder`) is unchanged.
- The test-plan contracts registry key for the 5-command implementation states Path 7's MCP leg as an unstamped `builder` alone: stale.
- §5 integration two-sided witnesses: the forced-window witnesses on record are all PTY or tui ones; a filesystem-placement witness now joins them (see New facts).
- §4 viola-state unit coverage list: it names the stamps and pin strict-modes refusals but not the new placement and lost-placement cases.
- §1 critical path `run` start sequence and Path 1's verification signal (pinned copy, tampered pinned exe exits 1): true as written; the copy is now placed once and never replaced, a concurrent first start re-hashing the standing copy.
- §1 filesystem trigger «pinned exe hash mismatch → exit 1»: now also reached by a lost placement whose standing copy differs (`HashMismatch`); the line stays true.
- §6 Contract suite, the pinned-exe content hash pinned by literal vectors: the hash itself is unchanged by the new arm; read it and confirm unmoved.
- The fake-agent copies' own `ETXTBSY` claim in test-data-fixtures (another writer's busy text file): a true claim of a different writer, unmoved.
- §5 Windows DACL read-back driver: the plain-home oracle now reads the DACL through a test-only helper that filters inherited allow ACEs carrying a write bit for an untrusted SID, with a control that cannot pass vacuously.
- §8 Mocking discipline: `place_private_new_with`'s `placed` hook is a private function parameter for the forced-window case, not a seam, mockall double, env read or feature; the seam list and the two `FAKE_AGENT_*` seams are unchanged.
- §11 Universal ban on test-side time and §2 determinism: the forced-window case forces its window through the hook, not a sleep; consistent.
- §5 Disk driver tempfile 3.27.0: unchanged; the placement uses its `into_temp_path` and `persist_noclobber`, already present.
- §6 «Skipped as untestable», a route entry's own local measurement kept in its `evidence/` with its own launcher: the race measurement and measurement B are such chunk-local measurements, never harness surface or a gate; consistent.
- §11 «NEVER treat a Linux-only local run as proof for Windows or macOS»: the forced-window Windows form is green-only on `windows-2025`, its red leg read on the Linux dev host only.
- The pre-push coverage count moved 3220 → 3227; test-plan states no test-case count, so any site that does state one is stale.
- The retired test-side workaround (Path 7's MCP leg booting `builder` alone to dodge the race): any text giving that as a race avoidance is now stale; `mcp_recorded_exchange.rs` keeps `builder` alone for no race reason, unmoved.
- §9 Release build: measurement B found one tree can build two different `viola` files with equal loaded sections; test-plan holds no reproducible-build claim, so a text stating release builds are byte-repeatable would be disproved.

## New facts
- F1 · integration-test-strategy · A filesystem-placement forced-window witness: `place_private_new_placed_copy_execs_at_once` (Unix) execs the placed copy right after a `Written` placement through `place_private_new_with`'s `placed` hook; its remove-the-guard leg is red on the dev host with `ExecutableFileBusy` (raw 26), green with the guard; its Windows form `place_private_new_placed_copy_runs_at_once` is green-only.
- F2 · unit-test-strategy · viola-state unit cases for the placement: `fs::tests::place_private_new_*` (bytes placed once at the mode with no temp left, a present name gives `Present` and is left byte-identical, a missing directory fails, the two forced-window cases) and `pin::tests::place_absent_*` (a copy another start placed: same bytes `Ok`, other bytes `HashMismatch` with the file byte-identical; a free path writes the copy).
- F3 · test-data-fixtures · The fake agent's `hook` receipt carries `spawn_os_error` when the spawn of an absolute `command` fails: the raw OS error as an integer, or `null` when the OS gives none; a successful spawn and the non-absolute arm add nothing.
- F4 · e2e-test-strategy · Path 7's MCP leg (`mcp_tools_path7_unverified_cli_answer`) boots the default `overseer` + `builder` pair unstamped, so it also witnesses two first starts on one fresh unstamped home both landing (the pinned-copy race), green on all three CI OSes.
- F5 · unit-test-strategy · On Windows the plain-home case's oracle is `strict::win::tests::inherited_untrusted_writes`: the inherited allow ACEs carrying a `WRITE_RIGHTS` bit for a SID `trusted_sid` does not trust; its control, a home `instance_tree()` makes, reads none.
- F6 · integration-test-strategy · `windows-2025` runners differ in a temp dir's inherited ACL by host on one image (one region Users write; another Authenticated Users `S-1-5-11` write with Users read), so an oracle naming one SID for a runner-inherited grant is runner-dependent and must read the grant by its right.
- F7 · mocking-stubbing-discipline · A forced-window test parameter on a private function (`place_private_new_with`'s `placed` hook) is not a seam: no env read, no feature, no mockall double, and it adds nothing to the test-seam list.

## Untouched
- §9 CI pipeline, matrix, uploads, caches and its build-failure lines (deny probes 13/13, release-check probes 5/5, ConPTY vendor 4/4): no workflow, gate script or count changed.
- §10 coverage thresholds, the coverage ignore regex, the mutation gate and the perf budgets and rows: unchanged; the perf run stayed green 7 of 7.
- Path 1's timing and the 20 s readiness kill line: red 1 is recorded as runner variance under a WATCH, the Path 1 test file is unchanged and no bound moved.
- §3 harness contract: no agent-run verb, flag, status field, verdict shape or session record changed.
- Paths 2 to 6 and scenarios E1, E2, E3 and E5: no send, wait, dialog, wheel, budget, env-strip, link or replay behaviour changed.
- Chaos, property, list-row integrity, error-sanitization and exit-cause suites: no case added or moved.
- Schema conformance and the obs schemas: `schemas/diag-line.v1.json` is unchanged, no log line, field, span or `detail` code was added, and `run.pin_copy` keeps `ok|pinned-hash-mismatch`.
- §7 fixture hygiene and recorded fixtures: no fixture, fixture set or scrub rule was added or changed.
- §1 tier, entity list and surfaces: no entity or surface added or removed.
- §11 anti-patterns and the test decisions log: no ban added, retired or reworded by this chunk.
