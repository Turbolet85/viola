
## 2026-10-17-cli-machine-contract — a11y: revive takes --json on both arms
**Section:** §1 A11y Scope Summary → CLI verbs entity; §8 Cognitive Accessibility → Error recovery (CLI bullet)
**Change:**
- `revive` takes `--json` on both arms (was: "`revive` takes no `--json`"): each strict-modes or preflight exit-1 refusal is one `{"v":1,"error":"<code>","detail":null}` document on stdout with empty stderr (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; without `--json` the static ASCII rows and the `unable:` / `hint:` pair are byte-identical. A post-preflight `run` start refusal prints no document (owed to "Exit-cause code catalogue").
- Error recovery: revive's fixed-message pair is the human form; under `--json` the document carries no hint (`revive_json_*`, `tests/cli_revive.rs`).
**Why:** the chunk landed revive's machine form; the output-discipline clauses already name `--json` mode, so no other a11y clause moves.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/
