@answer n=1 entity=standard-contracts#16.6 v=no
why: the session changed no CLI exit code or result shape, so the revive one-exit-1-document rule still holds
@answer n=2 entity=standard-contracts#17.1 v=yes mode=stale
why: src/cmd/hook.rs is reached whole by the Windows-remnant removal, and the statusline arm's canonical, trusted and STATUSLINE_DEADLINE claims sit in it with no report showing they hold on the changed file
@answer n=3 entity=standard-contracts#18.1 v=no
why: the session reports Session Liveness (heartbeat and pid plus start time) unchanged, so the server identity read still holds
@answer n=4 entity=standard-contracts#18.2 v=no
why: the session reports Session Liveness unchanged, so the without_endpoint cause and its strict-modes-first callers still hold
@answer n=5 entity=standard-contracts#18.3 v=no
why: the session reports Session Liveness unchanged, and the peer euid decision in viola_channel is not among the reached files
@answer n=6 entity=standard-contracts#19.1 v=yes mode=stale
why: src/run/version_gate.rs and src/cmd/list.rs are reached whole, and the agents read and board join claims sit in them with no report showing they hold on the changed files
