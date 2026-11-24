@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=132
last: - 2026-11-22: A rename of a product constant or a move of a n
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=25 class=omission
leaf-text: `launch{source:wrapper}` → `session-start` (the last start record
source: ctr-fkljkm43 «The events log reads wheel with cause start, then lineage with source wrapper, then launch with source wrapper, then, for a start under --origin, one window line with source wrapper, then the agent's session-start»
replace-with: `launch{source:wrapper}` → (for a start under `--origin`, one `window{source:wrapper}`) → `session-start` (the last start record
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: and neither does the real `hyprctl`, which the same fake agent stands in for as `hyprctl` by its own file name
source: dec-kfjkecgd «The fake agent stands as claude in a test-scoped bin dir, or as hyprctl or as foot, the terminal stand-in, by its own file name, answering from files beside its exe with no env var»
replace-with: and neither does the real `hyprctl` nor the real `foot` terminal, which the same fake agent stands in for as `hyprctl` or as `foot`, the terminal stand-in, by its own file name
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: each ledgered with its clock time first, its live arm refused under CI
source: dec-f4eqv5q5 «A live rig's stand-in program takes every argument the wrapper puts before a program's own words, the rig unsets every inherited VIOLA_* name before it starts anything, and every board read it makes runs with no claude on its PATH.»
replace-with: each ledgered with its clock time first (a live rig's stand-in program taking every argument the wrapper puts before a program's own words, every inherited `VIOLA_*` name unset before the rig starts anything, and every board read it makes run with no `claude` on its `PATH`; provisional), its live arm refused under CI
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=51 class=omission
leaf-text: NEVER run the real `claude`, the real `hyprctl` or real-CLI `viola verify` in CI
source: dec-k5rwj7no «the real hyprctl by the same binary as hyprctl, and the real foot terminal by the same binary as foot; tests never reach the real services.»
replace-with: NEVER run the real `claude`, the real `hyprctl`, the real `foot` terminal or real-CLI `viola verify` in CI
@leaf_edit n=5 leaf=.claude/docs/tests-summary.md line=51 class=omission
leaf-text: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
source: dec-zysuqw2t «A real terminal window placed on its parent's workspace on Hyprland is proven only by a live reading; the fakes prove the placement words and the window records only.»
replace-with: NEVER treat a green fake-agent run as proof of real-CLI behaviour, nor of a real terminal window placed on its parent's workspace on Hyprland, which only a live reading proves (the fakes prove the placement words and the window records only).
@leaf_edit n=6 leaf=.claude/rules/testing.md line=41 class=omission
leaf-text: compare kebab-case serde values and integer exits, never `Debug` strings.
source: dec-7errys5r «An oracle asserting that a refusal echoes none of the given words skips the one allowed word when the refusal's fixed line itself names it.»
replace-with: compare kebab-case serde values and integer exits, never `Debug` strings; an oracle asserting that a refusal echoes none of the given words skips the one allowed word when the refusal's fixed line itself names it.
