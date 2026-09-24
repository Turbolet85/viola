# Operator pass 2 — 2026-09-24-epoch-1-cleanup (superseded: base-missing)

Founder ruling (resume): amend + `--force-with-lease`.

| step | what | reading |
|---|---|---|
| amend | the leg fix (`run/mutants.rs` + its test), the gate trail JSON and the evidence files folded into the pre-CI commit, subject kept | `74dadd4` (`74dadd465f75186dd78a1535ac0560fe2df3ca6e`), parent `9df9e45`; tree clean |
| push | `git diff --quiet && git diff --cached --quiet && git push --force-with-lease=build/viola-0.1.0:bf87d7125cf0534f6b42883a394e24f7ae4d00d1 origin build/viola-0.1.0` | exit 0 · `+ bf87d71...74dadd4 build/viola-0.1.0 -> build/viola-0.1.0 (forced update)` |
| CI run | `gh run list --commit 74dadd4…` | run **36117447745** (push, `ci`) |
| mutation legs | job logs (windows job 108014755117) | both legs were handed `AGENT_RUN_CHUNK_BASE: bf87d7125cf0534f6b42883a394e24f7ae4d00d1` (`github.event.before`) and printed `{"v":1,"cmd":"run","ok":false,"reason":"base-missing","suites":[]}` · exit 1 within 25 s. `mutants (ubuntu-latest)` failure, `mutants (windows-2025)` failure, `mutants-verdict` failure |
| gates 17 / 18 | not run | the run is superseded by the founder's ruling; its mutation verdict is base-missing by construction |

**Cause:** after the force-push, `bf87d71` is reachable from no ref, so the `fetch-depth: 0` checkout does not contain it. `resolve_base`
then refuses it at `commit_exists` (`run/mutants.rs`), before any merge-base is taken. The session's recommendation assumed that
`chunk_diff`'s merge-base would fall back to 9df9e45. It never checked that the replaced commit would exist in the CI checkout. That
premise was false.

Other jobs at supersession: test ×3, release ×3, msrv, supply-chain, fuzz-replay and lint (macos) were success; lint (ubuntu, windows) was
still in progress. The watches were dropped.

**Remedy (founder ruling):** rewind the remote to `9df9e45`, then fast-forward it to `74dadd4`, so that `github.event.before` = 9df9e45 —
see `operator-pass-3.md`.
