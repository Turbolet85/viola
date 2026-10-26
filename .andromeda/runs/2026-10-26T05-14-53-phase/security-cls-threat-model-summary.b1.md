# Classified — security · threat-model-summary.b1

## Answers

threat-model-summary#1.1 · no · events.ndjson user content and retention, which the chunk does not change (Not needed: Data Protection and the NEVER-log floor)
threat-model-summary#1.2 · no · tool-call, plan and question content in events.ndjson and hook.dialog, which the chunk does not change (Not needed: NEVER-log floor and IPC access control)
threat-model-summary#1.3 · no · config, settings, snapshot and exec-path files, which the chunk does not change (Not needed: Data Protection)
threat-model-summary#1.4 · no · the transient CLAUDE* credentials and the R8 strip, which the chunk does not change (Not needed: Secret Management)
threat-model-summary#1.5 · no · snapshot pid and cwd and budget usage metadata, which the chunk does not change (Not needed: Data Protection and the NEVER-log floor)
threat-model-summary#2.1 · no · the local IPC trust boundary, which the chunk does not change though the channel's Tokio client rides on tokio (Not needed: IPC access control)
threat-model-summary#3.1 · no · the CLI send text framing and paste validation, which the chunk does not change (Not needed: input validation floors)
threat-model-summary#4.1 · no · the terminal stdin classifier, which the chunk does not change and run's pump keeps std threads with no Tokio (Not needed: boundaries other than supply chain)
threat-model-summary#5.1 · no · hook stdin payloads and the fail-open exit, which the chunk does not change on the sync hook path (Not needed: input validation floors and boundaries other than supply chain)
threat-model-summary#6.1 · no · the MCP tool surface and its input checks, which the chunk does not change in viola-mcp (Not needed: boundaries other than supply chain)
threat-model-summary#7.1 · no · CLI arguments, flags and environment, which the chunk does not change (Not needed: boundaries other than supply chain)
threat-model-summary#8.1 · no · filesystem state under the home and its code-bearing artefacts, which the chunk does not change (Not needed: Data Protection and boundaries other than supply chain)
threat-model-summary#9.1 · no · child process spawning and PATH resolution, which the chunk does not change (Not needed: boundaries other than supply chain)
threat-model-summary#10.1 · yes · the supply-chain boundary the chunk moves: cargo deny over deny.toml, the sole-root tokio ban and channel-tokio-free.sh, the committed Cargo.lock and the fuzz lockfile audit
threat-model-summary#11.1 · yes · the ci.yml supply-chain job (cargo deny, fuzz lock audit, tokio ban, channel-tokio-free.sh, ban probes), the MSRV 1.96 job and the release build through release-check.sh, all re-run or re-read by the chunk
