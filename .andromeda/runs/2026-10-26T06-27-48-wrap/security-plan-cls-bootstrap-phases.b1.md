# Classified — security-plan · bootstrap-phases.b1

## Answers
bootstrap-phases#1.1 · no · its crate pins (sha2 =0.11.0 included) still hold on the changed Cargo.toml and Cargo.lock, as only the tokio pin and its lock entry moved
bootstrap-phases#2.1 · no · its canary scan, fuzz-upload rule and no-secret claim live in CI workflows and the harness, and the report says no workflow changed
