# Apply — security-plan

## Verdicts
- fanout-security-plan--cascade.md 1 · applied · .andromeda/security-plan-detail/dependency-security.md:14#1
- fanout-security-plan--cascade.md 2 · refused · the retirement empties the whole group .andromeda/security-plan-detail/dependency-security.md:56–60 (its one line 58, with the blank at 55 before it), and this payload's in-line form cannot empty a line (cascade replace: «a line emptied — remove it by hand»): the report's Retired bullet bears it out, so the group lines 55–60 are a hand removal for the orchestrator, the file then ending at line 54 `<!-- /detail -->`

## Edits
- line: .andromeda/security-plan-detail/dependency-security.md:14
  old: «(tokio `=1.53.1`, `io-util`)»
  new: «(tokio `=1.53.3`, `io-util`)»

## Patterns
[[pattern]]
id = "sec-01"
fixed = 'tokio `=1.53.1`'
ignore_case = true
