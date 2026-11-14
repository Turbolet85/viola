@task_prose part=session
The session retired the usage-budget gate whole: no rate-limits reading, no budget file, no send budget rung, no budget-paused state, refusal, hint or board word, no release --budget, no Path 6 suite.
Exit code 11 (was budget-paused) is retired, never renumbered or reused; nothing maps to it, and an older wrapper's budget-paused reply now reads as unknown, exit 14.
viola list drops the budget member of its --json document (now items and skipped) and the BAY line's budget cells (5H, 7D, read, gate); the BAY line reads BAY then the skipped names; the WHEEL column keeps width 23 with no budget-paused form.
viola release takes no --budget flag (clap refuses it, exit 2); the channel release refuses any budget key as invalid-params and replies with the wheel holder alone; the driver release guard stays.
viola verify runs 23 steps and relies on 23 ledger rows (the status-line rate-limits row gone); --record writes the default status-line fixture alone.
A start writes one start record (the wheel start) before the hook session-start; every own-format version stays 1.
@task_file n=1 path=src/cmd/list.rs
what: the viola list verb: the board and its --json document envelope
@task_file n=2 path=src/human/board/bay.rs
what: the BAY line of the viola list board, its cells and their order
@task_file n=3 path=src/human/board/cells.rs
what: the board's cells, the WHEEL cell's words and the column widths
@task_file n=4 path=src/human/board/rows.rs
what: the board rows of viola list: per-instance lines and their states
@task_file n=5 path=src/human/board.rs
what: the viola list board writer
@task_file n=6 path=src/human.rs
what: the human output of the CLI verbs: the board writer's entry and the wheel verb lines
@task_file n=7 path=src/human/hints.rs
what: the refusal hints, one per refusal cause, the last stderr line
@task_file n=8 path=src/human/verbs.rs
what: the spoken output lines of the wheel and handoff verbs
@task_file n=9 path=src/cmd/release.rs
what: the viola release verb: its arguments and its output
@task_file n=10 path=src/cmd/send.rs
what: the viola send verb: its refusals and the CLI exit code map
@task_file n=11 path=src/cmd/verify.rs
what: the viola verify verb: its step lines, summary and --record
@task_file n=12 path=src/cmd/run.rs
what: the viola run start and the wrapper's channel method dispatch
@task_file n=13 path=src/cmd/hook/statusline.rs
what: the status-line hook: the pass-through of the status-line payload
@task_file n=14 path=src/run/wheel.rs
what: the wheel: who holds input, and the channel release with its driver guard
@task_file n=15 path=src/run/send.rs
what: the send pipeline and its refusal order
@task_file n=16 path=src/run/send/slot.rs
what: the send slot that runs one send at a time
@task_file n=17 path=src/run/budget.rs
what: the retired budget gate and its thresholds (deleted)
@task_file n=18 path=src/run/send/budget.rs
what: the retired send budget rung (deleted)
@task_file n=19 path=crates/viola-state/src/budget.rs
what: the retired budget file reader and writer (deleted)
@task_file n=20 path=crates/viola-state/src/board.rs
what: the board items that viola list renders, one per instance
@task_file n=21 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot on disk
@task_file n=22 path=crates/viola-core/src/lib.rs
what: the shared vocabulary: event kinds and refusal reasons
@task_file n=23 path=crates/viola-mcp/src/result.rs
what: the viola mcp tool results: success, refusal and invalid-params
@task_file n=24 path=crates/viola-mcp/src/tools.rs
what: the viola mcp tools and their descriptions
@task_file n=25 path=crates/viola-channel/src/client.rs
what: the channel client the frame verbs call the wrapper through
@task_file n=26 path=crates/viola-agent-claude/src/ledger.rs
what: the ledger of relied claude CLI rows that viola verify checks
@task_file n=27 path=crates/viola-agent-claude/src/statusline.rs
what: the claude status-line payload handling
@task_file n=28 path=src/obs.rs
what: the config keys and the diagnostics setup
@task_file n=29 path=schemas/config.v1.json
what: the config file schema
@task_file n=30 path=schemas/diag-line.v1.json
what: the diagnostics line schema
@task_file n=31 path=src/bin/viola-fake-agent.rs
what: the fake agent that replays claude in tests
@task_file n=32 path=tests/cli_list.rs
what: the viola list board and --json checks
@task_file n=33 path=tests/cross_list_rows.rs
what: the board row checks across instances, the BAY literal
@task_file n=34 path=tests/cli_wheel.rs
what: the wheel verb checks, the release guard
@task_file n=35 path=tests/cli_machine_contract.rs
what: the machine contract checks of the --json documents and exit codes
@task_file n=36 path=tests/cli_verify.rs
what: the viola verify checks: step lines and summary counts
@task_file n=37 path=tests/cli_budget.rs
what: the retired Path 6 budget CLI suite (deleted)
@task_file n=38 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the retired Path 6 budget end-to-end suite (deleted)
@task_file n=39 path=fixtures/viola-0.1.0/retired-words.json
what: the retired 0.1.0 words that compatibility cases read (new)
@task_file n=40 path=src/cmd/mod.rs
what: the CLI verb table and global flags
@task_item n=1 part=touches
contract/cli-exit-codes/11 states 11 as budget-paused: set it to 11 retired (was budget-paused), never reused.
@task_item n=2 part=touches
contract/cli-exit-codes/14 and contract/cli-exit-codes/json-form: unknown (14) now also reads an older wrapper's budget-paused reply; no code maps to 11.
@task_item n=3 part=touches
contract/release-budget-output/gate-lifted and contract/release-budget-output/gate-open: viola release --budget is gone with both lines; retire.
@task_item n=4 part=touches
contract/viola-list-board/gate-closed: the closed budget gate board form is gone; retire.
@task_item n=5 part=touches
contract/bay-line/cell-order: the BAY line's 5H, 7D, read and gate cells are gone; it reads BAY then the skipped names.
@task_item n=6 part=touches
decision/bay-line-unstyled and decision/only-header-bay-line: their BAY line lost its budget cells; set where they name them.
@task_item n=7 part=touches
contract/viola-list-json/document and contract/cli-verbs/list: the --json document is items and skipped, no budget member; list reads no budget or thresholds.
@task_item n=8 part=touches
contract/viola-list-board/column-widths and contract/viola-list-board/wheel-unknown: the WHEEL cell has no budget-paused form; its width stays 23 though its widest word left.
@task_item n=9 part=touches
contract/wheel-verb-output/release: release replies with the wheel holder alone, no budget_paused or override_until; the budget lifted/open lines gone.
@task_item n=10 part=touches
decision/argument-order-mirrors-channel: release params lose budget (from alone); a --budget flag is now refused.
@task_item n=11 part=touches
decision/hint-for-every-cause and decision/hints-name-next-verb: the budget-paused hint is gone with its cause.
@task_item n=12 part=touches
contract/viola-send-output/word-column, decision/shared-refusal-word-column and contract/readback-vocabulary/words: budget-paused leaves the refusal words.
@task_item n=13 part=touches
contract/cli-verbs/send and decision/wrapper-repeats-send-refusals: the send refusal order loses its budget rung.
@task_item n=14 part=touches
contract/mcp-tool-results/refusal and contract/mcp-tools/list: the budget-paused MCP refusal sample and the list description's usage budget are gone.
@task_item n=15 part=touches
decision/words-carry-every-state and decision/board-planning-forms-wording: budget-paused is no longer a board state word.
@task_item n=16 part=touches
contract/viola-verify-output/step-line, contract/viola-verify-output/summary and contract/viola-verify/summary: 23 steps and 23 rows (was 24); set any count held.
@task_item n=17 part=touches
contract/viola-verify-output/record: --record writes the default status-line fixture alone, no rate-limits fixture.
@task_item n=18 part=touches
contract/viola-verb-help/usage and contract/viola-help/verb-groups: release's usage loses --budget.
@task_item n=19 part=touches
contract/viola-run/start-output and contract/viola-revive/start-output: a start no longer evaluates the budget; one start record before session-start.
@task_item n=20 part=touches
contract/viola-list-board/plain-lines and contract/viola-list-board/uncoloured-modes: any budget-paused or BAY budget form in their lines leaves.
@task_fact n=1 slug=surface-cli
fact: viola has no usage-budget gate: no rate-limits reading, no budget file, no budget-paused state or refusal, no budget display on the board.
@task_fact n=2 slug=surface-cli
fact: viola release --budget is refused as an unknown argument (exit 2); the channel release refuses any budget key as invalid-params and moves no wheel.
@task_fact n=3 slug=surface-cli
fact: an older wrapper's budget-paused reply reads as unknown, exit 14, at the CLI.
@task_item n=1 part=untouched
absence/web-surface and absence/cli-deep-links: no page and no link coupling came or went.
@task_item n=2 part=untouched
The CLI token rendering rows (colours, radius, spacing, motion, roles): no colour or token moved.
@task_item n=3 part=untouched
The viola revive refusals, its --json error codes and --list rows: no revive refusal or code changed.
@task_item n=4 part=untouched
The viola link and unlink output rows: unchanged.
@task_item n=5 part=untouched
The viola plugin install rows: unchanged.
@task_item n=6 part=untouched
The viola wait output rows and viola answer output rows: unchanged.
@task_item n=7 part=untouched
The frame verb exit-21 hints and the instance-unreachable detail rows: unchanged.
@task_item n=8 part=untouched
decision/no-hint-names-release: still true; no hint names release.
@task_item n=9 part=untouched
Other budgets (performance, latency, retry): no layout row holds them and none moved.
