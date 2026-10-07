You are the drift-detector for layout-templates in an attended doc-reconcile pass. Read:
- the chunk report: /home/turbolet/dev/projects/viola/viola-0.1.0/chunks/2026-10-07-live-rows-and-paste-shapes-on-the-dev-host/report.md
- your document: /home/turbolet/dev/projects/viola/.andromeda/layout-templates.md
- your detectors (the drift-base entries scoped to layout-templates):
- id: D-layout-surface
  doc: layout-templates
  invariant: a new user-facing surface / region the chunk adds is described in §Primary Surfaces / the wireframes.
  check: agent-read — if the report adds a UI surface or region, confirm it maps to a §Wireframe entry; an undocumented surface is drift.
  severity: warning

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **layout-templates's own prose** (layout-templates's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; layout-templates merely mentioning a
library in its prose is NOT a change. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep layout-templates for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of layout-templates — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
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
