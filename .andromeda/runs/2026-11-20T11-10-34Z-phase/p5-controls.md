# P5 must-pass controls — 2026-11-20-permissionrequest-question-answered

Clone: `git clone --shared` of the project at HEAD `1e9f1ea3`, at `$TMPDIR/{scratchpad}/clone`. Each entry was fired
through `gate.py run --chunk 2026-11-20-permissionrequest-question-answered --docs-root . --entry {n} --project-root
{clone}`.

On the real tree, every entry read red on an absent input (the baseline): no kept stamp, no evidence sha, no kept
record. Inputs minted in the clone only:
- `target/live-rig/question-stamp.json`: a stamp envelope `data.versions.2.1.328.rows` with 27 rows, 26 `pass` and
  `browser-tools-prompt` `absent`;
- `target/live-rig/PQ/record/2.1.328/`: the committed 2.1.328 files less `Mcp.tools-list-1.json`, plus three
  `{}` files named `{PreToolUse,PermissionRequest,PostToolUse}.question-permission-1.json`;
- `.andromeda/cache/chunks/2026-11-20-permissionrequest-question-answered/evidence/live-stamp.sha256`: the live home
  stamp's sha256, read by `sha256sum` alone.

| entry | reading |
|---|---|
| 19 | green — exit 0, last line `27 ok` |
| 20 | green — exit 0, last line `live-stamp: unmoved` |
| 21 | green — exit 1, the four `<` / `>` lines present |
| 22 | green — exit 1, last line `4` |

The fail side of each is the baseline's red on the real tree (absent input). For entry 19, a short stamp would print
`short`, the `else` arm of its own expression.
