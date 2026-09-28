# layout-templates — amendments

History of amendments to `.andromeda/layout-templates.md`, one entry per amendment (form: `/andromeda-wrap-session` `references/sidecar-contract.md`). The body holds only current truth.

## 2026-09-28-capability-ledger-and-viola-verify — `viola verify`'s output structure and its refusal pairs
**Section:** Surface: cli Output structure — `viola verify`; Component — Primary content block 2 (refusal lines)
**Change:**
- The `viola verify` wireframe shows the six ledger rows `[01/06] shim-resolution …` … `[06/06] largest-hook-payload …`, the last stdout line `stamped 2.1.283  6 pass  0 fail`, a failing-row run (`4 pass  2 fail`, exit 1, the stamp still written) and the usage `viola verify [--record <DIR>] [-- <program> [args…]]`; `MM` grows as owning chunks land rows (was the 14-row `[01/14] S3 …` / `stamped 2.1.280  14 pass` sketch).
- The fixed-message exit-1 exceptions add `viola verify`'s four `unable:`/`hint:` pairs (CLI not found, `.cmd`/`.bat`, unreadable version, a dirty recording), run's pinned-copy refusal, and a failed verify's exact `error: internal error`.
**Why:** the chunk landed the verb with those lines (report Symbols; `tests/cli_verify.rs`).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
