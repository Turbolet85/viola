# Adaptation record — the 2026-10-16T14-58-10 0-pending wrap

Viola 0.1.0 becomes a pure CLI; the whole frontend moves to 0.2.0.

## The input
- `inputs#I1` (this run dir's `inputs/`, `--no-chunk`, kind `relay`): `../additional/viola-overseer/restructure-cli-only.md`,
  the overseer's relay of the founder's ruling of 2026-10-16. It carries his own words verbatim («…в 0.1.0 был чисто CLI…»), his
  dialog choices (finish the pending chunk first · the masters' web sections cut to a stub · the browser infrastructure
  removed by a 0.1.0 chunk), and five items for disposition at route-resolve. Its line numbers were measured at `880b056`;
  the entry titles were the anchors and each was re-read at `3fc2e61`.
- Authority: the ruling is the founder's own word, relayed verbatim. The five items are the overseer's elaboration of it,
  presented by the operator at invocation as the request. Each item names both its entries and their disposition, so it
  satisfies the trajectory gate (route-resolve, a recorded pre-direction). The operator answered the forks below in this
  session's dialogs, recording each as the overseer's word (technical, founder-delegated).

## The operator's answers (this session's dialogs)
1. Browser test infrastructure in the masters: **at "Remove the browser leg"'s wrap** (Recommended). «masters stay true to
   the tree; the test-infrastructure text is marked owned by "Remove the browser leg" and that chunk's wrap amends it when
   the code goes.»
2. v1-39: **add the note** (Recommended). «the relay's list missed v1-39; the same port clause, the same ruling — a dated
   note, the CLI half unweakened.»
3. The closed obs vocabulary (`ObsProcess::Ui`, `ObsEvent::HttpRequest|LivenessChanged|SseOpened|SseClosed`,
   `schemas/diag-line.v1.json` / `diag-detail.v1.json`): **true to the tree** (Recommended). «mark them reserved with no
   emitter in 0.1.0; the CARRY on "Remove the browser leg" removes code, schemas and master text together.»
4. Design-system's shared visual sections outside the named scope: **keep them for 0.2.0**. «escher, a Blitz fork, renders
   HTML/CSS too … each heading marked with one line: a 0.2.0 frontend design input; 0.1.0 renders no page. Cut only
   Surface: web-spa, the web motion and the web bans, as the ruling named.»

## Item 1 — Epoch 8's web entries leave 0.1.0 (applied)
- Ten markerless entries were appended verbatim, freight included, to `.andromeda/residuals.md` as `open` lines, origin
  `2026-10-16T14-58-10-wrap`, target 0.2.0 (`residual-01.txt` … `residual-10.txt`): Frontend toolchain · viola ui loopback
  server · Resumable SSE feed · Web test toolchain and a11y harness · Design token bundle and bay layout · Strip and readback
  primitives · Strip-bay live page · Access and error strips · Contrast, forced colours, reduced motion · A11y verdict
  across page states.
- Their lines and `↓` separators were deleted from the working route (20 `splice delete`, bottom-up, each
  `--expect-prefix`).
- The epoch header now reads `### Epoch 8 — Polish & ship`. "Version done-check" stays without «founder screen-reader
  pass recorded», which is residual 11.

## Item 2 — web clauses inside CLI entries (applied)
- "CLI machine contract": «config.json thresholds and port» became «config.json thresholds». The clause, with v1-39's
  same port clause, is residual 12.
- "Bounded inputs at every boundary" and "Sanitised error surfaces": measured, their route text names no web clause. The
  web clauses live in v1-46 («a bad `Last-Event-ID` pair drops the whole header») and v1-47 (the GUI token, cookie and
  launch URL; Problem Details). They are carried as residuals 13 and 14; both route lines stand unchanged.

## Item 3 — "Remove the browser leg" (applied)
- A new markerless entry heads the tail (`route-entry-remove-browser-leg.txt`). Its scope is ≤25 words. Three CARRYs:
  1. the ruling and every site measured at `3fc2e61`;
  2. answer 1 — the masters' browser-test text is owned there;
  3. the GUI harness and gate leftovers measured in the tree, and the reserved obs vocabulary (answer 3).
- The previous head ("Unwrapped sessions on the board") carried no PREREQ or WATCH, so nothing re-pinned.

## Item 4 — the ledger (applied)
- `matrix.py defer` (dated texts `defer-v1-*.md`): v1-22, v1-48, v1-49, v1-50, v1-51, v1-52 → `deferred`.
- `matrix.py note` (dated texts `note-v1-*.md`): v1-02, v1-04, v1-05, v1-09, v1-10, v1-11, v1-36, v1-37, v1-40, v1-46,
  v1-47 (planned), v1-08, v1-23 (verified), and v1-39 (answer 2). `refine` is the claiming chunk's verb (it takes
  `--chunk`), so each unclaimed cap's web clause leaves through a dated note. No acceptance was changed.
