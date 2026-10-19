proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola wait` / `viola last`"
    change: "The wireframe gains one stdout line for the new wake kind, under the `question` line: `dialog-expired  builder  dialog 7  cursor 49320  <- stdout (an expired hold takes the dialog form; planning form, the founder's wording owed)`. The prose under it says that `viola wait` woken by `dialog-expired` prints `dialog-expired  <name>  dialog <id>  cursor <n>`: ASCII, no SGR, written once, the same form as the three dialog kinds. That form is the planning form, and the founder's wording for the kind word and the line is owed to his Epoch 5 boundary batch (chunk 2026-10-19-dialog-hold-for-the-driver, `inputs#I3` row 3)."
    sidecar: "2026-10-19-dialog-hold-for-the-driver: `viola wait` wireframe gains the `dialog-expired` line (dialog form, `dialog <id>  cursor <n>`; planning form, the founder's wording owed, inputs#I3 row 3)."
    rationale: "The report's Symbols / APIs bullet for `src/cmd/wait.rs` adds `dialog-expired` to `DIALOG_KINDS` and says `viola wait` woken by `dialog-expired` prints `dialog-expired  <name>  dialog <id>  cursor <n>` (the planning form; the founder's wording owed, `inputs#I3` row 3). That is a new output line on the `viola wait` surface, and the wireframe has no entry for it. The Expected amendments bullet names layout-templates §Output structure — `viola wait` / `viola last` as the site, with pattern `waitline` at 0, so the site is the detector's to locate. The Outcome's (design, layouts, a11y) criterion confirms the line is ASCII, without SGR, written once."
    basis: "src/cmd/wait.rs:39-41"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola wait` / `viola last`"
    change: "The `session-end` line's annotation `(every non-dialog kind takes the turn-ended form)` now reads `(every kind other than the three dialog kinds and dialog-expired takes the turn-ended form)`. `dialog-expired` is no dialog kind, but it takes the dialog form, `dialog <id>  cursor <n>`."
    sidecar: "2026-10-19-dialog-hold-for-the-driver: `viola wait` wireframe's turn-ended-form annotation excepts `dialog-expired`, which takes the dialog form."
    rationale: "The report says `fn dialog_form(kind)` decides the human line form and that `DIALOG_KINDS: [&str; 4]` adds `dialog-expired` (Symbols / APIs, `src/cmd/wait.rs`). The wireframe's rule that every non-dialog kind takes the turn-ended form would put the wrapper-appended `dialog-expired` line, which is no dialog kind, in the turn-ended form. That is the claim the primary proposal retires, said again in the annotation."
    basis: "src/cmd/wait.rs:39-41"
    dependent-of: D-layout-surface
note: "The report's Retired item, `viola_core::DIALOG_DEADLINE`, appears nowhere in layout-templates, so there is no D-retired proposal. The proposed wireframe cursor figure (49320) is only an illustration. Like the line form, it waits on the founder's wording at his Epoch 5 batch."
