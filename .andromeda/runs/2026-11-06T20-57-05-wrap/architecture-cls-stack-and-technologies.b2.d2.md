@answer n=1 entity=stack-and-technologies#2.1 v=no
why: interprocess stays at 2.4.4 and the client-side SO_PEERCRED reads are untouched, so the reached client.rs change (Windows and EXE_SUFFIX removal) leaves this claim holding
@answer n=2 entity=stack-and-technologies#3.1 v=no
why: sysinfo is not in the changed Cargo.lock set and Session liveness is re-witnessed unchanged, so the process start-time check still holds
