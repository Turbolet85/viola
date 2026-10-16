
## 2026-10-16-remove-the-browser-leg — Typography: the CI render and the DejaVu provisioning leave with the browser leg
**Section:** Typography (the Rationale's «Both faces are Windows-only»; the per-OS fallback table's Linux column; «Assertions hold on the Linux fallback»; the stand-until note)
**Change:**
- Was: the headless GUI checks' render and contrast verdicts were judged on the ubuntu CI leg, the Linux stack was "what CI renders" (the table's «the CI render»), the CI image had to provide `fonts-dejavu-core` / `fonts-dejavu-extra`, and a note held this text until the route entry "Remove the browser leg" (the Kept clause of «2026-10-16T14-58-10-wrap — Surface: web-spa, web motion and web bans cut; the shared sections stand as 0.2.0 inputs»). Now 0.1.0 renders no page, so no CI step renders these stacks or provides their families, and the note is gone.
- The contrast and render assertions stand as a 0.2.0 design input: they hold with "DejaVu Sans Condensed" + "DejaVu Sans Mono" resolved, on the resolved family, and a resolved face outside the stacks breaks them. How a 0.2.0 frontend checks them is that version's to decide, its technology not chosen.
- The table's Linux column reads «Linux (CI-tested)».
**Why:** This chunk removed the browser test leg, on the founder's ruling of 2026-10-16 that 0.1.0 is a pure CLI (item 3). No DejaVu provisioning site existed at the base.
**Kept:** The per-OS stacks (`--font-label`, `--font-field`), the fallback table and the column sizing that takes DejaVu Sans Mono as the widest case, all 0.2.0 design inputs.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/
