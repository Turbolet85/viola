# Classified — arch · occupied-resources.b2

## Answers
occupied-resources#26.1 · no · budget.json, its lock and its checked read are the governor's and untouched (Not needed: budget.json); the lock-sibling rule the fix follows is the task's Needs entry, not this one
occupied-resources#27.1 · yes · the stamps envelope P3 reads for hash-keying is keyed by CLI version, written only by verify and read by run's version gate (`read_stamps_strict`), so P3's reading rests on it
occupied-resources#28.1 · no · verify's probe dirs and their drop guards are untouched; the chunk's verify edit is the pin_exe call, which writes the pinned copy outside `ledger/probes/`
occupied-resources#29.1 · yes · every wait of verify's run reads the stop flag, so a blocking lock or retry the fix adds on verify's pin_exe call must read it too
occupied-resources#30.1 · no · standing S3 · a test-only scratch path is registered in Occupied Resources, and viola never reads it
occupied-resources#31.1 · no · standing S2 · a test or harness home lives under target/e2e-home/viola-test-*/home or viola-session-*/home, removed only when its recorded owner process is gone
occupied-resources#32.1 · no · the chunk records, scrubs and adds no fixture; the committed sets and the fake agent's replays are not reached
occupied-resources#33.1 · no · standing S3 · a measurement or test-only scratch path is registered in Occupied Resources and gitignored, and viola never reads it
occupied-resources#34.1 · no · standing S2 · a new test or harness home lives under target/e2e-home/viola-test-*/home or viola-session-*/home, removed only when its recorded owner process is gone
occupied-resources#35.1 · no · the icacls trap needs a seeded home's inheritable grant through seed_conpty's hard links; the Windows race witness runs the unseeded product write path (Needs carve-out), so the trap is not reached
occupied-resources#36.1 · no · standing S3 · a measurement scratch path is registered in Occupied Resources and gitignored, and viola never reads it
occupied-resources#37.1 · no · run --mutants' scratch, its guard and its NOCOW rule are the harness's own; the chunk adds no scratch path there and changes no rule of that run
occupied-resources#38.1 · yes · a Windows witness on windows-2025 that reads the explicit owner-only DACL the fix keeps must accept LA in its SDDL oracle, never the user's SID
