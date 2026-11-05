@answer n=1 entity=e2e-test-strategy#6.3 v=yes
why: names the human-wins invariant and typed exits 10, 13 and 20, all named in the scope's Linux-behaviour item
@answer n=2 entity=e2e-test-strategy#6.4 v=no
why: cleanup step only (Ctrl-C, child wait, TempDir drop); no OS gate and no named behaviour
@answer n=3 entity=e2e-test-strategy#7.1 v=yes
why: names the Unix statusline pass-through arm and the tests/support/home.rs plant helpers the chunk edits
@answer n=4 entity=e2e-test-strategy#7.2 v=yes
why: states the Windows statusline arm that runs no command and the Unix-only marker equality, both in the chunk's scope
@answer n=5 entity=e2e-test-strategy#7.3 v=yes
why: states the cfg(not(unix)) statusline case on windows-2025 and per-OS CI legs that the chunk removes
@answer n=6 entity=e2e-test-strategy#7.4 v=no
why: cleanup step only (harness cleanup); no OS gate and no named behaviour
@answer n=7 entity=e2e-test-strategy#8.1 v=no
why: Path 7 boot steps with no OS gate; the version gate is not_needed and no edited unit is named
@answer n=8 entity=e2e-test-strategy#8.2 v=yes
why: states answer exit 12 (unverified-cli), a value in the scope's typed exit set 0, 1, 2, 10-14, 20, 21 that Linux must keep
@answer n=9 entity=e2e-test-strategy#8.3 v=yes
why: states the MCP leg green on all three CI OSes, a per-OS leg that the masters' OS-target amendment rewrites
@answer n=10 entity=e2e-test-strategy#8.4 v=no
why: cleanup step only (harness cleanup, TempDir drop for H3); no OS gate and no named behaviour
@answer n=11 entity=e2e-test-strategy#9.1 v=yes
why: names the 11 identity-floor names of the R8 strip, which the scope names as measured on the Windows host
@answer n=12 entity=e2e-test-strategy#9.2 v=yes
why: states the Unix claude_env_keep survival and the Windows handle non-inheritance assertion, both named in the scope
@answer n=13 entity=e2e-test-strategy#9.3 v=no
why: cleanup step only (TempDir drop); no OS gate and no named behaviour
@answer n=14 entity=e2e-test-strategy#10.1 v=yes
why: states a three-CI-runner green leg (run 38098238731), a per-OS leg the test-plan OS-target amendment rewrites; the E3 scenario itself is not_needed
@answer n=15 entity=e2e-test-strategy#11.1 v=no
why: E5 snapshot replay readers with no OS gate, no Windows or macOS text, and no behaviour the scope names
@answer n=16 entity=e2e-test-strategy#12.1 v=yes
why: names the cfg_attr(not(target_os = "macos")) impostor rows and the Windows debug-build timing, both removed arms
@answer n=17 entity=e2e-test-strategy#13.1 v=yes
why: names the harness-<os> per-OS artifact names and the ci.yml artifact upload that the scope's CI job edits remove
@answer n=18 entity=e2e-test-strategy#14.1 v=yes
why: names the Windows icacls strict-modes case, the .cmd human pair and the macOS euid carve-out, all removed arms
@answer n=19 entity=e2e-test-strategy#15.1 v=yes
why: names the Windows SQOS, pipe-rejection and DACL negatives and the macOS-carved server-identity and socket-dir tests
@answer n=20 entity=e2e-test-strategy#16.1 v=yes
why: its Windows terminal case and Unix-only no-ESC assertion are a cfg(windows) arm of cross_list_rows that the chunk removes
@answer n=21 entity=e2e-test-strategy#16.2 v=yes
why: a cfg(unix) stale-row dim test graded green on macos-latest, a macOS leg the chunk removes
@answer n=22 entity=e2e-test-strategy#17.1 v=yes
why: names the Windows-only .cmd case, the Windows vt100 literal-absence branch and the OS-specific output-outlives leg
@answer n=23 entity=e2e-test-strategy#17.2 v=yes
why: the fake agent's SIGHUP-ignoring exit is a cfg(target_os = "linux") gate the scope makes unconditional, and it names macos-latest
@answer n=24 entity=e2e-test-strategy#18.1 v=no
why: property suite parser targets and proptest settings are not_needed; no OS gate and no Windows text
@answer n=25 entity=e2e-test-strategy#19.1 v=yes
why: names contract tests green on all three OSes, the census of OS-gated contract tests and the deny.toml serde_json ban the scope edits
