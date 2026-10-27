# security-plan · picked accounting

## Picked
threat-model-summary#6.1 · proposed · the MCP-over-stdio vector gains the stdin line cap: an over-long driver line ends the session, never dispatched in part (F8)
input-validation#1.4 · not touched · the hook stdin cap and fail-open path are unchanged; the report only re-ran their witnesses and lists the capture arms in the enumeration
input-validation#1.5 · proposed · the MCP tool-inputs row stands, and the new `viola mcp` stdin row (F1, F2, F3) is added beside it as its nearest neighbour
input-validation#1.15 · proposed · the passthrough after a caught feed panic is now witnessed on the outer terminal (F9); its panicked / oversize detail already reads true
input-validation#2.0 · proposed · the Constants list gains the `viola mcp` stdin lines, and the compare-read bound, the enumeration and its own-exe exceptions land beside it (F4, F6, F7)
data-protection#1.1 · not touched · `replace_private_shared` still counts a failed replace done only when the file holds exactly the bytes; the bounded compare gives the same outcome and callers are unchanged
data-protection#2.1 · not touched · the no-clobber placement and `replace_private_shared` keeping only the plugin files and companions are unchanged in the reached `fs.rs`
data-protection#3.1 · not touched · `place_private_new` and its `Placed::Present` handling were not changed; only `holds_exactly` was added in `fs.rs`
data-protection#4.1 · not touched · `pin_bytes`'s re-hash of a lost placement is unchanged; the report names no change to it
dependency-security#2.1 · proposed · the deny-probes verdict moves from 13/13 to 14/14, and the new serde_json feature ban the probes prove lands beside it (F5)
error-handling#1.1 · not touched · the hook fail-open paths are unchanged, and the session added no error variant or `isError` shape
security-anti-patterns#2.1 · not touched · the `run --mutants` scratch-wipe guard is a different subject, and the report never reaches it
