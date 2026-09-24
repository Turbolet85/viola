2026-09-24 (2026-09-24-observability-gates, phase P5): not claimed. This chunk adds to `ci.yml`:
- fatal lint (`cargo fmt --all --check` and clippy `-D warnings`) on all three OSes;
- a missing-tool check that fails loudly (`jq --version`, the checksum-verified ripgrep install);
- homes kept until G2, G4 and the scan have read them;
- test-home uploads gated on a passing secret scan.

The acceptance still needs "the perf gates run in a job of their own". No perf gate exists at HEAD, and obs-plan §9 and test-plan disagree on the perf job (XF F1). So the claim waits for the chunk that lands hyperfine.
