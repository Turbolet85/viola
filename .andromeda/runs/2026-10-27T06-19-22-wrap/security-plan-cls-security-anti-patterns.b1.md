# Classified — security-plan · security-anti-patterns.b1

## Answers
security-anti-patterns#1.1 · no · the Untouched section keeps client server verification and the peer check unchanged, and the viola-mcp change only wraps the stdin reader
security-anti-patterns#2.1 · yes · stale · the session changed viola-state fs.rs (the compare read), and this wipe re-creates its scratch through create_private_dir in that same reached file, whose protected DACL the Untouched section does not cover
security-anti-patterns#3.1 · no · its named keepers (tests/support/home.rs and the harness's ensure_e2e_home) sit in no reached file, and the reached tests add checks, no removal
security-anti-patterns#4.1 · no · the change reaches no ledger, stamp or verify file, so the stamp gate and its ledger rows stand
security-anti-patterns#4.2 · no · the Untouched section keeps the home strict-modes rows unchanged, and replace_private_shared's callers and write path are unchanged, so the check-before-create order stands
security-anti-patterns#5.1 · no · the session adds no env read, CLI flag or config key, and the Untouched section keeps every secret, seam and scan class unchanged
