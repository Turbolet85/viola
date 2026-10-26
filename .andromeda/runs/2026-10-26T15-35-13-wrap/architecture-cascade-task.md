# Task — architecture · 2026-10-26-explicit-session-links

## The session
The driven instance's wrapper now serves the channel methods `link` and `unlink` (before, both answered `-32601`), through `LinkSet::link` / `LinkSet::unlink` and a shared `driver_of` check.
New CLI verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` were added; the verb's instance is the driven one.
The verbs run, in the founder's form: `check_instance` on the home and `instances/<driven>` before any role file, log or snapshot read; then `client::start`; then the liveness-only pre-check; then the server identity check through `cmd::client::connect`; then one request.
`LinkSet::remove` gets its first product caller (the `unlink` method), so a product path now appends an `unlink` event line (`source: wrapper`).
Retired: the hold that kept the explicit verbs unbuilt pending the founder's word, the `-32601` answer for `link` / `unlink`, and the dead-code allow on `LinkSet::remove`.
Counts moved: served channel methods from seven plus `hook.event` to nine; CLI frame verbs from six to eight; `check_instance(` product call sites under `src/` from 2 to 3; `cmd::client::connect` callers from 6 to 8.

## Reached files
- src/cmd/link.rs
- src/cmd/unlink.rs
- src/run/links.rs
- src/cmd/run.rs
- src/cmd/mod.rs
- src/human/verbs.rs
- src/human.rs
- tests/channel_links.rs
- tests/channel_endpoint.rs
- tests/cli_output_plain.rs
- tests/security_negatives_server_identity.rs
- tests/cli_links.rs
- .config/nextest.toml
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- [Links] decision (established-decisions dec-links, and its body summary): "only the implicit link is built, the explicit verbs held" is now stale. The explicit verbs and the served methods are built in the instance-check form, with no dated gap.
- dec-links names the route entry "Explicit session links" as the pending owner of the hold (established-decisions.md:150). That owner is now stale, because the entry is built.
- Channel methods contract, the `link` / `unlink` entry: it says neither is built, the wrapper answers `-32601`, the verbs are held, and the answer is `{}`. All of that is stale. Both are served now. `link` answers `{since, already_linked}` and `unlink` answers `{}`.
- Channel methods contract, the `from` paragraph: it lists which methods read `from` (`send`, `wait`, `last`, `answer`, `pause`). Neither `link` nor `unlink` reads `from`, so a re-read should find this claim unmoved.
- Event data contract, the closing paragraph: "No product path appends `unlink` yet: its one writer, `LinkSet::remove`, has no caller until the `unlink` method is served" is stale.
- Event data contract: the same paragraph says the wrapper appends `link` only on an implicit link (an issued `send` or a taken `answer`). The wrapper now also appends `link` on an explicit `link` method call.
- Instance snapshot `links` entry: the set now also changes on an explicit `link` / `unlink`. Each method answers only after its line and the snapshot write, and the set still follows the log.
- Conventions, CLI exit codes: exit `21` now also covers the strict-modes refusal of `viola link` / `viola unlink`, alongside `viola list` and `viola plugin install`. It also covers their server-identity mismatch.
- Conventions, CLI exit codes: exit `20` (wrapper protocol fault) now carries a `-32602` from `link` / `unlink`, for example a `driver` equal to `driven`.
- Conventions, CLI line: `viola link <driver> <driven>` and `viola unlink <driver> <driven>` now exist as built and take `--json`. The CLI adds `from` from `VIOLA_NAME`, and the wrapper ignores it.
- Conventions, Channel methods request list: it already names `link` and `unlink`. Re-read it: they are now served rather than listed-only.
- Conventions, `-32602` bad params: the code gains causes from `link` / `unlink`. These are a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, and a self-pair. Each answers `data: null` and writes no line.
- Conventions, `-32603`: it now also covers a `link` / `unlink` record or remove failure.
- CLI `--json` output contract: the `instance-unreachable` document (exit 21) is now also printed by the strict-modes refusal of `link` / `unlink`, with stderr empty. Their success documents are `{"v":1,"ok":{since, already_linked}}` and `{"v":1,"ok":{}}`.
- [API Style] decision: its list of requests carrying an `id` includes `link` and `unlink`. That stays true, and they are now dispatched.
- Instance snapshot `pid` / `started_at`: the server check that every frame-writing client makes now covers the two new verbs. The claim is true by class.
- Occupied resources, filesystem (occupied-resources.md:106): "these two product call sites" of `check_instance(` is stale. There are now three, adding the one site the link verbs share.
- Occupied resources, the `instances/<ViolaName>/` entry: "viola list, viola revive and hook statusline check these files before they use them, ahead of any log" now also holds for `viola link` / `viola unlink`.
- Occupied resources (occupied-resources.md:14): a statement of the six CLI frame verbs is stale. There are now eight: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`.
- Occupied resources, `diagnostics/` `cli-<name>.ndjson`: `link` / `unlink` write the `cli-<driven>` role file (`process-start` / `process-exit`). A `server-verify-failed` exit there carries `exit_code:21` and `during:"connect"`.
- Occupied resources, Subcommands list: it already names `link` and `unlink`. Re-read it: they are now built verbs.
- Occupied resources, test-only channel endpoints: this entry lists the channel test files that create `viola-test-chan-*` endpoints. The new channel test file reaches a booted wrapper's own endpoint through raw `viola_channel::Client::request` frames. Check whether the list or any test inventory needs it.
- Occupied resources, repository and test-gate facts: the nextest `ci` profile's 20 s stamped-home kill list gains `binary(channel_links)`.
- Served channel method count: from seven plus `hook.event` to nine (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`, plus the `hook.event` notification). This applies wherever architecture states the served set or count.
- [MCP] decision: `pause`, `link` and `unlink` stay CLI-only. Re-read it: the claim holds, and the verbs now exist.
- Owner citations: any architecture line naming "Explicit session links" as the owner of an unbuilt `link` / `unlink` is stale.

## New facts
- F1 · standard-contracts · A `link` of a pair already held appends nothing and answers the same `since` with `already_linked: true`. A new pair answers `since` (the new `link` line's `ts`) with `already_linked: false`.
- F2 · standard-contracts · `unlink` answers `{}` whether or not a pair was removed. An absent pair appends nothing.
- F3 · standard-contracts · `link` and `unlink` read neither `from` nor `conn` as identity: a `from` of any shape changes nothing. Each answers only after its event line and the snapshot write have landed.
- F4 · standard-contracts · A `-32602` from `link` / `unlink` (bad or missing name, a foreign `driven`, `driver` equal to `driven`) writes no line. A record or remove failure is `-32603`, and the set is left unchanged.
- F5 · conventions · A self-pair (`driver` equal to `driven`) is refused only by the wrapper (`-32602`, CLI exit 20), never client-side.
- F6 · established-decisions · The explicit verbs run `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`. That is ahead of any role file, log or snapshot read, because opening a log narrows an existing home to 0700. Then come the liveness pre-check and the server identity check (the founder's form, a boundary widening with no dated gap).
- F7 · conventions · The link verbs' strict-modes refusal is exit 21 (as for `list` and `plugin install`), not `revive`'s exit 1. A reply of an unexpected shape is an internal error, exit 1.
- F8 · standard-contracts · CLI `viola link --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` and `viola unlink --json` prints `{"v":1,"ok":{}}`. A refused strict-modes check prints the `instance-unreachable` document with stderr empty.
- F9 · established-decisions · The wrapper writes neither a span nor a process-log line for `link` / `unlink`. Its existing `channel-request` / `channel-response` pair carries `method:"link"|"unlink"`. The CLI side runs inside the `cli.link` / `cli.unlink` dispatch span.
- F10 · established-decisions · The link line's `since` is stamped with `Utc::now()` directly, outside the `Clock` seam.
- F11 · standard-contracts · A channel response line for an unserved method carries no `method` key, because the channel labels only its closed method list.

## Untouched
- The MCP tool set (`send`, `wait`, `last`, `answer`, `list`) and its plugin names: no tool was added, and `link` / `unlink` stay off the driver's tool surface.
- The `list` payload and the board: no link is shown there. The MCP and board halves of the two version capabilities are carried to the route entry "Version done-check", not built.
- The closed set of normalised event kinds and the event line envelope: `link` and `unlink` were already kinds with `data: {driver, driven}`, and no kind or field was added.
- The snapshot envelope and the instance snapshot's field list: no field was added, and `links` keeps its `{driver, driven, since}` shape. Only its writers grew.
- Stack and technologies, workspace crates and module boundaries: no dependency, version or crate changed. The root bin only gained two command modules.
- Environment variables: no new read. `from` comes from the existing `VIOLA_NAME` plumbing.
- IPC endpoint naming, socket directories and the Windows pipe security: unchanged.
- Capability ledger rows and CLI version compatibility: no row was added or relied on, and no `claude` behaviour is involved.
- Hook transport, the hook contract, the dialog hold and the statusline wrapper: none was reached.
- `config.json` keys, the `budget.json` read and the budget governor: no file, key or threshold was added or changed.
- `viola revive`, its preflight, the session chain and the `--json` revive codes: unchanged.
- The wheel, `pause` / `release` semantics and `release-from-driver`: unchanged.
- Fixtures, schemas, fuzz targets and the vendored ConPTY binaries: none was added or changed.
