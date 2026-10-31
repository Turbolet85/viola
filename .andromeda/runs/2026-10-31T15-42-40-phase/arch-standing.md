# arch · standing conventions

- S1 · Every new test runs in its own viola home through `--home`, sharing no endpoint, log or `budget.json`, and substitutes the fake agent by naming its executable after `--`, never the real CLI in CI · Established Decisions, [CI/CD]
  (.andromeda/architecture-detail/occupied-resources.md:91) res-env-vars
- S2 · A test home lives under the base of `target/e2e-home`, inside the obs gates and the secret scan, a home outside it being a named carve-out · .andromeda/architecture-detail/occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:91) res-env-vars
  (.andromeda/architecture-detail/occupied-resources.md:223) res-filesystem
  (.andromeda/architecture-detail/occupied-resources.md:244) res-repository
  (.andromeda/architecture-detail/occupied-resources.md:252) res-repository
- S3 · A new fake-agent scenario under `fixtures/fake-scripts/` holds synthetic text only and validates against `schemas/fake-script.v1.json` · .andromeda/architecture-detail/occupied-resources.md
- S4 · A process-log line is codes only, one complete object per single write, in its home-level role file, with content-bearing detail only in the instance detail files · Cross-cutting Patterns, Diagnostic output channels
- S5 · An OS-specific branch or test case compiles and is tested on its CI runner, and no case leans on a shell or a POSIX-only behaviour · Cross-cutting Patterns, Cross-platform discipline
- S6 · JSON field names are snake_case and wire enum values kebab-case in every viola format · Conventions, Naming patterns
