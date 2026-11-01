# Consolidation record — the 2026-11-01T02-46-44-wrap 0-pending wrap

On the operator's request (`inputs#I1` item 1), over the seven sidecars chunk `2026-10-31-exit-cause-code-catalogue` skipped. `--marker no-marker`. Before this wrap's own entries were appended:

- sidecars: architecture 0 re-worded · 2 pruned · 385976→383799
- sidecars: security-plan 0 re-worded · 3 pruned · 249801→246700
- sidecars: design-system 0 re-worded · 0 pruned · 76483→76483
- sidecars: layout-templates 0 re-worded · 0 pruned · 56268→56268
- sidecars: test-plan 0 re-worded · 1 pruned · 345739→344960
- sidecars: obs-plan 0 re-worded · 0 pruned · 191498→191498
- sidecars: a11y-plan 0 re-worded · 0 pruned · 88520→88520

**No rewriter ran.** Every sidecar's consolidation set read 0: off-form 0 and over-cap 0, every entry written on the form at its own wrap. The price of this consolidation is six tool-archived entries and no agent.

**Each pruned row was read against the entry that retires it** (consolidation.md step 6), and each named entry's claims are gone from the body:

- architecture: `2026-10-26-server-verification-before-any-frame — fork 1: …` and `… — read_snapshot's product callers re-measured at three`, both retired by `2026-10-26-home-and-code-bearing-file-integrity — the snapshot readers moved onto the replay: six product readers, read_snapshot none`
- security-plan: `2026-10-25-windows-endpoint-admission — the supply-chain red on RUSTSEC-2026-0386 and its owner`, retired by `2026-10-26-tokio-rustsec-2026-0386-lock-bump — the RUSTSEC-2026-0386 supply-chain red closed`
- security-plan: `2026-10-26-server-verification-before-any-frame — the snapshot readers are three, and stay on read_snapshot`, retired by `2026-10-26-home-and-code-bearing-file-integrity — the snapshot readers move onto read_snapshot_or_replay`
- security-plan: `2026-10-26-explicit-session-links — the collision check's code citation re-pointed`, retired by `2026-10-26-home-and-code-bearing-file-integrity — the server check's reference values and the collision check read through the replay`
- test-plan: `2026-10-13-t06-38-35-wrap — E3's explicit steps held on the Epoch 6 entry`, retired by `2026-10-26-explicit-session-links — E3 built: the explicit verbs' hold retired`

**Facts for the report line.** After the prune `summary` reads `supersedes: UNRESOLVED` architecture 2 · security-plan 3 · test-plan 1: each of those six `Supersedes` lines names an entry that now stands only in the archive, which is what a prune leaves, and nothing is ambiguous. `UNPARSED` 0 on all seven. Four sidecars stay over the 120 000 B whole-read bound (architecture, security-plan, test-plan, obs-plan): a prune-only consolidation cannot bring them under it, and phase reads them through the index.

**The store.** Consolidation does not go through the store: it writes only the sidecars and their archives, and `rec.py check` read `clean` straight after it, as before it.
