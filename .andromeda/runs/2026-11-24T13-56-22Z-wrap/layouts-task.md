@task_prose part=session
The session added a new CLI verb, viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json], which switches a running wrapped session to a given model and effort in place: the wrapper ends its own child and resumes the same session id on the new pair, and nothing is written to the user's own settings.
The verb is backed by a new wrapper channel method switch {v, sender, model?, effort?} answering {"ok":{"model","effort"}} or a refusal; the channel's served methods went from 9 to 10 requests.
The verb needs at least one half (usage exit 2 otherwise), exits 0, 10, 12, 13, 14, 20 or 21, prints <name>  switched  <model> <effort> on stdout, and refuses human-typing (10), unverified-cli (12), or not-delivered with turn-running, dialog-pending, no-session, child-not-ended or resume-failed (13), the first refusal in that order winning.
viola --help's setup: row now reads open, run, revive, adopt, up, switch, verify, plugin install (17 to 18 served verbs); a send during a switch is refused not-delivered turn-running; a later revive with no pair replays the switched pair.
A new 10 s reap bound (RESPAWN_REAP_BOUND) governs the child's end; F1 (the in-place restart by a channel method), F3 (the refusals mid-turn, mid-dialog, while the human types), F5 (CLI-only, no MCP tool) and F6 (the 10 s bound) stand PROVISIONAL on the delegate overseer's word, none the founder's; no capability is claimed and v2-30 and v2-31 stay unclaimed.
Proven over the fake agent alone; the real-CLI readings R6a and R6b are carried to The fleet on the founder's host.
@task_file n=1 path=src/cmd/switch.rs
what: new file — the viola switch CLI verb: its flags, the strict-modes check, the live endpoint, the request to the wrapper and its refusal and exit-code mapping
@task_file n=2 path=src/run/switch.rs
what: new file — the wrapper's switch channel method: the param checks of the model and effort halves, the refusal order, the respawn of the child on the resumed session with the new pair
@task_file n=3 path=tests/cli_switch.rs
what: new file — the integration tests of the viola switch verb: idle switch, user settings untouched, revive after a switch, the refusals, usage, instance-dead, the --json document
@task_file n=4 path=crates/viola-pty/src/pump.rs
what: edited — the PTY pump that copies the host terminal to and from the child; it now owns a respawned child and gains the respawn handle and the reap time bound
@task_file n=5 path=crates/viola-pty/src/lib.rs
what: edited — the PTY crate's public exports, now naming the respawn handle, its outcome and the reap bound
@task_file n=6 path=crates/viola-channel/src/server.rs
what: edited — the wrapper channel server's closed list of method labels, which gains switch (10 to 11 entries)
@task_file n=7 path=src/cmd/run.rs
what: edited — viola run's wrapper: the child pump and the channel call dispatch, which now routes the switch method
@task_file n=8 path=src/cmd/verify/typed/run.rs
what: edited — viola verify's run step that spawns the child through the pump, now passing a respawn handle no one uses
@task_file n=9 path=src/cmd/mod.rs
what: edited — the CLI verb table and the viola --help text, which now list switch in the setup: row after up
@task_file n=10 path=src/run/mod.rs
what: edited — the wrapper's module list, naming the new switch module
@task_file n=11 path=src/run/send.rs
what: edited — the wrapper's send method, now refused turn-running while a switch stands
@task_file n=12 path=src/run/send/slot.rs
what: edited — the send flight slot: the switch mark that blocks sends during a switch and the one-shot screen of the old child's late session-end
@task_file n=13 path=src/human.rs
what: edited — the human output module's exports, naming the switched line writer
@task_file n=14 path=src/human/verbs.rs
what: edited — the human output lines of the verbs, now with the switched line <name>  switched  <model> <effort>
@task_file n=15 path=src/human/hints.rs
what: edited — the hint table, one hint for every refusal cause, gaining dialog-pending, no-session, child-not-ended and resume-failed
@task_file n=16 path=tests/cli_output_plain.rs
what: edited — the plain-output tests of viola --help, whose served-verb list gains switch (17 to 18 verbs)
@task_file n=17 path=tests/contract_error_bodies.rs
what: edited — the error-body census: the channel's served methods list (10 to 11) and the CLI documents, gaining the three switch refusal bodies
@task_file n=18 path=tests/cross_exit_causes.rs
what: edited — the exit-cause table of refusals, from 1 to 5 rows (dialog-pending, no-session, child-not-ended, resume-failed, each exit 13)
@task_file n=19 path=tests/support/home.rs
what: edited — the test home support: a wrapper adopted as a given user home, beside adopt
@task_item n=1 part=touches
contract/cli-verbs/help: the viola --help output now lists switch among the served verbs, in setup: after up.
@task_item n=2 part=touches
contract/viola-help/setup-row: the setup row now reads open, run, revive, adopt, up, switch, verify, plugin install; a row holding the old row text is stale.
@task_item n=3 part=touches
contract/viola-help/verb-groups: the verb group rows gain switch in setup:; the served verb count moves from 17 to 18.
@task_item n=4 part=touches
The contract/cli-verbs rows are one verb each: a contract/cli-verbs/switch row is owed beside contract/cli-verbs/up and contract/cli-verbs/adopt (usage, at least one half, exits, output).
@task_item n=5 part=touches
contract/cli-exit-codes/10, 12, 13, 14, 20 and 21: a row naming the verbs or causes of each code now has switch among them (human-typing 10, unverified-cli 12, not-delivered with turn-running, dialog-pending, no-session, child-not-ended, resume-failed 13).
@task_item n=6 part=touches
decision/hint-for-every-cause: the refusal causes gain dialog-pending, no-session, child-not-ended and resume-failed for switch, each with its hint; decision/no-hint-names-release and decision/hints-name-next-verb hold over those new hints.
@task_item n=7 part=touches
decision/human-output-is-contract and decision/stdout-results-stderr-rest: a new human result line <name>  switched  <model> <effort> on stdout, plain ASCII, one line, no SGR.
@task_item n=8 part=touches
decision/json-on-every-data-verb and contract/cli-json-mode/one-document: switch takes --json and prints the pair document {"ok":{"model","effort"}} or a refusal document.
@task_item n=9 part=touches
decision/cli-holds-every-action, decision/mcp-hides-wheel-and-handoff-verbs and the contract/mcp-tools rows: switch is CLI-only, no MCP tool (F5, PROVISIONAL).
@task_item n=10 part=touches
decision/argument-order-mirrors-channel: --model and --effort mirror the switch channel method's model and effort params.
@task_item n=11 part=touches
decision/flat-verbs and decision/targets-are-wrapped-names: switch is a flat verb whose target is a wrapped ViolaName.
@task_item n=12 part=touches
contract/frame-verbs/strict-modes-refused and the contract/frame-verb-hints and contract/instance-unreachable-detail rows: switch runs the strict-modes refusal first, then the live endpoint, so it reaches the exit-21 causes (instance-dead among them).
@task_item n=13 part=touches
contract/cli-refusal-line/form and decision/shared-refusal-word-column: the switch refusals print as unable  <name>  not-delivered  turn-running and unable  <name>  human-typing.
@task_item n=14 part=touches
contract/viola-send-output rows and decision/wrapper-repeats-send-refusals: a send while a switch stands is refused not-delivered turn-running, an existing word now with one more cause.
@task_item n=15 part=touches
contract/run-revive-usage/pair-after-dashes and the viola revive start rows: the model and effort halves go through the same model and effort checks as run and revive, and a revive with no pair now resumes on the last switched pair.
@task_item n=16 part=touches
contract/viola-verb-help/usage: viola switch --help prints its own per-verb usage with the --model and --effort flags.
@task_item n=17 part=touches
decision/spoken-phraseology-verb-output: the switched word joins the verb output wording beside the wheel, handoff and dialog verbs.
@task_fact n=1 slug=surface-cli
fact: viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json] switches a running wrapped session to a new model and effort in place, the same session id resumed; at least one half is required (usage exit 2); it exits 0, 10, 12, 13, 14, 20 or 21; it writes nothing to the user's own settings.
@task_fact n=2 slug=surface-cli
fact: viola switch prints <name>  switched  <model> <effort> on stdout, and with --json the document {"ok":{"model","effort"}}.
@task_fact n=3 slug=surface-cli
fact: viola switch refuses in a fixed order, the first rung winning: human-typing (exit 10), not-delivered turn-running and not-delivered dialog-pending (exit 13), unverified-cli (exit 12), then not-delivered no-session, child-not-ended or resume-failed (exit 13); F3 PROVISIONAL.
@task_fact n=4 slug=surface-cli
fact: switch is a CLI-only verb with no MCP tool (F5, PROVISIONAL on the delegate overseer's word).
@task_fact n=5 slug=surface-cli
fact: a send while a switch stands is refused not-delivered turn-running until the new child's session start.
@task_fact n=6 slug=surface-cli
fact: a child not ended within the 10 s reap bound (F6, PROVISIONAL) is not replaced and the switch is refused not-delivered child-not-ended; viola writes no byte to the session's terminal at a respawn.
@task_item n=1 part=untouched
surface-web-spa and absence/web-surface: viola still renders no page.
@task_item n=2 part=untouched
The viola list board, the BAY line, columns, tree, stale rows, colour depth and the --json list document: no board cell or column changed.
@task_item n=3 part=untouched
viola link and unlink output rows: unchanged.
@task_item n=4 part=untouched
viola verify output and refusal rows (step line, summary, failing row, record, stop): the verify run gained no output.
@task_item n=5 part=untouched
viola wait, viola last and viola answer output rows: unchanged words and layouts.
@task_item n=6 part=untouched
The CLI token rendering rows and the SGR styling, banner, spinner, footer, prompt and pager decisions: no styling added.
@task_item n=7 part=untouched
viola plugin install rows and absence/cli-deep-links: unchanged.
