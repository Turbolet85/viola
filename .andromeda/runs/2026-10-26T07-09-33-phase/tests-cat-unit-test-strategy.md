# tests · Unit Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:12) The server-verification decision on Unix rests on the peer's credentials; its cross-user rejection must be a unit case of a pure decision function fed injected ids, since hosted runners cannot drive a second user.
  > The peer-credential decision function with injected `(peer_euid, own_euid)`, covering the cross-user rejection boundary from test-scope Sec 1.
- (.andromeda/test-plan-detail/unit-test-strategy.md:18) The five client-side snapshot readers this chunk's verification reads through are named here with the rule that counts them; any move of a reader off `read_snapshot` (held behind CARRY 7's stop rule) must restate this list by that rule.
  > The five other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:20) The pid + start-time match the clients add must agree with the existing same-process rule and be unit-tested the same way: a pure function fed its values, no clock.
  > The liveness classifier (live / stale / gone) is the pure `classify(beat_age, same_process)`, fed ages directly (no clock): a pid that is dead or carries another start time is `gone` at any age; for the same process 4.9 s and 5.0 s are live, 5.1 s and an absent beat `stale`; `same_process` compares the snapshot's `started_at` with the pid's OS start time.
- (.andromeda/test-plan-detail/unit-test-strategy.md:23) If verification takes the strict-modes half with it, the instance check is the existing unit-tested function it must reuse; the Windows instance-tree DACL case stays owed to the later entry, not this one.
  > The instance check `strict::check_instance(home, instance_dir)` reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order.
- (.andromeda/test-plan-detail/unit-test-strategy.md:23) The boundary to "Home and code-bearing file integrity" for the Windows DACL case must be kept, not silently claimed here.
  > Not covered: a widened DACL on an instance tree on Windows (the crate's Windows cases widen the ledger paths only); that case is owed to the route entry "Home and code-bearing file integrity".
- (.andromeda/test-plan-detail/unit-test-strategy.md:48) A verification mismatch on a CLI verb must map to exit 21 through the existing unit-tested exit-code mapping.
  > The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21.
- (.andromeda/test-plan-detail/unit-test-strategy.md:60) The client's refusals made before any frame must keep their order and stay ahead of the new pre-frame verification; these named unit cases must still pass.
  > the client's `deliver_empty_text_is_refused_before_any_frame` and `refusal_of_reads_a_refused_character_before_an_empty_text` in `src/cmd/send.rs`); `answer` goes human-typing → unverified-cli → unknown-dialog; `control-character` precedes all of them.
- (.andromeda/test-plan-detail/unit-test-strategy.md:62) The hook client's `hook.dialog` path gains the verification; its bound and the unit case that pins it must hold unchanged.
  > the hook's bound is `DIALOG_HOLD_MAX` plus the reply grace (`dialog_deadline_is_the_longest_hold_plus_the_reply_grace`, `src/cmd/hook.rs`)
- (.andromeda/test-plan-detail/unit-test-strategy.md:63) Any new root-bin unit case that captures the verification's log lines must go through the shared scoped-capture helper, never a bare `with_default`.
  > every scoped capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`), which registers once per process a pinned, always-interested `tracing::Dispatch`
- (.andromeda/test-plan-detail/unit-test-strategy.md:65) A capture added in `viola-channel` for the client-side check runs on the crate's own bare-`with_default` mechanism, which no entry owns.
  > The three crate-level scoped captures that share the mechanism, in `crates/viola-state/src/lib.rs`, `crates/viola-channel/src/test_support.rs` and `crates/viola-channel/src/lib.rs`, run on a bare `with_default`; never observed red, they have no owner.
- (.andromeda/test-plan.md:184) The required negative coverage for this chunk's three outcomes on a mismatch, one per client surface.
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan.md:46) Server verification is declared same-user testable; only the cross-user part is held at the peer-credential boundary.
  > same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable
- (.andromeda/test-plan.md:250) Each OS branch of the verification (Windows pipe server pid, Unix peer creds, macOS euid + dir only) must have its tests run on its own runner.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:355) New unit cases sit inline in the source file they test.
  > Test file location: inline `#[cfg(test)] mod tests` in the same source file, in every `crates/viola-<area>` and in root `src/`.
- (.andromeda/test-plan.md:357) Table-driven match / mismatch cases take labelled rstest cases, or literal tables where the crate (viola-core) has no rstest.
  > Table-driven cases use rstest `#[case]`, and each case name must be a readable label (for example `#[case::esc_refused]`).
- (.andromeda/test-plan.md:376) The channel's unit surface already holds the peer-credential decision; the verification's decision belongs beside it.
  > - **viola-channel:** the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping, the FNV-1a endpoint golden vectors, the peer-credential decision with injected euids, and Windows handle non-inheritance.
- (.andromeda/test-plan.md:378) The state crate's liveness classifier and strict-modes checks are the unit-tested pieces a client's verification reuses.
  > the pure liveness classifier fed ages directly (no clock), Unix strict-modes and Windows strict-modes over synthetic SDDL descriptors, and the instance check.
- (.andromeda/test-plan.md:389) A double for the verification's process lookup is allowed only on a trait the product already defines at its seams (the liveness probe).
  > Module logic with mocked dependencies (mockall doubles only for traits the product already defines at its seams: `Pty`, liveness probe, `Clock`)
- (.andromeda/test-plan.md:834) The hook client's verification runs inside its connect deadline, which must stay below the spine deadline as its unit case asserts.
  > The hook's own connect deadline is the crate-private, provisional `CONNECT_DEADLINE` = 750 ms in the root bin's `src/cmd/hook.rs`, below the gate and asserted below it by `connect_deadline_is_below_the_spine_deadline`.
- (.andromeda/test-plan.md:868) The pid / start-time decision must be held in §4 unit cases, not only at E2E.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:873) A Linux dev-host run is no proof of the Windows or macOS verification branch.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:881) Expected exit codes and outcomes in the new cases are literals, never imported from the product.
  > NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle. The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan.md:882) Outcomes are asserted by kebab-case value and integer exit code.
  > NEVER compare `RefusalReason` or exit codes by `Debug` string. Assert on the serde kebab-case value and the integer exit code.
