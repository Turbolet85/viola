# Implement journal — 2026-11-13-rows-proposal-rule-work

Each long run is journalled here before it is waited on (the operator's directive for this run).

- 2026-11-13T21:17Z · P2 firing 1 — `python -X utf8 {tools_dir}/gate.py run --chunk 2026-11-13-rows-proposal-rule-work --run-dir .andromeda/runs/2026-11-13T21-06-24-implement`, backgrounded; the whole block, legs excluded (31 non-leg entries, pre-push's own bound 7200 s). No deferral: Rust, the lockfile and tests all carry delta.
  Read 21:21Z: `entries 35 · green 31 · red 0 · recorded 1 · timeout 0 · not-run 3`.
- 2026-11-13T21:21Z · control 1 `link-replay` (entry 13) — `timeout -k 30 3300 python -X utf8 {tools_dir}/gate.py control --chunk 2026-11-13-rows-proposal-rule-work --controls-file .andromeda/runs/2026-11-13T21-06-24-implement/control-1.toml --run-dir .andromeda/runs/2026-11-13T21-06-24-implement --timeout 1500`, backgrounded; the clone under `~/.cache/andromeda-control/`, home disk 423G free.
  Read: `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (clone `~/.cache/andromeda-control/b6a89d02`).
- 2026-11-13T21:22Z · control 2 `strict-symlink` (entry 10) — the same form, `control-2.toml`, backgrounded.
  Read: `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (clone `~/.cache/andromeda-control/18908c2a`).
- 2026-11-13T21:22Z · control 3 `append-nofollow` (entry 10) — the same form, `control-3.toml`, backgrounded.
  Read: `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (clone `~/.cache/andromeda-control/342ec9d7`).
- 2026-11-13T21:23Z · control 4 `record-email` (entry 11) — the same form, `control-4.toml`, backgrounded.
  Read: `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (clone `~/.cache/andromeda-control/d37650e0`); step 2's own case not reached (fail-fast), evidence/control-4.txt.
- 2026-11-13T21:24Z · control 5 `hygiene-email` (entry 14) — the same form, `control-5.toml`, backgrounded.
  Read: `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (clone `~/.cache/andromeda-control/fae01c38`); status-line and script cases not reached (fail-fast).
- 2026-11-13T21:24Z · control 6 `redact-hook` (entry 11) — the same form, `control-6.toml`, backgrounded.
  Read: `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (clone `~/.cache/andromeda-control/b800aa4d`).
- 2026-11-13T21:28Z · implement ended green (outcome-2026-11-13-rows-proposal-rule-work.md); land tx 990.
## Operator pass (on the operator's word in the /implement call)
- 2026-11-13T21:29Z · pre-push before the pre-CI commit — `python -X utf8 {tools_dir}/gate.py run --chunk 2026-11-13-rows-proposal-rule-work --entry 32`, no run dir, backgrounded (bound 7200 s).
