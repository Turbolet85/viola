proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 18
    section: ## Input Validation
    change: After the `hook.event` re-validation line of the Channel frames row, add one line — **`link` / `unlink` params (wrapper, served since chunk 2026-10-26-explicit-session-links):** `driver` and `driven` are each read as a string through `ViolaName::try_new` (`LinkSet::link` / `LinkSet::unlink` sharing `driver_of` in `src/run/links.rs`); `driven` must be the serving wrapper's own name and `driver` must differ from `driven`; a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` `"invalid params"` with `data: null`, no line written; neither method reads `from` (a `from` of any shape changes nothing); a `record` / `remove` error is `-32603`; each answers after its line and the snapshot write (`link` `{"ok":{"since","already_linked"}}`, `unlink` `{"ok":{}}`).
    sidecar: Input Validation, Channel frames row — the served `link` / `unlink` methods' wrapper-side params check (names through `ViolaName::try_new`, own-name `driven`, no self-pair, `-32602` with `data: null` and no line, `from` unread) recorded (chunk 2026-10-26-explicit-session-links).
    rationale: The report's Symbols / APIs make `link` / `unlink` served channel methods (before `-32601`) with exactly this params validation; the row lists every other served method's wrapper params check but names `link` / `unlink` only by their name type.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 31
    section: ## Input Validation
    change: After the `pause` / `release`, `revive` and `list` arguments line of the CLI arguments / stdin row, add one line — **`link` / `unlink` arguments (chunk 2026-10-26-explicit-session-links):** `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` (one shared `LinkArgs`) take both names through `ViolaName::try_new` (the verbs' shared name parser); the verb's instance is the driven one; their `from` comes from `VIOLA_NAME` only; `driver` equal to `driven` is not refused client-side (the wrapper's `-32602`, exit 20). Both run `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read (the founder's own word at P4 of that chunk, «Сначала instance check», `inputs#I3`): a refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, no line in any log and the directory's mode as found; then the liveness pre-check `live_endpoint` and, on Linux and Windows, the server check through `cmd::client::connect` before their one frame. They read `snapshot.json` only after that check, so `viola link` / `viola unlink` are not among this row's interim gaps.
    sidecar: Input Validation, CLI arguments / stdin row — the new `viola link` / `viola unlink` verbs' arguments and their instance check before any log or snapshot read (exit 21, the `instance-unreachable` `--json` document), not a dated gap (chunk 2026-10-26-explicit-session-links, founder's word `inputs#I3`).
    rationale: The report's Symbols / APIs add the two verbs with `ViolaName` arguments and `link_call`'s order (step 1 `check_instance` before `client::start`, then `live_endpoint`, then `cmd::client::connect`); the row enumerates each frame verb's arguments and every snapshot-reading verb's strict-modes standing, and neither verb is there.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 50
    section: ## Input Validation
    change: In the Own state files row's link-rule sentence, the parenthetical «the wrapper wrote them: the driver from a `from` that passed `ViolaName::try_new`, the driven its own name» becomes «the wrapper wrote them: the driver from a `from` that passed `ViolaName::try_new` (an implicit link) or, since chunk 2026-10-26-explicit-session-links, from a `link` / `unlink` method's `driver` param that passed `ViolaName::try_new` (an explicit link, and the first product `unlink` line), the driven its own name».
    sidecar: Input Validation, Own state files row — a logged link line's driver now also comes from the served `link` / `unlink` methods' validated `driver` param, and a product path now writes `unlink` lines (chunk 2026-10-26-explicit-session-links).
    rationale: The report's Symbols / APIs: `link` calls `record(driver)` and `unlink` calls `remove(driver)` on a `driver` read through `ViolaName::try_new`, and a product path now appends an `unlink` event line (`source: wrapper`, `{driver, driven}`) — the first; the sentence names only `from` as the driver's source.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 51
    section: ## Input Validation
    change: In the replay sentence, «the wrapper's `LinkSet` (its restore at every start and its record of an implicit link)» becomes «the wrapper's `LinkSet` (its restore at every start and its record of an implicit link or, since chunk 2026-10-26-explicit-session-links, of an explicit `link` method)».
    sidecar: Input Validation, Own state files row — `LinkSet`'s record now also serves the explicit `link` method (chunk 2026-10-26-explicit-session-links).
    rationale: The report's Symbols / APIs: the served `link` method calls `LinkSet::record(driver)`, so the record is no longer only the implicit link's.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:24
    dependent-of: D-new-detail
note: D-security-deps clean (report Dependencies: none); D-security-auth reaches this file only through the verbs' strict-modes and server-check order, carried in the CLI arguments proposal; no Retired item is stated in this file (the hold and `-32601` appear nowhere here; line 17's `link`/`unlink` name rule and line 50's known-kind sentence stay true).
