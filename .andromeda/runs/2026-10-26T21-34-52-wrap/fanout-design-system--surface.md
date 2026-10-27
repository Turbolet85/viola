proposals:
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 130
    section: '## Surface: cli › ### Component Patterns'
    change: '| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause''s hint (not running / unwrapped / server verification); a refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal): `viola list`''s (the founder''s own answer, 2026-10-13, relayed verbatim by the Viola overseer), `viola link` / `viola unlink`''s of the driven instance, and `send`, `wait`, `last`, `answer`, `pause` and `release`''s instance-state pair of the named instance (the founder''s own answer to P4 fork Q3 of chunk 2026-10-26-home-and-code-bearing-file-integrity, `inputs#I3`, relayed by the overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), the strict-modes refusals of `list`, `link`, `unlink` and the six frame verbs included, with stderr empty |'
    sidecar: 'cli-exit-codes exit-21 row: the six frame verbs (send, wait, last, answer, pause, release) print link''s instance-state pair at a refused strict-modes check; strict-modes leaves the instance-unreachable hint causes (chunk 2026-10-26-home-and-code-bearing-file-integrity, founder P4 Q3, inputs#I3)'
    rationale: 'Report Symbols: refuse_untrusted runs check_instance first in send/wait/last/answer/pause/release and prints human::instance_state_pair(name), exit 21, or under --json exactly the instance-unreachable document; Decisions: the founder''s Q3 (the six verbs print link''s instance pair); Count 8 names surface.md:122 among the instance-pair rows to extend; the row still lists strict-modes as an instance-unreachable hint cause and names only list/link/unlink as printing the pair.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 57
    section: '## Surface: cli › ### Component Patterns'
    change: '    - the viola home or the instance fails strict-modes: no `instance-unreachable` line and no hint of this list; the verb prints the instance-state pair `unable: <name>''s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`, exit 21, as `link` does (chunk 2026-10-26-home-and-code-bearing-file-integrity, the founder''s own answer to P4 fork Q3, `inputs#I3`, relayed by the overseer)'
    sidecar: 'cli-send instance-unreachable hints: the strict-modes cause''s hint `the viola home is not private to you; <name> is not contacted until that is fixed` replaced by the instance-state pair the six frame verbs print (chunk 2026-10-26-home-and-code-bearing-file-integrity, inputs#I3 Q3)'
    rationale: 'Report Symbols (client.rs refuse_untrusted: refused -> human::instance_state_pair(name), Some(21)) and Decisions (founder Q3): the strict-modes refusal of every frame verb now prints the instance pair, so the standing strict-modes hint of instance-unreachable is a wording the chunk replaced.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:111
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 21
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: '- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`''s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder''s own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator''s answer superseded 2026-10-13), and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance and `send`, `wait`, `last`, `answer`, `pause` and `release` make of the named instance (chunk 2026-10-26-home-and-code-bearing-file-integrity, the founder''s own answer to P4 fork Q3, `inputs#I3`), use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal''s `hint:` line directly after it, and errors (`error: …`).'
    sidecar: 'cli-output stderr: the six frame verbs'' exit-21 strict-modes refusal joins the fixed `unable:` form list (chunk 2026-10-26-home-and-code-bearing-file-integrity, inputs#I3 Q3)'
    rationale: 'The line closes the fixed-form list with «only …»; report Symbols (refuse_untrusted -> human::refuse of the instance pair) and founder Q3 add the six frame verbs to it.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 37
    section: '## Surface: cli › ### Component Patterns'
    change: '- **Refusal** (exit 21; the founder''s own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator''s answer at that chunk''s planning, superseded 2026-10-13): a failed strict-modes check of the home, `instances/` or any one instance refuses the whole board, nothing from state printed, in the fixed `unable:` / `hint:` form of the start refusals (pattern 2): the home → `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions` (`list` is this pair''s first writer in source; `viola run` and `viola verify` print it at their exit-1 home-tree refusal); an instance → `unable: <name>''s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` (`revive`''s pair, shared; `viola link` / `viola unlink` print it naming the driven instance, and `send`, `wait`, `last`, `answer`, `pause` and `release` naming the named instance, at their exit-21 strict-modes refusal; `viola run` at its exit-1 instance-level start refusal).'
    sidecar: 'cli-list Refusal: the pairs'' other writers named — the six frame verbs and run (instance pair), run and verify (home pair) (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Count 8 names surface.md:37 as a row listing the verbs that print the instance pair, the six frame verbs joining it; Count 9 and Symbols (run.rs refuse_tree: HomeLevel -> home_state_pair, InstanceLevel -> instance_state_pair; verify.rs refuse_home -> home_state_pair) add run and verify as writers.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:35
    dependent-of: D-design-cli-wording
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 37
    section: '## Surface: cli › ### Component Patterns'
    change: '- As built since chunk 2026-10-26-home-and-code-bearing-file-integrity: `list`''s home check holds the home to the write rule while each instance''s check holds it to the read rule, so a home another user may only read (Unix 0750) passes a board with no instance and, with one, is refused with the instance pair naming that instance rather than the home pair: an inconsistency, `board.rs` unchanged, its owner the route pin this wrap''s P5 sets.'
    sidecar: 'cli-list: the read-widened-home inconsistency of list''s refusal pair recorded with its owner pending P5 (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 14)'
    rationale: 'Report Count 14: board::scan runs check_path (write rule) on the home, then check_instance per instance, which now holds the home to the read rule, so a 0750 home passes the board with no instance and refuses naming the instance (ScanRefused::Instance) with one; Route asks for P5 give it an owner. It changes which pair list prints for that home, a component state of this file.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:41
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 65
    section: '## Surface: cli › ### Component Patterns'
    change: '    - `unable: <name>''s state files can be written by another user` (strict-modes at start on `instances/<name>/` and the instance''s paths, before any log opens) → `hint: viola will not read them; make the viola home and its files owner-only` (`revive`''s pair, shared; chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    sidecar: 'cli-send run start refusals: run''s exit-1 instance-level strict-modes refusal prints revive''s instance-state pair (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Symbols: run.rs refuse_tree maps TreeRefused::InstanceLevel to human::instance_state_pair(name) through human::refuse, before open_wrapper_log, exit 1, no log, no line, no child; Outcome: run refuses a widened instances/<name> with its pair. run''s per-cause list (lines 59-65) states no such line: an unstated new wording. New line following line 65.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 65
    section: '## Surface: cli › ### Component Patterns'
    change: '    - `unable: the viola home is not private to you` (strict-modes at start, before any log opens, on the home tree: the home, `ledger/`, `bin/`, `plugin/`, the pinned key''s `bin/<key>/` and copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists) → `hint: the viola home must be readable only by you; viola does not change its permissions`'
    sidecar: 'cli-send run start refusals: the home pair''s reach widened to the whole home tree, checked before any log opens (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 9)'
    rationale: 'Report Symbols: run.rs checked_exe runs check_home_tree(home, Some(name), Some(&key)) — the home, ledger/, bin/, plugin/, bin/<key>/, the pinned viola, plugin/<key>/, hooks/, .claude-plugin/ — and refuse_tree prints the home pair for HomeLevel; Count 9 names surface.md:65 «(strict-modes at…» as the site where the home pair stands.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 72
    section: '## Surface: cli › ### Component Patterns'
    change: '    - at the start, after a passed preflight and before any log opens, a refused strict-modes check of the home tree: `run`''s home pair, or for the instance''s paths `<name>''s state files can be written by another user` and its hint (under `--json` the `strict-modes-failed` document; chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    sidecar: 'cli-send revive refusals: revive''s start runs run''s home-tree check before its log, printing run''s home pair or the instance pair (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Symbols: revive''s start (revive.rs:125-135) calls checked_exe / refuse_tree before open_wrapper_log, exit 1, no log, no line, no child, «revive --json keeps strict-modes-failed»; revive''s fixed-order list (lines 66-72) has no start-time home-tree cause. New line following line 72.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 107
    section: '## Surface: cli › ### Component Patterns'
    change: ' `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe. A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary: stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI` (pattern 2''s fixed form, plain ASCII, no path or pid), and it exits 1. A refused strict-modes check of the home tree (the home, `ledger/`, `bin/`, `plugin/`, each that exists; on Unix a home under a regular file included), made before its first log line, prints no step line and no summary: stderr carries the home pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, and it exits 1 with `stamps.json` unchanged; the stamps write refused by its own check maps to the same pair (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'cli-verify-run verify: the exit-1 home-pair refusal of a widened home tree or a refused stamps write (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Symbols: verify.rs runs check_home_tree(&home, None, None) before its cli log, refused -> refuse_home() (human::home_state_pair through human::refuse), exit 1, no step line, no summary, no line; measure''s StampsError::StrictModes maps to the same pair; Count 13: on Unix a home under a regular file is now this refusal. The verify entry states no such output.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 122
    section: '## Surface: cli › ### Component Patterns'
    change: '| 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `<name>''s command is a .cmd or .bat script` / `the viola home is not private to you` / `<name>''s state files can be written by another user`, or a `viola revive` refusal `unable: <name>''s state files can be written by another user` / `<name> has no logged session to resume` / `<name> has no logged session with that id` / `<name>''s session is open in another claude` / `<name>''s recorded directory is missing` (or `run`''s `already live` pair, or at its start `run`''s strict-modes pairs), or a `viola verify` refusal `unable: the viola home is not private to you` / `the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; codes `home-path` · `absolute-path` · `username` · `email`; never the content), each followed by its own hint, or a `viola verify` stopped by Ctrl-C or TERM before its rows are checked, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (stdout empty); a `verify` run with a failing ledger row prints no stderr word (its last stdout line reads `stamped <ver>  <n> pass  <m> fail`) | `viola revive --json`''s refusals of its strict-modes check and its preflight: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; arch §Standard Contracts); every other exit 1 has no `--json` document (a `run` start refusal reached through `revive` included, owed to "Exit-cause code catalogue") |'
    sidecar: 'cli-exit-codes exit-1 row: run''s instance-state pair, revive''s start strict-modes pairs and verify''s home pair joined (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 9)'
    rationale: 'Report Count 9 names surface.md:122 as a site where the home pair stands; Symbols: run''s refuse_tree prints the instance pair at InstanceLevel, revive''s start calls the same check, verify''s refuse_home prints the home pair — none of these three stands in the exit-1 row.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:36
    dependent-of: D-design-cli-wording
note: design-system's body (.andromeda/design-system.md:261 the stderr fixed-form list, :303, :330 verify, :332 run, :338 exit 21 phraseology, :342 link/unlink's pair) owes the same six-verb / run / verify additions — outside this category file, for the body's detector; D-design-tokens no drift (every Coverage row tokens n/a); no Retired item stands in this file.
