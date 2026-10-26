# arch · standing conventions

- S1 · every new test runs in its own viola home (`--home`) against the fake agent named after `--`, never the real `claude` · .andromeda/architecture-detail/established-decisions.md
  (.andromeda/architecture-detail/occupied-resources.md:20) res-binaries
- S2 · every new test home lives under `target/e2e-home/viola-test-*/home`, removed only when its recorded owner process is gone · .andromeda/architecture-detail/occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:72) res-env-vars
  (.andromeda/architecture-detail/occupied-resources.md:144) res-filesystem
  (.andromeda/architecture-detail/occupied-resources.md:165) res-repository
- S3 · every new channel `params`, `--json` document and event line carries `v` (and `params` the `sender`), readers tolerant with no `deny_unknown_fields` · §Conventions (Interfaces and versioning)
  (.andromeda/architecture-detail/stack-and-technologies.md:17) stack
- S4 · every new log line is codes-only, one `write`, in its role's home-level `diagnostics/` file; content-bearing detail only in the instance's `diagnostics/detail-*.ndjson`; stdout reserved for the `--json` result · §Cross-cutting Patterns (Diagnostic output channels)
- S5 · every new error type or variant is a `thiserror` enum with fixed messages, anyhow only in the root bin, no path, pid, endpoint or upstream text in an external error · §Conventions (Rust error types)
  (.andromeda/architecture-detail/established-decisions.md:210) dec-error-handling
  (.andromeda/architecture-detail/stack-and-technologies.md:24) stack
- S6 · every OS-specific branch (the server check's macOS gap included) compiles and is tested on its CI runner · §Cross-cutting Patterns (Cross-platform discipline)
  (.andromeda/architecture-detail/stack-and-technologies.md:31) stack
- S7 · JSON field names snake_case and enum wire values kebab-case in every new payload · §Conventions (Naming patterns)
