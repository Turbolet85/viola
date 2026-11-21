@task_prose part=chunk
Establishes the claude CLI's MCP client tool-call timeout for the versions viola stamps (2.1.328 and 2.1.330): its default, the names that move it, whether a stdio server differs, whether progress notifications reset it, and what the client does at the bound.
The reading is taken from the installed bundles read as strings only, with no live start and nothing read of the operator's real Claude home; a value so read is a reading, not a measurement.
Bounds in code the longest block of every tool viola mcp serves (send, wait, last, answer, list) below that timeout, send's whole path and wait's caller-supplied wait among them.
Where a tool's block can exceed the client's bound today, closes the gap by a compiled bound, a cap on a caller's argument or a documented split, each with a test that can fail.
Leaves to the P3 fork whether the bound stands as a 30th relied capability-ledger row with a viola verify probe, or as a compiled bound with a test and a drift reading.
@task_file n=1 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP server's tool handlers for send, wait, last, answer and list, the MCP tools contract — edits: each tool handler's deadline and wait's caller-supplied wait
@task_file n=2 path=crates/viola-mcp/src/lib.rs mark=edits
what: the stdio MCP server that serves the tool handlers and holds their deadlines
@task_file n=3 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results, the isError structuredContent with its closed refusal, detail and error codes
@task_item n=1 part=needs
The MCP tools contract for send, wait, last, answer and list: each tool's longest block is bounded in code and that bound sits below the client's tool-call timeout.
@task_item n=2 part=needs
send's whole path as one bounded block: the client-side checks, the channel frame, the wrapper's wheel read, the typing of the text as one bracketed paste, and the delivery confirmation with its waits.
@task_item n=3 part=needs
wait's caller-supplied wait, the timeout_ms MCP tool input: if the chunk caps it, the cap is a change to that input's contract and an over-cap value meets the invalid-params refusal the MCP tool inputs already carry.
@task_item n=4 part=needs
The shared CLI send and wait path the MCP tools reach through the channel's Tokio client and the wrapper's send handling: the channel frames' send and wait params and the wrapper channel methods send and wait, whose time bounds and who waits the chunk reads and may tighten.
@task_item n=5 part=needs
A tool that reaches its bound must answer with the closed MCP isError codes only: no upstream text, no path, pid or id, no parser error chain, a fixed message; any new code word joins the closed set and is said in the P3 block.
@task_item n=6 part=needs
The bound must not reorder the client's checks at the MCP seam: the strict-modes check first, the socket-directory check, the server verified before any frame, and a refused MCP frame tool sends nothing.
@task_item n=7 part=needs
The bound around send must keep the wheel rule whole: human typing takes the wheel and the human-typing send refusal stands; a bound never answers a dialog nor types into one.
@task_item n=8 part=needs
The client's timeout is read from the installed claude bundles as strings only: no live start of the real CLI, nothing read or written under the operator's Claude home, and the names that move the timeout recorded as names only, never their values.
@task_item n=9 part=needs
If the bound becomes a relied capability-ledger row with a viola verify probe, whatever verify newly reads, writes or spawns is a widening said whole in the P3 block: verify's child set, what is kept and where, and the proof that no text reaches a file, stamp, fixture or log, under verify's probe residual and probe cleanup rules.
@task_item n=10 part=needs
Any new environment read, seam or channel method the bound brings needs its log entry, and no config value, env var or flag may switch the bound off.
@task_item n=11 part=needs
A test that holds a block open to prove its bound uses a test-only seam under the fake-agent feature, such as the pump delay seam, compiled out of the release build.
@task_item n=12 part=needs
The bound's tests must be able to fail: a block that outlasts the client's timeout turns them red, with no retry rule and no loosened entry.
@task_item n=13 part=needs
The carried intermittent rehearsal red for the box read empty after x and del: at its next red met in this chunk, its printed rig summary and kept after-del frame are read and its cause named before the wait is mended, with no retry rule; not met, it is re-pinned whole.
@task_item n=14 part=needs
The sync crates stay free of tokio: a bound added on the shared send and wait path is placed so that no sync crate gains a tokio dependency.
@task_item n=15 part=needs
No documentation leaf gains a figure the code states, the bound's value included.
@task_standing n=1
convention: each new test runs under its own viola home, never the operator's, and its test home stays out of git
home: secret-management · bootstrap-phases
@task_standing n=2
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: each new log line honours the never-log floor: user content only in diagnostics, no CLAUDE values, no paths in error output
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=4
convention: each new recorded fixture is scrubbed or refused whole and reviewed before commit, names only where it records environment
home: data-protection
@task_item n=1 part=not_needed
Release, signing and deploy rows, hosting, remote access and network absences: the chunk ships no release and the pinned product with its re-pin stays outside it.
@task_item n=2 part=not_needed
Pinned binary placement, re-hash and no-clobber rows and plugin install rows: the pinned product is untouched.
@task_item n=3 part=not_needed
revive's arguments, its --json codes and its recorded cwd rules: revive is not touched.
@task_item n=4 part=not_needed
viola list's board scan, its home read rule and the terminal colour facts: the list verb is not touched.
@task_item n=5 part=not_needed
The hook's stdin, its decision details and its fail-open rules: the hook is not on the MCP tools' path.
@task_item n=6 part=not_needed
The link and unlink methods, their replies and their event log lines: neither is touched.
@task_item n=7 part=not_needed
pause and release, the driver-originated release guard and its errors: neither is a tool the chunk bounds.
@task_item n=8 part=not_needed
Event log growth and rotation, the trust record writer and the plugin files rewritten at start: no state file is changed.
@task_item n=9 part=not_needed
Third-party action pins, workflow permissions, checkout credentials and zizmor rows: no workflow is edited.
@task_item n=10 part=not_needed
Fuzz workspace, fuzz nightly pin, scheduled fuzzing and mutation testing at the epoch audit: no parser surface is added.
@task_item n=11 part=not_needed
The Hyprland recorder and the hyprctl deadline: the chunk reaches no window manager read.
@task_item n=12 part=not_needed
API-error classes in the Claude adapter and the transcript read: no turn end or transcript is read.
@task_item n=13 part=not_needed
The critical CVE response windows and self-update signatures: no dependency is added or updated and no updater exists.
