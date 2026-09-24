# Operator legs — 2026-09-24-workspace-tree-and-code-graph-planes

The plan's three `leg = 'operator'` entries were driven by hand once. This was the operator pass the overseer authorized at wrap P7 ("yes, two commits are fine; witness before the flip is the rule").

## 1. Push (clean-tree guard)
- `run`: `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0`
- Preceded by the operator pre-CI commit `0c2e0ccef741840d161a1e10651adbbca5f7b133` (`chore(2026-09-24-workspace-tree-and-code-graph-planes): operator pre-CI commit`), a whole-tree `git add -A`.
- exit 0: `3f385dd..0c2e0cc  build/viola-0.1.0 -> build/viola-0.1.0`. `git rev-list --count @{u}..HEAD` then read `0`.

## 2. Check-runs on the pushed sha
- `run`: `gh api repos/Turbolet85/viola/commits/$(git rev-parse HEAD)/check-runs --jq '[.check_runs[] | .conclusion] | unique | join(",")'`
- The sha read is `0c2e0ccef741840d161a1e10651adbbca5f7b133` (CI run 36029350628, all 15 check-runs completed).
- exit 0, printed `success`. The atom `last line success` holds, and so does `exit 0`.
- Per-run conclusions: all 15 are `success`: test ×3, lint ×3, release ×3, mutants ×2, mutants-verdict, msrv, fuzz-replay, supply-chain.

## 3. Release legs on the pushed sha
- `run`: `gh api repos/Turbolet85/viola/commits/$(git rev-parse HEAD)/check-runs --jq '[.check_runs[] | select(.name | startswith("release (")) | .conclusion] | join(",")'`
- exit 0, printed `success,success,success`. The atom `last line success,success,success` holds. This is v1-23 witness (2).

## In-job verdict lines (job logs via `gh api …/actions/jobs/{id}/logs`)
- `release (windows-2025|macos-latest|ubuntu-latest)`: `release-check probes: 3/3 refused, control clean` and `release-check: viola only` on each leg.
- `lint (windows-2025|macos-latest|ubuntu-latest)`: `orphans-check probes: 1/1 fired, control clean` and `orphans-check: 5/5 targets clean` on each leg.
- `mutants (ubuntu-latest)`: `no-rust-delta`.
- `supply-chain`: the `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` step ran (log line 302). The downloaded artifact `supply-chain` (run 36029350628) holds `deny-fuzz.json` with 1 `advisory-not-detected` warning plus the summary and 0 errors. `deny.json`, `deny-fuzz.json` and `zizmor.json` hold 0 absolute paths (pattern `(^|[^a-z])[A-Za-z]:\\|(^|[^a-z])[A-Za-z]:/[^/]|/home/runner|/Users/`, which is `https://`-safe). That re-confirms the ratified obs §8 item 6 admissibility on runner output.

## v1-23 witnesses
- (1) The inventory gate re-run after this wrap's architecture amendment read `45` / `0` (wrap light gate, entry 19).
- (2) The three release legs read `success` on the pushed sha (entry 3 above).

Both are read, so the verified flip is permitted (overseer condition).
