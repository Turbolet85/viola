@answer n=1 entity=integration-test-strategy#1.1 v=no
why: the module-to-module approach names no OS, no retired surface and no count, and its hook parse, core and state path still stands
@answer n=2 entity=integration-test-strategy#1.2 v=yes mode=stale
why: the concurrent-append tests are said to run on all 3 CI OSs, and the session retired the Windows and macOS legs so they run on ubuntu-latest alone
@answer n=3 entity=integration-test-strategy#1.3 v=yes mode=contradict
why: the Windows npm shim as a .cmd fixture beside a stub claude.exe is retired with the npm-shim resolution and the batch-script-child refusal
@answer n=4 entity=integration-test-strategy#1.4 v=yes mode=contradict
why: the Windows pipe DACL read-back and the pipe admission test tests/security_negatives_channel.rs are retired with the Windows named pipe
@answer n=5 entity=integration-test-strategy#1.5 v=yes mode=contradict
why: the ConPTY spawn, the Windows .cmd child refusal, the sideloaded ConPTY on windows-2025 and its measurements are retired with the ConPTY sideload and the Windows CI legs
@answer n=6 entity=integration-test-strategy#1.6 v=no
why: the hook_fail_open.rs claim names the Env::Made setup, the no_instance_dir row and the four-event rstest, which the ungating of the macOS cfg_attr pair leaves standing
@answer n=7 entity=integration-test-strategy#2.1 v=yes mode=contradict
why: the placed-copy witness is said green on macos-latest, a CI leg the session retired, and its Windows form on windows-2025 is retired with the Windows code
@answer n=8 entity=integration-test-strategy#2.2 v=yes mode=stale
why: the cleanup witness is said to run on both Unix runners, ubuntu-latest and macos-latest, and the session retired the macos-latest leg so the count is one
@answer n=9 entity=integration-test-strategy#3.0 v=yes mode=contradict
why: the Windows x64 seed_conpty carve-out, the conpty_sideload.rs unseeded-start claim, the Windows plain-home refusal and the windows-2025 runner-inherited grants are retired with the ConPTY sideload and the Windows code
@answer n=10 entity=integration-test-strategy#4.1 v=yes mode=contradict
why: the macOS EPIPE and ENOTCONN close readings, the server-identity negatives said to run on Linux and Windows with a macOS carve-out, the MCP euid mapping on Windows and macOS, and the Windows pipe Serving note are retired
@answer n=11 entity=integration-test-strategy#4.2 v=yes mode=contradict
why: the claims that the strict-modes checks hold on all 3 OSes, the Windows Users read grant refusal and the cfg(windows) protected-DACL test are retired with the Windows owner/DACL strict-modes arm and the Windows CI legs
@answer n=12 entity=integration-test-strategy#5.1 v=yes mode=contradict
why: the Windows relied count of 21, the [&str; 21] ROW_IDS literal, the 19 pass 2 fail Windows stamp, the Windows console-routine stop cases, the all-3-OS output and slash-args witnesses, and the batch-script-child exit code are retired
