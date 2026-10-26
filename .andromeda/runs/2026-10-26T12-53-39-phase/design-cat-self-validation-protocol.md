# design · Self-Validation Protocol — category read

## Carried
- (.andromeda/design-system.md:389) The chunk presents new human CLI output (the `link` / `unlink` result lines and the `handoff:  link, unlink` help row), so the protocol's checks must be run on that output before it is presented.
  > Before presenting ANY UI output, downstream implementation phases (per project's specialist plans) must run these checks:
- (.andromeda/design-system.md:412) The new `link` / `unlink` result lines must pass the Sameness Test: their words come from this product's own phraseology (the master's `link    overseer -> builder  since …Z` / `already linked` / `unlink  overseer -> builder` forms), never a generic "linked successfully" line.
  > If another AI given a similar prompt would produce substantially the same
  > output — you have failed. The interface must emerge from THIS product's
  > domain exploration, not from statistical patterns in training data.

## Bindings
- **Sameness Test ↔ the body's Surface: cli, Component Patterns 4 (wheel, link and gate verbs)**: the test is passed by printing the fixed link / unlink phraseology the cli surface names, ASCII `->` and two-space columns, uncoloured (`.andromeda/design-system.md:412`, `.andromeda/design-system.md:323`, `.andromeda/design-system.md:324`).
- **Token Test ↔ this chunk**: the category file's only test is a 0.2.0 frontend design input and binds no 0.1.0 CLI output, so this chunk owes it nothing (`.andromeda/design-system.md:407`, `.andromeda/design-system-detail/self-validation-protocol.md:8`).
- **Signature Test ↔ this chunk**: the readback box's one 0.1.0 place is CLI `viola send`'s column, which this chunk does not change; the link / unlink lines carry no readback box (`.andromeda/design-system.md:402`).
