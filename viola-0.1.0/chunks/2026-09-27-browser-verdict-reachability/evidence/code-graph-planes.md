# Code-graph planes: rust + ts (plan step 14; claim v1-08)

Measured on this host, 2026-09-27 (implement run `2026-09-27T18-52-22-implement`, gate entries 32–34, all green).

- `python -X utf8 scripts/code-graph.py refresh` (entry 32, exit 0) printed:
  ```
  tree-refresh[rust]: 2087 nodes / 8923 edges - 12s
  tree-refresh[ts]: 7 nodes / 1 edges - 0s
  ```
  The ts plane is detected from `e2e-web/tsconfig.json`, which is untracked and not ignored, so
  `git ls-files --cached --others --exclude-standard` lists it. It is indexed by the host's `scip-typescript`.
- `test -f .andromeda/cache/rust/tree.db && test -f .andromeda/cache/ts/tree.db` (entry 33) exits 0: both plane databases are
  present.
- `health.py check --root . --stack rust --style agent-driven` (entry 34, exit 0), check 11:
  ```
  check 11 · ✓ · seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0
  ```
- The architecture sentence (`.andromeda/architecture.md:428`, §Infrastructure Patterns → Code-graph planes):
  > **ts:** `e2e-web/` only, from its first chunk, through a tracked `e2e-web/tsconfig.json` (noEmit, strict) and scip-typescript. No
  > tsconfig, package manifest or bundler config ever covers `crates/viola-ui/` (the page has no JS build step); a CI-runnable guard
  > lists any such file.

  The guard is gate entry 14. It lists no `tsconfig*.json` or `package.json` under `crates/` (exit 1, last line 0).

From here on, every code-graph query names its plane (two planes are detected).
