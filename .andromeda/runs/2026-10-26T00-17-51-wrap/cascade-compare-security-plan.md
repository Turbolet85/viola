# Cascade compare — security-plan · wrap · 2026-10-25-windows-endpoint-admission
E items 3 · held 2 · lost at classify 0 · lost at strong 0 · beside a placed fact 1 · cascade only 1 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### dependency-security#2.1 · lost at classify · beside a placed fact F4 (near 14, dependency-security#2.1) · .andromeda/security-plan-detail/dependency-security.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 14
    section: "## Dependency Security"
    change: "- An open advisory against the locked tokio: RUSTSEC-2026-0386 (`unsound`, tokio 1.53.1) fails CI's `supply-chain` job on every run since the 2026-10-20 wrap (ci#40208779387 at `b6740d59`, before chunk 2026-10-25-windows-endpoint-admission's edits; ci#40247500584 at `e2bb59c8`, its only red, the other 14 checks `success`). It is owned by the working-route entry \"tokio RUSTSEC-2026-0386 lock bump\", placed right after \"Windows endpoint admission\": a lock bump of `tokio` to `>=1.53.3`, with no ignore-list widening (the overseer's founder-delegated answer, confirmed by the founder's own yes on that chunk's P5 card). Until it lands, RUSTSEC-2017-0008 stays the only `[advisories]` ignore."
    sidecar: "Dependency Security (deps): the open RUSTSEC-2026-0386 supply-chain red on tokio 1.53.1 recorded with its owner, the route entry \"tokio RUSTSEC-2026-0386 lock bump\" (bump to >=1.53.3, no ignore widened) — chunk 2026-10-25-windows-endpoint-admission."
    rationale: "The report's Cross-project claims and Outcome gates record the supply-chain red (`unsound` RUSTSEC-2026-0386, tokio 1.53.1) as not this chunk's, its owner the new route entry from inputs#I2 / inputs#I6 (Beyond the plan: a lock bump to >=1.53.3, no ignore-list widening); the sweep's `tokio-adv` pattern fired on no master text, so it is a new fact, and this file's deps group (tokio `=1.53.1` behind `viola-channel`'s `tokio` feature) is where the advisory and its no-ignore disposition belong."
    basis: viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/evidence/operator-pass.md

## Held

- authentication-authorization#1.3
- bootstrap-phases#1.1

## Cascade only

- threat-model-summary#2.1
