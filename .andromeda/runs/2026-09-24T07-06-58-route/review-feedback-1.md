# Review feedback 1 — Phase 4 (user; overseer edits founder-delegated)

1. **Deferred item resolved — Unix before Web UI.** The three Unix chunks (Linux live confirmation, Unix endpoint and
   home hardening, Linux and macOS parity) move ahead of the Web UI into a new Epoch 7 "Cross-OS completion"; the last
   epoch becomes "Web UI, polish & ship" (9 web chunks + Version done-check). Deviation from the intent's (f)→(g)
   order, taken on the measured test dependency (ubuntu-only Playwright boots harness `viola run` sessions; TP §3, §9).
2. **Chunk 1 rewritten as the skeleton** — it overlapped chunks 2 (fake agent) and 4 (JSON logs); those two deepen it.
   The overseer's wording ran 32 words, so it is trimmed to the 25-word limit, keeping every element: SHA-pinned
   actions, pinned 1.96-floor toolchain, nextest, the five agent-run commands, a minimal fake agent, one JSON line per
   role, the mutation gate.
3. **Linux live confirmation marked founder-attended** — it needs the founder's Linux laptop; it stays first in its
   epoch, and its hint says it is reorderable and never blocks the CI-run Unix chunks (which run against the fake agent).
   This relaxes intent F-53's "before the Unix-specific chunk lands" to a default position, on founder-delegated say-so.

Overseer note: coverage map checked, 53/53 claimed.
