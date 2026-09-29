# design extract

## Relevance
partial — the chunk renders no web-spa surface and adds no human CLI verb; design applies only where it touches `viola hook` (no human surface), the fake agent (test-only, no design surface) and, conditionally, a `viola verify` ledger-row line if P3/P4 adds a probe row for the cross-session-message tag.

## Constraints
- `viola hook` has no human design surface: it writes nothing to stderr and emits no colour, glyph or SGR (per design-system §Surface: cli → Platform-Specific Notes, and the Colour decision order, rule 3). The harness-origin classifier change (scope item 5) and the matcher evaluation (scope item 1) must add no human-facing output on the hook path.
- `viola run` prints nothing while the child runs, so the fake agent's screen and receipt belong to the child (per design-system §Surface: cli → Colour decision order, rule 2, and Component Patterns 5 `run`). The receipt-`size` resize-oracle work (scope item 2) must add no wrapper-side terminal byte.
- If the chunk adds a capability-ledger row (the cross-session-tag witness), its `viola verify` output line follows the step-counter pattern `[NN/MM] <row-id> <claim>  pass|fail`, and the summary line `stamped <ver>  <n> pass  <m> fail` changes only in its counts. `fail` stays uncoloured (per design-system §Surface: cli → Component Patterns 5 `verify`). Whether a row is added at all is for P3/P4 to decide.
- The WHEEL field reads the words `human` / `driver`. A harness-classified prompt must leave that word unchanged, and no new wheel vocabulary is introduced (per design-system §Anti-Patterns → Rejected Defaults, "Robot and person icons…", and §Surface: cli → Component Patterns 4). The wheel itself is out of scope; this constraint limits only how far the `origin` value reaches.
- Human CLI output stays ASCII-only, with no emoji or `✓`/`✗` prefixes, in any new verify line or test-facing human text (per design-system §Surface: cli → Width, and §Per-Surface Bans → cli).

## Patterns to follow
- The `verify` step counter's static appended stdout lines, one per ledger row, with the row count growing as owning chunks land rows (per design-system §Surface: cli → Component Patterns 5).
- The colour-decision short-circuit: `hook` / `run` / `--json` resolve to no colour before any depth probe (per design-system §Surface: cli → Colour decision order).

## Anti-patterns to avoid
- Printing upstream text (a prompt, a tag literal, a fixture payload) in any human error, hint or verify line (per design-system §Per-Surface Bans → cli, "NEVER print upstream text, paths, pids…").
- Colouring anything other than `DIALOG` (amber) or `stale` rows (dim), and any spinner or live-redrawing progress in verify output (per design-system §Per-Surface Bans → cli).

## Contract bindings
- design ↔ security: the no-upstream-text rule for human output restates the security-plan NEVER-log floor (per design-system §Per-Surface Bans → cli). The tag literal and the fixture payload bytes are upstream content.
- design ↔ architecture: `hook` never writing stderr is shared with architecture's hook fail-open invariant (per design-system §Surface: cli → Platform-Specific Notes).

## Acceptance criteria contributions
- (design) The hook path, including the new harness-origin classification and matcher evaluation, writes zero bytes to stderr and emits no SGR or glyph (per design-system §Surface: cli → Platform-Specific Notes / Colour decision order).
- (design) If a ledger row is added, `viola verify` prints it as one `[NN/MM] … pass|fail` stdout line in the existing step-counter form, with an uncoloured `fail` and an updated `stamped <ver>  <n> pass  <m> fail` summary (per design-system §Surface: cli → Component Patterns 5).
- (design) No new human output line contains a non-ASCII glyph, emoji or upstream text (per design-system §Surface: cli → Width / §Per-Surface Bans → cli).
