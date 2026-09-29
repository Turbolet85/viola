# Operator pass — 2026-09-29-sideloaded-conpty

Driven in the implement session on the overseer's word ("Then the operator pass, pushing through the guard with those
records cited, and the CI read decides").

## Entry 23 — `gate.py hygiene`
- First firing: `hygiene: refused 1 files — P1 0 · P2 0 · P3 1` — `.andromeda/runs/2026-09-29T08-18-26-phase/baseline/control/net.rs`
  (rust plane source: the one-line known-positive control of entry 9, `use std::net::TcpStream;`).
- Disposition, per the operator's convention for phase-run controls: renamed to `net.rs.txt`, bytes unchanged. The plan's
  entry-9 `baseline` note still names `net.rs` (plan.md is immutable here).
- Second firing: `hygiene: clean — read 36 (runs 32 · evidence 4) · trails 12 not read · binary 0 not read by P1`.

## Entries 24-29
- Pre-CI commit `2d83718`; entry 24 pushed it. Entry 25 (`ci.py conclusion`): **red**, ci#36563179341, 14/15, the
  one red `test (windows-2025)`: coverage 974/975, `tui_host_resize_in_the_pump_start_window_reaches_the_child`
  (262 ms; `fb78ddc` was green there, so this chunk's; cause and fix in `da1-stall.md`). The H2 loop step did not run.
- Fix `224efc4`, pushed. `ci.py conclusion`: **green**, ci#36563868040, checks 15/15, wall 1301 s.
- Entry 26 (`gh run view 36563868040 --log --job <test (windows-2025)> | grep 'h2-loop:'`): exit 0, both atoms
  held — `h2-loop: sideload iterations 200 · losses 0`, `h2-loop: inbox iterations 200 · losses 14`
  (`h2-with-without.md`).
- Removal commit `chore(2026-09-29-sideloaded-conpty): remove the H2 measurement`: the `h2-measure` feature, the two
  race tests and their helper, the CI loop step. Entry 27 (`! grep -rnE 'h2-measure|h2_race|h2-loop' crates .github
  Cargo.toml`): exit 0, no output. `ci.yml` against `fb78ddc`: the `ConPTY vendor verification` step only.

- Removal `8f643f2`; entry 28 pushed it. Entry 29 (`ci.py conclusion --sha HEAD --wait 1800`): **green**,
  ci#36566391084, checks 15/15, wall 320 s — the acceptance run.

## The pre-push guard, pushed through
Entry 18 (`agent-run.sh pre-push`) is red on its windows-tests stage only; the red is recorded
`red — not this chunk's` with its two-sided `fb78ddc` basis in `entry-6-not-this-chunk.md`, the acceptance leg is CI.
