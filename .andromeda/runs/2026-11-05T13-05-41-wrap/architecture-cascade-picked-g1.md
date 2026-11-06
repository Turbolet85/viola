@account n=1 entity=stack-and-technologies#1.1 v=proposed
why: the row names the binary `viola` / `viola.exe`; with Windows code removed (Retired) only `viola` is built
@account n=2 entity=stack-and-technologies#1.4 v=proposed
why: windows-sys, the ConPTY sideload, the console readers and the TerminateProcess fallback are removed and libc is no longer Unix-only (Symbols / APIs, Dependencies)
@account n=3 entity=stack-and-technologies#1.6 v=proposed
why: the Windows named pipe, its SQOS client open and the SDDL DACL are retired; the endpoint is a Unix socket only
@account n=4 entity=stack-and-technologies#1.9 v=proposed
why: the windows-sys owner/DACL arm and the Windows append-lock parenthesis left with the Windows code; replace_private is one attempt
@account n=5 entity=stack-and-technologies#1.10 v=proposed
why: the Windows x64 ConPTY companion re-hash (pin_companions) is removed with the sideload
@account n=6 entity=stack-and-technologies#1.22 v=proposed
why: the CI matrix went from windows-2025 / macos-latest / ubuntu-latest to `[ubuntu-latest]`, so «all three OSes» is stale
@account n=7 entity=stack-and-technologies#1.23 v=not_touched
why: the report moves none of its tools or versions (Dev-tool versions: none); install-ripgrep.sh and orphans-check.sh still install and refuse as stated, and the Windows-host hyperfine clause is a dated measurement recording history
@account n=8 entity=stack-and-technologies#2.1 v=proposed
why: the Windows GetNamedPipeServerProcessId pid, the macOS no-pid gap and the none-on-Windows euid clause are retired; SO_PEERCRED pid and euid stand
@account n=9 entity=stack-and-technologies#3.1 v=not_touched
why: the server check still compares the peer's pid and start time with the snapshot's; the report removes only the macOS no-pid arm and the peer_pid_reported parameter
