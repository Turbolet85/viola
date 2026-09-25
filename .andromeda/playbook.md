# Playbook — viola

<!--
Amendment-validation rules consulted by /andromeda-wrap-session's main agent when it validates the
amendments its fan-out proposed. This file GROWS from dogfood — it starts near-empty. One rule per entry:

  - pattern: {the class of amendment this matches}
    verdict: routine | escalate       # routine → apply silently; escalate → halt + ask the user
    note: {why}

"Main is uneasy" (no rule matches but it looks strange) → escalate too; a confirmed escalation pattern
becomes a new rule here. Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).
-->

## Rules
- pattern: Sequencing deferral — a drift-detector flags a gap (a tool not yet installed, a capability not yet hardened, a dependent not yet wired, a span not yet emitted, a test tier not yet built) whose resolution is a LATER, still-pending chunk's defined job. NOT Foundation-only: it equally covers a primitive shipping before its driver, a config landing before the runtime that traces it, and a gate deferred at /implement that the wrap light gate then runs green.
  verdict: routine
  note: builds land capabilities first and harden / depend on them in later chunks — expected ordering, NOT spec drift (the spec is right; the resolving chunk just hasn't run yet). Apply silently. Auditable caution — it is sequencing ONLY if a later chunk genuinely owns the resolution; a gap with NO resolving chunk anywhere in the route is real drift → escalate. And ownership is a ROUTE annotation (a CARRY/PREREQ pinned on the owning entry at route-resolve), never this note or prose — a note cannot verify delivery, and an owner-chunk can complete without delivering unless the obligation rides its line. A dismissal citing a route-sequenced owner whose entry is already `complete` is NOT routine — the owner came and went; escalate it.

- pattern: Not this chunk's drift — the proposal names a symbol / dependency / section / surface that does NOT appear in the report's Changes as this chunk's work: a dependency the chunk never touched, a plan↔plan bind neither of whose sections it changed, an invariant already satisfied by shipped infrastructure it merely consumed, a pre-existing surface it only extended internally.
  verdict: routine
  note: reject — the report's Changes is the single source of what changed this wrap, so a detector firing on anything else has mis-fired (commonly by reading the doc's own history or rationale as a current gap). Verify against the report before dismissing. Caution: where the proposal is ACCURATE and the drift REAL but pre-existing across several artifacts the chunk did not touch, do NOT amend the one artifact the detector named — that leaves a worse spec-vs-impl mismatch; route the whole family to its OWNED channel instead: a `CARRY:` pin on the markerless entry that owns that surface (route-resolve, this wrap), or — when it belongs to a future version — an `.andromeda/residuals.md` append.

- pattern: Registry over-reach — a detector proposes registering per-item REALIZATION into a spec registry that tracks its subject at CATEGORY grain: a public API symbol, an internal module, a CLI verb or flag, a command / handler name, an individual config file, a framework permission granted inside an already-registered capability file, a second transport instance for an already-registered surface.
  verdict: routine
  note: reject — the arch registries track occupied RESOURCES (ports / sockets / endpoints / IPC / events / env vars / crates / artifacts) and contract SHAPES, not the realization inside an already-registered category. Check what the registry actually enumerates, and whether earlier chunks of the same kind registered theirs — they usually did not. A genuinely new resource of a kind the registry DOES enumerate still registers normally.

- pattern: Accurate this-chunk addition — the proposal registers something the CURRENT chunk genuinely introduced or changed (its named symbols / values DO appear in the report's Changes), landing inside an existing section; or it reconciles a spec's illustrative wording / mechanism to the sound implementation the chunk shipped, where the report shows the invariant still holds.
  verdict: routine
  note: apply — the apply-side dual of the two reject rules above; without it a playbook only learns to dismiss. Bringing a body to current truth IS reconcile's job. The test is that the named thing is THIS chunk's and the invariant survives — only form or mechanism may differ. A REVERSAL of a locked decision is not this rule: escalate that once to ratify it.

- pattern: External decay — a gate turns red with NO in-diff cause: the lock / source / config it checks is un-drifted and the failure keys on the world moving while the project stood still (a freshly-fetched advisory DB, an expired tool or cert, a registry policy change) — typically surfacing after a pause.
  verdict: routine
  note: neither this chunk's drift (nothing in the diff caused it) nor sequencing (no future chunk owns it yet) — the world moved, the code didn't. It never blocks the chunk that DISCOVERED it, and it never resolves by silently widening an ignore list (actionable-with-fix items are not the non-actionable class an ignore legitimately absorbs). It ALWAYS produces an owner: a route entry, or a recorded bounded deferral. If the gate must pass meanwhile, an ID-scoped reasoned ignore NAMING its owning chunk is part of the pattern, not a shortcut — a permanently-red gate stops discriminating, so a NEW red becomes invisible. An operator-RATIFIED standing deferral may compact its re-pin and set a probe re-run interval — with the overlap probe NAMED and verified green every chunk, and interval skips recorded in the chunk report, never silent — or take the probe-auto-satisfy tier: the pin names its SIGNATURE (failing gate's exit + first diagnostic line + overlap result) and a byte-identical probe — read from the external artifact's current state, never only a local copy of it — satisfies it with a one-line record; any deviation restores the full form.

- pattern: Boundary widening — a chunk WIDENS what crosses an already-hardened boundary (a read-only channel gains a write, a validated surface admits a new input class, a subprocess/IPC boundary gains a new crossing) and the proposal records it.
  verdict: escalate
  note: always a human's call — never mint a routine rule for this class, however often it recurs: the recurrence is the reason it must keep reaching the operator (a routine verdict here silently widens a precedent). The escalation resolves on the operator's ratification, recorded in the sidecar.

- pattern: Verbatim scope copy — a proposal or a cascade-sweep hit lands inside obs-plan §1 (Obs Scope Summary). That section is the verbatim copy of obs-scope.md; its closing note (obs-plan.md:485) says it keeps its pending wording and that §3, §6 and §12 win where they differ.
  verdict: routine
  note: reject for §1 — never bring §1 to current truth. Apply the change in the section that wins (§3 / §6 / §12) instead. The cascade edited §1 twice in Epoch 1 (runs/2026-09-24T17-37-22-evolve-diagnose/proposals.md P9: L91, L112). Appended 2026-09-25 by the 2026-09-24-epoch-1-cleanup wrap, operator-approved.

_(more grow from escalations + resolved cases — the first five above were harvested from live projects that
derived them separately, the sixth is the never-routine class; anything genuinely project-specific still starts
here empty.)_
