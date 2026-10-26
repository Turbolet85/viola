# arch · standing conventions

- S1 · Each new test runs in its own viola home under `target/e2e-home/viola-test-*/home`, against the fake agent, never the real `claude` in CI · occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:159) res-repository
- S2 · A new channel test endpoint takes a test-only name `viola-test-chan-<pid>-<label>` outside the `viola-<h12>` namespace · occupied-resources.md
- S3 · Every OS-specific branch compiles and is tested on its CI runner (windows-2025, macos-latest, ubuntu-latest) · body §Cross-cutting Patterns
  (.andromeda/architecture-detail/stack-and-technologies.md:31) stack
- S4 · A new log line goes through `obs_event!` to the role's home-level file, codes only, content only in instance detail files · body §Cross-cutting Patterns
  (.andromeda/architecture-detail/stack-and-technologies.md:21) stack
- S5 · A new error variant is a fixed-message thiserror variant of the owning crate's enum, no path or payload · conventions.md
  (.andromeda/architecture-detail/established-decisions.md:184) dec-error-handling
  (.andromeda/architecture-detail/stack-and-technologies.md:24) stack
- S6 · A new snapshot or frame field is additive with `v` kept, readers tolerant of unknown fields · standard-contracts.md
