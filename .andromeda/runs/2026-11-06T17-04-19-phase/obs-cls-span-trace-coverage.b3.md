@answer n=1 entity=span-trace-coverage#15.1 v=no
why: the torn-append heal is the state-store appender's behaviour on events.ndjson, and the chunk touches no state-store reader or appender (not_needed item 5)
@answer n=2 entity=span-trace-coverage#16.1 v=yes
why: the live viola run passes run's version gate, which reads the ledger and carries cli_verified on process-start{subject:"claude-child"} and parse-rejected lines into run-<name>.ndjson, the carrier the bumped PTY seam's spawn line (need 6) and the live-session evidence (need 10) rest on
@answer n=3 entity=span-trace-coverage#17.1 v=yes
why: verify's five interactive PTY runs and their spawn pairs, and the Ctrl-C/TERM stop closing a killed PTY child with the portable-pty exit code (child_exit_status for a signalled child), which the portable-pty bump may move; CI runs tests/cli_verify.rs against the fake agent
