@task_prose part=session
The session made /context a listed local command of the claude ledger, its post-condition its stdout tag in the transcript, so viola send reads a /context send back by that tag.
It added a relied ledger row local-command-context, the 25th, worded «/context writes its stdout tag to the transcript and submits no prompt»; viola verify's Run B pastes /context as its eighth paste and stamps the row with a measured context latency.
verify's step lines now count to 25 and its all-pass summary reads 25 pass 0 fail; a stamp holding only the 24 earlier rows now reads unverified, and a failing row still reads fail, never absent.
A /context send on a verified CLI is confirmed by a new stdout tag; on an unverified CLI, or before its transcript exists, it reads unconfirmable with exit 0 and the --json ok document carrying confirmed false.
It measured live on claude 2.1.328 alone, in a scratch home: the row passed 25/25 and a wrapped /context send wrote one tag and submitted no prompt; the live home stays a 24-row stamp and its re-stamp is outside this session.
No CLI verb, flag, exit code, refusal, hint, styling, board column or MCP tool was added, renamed or retired; the send path's product code is unchanged.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the claude capability ledger: the list of listed local commands with their post-conditions, the relied rows with their ids and words, and each row's check
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit checks of the ledger's rows and their ids
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit checks of each relied row's check
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit checks of the verify probe literals and the local-command list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit checks of the typed verify runs' readings against the relied rows
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit checks of the verify stamp: which rows a version needs to read verified
@task_file n=7 path=src/cmd/verify.rs
what: the viola verify verb: its runs, step lines and summary
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed probe runs against the real claude CLI
@task_file n=9 path=src/cmd/verify/typed/turns.rs
what: viola verify's Run B pastes, the compiled probe literals it types into claude in order
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the fake claude agent the tests run in place of the real CLI
@task_file n=11 path=src/run/send/classify.rs
what: the send outcome classifier: how viola send reads a send back as confirmed, unconfirmable or unable
@task_file n=12 path=src/run/version_gate.rs
what: the version gate: whether the claude CLI version reads verified from its stamp
@task_file n=13 path=tests/cli_send.rs
what: integration checks of viola send's read back and --json document
@task_file n=14 path=tests/cli_verify.rs
what: integration checks of viola verify's output, step lines and summary
@task_file n=15 path=tests/cli_verify/runs.rs
what: integration checks of viola verify's typed runs
@task_file n=16 path=tests/cli_verify/stamps.rs
what: integration checks of the stamp viola verify writes and its pass and fail summary
@task_file n=17 path=tests/contract_ledger_probes.rs
what: the contract check that every recorded fixture set stamps every relied row over the fake agent
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: the end-to-end harness's run of viola verify and its pinned list of ledger rows
@task_file n=19 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig: its labelled live legs, their summary words and teardown
@task_file n=20 path=crates/viola-e2e/src/rig/screen.rs
what: the test-only live rig's screen model, read as cell attribute words, never text
@task_file n=21 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig's binary and its rehearse and live words
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent
@task_item n=1 part=touches
The viola send read back of a listed local command: /context joins /clear, /compact and /mcp as a local command read back by its post-condition, here its stdout tag; the per-command read back rows hold none for /context yet.
@task_item n=2 part=touches
The viola send unconfirmable output: a /context send before its transcript exists, or on an unverified CLI, reads unconfirmable with exit 0.
@task_item n=3 part=touches
The viola send --json ok document: a /context send that cannot be confirmed carries confirmed false with detail unconfirmable and its cursor.
@task_item n=4 part=touches
The readback vocabulary's state words as a /context send uses them (confirmed, unconfirmable): no word added, but a row listing which commands each word covers would now miss /context.
@task_item n=5 part=touches
The viola verify step line: its total moved from 24 to 25 relied rows; a row whose cell holds the figure 24 or the [n/24] form is stale.
@task_item n=6 part=touches
The viola verify summary and summary line: the all-pass summary now reads 25 pass 0 fail; a row whose cell holds 24 pass is stale.
@task_item n=7 part=touches
The viola verify exit 0 and exit 1 outputs: a stamp of only the 24 earlier rows now reads unverified, so a CLI verified before this session reads unverified until re-stamped.
@task_item n=8 part=touches
The viola verify failing row: the new row local-command-context fails as fail, never absent, as the other rows do.
@task_item n=9 part=touches
The viola verify --help paragraph and --record output, where either names what Run B pastes or how many rows it stamps: Run B now pastes /context eighth.
@task_item n=10 part=touches
The send-lines-appended and readback-mirror decisions, where they rule over the issue and outcome lines of a local-command send: /context now takes that path.
@task_item n=11 part=touches
The rule that turn-ended is not a send outcome: a /context send submits no prompt and ends no turn, so its outcome rests on the tag alone.
@task_item n=12 part=touches
The cli-verbs send and verify rows, where they name the local commands a send reads back or the rows verify stamps.
@task_fact n=1 slug=surface-cli
fact: viola send reads a /context send back as a listed local command: confirmed by a new stdout tag in the transcript on a verified CLI, unconfirmable on an unverified CLI or before its transcript exists, never by a turn end.
@task_item n=1 part=untouched
The web surface absence: no page or web output was added.
@task_item n=2 part=untouched
The viola list board, its BAY line, columns, colour depth, truncation and the unwrapped rack: no board output moved.
@task_item n=3 part=untouched
CLI exit codes and the exit-21 hints and --json detail words: no code, cause or hint was added or renamed.
@task_item n=4 part=untouched
The refusal hints decisions (a hint for every cause, hint last on stderr, hints name the next verb): the claude dialog hint cells the rig reads are no viola CLI hint.
@task_item n=5 part=untouched
The viola mcp server and its tools and results: no tool, parameter or result shape changed.
@task_item n=6 part=untouched
viola run, revive, wait, last, answer, link and unlink, plugin install and the wheel verbs: none of their outputs or refusals moved.
@task_item n=7 part=untouched
CLI token rendering, styling, TTY detection, spinner, banner and footer decisions: no styling or output mode moved.
@task_item n=8 part=untouched
The send text refusals before any request (empty text, control character): the /context text is validated as before.
