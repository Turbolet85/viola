# Classified — arch · occupied-resources.b2

## Answers
occupied-resources#23.1 · no · standing S1 · each new test runs in its own viola home under target/e2e-home/viola-test-*/home, against the fake agent, never the real claude in CI
occupied-resources#24.1 · no · the profraw-census per-run tallies under target/profraw-census/ are written only by scripts/profraw-census.sh, which no CI job, pre-push stage or harness command runs and no reached file touches
occupied-resources#25.1 · no · the mutation scratch beside the repository (its guard, wipe, Windows witness and NOCOW host fact) is written only by run --mutants, which no reached file touches
