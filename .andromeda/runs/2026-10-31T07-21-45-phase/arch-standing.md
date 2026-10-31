# arch · standing conventions

- S1 · A new test runs in its own viola home under `target/e2e-home`, passed by `--home`, sharing no endpoint, log or `budget.json` · established-decisions.md, [CI/CD] in the body; occupied-resources.md res-repository
- S2 · A new test or witness drives the fake agent, named after `--`; the real `claude` runs only in local live tests · Established Decisions, [CI/CD]
  (.andromeda/architecture-detail/stack-and-technologies.md:31) stack
- S3 · A new test endpoint stays outside the `viola-<h12>` namespace, named per test and never bound by `viola` · occupied-resources.md res-ipc-endpoints
- S4 · A new test-only file or directory is a recorded row of the filesystem or repository list, and `viola` never reads it · occupied-resources.md res-filesystem, res-repository
  (.andromeda/architecture-detail/occupied-resources.md:205) res-filesystem
- S5 · A new fixture holds synthetic text only and passes its schema and hygiene walk · occupied-resources.md res-repository
  (.andromeda/architecture-detail/occupied-resources.md:217) res-repository
- S6 · A new harness variable carries the `AGENT_RUN_` prefix and the binary never reads it; a test seam disables no control and widens no redaction · occupied-resources.md res-env-vars
- S7 · A new or changed log line stays codes only and conforms to the default-deny `schemas/diag-line.v1.json`, content going only to the detail schema · Cross-cutting Patterns, Diagnostic output channels; occupied-resources.md res-repository
- S8 · New code on `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O; only `viola-mcp` builds a runtime · Cross-cutting Patterns, Tokio containment
  (.andromeda/architecture-detail/established-decisions.md:8) dec-concurrency-backend-framework
  (.andromeda/architecture-detail/established-decisions.md:273) dec-module-boundaries
  (.andromeda/architecture-detail/stack-and-technologies.md:11) stack
- S9 · Every OS-specific branch of a new test or code path compiles and is tested on its CI runner; nothing shells out, and children spawn directly · Cross-cutting Patterns, Cross-platform discipline
- S10 · A new wire code is a kebab-case string and a new JSON field is snake_case; a new error enum variant goes in the owning crate's enum · Conventions, Naming patterns and Rust error types
- S11 · A new product reader of external bytes is bounded in its own statement and joins the contract census · cross-cutting-patterns.md crosscut-bounded-readers
