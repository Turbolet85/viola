# arch · standing conventions

- S1 · Each new test runs in its own viola home through `--home`, under `target/e2e-home`, so no two tests share an endpoint or log and the obs gates and secret scan cover it · Established Decisions, [CI/CD], and occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:91) res-env-vars
  (.andromeda/architecture-detail/occupied-resources.md:231) res-filesystem
  (.andromeda/architecture-detail/occupied-resources.md:252) res-repository
  (.andromeda/architecture-detail/occupied-resources.md:260) res-repository
- S2 · Each new test runs against the fake agent; the real `claude` runs only in local live tests, never in CI · Established Decisions, [CI/CD]
  (.andromeda/architecture-detail/occupied-resources.md:91) res-env-vars
- S3 · Every OS-specific branch compiles and is tested on its CI runner · Cross-cutting Patterns, Cross-platform discipline
- S4 · Each new log line is codes only in its role file under `diagnostics/`, content-bearing detail only in the instance's detail file, one line per `write`, inside the default-deny `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json` · Occupied Resources, Filesystem and Repository
  (.andromeda/architecture-detail/stack-and-technologies.md:21) stack
- S5 · A new test-only endpoint takes a name outside the `viola-<h12>` namespace, unique per test, in the `viola-test-chan-<pid>-<label>` form, and a new test-only file or directory holds its row in Occupied Resources · Occupied Resources, IPC endpoints and Filesystem
