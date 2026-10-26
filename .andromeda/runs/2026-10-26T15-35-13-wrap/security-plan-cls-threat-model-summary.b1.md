# Classified — security-plan · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · the new unlink line carries only the driver and driven names, and this row covers prompt and reply text the session does not write
threat-model-summary#1.2 · no · the session adds no dialog, plan or permission content, and hook.dialog is unchanged
threat-model-summary#1.3 · no · no config key or schema changed and the session writes no statusline, endpoint or pinned-path value
threat-model-summary#1.4 · no · link and unlink spawn no child and read no CLAUDE* value, so the R8 strip and the credential boundary are unchanged
threat-model-summary#1.5 · no · no snapshot or budget field changed, and the link names are not among the operational fields this row lists
threat-model-summary#2.1 · yes · add-to · F10: any same-OS-user process passing the endpoint controls can now add or remove a driver link through the served link and unlink methods, a record line and the snapshot's links only, and the frame-verb list moves from six to eight
threat-model-summary#3.1 · no · link and unlink carry names only and no send text, so the paste and CR/LF typing rules are untouched
threat-model-summary#4.1 · no · the stdin classifier and the wheel live in src/run/, which the session reaches only at links.rs, and link and unlink take no wheel
threat-model-summary#5.1 · no · viola hook, its payload parsing and fail-open rules and the statusline shell-out are unchanged
threat-model-summary#6.1 · no · the MCP tool list is unchanged, and the task reports that leaving link and unlink off it still holds
threat-model-summary#7.1 · no · the new verbs take their names through clap and ViolaName::try_new, so the All-subcommands entry and the clap and nutype boundary still hold
threat-model-summary#8.1 · no · the new verbs read only snapshot.json after the strict-modes check, the unlink line is an ordinary append, and no new log-replay reader is added
threat-model-summary#9.1 · no · link and unlink spawn no child process, so the spawn and PATH-resolution entries are unchanged
threat-model-summary#10.1 · no · no dependency, action, toolchain or deny setting changed, and .config/nextest.toml is test-runner config this row does not describe
threat-model-summary#11.1 · no · no CI job, workflow or release step changed, and this row does not describe nextest configuration
