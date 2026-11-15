# Cascade dispositions — 2026-11-15-live-round-on-the-trimmed-product (wrap 2026-11-15T11-38-30)

## Step 2 — the sweep
- Pattern files: none. Each `rows-land` dry run printed `patterns: none` — architecture (4 sets, prose alone), test-plan
  (1 set, prose alone), obs-plan (3 sets, prose alone), a11y-plan (1 state set, prose alone), security-plan (adds only,
  no set or retire). No main pattern file is written, so `cascade.py sweep` is skipped: «no pattern in any file». Step 3's
  leaves read each changed row (whole mode, this chunk closing Epoch 2).
- By-hand search instead, for the amended claim's mechanism — the start order and the trust keys — over all seven
  masters: `rec.py find --doc {each} --regex 'then session-start|session-start is its second|first two (event )?lines|lines 1.2|second and last start record'`
  and `--regex 'start\W{0,4},? then|before session-start|statusLine.command|trust dialog|folder-trust'`. Hits, each
  read in its row:
  - architecture `decision/wheel-starts-with-driver` · amended (TX 1507).
  - architecture `decision/no-typing-into-trust-dialogs` · no change — viola itself still types into no trust dialog;
    the rig's keys are the security-plan row's, a rig being no viola.
  - security-plan `decision/live-round-rig-keys-own-pty` · added (TX 1508).
  - security-plan `decision/plugin-install-live-round-rig` · no change — the 2026-10-18 precedent, a different round.
  - test-plan `contract/event-log/start-wheel-record` · amended (TX 1509).
  - test-plan `contract/run-start-events/order` («wheel with cause start, then the agent's session-start») · no change —
    an order, not an adjacency, and true of the harness's fake-agent start, where no trust key passes; consistent with
    obs-plan's amended row («before»).
  - test-plan `contract/boot-readiness/events` («Event lines 1 and 2 are a wheel line with cause start, then
    session-start») · no change — the harness boot runs a fresh instance on the fake agent: no reused name, no trust
    key; entry 27 read the same two lines on a live trusted-cwd start.
  - obs-plan `contract/run-start-events/order` and `decision/product-order-from-events-log` · amended (TX 1510).
- Curation homes and the judgment bases: the same patterns over `CLAUDE.md`'s session learnings and each rule's
  `## Session Additions` are read in step 3's whole-mode comparison and P3.

## Leaves
Mode: WHOLE for every group — `cascade.py leaves --all`, its `epoch ·` row «Epoch 2 — The budget gate and the status line
leave» · markerless 0 · gated 0 · closes. Set: 15 groups · 27 leaves · unmapped 2. Counts from the apply's `total` line:
`blocks 17 · applied 7 · reshaped 1 · held 9`. No hand write.

- **a11y-plan** (a11y-summary, rules/a11y) — 1 finding, applied: `rules/a11y.md:22` the passthrough role joins the no-SGR list (a reader adding colour to a passthrough role would act against `decision/no-sgr-off-terminal`).
- **architecture-1** (conventions, gotchas, services/viola-agent-claude) — 0 findings.
- **architecture-2** (services: channel, core, mcp, pty, state) — 1, applied: `services/viola-core.md:6` drops «PROVISIONAL» from the dialog-hold bounds (this chunk's `hold: stands`, TX 1507).
- **architecture-3** (services/viola, stack, workflow) — 1, applied: `services/viola.md:3` drops the retired [Budget Governor] pointer (`absence/usage-budget-gate`).
- **architecture-4** (rules/events) — 1, declined: `events.md:20` the retired `statusline` hook word — a detail of its master's row; no reader acts against the source from the map as it stands.
- **architecture+security-plan** (rules/api) — 2, declined: `api.md:17` the one-time bump exception is history of two removals, the rule as stated is the standing one; `api.md:29` the `--budget` usage exit is a detail of `contract/cli-verbs/release`.
- **masters** (CLAUDE.md) — 0 findings.
- **architecture+design-system+layout-templates** (rules/frontend) — 0.
- **architecture+test-plan+obs-plan** (docs/commands) — 2, declined: `commands.md:24` and `:56` details that stay in their masters (the retired-row stamp reading; the rmcp advisory ignore, stated whole in rules/security.md).
- **security-plan** (security-summary, rules/security) — 4: applied `security-summary.md:73#2` (this chunk's `decision/live-session-screens-kept-local`, TX 1508) and `rules/security.md:14` (`viola mcp`'s start holds the home to the write rule — a reader changing that start would otherwise skip the check); declined `security-summary.md:73#1` — its anchor «any other screen no key» stands in the plugin-install round's residual, a different round, and the replacement would state this chunk's input-box keys of that round (false there); the summary's closing «no standing permission: another such reading needs its own answer» already matches the new row's per-chunk word; declined `rules/security.md:12` — the role-file and SessionEnd details stay in their master row.
- **security-plan+obs-plan** (rules/ci) — 0.
- **design-system+layout-templates** (design-summary) — 0.
- **test-plan** (tests-summary, rules/testing, rules/verification-harness) — 3: applied `tests-summary.md:25` (stale «the second and last start record», TX 1509's amended row); declined `tests-summary.md:30` and `rules/testing.md:24` — details that stay in their masters.
- **test-plan+obs-plan** (rules/observability) — 2: applied `observability.md:20` (stale: a refused role-file open makes `viola_obs_init` return the error and install no subscriber, not a sink writer); declined `observability.md:27` — reshaped by the tool (a backslash escape the line does not carry) and a detail of its master row.
- **obs-plan** (obs-summary) — 0.
- **Unmapped:** `.claude/rules/host-linux.md` — the host's own leaf, no source states it, no comparer's; `.claude/rules/testing-src.md` — a pointer leaf into `rules/testing.md` sections, no text of a master repeated, left as it stands.
