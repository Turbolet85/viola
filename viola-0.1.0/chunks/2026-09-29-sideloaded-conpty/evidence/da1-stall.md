# Product finding — the sideloaded ConPTY holds a non-terminal run's child ~3 s for a DA1 answer

**Surfaced to the wrap** (overseer's word, 2026-09-29): a product finding, not fixed in the product. viola stays
silent; the test-side piped driver (`tests/run_cli.rs` `Piped`) answers DA1 the way a terminal would.
**Owner:** the route entry that first runs `viola run` headless (no terminal answering on its stdio).

## What happens
At start the sideloaded `OpenConsole.exe` (Microsoft.Windows.Console.ConPTY 1.24.260710001) writes a spawn
preamble that carries a DA1 query (`ESC[c`), and the child's start waits for the answer. A real terminal answers
it; a `viola run` on pipes (or any host that does not answer) waits ~3 s. The inbox conhost sends no DA1 query.

## Measured (this host, Windows 11 Pro 10.0.26200, 2026-09-29; `viola run builder -- viola-fake-agent` on pipes,
fresh home, time from spawn of `viola` to the fake agent's `start` receipt)

| viola | host's first output on viola's stdout | start receipt |
|---|---|---|
| pre-chunk `fb78ddc` (inbox conhost) | `ESC[?9001h ESC[?1004h` | 0.48 s |
| this chunk (sideloaded `OpenConsole.exe`), nothing answers | `ESC[1t ESC[c ESC[?1004h ESC[?9001h` | 3.54 s |
| this chunk, the probe answers `ESC[?1;0c` on stdin | same | 0.54 s |

At exit the sideloaded host writes `ESC[?1004l ESC[?9001l`. The inbox host's exit bytes on the same run:
`ESC[?25l ESC[?9001l ESC[?1004l ESC[2J ESC[m ESC[H`, a window-title OSC naming the child's program, `ESC[?25h`.

## Effect on the suite before the test-side answer
The six `run_cli` tests on the piped driver went red at their 7 s start wait under a parallel run (the other 11
green); alone one of them took 4.3 s against 1.2 s pre-chunk. With the driver answering DA1 (and the home seed
below), `run_cli` in parallel: 17/17 in 3 of 3 rounds.

## The a11y preamble note
The plan's implementation notes asked for the sideloaded spawn preamble to be recorded for the a11y amendment if it
differs from the inbox one: it does (the table above). The zero-viola-literals oracle is unaffected: none of these
bytes is viola's.
