@task_prose part=session
The session made the CLI's browser-tools prompt (Yes, use my browser) a third compiled modal literal beside the trust and imports literals, and a new first-screen word browser-tools.
It added a 24th relied ledger row, browser-tools-prompt, whose verdict is one of three words: pass, absent, or fail; absent counts toward verified on this row alone.
viola verify now prints 24 step lines ([NN/24]); a step line's last word is pass, absent or fail, and the summary's n pass counts every row that counts (an absent row included), so the all-pass summary reads 24 pass 0 fail.
A verified version now needs every relied row to count (pass, or absent on browser-tools-prompt), no longer every relied row pass; a stamp of the 23 pre-chunk ids reads unverified.
viola send and the run's pre-send gate refuse at once while the browser-tools prompt stands (exit 13, nothing typed); verify types nothing into it.
Test-only: the fake agent gains a --start-screen option; the live rig gains VP and VK arms with cache-a and browser-tools-prompt summary lines, the founder's own word allowing the rig to read the cache key name only.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: edited — the compiled screen signatures: the modal literals and the first-screen classification of a start
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the capability ledger: the relied rows, each row's verdict word, the verified-version rule and the stamp
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited — unit tests of the ledger row readings
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited — unit tests of the ledger row checks (modal signature, trust record)
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — unit tests of the ledger stamp and the verified-version rule
@task_file n=6 path=src/cmd/verify.rs
what: edited — the viola verify verb: its step lines, their verdict word, and its summary line
@task_file n=7 path=src/cmd/verify/typed/pty.rs
what: edited — viola verify's typed runs: the settle on a modal screen
@task_file n=8 path=src/cmd/verify/typed/turns.rs
what: edited — viola verify's typed runs: the turns typed into a ready input box
@task_file n=9 path=src/run/gate.rs
what: edited — the wrapper's pre-send gate: refusing a send while a modal stands
@task_file n=10 path=src/run/version_gate.rs
what: edited — the version gate: whether the installed CLI version is verified by the stamp
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: edited — the test-only fake agent and its argv options
@task_file n=12 path=tests/cli_send.rs
what: edited — integration tests of viola send and its refusals
@task_file n=13 path=tests/cli_verify.rs
what: edited — integration tests of viola verify output: step lines and summary
@task_file n=14 path=tests/cli_verify/runs.rs
what: edited — integration tests of viola verify's typed runs
@task_file n=15 path=tests/cli_verify/stamps.rs
what: edited — integration tests of viola verify's stamp
@task_file n=16 path=tests/contract_ledger_probes.rs
what: edited — contract test of the ledger row ids and their probes
@task_file n=17 path=crates/viola-e2e/src/harness/run.rs
what: edited — the test harness: its local-live ledger row list and the expected verify output
@task_file n=18 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the test-only live rig: its arms and summary lines
@task_file n=19 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the test-only live rig's private scratch home and its seeded config
@task_file n=20 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — rehearsal tests of the live rig over the fake agent
@task_item n=1 part=touches
contract/viola-verify-output/step-line: a step line's verdict is now pass, absent or fail (absent on the browser-tools-prompt row alone), and the counter is [NN/24].
@task_item n=2 part=touches
contract/viola-verify-output/summary: n pass now counts every row that counts, an absent row included; the all-pass summary reads 24 pass 0 fail.
@task_item n=3 part=touches
contract/viola-verify/summary: the same summary line change — n counts pass plus absent on the browser-tools-prompt row.
@task_item n=4 part=touches
contract/viola-verify-output/exit-0: its rule when every relied row passes is now when every relied row counts (pass, or absent on browser-tools-prompt).
@task_item n=5 part=touches
contract/viola-verify-output/exit-1: exit 1 now follows any relied row not counting; an absent on any row but browser-tools-prompt does not count.
@task_item n=6 part=touches
contract/viola-verify-output/failing-row: an absent line is not a failing row; only fail is.
@task_item n=7 part=touches
contract/viola-verify-output/plain-ascii: the new verdict word absent and the new row id browser-tools-prompt are plain ASCII words on the step line.
@task_item n=8 part=touches
contract/viola-verify-output/help-paragraph and contract/viola-verify-output/record: check whether either names the relied-row count or the verdict words, now 24 rows and three words.
@task_item n=9 part=touches
decision/words-carry-every-state: a new state word, absent, is carried by a word on the verify step line, no colour or glyph.
@task_item n=10 part=touches
contract/viola-send-output/input-not-ready-hint and contract/cli-exit-codes/13: viola send now refuses at once while the browser-tools prompt stands, as for any modal (exit 13, nothing typed).
@task_item n=11 part=touches
contract/cli-verbs/send: the send's pre-send gate reads a third modal literal, the browser-tools prompt.
@task_item n=12 part=touches
decision/human-output-is-contract: the verify step-line and summary output moved (new verdict word, 24 lines), an output contract change.
@task_item n=13 part=touches
contract/viola-verify-output/stop: check whether its line names the step counter or row count, now [NN/24].
@task_fact n=1 slug=surface-cli
fact: The viola verify step line of the browser-tools-prompt row reads absent when no typed run met the prompt; absent counts toward verified and toward the summary's n pass on that row alone, while the step line and stamp keep the word absent.
@task_fact n=2 slug=surface-cli
fact: viola verify types nothing into the CLI's browser-tools prompt; it reads the prompt as a modal, and until a person answers it once the version stays unverified.
@task_item n=1 part=untouched
The viola list board, the BAY line and the list --json document: no board column, word or width moved.
@task_item n=2 part=untouched
viola revive, viola run start output and their --json refusal codes: no start refusal or output line moved.
@task_item n=3 part=untouched
viola link and unlink output, viola last, viola wait and viola answer output: untouched.
@task_item n=4 part=untouched
The viola mcp tools and their results: no tool added or changed.
@task_item n=5 part=untouched
CLI token rendering, SGR styling, TTY detection and colour depth: the new word absent has no styling of its own.
@task_item n=6 part=untouched
CLI exit codes other than 13, the --json exit form and the internal-error contracts: no code added or moved.
@task_item n=7 part=untouched
viola --help verb groups and usage lines, the wheel verbs pause and release, and the frame verb exit-21 hints and details: untouched.
@task_item n=8 part=untouched
The web-surface absence and the CLI deep-links absence: no surface added.
