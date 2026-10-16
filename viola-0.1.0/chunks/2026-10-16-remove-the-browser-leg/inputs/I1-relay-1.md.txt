# P4 forks — 2026-10-16-remove-the-browser-leg (answered in the dialog, 2026-10-16T15:50Z)

## Question 1 — Scan classes
The secret scan still carries GUI-shaped classes that 0.1.0 can no longer produce. Which of them leave with the browser leg?
1. Remove all GUI forms (Recommended) — remove `token-query` (`?t=`), `gui-token` (the `ui/*.url` token) and the `viola_<port>=` cookie pair; keep the generic `cookie:` header and every CLAUDE* canary.
2. Remove only the two named — remove `token-query` and `gui-token`; keep the `cookie` class whole, `viola_<port>=` included.
3. Keep all, mark reserved — remove no scan class; the wrap amends security-plan and obs-plan to say the classes stand with no producer in 0.1.0.

Answer: "Remove all GUI forms (Recommended)"
Notes (verbatim): Overseer, technical (founder-delegated, under his CLI-only ruling of 2026-10-16): each removed class matches only the GUI's own token or cookie, which no 0.1.0 path writes; the generic cookie: header and every CLAUDE* canary stay.

## Question 2 — axum ban
`deny.toml` bans axum's `http2` feature ('the loopback GUI serves HTTP/1.1 only'), but axum is in neither lockfile. Do the ban and its deny-probe go?
1. Remove ban and probe (Recommended) — remove the `[[bans.features]]` axum block and the `axum-http2` row of `scripts/deny-probes.sh` together (13 → 12 probes); the wrap writes a dated ledger note on v1-23.
2. Keep both — leave the ban and its probe; the wrap rewords the ban's reason.

Answer: "Remove ban and probe (Recommended)"
Notes (verbatim): Overseer, technical (founder-delegated): the ban's stated reason left with the founder's ruling and axum is in no lockfile; ban and probe go together so every remaining ban stays proven live; v1-23's 13/13 gets a dated ledger note at the wrap.
