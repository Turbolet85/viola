# Apply — design-system

## Verdicts
- fanout-design-system--cascade.md 13 · applied · .andromeda/design-system.md:303#1
- fanout-design-system.md 1 · applied · .andromeda/design-system.md:342#1
- fanout-design-system--cascade.md 14 · folded into fanout-design-system.md 1
- fanout-design-system.md 2 · applied · .andromeda/design-system.md:261#1
- fanout-design-system--cascade.md 4 · folded into fanout-design-system.md 2
- fanout-design-system.md 3 · applied · .andromeda/design-system.md:338#1
- fanout-design-system.md 4 · applied · .andromeda/design-system.md:330#1
- fanout-design-system--cascade.md 12 · folded into fanout-design-system.md 4
- fanout-design-system--cascade.md 1 · applied · .andromeda/design-system-detail/surface.md:130#1, .andromeda/design-system-detail/surface.md:130#2, .andromeda/design-system-detail/surface.md:130#3
- fanout-design-system--cascade.md 2 · applied · .andromeda/design-system-detail/surface.md:57#1
- fanout-design-system--cascade.md 3 · applied · .andromeda/design-system-detail/surface.md:21#1
- fanout-design-system--cascade.md 5 · applied · .andromeda/design-system-detail/surface.md:37#1
- fanout-design-system--cascade.md 6 · applied · .andromeda/design-system-detail/surface.md:39+#1
- fanout-design-system--cascade.md 7 · applied · .andromeda/design-system-detail/surface.md:65#1
- fanout-design-system--cascade.md 8 · applied · .andromeda/design-system-detail/surface.md:72+#1
- fanout-design-system--cascade.md 9 · applied · .andromeda/design-system-detail/surface.md:122#1, .andromeda/design-system-detail/surface.md:122#2, .andromeda/design-system-detail/surface.md:122#3
- fanout-design-system--cascade.md 10 · applied · .andromeda/design-system-detail/surface.md:109+#1
- fanout-design-system--cascade.md 11 · applied · .andromeda/design-system-detail/surface.md:110+#1

## Edits
- line: .andromeda/design-system.md:303
  old: «`revive`'s read in a fixed order that stops at the first.»
  new: «`revive`'s read in a fixed order that stops at the first; the exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair (Exit-code phraseology below), not the `[/ ]` mirror.»
- line: .andromeda/design-system.md:342
  old: «`viola link` / `viola unlink`'s refused strict-modes check of the driven instance (exit 21) prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.»
  new: «`viola link` / `viola unlink`'s refused strict-modes check of the driven instance, and that of `viola send`, `wait`, `last`, `answer`, `pause` and `release` on their instance, made before any frame (exit 21), prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty; no frame and no line are written.»
- line: .andromeda/design-system.md:261
  old: «the exit-21 strict-modes refusals of `list`, `link` and `unlink`)»
  new: «the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release`)»
- line: .andromeda/design-system.md:338
  old: «21 `unable  instance-unreachable` then the cause's hint, its `--json` document with stderr empty.»
  new: «21 `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state `unable:` / `hint:` pair instead (pattern 1 and the paragraphs below), its `--json` document with stderr empty.»
- line: .andromeda/design-system.md:330
  old: «a run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.»
  new: «a run stopped by Ctrl-C or TERM before its rows are checked, or refused by the strict-modes check of its home tree before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1; a `stamps.json` update refused by its strict-modes check prints the home-state pair.»
- line: .andromeda/design-system-detail/surface.md:130
  old: «(not running / unwrapped / strict-modes / server verification)»
  new: «(not running / unwrapped / server verification)»
- line: .andromeda/design-system-detail/surface.md:130
  old: «relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable"»
  new: «relayed verbatim by the Viola overseer), and so does the instance check `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame, printing the instance-state pair naming it, with no frame and no line written | `{"v":1,"error":"instance-unreachable"»
- line: .andromeda/design-system-detail/surface.md:130
  old: «the `list`, `link` and `unlink` refusals included»
  new: «the `list`, `link` and `unlink` refusals and the six frame verbs' strict-modes refusals included»
- line: .andromeda/design-system-detail/surface.md:57
  old: «the viola home fails strict-modes: `the viola home is not private to you; <name> is not contacted until that is fixed`»
  new: «the home or the instance's state files fail the strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause` and `release` first; each before any frame): no `unable  instance-unreachable` line and no hint of this list, but the fixed instance-state pair `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` (pattern 1, Refusal), not the `[/ ]` mirror; under `--json` the `instance-unreachable` document»
- line: .andromeda/design-system-detail/surface.md:21
  old: «make of the driven instance, use the fixed message form»
  new: «make of the driven instance, and the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance, use the fixed message form»
- line: .andromeda/design-system-detail/surface.md:37
  old: «print it, naming the driven instance, at their exit-21 strict-modes refusal).»
  new: «print it, naming the driven instance, at their exit-21 strict-modes refusal; `viola send`, `wait`, `last`, `answer`, `pause` and `release` print it, naming their instance, at theirs; `viola run`'s and `viola revive`'s start print it at an instance-level refusal of their home-tree check, exit 1).»
