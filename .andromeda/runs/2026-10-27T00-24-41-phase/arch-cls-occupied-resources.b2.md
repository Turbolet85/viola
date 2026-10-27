# Classified — arch · occupied-resources.b2

## Answers
occupied-resources#26.1 · yes · budget.json is replaced whole through replace_private at 0600, the helper this fix edits, so that mode and whole replacement must hold
occupied-resources#27.1 · yes · ledger/stamps.json is replaced through replace_private by update_stamps, which runs after verify's pin_exe, and its per-CLI-version keying is what the chunk's hash-keying reading of the readers measures
occupied-resources#28.1 · no · verify's probe dirs and their drop guards belong to runs A–E, which the pin_exe edit does not reach, and the pinned copy lives under bin/
occupied-resources#29.1 · yes · verify's stop flag is read by every wait of the run, so any wait the fix adds in verify's pin_exe absent-copy arm must read it too
occupied-resources#30.1 · yes · the two-sided race test's root waits on child starts are Watch sites (WITHIN 7 s, report kept on a panic), instances this site count must record
occupied-resources#31.1 · no · the dev host's tmpfs backing of target/e2e-home is an environment fact the chunk's test homes inherit through S2, and the chunk changes no link, keeper or home class
occupied-resources#32.1 · no · the chunk records or changes no fixture and relies on no new CLI behaviour, so the committed sets are replayed unchanged
occupied-resources#33.1 · yes · bin/agents-mode is replaced through replace_private at 0600, the helper this fix edits, so that mode and whole replacement must hold; the harness's own write order is not reached
occupied-resources#34.1 · no · standing S2 · a new test or harness home lives under target/e2e-home/viola-test-*/home or viola-session-*/home, removed only when its recorded owner process is gone
occupied-resources#35.1 · yes · the Windows race witness runs the unseeded product write path, and an inheritable icacls grant on its home would reach the shared conpty seed and widen every seeded home, so the witness must add none
occupied-resources#36.1 · no · the profraw census is a dev-host witness for the corrupt-profile mechanism, which the chunk neither runs nor changes
occupied-resources#37.1 · no · the mutants scratch re-creates through create_private_dir, a unit of fs.rs the chunk does not edit, and the chunk runs no mutation
occupied-resources#38.1 · no · the LA-alias DACL oracle belongs to strict-modes tests the chunk does not touch, and its Windows reading is the copy's content and sharing, not its ACL
