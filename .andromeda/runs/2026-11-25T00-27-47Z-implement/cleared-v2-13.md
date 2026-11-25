# Clearance — matrix#v2-13, block R5 (chunk 2026-11-24-viola-adopt)

Written 2026-11-25T00:31Z by run 2026-11-25T00-27-47Z-implement (gated mode).

- **The clearing artifact:** `inputs#I9` — the overseer's relay `relay-r5a-gated.md` (clock 2026-11-25T00:27:36Z),
  quoting the operator's word «все норм пару часов роли не играет делаем как положено» and the id of a plain claude
  session he started, made one exchange in and ended himself (59c8b045…); ONE firing of R5a, R5b not re-fired, a red
  final. Copied verbatim to `.andromeda/runs/2026-11-25T00-27-47Z-implement/relay-1.md`.
- **R5a, fired ONCE** at 2026-11-25T00:29:58Z through `gate.py run --chunk 2026-11-24-viola-adopt --operator 21`
  (R5_HOME=~/.viola, haiku / low), HEAD d3cd68f7, `target/debug/viola` sha256 50f5e60d… from entry 20 fired just
  before: ` 21 e2e         green · exit 0 · 2.47s · 114 B → 21.log · history: unmoved`, its verdict line
  `r5a: green resumed=yes lineage=yes pair=haiku/low turn=ended ended=yes start=ok`
  (log `$TMPDIR/andromeda-gate/2026-11-24-viola-adopt/implement-2026-11-25T00-27-47Z/21.log`). The instance's events
  log reads `session-start{cause:resume}` on the given id, the one prompt confirmed and `turn-ended`.
- **R5b**, read green on 2026-11-24T18:34:07Z (entry 22, ONCE, a plain session the operator left running):
  `r5b: green refused=session-live started=none ended=yes` — not re-fired; it stands on the record.
- Both lines stand in `.andromeda/cache/chunks/2026-11-24-viola-adopt/evidence/r5.md`, which the gated entry 14 reads
  (`grep -c '^r5[ab]: green '` → 2). The same file also records two earlier R5a reds (2026-11-24T18:31:16Z and
  18:57:47Z, ids whose transcripts the CLI's store did not hold), each final for its firing; this green is a third
  firing on a fresh grant of the operator, not a retry of either.
