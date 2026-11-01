# Classified — architecture · occupied-resources.b1

## Answers
occupied-resources#1.1 · yes · add-to · F13: a force-killed wrapper's socket file stays in the per-user directory until a same-name bind takes it over, and that 0700 directory is now verified at the bind and at every client (refused, never narrowed); the socket's shape, name and 0600 mode stand.
occupied-resources#2.1 · yes · add-to · F3 and F4: the peer euid check (server_euid, peer_admitted) joins the pid and start-time check before any frame on Linux and macOS, a euid mismatch reads server-verify-failed, and a refused socket directory is a further no-frame refusal.
occupied-resources#3.1 · yes · add-to · F14: harness cleanup's endpoint_gone on Unix goes through unconnectable in crates/viola-e2e/src/harness/cleanup.rs (connect refused or NotFound reads gone), a test-only harness path that the viola-harness binary drives.
occupied-resources#4.1 · yes · stale · viola revive --json's exit-1 codes move from ten to eleven with socket-dir-refused, so the "one of the ten codes" reading at line 30 changes.
occupied-resources#5.1 · yes · add-to · exit 21 gains a refused socket directory for link and unlink, beside the instance check, no-endpoint and wrong-process refusals.
occupied-resources#6.1 · yes · add-to · exit 21 for the six frame verbs gains a refused socket directory, and run and revive's start exit 1 gains it too, writing a process-exit line, so the "before any log" clause covers only the home-tree refusal.
occupied-resources#7.1 · no · the hook capture and statusline forms run before any channel, obs init or trust check, and the session changes only hook's event and dialog paths (F6), so the capture arms' claims hold in the reached src/cmd/hook.rs.
occupied-resources#8.1 · no · the child's flags and their order are unchanged; the new socket-directory check runs before the spawn and moves no argv in the reached src/cmd/run.rs.
occupied-resources#9.1 · no · verify's probe plugin, its hook commands and its five run directories and flags are not reached; verify takes no socket check.
occupied-resources#10.1 · no · the settings override is still written after the first snapshot and before the spawn, which the new check (run before the first snapshot) does not contradict; the override rule is unchanged.
occupied-resources#11.1 · no · the session reads no new variable and the VIOLA_DIR readers and the grandparent derivation do not move.
occupied-resources#12.1 · no · hook's VIOLA_DIR path through instance_of, canonical and trusted is unchanged, and the socket check comes after it, so the absent-instance "exits 0 and writes nothing" claim holds.
occupied-resources#13.1 · no · the CLAUDE* strip and the identity floor live in viola-agent-claude, which is not reached.
occupied-resources#14.1 · yes · add-to · F15: SOCKET_ROOT_VAR (TMPDIR on macOS, XDG_RUNTIME_DIR elsewhere) is read only by the test helper tests/support/home.rs, so it joins this test-harness-only variable list.
occupied-resources#15.1 · no · the two fake-agent seams (hold_pump_start in run.rs, panic_if_asked in seam.rs) are unchanged, and the session adds no env, flag or feature seam.
occupied-resources#16.1 · no · config.json's keys, bounds and once-at-start reads (run::budget and run::dialog) are unchanged; the session changes the board's home rule, not the config read.
occupied-resources#17.1 · no · the pinned bin/ copy, its 0700 mode and pin_bytes / run.pin_copy outcomes are not moved by any reached change.
occupied-resources#18.1 · yes · add-to · a refused socket directory writes one line: a frame verb's process-exit at WARN with exit_code 21 and detail socket-dir-refused during connect (no channel-request), the start's process-exit with exit_code 1, and hook's hook-decision at WARN with detail socket-dir-refused on the dialog path; the schema's process-exit enum gains the word, and MCP's frame writes no line.
occupied-resources#19.1 · no · a refused peer reads end-of-file with no line, and the reached schema changes only the process-exit (:78) and hook-decision (:53) enums, so the channel-response detail enum holds.
occupied-resources#20.1 · no · verify's refusal codes and its process lines are not reached (src/cmd/verify.rs is not in the reached set), and the schema change touches no verify code.
occupied-resources#21.1 · no · every replay reader still reads the snapshot and writes its one state-recovered line as before, and the new check reads only the recorded endpoint's directory.
occupied-resources#22.1 · no · check_diagnostics and its folder, file and symlink rules stay as they are in the reached strict.rs; the session adds check_socket_dir beside it (F2) and does not touch the diagnostics check.
occupied-resources#23.1 · yes · stale · viola list's home is now checked with the read rule on every OS (F10), so the row's "check_path on the home" reading moved.
occupied-resources#24.1 · yes · add-to · F10: board::scan holds the home to check_read on every OS while read_budget_stamped, mcp's start and plugin install keep the write rule on the home, and the enum gains NotADirectory (F2) beside Readable and Symlink.
occupied-resources#25.1 · no · check_home_tree, its per-tree rules (home read, the rest write) and its order are unchanged in the reached strict.rs, as the report states.
