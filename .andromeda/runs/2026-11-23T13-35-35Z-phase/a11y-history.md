@history_item n=1 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — (8. Cognitive Accessibility, 2 rows) `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` each gained «paste write» in their list of bounds, every word kept. This is the current text of both picked time-bound rows. If the settled 5 s `HYPRCTL_DEADLINE` ever got a product caller, it would be a new bound needing a home, and both lists are now full at their bound.

@history_item n=2 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — `+ decision/start-verify-stop-grace-off-the-human`. Because both time-bound lists are full at their bound, the operator ruled that a new time bound lands as a row of its own beside them. This is the precedent for where a settled hyprctl read deadline would go if this chunk had to record it, instead of widening either picked row.

@history_item n=3 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` gained «MCP call deadline». The chunk also added `+ contract/instance-unreachable-detail/call-deadline` as a seventh closed cause. This is the precedent for a new deadline plus a new closed word together: each new bound viola holds is named in both rows, saying who it falls on.

@history_item n=4 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (8. Cognitive Accessibility) both time-bound rows widened from «the driver or viola's child» to also cover «viola's own start», adding the start's Claude config lock wait. This shows how the rows widen when a bound falls on a party they do not yet name. A test-only seam read (the tap, the fake hyprctl) falls on none of the named parties.

@history_item n=5 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` dropped the budget pause when its gate was removed whole. Precedent: the lists name only bounds the product holds. A bound with no product caller is not listed (the seam is a root dev-dependency only).

@history_item n=6 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — both time-bound rows dropped the 5 s status-line deadline and «the user's command». `− decision/statusline-output-passed-byte-for-byte` was removed as well. The lists track only bounds that stand in the product; a 5 s bound was retired here once its arm left.

@history_item n=7 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (§8 Timeout extensions) the `claude agents --json` read is a direct spawn with `take(MAX_FRAME + 1)` and a 5 s kill. A failed, oversize, malformed or timed-out read fails open, and the bound falls on viola's own child, never on a key or the human. This is the closest precedent for a bounded child read with a drain cap and a 5 s deadline, the same shape as the hyprctl read this chunk settles and witnesses («agents read» in both time-bound rows).

@history_item n=8 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (§8 Timeout extensions) the MCP `wait` cap `WAIT_CAP` (50 000 ms) was added as a driver-only bound: no key blocked and the wheel untouched. This is where «MCP wait cap» in the picked time-bound rows comes from, and the pattern for stating whom a bound falls on.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a peer's refusal screened to a code word; `unknown`'s detail a code word or nothing) `viola_core::is_code_word` takes 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first. A refusal that is no code word reads `unknown`, and a string detail that is no code word reads nothing. This is the origin of `decision/peer-replies-screened-to-code-words` and `contract/code-word/format`. `HyprlandError`'s closed words and `WORDS` must stay inside that format, and dropping reply text at the parse follows the same screening.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Linux-only platform line) the platform line became «Linux only — the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland». This is the source of `decision/linux-only`. The Hyprland seam and its fixtures are exactly those 0.2.0 window features, and they get no other-OS arm.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the a11y cases' CI home, the one ubuntu-latest leg) the cli and tui a11y cases now run in the `coverage` suite on `[ubuntu-latest]` alone, with `gate --require coverage,doctest` as the verdict. Any new seam or bounded-reader witness lands on that one leg.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the manual screen-reader pass, Orca on Linux alone) the tui pairing became Orca + foot on Hyprland, the founder's host's one terminal and the `xdg-terminal-exec` default. foot on Hyprland is the window R1's tap opens and kills; the manual pass is founder-owned and gates nothing.

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (the live takeover reading) the plan records one live reading on Linux, supplemental and never gating: a `wtype` key into a live session's window, on a compositor the test started itself. This is the origin of `decision/live-readings-never-gate`, and the precedent for R1 as a live compositor reading that informs but never gates CI.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (the takeover clause's second live reading) a second live reading, the founder's own key on the product binary, also recorded as supplemental and never gating. This confirms the rule R1 rests on: a live reading at the operator's host is evidence, never a CI gate.

@history_item n=15 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — (§10) where the CLI output-discipline invariants P4 and P6 live in crate tests, the epoch-boundary mutation audit (`run --mutants`) covers them; this replaced per-chunk and CI mutation gates. This is the origin of `decision/mutation-audit-covers-crate-invariants`, and it agrees with this phase's «no mutation tier».

@history_item n=16 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the tui passthrough boundary after a contained vt100 feed panic) the a11y-side record of the chunk that bounded inputs at every boundary, the lineage of `tests/contract_bounded_readers.rs` whose `hyprctl reply` row this chunk gives a drain witness. An a11y clause moved only where a boundary reached the human terminal. A seam read reaches none.

@history_item n=17 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0, the frontend stubs retired) the a11y plan now says viola has no web or graphical frontend, and §5, §6, §7 and the harness keys read N/A. The terminal fleet console's a11y scope belongs to «Fleet a11y verification». This is the source of the absences in the rows file. The Hyprland window work adds no graphical a11y surface.
