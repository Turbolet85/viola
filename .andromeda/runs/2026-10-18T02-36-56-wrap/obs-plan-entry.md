
## 2026-10-17-optional-plugin-install — plugin install's span, log shape and no-line refusal
**Section:** §2 (span naming); §3 (the `run` stderr-refusal bullet); §4 (the cli row; `verify` / `plugin install`); §6 (Filesystem refusals); §10 (bounded exemptions 1 and 7); §11 (the print-macro ban's `src/human.rs` exception)
**Change:**
- Span: was "Dispatch spans: … and `cli.plugin_install`" and "`cli.<verb>` names the dispatch span (for example … `cli.plugin_install`)"; now `cli.plugin_install` is no dispatch span, on `write_tree`, opened after the `cli` log opens, the tree check and its refusal outside it.
- Log shape: the self pair into the caller's `cli-<name>.ndjson` only under a parsed `VIOLA_NAME` and after the check passed (`process-exit` exit 0, no detail); no name, no log; its outcome the exit code and, under `--json`, the `ok` document, no path anywhere.
- Refusal: its refused check opens no role file and writes no line (exit 21, the stderr pair or the `instance-unreachable` document), in §6, exemption 1's pre-init window and exemption 7.
- Census: was `run` 5 · `verify` 5 · `list` 2 · `revive` 1; now 14 with `plugin install` 1, re-derived by `grep -rcE 'human::refuse\(' src` (§3 and §11).
**Why:** the chunk built the verb. Its exemption-7 shape is its plan's lean, approved by the operator at P5, not ruled by the founder.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/
