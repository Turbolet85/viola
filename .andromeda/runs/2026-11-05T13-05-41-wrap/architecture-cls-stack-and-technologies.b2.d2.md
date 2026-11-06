@answer n=1 entity=stack-and-technologies#3.1 v=no
why: the Linux server check still compares the connected peer's pid and start time against the snapshot; the session removed only the macOS no-peer-pid arm and the peer_pid_reported parameter, so the sysinfo start-time observed side holds.
