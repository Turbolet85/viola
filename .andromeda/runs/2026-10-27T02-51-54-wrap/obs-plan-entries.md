## 2026-10-27-pinned-copy-race-and-reproducible-builds — pinned-exe hash-mismatch refusal telemetry through a lost placement
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the required telemetry signal, its refusal-telemetry item)
**Change:**
- A new sub-item under the refusal-telemetry item: a pinned-exe hash mismatch is also reached when a start loses the pinned copy's no-clobber placement to a concurrent first start and the copy that stands re-hashes different (`place_absent`'s `PinError::HashMismatch`, `run.pin_copy` `pinned-hash-mismatch`, the file left as found).
- That path adds no line, field or `detail` code.
**Why:** the report's Symbols / APIs: `pin_bytes`' absent-copy arm now calls the private `place_absent`, whose `Present` arm (a concurrent first start placed the copy first) re-hashes the standing copy, equal reused and different `PinError::HashMismatch`; its Schema / config: no `detail` code, log line, field or span was added. §1 is a verbatim upstream copy kept current, by the founder's ruling of 2026-10-04, relayed by the overseer.
**Kept:** the master's broad hash-mismatch telemetry line and its tampered-copy refusal stay as written; both are still true.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — `run.pin_copy` under a concurrent first start
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: Scenario `viola run` start sequence to child spawn, a new values detail block)
**Change:**
- The scenario gains a values block for `run.pin_copy` under a concurrent first start: an absent copy is placed, never replaced. `pin_bytes`' private `place_absent` writes it through `viola_state::fs::place_private_new`: a temp file beside it, its mode set before any byte, closed, then persisted no-clobber.
- A start that loses the placement re-hashes the copy that stands: an equal copy is reused and `outcome` reads `ok`; a different one is `PinError::HashMismatch`, `outcome` `pinned-hash-mismatch`, the file left as found.
- The placement opens no span and writes no line or field of its own; the outcome set stays `ok|pinned-hash-mismatch`.
**Why:** the report's Summary and Symbols / APIs: the absent-copy write moved from `replace_private_shared` to a no-clobber placement of a closed temp file, so a concurrent first start's agent can no longer exec the copy while another start holds it open for write (`ETXTBSY`, 4 of 24 two-instance boots before, 0 of 24 after); its Schema / config and Coverage of new surfaces: `run.pin_copy` keeps `ok|pinned-hash-mismatch`, a lost placement reads `ok`, its instrumentation unchanged.
**Kept:** the standing `run.pin_copy` item (its outcome set and the `.cmd`/`.bat` refusal) is unchanged; the placement is stated in its own block beside it.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/
