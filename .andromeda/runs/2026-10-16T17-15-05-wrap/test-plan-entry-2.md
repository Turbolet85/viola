
## 2026-10-16-remove-the-browser-leg — Log format: the `ui` process, the four web events and the a11y rows leave
**Section:** §3 → Log format (Process logs, Required fields, Harness-side a11y rows removed, Constraints); §1 Test Scope Summary (Test harness requirements `run`, `logs`, Log format; Coverage triggers); §5 Module ↔ DB; §6 Non-path suites (Security sweep, Error sanitization and secret scan)
**Change:**
- Process logs write `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; `process` is `run|hook|mcp|cli`; the `corr` rules read `process-start`, `process-exit`, `panic` and `state-recovered`, `parse-rejected` (obs-plan D-03, D-05). Was also `ui-<port>`, the `ui` process and `http-request`, `liveness-changed`, `sse-opened`, `sse-closed`, marked reserved until the route entry "Remove the browser leg".
- The harness-side a11y rows (a Playwright fixture's `e2e-web/test-results/a11y/*.ndjson` against `e2e-web/schemas/a11y-row.v1.json`) are gone.
- No line may contain a `Cookie` header or a stripped `CLAUDE*` value (was also the GUI token and `?t=`); §1 and §6 restate it.
- §1: `run` item 3 reads none in 0.1.0; `logs` prints the per-process logs of `run` / `hook` / `mcp` / `cli`; no browser-suite runner. §5: the concurrent-append check names a colliding second `run` on the live `run-<name>` file. §6: the security sweep runs in nextest alone.
**Why:** This chunk removed the reserved web half of the closed obs vocabulary with the browser leg (founder ruling 2026-10-16, item 3). The key is kept equal to obs-plan §3's copy in the same pass.
**Kept:** §1's quotation of Founder Direction 3 («run/hook/mcp/ui»), a quote.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/
