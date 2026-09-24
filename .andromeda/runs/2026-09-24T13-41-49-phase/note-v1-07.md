2026-09-24 (2026-09-24-quality-gates, phase P5): not claimed. This chunk builds:
- per-OS coverage floors;
- the `gate` verdict per CI job, with missing artifact = breach;
- zero retries;
- a seeded fuzz replay that fails on an empty corpus, with its first target on `ViolaName::try_new`.

The acceptance still needs "hook deadlines hold on the worst sample" (no hook verb or hyperfine exists at HEAD) and "every security-named parser surface has seeded property tests" (0 of test-plan §6's seven parsers exist at HEAD). The claim waits for the chunks that land the hooks, perf gates and parsers.
