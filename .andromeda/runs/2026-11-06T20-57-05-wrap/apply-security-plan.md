@apply_row n=1 ref_file=fanout-security-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g1.md ref_n=10 v=folded_into to_file=fanout-security-plan.md to_n=1
@apply_row n=3 ref_file=fanout-security-plan.md ref_n=2 v=applied edit=E2
@apply_row n=4 ref_file=fanout-security-plan--cascade-g1.md ref_n=11 v=folded_into to_file=fanout-security-plan.md to_n=2
@apply_row n=5 ref_file=fanout-security-plan.md ref_n=3 v=applied edit=E3
@apply_row n=6 ref_file=fanout-security-plan--cascade-g1.md ref_n=12 v=folded_into to_file=fanout-security-plan.md to_n=3
@apply_row n=7 ref_file=fanout-security-plan.md ref_n=4 v=applied edit=E4
@apply_row n=8 ref_file=fanout-security-plan--cascade-g1.md ref_n=5 v=folded_into to_file=fanout-security-plan.md to_n=4
@apply_row n=9 ref_file=fanout-security-plan--cascade-g1.md ref_n=1 v=refused
why: a hand removal · fct-pq4mk5dy (its whole `OPEN — the board's read rule stands on a delegate's answer · owner: the Epoch 7 boundary batch` line goes; an in-line change never empties a line and a whole rewrite is a fence's or a table row's)
@apply_row n=10 ref_file=fanout-security-plan--cascade-g1.md ref_n=3 v=applied edit=E5
@apply_row n=11 ref_file=fanout-security-plan--cascade-g1.md ref_n=4 v=applied edit=E6
@apply_row n=12 ref_file=fanout-security-plan--cascade-g1.md ref_n=6 v=applied edit=E7
@apply_row n=13 ref_file=fanout-security-plan--cascade-g1.md ref_n=7 v=folded_into to_file=fanout-security-plan--cascade-g1.md to_n=6
@apply_row n=14 ref_file=fanout-security-plan--cascade-g1.md ref_n=8 v=applied edit=E8
@apply_row n=15 ref_file=fanout-security-plan--cascade-g1.md ref_n=9 v=folded_into to_file=fanout-security-plan--cascade-g1.md to_n=8
@apply_row n=16 ref_file=fanout-security-plan--cascade-g2.md ref_n=1 v=applied edit=E9
@apply_row n=17 ref_file=fanout-security-plan--cascade-g2.md ref_n=2 v=applied edit=E10
@fact id=fct-serp72ds expect_rev=1
old: and `ignore = [{ id = "RUSTSEC-2017-0008", reason = "serial via portable-pty =0.8.1; serial ports never opened" }]`. This is the only expected ignore; re-check it whenever the portable-pty pin moves.
new: and `ignore = []`: no advisory is ignored; a new ignore follows the Critical CVE response SLA below, its `reason` naming the disabled feature.

@fact id=fct-uydjlfhj expect_rev=1
old: (portable-pty `=0.8.1`)
new: (portable-pty `=0.9.0`)

@fact id=fct-4r3p2tgk expect_rev=1
old: bump, or swap along arch's named path (portable-pty → portable-pty-psmux 0.9.7).
new: bump the crate.

@fact id=fct-p7rzex2i expect_rev=2
old: on Linux the property rests on close-on-exec descriptors (recorded, not established; its measurement is owed to the working-route entry "Version done-check"). Any portable-pty swap (portable-pty-psmux 0.9.7 is the named candidate) must keep this property.
new: portable-pty closes every inherited descriptor in the child, witnessed by E2 and the live fd table (fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's, no file under the home; as measured at `viola-0.1.0/chunks/2026-11-06-version-done-check/evidence/live-round.md`).

@fact id=fct-m6zdbeoz expect_rev=2
old: stamped on 2.1.287 with the two `StatusLine` fixtures.
new: stamped on 2.1.287 with the two `StatusLine` fixtures; `statusline-rate-limits` reads the account's status-line plan state, not the CLI: on an unchanged 2.1.287 a verify read it red (`23 pass  1 fail`, the payload carrying only `seven_day`, as measured at `viola-0.1.0/chunks/2026-11-06-version-done-check/evidence/live-round.md`); owner: 0.2.0 (finding F-06i).

@fact id=fct-uxsbnoo5 expect_rev=1
old: and acting on it is the founder's.
new: and it is recorded as known with no action in 0.1.0, the pinned binary's re-hash staying the guard.

@fact id=fct-6nwwowx3 expect_rev=1
old: `cargo deny check advisories` reads red on RUSTSEC-2026-0441 alone (`pin-utils 0.1.0` unmaintained, reached through `portable-pty =0.8.1` → `nix 0.25.1` from `viola-pty`), in CI's `supply-chain` job and locally (as measured at viola-0.1.0/chunks/2026-11-05-linux-only-code-and-ci/report.md). Its owner is the CARRY on "Version done-check", whose remedy moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`; no ignore is widened, the `[advisories]` ignore list keeping its one entry.
new: `cargo deny check` reads `advisories ok, bans ok, licenses ok, sources ok` with the `[advisories]` ignore list empty: RUSTSEC-2026-0441 (`pin-utils 0.1.0`) is closed by `portable-pty =0.9.0`, `cargo tree -i pin-utils` finding no package, and CI's `supply-chain` job is green on `849aad18` (as measured at viola-0.1.0/chunks/2026-11-06-version-done-check/report.md).

@fact id=fct-zj6omeqt expect_rev=1
old: The `portable-pty =0.8.1` pin carries no Windows reason: portable-pty 0.9.0's Windows garbage-read bug reaches no code viola builds, and "Version done-check" measures moving off the pin.
new: `portable-pty` is pinned `=0.9.0` in `[workspace.dependencies]`, its comment reading only that the workspace pins every third-party version exactly; it brings `nix 0.28.0`, `serial2 0.2.39` and `cfg_aliases 0.1.1` into the Linux graph with no advisory ignore needed, `crates/viola-pty` compiles on it unchanged and `pty_backend()` stays `openpty` (viola-pty 33 of 33 green, as measured at viola-0.1.0/chunks/2026-11-06-version-done-check/report.md).

@fact id=fct-gnmmvxa5 expect_rev=1
old: Open divergences from the error-enum convention: `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`; owner: a CARRY on the route entry Version done-check.
new: Divergences from the error-enum convention, ratified as named exceptions: `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`.

@fact id=fct-hzzqncjb expect_rev=1
old: `McpError` · `ServeError`;
new: `McpError` · `ServeError` (the test-only `crates/viola-e2e` harness's derive outside the count);

@pattern id=sec-01
fixed: This is the only expected ignore
@pattern id=sec-02
regex: portable-pty `?=0\.8\.1
@pattern id=sec-03
fixed: swap along arch's named path
@pattern id=sec-04
fixed: recorded, not established
@pattern id=sec-05
fixed: acting on it is the founder's
@pattern id=sec-06
fixed: reads red on RUSTSEC-2026-0441
@pattern id=sec-07
fixed: pin carries no Windows reason
@pattern id=sec-08
fixed: Open divergences from the error-enum convention
