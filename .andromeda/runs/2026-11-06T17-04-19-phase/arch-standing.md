# arch · standing conventions

- S1 · every OS-gated test names its reason in the census binary `tests/contract_os_gated_tests.rs`, and a gate predicate naming `windows` or `target_os = "macos"` reads as a fault · cross-cutting-patterns
- S2 · every new test runs against the fake agent named after `--` in its own viola home (`--home`, under `target/e2e-home/`), never sharing an endpoint, log or `budget.json`. The real `claude` runs only in local live tests, refused under CI. · Established Decisions
- S3 · a test-only environment marker sits outside the `VIOLA_*` namespace, is read by no `viola` build, and is registered in Occupied Resources → Environment variables · occupied-resources
- S4 · every test-only path or scratch dir (temp-dir reports, `target/` probe and run dirs) is registered in Occupied Resources (Filesystem or Repository), is gitignored when under `target/`, and is never read or written by `viola` · occupied-resources
  (architecture/occupied-resources fct-jv5gqhbl) res-filesystem
  (architecture/occupied-resources fct-mtiut4fw) res-repository
  (architecture/occupied-resources fct-xtqblyzi) res-repository
- S5 · a new log line goes to its role's codes-only process log under `diagnostics/`, in the obs-owned format fixed by `schemas/diag-line.v1.json`. Content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`. · Cross-cutting Patterns
  (architecture/stack-and-technologies fct-rzlme45f) stack
  (architecture/stack-and-technologies fct-zse22tqm) stack
- S6 · a test-only feature or binary is never in a release build: it is gated behind its feature (`fake-agent`, `test-support`) and refused by the release check · occupied-resources
