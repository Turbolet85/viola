# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · the module-to-module approach, its example chain (hook parse, core event kind, state append) and cargo-nextest with rstest 0.27.0 are unchanged by the placement
integration-test-strategy#1.2 · no · tempfile 3.27.0 is unchanged (the placement reuses its persist_noclobber already present) and the concurrent-append check and File::lock contention hold on the changed files, CI ci#40439188053 green 15/15
integration-test-strategy#1.3 · no · the fake agent's agents modes (recorded, oversize, malformed) and the Windows .cmd shim fixture hold on the changed binary, whose only change is the hook receipt, CI ci#40439188053 green 15/15
integration-test-strategy#1.4 · no · the pipe's canonical-SDDL read-back, the Unix socket 0700/0600 metadata reads and windows-sys 0.61.2 hold on the changed files, the new helper serving only the plain-home oracle, CI ci#40439188053 green 15/15
integration-test-strategy#1.5 · yes · add-to · F1, a filesystem-placement forced-window witness (place_private_new_placed_copy_execs_at_once on Unix, its Windows form green-only), joins the PTY and tui forced-window witnesses here
integration-test-strategy#1.6 · no · viola hook's instance check, event line, hook-<name>.ndjson and budget.json are unchanged, and the session's hook receipt is the fake agent's own spawn record, not viola hook
integration-test-strategy#2.0 · yes · stale · the plain-home refused entry is Users on one windows-2025 runner but Authenticated Users (S-1-5-11) on another (F6), so the oracle now reads the inherited untrusted write grant by its right
integration-test-strategy#3.1 · no · the channel verbs, paste validation, Percent fallbacks and link logging are unchanged, and no channel test or channel code is reached, CI ci#40439188053 green 15/15
integration-test-strategy#3.2 · no · the refusal shapes, the widened home-tree and pinned-copy refusals and the protected home DACL hold on the changed files, the session changing only the plain-home oracle and the copy's placement, CI ci#40439188053 green 15/15
integration-test-strategy#4.1 · no · the verify literal lines, the fake agent's print mode, its receipt's cwd and prompt lines and the ledger row counts are unchanged, CI ci#40439188053 green 15/15
