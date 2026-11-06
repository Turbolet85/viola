@carried n=1 label=architecture/crosscut-config fact=fct-k6htnrfj
why: the chunk reads a dialog hold near the 3600 s ceiling live and settles that PROVISIONAL ceiling, so the hold it measures must read this key's range and default.
quote: `dialog_hold_s` (the dialog hold as a `DialogHold`, whole seconds 60–3600, default 60)
@carried n=2 label=architecture/crosscut-config fact=fct-k6htnrfj
why: the move off `portable-pty =0.8.1` brings a new dependency graph into the product, and no crate it brings may read an environment variable or build a Tokio runtime on the sync paths.
quote: a dependency or feature that reads an environment variable breaks this rule like a direct read does.
@carried n=2 run=2 label=architecture fact=fct-dkudywpi
quote: There are no secrets in v1, and environment variables are not a configuration channel: a dependency or feature that reads one breaks this rule like a direct read does.
@carried n=2 run=3 label=architecture fact=fct-me2myqzf
quote: New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime.
@carried n=3 label=architecture/crosscut-config fact=fct-k6htnrfj
why: item 17 sweeps the Windows remnants of `src/cmd/mod.rs` with the masters following, so this Windows clause of the master is removed with its code or kept with its stated reason.
quote: `canonical_home` canonicalises a home that exists (`std::fs::canonicalize`, a Windows `\\?\C:\…` written back as `C:\…`, every other verbatim form kept)
@carried n=4 label=architecture/crosscut-bounded-readers fact=fct-qip7sr3o
why: `crates/viola-pty` is reached whole by the `portable-pty` move, so any reader the new release's API reshapes there must stay bounded in its statement or be a named exception, the census green at the wrap.
quote: Every product reader of external bytes is bounded in the same statement (`take`) or named in an `EXCEPTIONS` row; none reads an external byte source to its end unbounded.
@carried n=4 run=2 label=architecture fact=fct-tpj3dxid
quote: Every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounded_readers.rs` enumerates the boundaries and censuses the product sources.
@carried n=5 label=architecture/crosscut-sanitised-error-surfaces fact=fct-7ok4646z
why: folding or ratifying `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`, and re-measuring the derive census, reshape `thiserror` enums whose messages must still interpolate nothing and whose writers must stay named.
quote: A census, `product_error_writers_are_each_named`, covers the product's error writers, and `thiserror_messages_interpolate_nothing` holds that a `thiserror` message interpolates nothing.
@carried n=6 label=architecture/crosscut-sanitised-error-surfaces fact=fct-7ok4646z
why: the socket-directory pair's wording is one of the four PROVISIONAL items brought to the founder's word here, and any wording he rules keeps these bounds.
quote: The `unable:` / `hint:` pair for a socket directory names only the parsed `ViolaName`, no path, pid or uid, is plain ASCII with `hint:` last and no SGR, its hint `send_hint`'s cause `socket-dir` in `src/human/hints.rs`.
@carried n=7 label=architecture/crosscut-os-gated-tests fact=fct-xxlqtlbc
why: `v1-53`'s claim re-reads this census (39 OS-gated test functions, 2 gated files), and item 17's remnant removal must move each `GATES` row with its gate in the same change.
quote: A gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a `GATES` row names it, which `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named` proves fires; the planted Windows cases build the word from a `WINDOWS` const. `os_gated_tests_are_each_named_with_their_reason` fails both ways, on a gate no row names and on a row whose gate no longer stands
@carried n=7 run=2 label=architecture fact=fct-fa2klbw5
quote: The product builds and is tested on Linux (CI on `ubuntu-latest`); every OS-gated test names its reason in the census binary `tests/contract_os_gated_tests.rs`, and a gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a row names it.
@binding n=1
**cross-cutting-patterns (config) ↔ Established Decisions (Hook Contract) and Occupied Resources (`config.json`, `schemas/config.v1.json`)**: the dialog-hold ceiling the live hold settles must read the same 60–3600 s range and 60 s default in all four places, and the PROVISIONAL mark must leave all of them together (architecture/crosscut-config fct-k6htnrfj) (architecture fct-4nt6qtyp) (architecture fct-i4wacyrp) (architecture fct-6e4hf2ai).
@binding n=2
**cross-cutting-patterns (config, Tokio containment) ↔ Established Decisions (PTY, Concurrency KEYSTONE)**: the `portable-pty` release adopted must drop `pin-utils` with no ignore-list widening, and its graph must also read no environment variable and bring no Tokio into the sync crates. The PTY decision's `=0.8.1` text and owner line follow the move (architecture/crosscut-config fct-k6htnrfj) (architecture fct-omwrdnlc) (architecture fct-l5jpezvw).
@binding n=3
**cross-cutting-patterns (OS-gated tests) ↔ body (Linux only) and the item 17 remnant sweep**: removing a Windows-gated test or file removes its `GATES` row in the same change. The census's planted `WINDOWS`-const cases are its own self-check and stay, kept with that reason. The count `v1-53`'s claim cites is re-read at HEAD after the sweep (architecture/crosscut-os-gated-tests fct-xxlqtlbc) (architecture fct-igbvtfv6).
@binding n=4
**cross-cutting-patterns (sanitised error surfaces) ↔ Established Decisions (Error Handling) and Conventions (Rust error types)**: each fold or ratification of the open divergences keeps every `thiserror` message free of interpolation and every error writer named. The derive count re-measured here reads one number in the decision, the Conventions line and the Inherited Defaults Errors line (architecture/crosscut-sanitised-error-surfaces fct-7ok4646z) (architecture fct-sha7o35z) (architecture fct-nhsg4k7g) (architecture fct-vf75c7sp).
@binding n=5
**cross-cutting-patterns (config, Windows clause) ↔ Established Decisions (CLI Conventions)**: the masters follow item 17's removals in `src/cmd/mod.rs`. Once its code is gone, the `canonical_home` Windows verbatim-form clause and the Git Bash rewritten-path warning `plain_drive` served each leave the master, or are kept with a reason (architecture/crosscut-config fct-k6htnrfj) (architecture fct-nvft5tqc).
