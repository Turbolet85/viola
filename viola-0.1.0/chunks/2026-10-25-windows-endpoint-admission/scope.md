# Scope — Windows endpoint admission (2026-10-25-windows-endpoint-admission)

**Working entry (viola-0.1.0/working-route.md, Epoch 6 — Security hardening):** Windows endpoint admission — pipe
access for the owning user and SYSTEM only, remote pipe clients rejected, squatted name blocks start (per
security-plan §Authentication).

**Invocation:** the operator's (`inputs#I3`), with the overseer's directive for this phase (`inputs#I1`): forced
sample at P2, every transcript and the run dir kept, a hard cascade failure stops the phase.

## What it builds
The Windows half of security-plan §Authentication & Authorization's "IPC access control (Windows)" clause, brought
from "partly landed" to fully asserted on the Windows CI leg: the wrapper's named-pipe listener admits only the owning
user's SID and SYSTEM, rejects a remote (network) pipe client, keeps the first-pipe-instance behaviour so a name
another process already holds refuses `viola run` at start, and holds non-inheritable handles. The entry is scoped to
what chunk 2026-09-27-wrapper-channel did NOT already assert (CARRY 1 below).

- Already asserted at HEAD, re-verified, never re-built [premise-corrected: the DACL read-back is a unit test in
  `crates/viola-channel/src/server/win.rs:121`, not in `tests/security_negatives_channel.rs`, which holds only the
  client SQOS negative at line 104]: the protected DACL from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` read back as two
  allow ACEs (`owner_only_sddl_is_protected_user_and_system`, `bound_pipe_dacl_reads_back_protected_user_and_system`);
  handles non-inheritable on both ends (`pipe_handles_on_both_ends_are_not_inheritable`, `server/win.rs:133`); the
  squatted-name exit-1 start refusal (`src/cmd/run.rs` `refuse_squatted`, `tests/path1_run_start.rs:116-124`,
  `tests/cli_instance_state.rs:263`).
- Already asserted at HEAD [premise-corrected: (b) is no gap — `bind_of_a_live_endpoint_is_taken_until_it_drops`
  (`crates/viola-channel/src/server.rs:970`, every OS) and `channel_endpoint_second_bind_is_taken_while_the_first_lives`
  (`tests/channel_endpoint.rs:141-147`) bind a second listener on a held name and read `BindTaken`, which on Windows is
  the first-instance flag's `ERROR_ACCESS_DENIED` mapped at `server.rs:108-112`]: the first-pipe-instance behaviour as
  a listener property.
- Not yet asserted, the chunk's own work: (a) the listener rejects a remote client — verified: interprocess 2.4.4 sets
  `PIPE_REJECT_REMOTE_CLIENTS` on every instance (`PipeListenerOptions::new()` `accept_remote: false`,
  `create_instance.rs:100-101`) and no viola test asserts it (the one hit is the doc comment `server.rs:95`); (c) the
  DACL's effect, not only its text: an open by a principal outside the two ACEs is denied — verified unasserted;
  whether a single-account hosted runner can witness (a) and (c) is P4's fork (research.md, Open questions), else each
  is owed to "Windows-only live measurements".
- Surfaces touched [premise-corrected: no product file needs a change — viola's listener already takes interprocess's
  defaults whole (`server.rs:104-107`) and the workspace windows-sys pin already carries every feature a witness needs
  (`Cargo.toml:247`)]: the Windows negatives in `tests/security_negatives_channel.rs`; no wire contract, no exit code
  and no refusal word changes (`squatted-name`, exit 1 stand).
- Surfaces added by the founder's P4 answers (`inputs#I5`; validation-1, intent-incomplete): fork (a)'s pipe-type
  read-back adds the `Wdk_Storage_FileSystem` feature on a root `[target.'cfg(windows)'.dev-dependencies]` windows-sys
  entry (`Cargo.toml`); fork 3's root cause adds one shared test helper in `src/main.rs`'s `test_support` and routes the
  root bin's nine scoped captures through it — every edit inside `#[cfg(test)]` code, no product behaviour moved.

## Boundaries
- Server verification before any frame (the client's SQOS open, pid and start-time check, the dated exceptions) is the
  next entry, "Server verification before any frame" — not this chunk.
- Unix socket hardening is "Unix endpoint and home hardening"; home and code-bearing file integrity is its own entry.
- No boundary is widened: this chunk narrows or asserts what crosses the Windows pipe. A widening found on the way halts
  for the founder's live word (`inputs#I1` item 5), never a recorded direction.

## Folded freight (working-route.md, the entry's own line)
- CARRY (chunk 2026-09-27-wrapper-channel): that chunk landed part of this ahead of Epoch 6: the protected listener DACL
  (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back canonical as two FA allow ACEs,
  `tests/security_negatives_channel.rs`), interprocess's defaults kept (`accept_remote(false)`, first-instance,
  non-inheritable), and the squatted-name exit-1 start refusal. Take-up scopes this entry to what that chunk did not
  assert. → folded into "What it builds" above; its test-file coordinate re-verified at P3.
