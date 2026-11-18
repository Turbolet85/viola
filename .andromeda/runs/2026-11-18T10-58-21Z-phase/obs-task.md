@task_prose part=chunk
Adds the product write of the CLI's per-project trust record at the one start `viola run` and `viola revive` share: after the version gate and before the spawn, on a verified CLI, when no record already trusts the child's folder read the CLI's way.
viola writes only `projects[<key>].hasTrustDialogAccepted` (the key the CLI itself writes) into the one Claude config file the started child reads, under the CLI's lock when free, by atomic replace, read back, at most 3 attempts.
A refused file form, a skip or a lost record writes nothing and leaves the dialog to the human, viola typing nothing; one closed word per start reaches the run log, nothing of the file anywhere.
It reuses the trust-record writer built for verify's Run F rather than adding a second writer, and re-reads the already measured no-flag finding and the relied trust-record row's live pass on 2.1.325, re-deriving neither.
Proof is a nothing-written case per refusal on the fake agent, plus the live start S1 with its control S2 under the founder's cap, asked in one block at the end of P3.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs mark=edits
what: the trust record's shape and the key rule (the git root inside a repository, else the cwd), the trust read the product write follows — edits: the trust read with the parent walk
@task_file n=2 path=src/trust_record.rs mark=edits
what: the trust-record file write: the CLI's lock, atomic replace keeping the file's mode, the file-form refusals and their closed code words — edits: the read-back, the retry up to 3 attempts, the refusals
@task_item n=1 part=needs
The wrapped start of `viola run` and `viola revive start` (revive's start logs as run): a new step between the version gate and the PTY spawn, taken only when the gate reads the CLI verified; an unreadable ledger or a stamp taken before the 23rd row reads unverified, and that start writes nothing.
@task_item n=2 part=needs
One closed word per start in the run process log saying what the trust write did (written, already trusted, skipped, refused by file form, lost race). It is a closed detail code carrying no path, no pid, no key, no folder and no file content, and it is added to the run log's line schema as an additive field or value.
@task_item n=3 part=needs
Nothing of the Claude config file reaches any diagnostics line, span field, detail file or stderr: no path of the file, no project key, no other project's entry, no folder name. Home-level lines stay path-free.
@task_item n=4 part=needs
The write reads one environment value, CLAUDE_CONFIG_DIR, and only when viola's home keeps that variable for the child (claude_env_keep), only to pick the file. Its value is never logged, and the CLAUDE environment strip and the env_kept names-only reporting of the claude-child process-start line are unchanged.
@task_item n=5 part=needs
Each refusal form writes nothing and leaves no temp sibling and no lock: the file absent (never created), unreadable, a link, not the JSON expected (not an object, `projects` not an object, the entry not an object), oversize past MAX_FRAME, a record still not read back after the 3rd attempt. Each becomes its own closed word, with no error chain or path printed.
@task_item n=6 part=needs
The skips that are no refusal: an unverified CLI, a folder the exact key or a parent already trusts (the walk stops at the git root inside a repository, at the filesystem root outside one), and the home folder. Each writes nothing and has its own case.
@task_item n=7 part=needs
The bounded retry: at most 3 attempts of lock, replace and read-back, then the dialog is the human's. A fixed bound inside one start, not a retry policy on a fault class, and no retry on a panic.
@task_item n=8 part=needs
The trust-record write was so far uninstrumented. This chunk gives the product write its one closed run-log word, adds no span of its own with path, key or env attributes, and leaves the start spans' field allow-list free of them.
@task_item n=9 part=needs
The e2e cases run against the fake agent, which reads its trust decision from the Claude config file, so a case fails when viola's write is gone. Real-CLI verify never runs in CI. A change to the fake's screens or trust reading puts cli_verify in the gate block.
@task_item n=10 part=needs
The live arms S1 (viola run in a fresh untrusted folder under the rig's private config, with one other project's entry as a byte-exact sentinel) and S2 (a plain claude in the same folder shape) are run on 2.1.325 only. The rig's own scratch config is written, and the founder's real config file is read only by the mark witness. Their evidence lines carry closed words, never file content.
@task_item n=11 part=needs
Every byte the rig types in S1 and S2 is named, the stop included: the standing Ctrl-C into the rig's own pty, never while a dialog stands, or else the no-key group kill. The session sets no wake or timer and ends every process it starts.
@task_item n=12 part=needs
Each control's timeout is named in the plan and fired with that flag, and a test that waits takes a bound under the 20 s lint, waiting on a log line, never on a sleep. A test-support helper spawning an instrumented binary under a cleared environment runs it in a scratch directory.
@task_item n=13 part=needs
The security rows saying viola writes nothing into the Claude config file are amended at this entry's wrap, and the plan lists them as Expected amendments. Writing the CLI's own state file is a named boundary widening with no network egress and no new dependency.
@task_standing n=1
convention: each new integration or e2e test's home lives under the one e2e test root the secret scan covers, and is kept until the gate steps run
home: ci-integration
@task_standing n=2
convention: each line a new test's process emits is schema-checked against the diagnostic line schema, and the panic and schema gates run on it even after a failed test
home: ci-integration
@task_standing n=3
convention: each new log word or field is additive to the line schema, from a closed enum, with frozen field names and no path or pid in a detail code
home: log-coverage
@task_standing n=4
convention: each new test that waits for a log line polls it under a bound, never a sleep-based wait
home: slo-invariants-telemetry-budgets
@task_standing n=5
convention: each new event or log line goes through the one obs_event macro, never a raw tracing macro, and any new span skips all arguments with an explicit field allow-list
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
The MCP stdio server's tools, frames and isError results: the chunk adds no tool and changes no MCP output. The size bound is borrowed from MAX_FRAME, and the MCP stdin contract itself is unchanged.
@task_item n=2 part=not_needed
The channel JSON-RPC lines, their params, conn and corr joins, and the send, wait, last and wheel paths with their spans and refusal words: the trust write runs before the spawn and touches no channel traffic.
@task_item n=3 part=not_needed
The dialog hook and its hook-invoked and hook-decision lines, fail-open codes and hook latency budgets: the chunk adds no hook behaviour, and the trust dialog it removes is not a hook-answered dialog.
@task_item n=4 part=not_needed
Heartbeat, liveness and list status rows, the agent-run status and logs-wrapper outputs, and state-recovered and snapshot replay: the chunk changes none of them.
@task_item n=5 part=not_needed
Link and unlink verbs, the events log's link records, and the release-from-driver refusal: unrelated to a start's trust write.
@task_item n=6 part=not_needed
The future opt-in error reporter rows (ureq and rustls, off by default, egress entry): no reporter is added.
@task_item n=7 part=not_needed
Metric derivation rows (gauges from harness snapshots, histograms as raw samples, quantiles report-only): the chunk adds no metric, only a closed log word.
@task_item n=8 part=not_needed
The secret-scan hit report's field contracts (file, line, byte offset, pattern class): the chunk changes no scan output shape.
