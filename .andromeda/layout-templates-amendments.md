# layout-templates — amendments

History of amendments to `.andromeda/layout-templates.md`, one entry per amendment (form: `/andromeda-wrap-session` `references/sidecar-contract.md`). The body holds only current truth.

## 2026-09-28-capability-ledger-and-viola-verify — `viola verify`'s output structure and its refusal pairs
**Section:** Surface: cli Output structure — `viola verify`; Component — Primary content block 2 (refusal lines)
**Change:**
- The `viola verify` wireframe shows the six ledger rows `[01/06] shim-resolution …` … `[06/06] largest-hook-payload …`, the last stdout line `stamped 2.1.283  6 pass  0 fail`, a failing-row run (`4 pass  2 fail`, exit 1, the stamp still written) and the usage `viola verify [--record <DIR>] [-- <program> [args…]]`; `MM` grows as owning chunks land rows (was the 14-row `[01/14] S3 …` / `stamped 2.1.280  14 pass` sketch).
- The fixed-message exit-1 exceptions add `viola verify`'s four `unable:`/`hint:` pairs (CLI not found, `.cmd`/`.bat`, unreadable version, a dirty recording), run's pinned-copy refusal, and a failed verify's exact `error: internal error`.
**Why:** the chunk landed the verb with those lines (report Symbols; `tests/cli_verify.rs`).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-29-t15-07-57-wrap — registry migration (U35): the layout-templates Decisions Log leaves the body
**Section:** Decisions Log; Surface: web-spa; Component — Header (`<viola-atis>`); Component — Primary navigation (rack separators and keyboard order); Component — Hero / signature section; Surface: cli
**Change:**
- The log moved verbatim to layout-templates-amendments-archive.md (4 entries: the 2026-09-24 initial generation and the three 2026-09-24 overseer fix passes T3–T5, Y4, Z10).
- Surface: web-spa — the document changes no design-system token, colour, font stack, expression level or signature drawing; design-system Component Patterns are its base.
- Header — every boxed ATIS word carries `space-micro` inline padding.
- Primary navigation — the semantic roles of racks, strips and tape, and the announcement mechanics, are a11y's.
- Primary navigation — the v1 bay reserves no control slot; the v1.x brake controls are placed with the security plan at implementation.
- Hero / signature — the cock plays only on a WRAPPED-rack strip, never in the tape, on an unwrapped strip or on the CLI.
- Hero / signature — the `dialog_pending` data plumbing is decided at implementation, outside the layout.
- Surface: cli — one token vocabulary on both surfaces; the differences are platform mechanics only.
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/

## 2026-10-01-t12-19-55-wrap — web-spa tooling context: React + TypeScript
**Section:** Surface: web-spa → Tooling context
**Change:** Framework was Lit 3.3.3 (vendored ESM, no JS build step) with light-DOM `viola-*` Lit elements; now React + TypeScript, a built bundle embedded in `viola`, with the bundler, versions and embedding OPEN (owned by the route's frontend-toolchain entry) and the page built from plain `viola-*` components whose mapping onto React's output is OPEN. The no-`style` rule names React's `style` prop; `dangerouslySetInnerHTML` replaces `styleMap` / `unsafeHTML` in it.
**Why:** founder ruling of 2026-09-30, relayed by the overseer (0.2.0's node-graph view does not fit a build-less Lit page).
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/

## 2026-10-04-wait-and-last — wait / last lines as landed
**Section:** Surface: cli → signature placement · Output structure — `viola wait` / `viola last` · Component — Primary content block 2
**Change:**
- The output structure gains the `session-end` line (every non-dialog kind takes the `turn-ended` form), `dialog unknown`, the exit-21 pair `unable  <name>  instance-unreachable` + its `hint:`, `waiting:` only on a terminal stderr without `--json`, one `--json` document, and the once-printed `error: internal error`.
- Block 2: was "a failed or panicked `verify` prints `error: internal error`"; now every `cli` verb (`send`, `wait`, `last`, `verify`) from the one catch site; the wait/last exit-21 hint is listed; `send` keeps its `[/ ] unable` mirror.
- wait/last join the verbs sharing the `unable` word column.
**Why:** the lines the chunk shipped (the plan's lean: the layout form with the name, over the design line that omits it).
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — the catch-site verb list gains answer
**Section:** §Surface: cli → Component — Primary content block 2 (refusal lines)
**Change:** a failed or panicked `cli` verb was `send`, `wait`, `last`, `verify`; now `send`, `wait`, `last`, `answer`, `verify` — each prints exactly `error: internal error` once from the one catch site, no hint.
**Why:** the chunk added the `viola answer` verb, which exits 1 through the shared catch site.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/