- CARRY (chunk 2026-10-19-overseer-switch-to-viola, the founder's own word at its wrap's P5 halt,
  `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/inputs/` entry I22 of that chunk): 
  `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 went red once
  under a plain `cargo test` (libtest threads; its log capture found no `send-refused` line; since
  2026-10-18-compact-post-condition-ledger-row) and has since run green only under nextest — run the case under a plain
  `cargo test` three times, or fix its log capture. → an obligation of this chunk (its own acceptance criterion).
- The CARRY's mechanism claim, kept verbatim: "its log capture found no `send-refused` line" under libtest threads
  [premise-corrected: not reproduced at P3 — `cargo test --bin viola` three times at `b6740d59` on the Linux dev host,
  case_4 ok 3 of 3 (research.md, Measurements); the capture is `with_default`, thread-scoped
  (`src/run/send/test_support.rs:418-424`), in a binary whose `run::gate` test installs a global subscriber; the cause
  stays unknown, neither confirmed nor falsified]. The same three runs were red elsewhere, in three tests sharing
  process-global state (`run::gate::tests::feed_panic_…` 3/3, `obs::tests::log_config_rejection_writes_one_warn_line`
  2/3, `cmd::verify::stop::tests::install_…case_2_sigterm` 2/3) — met during this chunk, dispositioned at P4.
- watch: the macOS readiness stall, `wrapper <name> not ready` with a wrapper that wrote `process-start{subject:"self"}`
  and nothing more, inside `run.start` before the version gate, its phase not named; recorded on 1 of 4 macOS attempts
  read (ci#39321117796 attempt 1), its witness the stall report `tests/support/home.rs` `wait_ready` writes into the
  panic; green on `test (macos-latest)` at ci#39400402749 attempt 1 and ci#39477233397 attempt 2; closed UNMET at its
  chunk on the founder's own word «Закрыть + WATCH» (`viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/inputs/I9-relay-1.md.txt`)
  (2/3; since 2026-10-19-newer-claude-cli-rows; retires: a recurrence, or 3 green runs).

## CI since the last wrap (Setup 5a, `ci-no-marker.json` in the run dir)
Every sha from the last wrap's flip (`1862fcfb`) through HEAD (`b6740d59`), newest first:
- `b6740d59` red · 15/15 · first fail +21 s `supply-chain` · wall 686 s · ci#40208779387
- `d08eeada` red · 15/15 · first fail +23 s `supply-chain` · wall 664 s · ci#40205704618
- `d6298fbd` red · 15/15 · first fail +32 s `supply-chain` · wall 585 s · ci#40202532073
- `f1086a2b` red · 15/15 · first fail +25 s `supply-chain` · wall 675 s · ci#40199572471
- `3b1d9e28` untested — pushed under a later tip
- `b8014bf4` red · 15/15 · first fail +23 s `supply-chain` · wall 632 s · ci#40192145623
- `6467439b` red · 15/15 · first fail +24 s `supply-chain` · wall 636 s · ci#40190099627
- `e4aeb25b` red · 15/15 · first fail +22 s `supply-chain` · wall 1055 s · ci#40184702804
- `24eef748` untested — pushed under a later tip
- `bda69648` untested — pushed under a later tip
- `1862fcfb` green · 15/15 · wall 995 s · ci#39493188302

- The red, not this chunk's (runner-only, closed against its recorded run: verified still failing at ci#40208779387,
  sha `b6740d59`, job `supply-chain`): every red run fails `supply-chain` alone, on `cargo deny`'s advisories check —
  RUSTSEC-2026-0386 (dated 2026-10-13, `unsound`, `tokio::io::unix::AsyncFd` constructors) against `tokio 1.53.1` in
  `Cargo.lock`, fix `>=1.53.3` (ci#40208779387, sha `b6740d59`, job `supply-chain`, its log's one `error` diagnostic).
  External decay (playbook: the lock did not move, the advisory DB did); it does not intersect what this chunk builds.
  Disposition, the overseer's answer at this P1 (founder-delegated, `inputs#I2`): owned by a NEW route entry
  "tokio RUSTSEC-2026-0386 lock bump", placed right after this entry so it runs next; no ignore-list widening. Phase
  writes no route entry (route-contract §Write row 17 is the wrap's), so the insertion rides this chunk's P5 review
  card `## Beyond the plan` for the wrap to make. This chunk carries no acceptance criterion on it; its own CI runs
  will read `supply-chain` red until that entry lands.
