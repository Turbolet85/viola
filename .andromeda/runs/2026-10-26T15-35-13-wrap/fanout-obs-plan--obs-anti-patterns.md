proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-anti-patterns.md
    near: 8
    section: '## 11. Obs Anti-Patterns (NEVER do these) › ### Logs'
    change: 'In the print-macro ban''s carve-out, the refusers writing through `src/human.rs` gain the two new verbs and the census moves 14 → 15: read "`run`''s and `revive`''s pre-spawn refusals, `revive --list`''s, `verify`''s, `list`''s, `plugin install`''s and `viola link` / `viola unlink`''s strict-modes refusals (the fixed instance-state `unable` / `hint:` pair, from the one call site in `src/cmd/link.rs` both verbs share) write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 · `link` 1 = 15, by the `human::refuse(` count per file under `src/`, chunk 2026-10-26-explicit-session-links)", the rest of the line unchanged.'
    sidecar: 'obs-anti-patterns.md §11 Logs: the print-macro carve-out''s human::refuse callers gain link / unlink (census 14 → 15, link 1), chunk 2026-10-26-explicit-session-links.'
    rationale: 'Report Changes › Symbols / APIs: link_call step 1 refuses a failed check_instance through human::refuse with human::instance_state_pair(driven), exit 21; Remaining-caller facts and Counts › refuse: the human::refuse census moved 14 → 15 (verify 5 · run 5 · list 2 · revive 1 · plugin 1 · link 1). This file''s caller census (run 5 · verify 5 · list 2 · revive 1 · plugin install 1) and its refuser list now miss the new site; the ban itself holds (the result lines are one write_all each in src/human/verbs.rs, no print macro).'
    basis: .andromeda/obs-plan-detail/obs-anti-patterns.md:8
note: 'The same census in the body (.andromeda/obs-plan.md:311, run 5 · verify 5 · list 2 · revive 1 · plugin install 1) is the report''s listed refuse-pattern stale row and is the body detector''s to amend; this file''s line 8 holds a second wording of it the pattern did not count.'
