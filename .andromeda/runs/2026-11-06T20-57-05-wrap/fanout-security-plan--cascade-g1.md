@proposal n=1 detector=D-cascade severity=warning file=authentication-authorization anchor=fct-pq4mk5dy basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-gtooygvh
section: ## Authentication & Authorization
change: remove the line «OPEN — the board's read rule stands on a delegate's answer · owner: the Epoch 7 boundary batch»; the board's read rule now stands on the founder's own word (2026-11-06, Epoch 7's four PROVISIONAL items confirmed, the board's read rule among them)
sidecar: chunk 2026-11-06-version-done-check — the board read rule's OPEN closed on the founder's own word (inputs#I2 dialog 6/7)
rationale: the report's Decisions (fct-gtooygvh) and inputs#I2 (fct-q2lwjk3d) record Epoch 7's four PROVISIONAL items, the board's read rule among them, CONFIRMED as his own word
@proposal n=2 detector=D-new-detail severity=warning file=authentication-authorization anchor=fct-65crhg3m fact=F3 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## Authentication & Authorization
change: add beside the `~/.viola/` access-control row: the ledger row `statusline-rate-limits` reads the account's status-line plan state, not the CLI, so a `viola verify` can read red (`23 pass  1 fail`) on an unchanged CLI 2.1.287 and switch no dialog answering on, the `budget-paused` refusal resting on the same row; owner 0.2.0 (finding F-06i)
sidecar: chunk 2026-11-06-version-done-check — the statusline-rate-limits row measured as reading the account's plan state (F3), owner 0.2.0
rationale: Spec claims disproved by measurement (fct-npz56o6m): the rig home's verify read the row red on unchanged 2.1.287 because the payload carries only `seven_day`; owner 0.2.0, F-06i (inputs#I7 item 1)
@proposal n=3 detector=D-cascade severity=warning file=input-validation anchor=fct-m6zdbeoz basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## Input Validation
change: after «relied on every build and stamped on 2.1.287 with the two `StatusLine` fixtures» add: «`statusline-rate-limits` reads the account's status-line plan state, not the CLI: on an unchanged 2.1.287 a verify read it red (`23 pass  1 fail`, only `seven_day` carried), the dev home's stamps unmoved; owner 0.2.0 (finding F-06i)»
sidecar: chunk 2026-11-06-version-done-check — the statusline-rate-limits row's CLI-behaviour claim qualified by the live measurement, owner 0.2.0
rationale: the report (fct-npz56o6m) disproves the row as a CLI behaviour relied on every build and names the budget-paused refusal resting on it; the dev home's stamps are not moved (fct-u65xlcax)
@proposal n=4 detector=D-cascade severity=warning file=data-protection anchor=fct-uxsbnoo5 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: ## Data Protection
change: replace «and acting on it is the founder's» with «and the founder recorded it as known with no action in 0.1.0 (2026-11-06), the pinned binary's re-hash staying the guard»
sidecar: chunk 2026-11-06-version-done-check — Measurement B recorded as known, no action in 0.1.0, on the founder's word (inputs#I2 dialog 7/7)
rationale: inputs#I2 dialog 7/7 and card row 7 (fct-q2lwjk3d): Measurement B known, no action in 0.1.0, the pin re-hash stays the guard
@proposal n=5 detector=D-cascade severity=warning file=body anchor=fct-p7rzex2i basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: ## Data Protection
change: **Handle inheritance:** channel handles never leak into the `claude` child: portable-pty closes every inherited descriptor in the child (`close_random_fds()`), witnessed live at chunk 2026-11-06-version-done-check by the child's fd table (fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's); the owed measurement and the portable-pty-psmux swap clause are dropped
sidecar: chunk 2026-11-06-version-done-check — handle inheritance re-worded off «recorded, not established» to the measured mechanism; the swap-candidate clause retired with the swap path
rationale: the live round read the child's /proc fd table with no viola or wrapper socket (fct-q2lwjk3d); inputs#I7 orders the mechanism wording; the swap trigger is retired (fct-uufjzm4k); expected amendment 6 (fct-ngwjb7xx)
@proposal n=6 detector=D-cascade severity=warning file=dependency-security anchor=fct-6nwwowx3 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Dependency Security
change: replace the entity: RUSTSEC-2026-0441 (`pin-utils 0.1.0` ← `nix 0.25.1` ← `portable-pty =0.8.1`) is closed by the move to `portable-pty =0.9.0` (chunk 2026-11-06-version-done-check): `cargo tree -i pin-utils` finds no package, `cargo deny check` reads `advisories ok`, CI `supply-chain` green on `849aad18`, and the `[advisories]` ignore list is empty, none widened
sidecar: chunk 2026-11-06-version-done-check — RUSTSEC-2026-0441's red reading and its CARRY discharged by portable-pty =0.9.0
rationale: Dependencies (fct-xc746owh) and Outcome (fct-bggboyf2): pin-utils gone, cargo deny advisories ok, ignore = []
@proposal n=7 detector=D-new-detail severity=warning file=dependency-security anchor=fct-6nwwowx3 fact=F1 dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Dependency Security
change: RUSTSEC-2026-0441 (pin-utils 0.1.0 via nix 0.25.1 via portable-pty 0.8.1) is closed by the move to portable-pty =0.9.0; `cargo tree -i pin-utils` finds no package — carried by proposal 6's rewrite of this entity
sidecar: chunk 2026-11-06-version-done-check — F1, the RUSTSEC-2026-0441 closure, placed at its entity
rationale: F1 is the closure the contradicting entity's rewrite states (fct-xc746owh)
@proposal n=8 detector=D-retired severity=warning file=dependency-security anchor=fct-zj6omeqt basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Dependency Security
change: replace the entity: `portable-pty` is pinned `=0.9.0` in `[workspace.dependencies]` (moved off `=0.8.1` at chunk 2026-11-06-version-done-check), its comment reading only that the workspace pins every third-party version exactly; `crates/viola-pty` compiles unchanged on it, `pty_backend()` stays `openpty`, viola-pty 33 of 33 green
sidecar: chunk 2026-11-06-version-done-check — the =0.8.1 pin and its Windows rationale retired; portable-pty =0.9.0
rationale: Retired (fct-uufjzm4k) names the =0.8.1 pin and its Windows rationale; Dependencies (fct-xc746owh) and Outcome (fct-b2ryvv3f) give =0.9.0, the comment and the green viola-pty run
@proposal n=9 detector=D-new-detail severity=warning file=dependency-security anchor=fct-zj6omeqt fact=F2 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Dependency Security
change: add beside the portable-pty pin: `portable-pty =0.9.0` brings `nix 0.28.0`, `serial2 0.2.39` and `cfg_aliases 0.1.1` into the Linux graph, with no advisory ignore needed (`pin-utils`, the `serial` family, `ioctl-rs`, `memoffset 0.6.5` and `termios` left it; `cargo deny check` bans and licenses ok)
sidecar: chunk 2026-11-06-version-done-check — F2, the graph delta of portable-pty =0.9.0
rationale: Dependencies (fct-xc746owh): `cargo update -p portable-pty` only, the listed adds and removals, cargo deny all four families ok
@proposal n=10 detector=D-retired severity=warning file=body anchor=fct-serp72ds basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Dependency Security
change: `[advisories]`: `unmaintained = "all"` (the default), `unsound = "all"`, `yanked = "deny"`, and `ignore = []`: no ignore is expected (RUSTSEC-2017-0008 left with the `serial` crate at the portable-pty `=0.9.0` move); any new ignore follows the Critical CVE response SLA
sidecar: chunk 2026-11-06-version-done-check — the RUSTSEC-2017-0008 ignore retired; the advisories ignore list empty
rationale: Retired (fct-uufjzm4k) names the RUSTSEC-2017-0008 ignore; Counts (fct-u65xlcax) `[advisories] ignore` entries 1 → 0
@proposal n=11 detector=D-retired severity=warning file=body anchor=fct-uydjlfhj basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Dependency Security
change: replace «(portable-pty `=0.8.1`)» with «(portable-pty `=0.9.0`)»
sidecar: chunk 2026-11-06-version-done-check — the pinning line's portable-pty pin moved to =0.9.0
rationale: Dependencies (fct-xc746owh): `portable-pty` `=0.8.1` → `=0.9.0` in `[workspace.dependencies]`; Retired names the `=0.8.1` pin
@proposal n=12 detector=D-retired severity=warning file=body anchor=fct-4r3p2tgk basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Dependency Security
change: Within 7 days of a RustSec/GHSA advisory against a crate and feature viola enables: bump it (the portable-pty-psmux swap path is retired, spent at the portable-pty =0.9.0 move)
sidecar: chunk 2026-11-06-version-done-check — the CVE SLA's portable-pty-psmux swap path retired
rationale: Retired (fct-uufjzm4k): the PTY swap trigger (portable-pty-psmux path) is spent, 0.9.0 dropping pin-utils upstream
@remark
Error Handling (fct-gnmmvxa5/fct-hzzqncjb, T1 and the thiserror census), the Anti-Patterns stamp rules (fct-syrndlhx/fct-7izg6s4l, F3's indirect reach) and other Epoch 7 PROVISIONAL marks sit in other groups' categories; D-security-input/auth/deps fire on nothing (no new input surface, no auth change, serial2/cfg_aliases pass cargo deny bans)