- after: .andromeda/design-system-detail/surface.md:39
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cli-list -->»
  add: «»
  add: «- The board's strict-modes scan holds the home to two rules: it checks the home by the write rule, then each instance's set, which holds the home to the read rule. So a home that group or other may read but not write passes as the empty board when no instance exists, and with an instance refuses the whole board with the instance-state pair naming that instance: which pair the refusal prints depends on whether an instance exists. Pending, owner: a CARRY on the working-route entry «Unix endpoint and home hardening».»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/design-system-detail/surface.md:65
  old: «(strict-modes at start) → `hint: the viola home must be readable only by you; viola does not change its permissions`»
  new: «(a home-level refusal of the home-tree check made at start before any log, of the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/`, each that exists; no child) → `hint: the viola home must be readable only by you; viola does not change its permissions`; an instance-level refusal of the same check prints `unable: <name>'s state files can be written by another user` → `hint: viola will not read them; make the viola home and its files owner-only`»
- after: .andromeda/design-system-detail/surface.md:72
  anchor: «    - `unable: <name>'s recorded directory is missing` → `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`»
  add: «    - the home-tree check of its start, after a passed preflight and before any log (no child): a home-level refusal → `unable: the viola home is not private to you` → `hint: the viola home must be readable only by you; viola does not change its permissions`; an instance-level refusal → the instance pair above; under `--json` the `strict-modes-failed` document»
- line: .andromeda/design-system-detail/surface.md:122
  old: «`<name>'s command is a .cmd or .bat script` / `the viola home is not private to you`, or a `viola revive` refusal»
  new: «`<name>'s command is a .cmd or .bat script` / `the viola home is not private to you` / `<name>'s state files can be written by another user`, or a `viola revive` refusal»
- line: .andromeda/design-system-detail/surface.md:122
  old: «`<name>'s recorded directory is missing` (or `run`'s `already live` pair)»
  new: «`<name>'s recorded directory is missing` / `the viola home is not private to you` (or `run`'s `already live` pair)»
- line: .andromeda/design-system-detail/surface.md:122
  old: «`the CLI version could not be read` / `a recorded fixture»
  new: «`the CLI version could not be read` / `the viola home is not private to you` (its home tree refused before its log, no step line and no summary, or a `stamps.json` update refused by its strict-modes check) / `a recorded fixture»
- after: .andromeda/design-system-detail/surface.md:109
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cli-verify-run -->»
  add: «»
  add: «- Before its log, `verify` checks its home tree (the home, `ledger/`, `bin/`, `plugin/`, each that exists): a refusal prints no step line and no summary, only the home-state pair `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions` (pattern 2's fixed form), and exits 1; a `stamps.json` update refused by its strict-modes check prints the same pair.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/design-system-detail/surface.md:110
  anchor: «»
  next: «<!-- detail: values · cli-verify-run -->»
  add: «<!-- detail: values · cli-verify-run -->»
  add: «»
  add: «- On Unix a home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1) and the frame verbs the instance-state pair (exit 21). On Windows such a path stats `NotFound`, is passed as absent, and keeps the catch-site line.»
  add: «»
  add: «<!-- /detail -->»
  add: «»

## Patterns
[[pattern]]
id = "ds-01"
fixed = 'is not contacted until that is fixed'

[[pattern]]
id = "ds-02"
fixed = 'unwrapped / strict-modes / server verification'

[[pattern]]
id = "ds-03"
fixed = 'strict-modes at start'

[[pattern]]
id = "ds-04"
fixed = 'unable  instance-unreachable` then the cause'

[[pattern]]
id = "ds-05"
fixed = 'strict-modes refusals of `list`, `link` and `unlink`'

[[pattern]]
id = "ds-06"
fixed = 'the `list`, `link` and `unlink` refusals included'
