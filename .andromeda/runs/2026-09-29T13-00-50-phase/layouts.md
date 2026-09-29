# layouts extract

## Relevance
partial — the chunk builds no surface and changes no layout (its core is the test-only contract suite, the fake agent and the fixture schema check), but two edges reach layout mandates: item 5's harness-origin fix changes which tape line a cross-session prompt renders as on the web-spa tower tape, and any capability-ledger row a witness adds lands as a `viola verify` step line on the cli surface.

## Constraints
- A `prompt-submitted` classified `harness` renders on the tower tape as a `prompt · harness` line, entirely in the tertiary text treatment; a `human` one renders as an ordinary prompt line (per layout-templates §Surface: web-spa / Component — Primary content block 3: the tower tape, "Per-kind colour"). Item 5 moves cross-session-tagged prompts from the second class to the first; whether the tape's per-kind mapping already keys on `origin` is research's question.
- Only a driver-origin `prompt-submitted` fills the readback box to `read` and is folded into the send line rather than getting its own tape line (per layout-templates §Surface: web-spa, the readback paragraph at :49, and §Component — Hero / signature section). The fix must not reclassify a cross-session prompt as `driver`: `harness` is the target class, so it neither fills a readback nor folds into a send.
- `viola verify` prints exactly one stdout step line per ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, with the `stamped <ver>  <n> pass  <m> fail` summary as the last stdout line; `MM` grows as owning chunks land their rows (per layout-templates §Surface: cli / Output structure — `viola verify`). Binding only if P3/P4 adds a ledger row (e.g. a cross-session-tag witness or a matcher probe).
- `viola verify --record <DIR>` adds no human line; output is plain ASCII, no colour, each step line appended once with no progress bar or redraw (per layout-templates §Surface: cli / Output structure — `viola verify`, and §Decisions Log motion note for cli). Any fixture re-recording a witness needs keeps that output shape.
- A failing ledger row exits 1 with the stamp still written, and refusals print as `unable:`/`hint:` pairs (per layout-templates §Surface: cli / Output structure — `viola verify` and §Component — Primary content block 2: refusal lines and the `unable` column).

## Patterns to follow
- The tape's per-kind treatment is driven from the event's domain row (kind + origin), not from text inspection in the page (per layout-templates §Component — Primary content block 3: the tower tape, "from the domain rows"): the classifier fix belongs upstream in the `origin` field, and the page follows unchanged.
- Ledger-ordered step lines with a growing `MM` denominator (per layout-templates §Output structure — `viola verify`): a new row appends at its ledger position; existing row ids and words stay stable.

## Anti-patterns to avoid
- Adding any human-readable stdout line to `viola verify --record`, or a progress/redraw pattern to verify, for the contract suite's convenience (per layout-templates §Output structure — `viola verify`).
- Special-casing the cross-session tag in the web page (text-matching the prompt to grey it) instead of carrying it as `origin: harness` from the classifier (per layout-templates §Component — Primary content block 3: the tower tape).

## Contract bindings
- layouts ↔ events/architecture: the tape's `prompt · harness` treatment and the readback fold both key on `prompt-submitted`'s `origin` value (per layout-templates §Component — Primary content block 3 and §Component — Hero / signature section); the `origin` enum itself is owned by architecture §Standard Contracts (event line), not by this plan.
- layouts ↔ capability ledger (tests/verify): each ledger row is one `viola verify` step line (per layout-templates §Output structure — `viola verify`); a row added here changes the step count every verify-output assertion reads.

## Acceptance criteria contributions
- (layouts) A `prompt-submitted` whose prompt starts with the cross-session-message tag carries `origin: harness`, so it would render as a `prompt · harness` tape line and neither fills a readback box nor folds into a send; the same tag mid-prompt stays `human` (per layout-templates §Component — Primary content block 3: the tower tape, and §Component — Hero / signature section).
- (layouts) If a ledger row is added, `viola verify` prints it as one `[NN/MM] <row id> <row words>  pass|fail` line in ledger order, `MM` equals the new row count, and `stamped <ver>  <n> pass  <m> fail` stays the last stdout line (per layout-templates §Output structure — `viola verify`).
- (layouts) `viola verify --record <DIR>` output gains no human line from this chunk's changes (per layout-templates §Output structure — `viola verify`).
