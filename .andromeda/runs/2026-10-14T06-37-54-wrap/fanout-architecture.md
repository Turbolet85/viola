proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (the ledger-rows bullet on the statusline)"
    change: "Replace the statusline bullet's 'None of the three has a row yet …' with: Statusline, rows `statusline-override` (a `--settings` statusLine runs in the status line's place) · `statusline-shell` (a statusline command runs through /bin/sh) · `statusline-rate-limits` (the statusline payload carries two whole-number rate_limits windows), landed at chunk 2026-10-13-owed-ledger-rows; they are relied only where the status line is wrapped (`statusline::WRAPPED_HERE`, Unix), so on Unix the statusline pass-through and the budget gate's `budget-paused` refusal rest on stamped rows; the Windows legs stay owed to \"Windows-only live measurements\"."
    sidecar: "CLI Version Compatibility: the three statusline shapes are now the ledger rows statusline-override, statusline-shell and statusline-rate-limits, relied on Unix only (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs: `LedgerRow` gains StatusLineOverride, StatusLineShell and StatusLineRateLimits; `relied_on` is false for the three off Unix; Expected amendments name architecture.md:79 (sl-shapes ×2). The body still says none of the three has a row."
    basis: "architecture.md:79; crates/viola-agent-claude/src/ledger.rs:128-147"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (the ledger paragraph: rows, Probes, verdict)"
    change: "The ledger holds twenty-one rows (the members of `LedgerRow::ALL`, in its order, ending `paste-newline` · `statusline-override` · `statusline-shell` · `statusline-rate-limits`), widened by chunk 2026-10-13-owed-ledger-rows; `LedgerRow::relied()` is the per-OS relied set (`relied_on(row, statusline::WRAPPED_HERE)`: all 21 on Unix, the first 18 on Windows) and a version is verified only when its stamp holds every relied row `pass` (was every landed row); `check(row, &Probes)` reads `Probes { print, typed, trusted, dialogs, statusline }` (`statusline`: Run B's `StatusLineCapture`s in `k` order): the override row passes on at least one capture, the shell row on at least one capture all `bin-sh`, the rate-limits row on a capture whose `rate_limits.five_hour` and `.seven_day` each hold an integer `used_percentage` 0-100 and an integer `resets_at` (`whole_rate_limits`, narrower than `statusline::reading`)."
    sidecar: "CLI Version Compatibility: eighteen rows become twenty-one; the verdict and verify's row walk read LedgerRow::relied() (21 Unix / 18 Windows); Probes gains statusline (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`LedgerRow::ALL` is `[Self; 21]`; `verified` iterates `relied()`; `Probes.statusline`; the three check arms) and Counts (rows18 at architecture.md:91 ×4)."
    basis: "architecture.md:91; crates/viola-agent-claude/src/ledger.rs:128-147; crates/viola-agent-claude/src/ledger.rs:898-913"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (the `viola verify` step lines)"
    change: "verify checks the relied rows (`check_rows` walks `LedgerRow::relied()`) and prints one step line per relied row, `MM` being the relied count: 21 on Unix (the last `[21/21] statusline-rate-limits …`), 18 on Windows (the last `[18/18] paste-newline …`); the summary's n + m equals MM."
    sidecar: "CLI Version Compatibility: verify's step counter MM is the relied count, 21 on Unix and 18 on Windows (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`src/cmd/verify.rs`: `check_rows` walks `LedgerRow::relied()` and the step line's total is its length) and Counts (step-counter: architecture.md:91 ×3)."
    basis: "architecture.md:91"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (a non-null dialog decision and older stamps)"
    change: "A non-`null` dialog decision needs a stamp whose relied rows all pass (was: whose eighteen rows pass); on Unix a stamp holding exactly the eighteen ids every stamp written before chunk 2026-10-13-owed-ledger-rows holds, all `pass`, reads unverified, so every Unix home stamped before that chunk reads `cli_verified:false` until its next `viola verify`; on Windows the same stamp reads verified."
    sidecar: "CLI Version Compatibility: the dialog-decision stamp condition is the relied set; an 18-id stamp reads unverified on Unix and verified on Windows (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs: 'on Unix an 18-id all-`pass` stamp reads unverified, on Windows verified'; `src/run/version_gate.rs` reads `verified` over `relied()`."
    basis: "architecture.md:91"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (the statusline pass-through's rows)"
    change: "Replace 'The statusline pass-through landed at chunk 2026-10-10-statusline-pass-through with no statusline row; its rows are owed' with: it landed with no row; its three rows (override, shell, rate_limits) landed at chunk 2026-10-13-owed-ledger-rows, probed by Run B's probe `--settings` (Unix only) and stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  21 pass  0 fail`, that chunk's `evidence/live-round.md`); the Windows legs stay owed to \"Windows-only live measurements\"."
    sidecar: "CLI Version Compatibility: the statusline pass-through's owed rows landed (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (the three rows and their check arms) and Outcome (the record round `stamped 2.1.287  21 pass  0 fail`); the body still calls the rows owed."
    basis: "architecture.md:91"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (Run B's spawn)"
    change: "Run B, on a wrapped OS (Unix) and only for shell-plain paths, also gets one `--settings <probe dir>/typed/statusline.json` after the `--plugin-dir` pair (`typed::trusted_args`; `Inputs.statusline_settings`), the document `statusline::probe_document` builds: a `statusLine` command `<pinned> hook statusline --capture <probe dir>/typed/statusline --shell \"$0\"`, so each status line payload is filed by the hidden capture arm with its shell verdict; on Windows nothing is passed; Runs A, C and D keep their argv and no run is added (the founder's own fork-3 answer, chunk 2026-10-13-owed-ledger-rows `inputs#I1`)."
    sidecar: "CLI Version Compatibility: Run B carries one probe --settings on Unix (the statusline capture document), the founder's fork-3 answer (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`statusline::probe_document`, `typed.rs` `trusted_args` appends `--settings` when Some, Unix; Run B the one caller), Counts (Run B's `--settings` 0 → 1, Unix only) and Expected amendments (architecture.md:91 settings-flag ×2, capture-arm ×1)."
    basis: "architecture.md:91; crates/viola-agent-claude/src/statusline.rs:113-119; src/cmd/verify/typed.rs:147-157"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Plugin Scope] (verify's unwrapped children)"
    change: "Runs C and D, and Run B on Windows, run the user's own status line; on Unix Run B's status line is the probe document's capture command (its one probe `--settings`), not the user's; all three still run the user's global hooks and leave the CLI's transcripts."
    sidecar: "Plugin Scope: on Unix Run B's status line is the probe capture, not the user's (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs: Run B's `--settings` carries a statusLine that 'runs in the status line's place' (the `statusline-override` row's words); the Plugin Scope sentence 'Runs B, C and D … also run the user's own global hooks and status line' no longer holds for Run B on Unix."
    basis: "src/cmd/verify/typed.rs:147-157"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Occupied Resources → Claude Code integration names (verify's probe plugin and child flags)"
    change: "Run B with cwd `<cwd>/.viola-verify-<pid>/` and, on Unix for shell-plain paths, `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (the statusline probe document); on Windows no flag is added."
    sidecar: "Occupied Resources: Run B's child flags gain the Unix probe --settings (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`typed.rs` `trusted_args`; Run B the one caller) and Counts (Run B's `--settings` 0 → 1, Unix only)."
    basis: "src/cmd/verify/typed.rs:147-157"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Cross-cutting Patterns → Capability ledger as the single gate"
    change: "Retire 'the statusline pass-through's three shapes … have no row yet … The rows are owed …' and the `rate_limits`-outside-the-ruled-limit sentences: the three shapes are the rows `statusline-override` · `statusline-shell` · `statusline-rate-limits` since chunk 2026-10-13-owed-ledger-rows, relied on Unix only and stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  21 pass  0 fail`), so `budget-paused` now rests on a stamped row; the Windows legs stay owed to \"Windows-only live measurements\". Keep the paste-newline chunk's `stamped 2.1.287  18 pass  0 fail` as that chunk's dated reading."
    sidecar: "Capability ledger: the statusline pass-through's three relied shapes are rowed (chunk 2026-10-13-owed-ledger-rows); the 18-pass reading kept as history."
    rationale: "Report Changes → Symbols / APIs (the three rows) and Expected amendments (architecture.md:472 owed-rows ×6, sl-shapes ×6; its 18-pass reading is a dated live reading, kept)."
    basis: "architecture.md:472"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Cross-cutting Patterns → Capability ledger as the single gate (the dialog rows' cli_verified)"
    change: "The dialog rows sit inside the relied-set `cli_verified` (`LedgerRow::relied()`: 21 rows on Unix, 18 on Windows), not 'the eighteen-row (`LedgerRow::ALL`) `cli_verified`'."
    sidecar: "Capability ledger: cli_verified is read over the per-OS relied set (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Counts (rows18 at architecture.md:472 ×2, one moving to the per-OS rule) and Symbols / APIs (`verified` iterates `relied()`)."
    basis: "architecture.md:472"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Cross-cutting Patterns → Capability ledger as the single gate (viola revive's `--resume` row)"
    change: "`claude --resume <id>`'s row (and `/compact`'s) is owed to the new working-route entry \"Resume and compact ledger rows\", directly after \"Owed ledger rows\" (the founder's fork 1, chunk 2026-10-13-owed-ledger-rows `inputs#I1`, card `inputs#I2`), no longer to \"Owed ledger rows\"."
    sidecar: "Capability ledger: the --resume row's owner moves to \"Resume and compact ledger rows\" (founder fork 1, chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Cross-cutting claims (`inputs#I1` forks; `inputs#I2` card item 1, mint \"Resume and compact ledger rows\" with CARRY 2 `--resume` and CARRY 3 `/compact`) and Expected amendments (architecture.md:472 resume-compact)."
    basis: "architecture.md:472"
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → hook statusline (the shell-out)"
    change: "Replace 'The shell is relied on with no ledger row' with: the shell is the ledger row `statusline-shell` (probed by verify's capture arm through Run B's `--shell \"$0\"` word, relied on Unix only, chunk 2026-10-13-owed-ledger-rows)."
    sidecar: "Standard Contracts: /bin/sh as the statusline shell is the row statusline-shell (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`StatusLineShell`, `ShellVerdict`, the capture arm's `--shell` word); the shell-out bullet still says no row."
    basis: "crates/viola-agent-claude/src/ledger.rs:604-610"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Occupied Resources → Claude Code integration names (per-session settings override)"
    change: "Replace 'That the child takes a `statusLine` given through `--settings` is relied on with no ledger row' with: it is the ledger row `statusline-override`, probed by Run B's probe `--settings` and relied on Unix only (chunk 2026-10-13-owed-ledger-rows)."
    sidecar: "Occupied Resources: the settings override's reliance is the row statusline-override (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`StatusLineOverride`, 'a --settings statusLine runs in the status line's place')."
    basis: "crates/viola-agent-claude/src/ledger.rs:128-147"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → Ledger stamps envelope"
    change: "A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all 21 on Unix, the first 18 on Windows), so an all-`pass` 18-id stamp reads verified on Windows and unverified on Unix."
    sidecar: "Standard Contracts: the stamps verdict reads the per-OS relied set (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs: 'The stamp verdict `verified` iterates `LedgerRow::relied()` (was `ALL`)'; the envelope text says every one of its rows."
    basis: "crates/viola-agent-claude/src/ledger.rs:143-147"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → CI/CD approach"
    change: "`viola verify` runs in CI against the fake agent over the relied rows (`LedgerRow::relied()`: all twenty-one on the Unix runners, the eighteen on `windows-2025`), not 'all eighteen ledger rows (`LedgerRow::ALL`)'."
    sidecar: "CI/CD approach: verify's CI rows are the per-OS relied set (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Counts (rows18 at registries/contracts/architecture/ci-cd-approach.md:20) and Harness / gate surface (`LEDGER_ROWS` 21 Unix / 18 `not(unix)`)."
    basis: ".andromeda/registries/contracts/architecture/ci-cd-approach.md:20"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → Crate dependency direction"
    change: "The pure `ledger` module holds twenty-one rows (`LedgerRow::ALL`) and the per-OS `relied()` / `relied_here`, `ShellVerdict`, `StatusLineCapture`, `statusline_variants` and the `StatusLine.<k>` name helpers; the pure `statusline` module adds `WRAPPED_HERE` (now `pub`), `BIN_SH`, `SHELL_FLAG` and `probe_document`."
    sidecar: "Crate dependency direction: viola-agent-claude's ledger and statusline symbol lists follow the statusline rows (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Counts (rows18 at registries/contracts/architecture/crate-dependency-direction.md:6) and Symbols / APIs (the new `pub` items in `ledger` and `statusline`); no dependency added."
    basis: ".andromeda/registries/contracts/architecture/crate-dependency-direction.md:6"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Standard Contracts → list"
    change: "The `claude agents --json` join-field ledger row and its probe are owed to \"Unwrapped sessions on the board\" with the read itself (the founder's fork 2, chunk 2026-10-13-owed-ledger-rows `inputs#I1`, card `inputs#I2`), no longer to \"Owed ledger rows\"."
    sidecar: "Standard Contracts: the agents --json join row moves to \"Unwrapped sessions on the board\" (founder fork 2, chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Cross-cutting claims (`inputs#I2` card item 2: the `claude agents --json` join row and its probe move onto \"Unwrapped sessions on the board\", from the founder's fork 2) and Expected amendments (architecture.md:291 agents-json ×3)."
    basis: "architecture.md:291"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (the `claude agents --json` join-field row)"
    change: "Name the row's owner: owed with its probe to \"Unwrapped sessions on the board\" (the founder's fork 2, chunk 2026-10-13-owed-ledger-rows `inputs#I1`)."
    sidecar: "CLI Version Compatibility: the agents --json join row names its owning entry (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Expected amendments (architecture.md:86 agents-json) and Cross-cutting claims (`inputs#I2` card item 2)."
    basis: "architecture.md:86"
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Hidden flags"
    change: "Add `hook statusline --capture <DIR> [--shell <WORD>]`, the statusline probe capture (`cmd::hook::capture_statusline`), sole caller Run B's probe settings: an absolute, existing `<DIR>`; stdin through `take(MAX_FRAME + 1)`, refused past `MAX_FRAME`; the payload filed raw at the first free `StatusLine.<k>.json` (`create_private_new` claim, `replace_private` 0600) and `StatusLine.<k>.shell` holding exactly `bin-sh` when the word equals `/bin/sh` byte for byte, else `other` (a missing word is `other`), the word itself written nowhere; nothing printed, no `VIOLA_*`, no obs init, no channel, no `budget.json`, no snapshot, exit 0 always; `--shell` on any other arm changes nothing."
    sidecar: "Occupied Resources: the hidden hook statusline --capture <DIR> --shell <WORD> arm registered (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`src/cmd/hook.rs`: `HookArgs` gains a hidden `--shell <WORD>`; the `capture_statusline` arm) and Schema / config ('No … CLI flag added beyond the hidden `hook --shell <WORD>`'); the registry lists only `hook <event> --capture <DIR> [--answers <DIR>]`."
    basis: "src/cmd/hook.rs:404-435"
  - detector: D-arch-resources
    severity: warning
    section: "§Established Decisions → [Hook Contract] (the hidden capture arm)"
    change: "The hidden capture arm is called by `viola verify`'s probe plugins and, as `hook statusline --capture <DIR> [--shell <WORD>]`, by Run B's probe settings `statusLine` command, which files `StatusLine.<k>.json` and `StatusLine.<k>.shell` (`bin-sh` | `other`) under the same rules: no `VIOLA_*`, no obs init, no channel, nothing printed, exit 0."
    sidecar: "Hook Contract: the capture arm's callers and files include the statusline probe capture (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`capture_statusline`, sole caller Run B's probe settings); Hook Contract says only the probe plugins call the arm and names only `<PascalEvent>.<k>.json`."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → hook statusline (the exemption bullet)"
    change: "Add: with `--capture <DIR>` the arm takes the probe capture before the pass-through (no instance check, no `budget.json` write, no command run, nothing printed); without `--capture` the pass-through is unchanged."
    sidecar: "Standard Contracts: hook statusline names its --capture probe arm beside the pass-through (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs: '`hook statusline --capture <DIR>` takes a new arm before the pass-through … `hook statusline` without `--capture` keeps its pass-through unchanged'."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Cross-cutting Patterns → Diagnostic output channels"
    change: "On the `hook statusline` pass-through stdout carries the user's status line output and nothing else; its `--capture` probe arm prints nothing."
    sidecar: "Diagnostic output channels: the statusline capture arm prints nothing (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`capture_statusline`: 'nothing printed, exit 0 on every path')."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → viola home (`ledger/probes/<pid>/`)"
    change: "verify's probe dir also holds `typed/statusline/` (0700; `StatusLine.<k>.json` raw payloads and `StatusLine.<k>.shell` verdicts at 0600, claimed by the hidden statusline capture arm and read by `read_statusline` in `k` order through `take(MAX_FRAME + 1)`, the shell file capped at 16 bytes) and, on Unix when `probe_document` is Some, `typed/statusline.json` (0600, Run B's probe settings); both go with the probe dir's drop guard."
    sidecar: "Occupied Resources: the probe dir's typed/statusline/ captures and typed/statusline.json settings registered (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`src/cmd/verify.rs`: `ProbeDir::create` makes `typed/statusline/` (0700) and writes `typed/statusline.json` (0600); `read_statusline`); the registry lists only `{plugin/,captures/,questions/,plan/}`."
    basis: "src/cmd/verify.rs:411-440"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Repository (`fixtures/claude/<cli-version>/`)"
    change: "`--record` also writes Run B's status line captures as `StatusLine.default-1.json` (the first capture without `rate_limits`) and `StatusLine.rate-limits-1.json` (the first that passes the rate-limits row) (`ledger::statusline_variants`), through the same scrub and refusal, only at 0 failing rows, validated by `schemas/claude-statusline-fixture.v1.json` and hygiene-walked; the fake agent replays them under `--framing`. `2.1.287` gains both, recorded live at chunk 2026-10-13-owed-ledger-rows and copied alone (31 files, the listed classes plus 2 statusline variants), and is stamped at the relied set by `contract_ledger_probes` (its `ROW_IDS` the twenty-one ids; `relied()` 21 on Unix, 18 on Windows) instead of 'eighteen rows … `18 pass  0 fail`'. Their `workspace.repo` names this repository's own GitHub remote and is kept (the founder's word, that chunk's `inputs#I3`)."
    sidecar: "Occupied Resources: the 2.1.287 fixture set gains the two StatusLine variants (31 files) and is stamped at the relied set; a workspace.repo naming this repository's remote is kept (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Files (two new `StatusLine` fixtures), Symbols / APIs (`statusline_variants`; `named_payloads` adds them to what `--record` scrubs), Counts (2.1.287 fixture set +2; rows18 at architecture.md:422 ×2), Schema / config (the kept `workspace.repo.owner`, `inputs#I3`)."
    basis: "architecture.md:422; crates/viola-agent-claude/src/ledger.rs:920-937"
  - detector: D-arch-resources
    severity: warning
    section: "§Established Decisions → [CLI Version Compatibility] (`--record`)"
    change: "`--record` also writes the two status line variants `StatusLine.default-1.json` and `StatusLine.rate-limits-1.json` (`ledger::statusline_variants`) through the same scrub and refusal, only when 0 rows fail."
    sidecar: "CLI Version Compatibility: --record writes the StatusLine variants (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`named_payloads` adds the two `StatusLine` variants to what `--record` scrubs, refuses on `unclean` and writes only at 0 failing rows)."
    basis: "architecture.md:91"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure (`fixtures/claude/<cli-version>/`)"
    change: "The fixtures comment also names the `StatusLine.<k>.json` status line variants recorded by `viola verify`."
    sidecar: "Project directory structure: fixtures/claude names the StatusLine variants (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Files (two new `fixtures/claude/2.1.287/StatusLine.*.json`)."
    basis: ".andromeda/registries/contracts/architecture/project-directory-structure.md"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Repository (schemas)"
    change: "Add `schemas/claude-statusline-fixture.v1.json`: the tolerant schema for recorded status line fixtures (an object, `session_id` required as a string; `rate_limits`, when present, an object requiring `five_hour` and `seven_day` objects; test-side), against which `tests/contract_fixture_hygiene.rs` validates every `fixtures/claude/*/StatusLine.*.json`, every other fixture still against `claude-fixture.v1.json`."
    sidecar: "Occupied Resources: schemas/claude-statusline-fixture.v1.json registered (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Files and Schema / config (new `schemas/claude-statusline-fixture.v1.json`) and Expected amendments (architecture.md:425 fixture-schema)."
    basis: "architecture.md:425; schemas/claude-statusline-fixture.v1.json:1-19"
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure (`schemas/`)"
    change: "The `schemas/` comment lists `claude-statusline-fixture.v1.json` (test-side) beside `claude-fixture.v1.json` and `claude-screen.v1.json`."
    sidecar: "Project directory structure: schemas/ lists claude-statusline-fixture.v1.json (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Expected amendments (registries/contracts/architecture/project-directory-structure.md:50 fixture-schema)."
    basis: ".andromeda/registries/contracts/architecture/project-directory-structure.md:50"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Repository (test-only binaries: `viola-fake-agent`)"
    change: "`--framing` also replays the status line: with `--settings <file>` naming an absolute `statusLine.command`, the command split on ASCII white space and the one word exactly `\"$0\"` handed over as `/bin/sh` (`shell_expanded`), spawned directly (never a shell) with the set's `StatusLine.default-1.json` after the launch SessionStart's hooks and `StatusLine.rate-limits-1.json` once at the end of the first Stop's turn, after the turn screen, each a `statusline` receipt with `stdin_hex`; a relative command is receipted and never run, and a set without the variants fires nothing. No option is added (twelve stands), so `--settings` no longer runs a command 'only with both' it and `--statusline-stdin`."
    sidecar: "Occupied Resources: the fake agent's --framing replays the recorded StatusLine variants through a --settings statusLine command (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "Report Changes → Symbols / APIs (`src/bin/viola-fake-agent.rs` `replay_statusline` and `shell_expanded`; 'No new option'; the fix commit moved the first-Stop replay after the turn screen, Deviations 10)."
    basis: "src/bin/viola-fake-agent.rs:518-535; src/bin/viola-fake-agent.rs:352-364"
note: "Several primaries share detector D-arch-decisions (statusline rows :79, row set :91, Run B --settings, revive --resume owner, agents --json owner) and D-arch-resources (hidden --shell arm, probe dir files, fixtures, schema, fake agent); each dependent's sidecar names the claim it follows. No new dependency, runtime or crate (report Dependencies: none), so D-arch-decisions found no Stack drift. Past readings kept as history: architecture.md:472's `stamped 2.1.287  18 pass  0 fail`."
