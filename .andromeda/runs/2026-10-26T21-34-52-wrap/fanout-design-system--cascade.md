proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 130
    section: "## Surface: cli › ### Component Patterns"
    change: "In the 21 row's human column, replace «(not running / unwrapped / strict-modes / server verification)» with «(not running / unwrapped / server verification)» and widen the carve-out to: «`viola list`'s refused strict-modes check, `viola link` / `viola unlink`'s of the driven instance, and the instance strict-modes check `viola send`, `wait`, `last`, `answer`, `pause` and `release` run first (one helper shared with `link` / `unlink`; no frame and no line written), print instead the fixed home-state or instance-state `unable:` / `hint:` pair, the six frame verbs the instance pair naming their instance (`link`'s form; the founder's own answer, P4 fork Q3, chunk 2026-10-26-home-and-code-bearing-file-integrity, `inputs#I3`)»; in the `--json` column read «the `list`, `link`, `unlink` refusals and the six frame verbs' strict-modes refusals included, with stderr empty»."
    sidecar: "surface.md exit-21 row: the six frame verbs' strict-modes refusal prints link's instance pair (or the instance-unreachable document); strict-modes leaves the instance-unreachable hint causes (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols: `refuse_untrusted` runs `check_instance` first in send, wait, last, answer, pause and release and, refused, prints `human::instance_state_pair(name)` or exactly the instance-unreachable document, exit 21, opening nothing and writing no line; Decisions: the founder's P4 Q2 (no line) and Q3 (link's instance pair)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    fact: F1
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 57
    section: "## Surface: cli › ### Component Patterns"
    change: "Replace the cause «the viola home fails strict-modes: `the viola home is not private to you; <name> is not contacted until that is fixed`» with: «the home or the instance's state files fail the instance strict-modes check (`send` after reading its text, and `wait`, `last`, `answer`, `pause` and `release` first, before any frame): no `unable  instance-unreachable` line; the fixed instance-state pair `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`, exit 21, not drawn as the readback mirror (`link`'s form)»."
    sidecar: "surface.md send hints: the strict-modes cause of instance-unreachable retired; the six frame verbs print link's instance pair instead (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols (client.rs `refuse_untrusted`, send.rs:190-192 after the text read): the refusal prints the instance pair, so the standing strict-modes hint is a wording the chunk replaced (D-design-cli-wording)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    dependent-of: D-new-detail
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 21
    section: "## Surface: cli › ### Tokens (platform-specific)"
    change: "After «and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance,» add «and the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance (chunk 2026-10-26-home-and-code-bearing-file-integrity, the founder's own answer, P4 fork Q3, `inputs#I3`),» so the six frame verbs join the fixed `unable: <text>` form's list."
    sidecar: "surface.md stderr streams: the six frame verbs' exit-21 strict-modes refusal joins the fixed unable: form (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols: the six verbs refuse through `human::refuse` of the instance pair, exit 21; the stream rule lists the fixed form's users and omits them."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    dependent-of: D-new-detail
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 261
    section: "## Surface: cli › ### Tokens (platform-specific)"
    change: "Read «… or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release`)»."
    sidecar: "design-system.md Streams: the six frame verbs join the exit-21 fixed-form list (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "The broad Streams line the surface#2.1 entity details names the same list; report Symbols (`refuse_untrusted` at the six verbs)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    dependent-of: D-new-detail
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 37
    section: "## Surface: cli › ### Component Patterns"
    change: "In the Refusal item's instance-pair parenthetical, after «`viola link` / `viola unlink` print it, naming the driven instance, at their exit-21 strict-modes refusal» add «; `viola send`, `wait`, `last`, `answer`, `pause` and `release` print it, naming their instance, at their exit-21 strict-modes refusal, and `viola run`'s and `viola revive`'s start at an instance-level refusal of their home-tree check (exit 1)»."
    sidecar: "surface.md list Refusal: the instance pair's printers widened to the six frame verbs and run/revive's instance-level start refusal (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes Count 8 (`instance-pair`, design-system surface.md:37 among the three re-read hits: the six frame verbs join the list) and Symbols (run.rs `refuse_tree`: InstanceLevel → `human::instance_state_pair(name)`)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:35"
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 37
    section: "## Surface: cli › ### Component Patterns"
    change: "Add after the Refusal item: «As built (chunk 2026-10-26-home-and-code-bearing-file-integrity): the per-instance check now holds the home to the read rule, so a home that group or other may read (0750) passes with no instance (the empty board) and, with an instance, refuses the whole board naming that instance in the instance-state pair: which pair the board refusal prints depends on whether an instance exists. Pending, owner: a CARRY on the working-route entry «Unix endpoint and home hardening».»"
    sidecar: "surface.md list Refusal: a 0750 home passes an empty board and refuses naming the instance otherwise; the inconsistency owed to «Unix endpoint and home hardening» (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes Count 14 (`board::scan`: `check_path` write rule on the home, then `check_instance` per instance, now holding the home to the read rule; `board.rs` unchanged) and Outcome's route ask naming the owner."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:41"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 65
    section: "## Surface: cli › ### Component Patterns"
    change: "Read the qualifier «(strict-modes at start)» as «(a home-level refusal of the start's home-tree check: the home, the instance set, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/`, each before any log, no child)» and add after it the item «`unable: <name>'s state files can be written by another user` (an instance-level refusal of the same check) → `hint: viola will not read them; make the viola home and its files owner-only`»."
    sidecar: "surface.md run start refusals: the home-tree check before any log; home-level → home pair, instance-level → instance pair, exit 1 (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols (run.rs `checked_exe` → `check_home_tree(home, Some(name), Some(&key))`; `refuse_tree`: HomeLevel → home pair, InstanceLevel → instance pair; before `open_wrapper_log`, exit 1, no log, no line, no child)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18"
    fact: F3
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 67
    section: "## Surface: cli › ### Component Patterns"
    change: "Add to `viola revive`'s list, beside its strict-modes item: «its start's home-tree check (the home, `ledger/`, `bin/`, `plugin/`, the pinned copy), before any log: a home-level refusal → `unable: the viola home is not private to you` → `hint: the viola home must be readable only by you; viola does not change its permissions`; an instance-level refusal → the instance pair above; no child; under `--json` `strict-modes-failed`»."
    sidecar: "surface.md revive refusals: its start's home-tree check adds the home pair (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols: `revive`'s start (revive.rs:125-135) calls `checked_exe` / `refuse_tree` before `open_wrapper_log`, exit 1, revive `--json` keeps `strict-modes-failed`."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18"
    dependent-of: D-new-detail
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 122
    section: "## Surface: cli › ### Component Patterns"
    change: "In the 1 row's human column add `<name>'s state files can be written by another user` to `viola run`'s start refusals (an instance-level home-tree refusal), `the viola home is not private to you` to `viola revive`'s (its start's home-level tree refusal), and to `viola verify`'s refusals `unable: the viola home is not private to you` (its home-tree check of the home, `ledger/`, `bin/` and `plugin/` before its log, no step line and no summary; or a `stamps.json` update refused by strict-modes), each with its own hint."
    sidecar: "surface.md exit-1 row: run gains the instance pair, revive and verify the home pair (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes Counts 8 and 9 name surface.md:122 among the instance-pair and home-pair hits; Symbols (run.rs `refuse_tree`, verify.rs `refuse_home` and `measure`'s `StampsError::StrictModes`)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:36"
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 107
    section: "## Surface: cli › ### Component Patterns"
    change: "Add after the Ctrl-C / TERM sentence: «Before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`: a refusal prints no step line and no summary, only the home-state pair `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`, and exits 1; a `stamps.json` update refused by strict-modes prints the same pair.»"
    sidecar: "surface.md verify: a refused home tree, or a stamps update refused by strict-modes, prints the home pair (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols: verify.rs `check_home_tree(&home, None, None)` before its `cli` log, refused → `refuse_home()` (the home pair), exit 1, no step line, no summary, no line; `measure` maps `StampsError::StrictModes` to the same pair."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 107
    section: "## Surface: cli › ### Component Patterns"
    change: "Add after the home-tree sentence: «On Unix a home under a regular file is a strict-modes refusal, not the `cli` catch-site line: `verify` prints the home pair (exit 1) and the frame verbs the instance pair (exit 21); on Windows such a path stats `NotFound` and keeps its earlier outcome.»"
    sidecar: "surface.md verify: an unstattable Unix home is a strict-modes refusal; Windows unchanged (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes Count 13 and Deviation 8 (`verify_under_a_regular_file_refuses_the_home`, a per-OS oracle: the catch-site line on Windows, where such a path stats `NotFound`)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:40"
    fact: F5
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 330
    section: "## Surface: cli › ### Component Patterns"
    change: "Read «… a run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home-tree check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1»."
    sidecar: "design-system.md verify: the home-tree refusal joins the no-step-line case (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "The broad verify line the surface#10.1 entity details names only the Ctrl-C / TERM stop; report Symbols (verify.rs 64-70)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19"
    dependent-of: D-new-detail
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 303
    section: "## Surface: cli › ### Component Patterns"
    change: "Read «…; `run`'s and `revive`'s start refusals take a fixed-message `unable:` line and one hint per cause, `revive`'s read in a fixed order that stops at the first, and the exit-21 strict-modes refusal of `send` and the other frame verbs prints `link`'s fixed instance-state pair, not drawn as the mirror.»"
    sidecar: "design-system.md send refusals: the frame verbs' strict-modes refusal is link's fixed instance pair, not the mirror (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols (send.rs:190-192: `refuse_untrusted` after the text read, before any frame; the instance pair through `human::refuse`)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    dependent-of: D-new-detail
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 342
    section: "## Surface: cli › ### Component Patterns"
    change: "Read «`viola link` / `viola unlink`'s refused strict-modes check of the driven instance, and that of `viola send`, `wait`, `last`, `answer`, `pause` and `release` on their instance (one shared helper), (exit 21) prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty»."
    sidecar: "design-system.md exit codes: the six frame verbs join link/unlink's instance-pair paragraph (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols: `link_call` moved onto `refuse_untrusted` unchanged in behaviour, now shared by the six frame verbs."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    dependent-of: D-new-detail
note: "surface#8.1 and surface#11.1 hold as written; layout-templates' instance-pair rows (layout-templates.md:253, :329, report Count 8) are that master's; the diagnostics symlink/mode check (surface note at design-system.md:356) leaves the wording true; D-design-tokens: every new surface's tokens is n/a."
