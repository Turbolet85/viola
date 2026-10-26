# New groups — test-plan · mocking-stubbing-discipline

## Verdicts
- fanout-test-plan--cascade-g1.md 26 · contradiction · lines 8–8 · «`chrono::Utc::now()` is read only through the clock seam, with one exception as landed» · the file counts one landed `Utc::now()` exception (the budget gate's three evaluation points), the proposal two (adding `LinkSet::line`'s `since`), which no standing group names; the proposal's own change rewrites this run
