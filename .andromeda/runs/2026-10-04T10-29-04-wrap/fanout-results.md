# Fan-out results — 2026-10-04-wait-and-last

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt sent verbatim. Returns parsed as YAML; no
HTML entities met (`entities=0` on every return); stripping removed only trailing `#` commentary lines (kept below as
the verdict notes). No raw twin is warranted (no `proposals: []` return was changed by stripping beyond commentary, no
parse or probe failure).

## Verdicts
- architecture — 10 proposals
- security-plan — 7 proposals (D-security-deps: no drift — Dependencies "none")
- design-system — 0 (`proposals: []`; note: the cli pattern 3 `session-end` line is content, not token drift)
- layout-templates — 4 proposals
- test-plan — 1 proposal
- obs-plan — 2 proposals (D-obs-stack, D-obs-pii: no drift)
- a11y-plan — 0 (`proposals: []`; §1 / §4 P6 / §5 / §7 / §8 already cover the linear, escaped wait/last output)

## Proposals and dispositions

### architecture
- A1 · D-arch-resources · §Conventions exit `1` (`:139`) — the `cli` role's internal error printed once by the `main`
  catch site for every verb but `run` / `hook` / a flag; no dispatch arm prints; the wait/last exit-1 causes.
  → **apply** (check 1: Accurate this-chunk addition, routine; check 6: disproved claim 2).
- A2 · dependent-of A1 · §Infrastructure Patterns → Build system (key file `build-system.md:4`) — the `src/human.rs`
  caller list gains send / wait / last + the catch site; the internal error is no longer verify's alone. → **apply**.
- A3 · D-arch-resources · §Occupied Resources → Filesystem `diagnostics/` (`:398`) — `cli-<name>.ndjson` gains the
  send / wait / last producers. → **apply, re-derived**: the proposal's "written only when `VIOLA_NAME` holds a valid
  name" is NOT the report's fact (send / wait / last init obs with the target `<name>` argument — report Symbols,
  `client::start`); applied from the report.
- A4 · D-arch-resources · §Standard Contracts → Channel methods `wait` (`:276`) — `from?`, u64 bounds / `-32602`, start
  offset, line-starts rule, `WAIT_WAKE`, the deadline, the `WaitFeed` wake under the append's lock. → **apply** (also
  the Expected amendment "§Standard Contracts — the wake signal").
- A5 · D-arch-resources · §Standard Contracts → Channel methods `last` (`:277`) — `from?`, null semantics, the
  in-lock newest turn, the rebuild before `server.serve`. → **apply** (Expected amendment "the `last` rebuild").
- A6 · D-arch-resources · §Standard Contracts → the `from` sentence (`:274`) — the envelope types no `from`; each
  method answers `-32602`. → **apply**.
- A7 · D-arch-resources · §Infrastructure Patterns → Project directory structure (key file) — `cmd/client.rs`,
  `run/wait.rs`, `human.rs` callers. → **apply** (Expected amendment "directory tree").
- A8 · D-arch-decisions · §Established Decisions [Database / State Store] (`:50`) — as landed, the first events reader
  tolerates but does not heal a torn last line; healing owed to `working-route.md:85`. → **apply** (check 1:
  Sequencing deferral + Accurate this-chunk addition; the locked decision is qualified as landed, not reversed).
- A9 · dependent-of A8 · Project directory structure `viola-state/` comment — the landed events reader named, healing
  qualified. → **apply**.
