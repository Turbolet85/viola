@answer n=1 entity=standard-contracts#16.6 v=no
why: the session added or changed no refusal reason, detail, exit code or result shape, so the revive --json internal-error document stands.
@answer n=2 entity=standard-contracts#17.1 v=no
why: the session changed no statusline arm behaviour, exit code or instance-snapshot field, and its hook.rs edits are Windows-remnant removals only.
@answer n=3 entity=standard-contracts#18.1 v=no
why: Session Liveness (heartbeat, pid and start time) is unchanged and no channel method or frame was added.
@answer n=4 entity=standard-contracts#18.2 v=no
why: Session Liveness is unchanged, and the client.rs edit is a Windows-remnant removal that leaves without_endpoint's cause reads as they are.
@answer n=5 entity=standard-contracts#18.3 v=no
why: the peer euid decision is channel-side and Session Liveness is unchanged, with no channel method or frame added.
@answer n=6 entity=standard-contracts#19.1 v=no
why: the list payload and the claude agents read are unchanged, the list.rs and version_gate.rs edits are Windows-remnant removals, and the live round re-witnessed the list contract.
