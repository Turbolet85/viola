# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · yes · wait and last return these lines verbatim (wait.rs, last.rs and the MCP wait and last tools), so the server check must hold before any of it leaves
threat-model-summary#1.2 · yes · the hook.dialog send carries question, option, plan and tool-input text and answer frames carry dialog decisions, so the check must hold before either leaves
threat-model-summary#1.3 · yes · the check trusts the pid, started_at and endpoint that snapshot.json records, so that snapshot's integrity binds (its plugin, marketplace and bin rows stay with Home and code-bearing file integrity)
threat-model-summary#1.4 · no · the chunk reads no environment, stores no secret and spawns no child, so the R8 strip and secret storage stay untouched (task Not needed)
threat-model-summary#1.5 · yes · the check reads pid and started_at from the snapshot, and the mismatch report may carry them only to diagnostics/, never to --json, isError or error.data
threat-model-summary#2.1 · yes · server impersonation is the threat this check answers, since clients act on whatever answers at the snapshot's endpoint, so the check sits before every frame a client writes
threat-model-summary#3.1 · no · the chunk changes no text path; bracketed-paste framing, the typed-text rules and the empty-text refusal stay as they are (task Not needed: paste-text validation)
threat-model-summary#4.1 · no · the chunk touches no stdin path; the human's stdin classifier and the wheel's human-typing refusal stay as they are (task Not needed)
threat-model-summary#5.1 · yes · hook.event and hook.dialog gain the server check, so a mismatch must fail open (exit 0, nothing on stdout or stderr, no frame) under the hook's fail-open floor and the dialog payload shape check
threat-model-summary#6.1 · yes · tools.rs edits the send, wait, last and answer tools, so their instance-unreachable isError shape and the driver model's answer path on a PermissionRequest sit behind the check; release and pause stay unlisted
threat-model-summary#7.1 · yes · --home and VIOLA_DIR choose the snapshot the check reads, so canonicalisation and the strict-modes check come before that read, and no flag or variable may switch the check off
threat-model-summary#8.1 · yes · the five snapshot readers the scope names keep read-as-absent for an unreadable or newer snapshot, and the log replay stays with its two named product readers (the stop rule)
threat-model-summary#9.1 · no · the chunk spawns no child and changes no program resolution; run.rs edits only its collision check and the mcp list tool is untouched
threat-model-summary#10.1 · yes · the check lands in viola-channel's sync and Tokio clients, so the sync crates' tokio ban (deny-sync.toml) and scripts/channel-tokio-free.sh must stay green, with Tokio behind the tokio feature
threat-model-summary#11.1 · no · the chunk adds no CI job or workflow (task Not needed: supply-chain gates and workflows) and its tests run in the existing per-OS matrix