- A10 · D-arch-decisions · §Established Decisions [Message Broker / IPC] (`:52`) — as landed, an `after`-less `wait`
  starts at the log end and no dialog can be pending yet; the pending-dialog return lands with `:78`. → **apply**
  (Sequencing deferral; the `:78` pin is this wrap's P5 directive (1)).

### security-plan
- S1 · D-security-auth · §Authentication & Authorization → IPC client-side server verification (`:205`) — the fifth
  dated gap: CLI `viola wait` / `viola last` frames after `send`'s liveness-only pre-check; the served-as line.
  → **escalate → resolved** (check 1: "Boundary widening" + "Boundary widening — what ratifies it", escalate). The
  ratification is on disk and qualifies under that rule: plan.md §Ruled at P4 F1, the founder's answer given live at
  this chunk's P4 AFTER this widening was shown, relayed by the Viola overseer; the operator's wrap directive (2)
  confirms the record's form. Applied; the sidecar records it as the founder's, the relay named.
- S2 · dependent-of S1 · §Authentication & Authorization → `~/.viola/` access control (`:207`) → **apply** (same group).
- S3 · dependent-of S1 · §Input Validation → CLI arguments / stdin (`:230`) → **apply** (same group; the name + u64 flag
  parsing is report fact).
- S4 · dependent-of S1 · §Input Validation → Own state files on read (`:235`), the strict-modes exception clause →
  **apply** (same group).
- S5 · dependent-of S1 · §Security Anti-Patterns → Authentication NEVER-write rule (`:524`) → **apply** (same group).
- S6 · D-security-input · §Input Validation → Channel frames (`:224`) — the `wait` / `last` params clause; the envelope
  types no method param. → **apply** (routine; check 6: disproved claim 1 — the row's `-32602` now holds, recorded).
- S7 · D-security-input · §Input Validation → Own state files on read (`:235`), line-cap clause — the
  `events::read_from` reader. → **apply** (Expected amendment "`MAX_FRAME` consumers gain the events reader").

### layout-templates
- L1 · D-layout-surface · §Output structure — `viola wait` / `viola last` (`:427-441`) — the `session-end` /
  non-dialog line, `dialog unknown`, the exit-21 pair, `--json` one document, `error: internal error`. → **apply**.
- L2 · dependent-of L1 · Primary content block 2 (`:542`) — `error: internal error` for every `cli` verb from the
  catch site. → **apply**.
- L3 · dependent-of L1 · Primary content block 2 hint line (`:543-544`) — the wait/last exit-21 hint. → **apply**.
- L4 · dependent-of L1 · §Surface: cli signature placement (`:353`) — wait/last share the `unable` column. → **apply**.

### test-plan
- T1 · D-tests-coverage · §6 Path 3 Surfaces involved (`:757`) — "As landed": cli + ipc-internal on three OSes; MCP
  `:102`, `/api/sessions` `:129`, page STATUS `:139`, dialog kinds `:78` owed. → **apply** (Expected amendment).

### obs-plan
- O1 · D-obs-instrumentation · §4 Scenario `wait` / `last` (`:658-659`) — `after` / `timeout_ms` / `outcome` scoped to
  `wait`; `outcome` derived in `viola-channel`; client-only `instance-unreachable`. → **apply** (Expected amendment).
- O2 · D-obs-instrumentation · §4 Scenario `wait` / `last` (`:660`) — exit 21 for wait/last is `instance-dead` at
  WARN only until Epoch 6. → **apply** (Expected amendment "send's exit 21 at WARN" is already §6 `:844`'s truth — the
  code moved to it; no body change needed for `send`).

### Raised by the orchestrator
- R1 · check 5 · design-system cli Component Patterns 3 (`:783`) — the `session-end  <name>  <time>  cursor <n>` result
  line (and the non-dialog form), `dialog unknown`. No detector proposed it (design-system's only detector is tokens).
  → **apply** (routine: Accurate this-chunk addition; report Symbols `src/human.rs`).

## Validate — the six checks
1. Playbook — 23 routine · 5 escalate (S1–S5, one group) resolved on the founder's on-disk ratification · 0 unmatched.
2. Cross-contradiction — none (no two proposals edit one section in opposing directions; S4 and S7 edit two clauses
   of one row in the same direction).
3. Intent-consistency — the report's deviations are justified; the one scope-record line (`frame.rs`, in-intent,
   serves step 11) holds: the acceptance `channel_wait_last_invalid_params` names `-32602` for `last {"from":7}`.
4. Absence-needs-evidence — no proposal claims an absence; the cascade sweep carries the caught-ALL claims.
5. Expected amendments — every entry matched: security (S1–S7), arch exit 1 (A1), tree (A7), Standard Contracts
   (A4, A5), obs §4 (O1, O2), test §6 (T1), layout output + block 2 (L1–L3), design pattern 3 (R1, raised).
6. Disproved claims — 1: S6 (impl now holds the row) · 2: A1 · 3: plan-only, no master states it (`grep -c
   "overflow means no deadline"` over the seven masters → 0) — all DISPOSED.
