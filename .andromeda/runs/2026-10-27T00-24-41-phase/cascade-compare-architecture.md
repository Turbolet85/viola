# Cascade compare — architecture · phase · 2026-10-27-pinned-copy-race-and-reproducible-builds
E items 93 · held 27 · lost at classify 4 · lost at strong 0 · beside a placed fact - · cascade only 8 · listed 4 · body 39 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### conventions#4.1 · lost at classify · .andromeda/architecture-detail/conventions.md:46
- (.andromeda/architecture-detail/conventions.md:46) Verify's home-tree check comes before its `cli` log and before its `pin_exe`; the fix must not move the pin ahead of that check.
  > Exit `1` also covers `viola verify` refusing the home tree (`check_home_tree` with no name and no key: the home, `ledger/`, `bin/` and `plugin/`, each that exists), checked before its `cli` log opens: the home-state `unable:` / `hint:` pair on stderr, no step line, no summary and no log line.

### occupied-resources#1.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:8
- (.andromeda/architecture-detail/occupied-resources.md:8) The start arbiter on Unix is keyed per instance name, so it does not order two different instances' first pins on one home. The fix must leave the arbiter and its taken-name refusal unchanged.
  > The start arbiter is the sibling `<socket dir>/viola-<h12>.lock`: `run` holds an exclusive lock on it, and a held lock is the taken-name refusal.

### stack-and-technologies#1.24 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:33
- (.andromeda/architecture-detail/stack-and-technologies.md:33) Release tooling (dist, cargo-auditable) is v1.x, not v1: part B measures the current release profile and adopts none of it; any build change on the measurement is the founder's.
  > | Release (v1.x, not v1) | dist (cargo-dist) 0.33.0 + cargo-auditable 0.7.6; later self_update 1.3.0 | Public signed releases and installers once distribution is in scope |

### standard-contracts#11.1 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:87
- (.andromeda/architecture-detail/standard-contracts.md:87) The race's symptom (a hook spawn of the pinned path failing, so `session-start` never lands) is read through this contract. The two-sided witness must read a `session-start` line in `events.ndjson` that a `hook.event` appended for each instance, and the fix must leave the hook's fail-open path unchanged.
  > `hook.event`, sent id-less by `hook` once its instance check (`trusted`) and its server check have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot or a failed server check sends nothing and the hook fails open): `{ts, event:{kind, data}}`.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### established-decisions#25.1 · listed (lost at classify) · .andromeda/architecture-detail/established-decisions.md:235
- (.andromeda/architecture-detail/established-decisions.md:235) A new failure the fix needs folds into an existing `viola-state` error enum with a fixed message, adding no new enum.
  > - **[Error Handling] thiserror 2.0.20, one enum per workspace crate (one ratified exception:

### established-decisions#25.1 · listed (lost at classify) · .andromeda/architecture-detail/established-decisions.md:235
- (.andromeda/architecture-detail/established-decisions.md:235) `viola-state`'s divergences are named and their fold is open; the chunk widens none of them.
  > A third open divergence, not an exception: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`; the rule stands, and its fold or ratification is open;

### occupied-resources#34.1 · listed (lost at classify) · .andromeda/architecture-detail/occupied-resources.md:225
- (.andromeda/architecture-detail/occupied-resources.md:225) The race appears only on an unstamped home, because a stamped one is pinned by `viola verify` first. A reproduction must boot harness sessions `--unstamped`, or use a fresh unstamped root home, or it passes without reaching the race.
  > `target/e2e-home/viola-session-*/home` (harness sessions, stamped at `boot` step 4 by `viola verify` against the fake agent unless `--unstamped`), `target/e2e-home/viola-test-*/home` (root rstest homes, `tests/support/home.rs`; the `stamped_home` fixture stamps through `viola verify` against the fake agent at the recorded 2.1.287)

### occupied-resources#34.1 · listed (lost at classify) · .andromeda/architecture-detail/occupied-resources.md:226
- (.andromeda/architecture-detail/occupied-resources.md:226) On the dev host, test homes sit on the tmpfs link, chosen so the pinned-copy write is fast. The race window is narrower there than on the shared volume, so the two-sided witness must show the window is reachable on this rig. The CARRY's 6-of-24 was read here.
  > It is there so a start's pinned copy does not wait on the shared volume's dirty-page budget, as measured at chunk 2026-10-07-test-homes-off-the-contended-volume `evidence/contended-reading.ndjson`: inside another project's build, five 52 MB writes at once read a 10.999 s median on the shared volume and 0.285 s at most through the link, with the verify-driven test binary green in the same window.

### occupied-resources#31.1 · listed (lost at classify) · .andromeda/architecture-detail/occupied-resources.md:204
- (.andromeda/architecture-detail/occupied-resources.md:204) Repeated concurrent boots on the dev host need the tmpfs backing to stay above its headroom floor. The keepers refuse a backing below 9 GiB free, so the free space is read before the boot loops run.
  > which refuse a target that is not a real, owner-only directory or whose filesystem reads less than the 9 GiB headroom floor free (Repository, below).

### stack-and-technologies#1.15 · listed (lost at strong) · .andromeda/architecture-detail/stack-and-technologies.md:24
- (.andromeda/architecture-detail/stack-and-technologies.md:24) A new pin failure the fix surfaces must be a typed variant of `viola-state`'s thiserror enum, its chain only at the bin edge.
  > thiserror 2.0.20 (per crate; `viola-pty`'s `PtyError` is hand-written); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains built at dispatch and recorded only in the owner-only instance detail file

## Held

- conventions#2.1
- conventions#3.1
- cross-cutting-patterns#1.1
- established-decisions#1.1
- established-decisions#14.1
- established-decisions#16.1
- established-decisions#2.1
- established-decisions#23.1
- established-decisions#24.1
- established-decisions#24.5
- established-decisions#27.1
- established-decisions#28.1
- established-decisions#28.2
- established-decisions#5.1
- established-decisions#6.1
- occupied-resources#22.1
- occupied-resources#27.1
- stack-and-technologies#1.1
- stack-and-technologies#1.10
- stack-and-technologies#1.2
- stack-and-technologies#1.21
- stack-and-technologies#1.22
- stack-and-technologies#1.23
- stack-and-technologies#1.4
- stack-and-technologies#1.9
- standard-contracts#2.0
- standard-contracts#3.0

## Cascade only

- established-decisions#22.1
- established-decisions#26.1
- established-decisions#30.1
- occupied-resources#10.1
- occupied-resources#23.1
- occupied-resources#26.1
- occupied-resources#6.1
- occupied-resources#8.1
