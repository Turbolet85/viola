proposals:
  - detector: D-new-detail
    severity: warning
    fact: F4
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 110
    section: "## Occupied Resources"
    change: "A new `<!-- detail: values · res-filesystem -->` block after the `config.json` block: - `bin/<version>-<hash>/viola(.exe)`, the pinned copy (0700 on Unix): when it is absent, the `NotFound` arm of `viola_state::pin::pin_bytes` places it through `pin::place_absent`, a no-clobber placement of a closed temp file (`viola_state::fs::place_private_new` at `DIR_MODE`), so the copy is placed once and never replaced. `Written` is ok. `Present` (a concurrent first start placed the copy first) re-hashes the copy that stands: equal bytes are reused, different bytes are `PinError::HashMismatch` with the file left as found and the start refused `pinned-hash-mismatch` (exit 1). `run.pin_copy` keeps `ok|pinned-hash-mismatch`, and a lost placement with equal bytes logs `ok`; no line, field or code is added."
    sidecar: "occupied-resources res-filesystem: new record for the pinned copy's no-clobber absent-copy placement and the lost placement's re-hash (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds)"
    rationale: "The report's Symbols / APIs: the NotFound arm of pin_bytes now calls place_absent (pin.rs:108-116), Written ok, Present re-hashes the standing copy, equal reused, different PinError::HashMismatch with the file left as found; Schema / config: run.pin_copy keeps ok|pinned-hash-mismatch and a lost placement reads ok. The detail file holds no pinned-copy record, so the body's Filesystem line has no detail beside it; placed after config.json as the body orders it."
    basis: crates/viola-state/src/pin.rs:108
