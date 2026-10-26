# Apply — architecture

## Verdicts
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:442#1
- fanout-architecture--cascade.md 2 · folded into fanout-architecture.md 1
- fanout-architecture--cascade.md 1 · applied · .andromeda/architecture-detail/stack-and-technologies.md:11#1

## Edits
- line: .andromeda/architecture.md:442
  old: «Tokio 1.53.1 only in `mcp`»
  new: «Tokio 1.53.3 only in `mcp`»
- line: .andromeda/architecture-detail/stack-and-technologies.md:11
  old: «Tokio 1.53.1 built only inside `mcp`»
  new: «Tokio 1.53.3 built only inside `mcp`»

## Patterns
[[pattern]]
id = "arch-01"
fixed = 'Tokio 1.53.1 only in'
ignore_case = true

[[pattern]]
id = "arch-02"
fixed = 'Tokio 1.53.1 built only inside'
ignore_case = true
