proposals:
  - detector: D-retired
    severity: warning
    section: '§Established Decisions ([Links])'
    change: 'Replace "only the implicit link is built, the explicit verbs held" with: the implicit link and the explicit `viola link <driver> <driven>` / `viola unlink <driver> <driven>` are built (chunk 2026-10-26-explicit-session-links, in the founder''s instance-check form): each verb runs `check_instance` on the driven instance before any log or snapshot read, then the liveness pre-check and the server check before its one frame, and the driven instance''s wrapper serves the `link` / `unlink` methods, `unlink` the product caller of `LinkSet::remove`.'
    sidecar: '[Links]: the explicit-verb hold retired; `viola link` / `viola unlink` and the `link` / `unlink` channel methods built in the instance-check form (chunk 2026-10-26-explicit-session-links, inputs#I3).'
    rationale: 'The report''s Retired bullet names the hold ("the explicit `viola link` / `viola unlink` are not built / held for the founder''s word", every master line stating it); Symbols / APIs lands both verbs (`src/cmd/link.rs:85-136`, check_instance at `src/cmd/link.rs:98-106` before `client::start`) and both served methods (`src/run/links.rs:70-83`); Decisions & corrections records the founder''s word inputs#I3.'
    basis: 'architecture.md:51'
  - detector: D-retired
    severity: warning
    section: '§Standard Contracts (Channel methods)'
    change: 'Replace the `link` / `unlink` row with: `link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, sent to the driven instance''s wrapper and served since chunk 2026-10-26-explicit-session-links; each reads `driver` and `driven` through `ViolaName::try_new` and answers a missing, non-string or invalid name, a `driven` that is not the wrapper''s own name, and a `driver` equal to `driven` `-32602` with `data: null`, no line written; neither reads `from`; `link` records the pair and answers `since` (the link line''s `ts`), a second `link` of a held pair appending nothing and answering the same `since` with `already_linked: true`; `unlink` removes the pair and answers `{}` whether or not one was held (an absent pair appends nothing); each answers after its line and the snapshot write, a failed record or remove `-32603`.'
    sidecar: 'Channel methods: `link` / `unlink` served (no longer `-32601`), their `{since, already_linked}` / `{}` payloads and their `-32602` causes (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Retired bullet names "the `-32601` answer for `link` / `unlink`" and the hold; Symbols / APIs gives the served semantics (`src/run/links.rs:70-76`, `:78-83`, `driver_of` `:85-99`) and the reply shapes; its notserved pattern reads this row stale.'
    basis: 'architecture.md:193'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Standard Contracts (Event data per kind, the wrapper-appended paragraph)'
    change: 'The driven instance''s wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or on a served `link` call, and, since chunk 2026-10-26-explicit-session-links, appends `unlink` (`source: wrapper`, `{driver, driven}`) once when a served `unlink` call removes a held pair through `LinkSet::remove` (an absent pair appends nothing); both are log-only and never wake `wait`. Drop "No product path appends `unlink` yet: its one writer, `LinkSet::remove`, has no caller until the `unlink` method is served".'
    sidecar: 'Event data: a product `unlink` line is now written (the served `unlink` method, `LinkSet::remove`''s first product caller); `link` lines also come from the served `link` method.'
    rationale: 'Symbols / APIs: "`LinkSet::remove` has its first product caller (the `unlink` method) … A product path now appends an `unlink` event line (`source: wrapper`, `{driver, driven}`) — the first"; Counts names `architecture.md:220` stale under pattern `remove`; Retired names `LinkSet::remove`''s `dead_code` allow.'
    basis: 'architecture.md:220'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'The `src/run/links.rs` row: the LinkSet (one per wrapper, the instance''s link set under one lock: `restore` from the log at start, `current`, `record`, `remove`, and the served `link` / `unlink` channel methods sharing `driver_of`; `remove`''s product caller is the `unlink` method) — drop "which has no product caller yet".'
    sidecar: 'Project directory structure: links.rs names the served `link` / `unlink` methods; `LinkSet::remove` has its product caller.'
    rationale: 'Symbols / APIs: `LinkSet::link` / `LinkSet::unlink` (`src/run/links.rs:70-83`) share `driver_of` (`src/run/links.rs:85-99`), and `LinkSet::remove` has its first product caller; Retired names `LinkSet::remove`''s `dead_code` allow. The keyed contract restates the retired no-caller claim, a site the report''s `remove` pattern did not list.'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-retired
  - detector: D-arch-resources
    severity: warning
    section: '§Conventions (CLI exit codes)'
    change: '`21` instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link` or `viola unlink` (the two verbs'' `check_instance` on the driven instance; exit 20 for a wrapper `-32602`, e.g. `driver` equal to `driven`).'
    sidecar: 'CLI exit codes: exit 21 also covers `viola link` / `viola unlink`''s refused strict-modes check (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Symbols / APIs step 1: `check_instance` refused → exit 21, `--json` `{"v":1,"error":"instance-unreachable","detail":null}`; Decisions & corrections: "The verbs'' strict-modes refusal is exit 21 (as `list` / `plugin install`), not `revive`''s exit 1"; the report''s Expected amendments name §Conventions CLI exit codes and say no pattern fired for the row.'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources (Filesystem, `instances/<ViolaName>/`)'
    change: '`viola list`, `viola revive`, `hook statusline`, `viola link` and `viola unlink` check these files before they use them, ahead of any log, because opening a log narrows an existing home (the two link verbs check the driven instance before `client::start` opens their `cli-<driven>` role file).'
    sidecar: 'Occupied Resources: `viola link` / `viola unlink` join the processes that check the instance files before any log (`check_instance` product call sites 2 → 3).'
    rationale: 'Symbols / APIs step 1: `check_instance(<absolute home>, <home>/instances/<driven>)` (`src/cmd/link.rs:98-106`) BEFORE `client::start`, so before any role file or log opens; Counts: `check_instance(` product call sites 2 → 3, and "a detector extends a list of refusing verbs where it finds one"; the report''s Expected amendments name occupied-resources'' checking processes.'
note: 'No D-arch-decisions drift (Dependencies: none; no crate added). The detail-file sites the report counts stale (occupied-resources.md:14, :106; established-decisions.md:150) are the cascade''s, not proposed here; the `instance-unreachable` CLI `--json` document in §Standard Contracts already covers the two verbs'' exit-21 refusals generically.'
