# Outcome — 2026-11-01-unix-endpoint-and-home-hardening

**green + smoke.** Every entry of the block fired by /implement reads green on the final tree: `entries 25 · green 21 · red 0 · recorded 1 · timeout 0 · not-run 3`, the three not-run the operator entries 23, 24 and 25. `v1-44` is `implemented`. Next: the operator pass, entries 23 (hygiene), 24 (push) and 25 (CI read), then `/andromeda-wrap-session`.

No live `claude` ran. The macOS and Windows readings are CI's, at the operator pass: until then, `security_negatives_socket_dir` and the euid read have run on Linux only.

## Files changed

26 tracked files and 1 new, under `src`, `crates`, `tests`, `schemas`, `Cargo.toml` and `.config`.

- New: `tests/security_negatives_socket_dir.rs`.
- Listed by research: the 24 of its modify list.
- Recorded in `doc:2026-11-01-unix-endpoint-and-home-hardening/scope-record`:
  - `tests/security_negatives_server_identity.rs` · companion · serves step 16 · self
  - `tests/hook_fail_open.rs` · companion · serves step 16 · self

`gate.py scope`: `scope: clean — changed 27 · listed 25 · recorded 2 (companion 2 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 663`.

## Deviations from the plan

1. **`peer_euid_of`, not the plan's `server_euid_of`.** The same read serves the wrapper reading a client, so the name says peer; `Client::server_euid` and `TokioClient::server_euid` keep the plan's names.
2. **`check_endpoint_dir` added beside `check_socket_dir`.** Three client seams and the bind each need the recorded endpoint's parent; one helper holds that.
3. **`first_life_on` stands in `tests/support/home.rs`,** not beside `first_life` in `tests/support/start.rs`, which the guard entry's pattern does not allow.
4. **The span recorder `Seen` moved to a new test module `refused_seen` in `src/cmd/client.rs`,** out of the macOS-gated `refused_inside`, so the two new cases that run on macOS can read it.
5. **`connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` opens the span by hand** (`wait.block`) around `connect`: the check sits in `connect`, whose eight callers are research's graph.
6. **A row in `crates/viola-mcp/src/result.rs`'s unit table was made and taken back with `git checkout`.** The file is on no list; `handlers.rs` witnesses the word.

## What the run measured against the plan

