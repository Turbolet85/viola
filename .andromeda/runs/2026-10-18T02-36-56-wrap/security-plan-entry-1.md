
## 2026-10-17-optional-plugin-install — plugin install's tree, its check and the marketplace .mcp.json
**Section:** §Authentication & Authorization (the `~/.viola/` access-control row); §Data Protection (At rest → Code-bearing artefacts); Threat Model Summary (Data classification → config; Attack surface → Filesystem state under `~/.viola/`)
**Change:**
- Access control: `viola plugin install` checks only its home and each existing `marketplace/` tree dir through `check_path` before anything under the home is created, written or opened; a refusal exits 21 (`instance-unreachable` under `--json`), nothing created, no log opened. `marketplace/` is no trusted file (viola never reads it back); the installed `command` gets no re-hash.
- Code-bearing artefacts: `marketplace/` is written only by the verb — two manifests and `viola/.mcp.json`, whose `mcpServers.viola` `command` is the canonical exe that ran the install and whose `args` are `["--home", <the canonical home>, "mcp"]`, built by `serde_json`, no hooks; each file rewritten whole at 0600, each dir 0700, any other entry left as found. viola spawns no `claude` and writes nothing under `~/.claude`.
- Threat Model: `marketplace/viola/.mcp.json` joins the config files holding an exec path and the code-bearing entry points.
**Why:** the chunk built the verb; write + print with no `claude` child is the founder's own answer to P4 fork Q1, relayed verbatim by the Viola overseer. The unpinned `command` is the standing design ([Deployment / Distribution]). The tree's integrity against a same-user writer is owned by the route entry "Home and code-bearing file integrity", on the operator's word at P5.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/
