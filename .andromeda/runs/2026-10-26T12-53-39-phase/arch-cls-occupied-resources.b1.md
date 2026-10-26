# Classified — arch · occupied-resources.b1

## Answers
occupied-resources#1.1 · yes · the verbs connect to the snapshot's recorded `endpoint` and never rebuild the path, so the endpoint rule binds their connect
occupied-resources#2.1 · yes · the verbs are frame-writing clients, so the serving-process check against `snapshot.json` binds before their first frame (exit 21 `instance-unreachable` on a mismatch)
occupied-resources#3.1 · no · standing S1 · every new test runs in its own viola home against the fake agent named after `--`, never the real `claude`
occupied-resources#4.1 · yes · the served subcommand list (`link` and `unlink` already named) and its verb conventions bind, since the chunk serves both in the `src/cmd/mod.rs` dispatch
occupied-resources#5.1 · no · the hidden `hook --capture` arms are another job of the `src/cmd/mod.rs` dispatch that the chunk only keeps holding
occupied-resources#6.1 · no · the child's spawn flags and `resume_args` are spawn-time, and the chunk spawns no child
occupied-resources#7.1 · no · verify's probe plugin and children are not reached, since no `src/cmd/verify` file is touched and a link records no ledger row
occupied-resources#8.1 · no · the per-session `settings.json` override belongs to a start job of `src/cmd/run.rs` that the chunk keeps, and the chunk writes no settings file
occupied-resources#9.1 · yes · the two verbs resolve home through `--home`, then the `VIOLA_DIR` grandparent via `resolved_home`'s shape check, and take `from` from `VIOLA_NAME`
occupied-resources#10.1 · no · the R8 strip and identity floor govern the child's environment, and the chunk spawns no child and reads no `CLAUDE*` value
occupied-resources#11.1 · no · standing S2 · every new test home lives under `target/e2e-home/viola-test-*/home`, removed only when its recorded owner process is gone
occupied-resources#12.1 · no · the two fake-agent seams sit in other jobs (the `src/cmd/run.rs` pump start, `src/cmd/hook/seam.rs`) that the chunk keeps, and the new tests set neither
occupied-resources#13.1 · no · the chunk reads no `config.json` key, and no new line depends on `diagnostics_level`
occupied-resources#14.1 · yes · the verbs are new `cli-<name>.ndjson` producers (`process-start`, `channel-request` / `channel-response`, `process-exit`, `server-verify-failed` at WARN, `instance-dead`), so the producer list binds
occupied-resources#15.1 · yes · the `link` / `unlink` lines go to `events.ndjson` (never truncated, byte offsets kept) and the snapshot's `links` sits in `snapshot.json`, both files of this entity
occupied-resources#16.1 · no · `budget.json` and its gate belong to the `send` path, which the link method does not touch
occupied-resources#17.1 · no · a link writes and reads no stamp, and `run`'s version gate is another job the chunk keeps
occupied-resources#18.1 · no · `ledger/probes/<pid>/` is verify's probe directory, and verify is not reached
occupied-resources#19.1 · no · verify's interactive probe directories are not reached, since the chunk creates no `.viola-verify-*` dir
occupied-resources#20.1 · yes · new root cases that wait on the wrapper take the shared `Watch` bound (WITHIN 7 s) and its report, and any new direct wait site changes the count recorded here
occupied-resources#21.1 · no · standing S2 · every new test home lives under `target/e2e-home/viola-test-*/home`, removed only when its recorded owner process is gone
occupied-resources#22.1 · yes · the P3 re-read of `v1-13`'s MCP half is witnessed by the recorded `Mcp.tools-list-1.json` exchange this entity names (`tests/mcp_recorded_exchange.rs`), and the no-touch re-read covers its committed sets
occupied-resources#23.1 · no · `target/agent-run/` is the harness's own session record and artifacts, written by `viola-harness`, which the chunk does not touch
