@task_prose part=chunk
At the one start shared by viola run and viola revive, after the version gate and before the spawn, on a verified CLI, viola writes the CLI's per-project trust record (one key, hasTrustDialogAccepted) for the child's folder when no record already trusts it read the CLI's way.
The write goes into the one Claude config file the started child reads, under the CLI's lock when free, by atomic replace, read back, at most 3 attempts; any refused file form or a lost record writes nothing more and leaves the trust dialog to the human, viola typing no key.
One closed word per start reaches the run log, nothing of the file anywhere; every refusal and skip gets a case proving nothing was written, built on the fake agent that reads the record.
The live proof is S1 (viola run in a fresh untrusted folder under the rig's private config) with its control S2 (a plain claude in the same folder shape), asked as one block at the end of P3.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude agent's trust record shape and the CLI's key rule (git root inside a repository, else the cwd), reused by the product write and its CLI-way trust read with the parent walk
@task_file n=2 path=src/trust_record.rs
what: the trust-record file writer — lock, atomic replace keeping the file's mode, the refusals that write nothing, its closed code words — reused for the product write, with read-back and retry to close at P3
@task_file n=3 path=src/bin/viola-fake-agent
what: the fake agent that decides its trust dialog from the Claude config file, so an e2e case of the write can fail when viola's write is gone
@task_item n=1 part=needs
The viola run TUI passthrough entry: the trust write sits inside the run and revive start, before the spawn, and must leave the passthrough's entry and its first screen to the child, with no viola bytes reaching the terminal.
@task_item n=2 part=needs
Zero viola bytes on the terminal and the tui terminal belonging to the child: the write's outcome word goes only to the run log, never to the human's terminal, and nothing of the config file is printed anywhere.
@task_item n=3 part=needs
Run diagnostics never reach the terminal: the one closed word per start is a run diagnostic carried by the run log alone.
@task_item n=4 part=needs
Human keystrokes never blocked and the human taking the wheel: when the write refuses or the record is lost, the trust dialog stays the human's and viola types no key into it.
@task_item n=5 part=needs
The closed word the write records per start follows the code word format, a closed list of kebab-case words with no file content or path interpolated.
@task_item n=6 part=needs
Paths not carried: neither the config file's path, the folder key nor any file content appears in the closed word or any output.
@task_item n=7 part=needs
Time bounds fall on the driver only: the lock wait and the at-most-3 attempts bound viola's own write, never a limit put on the human, and a lost race degrades to the human's dialog.
@task_item n=8 part=needs
The write's refusals are silent degradations, not CLI refusals: a file absent, unreadable, a link, not the JSON expected or oversize, a lost race, an unverified CLI, a folder already trusted, or the home folder each leave the start to proceed with no new refusal line or exit code.
@task_item n=9 part=needs
Start refusals at run and revive (home strict-modes failure, socket directory failure) keep their order and form; the trust write comes after the version gate and does not run on a start already refused.
@task_item n=10 part=needs
Boundary tests sequenced on records and the boundary check never inspecting claude content: the fake agent's first screen reading trust or not is read as a signature, not by parsing screen content.
@task_item n=11 part=needs
Live readings never gate: the S1 and S2 live starts are evidence under the founder's cap and gate no CI job.
@task_item n=12 part=needs
A test that waits takes a bound under the 20 s lint, and each control's timeout is named and fired; ids within 16 characters.
@task_standing n=1
convention: each new test runs inside the existing test suites, with no separate a11y driver or suite
home: 1-a11y-scope-summary
@task_standing n=2
convention: the CI test gate allows no skips; a new test is never marked ignored or skipped
home: 1-a11y-scope-summary
@task_standing n=3
convention: zero flakiness and no retries for each new test; a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: a11y failures are nextest failures, running in the existing test suites
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=5
convention: a new run-log line carries the log identity tags
home: a11y-scope-summary
@task_item n=1 part=not_needed
The viola list board, its caption, colour, stale row, truncation and unwrapped separator: the chunk changes no board output.
@task_item n=2 part=not_needed
Visual design rows (SGR, amber, bold, dim, colour decision inputs): the write prints nothing to any terminal.
@task_item n=3 part=not_needed
viola --help rows, group labels and served verbs: no verb or flag is added.
@task_item n=4 part=not_needed
viola link and unlink, plugin install, revive --list and wait output rows: those surfaces are untouched.
@task_item n=5 part=not_needed
Hook event and dialog calls, MCP server frame tools and frame verb refusals: the write happens at the start, before any frame or hook.
@task_item n=6 part=not_needed
CLI --json failure documents, revive --json refusal documents and instance-unreachable details: the write adds no refusal code or document.
@task_item n=7 part=not_needed
Manual screen-reader and keyboard passes (Orca targets and setups): the chunk's live proof is S1 and S2 on the rig, not a manual a11y pass.
@task_item n=8 part=not_needed
Absences of a web or graphical frontend, touch surface, rendered page and internationalization: no such surface is reached.
@task_item n=9 part=not_needed
Send refusals and the send-readback path: the chunk types nothing and sends nothing into the child.
