# security · picked accounting · group 1 of 2

## Picked
threat-model-summary#1.3 · carried · names the integrity-sensitive, code-bearing home files (`statusline_command`, stamps, plugin and marketplace exec paths, the pinned binary) whose integrity the chunk owns
threat-model-summary#1.5 · not in scope · classes the snapshot's and `budget.json`'s operational metadata; the chunk changes neither what they hold nor where it appears
threat-model-summary#2.1 · carried · records that clients act on the snapshot's `endpoint` and read it without the strict-modes check, the gap the chunk closes
threat-model-summary#5.1 · carried · the hook's fail-open rule and `statusline_command`'s integrity resting on the home's permissions rule the hook's new pre-read check
threat-model-summary#6.1 · not in scope · MCP input schemas and the driver's trust in driven output; the chunk reaches `viola mcp` only at its frame tools' snapshot read
threat-model-summary#7.1 · carried · `--home` / `VIOLA_DIR` choose the tree the new check runs on, a home resolution the scope names
threat-model-summary#8.1 · carried · the filesystem boundary's readers, code-bearing artefacts, replay readers and content-hash-only integrity are the chunk's subject
threat-model-summary#9.1 · not in scope · child spawning and PATH resolution; the chunk spawns nothing new and changes no resolver, reaching verify only at its stamps read
threat-model-summary#10.1 · not in scope · standing S6 · each new dependency or feature passes `cargo deny check`, the sync-crate tokio ban and the C-build ban
threat-model-summary#11.1 · carried · the `windows-2025` leg is the chunk's Windows proof and the release build through `scripts/release-check.sh` is what it re-measures
authentication-authorization#1.2 · not in scope · the all-or-nothing served surface; the chunk adds no method and changes no params check
authentication-authorization#1.3 · not in scope · the Windows pipe listener's SDDL; the scope leaves the IPC listener unchanged
authentication-authorization#1.6 · carried · the dated gaps' strict-modes halves it records are what the chunk closes, beside the refusal surfaces and the collision check it edits
authentication-authorization#1.7 · not in scope · the wrapper's `release-from-driver` refusal; the chunk reaches `release` only at its client's pre-frame read
authentication-authorization#1.8 · carried · the home's creation modes, the strict-modes entry points, the dated gaps, the trusted-file list and the Windows owner + DACL check are the chunk's work
authentication-authorization#3.1 · not in scope · the channel's peer-pid read, a component the chunk only uses and leaves unchanged
authentication-authorization#3.2 · carried · the verdict's one `read_snapshot` call through `live_endpoint` and the hook's `connect` is the read the chunk puts behind the check
authentication-authorization#3.3 · carried · `link` / `unlink`'s check-before-`client::start` form is the target every other frame verb takes
authentication-authorization#3.4 · carried · the exit-21 CLI refusal and its order before the liveness and server checks rule the new strict-modes refusals
authentication-authorization#3.5 · not in scope · an exiting wrapper's Windows pipe, owed to "Windows-only live measurements"; the hook's verdict is unchanged
authentication-authorization#3.7 · not in scope · `link` / `unlink`'s own tests; the chunk changes neither verb
