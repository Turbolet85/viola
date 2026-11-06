@amendment marker=2026-11-06-version-done-check
title: handle inheritance measured on portable-pty
section: §Data Protection (body: the Handle inheritance bullet)
change:
  - Handle inheritance rests on portable-pty closing every inherited descriptor in the child, witnessed by E2 and the live fd table: fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's, no file under the home (was close-on-exec descriptors, recorded, not established, the measurement owed to the route entry "Version done-check").
  - The clause that any portable-pty swap (portable-pty-psmux 0.9.7 the named candidate) must keep the property is gone.
why: the live round on claude 2.1.287 read the wrapped child's fd table with no viola or wrapper socket and no file under the home; the mechanism's wording is the overseer's wrap directive's (founder-delegated); the swap trigger is spent, portable-pty 0.9.0 dropping pin-utils upstream.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: cargo-deny advisories green, the ignore list empty
section: §Dependency Security (body: the `[advisories]` bullet; dependency-security.md: the `cargo deny check` reading)
change:
  - `[advisories]` reads `ignore = []`: no advisory is ignored, and a new ignore follows the Critical CVE response SLA, its `reason` naming the disabled feature (was one ignore, RUSTSEC-2017-0008, `serial` via portable-pty =0.8.1, the only expected one, re-checked whenever the pin moved).
  - `cargo deny check` reads advisories, bans, licenses and sources ok with the ignore list empty: RUSTSEC-2026-0441 (`pin-utils 0.1.0`) is closed by portable-pty `=0.9.0`, `cargo tree -i pin-utils` finding no package, and CI's `supply-chain` job is green on `849aad18` (was red on RUSTSEC-2026-0441 alone through portable-pty =0.8.1 → nix 0.25.1, owned by the CARRY on "Version done-check", the ignore list keeping its one entry).
why: the chunk moved portable-pty to =0.9.0, whose graph drops pin-utils and the `serial` crate family; the RUSTSEC-2017-0008 entry left with its crate, a narrowing, no ignore added; the CARRY's remedy is done.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: portable-pty pinned =0.9.0, the swap path retired
section: §Dependency Security (body: the pinning bullet, the Critical CVE response SLA; dependency-security.md: the portable-pty pin)
change:
  - The root graph's pin example reads portable-pty `=0.9.0` (was `=0.8.1`).
  - The CVE SLA's remedy reads: bump the crate (was bump, or swap along arch's named path, portable-pty → portable-pty-psmux 0.9.7).
  - `portable-pty` is pinned `=0.9.0` in `[workspace.dependencies]`, its comment saying only that the workspace pins every third-party version exactly; it brings `nix 0.28.0`, `serial2 0.2.39` and `cfg_aliases 0.1.1` into the Linux graph with no advisory ignore needed; `crates/viola-pty` compiles on it unchanged and `pty_backend()` stays `openpty`, viola-pty 33 of 33 green (was the `=0.8.1` pin carrying no Windows reason, "Version done-check" to measure moving off it).
why: the chunk moved the pin by `cargo update -p portable-pty` only; the `=0.8.1` pin and its Windows rationale are retired with Windows, and the swap trigger is spent, 0.9.0 dropping pin-utils upstream.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the board's read rule confirmed
section: §Authentication & Authorization (authentication-authorization.md: the board's read rule)
change:
  - The read rule reads: a home another user may read is refused, with or without an instance (was OPEN — the board's read rule standing on a delegate's answer, owner the Epoch 7 boundary batch).
why: the founder's own word, by dialog on 2026-11-06 19:34Z, recorded in the overseer's relay, confirmed Epoch 7's four PROVISIONAL items, the board's read rule among them.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: Measurement B recorded as known
section: §Data Protection (data-protection.md: Code-bearing artefacts, one tree's release builds)
change:
  - The one-tree, two-keys reading is recorded as known with no action in 0.1.0, the pinned binary's re-hash staying the guard (was: acting on it is the founder's).
why: the founder's own word, by dialog on 2026-11-06 19:34Z, recorded in the overseer's relay, took Measurement B as known with no action in 0.1.0; no cause is claimed.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the error-enum exceptions and the thiserror census
section: §Error Handling (error-handling.md: the `thiserror::Error` derive census, the error-enum divergences)
change:
  - The derive census counts product sources: nine, the test-only `crates/viola-e2e` harness's derive outside the count.
  - `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused` are divergences from the error-enum convention ratified as named exceptions (was open divergences, owned by a CARRY on the route entry Version done-check).
why: the derive grep reads ten rows, nine outside `crates/viola-e2e`, the tenth test-only; the rule's re-wording, the count staying nine, is the overseer's wrap directive's (founder-delegated). The ratification is the overseer's T1, confirmed by the founder's own word, by dialog on 2026-11-06 19:34Z.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: statusline-rate-limits reads the account's plan
section: §Input Validation (input-validation.md: the recorded `statusline_command`'s What to validate)
change:
  - The ledger rows' sentence now adds: `statusline-rate-limits` reads the account's status-line plan state, not the CLI; on an unchanged 2.1.287 a verify read it red (`23 pass  1 fail`, the payload carrying only `seven_day`); owner: 0.2.0 (finding F-06i) (was the row stated as relied on every build and stamped on 2.1.287, unqualified).
why: on an unchanged CLI 2.1.287 the rig home's verify read the row red because the account's status-line payload carries `five_hour` unknown and only `seven_day`; the overseer, founder-delegated, ruled the red a finding: the row reads the account's plan. The founder's own word of 2026-11-06 20:53Z, by dialog, deferred v1-27 and asked why viola leans on the user's status line; the overseer's wrap directive (items 1 and 3, founder-delegated) routes that question and the row's reading to 0.2.0's intent as finding F-06i — the routing is the directive's, not the founder's word.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
