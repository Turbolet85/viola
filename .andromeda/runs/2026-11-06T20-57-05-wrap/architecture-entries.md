@amendment marker=2026-11-06-version-done-check
title: portable-pty pinned at =0.9.0
section: §Stack and Technologies (body: the stack line; stack-and-technologies.md: the PTY layer row) · §Established Decisions [PTY] (body; established-decisions.md) · §Infrastructure Patterns (build-system: the `[workspace.dependencies]` clause; project-directory-structure: the `viola-pty/` tree line)
change:
  - Every pin site reads portable-pty `=0.9.0` behind viola's own `pty` seam (was `=0.8.1`): the body stack line, the body and detailed [PTY] decisions, the PTY layer row, the build-system workspace pin and the `viola-pty/` tree comment.
  - The body [PTY] rationale reads: exact, as the workspace pins every third-party version; 0.9.0's graph holds neither `pin-utils` (RUSTSEC-2026-0441) nor the `serial` crate family, and `viola-pty` builds and passes its tests on it unchanged (was: 0.8.1 spike-proven, the move off it owed to "Version done-check").
  - The detailed [PTY] rationale reads: exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils`, so RUSTSEC-2026-0441 does not reach the build, and `viola-pty` compiles unchanged on it. The 0.8.1 spike-proven sentence, the 0.9.0 Windows garbage-read sentence, the move owed to "Version done-check" and the portable-pty-psmux 0.9.7 swap trigger are gone.
why: the chunk moved `portable-pty` from `=0.8.1` to `=0.9.0` in `[workspace.dependencies]`; the graph lost `pin-utils` and the `serial` family (`nix` 0.25.1 to 0.28.0), `viola-pty` compiles unchanged and passes 33 of 33 on it, and `pty_backend()` stays `openpty`; the Windows rationale and the spent swap trigger were retired with it.
kept: the seam's operations, the handle-based exit detection, `host_stdin()`, `HostTerminal` raw mode and `pty_backend()` as `openpty` stand as they read.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the Git Bash rewritten-path warning and reason retired
section: §Established Decisions [Hook Transport] and [CLI Conventions] (body; established-decisions.md)
change:
  - The [Hook Transport] rationale reads «exec form skips shell startup» in both the body and the detailed decision (was: skips Git Bash path rewriting and shell startup).
  - [CLI Conventions] no longer says viola warns when an argument carries a Git Bash rewritten-path prefix, and its rationale reads only that agents need machine-parseable refusals (was: Git Bash rewrites `/skill` arguments into Windows paths, and agents need machine-parseable refusals), in the body and the detailed decision.
why: the chunk removed the Git Bash rewritten-path warning (`warn_if_rewritten` and `looks_rewritten` in `src/cmd/send.rs`, their unit table and CLI witness), so `send` writes no stderr line on a rewritten path. This is the overseer's T2, accepted on the founder's own word of 2026-11-06 19:34Z and relayed by the overseer. On a Linux-only build Git Bash rewriting does not bear on exec form.
kept: prompt text still reaches viola only from stdin or `--file`, never from a leading-slash argument; `viola revive --json` and the other [Hook Transport] reasons (http, `mcp_tool`, the 1.5 s SessionEnd budget, the tier-map ledger row) stand.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the dialog-hold ceiling owned by 0.2.0's first entry
section: §Established Decisions [Hook Contract] (body; established-decisions.md: the dialog-hold constants and the ceiling item)
change:
  - The body [Hook Contract] reads the ceiling PROVISIONAL, owner: 0.2.0's first entry (was owner: "Version done-check").
  - The `DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX` mark reads PROVISIONAL, owner: 0.2.0's first entry.
  - The ceiling item reads: a hold near the 3600 s ceiling has not been read live; the ceiling is PROVISIONAL, owner: 0.2.0's first entry (was: first read live at "Version done-check").
why: the chunk's live round did not read the 3600 s hold, because an unverified CLI replies `null` to a dialog at once. The ceiling stays PROVISIONAL and passes to 0.2.0's first entry on the founder's own conditional of 2026-11-06 19:34Z (a red hold leaves the ceiling PROVISIONAL, owned by 0.2.0's first entry), relayed in the overseer's wrap directive item 2.
kept: `dialog_hold_s` stays 60 to 3600 s, default 60; the reply-read bound `DIALOG_HOLD_MAX + 5 s` (3605 s) and the 3615 s hook `timeout` stand.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the one-enum divergences ratified as named exceptions
section: §Established Decisions [Error Handling] (body; established-decisions.md: the decision and the census item) · §Conventions (Rust error types) · §Inherited Defaults (Errors)
change:
  - `viola-mcp`'s `ServeError`, `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused` read as ratified exceptions beside `viola-pty`'s hand-written `PtyError`. The detailed decision reads six ratified exceptions. Before, there was one ratified exception, two open divergences and three the census added, the rule standing, their fold or ratification open, and all five owned by a CARRY on "Version done-check". That CARRY clause is gone.
  - The census item reads: the product sources, outside the test-only `crates/viola-e2e`, hold 9 `thiserror::Error` derives; the grep's one further row, `crates/viola-e2e/src/harness/mod.rs`, is test-only (was: the product sources hold 9). The Conventions line reads the nine with `crates/viola-e2e` outside the count.
why: the five were ratified as named exceptions under the overseer's T1, on the founder's own word of 2026-11-06 19:34Z, relayed by the overseer. The census grep reads 10 rows, 9 of them outside `crates/viola-e2e`. Re-wording the rule to the product sources, with 9 staying, is the overseer's wrap directive item 4 (the operator's request, founder-delegated).
kept: `AgentError` holding `StampsMalformed`, anyhow 1.0.104 only in the `viola` bin, `RefusalReason` in `viola-core` and the rationale stand.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: Measurement B known, with no action in 0.1.0
section: §Established Decisions [Deployment / Distribution] (body; established-decisions.md: the two-files-from-one-tree item)
change:
  - The body rationale reads: its cause not explained, known and left without action in 0.1.0, the re-hash the guard (was: acting on it owed to the working-route entry "Version done-check").
  - The item reads: its cause is not explained; it is recorded as known, with no action in 0.1.0, and the pinned copy's re-hash stays the guard (was: acting on it owed to "Version done-check").
why: on the founder's own word of 2026-11-06 19:34Z, relayed by the overseer, Measurement B is known and needs no action in 0.1.0, and the pinned copy's re-hash stays the guard.
kept: the measurement stands as recorded: six cold builds gave two files, unequal in 3 of 34 sections, every loaded section equal.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: canonical_home's Windows verbatim-drive clause gone
section: §Cross-cutting Patterns (cross-cutting-patterns.md: the viola-home bullet)
change:
  - `canonical_home` reads: canonicalises a home that exists (`std::fs::canonicalize`) and keeps `std::path::absolute`'s spelling for one that does not, creating nothing (was: a Windows `\\?\C:\…` written back as `C:\…`, every other verbatim form kept).
why: the chunk retired `plain_drive` and its Windows verbatim-drive handling; its call in `canonical_home` was removed, and the signature is kept.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the statusline-rate-limits row reads the account's plan state
section: §Cross-cutting Patterns (cross-cutting-patterns.md: the statusline ledger rows) · §Established Decisions (established-decisions.md: two new items)
change:
  - The cross-cutting fact reads that `budget-paused` rests on a stamped row, one that reads the account's plan state, not the CLI alone (was: rests on a stamped row).
  - New item: the `statusline-rate-limits` row reads the account's plan state, not the CLI alone. On an account whose status-line payload carries `five_hour` `unknown` and only `seven_day`, `viola verify` fails the row on an unchanged CLI (`stamped 2.1.287  23 pass  1 fail`, `[21/24] statusline-rate-limits`, on the rig home, as measured on the Linux dev host). The dev home's stamps are unmoved. What the budget gate stands on is OPEN (finding F-06i), owner: 0.2.0's intent.
  - New OPEN: whether the budget gate stays and what it stands on, given that it rests on the user's status-line `rate_limits` payload, which reads the account's plan state (finding F-06i) · owner: 0.2.0's intent.
why: on an unchanged 2.1.287 the rig home's verify read the row red because the account's payload carries only the seven-day window. The overseer (founder-delegated, 20:16Z) ruled the red a finding: the row reads the account's plan. On 2026-11-06 at 20:53Z the founder, in his own word, deferred v1-27 and asked why viola leans on the user's status line. The overseer's wrap directive (items 1 and 3, founder-delegated) routes that question and the row's reading to 0.2.0's intent as finding F-06i; the routing is the directive's, not the founder's word.
kept: the ledger stays 24 rows, stamped on the dev home.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: RUSTSEC-2026-0441 closed and every CI job green
section: §Established Decisions (established-decisions.md: the RUSTSEC-2026-0441 item) · §Infrastructure Patterns (ci-cd-approach: the jobs sentence)
change:
  - The RUSTSEC-2026-0441 item is retired. It had said `pin-utils`, via `portable-pty =0.8.1` and `nix 0.25.1`, reads red in supply-chain's advisories family, owned by a CARRY on "Version done-check".
  - The jobs sentence reads 7 jobs, every one green on ci#42288148289 (was: every one green on ci#42030025050 but `supply-chain`, red on RUSTSEC-2026-0441 alone).
why: the `=0.9.0` move drops `pin-utils` from the graph (`cargo tree -i pin-utils` finds no package), `cargo deny check` reads advisories ok with `ignore = []`, and ci#42288148289 ran all seven jobs green, `supply-chain` among them.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the directory tree's removed-file pointers gone
section: §Infrastructure Patterns (project-directory-structure: the tree)
change:
  - The tree's seven pointer lines to files removed with the Windows code, each recoverable at the tag multi-os-final, are gone: `(conpty.rs)`, the `sideload` module note, `(conpty-vendor.sh)`, `(vendor/conpty/)`, and `(windows-mutants.yml)` with its two comment lines.
  - The `cmd/*.toml` trycmd line is gone.
why: this whole-fact edit is the overseer's wrap directive item 4 (the operator's request, founder-delegated). The trycmd line named a `tests/cmd/` the tree does not hold.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
