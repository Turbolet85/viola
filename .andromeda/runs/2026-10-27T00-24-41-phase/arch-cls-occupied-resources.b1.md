# Classified — arch · occupied-resources.b1

## Answers
occupied-resources#1.1 · no · the socket dir, its 0600 chmod and the start arbiter are Not needed socket hardening, and the fix guards only the pinned copy
occupied-resources#2.1 · no · the wrapper channel and its server identity check are Not needed, since the fix sends no frame
occupied-resources#3.1 · no · standing S6 · a test-only binary or feature never reaches a release build
occupied-resources#4.1 · no · the subcommand set and argv shapes are untouched, since the chunk edits only run's start pin call and verify's pin_exe call
occupied-resources#5.1 · no · link and unlink are Not needed, and the chunk reaches neither verb
occupied-resources#6.1 · yes · run's and verify's home-tree refusals exit 1 before any log, so the absent-copy write stays behind check_home_tree over bin/ and plugin/
occupied-resources#7.1 · no · the hidden hook --capture arms are verify's probe capture, while the agent's hook spawn is the event arm
occupied-resources#8.1 · yes · --plugin-dir names the per-start plugin folder the chunk rewrites, passed first on every start, so the rewrite must hold for its reader
occupied-resources#9.1 · no · verify's probe plugin and interactive runs are other units of verify.rs, and the chunk edits only its pin_exe call, which runs before the probes
occupied-resources#10.1 · yes · settings.json is rewritten through replace_private, the helper the fix edits, so its 0600 whole rewrite before the spawn is a caller contract
occupied-resources#11.1 · no · VIOLA_DIR is set and read outside the pin path, and the chunk changes no env assignment
occupied-resources#12.1 · no · hook's instance check is hook's own code the fix does not touch, and its exit-0 contract is carried by the Hook Contract need
occupied-resources#13.1 · no · the R8 strip of the child's env is run's spawn code, which the chunk does not edit
occupied-resources#14.1 · no · the harness-only env vars are read by the test chain the new tests reuse unchanged, and a new test home's removal is standing S2
occupied-resources#15.1 · no · standing S6 · a test-only binary or feature never reaches a release build
occupied-resources#16.1 · no · config.json is read at the wrapper's start, which the chunk does not edit
occupied-resources#17.1 · no · standing S4 · a new log line goes only through obs_event! to the role's codes-only process log under diagnostics/
occupied-resources#18.1 · no · state-recovered lines come from replay readers the chunk neither changes nor edits
occupied-resources#19.1 · no · the diagnostics write-path checks in obs.rs are not reached by the pin fix
occupied-resources#20.1 · no · the instance file set and its entry checks are not what the pin fix edits, and run's start checks them through check_home_tree unchanged
occupied-resources#21.1 · yes · the write rule, the base of the read rule, is what a new .lock or temp entry under bin/<key>/ or plugin/<key>/ must pass
occupied-resources#22.1 · yes · check_home_tree with a key checks bin/<key>/, the pinned viola and plugin/<key>/ under the write rule, the check run's pin keeps ahead of its write
occupied-resources#23.1 · yes · run's start checks through check_home_tree before open_wrapper_log and starts no child on refusal, so the absent-copy write stays behind that check
occupied-resources#24.1 · no · the per-OS oracle binds only a test that plants a regular file above a checked path, which the scope does not do
occupied-resources#25.1 · no · board::scan's home rule is Not needed, owed to «Unix endpoint and home hardening»
