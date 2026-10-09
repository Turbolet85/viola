# Cascade dispositions — the 2026-10-09T14-44-30 0-pending wrap

One amendment in this pass: the a11y keyboard-test-harness key file
(`.andromeda/registries/contracts/a11y-plan/keyboard-test-harness.md:8`) names the working-route entry
"Windows-only live measurements" by its title where it said route `:140`.

**The search** (`cascade-patterns.toml`, run by `cascade.py sweep` after the edit; baseline `fe4f47fb`, the listing
is `cascade-listing.txt`): three patterns over the seven masters, every `.andromeda/registries/**` file, the three
curation homes, the two judgment bases and the leaf bodies.

- `route140` — fixed `route` + the backticked `:140`: the retired wording itself.
- `real-win-term` — fixed "real Windows terminal", case-insensitive: the claim's subject, wherever a master says
  where or whether that report is measured.
- `measured-live-at` — regex "measured live at", case-insensitive: the claim's verb phrase.

Not searched for: other bare route numbers (a separate read, recorded in `citation-dispositions.md`); the ledger's
dated notes on `v1-31`, `v1-32` and `v1-40`, which cite `:90` and are history, not master text.

**Every row the listing printed:**

| row | disposition |
|---|---|
| `route140` · 0 rows (control fired on the pre-pass key file) | the retired wording stands nowhere after the edit |
| `.andromeda/architecture.md:70` `real-win-term` standing | no change — a true claim sharing the token: "a mouse report from a real Windows terminal is not yet measured", which names no route number and agrees with the key file |
| `keyboard-test-harness.md:8` `real-win-term` standing, edited | amended — this is the amended sentence; read whole after the edit, the phrase stands once and now ends in the entry's title |
| `keyboard-test-harness.md:8` `measured-live-at` standing, edited | amended — the same sentence; no second "measured live at" on the line (×1) |

Leaf rows: none (leaf 0 for all three patterns). The key file's leaves are its master's
(`.claude/docs/a11y-summary.md`, `.claude/rules/a11y.md`, CLAUDE.md's warnings block); none carries the sentence or
a route number for that entry, so the recompute changes nothing in them. Curation homes 0 · judgment bases 0.

`registry.py check --project . --master .andromeda/a11y-plan.md`: 0 defects after the edit.
