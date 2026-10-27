# arch · standing conventions

- S1 · Each new test runs in its own viola home (`--home`) under `target/e2e-home/viola-test-*/home`, never sharing an endpoint, log or `budget.json` · occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:210) res-filesystem
  (.andromeda/architecture-detail/occupied-resources.md:231) res-repository
- S2 · A new test-only endpoint is named outside the `viola-<h12>` namespace, unique per test, never bound by `viola` · occupied-resources.md
- S3 · A new fuzz target or seed lives in the separate `fuzz/` workspace with synthetic seeds kept byte-exact by `.gitattributes`, and the stated count carries its `ls` rule · occupied-resources.md
- S4 · A new log line is a codes-only process-log line under `diagnostics/` in the obs-owned line schema, content only in the instance detail files · §Cross-cutting Patterns (Diagnostic output channels)
- S5 · A new fixture is scrubbed and hygiene-walked against its tolerant schema, an unclean recording refused whole naming the file and the check code · occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:216) res-repository
- S6 · A test-only binary, feature or seam never reaches a release build · occupied-resources.md
  (.andromeda/architecture-detail/occupied-resources.md:20) res-binaries
  (.andromeda/architecture-detail/occupied-resources.md:100) res-env-vars
- S7 · Every OS-specific branch compiles and is tested on its own CI runner against the fake agent · §Cross-cutting Patterns (Cross-platform discipline)
  (.andromeda/architecture-detail/occupied-resources.md:167) res-filesystem
- S8 · Product crates write no `print!` / `eprintln!` / `dbg!`; stdout stays reserved · §Cross-cutting Patterns (Diagnostic output channels)
