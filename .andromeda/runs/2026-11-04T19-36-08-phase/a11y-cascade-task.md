@task_prose part=chunk
Brings `viola run` on Linux and macOS onto portable-pty's openpty backend behind the same pty seam as Windows' ConPTY: spawn, read, write, resize, wait and kill behave as the Windows wrapper's do, as far as the fake agent can read them.
Makes the full fake-agent suite the same on all three OSes: every end-to-end and integration test of a behaviour the two Unix OSes share runs on ubuntu-latest and macos-latest, and each test left cfg(windows), cfg(unix) or cfg(target_os) names the reason its behaviour belongs to one OS.
Re-reads the obs gates G1 to G4 and the CI test, perf and lint jobs, which must be green on ubuntu-latest and macos-latest beside windows-2025, and keeps one diagnostics schema on every OS (process-log lines, events.ndjson, snapshots, every --json document).
Takes up the carried integration-tier witness of dim only on stale rows (v1-40) under the outer PTY. It also carries the wrapper-kill read on Unix and the reply-probe hypothesis, all on the fake agent with no live claude session; Windows behaviour, pins and tests do not move.
@task_file n=1 path=.github/workflows/ci.yml mark=re-reads
what: the test, perf and lint jobs over matrix.os reading green on ubuntu-latest and macos-latest beside windows-2025
@task_file n=2 path=schemas/diag-line.v1.json
@task_file n=3 path=tests/support/home.rs
@task_file n=4 path=tests/run_cli.rs
@task_item n=1 part=needs
The cli surface's output discipline, run on the ubuntu-latest and macos-latest legs: no SGR, glyph or cursor control of viola's own under non-TTY, NO_COLOR, TERM=dumb, --json, viola run or a passed viola revive; every SGR span beside its word; C0/C1 control characters escaped except \n and \t, which list rows escape too.
@task_item n=2 part=needs
The tui passthrough boundary under the openpty wrapper (path P4): viola writes zero terminal bytes except the child's output, human keystrokes go through unblocked past the current atomic paste, and focus, mouse and resize sequences and terminal replies do not count as human editing and do not move the wheel on Linux and macOS. On windows-2025 the platform fact stays that an injected mouse report takes the wheel (F-W3).
@task_item n=3 part=needs
The boundary after a contained vt100 feed panic: no viola literal on the outer terminal on every OS, and the child's own bytes on Unix.
@task_item n=4 part=needs
Visual SGR values the outer-PTY witness of dim only on stale rows reads: dim SGR on stale rows only, bold only on NAME values, amber only on the DIALOG word, each state still carried by its word, unable and fail never coloured. The plan's Owed line under ARIA Patterns & Roles assigns this witness to the Linux and macOS parity entry.
@task_item n=5 part=needs
Unwrapped NAME truncation under the outer PTY: cut with ... only on a terminal too narrow, never inside a \xHH escape; a wrapped NAME, piped output and --json are never cut. This applies when resize behaves under openpty as it does under ConPTY.
@task_item n=6 part=needs
Path P6 board, wait/last and verify output on the Unix legs: linear static output, waiting: <name> once on stderr and on a TTY only, verify step lines [NN/NN] … pass|fail, the CAPTION literal order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the -- UNWRAPPED - READ-ONLY -- separator only when unwrapped rows exist.
@task_item n=7 part=needs
Error recovery layout on all three OS legs: unable <reason> <detail> on stderr then hint: as the last stderr line, typed exit codes, and hint-free --json. The exceptions are unknown (exit 14, no hint) and the answer and send exit-2 fixed stderr line with no document. Diagnostics must carry the same words on every OS.
@task_item n=8 part=needs
The per-OS split the error-recovery verification names: the frame verbs' server-verify peer pid witness runs on windows-2025 and ubuntu-latest only. On macOS there is no peer pid, so the word is an euid mismatch alone and no test reads it as a pid or start-time comparison (the recorded macOS carve-out). The socket-directory refusal (one unable: <text> line) runs on ubuntu-latest and macos-latest.
@task_item n=9 part=needs
viola revive --json on the Unix legs: every refusal of the start it shares with run prints one exit-1 document (batch-script-child, pinned-hash-mismatch, squatted-name, program-not-found internal-error), and a pump_child error prints none, because the child has held the terminal.
@task_item n=10 part=needs
Fixed CLI strings stay identical on every OS: they are pinned by trycmd or, for viola list, --help, plugin install, the server-verify, socket-directory, unwrapped-name and endpoint-lost hints and the link/unlink result lines, by literal. Any OS-only wording fails these pins.
@task_item n=11 part=needs
Every time bound the Unix wrapper holds (send readiness gate, confirmation windows, dialog hold, status-line deadline, the wait timeout and cap, the budget pause) falls on the driver, the user's command or viola's own child, never on the human: no key is blocked, refused or delayed by it.
@task_item n=12 part=needs
Where the cli and tui cases run and land: inside nextest-integration / nextest-e2e locally and, in CI, inside each per-OS test job's single instrumented coverage suite. Each leg's gate --require coverage,doctest is its own verdict and never allows skips; failures land in the run JSON's suites[].failures[]. A breach (suite-failed, suite-skipped, suite-missing, artifact-missing) fails the build.
@task_item n=13 part=needs
The fake-agent Unix suite reads only viola's own bytes and the boundary. It never parses the rendered claude TUI screen for content and never takes screenshots as a11y evidence (R7).
@task_item n=14 part=needs
The P4 cli leg on the Unix legs: send refused with human-typing on stderr, with a typed exit code, and kebab-case codes never shown without the plain word unable.
@task_standing n=1
convention: each new cli output-discipline or tui boundary test runs inside the existing nextest-integration / nextest-e2e suites (CI: the per-OS coverage suite) on all three OS legs; no new suite enum value such as a11y without a Decisions Log entry, and no separate a11y driver outside scripts/agent-run.* run
home: ## 9. CI Integration
@task_standing n=2
convention: each new test obeys zero flakiness: retries 0, no retry-once policy for a11y violations, #[ignore] banned, a flake keeps the chunk red until the root cause is fixed
home: ## 10. SLO Invariants & A11y Budgets
@task_standing n=3
convention: no new cli or tui test result is cited as WCAG conformance evidence; assert_cmd / trycmd / portable-pty results are output-discipline stand-ins for SC 1.3.2, 1.4.1 and 3.3.1 only
home: ## 11. A11y Anti-Patterns
@task_standing n=4
convention: no new a11y artifact upload and no stage or performance budget of its own; a cli or tui failure is a nextest failure in the harness run JSON
home: ## 9. CI Integration
@task_standing n=5
convention: an output-discipline invariant that lives in crate tests falls under the epoch-boundary mutation audit (run --mutants)
home: ## 10. SLO Invariants & A11y Budgets
@task_item n=1 part=not_needed
The frontend's keyboard navigation, screen-reader and visual-design rules for 0.2.0: the frontend stays deferred and its technology is unchosen.
@task_item n=2 part=not_needed
The Claude Code plugin entity (hooks, hook statusline, stdio MCP server) and the wrapper IPC channel: they are not-assertable machine-to-machine traffic with no UI surface, and hook keeps its contract.
@task_item n=3 part=not_needed
The internal crates entity: it is library-only and not assertable.
@task_item n=4 part=not_needed
The manual screen-reader passes (NVDA, VoiceOver, Orca) and the manual keyboard pass on P4 and P6: they are founder-owned supplements, and this chunk runs no live session and no attended pass.
@task_item n=5 part=not_needed
The tui screen-reader row for Windows (NVDA + Windows Terminal, the O1 step) and the Windows ConPTY byte reading owed to Windows-only live measurements: no Windows behaviour moves in this chunk.
@task_item n=6 part=not_needed
The compliance cross-check and the not-triggered wcag-mapping and regulated-compliance triggers: no regime applies to any surface.
@task_item n=7 part=not_needed
The not-triggered target-size and multi-language triggers: there is no touch surface and no i18n convention, and Cyrillic, emoji and CJK appear only as passthrough content.
@task_item n=8 part=not_needed
Plain-language reading level: SC 3.1.5 is not claimed and no readability scorer is introduced (D-A11Y-16).
@task_item n=9 part=not_needed
The A11y Decisions Log history and its amendments archive: they are history, not a rule over this change.
@task_item n=10 part=not_needed
The a11y tier justification and the POUR signal pyramid in their own right: they summarise the strategy and set no requirement beyond the surfaces and paths named in the needs.
