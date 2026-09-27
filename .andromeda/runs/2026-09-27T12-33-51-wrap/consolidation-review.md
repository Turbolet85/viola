# Consolidation review — DROPPED spans read against the originals (directive 3)

`sidecar.py consolidate --dry-run` per sidecar (listings: `consolidate/{doc}-dry.txt`), then every dropped backticked
span of every re-worded entry printed with its original sentence (scratchpad `dropped.py`: the spans in `{row}.md`
absent from `{row}.out.md`, each with ±150 chars of the original) and read.

| doc | rows re-worded | rows with drops | verdict |
|---|---|---|---|
| a11y-plan | 4 | 3 | all drops are sweep patterns, line refs and leaf paths (the `**Sweep:**` blocks) |
| obs-plan | 12 | 11 | sweep patterns / controls, leaf paths, a raw reading (`s:/` inside `https://`), report symbol pointers (`secret_scan::scan`, `classify`) in provenance |
| security-plan | 12 | 10 | sweep patterns and leaf paths; row 8's `8e25ca7` / CI-run witness and `ReOpenFile` are evidence pointers in Why |
| architecture | 17 | 17 | sweep patterns / controls and leaf paths; row 11's `e2e-web/test-results/a11y/` + `lint/` survive in the output as "`e2e-web/test-results/` with `a11y/` and `lint/`" |
| test-plan | 18 | 16 | sweep patterns / controls, leaf paths, commit and CI-run ids, witness readings (`9 caught`), the `git ls-remote` measurement |

Result: no dropped span is a rule, identifier, count, threshold or retired claim a body carries or a later chunk is
bound by — every one is sweep record, evidence locator, raw reading or provenance, which `sidecar-contract.md` §Entry
form moves out (reachable by `Ref` and in the archive). 0 outputs rewritten. The ADDED rows (obs 5 `tracing::{event,…}`,
security 11 `cfg(feature="fake-agent")`, arch 2 `dtolnay/rust-toolchain@stable`, arch 9 `--all-targets -- -D warnings`,
test 11 `outcomes.json`, test 16 `concurrency:`), each checked against its original: obs 5, security 11 and arch 9 are
literal substrings of the original; test 11 (the original's `outcomes\.json` sweep row, "all the local `run --mutants`
verdict") and test 16 (the original's "declined-concurrency entry") re-spell what the original states; arch 2 names the
retired claim "was `dtolnay/rust-toolchain@stable`", and that is measured fact: `git show 270ef29:.andromeda/architecture.md`
line 433 reads "Setup steps: `dtolnay/rust-toolchain@stable` with rustfmt and clippy, then `Swatinem/rust-cache@v2.9.2`",
and `git show b0236ca -- .andromeda/architecture.md` (feat of chunk 2026-09-24-three-os-ci-headless-harness-skeleton)
removes it at diff lines 48 (the §Stack CI/CD row's `dtolnay/rust-toolchain`) and 155 (the Setup-steps line); read by this
wrap at the overseer's pointer. The output is kept as written.
