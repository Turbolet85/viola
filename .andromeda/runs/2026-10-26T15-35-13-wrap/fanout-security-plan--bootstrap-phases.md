proposals:
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 15
    section: '## Bootstrap phases (derive for route / setup-project)'
    change: 'In the `viola-channel` client item, the clause "the snapshot read after the strict-modes check is owed to the route entry "Home and code-bearing file integrity"" becomes: "the snapshot read after the strict-modes check is owed to the route entry "Home and code-bearing file integrity" for every client except CLI `viola link` / `viola unlink`, which run `viola_state::strict::check_instance` on the home and the driven instance before any role file or log opens and before any snapshot value is read, then the liveness pre-check and the server check (a refused check exit 21, no line written; the founder''s word, chunk 2026-10-26-explicit-session-links)"; the rest of the item stands.'
    sidecar: 'security-plan Bootstrap phases (auth-scaffolding-baseline, viola-channel client): the owed strict-modes snapshot read now names its first two exceptions, CLI viola link / viola unlink, which check the driven instance before any log or snapshot read (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report (viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41-46, :265-267) records both new verbs running check_instance before client::start and before any snapshot value is read, the founder''s word inputs#I3 ratifying it with no dated gap taken, and its expected amendments (:176-179) name the CLI frame verbs under the instance check, not a dated gap; the file states the strict-modes snapshot read as owed for every client.'
    basis: .andromeda/security-plan-detail/bootstrap-phases.md:15
note: D-security-input holds (the link / unlink params go through ViolaName::try_new as .andromeda/security-plan.md:328 already wires, report :18-23, :241-244); D-security-deps holds (report :81, Dependencies none); no Retired item stands in the category file.
