# arch · standing conventions

- S1 · every new test runs in its own viola home through `--home`, under `target/e2e-home` · Established Decisions ([CI/CD]) / .andromeda/architecture-detail/occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:150) res-filesystem
- S2 · every OS-specific branch compiles and is tested on its own CI runner · Cross-cutting Patterns (Cross-platform discipline)
- S3 · new code on the `run`, `hook`, `send` or channel paths uses std threads and blocking I/O; only `viola-mcp` builds a runtime · Cross-cutting Patterns (Tokio containment)
  (.andromeda/architecture-detail/established-decisions.md:8) dec-concurrency-backend-framework
  (.andromeda/architecture-detail/established-decisions.md:219) dec-module-boundaries
  (.andromeda/architecture-detail/stack-and-technologies.md:11) stack
- S4 · a new error type is one thiserror enum per crate with fixed messages, anyhow only in the root bin · Conventions (Rust error types) / .andromeda/architecture-detail/conventions.md
  (.andromeda/architecture-detail/established-decisions.md:213) dec-error-handling
  (.andromeda/architecture-detail/stack-and-technologies.md:24) stack
- S5 · a new test-only path or resource is recorded in the Occupied Resources filesystem or repository list · .andromeda/architecture-detail/occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:144) res-filesystem
