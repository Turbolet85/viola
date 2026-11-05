@answer n=1 entity=span-trace-coverage#5.1 v=no
why: the pin_copy placement lives in viola_state fs, not a reached file, and states the Linux no-clobber placement the chunk does not change
@answer n=2 entity=span-trace-coverage#6.1 v=yes
why: the server check names the macOS euid-only carve-out, which scope item 10 says leaves with the macOS arm
@answer n=3 entity=span-trace-coverage#6.2 v=no
why: required span attributes of the send path, OS-free and untouched by the removal
@answer n=4 entity=span-trace-coverage#6.3 v=yes
why: the client-side server-verification refusal line names the macOS euid-only carve-out the chunk removes
@answer n=5 entity=span-trace-coverage#6.4 v=yes
why: the server-verify-failed exit-21 cleanup names the macOS euid-only carve-out the chunk removes
@answer n=6 entity=span-trace-coverage#7.1 v=yes
why: the wait and last server check names the macOS carve-out on the server-verification path scope item 10 names
@answer n=7 entity=span-trace-coverage#7.2 v=no
why: wait.block and run.wait_dispatch span attributes, OS-free and untouched by the removal
@answer n=8 entity=span-trace-coverage#7.3 v=yes
why: wait and last log lines carry server-verify-failed and the macOS carve-out that scope item 10 names
@answer n=9 entity=span-trace-coverage#7.4 v=yes
why: wait.block closes on a server-verification refusal, a path scope item 10 names
@answer n=10 entity=span-trace-coverage#8.1 v=yes
why: the dialog and answer server check names the macOS euid-only carve-out the scope removes
@answer n=11 entity=span-trace-coverage#8.2 v=no
why: dialog span attributes, OS-free and not touched by the removal
@answer n=12 entity=span-trace-coverage#8.3 v=yes
why: the server-verify-failed fail-open and CLI answer refusal lines, a detail scope item 10 says stays unchanged on Linux and the server check edit reaches
@answer n=13 entity=span-trace-coverage#8.4 v=no
why: dialog hold cleanup and dialog-expired, with no OS gate or server check
@answer n=14 entity=span-trace-coverage#9.1 v=yes
why: the pause and release server check names the macOS euid-only carve-out the scope removes
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: wheel transition span attributes, OS-free and untouched
@answer n=16 entity=span-trace-coverage#9.3 v=yes
why: pause and release log lines carry server-verify-failed, a detail scope item 10 says stays unchanged on Linux
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: wheel point-in-time span and product event, OS-free and untouched
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: budget governor spans, which scope item 13 says the OS removal does not touch
@answer n=19 entity=span-trace-coverage#10.2 v=no
why: budget span attributes, OS-free and untouched
@answer n=20 entity=span-trace-coverage#10.3 v=no
why: statusline shell-out log lines, which scope item 13 says the OS removal does not touch; the Windows clause is the wrap's amendment
@answer n=21 entity=span-trace-coverage#10.4 v=no
why: statusline shell-out bound and cleanup, untouched by the removal
@answer n=22 entity=span-trace-coverage#11.1 v=yes
why: the E2 env strip's Windows arm (registry persistent set, case-insensitive match) is a cfg(windows) unit the removal deletes, and its Unix rule must hold through it
@answer n=23 entity=span-trace-coverage#12.1 v=no
why: the implicit link record on send and answer, OS-free and untouched
@answer n=24 entity=span-trace-coverage#13.1 v=no
why: explicit link and unlink methods, OS-free and untouched
@answer n=25 entity=span-trace-coverage#14.1 v=no
why: state-recovered replay lines, OS-free and untouched