- `requirements.md` and `intent.md` are untouched; a ruling on an existing requirement is a dated ledger note.
- Coverage after: `verified 16/53 · deferred 6 · planned 31 · unclaimed 31`.

## Item 5 — the masters' web sections (applied as amendments, full apply-form)
- Seven read-only mappers (one per master and its key files), then planners turning each map into a guarded edit plan
  (`edits-*.json`), every plan reviewed whole and applied by `medit.py`. That is the line-write contract's hand-rolled
  write: binary read, unique anchors, LF kept, read-back with every untouched line identical.
- Ops dropped on review:
  - security-plan 51 (`axum` `http2` ban: `deny.toml` holds it);
  - security-plan 59 and 70 (the scan's `?t=` note and its GUI classes: `secret_scan.rs` holds them);
  - obs-plan 100 (the same scan classes).
- Line counts, before → after:

  | Master | Lines |
  |---|---|
  | architecture | 524 → 459 |
  | security-plan | 626 → 554 |
  | design-system | 930 → 598 |
  | layout-templates | 627 → 320 |
  | test-plan | 1367 → 1226 |
  | obs-plan | 1205 → 1124 |
  | a11y-plan | 1077 → 420 |

  Key files: 4 architecture, 7 test-plan, 10 obs-plan, 10 a11y-plan. `registry.py check --all`: 0 defects.
- The bound test-plan §3 ↔ obs-plan §3 Log-format line was reconciled byte for byte (`edits-obs-plan--bind-*.json`).
- Three citations of the departed "frontend-toolchain" route entry were re-pointed.
- The cascade:
  - sweep: `cascade-patterns.toml`, 11 patterns, 588 rows over 51 files;
  - dispositions: `cascade-dispositions.md`;
  - leaves: 14 comparers, 230 findings (155 by the tool, 75 by hand, 2 declined);
  - one sidecar entry per master, each `on-form` and read back as its file's last line.

## Named for the founder (his batch; none ruled here)
- **Dangling Brand citations.** design-system Brand Identity stays untouched by his word. Its L41 / L45 cite the cut
  Motion values and its L16 / L21 name the cut web-spa components.
- **The a11y tier.** a11y-plan's `Standard (1)` tier line now rests on the terminal surfaces alone and is his to re-decide.
- **Web-only leaves.** `.claude/rules/frontend.md`, `.claude/rules/a11y.md` and the doc `.claude/docs/services/viola-ui.md`
  (a stub for a crate that does not exist) are named, not deleted.
- **The obs-plan §3 copy.** It lagged test-plan's Log-format text before this pass.
- **Item 4's sources.** Its deferral and note texts rest on the relay's list, widened by answer 2.

## Not run on this path
No report and no drift fan-out (this path runs none). No citation sweep (P2's opening; nothing here re-points a master's
`path:N` citation). No curation: the conversation carried answers, not corrections. No evolve checkpoint (the path has
none). No `gated` record exists to re-verify (`route.py cursor`: gated 0).
