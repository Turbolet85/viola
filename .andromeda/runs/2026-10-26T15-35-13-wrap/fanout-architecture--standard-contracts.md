proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 47
    section: ## Standard Contracts
    change: Name the two served methods in the server-check client list — "Before its first frame, the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools, compares the connected stream's peer pid and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (Session liveness, below); a mismatch, or a check that cannot be made, writes no frame. The CLI `link` / `unlink` verbs first run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens and before any snapshot value is read (the founder's form, chunk 2026-10-26-explicit-session-links `inputs#I3`). macOS reports no peer pid and stays a recorded gap."
    sidecar: standard-contracts — the server-check client list gains `link` / `unlink`, with the verbs' instance check ahead of it (chunk 2026-10-26-explicit-session-links)
    rationale: The report's Symbols / APIs make `viola link` / `viola unlink` CLI frame verbs (six → eight) whose request goes through `cmd::client::connect` (the peer pid / start-time check) after `check_instance` and the liveness pre-check; the file's client list still names only the six earlier verbs, leaving the two new channel clients unregistered.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:40
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 61
    section: ## Standard Contracts
    change: New group (values · contract-channel-methods) after the `wait` group — "- `link` and `unlink` `{driver, driven}`, sent to the driven instance's wrapper, are served since chunk 2026-10-26-explicit-session-links (before it both answered `-32601`): `LinkSet::link` and `LinkSet::unlink` (`src/run/links.rs`) share `driver_of`, which reads `driver` and `driven` as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, and a `driver` equal to `driven` are each `-32602` `invalid params` with `data: null`, no line written, refused by the wrapper only, never client-side. Neither reads `from` (a `from` of any shape changes nothing). `link` records the pair ([Links]) and answers `{since, already_linked}`: `since` is the link line's `ts`, and a second `link` of a held pair appends nothing and answers the same `since` with `already_linked: true`. `unlink` removes the pair through `LinkSet::remove`, appending one `unlink` line (`source: wrapper`), and answers `{}` whether or not a pair was removed (an absent pair appends nothing). A failed append is `-32603` with the set unchanged. Each answers after its line and the snapshot write; neither opens a span nor writes a process-log line beyond the wrapper's `channel-request` / `channel-response` pair, which carries `method:"link"|"unlink"`. The boundary widening is the founder's own word (P4, `inputs#I3`)."
    sidecar: standard-contracts — new channel-methods group for the served `link` / `unlink` (params, `-32602` causes, `{since, already_linked}` / `{}`, the product `unlink` line) (chunk 2026-10-26-explicit-session-links)
    rationale: The report's Symbols / APIs serve the two methods (`src/run/links.rs:70-99`, routed at `src/cmd/run.rs:144-145`) with these params, refusals and payloads; the file's channel-methods groups detail every other served method but carry no `link` / `unlink` entry, and the plan's expected amendments name §Standard Contracts Channel methods (served, their `-32602` causes).
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 59
    section: ## Standard Contracts
    change: In the `wait` group's wake sentence, name the now-written `unlink` record among the wrapper records that never wake — "Hook lines signal, and of the wrapper's own records only `dialog-expired`, appended through `WaitFeed::appending` (the wrapper's `send-*`, `link`, `unlink` and start records never wake)."
    sidecar: standard-contracts — `wait`'s never-waking wrapper records gain `unlink`, now written by a product path (chunk 2026-10-26-explicit-session-links)
    rationale: The report's Symbols / APIs record that a product path now appends an `unlink` event line (`source: wrapper`) for the first time; `unlink` is not in `WAIT_WAKE` (the six kinds the same group lists), so it joins the wrapper records that never wake, which the sentence enumerates.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 79
    section: ## Standard Contracts
    change: New group (values · contract-event-data) after the `prompt-submitted` group — "- `link` / `unlink`: `{driver, driven}`, `source: wrapper`, appended by the driven instance's wrapper only. A `link` line is appended once per new pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or on the served `link` method; a held pair appends none. Since chunk 2026-10-26-explicit-session-links a product path appends `unlink`: the served `unlink` method, through `LinkSet::remove` (its first product caller), one line when it removes a held pair and none for an absent pair. Both are log-only and never wake `wait`; a replay learns the link set from them (`replay::apply_link_line`)."
    sidecar: standard-contracts — new event-data group for `link` / `unlink`, the product `unlink` line and its writer `LinkSet::remove` (chunk 2026-10-26-explicit-session-links)
    rationale: The report's Symbols / APIs and Schema / config state the product `unlink` event line (kind in the closed set, `data: {driver, driven}`, `source: wrapper`) is now written, `LinkSet::remove` having its first product caller; the plan's expected amendments name §Standard Contracts Event data (a product `unlink` line), and the file's event-data detail holds no `link` / `unlink` entry.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:82
note: The body's own stale lines in this section — architecture.md:193 (`link`/`unlink` answered `-32601`, verbs held), :220 (no product path appends `unlink`), :157 / :178 (server-check and `from`-reading lists) and :51 ([Links] verbs held) — sit outside this category file and belong to the body's detector; D-arch-decisions found no new library, runtime or contradicted decision (Dependencies: none).
