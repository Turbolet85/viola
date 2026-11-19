@task_prose part=session
The session landed the recorded fixture set for claude 2.1.328, a byte copy of the live rig's recorder output (28 files, the same names as the 2.1.325 set), and stamped it: the stamped recorded sets go from 3 to 4, the newest stamped set is now 2.1.328.
The drift, recorded-exchange, hygiene and ledger-probe suites read green over every set, 2.1.328 included; no product crate changed, the default CLI version, the harness default and the stamped-home version did not move.
The test-only live rig's context arm gained an after-send transcript lookup and a new closed summary word, context-transcript-after: read|absent|unread, printed by the test-only viola-live-rig and by no viola verb.
One live reading, no verification claimed: on claude 2.1.328 the /context stdout reaches the session transcript as one local-command output tag (transcript absent before the send, read after it).
Nothing was retired; no viola verb, output line, refusal, hint, exit code, MCP tool or token rendering changed.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig's context arm: its after-send transcript lookup, the read/absent/unread transcript word and the new context-transcript-after summary key, with two unit cases
@task_file n=2 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal over the fake agent, the cx case now asserting context-transcript-after: absent
@task_file n=3 path=fixtures/claude/2.1.328/
what: the new recorded CLI fixture set for claude 2.1.328: hook payloads, screen captures, the verify record and the MCP tools-list exchange, recorded under the rig's private config
@task_file n=4 path=tests/contract_ledger_probes.rs
what: the contract ledger probe suite and its list of stamped recorded sets, now four, 2.1.328 added
@task_item n=1 part=touches
Any row whose cell names the stamped recorded sets by member or count, or the newest stamped set: the stamped sets went from 3 to 4 and the newest moved from 2.1.325 to 2.1.328 (the sweep found two layout-templates rows with the word stamped, both read no-change, as the stamped summary line of viola verify).
@task_item n=2 part=touches
The viola verify output rows (summary line, step line, --record, exit 0, failing row): a verify record for 2.1.328 now stands in the recorded set; the output form did not change, but any row naming a version or a stamped set member is to be read.
@task_item n=3 part=touches
The viola verify refusal for a recorded fixture that is not clean: the new 2.1.328 set passed the fixture hygiene walk with no hand edit; the refusal's wording and trigger are unchanged.
@task_item n=4 part=touches
The viola mcp tools rows (answer, last, list, send, wait) and the MCP tool result rows: a recorded tools-list exchange for 2.1.328 was added to the fixtures and the drift suites read it green; the tool set and result shapes did not move.
@task_item n=5 part=touches
The send read-back rows for slash commands (/clear, /compact, /mcp): the live reading on 2.1.328 shows /context output reaching the transcript as one local-command output tag; listing /context as a local command is a carry proposal for the wrap, not landed, so no read-back row changes now.
@task_item n=6 part=touches
The viola send input-not-ready hint and the pre-send gate it reports: a transcript row quoting a modal literal was measured to refuse a send as not-delivered input-not-ready; not mended here, a carry proposal only.
@task_item n=7 part=touches
Human output is a contract, held by the recorded fixtures and drift suites: the drift and recorded-exchange suites now walk one more recorded set; the decision itself is not changed.
@task_item n=1 part=untouched
The web surface absence and the deep-link absence: the session reaches no web or URL surface.
@task_item n=2 part=untouched
The viola list board and BAY line rows (columns, widths, truncation, colour depth, stale rows, rack separator, styling): no board output changed.
@task_item n=3 part=untouched
The CLI token rendering rows and the SGR styling, TTY detection, banner, spinner, pager and footer decisions: no styling or rendering changed.
@task_item n=4 part=untouched
The CLI exit codes, internal-error forms, refusal line form and the exit-21 hints and --json details: no code, refusal or hint moved.
@task_item n=5 part=untouched
The viola revive, viola run start, link/unlink, plugin install, wheel verb and viola wait output rows: none of those verbs or their outputs changed.
@task_item n=6 part=untouched
The clap verb surface, --help usage and verb groups, flat verbs and argument order: no verb, flag or hidden verb was added or removed.
