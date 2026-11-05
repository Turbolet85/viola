@answer n=1 entity=test-strategy#1.1 v=yes
why: names the macOS carve-outs and the all-3-OS run that the chunk removes with the cfg(target_os = "macos") arms (task_items 11 and 13)
@answer n=2 entity=test-strategy#1.2 v=no
why: generic Standard Contract suite home naming no OS arm, and the kept Linux suites still meet it
@answer n=3 entity=test-strategy#1.3 v=no
why: E2E harness rule names no OS arm, ConPTY or sideload, and the Linux paths it covers are unchanged
@answer n=4 entity=test-strategy#1.4 v=yes
why: names the time-boxed ubuntu cargo-fuzz nightly job and corpus replay gate that the nightly.yml and ci.yml edits must keep on ubuntu (task_item 4)
@answer n=5 entity=test-strategy#1.5 v=yes
why: names the per-OS perf CI job whose Windows and macOS legs leave with ci.yml (task_item 1)
@answer n=6 entity=test-strategy#1.6 v=no
why: chaos faults are Linux process and file operations with no OS-gated arm named, and the .cmd chaos case is handled as task_item 10
@answer n=7 entity=test-strategy#1.7 v=yes
why: its known survivor is the euid wiring of the three client seams, which task_item 20 says no committed test or seam may force
@answer n=8 entity=test-strategy#1.8 v=yes
why: the statusline replay test is Unix-gated, and task_item 12 makes the statusline pass-through Unix gate unconditional once its Windows arm leaves
@answer n=9 entity=test-strategy#2.1 v=yes
why: names the icacls Windows arm of home.rs, the Windows debugger attach of frozen.rs and the macOS TMPDIR socket-root arm, all cfg arms the chunk removes (task_items 8 and 11)
@answer n=10 entity=test-strategy#2.2 v=yes
why: contract_os_gated_tests.rs is the OS-gated census the chunk edits (task_file n=11), and this rule fixes its contract_ binary home and its run under --e2e
@answer n=11 entity=test-strategy#3.1 v=yes
why: names the Windows listener remote-client and DACL denial tests and the server-identity test off macOS, which task_items 9 and 11 remove
@answer n=12 entity=test-strategy#3.2 v=no
why: paste-injection rule names no OS arm and no removed test, and the kept Linux suites meet it
@answer n=13 entity=test-strategy#3.3 v=no
why: hook stdin row names no OS arm, and hook exit behaviour is unchanged on Linux and not stated here
@answer n=14 entity=test-strategy#3.4 v=no
why: MCP E2E row names no OS arm or removed test
@answer n=15 entity=test-strategy#3.5 v=no
why: CLI and env row names no OS arm or removed test
@answer n=16 entity=test-strategy#3.6 v=yes
why: names the Windows --home protected DACL read back through icacls which task_item 8 removes
@answer n=17 entity=test-strategy#3.7 v=no
why: child-spawn row names no OS arm, and the ConPTY-hosted children of task_item 7 are not stated here
@answer n=18 entity=test-strategy#3.8 v=yes
why: names cargo deny, the sole-root tokio ban and the ban probes, which task_item 17 keeps green once the Windows dependencies leave
@answer n=19 entity=test-strategy#3.9 v=no
why: property-test row locates the fuzz job by name only, the ubuntu fuzz job stays unchanged and is covered by test-strategy#1.4
@answer n=20 entity=test-strategy#3.10 v=no
why: chaos trigger row is a location pointer with no OS-gated arm
@answer n=21 entity=test-strategy#3.11 v=no
why: performance-budget row is a location pointer to §10 with no OS arm, and the per-OS perf job is covered by test-strategy#1.5
@answer n=22 entity=test-strategy#3.12 v=no
why: cross-surface row names no OS arm or removed test
@answer n=23 entity=test-strategy#3.13 v=yes
why: multi-os-compat row names the 3-OS CI matrix and the OS-gated census contract_os_gated_tests.rs that the chunk re-points or retires (task_file n=11, task_items 5 and 15)
@answer n=24 entity=test-strategy#3.14 v=no
why: multi-version-compat row names no OS arm or removed test
@answer n=25 entity=test-strategy#3.15 v=no
why: contract-sandbox row names no OS arm, and the fixtures it points to are unchanged
