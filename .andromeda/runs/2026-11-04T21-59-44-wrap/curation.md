# Curation — 2026-11-04-linux-and-macos-parity

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "When a test's OS reader fails only on a CI runner the host cannot reproduce, first make the failure carry what the reader returned and push that alone; change or re-gate the reader only on that reading" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): corrected in place — "A sidecar entry payload lands through `rec.py amend`, never appended to a file" (was: needs a leading blank line before it is appended, `splice.py append --blank-before`)
  Filters: 0 dup · 4 task-specific / homed elsewhere · 0 conflict · 0 deferred (cap) · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "a runner-only OS-reader red is read by one measurement commit before the reader changes"
  CLAUDE.md size: 131/200 · T1 3.4 KB, 0 over 600 B

## Applied
- T2 `.claude/rules/testing.md` `## Session Additions`, new entry 2026-11-04 (landed tx 153).
  Proof: ci#41912866163 read the widened hangup case red on `macos-latest` with no reading of `ps`; the measurement commit
  `28bbddab` made the failure carry `ps`'s exit and both streams; ci#41915621814 then read `ps: sigignore: keyword not found`,
  and the reader's fate (Linux-only) was ruled on that reading (`evidence/operator-pass.md`). Signals: verified by
  measurement +0.4, specific technical detail +0.2, reached no other durable home +0.2 (no master, route annotation, rule
  or contract carries the practice) = 0.8.
- T3 `.claude/docs/session-learnings.md` — a CORRECTION (cap-exempt), the entry of 2026-10-26 edited in place with a
  `[corrected 2026-11-04: …]` tag (landed tx 152), on the operator's directive `inputs#I5` item 6.
  Proof: the installed `splice.py --help` lists `summary` · `index` · `delete` · `section` only; this wrap landed its 19
  sidecar entries through `rec.py amend` (tx 139–144), each read back by its id.

## Decisions & corrections — each entry's disposition
- The operator's implement directive (live sessions 0, tool-fault stop, the eight records commits, the operator pass) — not a learning: a chunk's directive, its facts in the report.
- The ruling at the macOS red, "Gate back to Linux" — not a learning: one case's gate, homed in test-plan §6 (amended this wrap) and `.claude/rules/verification-harness.md:41` (leaf, re-derived).
- Measured: macOS `ps` has no `sigignore` keyword — rejected, Filter 4 (0.6 exactly: measured +0.4, specific +0.2; homed in test-plan §6 and the verification-harness leaf this wrap, so neither conditional signal applies).
- Measured: a runner-only red read by one measurement commit — applied, T2 testing.md (above).
- The plan's census prediction 50 → 52 + 7, taken by a narrower rule than the plan states — recurrence-despite-learning: CLAUDE.md Tier 1 "A count written into a master carries the rule that produced it … Extended 2026-10-26: a count a plan or research states is taken by its master's rule too" → handoff.
- Research's 44 + 4 beside a list of five — the same recurrence (a count stated beside its list, never re-counted) → handoff, with the entry above.
- macOS check-only clippy before each push — not a learning: the standing testing.md 2026-10-31 entry applied as written (no recurrence).
- Guard refusals (a tool's output piped into `head` / `sed`, a heredoc with a file target) — not a learning: each re-issued once in the form the guard passes; the host rule already states both.
- The P5 card's word "yes, apply" and its three CARRYs onto «Version done-check» — not a learning: P5's route edit.
- `v1-53` stays planned with its dated note — not a learning: the matrix's.
- `.claude/docs/session-learnings.md:46` names `splice.py append` / `--blank-before` — corrected in place (above).
- The wrap's own: the report's `kill27` site sweep matched digits only and missed the key file's «twenty-six» — recurrence-despite-learning: `.claude/docs/session-learnings.md` "A count sweep misses a count written in words with a noun phrase after it" (2026-10-31) → handoff.
- The wrap's own: the Q1 answer's correction (the Linux-only gate was the overseer's technical answer, not a founder ruling; `inputs#I6`) — not a learning: a record correction, written into the sidecar entry's Why.
- The wrap's own: `leaves --apply --only` keeps the last of repeated flags, and its glob met the apply's own receipt — not a learning: tool telemetry (the friction family), never curated.