- (.andromeda/test-plan.md:905) A unit case needing a mismatched or malformed `snapshot.json` must write it through the writer and then alter its bytes, never hand-write it.
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version

## Bindings
- **Peer-credential decision ↔ body §1 viola-channel entity**: the unit case at (.andromeda/test-plan-detail/unit-test-strategy.md:12) is where the body's cross-user verification part is asserted, (.andromeda/test-plan.md:46).
- **Five snapshot readers ↔ body §6 E5 as landed**: the readers' list at (.andromeda/test-plan-detail/unit-test-strategy.md:18) is what keeps E5's steps 1–3 unbuilt and owed to this route entry, (.andromeda/test-plan.md:590); bound to the scope's held CARRY 7.
- **Same-process rule ↔ body §10 heartbeat row**: the verification's pid + start-time match agrees with `classify`'s `same_process` (.andromeda/test-plan-detail/unit-test-strategy.md:20) and the gone-at-any-age row (.andromeda/test-plan.md:842).
- **Exit-code mapping ↔ body IPC coverage trigger**: exit 21 on mismatch (.andromeda/test-plan-detail/unit-test-strategy.md:48) is the CLI leg of (.andromeda/test-plan.md:184).
- **Hook dialog bound ↔ body §10 connect deadline**: the hook's verification fits both (.andromeda/test-plan-detail/unit-test-strategy.md:62) and (.andromeda/test-plan.md:834).
- **Instance check ↔ route "Home and code-bearing file integrity"**: the Windows DACL case stays owed there (.andromeda/test-plan-detail/unit-test-strategy.md:23), the strict-modes refusal exits stated at (.andromeda/test-plan.md:437).
