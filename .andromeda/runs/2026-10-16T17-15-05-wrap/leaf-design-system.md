group: design-system
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:34   class: stale
  leaf-text: «**The Linux DejaVu render is what CI asserts** (until the route entry "Remove the browser leg" removes the browser test leg).»
  source: .andromeda/design-system.md:269 «0.1.0 renders no page, so no CI step renders these stacks or provides their families; how a 0.2.0 frontend checks these assertions is that version's to decide, its technology not chosen.»
  replace-with: «**Every contrast and render assertion must hold on the resolved Linux DejaVu families**; 0.1.0 renders no page, so no CI step renders these stacks or provides their families (how a 0.2.0 frontend checks them is that version's to decide).»
