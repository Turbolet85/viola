# Apply — design-system · part 1 of 2

## Verdicts
- fanout-design-system--cascade.md 19 · applied · .andromeda/design-system.md:303#1
- fanout-design-system.md 1 · folded into fanout-design-system--cascade.md 19
- fanout-design-system--cascade.md 20 · applied · .andromeda/design-system.md:334#1
- fanout-design-system.md 3 · folded into fanout-design-system--cascade.md 20
- fanout-design-system--cascade.md 21 · applied · .andromeda/design-system.md:338#1
- fanout-design-system.md 2 · folded into fanout-design-system--cascade.md 21

## Edits
- line: .andromeda/design-system.md:303
  old: «one of five closed words»
  new: «one of six closed words»
- line: .andromeda/design-system.md:334
  old: «`squatted-name` · `internal-error`»
  new: «`squatted-name` · `socket-dir-refused` · `internal-error`»
- line: .andromeda/design-system.md:338
  old: «`server-verify-failed`, with stderr empty»
  new: «`server-verify-failed` · `socket-dir-refused`, with stderr empty»

## Patterns
[[pattern]]
id = "ds1-01"
fixed = 'one of five closed words'
ignore_case = true

[[pattern]]
id = "ds1-02"
regex = '`squatted-name`[ ·,]+`internal-error`'

[[pattern]]
id = "ds1-03"
regex = '`endpoint-lost`[ ·,]+`server-verify-failed`,? with stderr empty'
