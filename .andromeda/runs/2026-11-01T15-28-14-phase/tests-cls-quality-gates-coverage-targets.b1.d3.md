# Classified — tests · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.1 · no · standing S3 · `Command::env` per child, and `.env_clear()` re-adding `LLVM_PROFILE_FILE`
quality-gates-coverage-targets#2.0 · no · the chunk has no mutation gate: `run --mutants` and its gate are out of scope and judged at the epoch-boundary audit only
quality-gates-coverage-targets#3.1 · yes · the chunk's `check_instance` and server check sit in the timed hook path, and the scope names the hyperfine `max` rows under `SPINE_DEADLINE` that this `--perf` session measures
