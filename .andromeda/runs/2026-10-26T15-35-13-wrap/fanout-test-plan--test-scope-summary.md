proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: '## 1. Test Scope Summary'
    change: 'in the As landed (server verification) item, replace "the six CLI frame verbs by `tests/security_negatives_server_identity.rs`" with "the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs`"; the rest of the item unchanged'
    sidecar: 'test-scope-summary.md §1 viola-channel As landed (server verification): the CLI frame verbs held by the identity table move six to eight (`link`, `unlink` added, chunk 2026-10-26-explicit-session-links)'
    rationale: 'report Counts / qualifiers moved: CLI frame verbs six to eight (pattern frameverbs) and the identity table 19 to 25 cases (pattern idtable), each naming test-scope-summary.md:12 stale; Symbols / APIs: "The CLI frame verbs are now eight"'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:94
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 35
    section: '## 1. Test Scope Summary'
    change: 'add a sub-item after the CLI verbs entity Testability line: "  - **As landed (link / unlink):** `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` (chunk 2026-10-26-explicit-session-links) run `check_instance` on the driven instance before any log opens or snapshot value is read (refused: exit 21, human the fixed instance-state `unable` / `hint:` pair, `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, the directory mode as found), then the liveness pre-check and the server check (exit 21); success under `--json` is `{"v":1,"ok":{"since","already_linked"}}` for `link` and `{"v":1,"ok":{}}` for `unlink`, a wrapper error (a `driver` equal to `driven` included) exit 20; the human lines `link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked` and `unlink  <driver> -> <driven>` are plain in every mode; held by `tests/cli_links.rs`"'
    sidecar: 'test-scope-summary.md §1 CLI verbs entity: As landed (link / unlink) added, the two verbs, their instance check first, exits and `--json` documents (chunk 2026-10-26-explicit-session-links)'
    rationale: 'report Symbols / APIs: new verbs `viola link` / `viola unlink`, `link_call` steps 1-6 (check_instance before client::start, exit 21 / 20 / 1, `--json` documents), three result lines; Outcome: cli_links 12 passed'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 61
    section: '## 1. Test Scope Summary'
    change: 'add a sub-item after the wrapper-channel surface Notes line: "  - **As landed (link / unlink):** both methods are served by the driven instance wrapper since chunk 2026-10-26-explicit-session-links (before, `-32601`): a missing, non-string or invalid name, a `driven` that is not the serving wrapper own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written; `from` is not read; `link` answers `{"ok":{"since","already_linked"}}` (a held pair appends nothing and answers the same `since`), `unlink` answers `{"ok":{}}` whether or not a pair was removed, its removal appending the first product `unlink` line; held by `tests/channel_links.rs` (`channel_link_names_are_validated_by_the_wrapper`) and the `links_set_method_*` units"'
    sidecar: 'test-scope-summary.md §1 ipc-internal wrapper-channel surface: As landed (link / unlink) added, the served methods, their `-32602` causes and `ok` shapes (chunk 2026-10-26-explicit-session-links)'
    rationale: 'report Symbols / APIs: channel methods `link` and `unlink` are served (`driver_of` refusals `-32602` data null, no line; `{since, already_linked}` / `{}`; `LinkSet::remove` first product caller); Retired: the `-32601` answer for `link` / `unlink`; Outcome: channel_links 14 passed'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18
note: 'outside this file - test-plan.md:170-174 (body §1) still lists `link` / `unlink` as an E2E expansion scenario, read true; line 59 names `-32602` only with `data:{supported, wrapper}` while the link / unlink causes (like `send` invalid params) carry `data: null` - pre-existing wording, not proposed'
