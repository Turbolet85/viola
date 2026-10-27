# Red 1: Path 1's phases, measured on the dev host (plan step 10) — stop rule S2

Read 2026-10-27 on the Linux dev host (host `QUIET` 02:16:46Z..02:21:46Z), run dir
`.andromeda/runs/2026-10-27T02-08-46-implement/`. Firings: `bash scripts/agent-run.sh run --e2e --filter
'binary(path1_run_start)'`, sequential, each under `timeout -k 10 900` (nextest's own kill line inside it;
`.config/nextest.toml` untouched). A temporary per-step record in `tests/path1_run_start.rs` appended each step's
`std::time::Instant` span to `target/path1-phases.ndjson` (a measurement only, no wait, no verdict); it is removed —
the file is HEAD's bytes again (`git diff --stat -- tests/path1_run_start.rs` empty).

## The readings

Two sets of five firings, every one `ok:true`, `nextest-e2e passed 1, failed 0`. The first set (02:21:58Z..02:22:46Z)
kept only its last firing's JUnit time (the loop copied the wrong file; `6.092 s`), so a second set
(02:23:35Z..02:24:07Z) was fired with each firing's JUnit kept. Steps in ms; "body" is the test function's own span,
from its first statement to its last; "JUnit" is nextest's time for the case, the `stamped_home` fixture included.

| set·firing | 1 boot | 2 snapshot+board | 3 already-live | 4 squatted | 5a stop | 5b tamper | 5c refused start | 6a restart | 6b re-copy compare | 6d stop | body | JUnit |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1·1 | 259 | 3 | 87 | 225 | 2 | 84 | 237 | 240 | 84 | 12 | 1232 | — |
| 1·2 | 256 | 3 | 89 | 243 | 2 | 88 | 238 | 236 | 84 | 11 | 1251 | — |
| 1·3 | 261 | 4 | 89 | 225 | 2 | 85 | 235 | 264 | 96 | 12 | 1273 | — |
| 1·4 | 272 | 4 | 90 | 230 | 3 | 88 | 235 | 233 | 83 | 12 | 1248 | — |
| 1·5 | 247 | 3 | 84 | 220 | 2 | 81 | 233 | 230 | 85 | 11 | 1197 | 6.092 s |
| 2·1 | 254 | 4 | 86 | 215 | 2 | 80 | 228 | 230 | 82 | 12 | 1193 | 6.086 s |
| 2·2 | 254 | 3 | 84 | 215 | 2 | 79 | 230 | 232 | 81 | 11 | 1191 | 6.072 s |
| 2·3 | 250 | 3 | 84 | 220 | 2 | 78 | 225 | 230 | 81 | 12 | 1185 | 6.063 s |
| 2·4 | 247 | 3 | 84 | 216 | 2 | 78 | 225 | 231 | 82 | 11 | 1179 | 6.068 s |
| 2·5 | 252 | 4 | 86 | 218 | 2 | 81 | 224 | 229 | 80 | 11 | 1187 | 6.067 s |

## What they name

- **The `stamped_home` fixture** — `viola verify` against the fake agent before the test body — is JUnit less body:
  about 4.88 s of 6.07 s (80 %).
- **The body's four product starts** — 1 boot, 4 the squatted start, 5c the refused start, 6a the restart, each
  215–270 ms — are 0.93 s of the body's 1.19 s (15 % of the case).
- **The test's own I/O** — 5b's whole-copy read and write to flip one byte, 6b's two whole-file reads compared —
  is 0.16 s (2.7 % of the case).

Beside the CI readings (research.md, CI reds): green `ci#40403389629` 13.468 s; red `ci#40425866096` 20.089 s, its
kept home showing the start's first line ≈ 11.5 s after the case began (the fixture), the version probe 3.3 s after
that (the product start's own work before the child), and the run inside step 5 at the kill.

## Stop rule S2

The work that dominates is the `stamped_home` verify and the product starts — Path 1's asserted contract (a
stamped home; test-plan §6 Path 1 steps 1–6). The only removable work in the test file, 5b's and 6b's whole-file
I/O, is 2.7 % of the case on this host: removing it would not bring the case off two-thirds of its kill line.
So: stopped. The readings go to the operator; no bound moved, no step cut, `.config/nextest.toml` and
`tests/path1_run_start.rs` unchanged. The "after" readings were not taken (nothing was changed to read after).
