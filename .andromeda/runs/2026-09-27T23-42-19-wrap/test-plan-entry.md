
## 2026-09-27-hooks-to-normalised-events — mutation target dir, readiness line 3 to :51, forced panic and perf to the "Hook perf gate" tail
**Section:** §1 (cli `boot` readiness; cli events order; hook Required test type) · §2 (Property and Performance rows) · §3 (`boot` readiness; `run` step 4 Mutation; `--fuzz-replay`) · §5 Module ↔ DB · §6 (Path 1; budget path step 2; Security sweep; Property suite) · §7 Fake agent · §9 Perf row · §10 Performance budgets
**Change:**
- `run --mutants` builds in its own target dir `target/mutants` (`CARGO_TARGET_DIR=<repo>/target/mutants` for the root pre-build, relative for cargo-mutants; was removed from the environment): a copied default `target/` kept the original `CARGO_BIN_EXE_*` paths, so root integration tests drove the unmutated binary.
- Harness `boot` readiness line 3 (`session-start{source:"hook"}`) joins with "Capability ledger and viola verify" (was "Hooks to normalised events"); Path 1 asserts records 1–3 + M6 through the sibling test; the fake agent fires SessionStart/default once after `start_receipts`.
- The forced-panic fail-open case moves to the "Hook perf gate" chunk with its trigger `FAKE_AGENT_HOOK_PANIC` (fake-agent only); the landed matrix is 11 cases, the panic path covered in-process by three unit tests.
- The hook perf rows, `--perf`, hyperfine and the CI perf job land with "Hook perf gate"; the gate stays provisionally 1.0 s, the hook's own `SPINE_DEADLINE` is 750 ms; interim bound = the matrix's `< 1.0 s`.
- Concurrent-append landed for the hook files (8 processes, 16 + 8 lines, 3 OSes) without a >4 KiB line; that half moves to the tail. Properties: hook stdin + prompt round-trip landed at 512 cases; `resets_at` → "Statusline pass-through". Fuzz `hook_stdin` (10 seeds) joined.
**Why:** the hooks chunk as built plus the P4 split and the P5 review. The `FAKE_AGENT_HOOK_PANIC` seam was ratified by the founder, live, on 2026-09-28 (relay: the Viola overseer); it needs its security-plan Decisions Log entry when it lands.
**Ref:** .andromeda/runs/2026-09-27T23-42-19-wrap/
