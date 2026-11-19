@task_prose part=session
viola verify now prints one fixed line before its summary when the browser-tools prompt stood at start, saying viola never answers it and the human answers it once in claude; the prompt-absent output is unchanged.
The Claude adapter's ledger gained a public reading of whether a verify run met the browser-tools prompt; no IPC method, event, socket, env var, config key, stamp shape or dependency was added.
The test-only live rig gained two arms: vh runs a verify start under the founder's real home and real Claude config, reading no credentials and keeping key names and one closed needle word only; cx sends /context through viola send under the rig's private config and reads its post-condition from a bounded transcript read.
Measured live on claude 2.1.328: the real home's stamps file now holds a 24-row 2.1.328 stamp, every row passing, other entries kept; both needles for /mcp disable read absent before and after in the real Claude config, and no new projects entry appeared; the /context reading was not delivered and its transcript was absent.
The pinned product was rebuilt from the pushed head and its release build placed beside the product with a rollback copy; the product file was not switched, that step stays with the operator.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the Claude adapter's verify ledger: the relied rows, their checks against compiled literals and the row words verify stamps
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — the unit cases of the verify ledger's stamp rows
@task_file n=3 path=src/cmd/verify.rs
what: edited — viola verify, the command that runs the probes against the real claude CLI, writes the stamps file and prints its human result
@task_file n=4 path=tests/cli_verify.rs
what: edited — the integration tests of viola verify's output and refusals against the fake agent
@task_file n=5 path=tests/support/fake.rs
what: edited — the shared test support that plants the fake agent's screens
@task_file n=6 path=tests/cli_send.rs
what: edited — the integration tests of viola send and run against the fake agent
@task_file n=7 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the test-only live rig: its arms, labels, plans and summary words for live rounds against the real claude CLI
@task_file n=8 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's child isolation: the child environment and the bounded readers of the Claude config by key names
@task_file n=9 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the test-only live rig binary, its live and rehearse commands and their flags
@task_file n=10 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the rehearsals of the live rig against the fake agent
@task_item n=1 part=touches
The rule that the live rig's child is isolated from the user's Claude config gains an exception: the vh arm runs the child under the real home and real config, with no config-dir override.
@task_item n=2 part=touches
The rule that tests use their own home: the vh rig arm is a test-only start pointed at the real home on purpose; its rehearsal refuses vh without an explicit target home, Claude dir and user config.
@task_item n=3 part=touches
The rows that viola writes nothing under the Claude home or in Claude's config: the vh start read the real config by key names before and after, and on 2.1.328 the /mcp disable needle read absent in both the user config and the settings files.
@task_item n=4 part=touches
The row that leaves unmeasured whether the /mcp disable probe writes to the user's real Claude config: one version's reading (2.1.328, absent before and after) now exists, never a general measurement; why no new projects entry appeared stays unmeasured.
@task_item n=5 part=touches
The rows that verify never answers dialogs and never types into one: verify now tells the human so in a fixed line when the browser-tools prompt stood at start.
@task_item n=6 part=touches
The rows that verify reads the screen only by signatures against compiled literals: the new line rests on the existing browser-tools reading of the ledger, no new screen read.
@task_item n=7 part=touches
The rules on human output: fixed messages that interpolate nothing and escape controls; the new verify line is a fixed plain ASCII literal with no SGR.
@task_item n=8 part=touches
The row that only viola verify writes the stamps file, under the strict-modes check and the lock: the live home's 2.1.328 entry was replaced by verify with a 24-row stamp, other entries kept.
@task_item n=9 part=touches
The rows on the pinned viola copy under the viola home (placed once, never replaced, never deleted, one tree may key two folders): the bin folder now holds four keys after the live start, the older ones kept.
@task_item n=10 part=touches
The pinned product binary: a release build was placed beside the product with a rollback copy and the product file was not replaced; the switch is an operator step outside viola.
@task_item n=11 part=touches
The rows that the release build refuses test-only binaries and features: the release check of the re-pin build read viola only, the live rig binary staying test-only.
@task_item n=12 part=touches
The rows that every product reader is bounded and reads are capped at MAX_FRAME: the rig's new config, settings and transcript readers read through MAX_FRAME and keep no value.
@task_item n=13 part=touches
The live rig's closed-word and keep-nothing rows (prompt capture in closed words, cache key name read only, MCP arm keeps no output): the vh and cx arms add summary words that are names, counts or closed words only.
@task_item n=14 part=touches
The rows on the live rig's credentials copy and the CLAUDE credential strip from the child: vh reads no credentials and uses the stripped child environment less the config-dir override; its summary read credentials none and keys none.
@task_item n=15 part=touches
The row that the live-round rig keys only into its own pty: cx sends /context through viola send to its own instance under the rig's private config.
@task_item n=16 part=touches
The rows that the real claude CLI runs only locally and CI holds no secrets: the live starts ran on the local host; CI on the pushed head read green 7 of 7 with no real claude.
@task_item n=17 part=touches
The rows that test-side removal touches only its own dirs: a rig hand directory under the temp root was left standing after a recursive removal was refused, and stays on the operator's desk list.
@task_item n=18 part=touches
The rows on verify writing the trust record flag only and skipping trusted folders: under the real home the trust-record row read pass and no new projects entry appeared.
@task_fact n=1 slug=secret-management
fact: The test-only live rig's vh arm is the one named exception to child isolation and own-home tests: it runs under the real home and Claude config, reads no credentials, keeps key names and one closed needle word only, prints no value, and is refused without an explicit target home, Claude dir and user config.
@task_fact n=2 slug=data-protection
fact: The live rig's cx arm reads the session transcript through MAX_FRAME, counts tags past the kept length and keeps no byte of it; its send goes through viola send under the rig's private config.
@task_fact n=3 slug=data-protection
fact: On claude 2.1.328 the /mcp disable probe wrote nothing naming its server into the user's Claude config or settings files (absent before and after); one version's reading, and why no new projects entry appeared stays unmeasured.
@task_fact n=4 slug=threat-model-summary
fact: When the browser-tools prompt stood at start, viola verify prints a fixed line before its summary saying viola never answers it and the human answers it once in claude; the prompt-absent output is unchanged.
@task_item n=1 part=untouched
The IPC channel, its socket directory, peer euid checks, server verification and the channel methods, errors and refusals: no method, frame, endpoint or socket was added or moved.
@task_item n=2 part=untouched
The MCP server, its tools, inputs, stdin cap and isError results.
@task_item n=3 part=untouched
The hook contract, its payload parsing, fail-open behaviour and hook-decision details.
@task_item n=4 part=untouched
Dependency security: no dependency, toolchain pin, advisory ignore, licence, feature set or workflow changed.
@task_item n=5 part=untouched
Paste and send text validation, control-character and empty-text refusals, and the send and answer refusal words.
@task_item n=6 part=untouched
config.json keys, CLI exit codes, the state-root selection and revive's arguments and codes.
@task_item n=7 part=untouched
The CI artifact canary scan and its classes, and the pre-push gate's hermetic environment and tool pins.
