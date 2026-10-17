
## 2026-10-17-cli-machine-contract — layouts: viola revive takes --json; revive --list at the cli catch site
**Section:** §Output structure — `viola revive`; §Component — Primary navigation (global flags); §Component — Primary content block 2 (line form)
**Change:**
- `viola revive` usage carries `[--json]` on both forms (was: "The verb takes no `--json`"); under it each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` (both pairs) · `session-live` · `cwd-missing`; `--list --json` prints the rows as one document, or the `no-session` document; a wireframe shows both. Without `--json` pairs and rows are byte-identical. A post-preflight `run` start refusal prints no document (owed to "Exit-cause code catalogue").
- `--json` is the machine view of every data verb, `viola revive` included (was: "except `viola revive`, which takes none yet", owed to "CLI machine contract").
- Line form: revive's pairs are its document under `--json`; `revive --list` joins the `cli` verbs that print exactly `error: internal error` from the one catch site (a `revive` start stays outside it).
**Why:** the chunk landed revive's machine form and filed `revive --list` under the `cli` role.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/
