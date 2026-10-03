# Host snapshot — before (step 1)

Read-only, 2026-10-02T19:39:30Z, before any test run of this /implement. Counts only. The user temp dir is written
`%TEMP%` (the probe reads `TMP`, then `TEMP`). The probe is a scratchpad script, not committed. It writes the two name sets
to the scratchpad for the after-diff.

## `target/e2e-home/`
| class | count |
|---|---|
| entries | 4 804 |
| `viola-test-*` | 4 768 |
| `viola-session-*` | 33 |
| `probe-*` (answer · base · plain) | 3 |
| entries carrying `owner.json` | 0 |

## `%TEMP%/.tmp*` directories
| shape | count |
|---|---|
| total | 23 270 |
| git repo (`.git` present) | 13 497 |
| cargo project (`src` / `Cargo.toml`) | 1 173 |
| empty | 5 368 |
| other | 3 232 |

By mtime day: 05-20 5 · 06-01 1 · 08-24 2 · 08-28 2 · 08-29 2 · 09-01 1 · 09-10 2 · 09-12 2 · 09-24 3 133 · 09-25 3 672 ·
09-26 2 155 · 09-27 5 594 · 09-28 71 · 09-29 205 · 10-01 4 377 · 10-02 4 046. (10-02's dirs predate this run. They are
P5's M1 baselines and harness self-tests.)

## Watch reports
- `%TEMP%/viola-root-watch/`: 61 reports
- `%TEMP%/viola-pty-watch/`: 0

## Note on research's figures
research.md counted 23 269 dirs. This probe reads 23 270, one more dir dated 10-02. The shape split differs from research's
10-01-only split because this table covers every day.
