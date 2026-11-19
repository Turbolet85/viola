@task_prose part=session
The session stamped the founder's real home live on claude 2.1.328 at all 24 relied rows, then rebuilt the product and placed the re-pinned binary beside it; the switch to it was not made.
viola verify now prints one fixed line before the summary when the browser-tools prompt stood at start: the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again. The summary stays the last line and the output when the prompt is absent is unchanged.
The agent crate's ledger gains a public reader that says whether a typed run met the browser-tools prompt; a shared test helper building the browser-tools screen moved into the root tests' support module.
The test-only live rig gains two arms: vh (labels L1 and L1-2) runs verify under a given real home and Claude config, reads names and one needle only and keeps no value; cx (label L2) sends /context once through viola send under a private config and reads its post-condition. Rehearse gains --target-home and --claude-dir; vh without all three home flags is refused as usage, exit 2.
Measured live on 2.1.328: L1 read verify 24/24 with the browser-tools prompt absent and the other stamps kept; /mcp disable wrote nothing naming its server into the real config; L2's send came back not-delivered no-prompt-submitted with no transcript, so the /context post-condition is a reading of one half, never a verification.
CI on the pushed head was green 7/7; the intermittent red of the rig rehearsal that reads the box empty after x and del stays open with its cause unread; no retry rule and no loosened entry were added.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the agent ledger of relied rows and the version gate's reading of a typed verify run, including the browser-tools prompt row; edited to add a public reader of whether the run met that prompt
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's inline unit cases over stamps and typed runs; edited to add a case that the browser-tools reader is true only when a run met the prompt
@task_file n=3 path=src/cmd/verify.rs
what: the viola verify command, its step lines, its stamp write and its summary line; edited to print the browser-tools line after the stamp write and before the summary
@task_file n=4 path=tests/cli_verify.rs
what: the root integration binary for the viola verify CLI surface, its human output pinned by literal asserts; edited to add a case that a met browser-tools prompt is said before the summary
@task_file n=5 path=tests/support/fake.rs
what: the root tests' shared support for the fake agent, its recorded screens and helpers; edited to hold the browser-tools screen helper used by the verify and send binaries
@task_file n=6 path=tests/cli_send.rs
what: the root integration binary for the viola send CLI surface; edited to take the browser-tools screen helper from the shared support instead of its own copy
@task_file n=7 path=crates/viola-e2e/src/rig/mod.rs
what: the standing committed live rig in the test-only E2E crate, its arms, labels, plan, summary words and teardown; edited to add the vh real-home arm and the cx /context arm with their readers
@task_file n=8 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of each start, the child environment and its strip, and its bounded readers of config files; edited to add a real-home child environment and name-only readers of keys, entries, needles and other stamps
@task_file n=9 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command line, its live and rehearse subcommands and their flags; edited to add the vh and cx arms and the --target-home and --claude-dir rehearse flags with a usage refusal
@task_file n=10 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal integration binary that runs the live rig against the fake agent; edited to add rehearsals of vh over a planted home, its refusal without an explicit home, and of cx's single /context send
@task_item n=1 part=touches
The viola verify output contract: the summary line stays the last line but is no longer always preceded only by step lines; a met browser-tools prompt adds one fixed line before it.
@task_item n=2 part=touches
CLI human output pinned by literal asserts: a new fixed verify literal is pinned, plain ASCII with no SGR, and the prompt-absent all-pass stdout stays pinned.
@task_item n=3 part=touches
The browser-tools prompt as a relied ledger row read as a compiled screen signature: a new public reader of that row's reading feeds verify's output, with the row's check unchanged.
@task_item n=4 part=touches
The one standing committed live rig: it gains the arms vh and cx, the labels L1, L1-2 and L2, and two rehearse flags.
@task_item n=5 part=touches
Tests never read the host's claude sessions, every test creates its own home, every test verify start is tied to a test-owned home: the live rig's vh arm is a local-only exception that runs verify under the real home and config, reading names and one needle only and keeping no value.
@task_item n=6 part=touches
The claude child environment strip and test-started viola dropping inherited env: the vh child environment is the rig's usual child env without CLAUDE_CONFIG_DIR.
@task_item n=7 part=touches
Real claude CLI tested local-only, never in CI: two live starts ran locally on claude 2.1.328 by path; CI stayed on the fake agent.
@task_item n=8 part=touches
A live reading is recorded as read, never as verification: the /context post-condition and the /mcp disable reading on 2.1.328 are kept as readings of one version, the /context half not met as worded.
@task_item n=9 part=touches
Version gate needs every relied row and the unverified CLI gate: the live home's 2.1.328 stamp now holds all 24 relied rows passing, while the 2.1.325 entry stays at 21 rows and reads unverified to a binary of this tree.
@task_item n=10 part=touches
Stamps written only by verify and single writers of stamps: the 2.1.328 stamp was replaced by verify through the rig, other entries kept as the rig's stamp-others word reads.
@task_item n=11 part=touches
Every external reader is bounded and bounded readers keep the frame cap: the rig's new config, settings and transcript readers read through MAX_FRAME.
@task_item n=12 part=touches
One shared copy of each test helper and shared helpers never copied: the browser-tools screen builder moved from one root integration binary into the shared support.
@task_item n=13 part=touches
No test retries or sleep synchronisation and the zero-flakiness budget: the intermittent rig rehearsal red stays open with no retry rule, and cx polls for its tag within the send's submit bound rather than sleeping.
@task_item n=14 part=touches
Local commands confirmed by post-condition only and viola send's unconfirmed send: the live /context send read not-delivered no-prompt-submitted on 2.1.328.
@task_item n=15 part=touches
Fixtures recorded per CLI version and every CLI-version fixture set replayed: no 2.1.328 recorded fixture set was made; one is proposed as a carry, so 2.1.328 has a live stamp without a committed fixture set.
@task_item n=16 part=touches
Release build carries viola only: the re-pin's release build read release-check viola only before it was placed beside the product.
@task_item n=17 part=touches
Unbuilt harness surface is a usage error, in the rig's own command line: vh without --target-home, --claude-dir and --user-config is refused usage with exit 2.
@task_item n=18 part=touches
Logs carry no secrets and fixtures free of real data: vh reads no credentials, prints project keys never, and its summary holds names and closed words only.
@task_item n=19 part=touches
Test function naming and the test-only E2E crate: new rig and verify cases follow the naming rule, and every rig change stays inside the one test-only workspace member.
@task_fact n=1 slug=5-integration-test-strategy
fact: viola verify prints the fixed line «the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again» before the summary when the browser-tools prompt stood at start; the summary stays last and the prompt-absent stdout is unchanged.
@task_fact n=2 slug=5-integration-test-strategy
fact: The live rig's labels L1 and L1-2 take the arm vh and the label L2 takes the arm cx; cx sends /context once through viola send under the rig's private config and reads context-post as one of both, prompt-submitted, tag or none.
@task_fact n=3 slug=test-strategy
fact: Exception to tests never reading the host's claude: the live rig's vh arm, local only, runs verify under a given real home and Claude config, reads key and entry names and one needle as a closed word only, reads no credentials and keeps no value; without all three explicit home flags it is refused as usage.
@task_item n=1 part=untouched
CI workflows, jobs, artifacts, caches and supply-chain gates: no CI step, action pin or dependency changed.
@task_item n=2 part=untouched
Coverage floors, mutation gates and performance budgets: no threshold, kill line or budget moved.
@task_item n=3 part=untouched
The wrapper channel, IPC methods, socket and endpoint checks: no method, endpoint, event or env var added.
@task_item n=4 part=untouched
MCP tools, their schemas and replies: the MCP surface is untouched.
@task_item n=5 part=untouched
Hook exec-form entry points, dialog hook decision bodies and their deadlines: no hook path changed.
@task_item n=6 part=untouched
The harness contract, agent-run commands, session records, status shape and pre-push gate: no harness surface changed.
@task_item n=7 part=untouched
Log format, process log lines and event kinds: verify's new line adds no log line, span or event.
@task_item n=8 part=untouched
viola answer, wait, last, list, release and revive contracts and exit codes: none moved.