- **Research's 0700 reading of a bare temp dir was false.** `tempfile` 3.27 leaves the mode to the umask, 0755 here. The new client check refused nine `handlers` cases on the first run, and would have refused the impostors of `security_negatives_server_identity` and `hook_fail_open`, and `refused_inside`. This is step 16's named trap; each site now narrows its directory through `create_private_dir`. The sweep's no-change row for `tests/security_negatives_server_identity.rs` rests on the false reading.
- **Perf:** the five hook rows read a mean of 2.0, 2.0, 2.0, 2.0 and 2.3 ms, the highest max 2.8 ms, under `SPINE_DEADLINE`. The predicted rise of a few tenths of a millisecond over the P5 baseline does not show: this run's rows sit at or under it.
- **Counts by the plan's rules:** `human::refuse(` under `src/` 19, `run` 8; `thiserror` derives 9 (`command grep -rn "thiserror::Error" src crates --include=*.rs`, less `viola-e2e`). The plan's predictions hold.
- **Entries:** integration 347 passed, `pre-push` `linux-tests` 3446 passed with `breaches` empty.
- **macOS check-only clippy** (test-plan §9, the operator's word for this run): `CARGO_TARGET_DIR=target/maccheck cargo clippy --workspace --all-targets --features fake-agent --target aarch64-apple-darwin -- -D warnings`, exit 0.
- **Remove-the-guard:** ten runs, all red under the neutraliser, each file restored and read back by SHA-256, in `evidence/remove-the-guard.md`. Seven remove a guard; three force a refusing decision, since one account can make no foreign peer.

## What the tests do not prove

- A foreign euid and a foreign owner are not driven: the decisions are held by injected values, and each client seam's wiring by the forced-refusal readings of this run, which no committed test repeats. A mutant replacing a seam's `peer_admitted` read with an always-admit survives every committed test by construction (plan, test-plan §1).
- The Windows `grant_read` case and `board_scan_holds_a_read_granted_home…` compile here (entry 4) and run only on `windows-2025`.

## Surfaced

None that needs the operator before the wrap. For the wrap's cascade: `.claude/rules/` still name five causes, the owed client euid check and the board's two rules; the plan and `inputs#I2` win on those clauses until then. The hint's wording stays a planning form, the founder's word owed, per the plan's `## Beyond the plan`.

## Guard pass line

Entry 15, from its log, whole:

```
guard: allowed 25 — .config/nextest.toml, Cargo.toml, crates/viola-channel/src/client.rs, crates/viola-channel/src/lib.rs, crates/viola-channel/src/server.rs, crates/viola-channel/src/tokio_client.rs, crates/viola-core/src/lib.rs, crates/viola-e2e/src/harness/cleanup.rs, crates/viola-e2e/tests/harness_lifecycle.rs, crates/viola-mcp/src/tools.rs, crates/viola-mcp/tests/handlers.rs, crates/viola-state/src/board.rs, crates/viola-state/src/strict.rs, schemas/diag-line.v1.json, src/cmd/client.rs, src/cmd/hook.rs, src/cmd/run.rs, src/human/hints.rs, tests/cli_controls_not_disableable.rs, tests/cli_error_bodies.rs, tests/cli_list.rs, tests/contract_error_bodies.rs, tests/cross_exit_causes.rs, tests/security_negatives_socket_dir.rs, tests/support/home.rs · companions excluded 2 — tests/hook_fail_open.rs (serves step 16), tests/security_negatives_server_identity.rs (serves step 16) · changed 27 · excluded 0
guard: clean
```

Excluded and what each serves: `tests/hook_fail_open.rs` and `tests/security_negatives_server_identity.rs`, step 16: the impostor's temp socket directory made private, so each keeps reaching its verdict past the new check.

## Process hygiene

`gate.py census --comm viola --comm viola-fake-agent --args projects/viola/target/debug/viola --args projects/viola/target/harness`, 8 running. Each row's path is its own, none under this repository's `target/`.

| process | started by | final state |
|---|---|---|
| wrappers `pulse-builder`, `escher-builder`, `conductor-builder`, `andromeda-worker` (another tree's build) | not this run | left running: other sessions' — their owners stop them |
| `viola mcp` (the home's pinned copy), `viola hook`, two `viola wait` (other trees' builds) | not this run | left running: the overseer's and other sessions' — their owners stop them |
| this repository's test wrappers and fake agents | this run's tests and the two harness sessions | terminated: none in the census; entry 21's cleanup read `processes_gone` and `endpoint_gone` true |

## Decisions & corrections

- The operator's direction for this run: "No live claude session in this chunk, as the plan states; every test or control that spawns viola run, viola mcp or a wrapper gets a kill deadline. Documents and files land by rec.py put --doc and rec.py land, per the letter. Journal each long run in the run dir before waiting on it. Run the operator pass with the ci.py conclusion read (leg=operator) as usual; its first push carries the three door commits. Run the macOS check-only clippy before the first push." The journal is `journal.md` in this run dir.
- Guard refusals: the Bash guard's cat-heredoc arm twice and its filter arm once, each re-issued once in the form it passes.
- Recurrence, standing rule (`code-writing-discipline.md`, a scripted write fires no hook): a Python edit carried no formatter in its call; entry 2 read red at the first block.
- Recurrence, `testing.md` 2026-10-16 (an oracle copied from the first run): the frame-verb case typed the plan's `unable  builder  instance-unreachable`; `send` prints its mirror's form. Copied from the first run after.
- `serves` in a scope-record line is `step N` or one path, read by `gate.py guard`; a sentence there reads as outside (entry 15's first firing).
- Sweep hazard: a bare `tempfile::tempdir()` is not 0700 on a default umask, and a test that puts an endpoint in one meets the socket-directory check.
