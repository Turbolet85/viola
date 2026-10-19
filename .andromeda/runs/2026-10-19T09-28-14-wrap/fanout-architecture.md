proposals:
  - detector: D-retired
    severity: warning
    section: §Established Decisions → [Hook Contract] Hooks fail open
    change: >-
      Replace the dialog-deadline sentences with the hold — a sync dialog hook waits on `hook.dialog` for at most the instance's dialog hold, a `viola_core::DialogHold` (nutype over u16, 60–3600 s) read once at the wrapper's start from `config.json` `dialog_hold_s` (default `DIALOG_HOLD_DEFAULT` = 60 s, bounds `DIALOG_HOLD_MIN` = 60 s / `DIALOG_HOLD_MAX` = 3600 s, each PROVISIONAL; the founder's own answer `inputs#I2` Q1 «Ключ, по умолчанию 60», a boundary widening) and handed to the slot by `DialogSlot::hold_for`; the hook bounds its reply read at `DIALOG_HOLD_MAX + 5 s` = 3605 s (the ceiling, not each start's hold), and the dialog hooks' `hooks.json` `timeout` is 3615, above the ceiling by a margin `hooks_json_dialog_timeouts_outlast_the_longest_hold` pins; at the hold's end the wrapper replies `null` (the hook exits 0 with no body and logs `decision_emitted:false, deadline_hit:false`; its own `deadline_hit:true` is only the backstop of its read timing out), clears the pending `dialog_id`, appends one `dialog-expired` line (`source: wrapper`, `{dialog_id, dialog_kind}`, no question, plan or tool text) through `WaitFeed::appending` and logs one `dialog-answered{deadline_hit:true}` with no `from`; a hand-back or an answer writes neither; a failed append changes nothing else; a later `answer` stays `not-delivered` / `unknown-dialog`; as measured on claude 2.1.287 on the Linux dev host (chunk 2026-10-19-dialog-hold-for-the-driver `evidence/live-round.md`) a dialog hook held 671 s returned the driver's `viola answer` decision, its Windows leg owed to "Windows-only live measurements" and a hold near the 3600 s ceiling first read live at "Overseer switch to viola".
    sidecar: >-
      [Hook Contract]: DIALOG_DEADLINE (60 s) retired for the configurable DialogHold (`dialog_hold_s`, 60–3600, default 60); hook bound 65 s → 3605 s; dialog `timeout` 75 → 3615; expiry appends one wrapper `dialog-expired` and logs `dialog-answered{deadline_hit:true}`; the 671 s live reading on 2.1.287 Linux.
    rationale: >-
      The report's Retired bullet names `viola_core::DIALOG_DEADLINE` (its meaning moves to `DIALOG_HOLD_DEFAULT` / `DIALOG_HOLD_MAX`); Symbols / APIs give `DialogHold`, `hold_for`, `record_expiry`, the hook's `dialog_deadline` = 3605 s; Schema / config give `dialog_hold_s` and the `hooks.json` timeout 75 → 3615; Spec claims disproved say the hook's `deadline_hit:true` is written only on its own read timeout; Outcome's live criterion gives the 671.3 s answer, Linux only. The counts sweep reads `architecture.md:66` stale under patterns `deadline` ×3, `timeout75` @c2568 and `hookbound` @c2461.
    basis: .andromeda/architecture.md:66
  - detector: D-retired
    severity: warning
    section: §Established Decisions → [Human Takeover / Wheel]
    change: >-
      Re-word the two "dialog deadline" mentions to the dialog hold — "without waiting out the dialog hold" and "never hidden from the human for the rest of the hold" — so the passage names the term [Hook Contract] now defines instead of the retired constant's.
    sidecar: >-
      [Human Takeover / Wheel]: 'dialog deadline' → 'dialog hold' (×2), following DIALOG_DEADLINE's retirement.
    rationale: >-
      The claim stays true (a wheel move hands the held dialog back at once — Outcome's a11y criterion, `hook_dialog_held_is_handed_back_null_at_once_on_a_wheel_move`), but once [Hook Contract] stops defining a "dialog deadline" the term is orphaned; the report's sweep lists `architecture.md:67` @c9228 as the hold's end sharing the token, reworded if the detector judges it.
    basis: .andromeda/architecture.md:67
    dependent-of: D-retired
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Registered hook events)
    change: >-
      The PreToolUse and PermissionRequest entries' `timeout` reads 3615 (was 75; 15 s over `DIALOG_HOLD_MAX`, as of chunk 2026-10-19-dialog-hold-for-the-driver); the spine `timeout` 5 entries unchanged.
    sidecar: >-
      §Occupied Resources: the dialog hooks' `timeout` 75 → 3615.
    rationale: >-
      Report Schema / config — the embedded `plugin/hooks/hooks.json` two dialog hooks' `"timeout"` go from 75 to 3615; the sweep reads `architecture.md:309` stale under pattern `timeout75`.
    basis: .andromeda/architecture.md:309
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Deployment / Distribution]
    change: >-
      The dialog tier's PreToolUse (`AskUserQuestion|ExitPlanMode`) and PermissionRequest entries each carry `"timeout": 3615` (was 75, raised at chunk 2026-10-19-dialog-hold-for-the-driver to outlast `DIALOG_HOLD_MAX`); the file is still rendered by `@@VIOLA_BIN@@` substitution alone.
    sidecar: >-
      [Deployment / Distribution]: the dialog hooks' `"timeout": 75` → 3615.
    rationale: >-
      Report Schema / config (hooks.json timeout 75 → 3615, rendering unchanged); the sweep reads `architecture.md:95` @c2583 stale under pattern `timeout75`.
    basis: .andromeda/architecture.md:95
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (`config.json`)
    change: >-
      Add the key `dialog_hold_s` — a whole number of seconds from 60 to 3600, default 60, read once at the wrapper's start through `run::dialog::read_hold` (`take(MAX_FRAME)`, `v` = 1, right after `read_thresholds`); an absent, unreadable or malformed document or a `v` other than 1 gives the default with no rejection, and any other value (0, 59, 3601, -1, 600.5, `"600"`, `null`, `true`) gives 60 plus exactly one `parse-rejected{parser:"config-json", detail:"malformed"}`; no value makes the hold zero or unbounded.
    sidecar: >-
      §Occupied Resources config.json: + `dialog_hold_s` (60–3600, default 60, read once at start by `read_hold`).
    rationale: >-
      Report Schema / config — `config.json` gains `dialog_hold_s` with exactly these read and fallback rules; Symbols / APIs give `read_hold`; the sweep reads `architecture.md:339` stale by omission under pattern `cfgkeys`.
    basis: .andromeda/architecture.md:339
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (`schemas/config.v1.json`)
    change: >-
      The schema's properties add `dialog_hold_s` (integer, minimum 60, maximum 3600), and the pinned set reads the six `CONFIG_KEYS` of `src/obs.rs` (was five).
    sidecar: >-
      §Occupied Resources config.v1.json: + `dialog_hold_s` 60–3600; CONFIG_KEYS five → six.
    rationale: >-
      Report Schema / config — `schemas/config.v1.json` gains `"dialog_hold_s": { "type": "integer", "minimum": 60, "maximum": 3600 }` and `CONFIG_KEYS: [&str; 6]` (was 5); the sweep reads `architecture.md:366` ×2 stale under pattern `cfgkeys`.
    basis: .andromeda/architecture.md:366
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Cross-cutting Patterns → Config management (Settings)
    change: >-
      The Settings list adds `dialog_hold_s` — the dialog hold in whole seconds, 60–3600, default 60, a `DialogHold`.
    sidecar: >-
      §Cross-cutting Patterns Config management: Settings + `dialog_hold_s`.
    rationale: >-
      Report Schema / config adds the key; its Expected amendments name "architecture §Cross-cutting Patterns, Config management", and that section's Settings bullet enumerates the budget thresholds, `diagnostics_level` and `claude_env_keep` without it.
    basis: .andromeda/architecture.md
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Conventions (Normalised event kinds)
    change: >-
      The normalised kind list adds `dialog-expired` (last, as in `EventKind::ALL`, which now holds 16).
    sidecar: >-
      §Conventions: normalised event kinds + `dialog-expired`.
    rationale: >-
      Report Symbols / APIs — `viola_core::EventKind::DialogExpired`, kebab `dialog-expired`, `EventKind::ALL` 15 → 16 with the new kind last; Expected amendments name §Conventions (normalised kinds). The list enumerates fifteen kinds without it.
    basis: .andromeda/architecture.md
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (Event `data` per kind)
    change: >-
      Add `dialog-expired`: `{dialog_id, dialog_kind}`, `source: wrapper`, appended once at a held dialog's hold end, carrying no question, plan or tool text.
    sidecar: >-
      §Standard Contracts Event data: + `dialog-expired` `{dialog_id, dialog_kind}` (source wrapper).
    rationale: >-
      Report Schema / config — the event-line contract gains `dialog-expired` `{dialog_id, dialog_kind}`, `source: wrapper`; Symbols / APIs give `data_keys` `["dialog_id", "dialog_kind"]`; Expected amendments name §Standard Contracts (Event `data`).
    basis: .andromeda/architecture.md
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (`wait`)
    change: >-
      The wake set reads `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` (`viola_core::EventKind::WAIT_WAKE`, six), so the channel `outcome` and both `wait`s take the new kind with no edit.
    sidecar: >-
      §Standard Contracts wait: WAIT_WAKE five → six (+ `dialog-expired`).
    rationale: >-
      Report Symbols / APIs — `EventKind::WAIT_WAKE: [Self; 6]` (was 5) adding `dialog-expired`; `wait_outcome` and `src/run/wait.rs`'s scan derive from it unchanged; the sweep reads `architecture.md:223` stale under pattern `wake5`.
    basis: .andromeda/architecture.md:223
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (`wait`)
    change: >-
      Retire "Only hook lines signal (the wrapper's own `send-*`, `link` and start records never wake)" — hook lines and one wrapper record signal: the `dialog-expired` line `DialogSlot::record_expiry` appends through `WaitFeed::appending`; the wrapper's `send-*`, `link` and start records still never wake.
    sidecar: >-
      §Standard Contracts wait: 'only hook lines signal' gains the wrapper's `dialog-expired` exception.
    rationale: >-
      Report Symbols / APIs — `record_expiry` appends the line "through `WaitFeed::appending`, so a parked `wait` wakes"; Schema / config — "It is the first wrapper-appended kind that wakes `wait`". The same line's "Only hook lines signal" restates the log-only claim the sweep's `logonly` pattern retires.
    basis: .andromeda/architecture.md:223
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (wrapper-appended records)
    change: >-
      Add that the wrapper also appends `dialog-expired` (`source: wrapper`, `{dialog_id, dialog_kind}`) once at a held dialog's hold end, and that, unlike `wheel`, `budget-gate`, `send-*` and `link`, it wakes `wait` — the one wrapper-appended kind that does.
    sidecar: >-
      §Standard Contracts: wrapper-appended records gain `dialog-expired`, the one that wakes `wait`.
    rationale: >-
      Report Schema / config ("the first wrapper-appended kind that wakes `wait`"); the sweep reads `architecture.md:254` ×2 stale by omission under pattern `logonly`.
    basis: .andromeda/architecture.md:254
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Message Broker / IPC]
    change: >-
      "`wait` blocks until the next driver-relevant event" lists `turn-ended`, `question`, `permission`, `plan`, `session-end` or `dialog-expired` (the `WAIT_WAKE` six).
    sidecar: >-
      [Message Broker / IPC]: the driver-relevant events `wait` blocks on + `dialog-expired`.
    rationale: >-
      Report Symbols / APIs — `WAIT_WAKE` 5 → 6 adding `dialog-expired`. This passage restates the five-kind wake set in its own words (the `wake5` pattern did not fire on it; the sweep lists `architecture.md:49` only under `pendingsnap` @c701).
    basis: .andromeda/architecture.md:49
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (snapshot `pending_dialog`)
    change: >-
      Add when it is written — for a held dialog, `pending_dialog` is on `snapshot.json` before the dialog's event line is appended, and written back to none when that append fails (`DialogSlot::register`, commit `d146c50c`; written after the append from chunk 2026-10-04-dialog-answers-by-dialog-id until then), so a reader woken by the line sees it.
    sidecar: >-
      §Standard Contracts snapshot: `pending_dialog` written before the dialog's line, cleared on a failed append (d146c50c).
    rationale: >-
      Report Symbols / APIs (`DialogSlot::register`, the operator-pass fix `d146c50c`) and Deviation 4 (the macOS `cross_list_rows` red at `3def5b5b` read `DIALOG none` on a held dialog); the sweep reads `architecture.md:206` stale by omission under pattern `pendingsnap`.
    basis: .andromeda/architecture.md:206
note: "D-arch-decisions found no drift — no dependency changed and `DialogHold` is a nutype, which §Stack allows; the §Stack Domain newtypes row and [Validation] name only `ViolaName` and `Percent` as nutype uses and could add `DialogHold` (no line in the report, left to the orchestrator). No keyed contract file carries a retired claim, and no IPC method, endpoint, port, socket, env var or workspace crate was added."
