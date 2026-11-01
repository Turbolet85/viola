# Classified — arch · established-decisions.b4

## Answers
established-decisions#33.2 · no · Windows-only pin_companions owner/DACL check on bin/<version>-<hash>/conpty/; the chunk changes no Windows behaviour (Not needed: Windows endpoint security).
established-decisions#33.3 · no · reproducible-build divergence of one tree into two bin/plugin keys; no build, pin or copy-placement file is reached by the chunk.
established-decisions#34.1 · yes · `viola plugin install` writes only `<home>/marketplace/` with the `.mcp.json` command and `--home` args, and its strict-modes refusal (exit 21) must run ahead of that write; the plugin's `viola mcp` server is what the new client-side euid check on `viola mcp` frame tools acts on.
established-decisions#35.1 · yes · `viola revive --json` closed code list (`strict-modes-failed` and the other codes) through `client::refused_document` is the list the new refusals must fit; the new refusals extend it with their closed codes and hints.
