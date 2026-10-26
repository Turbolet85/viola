# Classified — test-plan · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.1 · no · the session moves only the tokio pin in Cargo.toml and Cargo.lock, and this coverage text (LLVM_PROFILE_FILE propagation, env_clear, the profraw census) names no tokio version and no coverage rule or threshold that moved
quality-gates-coverage-targets#2.0 · no · the session's diff names no .rs path (Cargo.toml and Cargo.lock only), so it is the no-rust-delta case this text already states, and no mutation run, verdict rule or measured score moved
quality-gates-coverage-targets#3.1 · no · the session changes no hook, hyperfine row, perf-session step or SPINE_DEADLINE surface (only the tokio lock pin moved), and the perf procedure names no tokio version or Cargo.lock entry
