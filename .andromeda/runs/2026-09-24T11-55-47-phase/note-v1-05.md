2026-09-24 (2026-09-24-observability-gates, phase P5): not claimed. This chunk lands:
- the print/dbg and raw-tracing lint bans with both-ways probes, and the member-list assertion;
- the panic-hook-first witness;
- G2 zero-panic, G4 schema conformance (`viola-harness schema-check`) and the canary secret scan (`viola-harness secret-scan`) over every CI test home, each before any upload.

The acceptance still needs a subject that does not exist at HEAD:
- "the hook still ends successfully": `viola hook` does not exist yet. It belongs to the Hooks to normalised events chunk, which also owns the panic-in-hook exit-0 case.
- "no setting disables redaction": there is no config key that touches redaction yet. It belongs to the chunk that adds the first redaction-bearing setting.
