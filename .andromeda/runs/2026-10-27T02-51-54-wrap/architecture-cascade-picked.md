# architecture · picked accounting

## Picked
stack-and-technologies#1.9 · proposed · its one-shared-helper row omits the new no-clobber `persist_noclobber` placement the pinned copy now takes (`place_private_new`)
stack-and-technologies#1.18 · not touched · no MCP surface, rmcp feature or dependency moved; `mcp_tools.rs` only lost lines and its client-only rmcp claim still holds
established-decisions#6.1 · proposed · it lists the pinned copy among `replace_private_shared`'s files and claims ONE helper; the copy is now placed by `place_private_new` (F1, F2, F3 beside it)
established-decisions#27.1 · proposed · measurement B (F5) qualifies its content-hash-keys-the-copy rationale: one tree built two files and can key two folders
conventions#2.1 · not touched · a lost placement over different bytes is a pinned copy failing its re-hash, the same `pinned-hash-mismatch` text and exit 1 (report: `conventions.md:23` no-change)
occupied-resources#3.1 · proposed · the fake agent it describes now carries `spawn_os_error` on its `hook` receipt (F7)
occupied-resources#19.1 · not touched · `src/obs.rs` is not reached and `strict.rs` gained only a `cfg(windows)` test module, so the diagnostics-open check holds
occupied-resources#20.1 · proposed · its Windows plain-home reading names an inherited Users (`S-1-5-32-545`) write entry, disproved at ci#40425866096 (Authenticated Users held the write)
occupied-resources#21.1 · not touched · the read rule's product code in `strict.rs` is unchanged; the chunk added only a `cfg(windows)` test helper
occupied-resources#24.1 · not touched · the path-under-a-regular-file oracle is no surface of this chunk and the report moves none of its readings
occupied-resources#25.1 · not touched · `board::scan` is not reached and the strict rules' product bodies are unchanged, so the two home checks still disagree as stated
occupied-resources#35.1 · not touched · no `seed_conpty` or test-home seeding changed; the report reads no icacls grant through the seed
occupied-resources#37.1 · not touched · its BUILTIN\Users reading is the workspace drive's scratch at ci#38375200217, not the temp dir the report measured
occupied-resources#38.1 · proposed · F6, a runner-dependent SID oracle read by its right, sits beside its LA-alias oracle lesson
