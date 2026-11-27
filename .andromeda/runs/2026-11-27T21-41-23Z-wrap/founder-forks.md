# Founder forks — wrap 2026-11-27T21-41-23Z-wrap (chunk 2026-11-27-epoch-5-cleanup)

Written 2026-11-27T22:09:22Z. The operator is away; nothing here is asked by dialog (`inputs#I7`). Nothing below is
a boundary widening, and nothing waits on it: the wrap commits without these rows. Each is a master row the fit reader
judged `not_master` (the code holds it); only his own word lands a row against that verdict
(`amendment-flow.md` §Validate check 7). Default while unanswered: no row, as landed.

1. `architecture · decision/shared-start-prologue` — «Run, revive and adopt start through one shared start prologue:
   the instance trust check, then the checked executable (a strict-modes refusal by the caller's writer under --json,
   else a home-tree refusal), then the wrapper log open, in that order; no start path carries its own copy of these
   checks.» — fit: not_master type 4.
2. `architecture · decision/one-window-step-refusal-writer` — «up's refused window-step bodies come from open's one
   refusal writer, so up and open refuse a window step with the same body; no verb keeps its own copy of that writer.»
   — fit: not_master type 4.
3. `test-plan · decision/boot-paths-declared-once` (the plan's discretion item 9) — «The project's boot paths are two
   named members: the binary's entry point, and the one shared start that viola run, viola revive and viola adopt each
   start through; a chunk that changes either member changes the boot path.» — fit: not_master type 5.

His answer per item: «land it» (a later wrap lands it as his exception, its entry saying so) or «leave it» (nothing).
