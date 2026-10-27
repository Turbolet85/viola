# arch · picked accounting · group 1 of 4

## Picked
stack-and-technologies#1.1 · carried · the toolchain pin is an input of the two clean release builds the chunk measures, and the 1.96 floor bounds any std API (`File::lock`) a race guard adds
stack-and-technologies#1.2 · carried · the race fix on the `run` and `verify` paths, inside the sync crate `viola-state`, must use std threads and blocking I/O only
stack-and-technologies#1.4 · carried · the chunk edits `pin_companions`, which writes the vendored ConPTY pair embedded in `viola`, and that embedded pair is part of the release file the measurement compares
stack-and-technologies#1.7 · not in scope · the chunk adds no store and writes no log or snapshot; the pinned copy is neither, a re-check every change owes
stack-and-technologies#1.9 · carried · the chunk edits `replace_private_shared` / `replace_private`, the one tempfile `persist` helper, and any guard it adds is a std `File::lock` on a separate `.lock` file
stack-and-technologies#1.10 · carried · the truncated SHA-256 keys `bin/` and `plugin/` and the re-hash before reuse that the chunk re-asserts, and the key the reproducible-build measurement says a rebuild may move
stack-and-technologies#1.15 · not in scope · standing S7 · a new error type follows the per-crate `<Crate>Error` thiserror enum with fixed messages
stack-and-technologies#1.21 · carried · the chunk edits the per-start plugin rewrite, which writes out the plugin compiled into the binary, and the install path is the build whose reproducibility is measured
stack-and-technologies#1.22 · carried · the scope reads Windows on the CI runner only and leaves open, as P4's fork, whether CI's three OSes are read for the measurement
stack-and-technologies#1.23 · carried · the measurement runs through `scripts/release-check.sh`, which refuses `tool-missing: jq` without the runner-provided `jq`
