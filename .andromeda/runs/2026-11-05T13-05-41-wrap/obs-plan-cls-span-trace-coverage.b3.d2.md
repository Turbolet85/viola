@answer n=1 entity=span-trace-coverage#15.1 v=yes mode=stale
why: viola-state fs.rs is reached whole and the lock-held append that writes the torn-line heal may sit in it, so the heal claim cannot be cleared on the changed file
@answer n=2 entity=span-trace-coverage#16.1 v=no
why: the ledger's strict-modes refusal stays on Linux because the change removes only the Windows owner/DACL arm of the strict check, and the Unix refusal and its parse-rejected detail still hold
@answer n=3 entity=span-trace-coverage#17.1 v=yes mode=contradict
why: the change retires the console-control stop arm (Ctrl-C and Ctrl-Break, Windows close, logoff and shutdown) and makes LedgerRow::relied() all 24 rows on every build, so the 24-on-Unix and 21-on-Windows relied count and its WRAPPED_HERE filter no longer hold
