@task_prose part=session
The session made /context a seventh listed local command for viola send, read back by the one stdout tag it writes to the transcript, with no prompt submitted; it is the fourth post-conditioned local command after /clear, /compact and /mcp.
A send of /context on a verified CLI reads back confirmed by its new tag; on an unverified CLI, or before its transcript exists, it reads unconfirmable with exit 0.
viola verify's Run B pastes /context as its eighth compiled paste, and the ledger gains a 25th relied row, local-command-context; the step lines now count to 25 and the all-pass summary reads 25 pass 0 fail.
A stamp holding only the 24 earlier rows now reads unverified on every binary built after this change, so every stamped version needs a 25-row re-stamp; the live home was not re-stamped.
The live rig, test-only, gained CS and CC labels, a kept stamp, a config top-level reading and hint-cell attribute words; live on claude 2.1.328 in a scratch home, /context passed as a verify probe and as a wrapped send.
No verb, flag, exit code, colour, glyph, hint wording or output column was added or changed.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the ledger of relied rows and the compiled local-command list with each command's post-condition, the row checks and the stamp merge
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit checks of the ledger's row list and ids
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit checks of each relied row's pass and fail check
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit checks pinning the verify probe literals and the local-command list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit checks of the typed verify runs' readings, including the /context row's check
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit checks of stamps.json versions read verified or unverified by their rows
@task_file n=7 path=src/cmd/verify.rs
what: the viola verify verb, its step lines and its summary line
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs that drive the wrapped CLI and collect its readings
@task_file n=9 path=src/cmd/verify/typed/turns.rs
what: viola verify Run B's framing turns, the compiled pastes it types into the CLI
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the fake agent the tests run in place of claude, its framing echo of the verify probe commands
@task_file n=11 path=src/run/send/classify.rs
what: the send readback classifier that turns a send into confirmed, unconfirmable or unable
@task_file n=12 path=src/run/version_gate.rs
what: the version gate that reads a CLI version as verified or unverified from its stamped rows
@task_file n=13 path=tests/cli_send.rs
what: integration tests of viola send and its readback, human and --json
@task_file n=14 path=tests/cli_verify.rs
what: integration tests of viola verify output, step lines and summary
@task_file n=15 path=tests/cli_verify/runs.rs
what: integration tests of viola verify's runs and their step lines
@task_file n=16 path=tests/cli_verify/stamps.rs
what: integration tests of viola verify's stamps.json updates and its pass and fail summary
@task_file n=17 path=tests/contract_ledger_probes.rs
what: the contract check that every recorded fixture set stamps every relied row over the fake agent
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: the end-to-end harness that boots a stamped home, with its pinned list of ledger rows and verify output
@task_file n=19 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig, its labels, arms, private config, kept stamp and summary words
@task_file n=20 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model that reads the readback box's cells as attribute words, never text
@task_file n=21 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary and its rehearse and live words
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent
@task_item n=1 part=touches
The Send readback contracts for listed local commands (/clear, /compact, /mcp): /context joins them as a fourth listed local command, confirmed by one new stdout tag and no prompt submitted.
@task_item n=2 part=touches
The send readback unconfirmable contracts and the readback box unconfirmable state: /context before its transcript exists, or on an unverified CLI, now reads unconfirmable, exit 0, never confirmed.
@task_item n=3 part=touches
The viola send readback mirror rows (open, read back, unconfirmable): /context goes through the same mirror; any row listing the local commands by name is now short by one.
@task_item n=4 part=touches
The viola verify output step line and summary line contracts: the relied row count moved 24 to 25, step lines read [n/25] and the all-pass summary 25 pass 0 fail; a row holding the figure 24 is stale.
@task_item n=5 part=touches
The viola verify output fail word: the new local-command-context row reads fail, never absent, when it fails.
@task_item n=6 part=touches
The CLI verification state contracts (verified, unverified-cli) and the unverified-cli refusal hint: a stamp of exactly the 24 earlier rows now reads unverified, so a CLI version stamped before this change is unverified until re-stamped with 25 rows.
@task_item n=7 part=touches
The decisions against optimistic send confirmation and announcing success before confirmation: /context is confirmed only by its transcript tag, and reads unconfirmable where no tag can be read.
@task_item n=8 part=touches
The readback box signature and its turn-ended rule, indirectly through the classifier: a listed local command fills the readback from its stdout tag, not from a turn end.
@task_item n=9 part=touches
The readback box CLI mirror and the rule that hints never quote text, indirectly through the rig's screen model: the rig now reads every drawn hint cell past the cursor's cell as attribute words only, never text.
@task_fact n=1 slug=brand-identity
fact: Send readback for /context: a listed local command confirmed by one new local-command stdout tag in the transcript with no prompt submitted; on an unverified CLI or before its transcript exists it reads unconfirmable, exit 0.
@task_fact n=2 slug=surface-cli
fact: viola verify Run B pastes /context as its eighth compiled paste, and its result is the relied row local-command-context, reading pass or fail on the step line like every other row.
@task_item n=1 part=untouched
Colour palette, colour tokens, colour depth and colour decision order: no colour, SGR span or amber use changed.
@task_item n=2 part=untouched
Typography, spacing, iconography, glyphs, separators and the viola list board and table: no column, width, glyph or separator changed.
@task_item n=3 part=untouched
CLI exit codes, exit 21 causes, strict-modes refusals, start refusals and revive refusals and their --json codes: no code or cause added or moved.
@task_item n=4 part=untouched
Verb names, flat verbs and viola --help verb groups: no verb, flag or hidden verb added.
@task_item n=5 part=untouched
Refusal hint wording, peer reply screening, wrapper fault detail and the wrapper socket: no hint text, code word or IPC method changed.
@task_item n=6 part=untouched
Flight-progress strip, handoff, pause and release, wait, link, last and answer output: none of these outputs changed.
@task_item n=7 part=untouched
Anti-patterns and the self-validation protocol tests: no spinner, motion, emoji or stack trace introduced.
@task_item n=8 part=untouched
Web page surface and rendered-page absences: viola still renders no page.
