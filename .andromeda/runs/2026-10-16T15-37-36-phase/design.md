# design extract

## Relevance
partial — the chunk renders nothing and changes no CLI output, but the design-system still holds browser-leg text (the headless GUI render and contrast checks, the CI font provisioning) that this entry owns and whose wrap amends it.

## Constraints
- design-system §Typography (the paragraph after "Assertions hold on the Linux fallback") says the browser test leg named there "stands until the route entry 'Remove the browser leg' removes it; that entry's wrap amends this text". This chunk is that entry, so the plan must list the amendment under Expected amendments. Implement writes no spec text: the phase never amends a master.
- design-system §Typography, the Rationale bullet "Both faces are Windows-only", also cites the headless GUI checks' render and contrast verdicts judged on ubuntu, with the headless browser running on all three CI OSes. That is the same browser-leg claim. It joins the same Expected amendment so the design-system matches the tree once `e2e-web/` and the `run --browser` steps are gone.
- design-system §Typography "Assertions hold on the Linux fallback" makes the CI image provide the DejaVu families (`fonts-dejavu-core`, `fonts-dejavu-extra`) for the browser render. Research has to answer two things: whether any workflow or harness step installs or checks these packages, and whether it does so only for the browser leg. If one does, it is a browser step and leaves with the leg. If none does, nothing changes.
- design-system §Surface: web-spa reads only the stub «The frontend is deferred to 0.2.0; its technology is not chosen.» Whatever this chunk removes, it pre-builds and reserves nothing for the 0.2.0 frontend.
- design-system §Color Palette (Surface Scale, Text Hierarchy, Semantic Colors, Border Progression), §Spacing, §Depth Strategy, §Border Radius, §Iconography and §Self-Validation Protocol (4. Token Test) each carry the marker "A 0.2.0 frontend design input; 0.1.0 renders no page." They stay as 0.2.0 design inputs. Removing the browser leg that once asserted them does not remove them or make them stale.
- design-system §Surface: cli (Component Patterns 1–5 and the Exit-code phraseology table) fixes every human output byte of `list`, `send`, `wait` / `last`, the wheel verbs, `verify` and `run` / `revive`. The chunk is out of product behaviour, so none of these lines, hints or exit words may change.

## Patterns to follow
- The stand-until marker (design-system §Typography): a master sentence names the route entry that removes what it describes, and that entry's wrap amends the sentence. Phase and implement only name it as an Expected amendment.
- The deferral stub (design-system §Surface: web-spa): a web section collapses to «The frontend is deferred to 0.2.0; its technology is not chosen.» Where the wrap's amendment shortens browser-only text, it uses this exact wording and no new prose.
- The section marker "A 0.2.0 frontend design input; 0.1.0 renders no page." (design-system §Color Palette, Surface Scale): shared visual sections stay true to the tree by being labelled as future input, not by being deleted.

## Anti-patterns to avoid
- Deleting or rewriting the design-system's shared visual sections (palette, type stacks, spacing, depth, radius, iconography, Token Test) as browser-leg leftovers. Per design-system §Color Palette (the Surface Scale marker) they are 0.2.0 design inputs, not test infrastructure.
- Touching the CLI's colour layer, its words or its streams while removing the browser leg. design-system §Anti-Patterns (Per-Surface Bans: cli) and §Surface: cli (Colour decision order) are product behaviour, and this chunk's boundary excludes product behaviour.

## Contract bindings
- design-system §Typography (headless render and contrast assertions on the Linux fallback) ↔ the test-plan browser leg and the a11y-plan contrast checks. All three describe the one browser suite this chunk removes, so the wrap's amendments to these three masters land together.
- design-system §Color Palette (Contrast rules) ↔ a11y §Contrast (SC 1.4.3). Once the browser leg is gone, 0.1.0 has no automated contrast assertion. That is consistent only because 0.1.0 renders no page (the §Color Palette marker), so the binding is carried as a 0.2.0 obligation and not dropped.

## Acceptance criteria contributions
- The plan's Expected amendments name the design-system §Typography browser-leg text: the Rationale bullet's headless-GUI-check clause, the "Assertions hold on the Linux fallback" paragraph's CI-render and DejaVu-provisioning claims, and the stand-until note. The chunk's implement diff touches no line of `.andromeda/design-system.md` (per design-system §Typography).
- After the chunk, no workflow, pre-push stage or harness verb installs or checks a font package for a browser render. The search has to find the site at base first; if it finds none at base, the check is recorded as "no site at base", not as passed (per design-system §Typography, Assertions hold on the Linux fallback).
- The tests that pin the CLI human output samples (`viola list` rows, the `viola send` readback mirror, `verify` step lines, the start-refusal `unable:` / `hint:` pairs) pass unchanged, with no expected-output edit in the diff (per design-system §Surface: cli, Component Patterns).
