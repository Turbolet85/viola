# Citation dispositions — security-plan

changed .andromeda/security-plan-detail/authentication-authorization.md:27@c1725 src/cmd/run.rs:383 → ? — rewritten → claim false — raised at Validate (the collision check no longer reads the snapshot through `read_snapshot`: its caller reads it once through `read_snapshot_or_replay` (src/cmd/run.rs:277) and hands the `Recovered` to `collision_check` (src/cmd/run.rs:418-445), which judges a replayed snapshot by the heartbeat alone, with no pid or start time; the no-connection, no-frame half still holds)
