@task_prose part=session
The session made /context a listed local command whose delivery is confirmed by its local-command stdout tag in the transcript (post-condition CommandStdout), the fourth post-conditioned local command beside /clear, /compact and /mcp.
It added a 25th relied capability-ledger row, local-command-context: /context writes its stdout tag and submits no prompt, its latency within the compiled confirmation window; a stamp holding only the 24 earlier rows now reads unverified on every binary built after it.
viola verify's Run B now pastes /context as its eighth compiled literal, the probe for that row, and the stamps file's version entry gains the row and a measured context latency; verify's step lines and all-pass summary count 25.
The fake agent's framing echo also echoes the /context probe literal; the test-only live rig gained a CS leg (a scratch verify that keeps its stamp) and a CC leg (a wrapped send of /context under that kept stamp), plus hint-cell attribute readings.
Measured live on claude 2.1.328 alone, in the rig's private scratch config: the row passed 25/25 and a wrapped /context send was confirmed with one tag and no prompt submitted; the live home stays a 24-row stamp, its re-stamp left to the product re-pin step.
The wrap also owes the three architecture excepts on the test-only rig seeding config their word private back, within the cell bound, with no member dropped or merged.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the per-behaviour CLI capability ledger: the relied row set, the compiled list of local commands with their delivery post-conditions, the probe literals, each row's check and the stamp merge of measured values
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited — unit cases of the capability ledger
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited — unit cases of the ledger rows' checks
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: edited — unit cases pinning verify's framing probe literals and the local-command list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: edited — unit cases of the typed probe runs' checks, now the local-command-context check
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — unit cases of the ledger stamps: a version verified only when every relied row passes
@task_file n=7 path=src/cmd/verify.rs
what: edited — viola verify, the one stamps writer: its step lines and summary over the relied rows
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: edited — verify's typed probe runs, threading the measured context latency into the stamp
@task_file n=9 path=src/cmd/verify/typed/turns.rs
what: edited — verify's Run B added turns: the compiled pastes into its own session, now /context the eighth
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent test binary: its framing echo of verify's probe commands
@task_file n=11 path=src/run/send/classify.rs
what: edited — send's classification and after-the-fact confirmation of a local command by the ledger (unit cases added only)
@task_file n=12 path=src/run/version_gate.rs
what: edited — the version gate: a CLI version read verified or unverified from the stamps
@task_file n=13 path=tests/cli_send.rs
what: edited — integration cases of the send verb, now a /context send confirmed and unconfirmable before its transcript exists
@task_file n=14 path=tests/cli_verify.rs
what: edited — integration cases of viola verify over the fake agent
@task_file n=15 path=tests/cli_verify/runs.rs
what: edited — integration cases of verify's probe runs
@task_file n=16 path=tests/cli_verify/stamps.rs
what: edited — integration cases of the stamps verify writes
@task_file n=17 path=tests/contract_ledger_probes.rs
what: edited — contract check that every stamped recorded fixture set stamps every relied row over the fake agent
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: edited — the end-to-end harness: its pinned list of relied ledger row ids
@task_file n=19 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the test-only live rig against the real claude: its legs and labels, kept-stamp and stamp-seed, private config reading and stamp lines
@task_file n=20 path=crates/viola-e2e/src/rig/screen.rs
what: edited — the test-only live rig's screen model reading, now hint-cell attribute words
@task_file n=21 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the viola-live-rig test binary's commands, now rehearse cs and rehearse cc
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsals against the fake agent
@task_item n=1 part=touches
The rule that local commands are listed with delivery post-conditions and classified by the ledger: the list gains /context with the stdout-tag post-condition, four post-conditioned commands now.
@task_item n=2 part=touches
The rule that a local command is confirmed by its stdout echo or tag and its echo ledger row and probe: /context is a second command confirmed by its stdout tag, with its own relied row and probe beside the echo one.
@task_item n=3 part=touches
The per-behaviour capability ledger and its relied set: a 25th relied row, local-command-context; any row naming the relied rows or their count is reached.
@task_item n=4 part=touches
A CLI version is verified only when every relied row counts, and malformed or partial stamps read unverified: a 24-row stamp, the live home's among them, now reads unverified until re-stamped with 25 rows.
@task_item n=5 part=touches
The ledger stamps envelope contract (rows and measured): the version entry gains row local-command-context and measured.typed_probe.context_latency_ms, additive; the stamps merge still replaces its own version's entry whole.
@task_item n=6 part=touches
Verify's bounded child set and Run B's compiled pastes: Run B pastes /context as its eighth compiled literal, a widening of the ratified child set on the founder's word; verify's claude process count stays 9.
@task_item n=7 part=touches
The viola verify stdout step line and summary line contracts: the step count reads n/25 and the all-pass summary 25 pass.
@task_item n=8 part=touches
Verify's CLI transcripts as an accepted residual: the /context paste adds to what verify's scratch config holds; measured only as the whole config's top-level names and counts, the paste's own share unseparated.
@task_item n=9 part=touches
The send unconfirmable result and ok payload: a /context send before its transcript exists, or on an unverified CLI, reads ok with confirmed false and detail unconfirmable.
@task_item n=10 part=touches
Every send confirmed after the fact and unconfirmed delivery reported as such: indirect, through the confirmation of /context by a new stdout tag and no prompt submitted.
@task_item n=11 part=touches
Transport-only degrade and the partial gate on an unverified CLI: indirect, through the version gate now requiring 25 rows.
@task_item n=12 part=touches
The fake agent's test seams: its framing echo takes the /context probe literal too; no option, environment variable or seam added.
@task_item n=13 part=touches
The excepts of the three rows on Claude-first with no adapter trait, Claude-specific shapes isolated and Claude payload shapes in one crate, each opening Test-only rig seeds config: owed their word private back within the cell bound, no member dropped or merged.
@task_item n=14 part=touches
viola-e2e is test-only and the viola-live-rig binary name: the rig gains the rehearse cs and cc words and the CS, CS-2, CC and CC-2 labels, still test-only.
@task_item n=15 part=touches
verify is the one stamps writer and the ledger stamps have one writer: the test-only rig copies the stamp a scratch verify wrote and seeds it into a later scratch home's ledger, never the real home's.
@task_item n=16 part=touches
Real CLI probes run only locally and no live claude in CI: the CS and CC legs ran locally against claude 2.1.328 under the rig's private config; CI still runs against the fake agent.
@task_item n=17 part=touches
Screen content read only by verify and run never reading screen content: indirect, the test-only rig now reads attribute words of drawn hint cells past the cursor, never their text.
@task_item n=18 part=touches
The confirmation window compiled: the new row's check bounds the /context latency by the compiled confirmation window fallback.
@task_fact n=1 slug=established-decisions
fact: /context is a listed local command whose delivery is confirmed by a new local-command stdout tag in the transcript and no prompt submitted; before its transcript exists, or on an unverified CLI, its send reads unconfirmable.
@task_fact n=2 slug=established-decisions
fact: The relied ledger row local-command-context holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window; verify probes it by pasting /context as Run B's eighth compiled literal, and a failing read of it is fail, never absent.
@task_fact n=3 slug=standard-contracts
fact: The ledger stamps envelope's measured block carries typed_probe.context_latency_ms, the /context probe's measured latency, written by the stamps merge with the version's entry.
@task_item n=1 part=untouched
The MCP server, its tools, results and stdin handling: no MCP tool or result changed.
@task_item n=2 part=untouched
The wrapper channel protocol, its methods, frames, fault codes and endpoints: no IPC method, socket, port or protocol version added.
@task_item n=3 part=untouched
Hooks, the hook-to-kind map, hook tiers, exit bodies and dialog holds: no hook or event kind changed.
@task_item n=4 part=untouched
The wheel, links, pause and release rules: untouched.
@task_item n=5 part=untouched
revive, list and the board, their JSON documents and exit codes: untouched.
@task_item n=6 part=untouched
Build system, dependency policy, toolchain, CI workflow steps and coverage floors: no dependency, CI step, gate or floor moved.
@task_item n=7 part=untouched
Configuration keys, environment variable names and the VIOLA_ and AGENT_RUN_ namespaces: no config key or environment variable added.
@task_item n=8 part=untouched
The Hyprland seam, trust writer code words, the pinned copy and plugin install: untouched.
@task_item n=9 part=untouched
Name rows for binaries, crates, commands and MCP tools: no binary, crate, command or tool name taken or freed.
