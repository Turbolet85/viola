proposals:
  - detector: D-layout-surface
    severity: warning
    section: "Output structure — `viola revive`"
    change: "Retire \"The verb takes no `--json`\": the verb takes `--json` on both arms — a preflight refusal is one stdout document `{\"v\":1,\"error\":\"<code>\",\"detail\":null}`, exit 1, empty stderr, `<code>` one of `strict-modes-failed`, `already-live`, `no-session` (both of its pairs), `session-live`, `cwd-missing`; `--list --json` prints `{\"v\":1,\"ok\":{\"sessions\":[{\"ts\",\"cause\",\"id\"},…]}}` in log order (fields as the human rows), or the `no-session` document with exit 1; the human pairs and `--list` rows stay byte-identical; the wireframe block gains a `$ viola revive builder --list --json` example and one refusal document; a refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`) prints no document as built (the gap the report names, owner P2/P7)."
    sidecar: "§Output structure — viola revive: --json on both arms as landed (the five-code exit-1 document, --list's ok.sessions payload), the post-preflight no-document gap stated (chunk 2026-10-17-cli-machine-contract)."
    rationale: "The report's Symbols / APIs (`src/cmd/revive.rs`: `ReviveArgs` gains `--json`, `Refusal::write(name, json)`, `ListRow`; `src/cmd/client.rs` `refused_document`; `src/cmd/run.rs` `collision_check(…, json)`) adds a machine-output surface to `viola revive` that the doc's revive wireframe denies; Counts / qualifiers moved lists layout-templates.md:243 stale under pattern `revive-nojson`; Expected amendments names this section; Spec claims disproved records that run::start's own refusals print no document."
    basis: "layout-templates.md:243"
  - detector: D-layout-surface
    severity: warning
    section: "Component — Primary navigation (verb structure)"
    change: "Global flag bullet: `--json` is the machine view of every data verb, `viola revive` included (both arms: its refusal document and `--list`'s `ok.sessions`); drop \"which takes none yet (its `--list` rows are the human form only; owed to the working-route entry \"CLI machine contract\")\"."
    sidecar: "§Component — Primary navigation: revive's --json exception and the owed clause to \"CLI machine contract\" retired (chunk 2026-10-17-cli-machine-contract)."
    rationale: "Same retired claim (revive takes no `--json`) restated in the navigation component; the report's Counts / qualifiers moved lists layout-templates.md:266 stale under `revive-nojson` and `owner-title` (the owed clause's owner is this chunk)."
    basis: "layout-templates.md:266"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "Component — Primary content block 2: refusal lines and the `unable` column"
    change: "In the line-form bullet, add `revive --list` to the failed-or-panicked `cli` verbs that print exactly `error: internal error` once from the one catch site (a `revive` start stays outside it), and note that under `--json` revive's exit-1 pairs are replaced by its `{\"v\":1,\"error\":\"<code>\",\"detail\":null}` document."
    sidecar: "§Primary content block 2: revive --list joins the cli catch site's `error: internal error`; revive's --json refusal document named (chunk 2026-10-17-cli-machine-contract)."
    rationale: "The report's Symbols / APIs (`src/main.rs` `role_of`: `revive … --list` before a `--` is `Role::Cli`, so a `--list` dispatch Err or panic reaches the `cli` catch site, exactly `error: internal error`) widens the cli-verb list this bullet closes; the revive refusal pairs it lists now have a `--json` document form (`Refusal::write`)."
    basis: "src/main.rs"
note: "The revive --json gap (run::start's refusals after a passed preflight print no document; codes outside the closed five) is the report's Spec claims disproved item, owner P2/P7; proposal 1 states it as built rather than resolving it. layout-templates.md:58 (Primary screens) asserts no absence of --json and was left unproposed."
