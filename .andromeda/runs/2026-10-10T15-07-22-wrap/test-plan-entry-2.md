
## 2026-10-10-viola-revive — the fake agent's resume payload and ten options; the root waits at 26 in 19 and the kill path's endpoint wait
**Section:** §7 Test Data & Fixtures (Fake agent: the hook-commands bullet, Modes) · §3 → 5-command implementation (`run` step 2)
**Change:**
- §7: under `--resume <id>` the fake agent sets `source` to `resume` and `session_id` to the id given on the recorded launch SessionStart, with `--fork-session` beside it the compiled id `0f0e0d0c-0b0a-4908-8706-050403020100`, the trailing newline kept; without `--resume` every payload is the recorded bytes (was: only UserPromptSubmit's `prompt` is set, and no other payload is built). Ten argv options (was eight; the list is the count's rule): eight for verify's runs, two for revive's tests.
- §7: one payload is set without a recorded fixture. No recorded SessionStart has source `resume`; the live payload on `claude` 2.1.287 holds 10 key names against the rewritten 5. No reading became a fixture or a ledger row; the recorded payload is owed to the route entry "Paste newline ledger row".
- Key file: `Instant::now() + WITHIN` reads 26 sites in 19 files (was 23 in 17). The kill path (`Wrapper::kill`) waits for the wrapper's exit, then for the child it recorded to be gone by pid and start time, then on `holder_gone` (on Unix a connect refused or the path absent, on Windows the pipe not found); the stop path keeps `unconnectable`.
**Why:** the two set fields are the operator's word at the chunk's P4. The plan's `wait_endpoint_gone` after a kill was disproved by measurement: a killed wrapper leaves its socket file on Unix, so a wait on the file being absent never ends.
**Kept:** the key file's `cleanup` sentence on a force-kill is unchanged: what the harness reads after a force-killed wrapper on Unix is not measured, and it is carried as a labelled hypothesis on the route entry "Unix endpoint and home hardening".
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/
