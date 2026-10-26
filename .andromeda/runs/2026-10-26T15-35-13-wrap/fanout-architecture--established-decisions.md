proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 150
    section: '## Established Decisions'
    change: 'Replace the whole hold paragraph with: As landed (chunk 2026-10-26-explicit-session-links) the explicit `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` and their two channel methods are built, in the form the founder named for the boundary widening (his own word at P4, «Сначала instance check (Рекомендую)», relayed verbatim, that chunk''s `inputs#I3`): the driven instance''s wrapper serves `link` and `unlink` (no longer `-32601`), and each verb runs the strict-modes check on the home and the driven instance before any role file or log opens and before any snapshot value is read, then the liveness-only pre-check, then the server check, so no dated gap is taken. `LinkSet::remove` has its first product caller (the `unlink` method), and a product path now appends an `unlink` line. The hold the Epoch 6 working-route entry "Explicit session links" carried (the founder''s answer of 2026-10-13 that no explicit verb is built before Epoch 6) is discharged by that entry.'
    sidecar: 'dec-links: the hold on the explicit `viola link` / `viola unlink` verbs and their channel methods (unbuilt, `-32601`, `LinkSet::remove` with no caller, the three crossings unratified) is retired; the verbs and methods are built in the instance-check form on the founder''s word inputs#I3 (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Retired bullet names the hold itself, "the explicit `viola link` / `viola unlink` are not built / held for the founder''s word", the `-32601` answer for `link` / `unlink` and `LinkSet::remove`''s `dead_code` allow (report.md:86-89); the owner pattern `title` and the pattern `remove` each list this line as stale (report.md:115-117, :133-134); the Decisions record the founder''s P4 word for the instance-check form (report.md:265-267). Line 150 still states all of it as standing.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:86
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 149
    section: '## Established Decisions'
    change: 'Replace the sentence "As landed (chunk 2026-10-10-session-links) only the implicit link is built." with "The implicit link landed with chunk 2026-10-10-session-links, and the explicit verbs and methods with chunk 2026-10-26-explicit-session-links (below)." and keep the rest of the paragraph.'
    sidecar: 'dec-links: the [Links] entry no longer says only the implicit link is built; it names the two landing chunks (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The Retired bullet retires the hold, every master line stating it included (report.md:86-88); the pattern `linkverbs` reads this line true only for its head, and the claim that only the implicit link is built is the same retired hold (report.md:189-191, Symbols / APIs report.md:18-29, :38-40).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:86
    dependent-of: D-retired
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 149
    section: '## Established Decisions'
    change: 'Replace "and an explicit `viola link` of an existing pair adds none and returns `{}`" with "and an explicit `viola link` of a held pair adds none and answers `{since, already_linked: true}` with that pair''s `since` (a new pair answers `already_linked: false`)"; the clause "so `since` is the time of the first link after any `unlink`" stays.'
    sidecar: 'dec-links: the explicit `link` answer is `{since, already_linked}` as built, not `{}` (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The built `link` method answers `{"ok":{"since":<the link line''s ts>,"already_linked":<bool>}}`, a second `link` of a held pair appending nothing and answering the same `since` with `already_linked: true` (report.md:23-26); the plan''s expected amendment names [Links] `{since, already_linked}` (report.md:184-185). The locked decision''s `{}` is contradicted by the built contract.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 150
    section: '## Established Decisions'
    change: 'Add after line 150, as its own continuation line: The served methods (`LinkSet::link` and `LinkSet::unlink` in `src/run/links.rs`, sharing `driver_of`, routed by `dispatch_call` in `src/cmd/run.rs`; the served set is now nine methods with the `hook.event` notification): `params.driver` and `params.driven` are read as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the serving wrapper''s own name, and a `driver` equal to `driven` are each `-32602` `invalid params` with `data: null` and no line written (the self-pair refused by the wrapper only, never client-side: the operator''s answer of chunk 2026-10-10-session-links, card lean 2). Neither method reads `from`, whatever its shape. `link` records the pair and answers `{since, already_linked}`; `unlink` removes it and answers `{}` whether or not a pair was removed, an absent pair appending nothing. A record or remove error is `-32603`. Each answers after its line and the snapshot write; neither opens a span or writes a process-log line, the wrapper''s `channel-request` / `channel-response` pair carrying `method:"link"|"unlink"`. The `link` line''s `since` is stamped from the wall clock directly, outside the clock seam.'
    sidecar: 'dec-links: new detail — the served `link` / `unlink` methods: name, own-name and self-pair validation (`-32602`, no line), `from` unread, `{since, already_linked}` / `{}` answers, `-32603` on a failed record, answered after the line and the snapshot write (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Symbols / APIs give the methods'' validation, answers, error and ordering (report.md:18-29), the dispatch route and the nine served methods (report.md:33-37), and the `since` stamp outside the clock seam (report.md:278-279); the plan''s expected amendment names §Established Decisions [Links] for the explicit methods (report.md:184-185).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 150
    section: '## Established Decisions'
    change: 'Add after the served-methods line, as its own continuation line: The verbs (`src/cmd/link.rs` and `src/cmd/unlink.rs`, one shared `link_call`) take the driven instance as theirs and run, in order: `viola_state::strict::check_instance` on the absolute home and `instances/<driven>`, before `client::start`, a refusal exit 21 (as `list` and `plugin install`, not `revive`''s exit 1: the founder''s option text, card lean 5) printing the fixed instance-state `unable:` / `hint:` pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, no line in any log and the directory''s mode as found; then `client::start` (the `cli-<driven>` role file); the liveness-only pre-check `live_endpoint` (none: exit 21, the not-running hint); params `{driver, driven}` plus `from` when `VIOLA_NAME` is set; and one request through `cmd::client::connect` under `cli.link` / `cli.unlink`, the server identity check on Linux and Windows (macOS the recorded gap), a server that is not the recorded wrapper getting no frame (exit 21 with the server-verify hint, `server-verify-failed`). A wrapper error, `-32602` included, is exit 20; an `ok` of another shape is exit 1. The check precedes the role file because opening one narrows an existing home to 0700, so a check made after it passes a 0770 home. With them the CLI frame verbs are eight, and only `link` and `unlink` also run `check_instance` first.'
    sidecar: 'dec-links: new detail — the `viola link` / `viola unlink` verbs: `check_instance` before any role file or log (exit 21, instance-state pair or the instance-unreachable document), then liveness, then the server check through `cmd::client::connect`, exit 20 / 21 / 1 mapping (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Symbols / APIs give `link_call`''s order, refusals and exits (report.md:38-59) and the eight CLI frame verbs, only `link` / `unlink` running `check_instance` first (report.md:77-79); the Decisions give exit 21 over `revive`''s exit 1 (report.md:271-272) and the order trap (report.md:273-275).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:38
note: 'The body carries the same retired hold outside this file: .andromeda/architecture.md:51 ([Links] "only the implicit link is built, the explicit verbs held"), :193 (`link` / `unlink` answered `-32601`, held) and :220 ("No product path appends `unlink` yet"); those are the body''s and standard-contracts'' detectors to amend (report.md:115-117, :189-191). Lines 85 (`pause`, `link`, `unlink` CLI-only) and 202 (link-set restore at start) read true; no new dependency, runtime or crate (report.md:80-81).'
