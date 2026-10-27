# arch · standing conventions

- S1 · each new test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json` · Established Decisions [CI/CD] (established-decisions.md)
- S2 · a new test or harness home lives under `target/e2e-home/viola-test-*/home` or `viola-session-*/home`, removed only when its recorded owner process is gone · occupied-resources.md (Repository)
  (.andromeda/architecture-detail/occupied-resources.md:204) res-filesystem
  (.andromeda/architecture-detail/occupied-resources.md:225) res-repository
- S3 · a new test-only or measurement scratch path, target dir or evidence file is registered in Occupied Resources and gitignored, and `viola` never reads it · occupied-resources.md (Repository, Filesystem)
  (.andromeda/architecture-detail/occupied-resources.md:198) res-filesystem
  (.andromeda/architecture-detail/occupied-resources.md:239) res-repository
- S4 · a new log line goes only through `obs_event!` to the role's codes-only process log under `diagnostics/`, content only in the instance detail file; `run` writes nothing to the terminal while the child runs · Cross-cutting Patterns (Diagnostic output channels)
  (.andromeda/architecture-detail/occupied-resources.md:114) res-filesystem
  (.andromeda/architecture-detail/stack-and-technologies.md:17) stack
  (.andromeda/architecture-detail/stack-and-technologies.md:21) stack
- S5 · every OS-specific branch compiles and is tested on its CI runner; paths written into plugin files use forward slashes; children are spawned directly, never through a shell · Cross-cutting Patterns (Cross-platform discipline)
- S6 · a test-only binary or feature never reaches a release build, and `scripts/release-check.sh` refuses an artifact built with `test-support` or `fake-agent` · occupied-resources.md (Binary; Workspace crates)
  (.andromeda/architecture-detail/occupied-resources.md:20) res-binaries
  (.andromeda/architecture-detail/occupied-resources.md:100) res-env-vars
- S7 · a new error type follows the per-crate `<Crate>Error` thiserror enum with fixed messages · Conventions (Rust error types)
  (.andromeda/architecture-detail/established-decisions.md:235) dec-error-handling
  (.andromeda/architecture-detail/stack-and-technologies.md:24) stack
