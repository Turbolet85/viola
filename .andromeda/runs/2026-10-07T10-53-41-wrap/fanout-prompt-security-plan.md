You are the drift-detector for security-plan in an attended doc-reconcile pass. Read:
- the chunk report: /home/turbolet/dev/projects/viola/viola-0.1.0/chunks/2026-10-07-live-rows-and-paste-shapes-on-the-dev-host/report.md
- your document: /home/turbolet/dev/projects/viola/.andromeda/security-plan.md
- your detectors (the drift-base entries scoped to security-plan):
- id: D-security-input
  doc: security-plan
  invariant: every new external-input surface (IPC / HTTP / deserialized struct) the chunk adds is validated per §Input Validation.
  check: agent-read — for each new external-input symbol in the report, confirm the validation §Input Validation mandates (for viola: names through `ViolaName::try_new`, `Read::take(MAX_FRAME)` on the reader, closed enums, `validate_paste_text` on paste and free-text answers - arch rejects garde/validator) is present; an unvalidated boundary is drift.
  severity: escalate
- id: D-security-auth
  doc: security-plan
  invariant: the auth / crypto / secret handling the chunk uses matches §Authentication + §Secret Management (no off-spec auth library or secret source).
  check: agent-read — if the report touches identity / session / token / keys, confirm the library + flow match §Authentication / §Secret Management; a mismatch is drift.
  severity: warning
- id: D-security-deps
  doc: security-plan
  invariant: every new dependency the chunk adds is allowed by §Dependency Security (not on the ban list).
  check: agent-read — check the report's new Dependencies against the §Dependency Security bans; a banned / unvetted dependency is drift.
  severity: escalate

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **security-plan's own prose** (security-plan's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; security-plan merely mentioning a
library in its prose is NOT a change. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep security-plan for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of security-plan — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
Decisions Log, which takes no new entry — never a distillation (CLAUDE.md, `.claude/rules/*`, `.claude/docs/*`: the cascade re-derives
them) and never a preserve-verbatim curation home (`## Session Additions`, `USER:session-learnings`,
`docs/session-learnings.md`: curation's channel, not yours).

Return ONLY YAML (or `proposals: []` if no drift):
proposals:
  - detector: D-{slug}
    severity: warning | escalate
    section: {the doc section to edit}
    change: {one line — what the body should now say}
    sidecar: {one line — changelog entry for the amendments sidecar}
    rationale: {why — cite the report}
    basis: {file:line the claim measured — optional; carrying it makes the orchestrator's re-derivation one read}
    dependent-of: {the primary proposal's detector — ONLY on a duplicate-occurrence proposal}
You PROPOSE only. Do not edit any file. The orchestrator validates and applies.
