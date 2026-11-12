@row_add n=1 row=decision/hook-fails-open area=threat-model-summary rec=1,2,3
name: Hook fails open to the human
rule: Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr.
why: So the dialog renders for the human.

@row_add n=2 row=decision/readers-tolerate-torn-lines area=threat-model-summary rec=4,5
name: Readers tolerate torn lines
rule: Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.

@row_add n=3 row=decision/pinned-artefacts-never-deleted area=threat-model-summary rec=4,5
name: Pinned binary and plugin folders kept
rule: The pinned binary and the plugin folders are never deleted in v1.

@row_add n=4 row=decision/cargo-deny-gate area=threat-model-summary rec=6,7
name: Cargo deny gate on the root graph
rule: Cargo dependencies are held to cargo deny check over the root lockfile: advisories, licences, sources and C-crate, telemetry-crate and feature bans.
guard: cargo deny check
except: the test-only fuzz workspace sits outside it by a ratified exemption, its lockfile getting its own advisory and source audit

@row_add n=5 row=decision/actions-sha-pinned area=threat-model-summary rec=6,7
name: Third-party actions SHA-pinned
rule: Third-party GitHub Actions are SHA-pinned.

@row_add n=6 row=decision/no-abstract-namespace area=threat-model-summary rec=8,9,10
name: No Linux abstract namespace
rule: The Linux abstract namespace is never used for IPC sockets.

@row_add n=7 row=absence/outbound-network-calls area=threat-model-summary rec=8,11
capability: Outbound network calls made by viola itself

@row_add n=8 row=contract/send-refusal/empty-text area=input-validation rec=12,13,14
boundary: send refusal
item: empty-text
promise: A text whose typed text is empty, the empty text included, is refused not-delivered with detail empty-text (CLI exit 13), and nothing is typed.

@row_add n=9 row=decision/budget-file-single-writer area=input-validation rec=15,16
name: hook statusline is the only budget writer
rule: budget.json is written by hook statusline alone.

@row_add n=10 row=decision/mcp-stdin-line-cap area=input-validation rec=17,18
name: MCP stdin lines capped at the frame limit
rule: Each MCP stdin line is capped at MAX_FRAME, its newline counted; an over-long line is refused whole, never truncated or skipped, and ends the session as a stdin close does, its warning holding no byte of it.
why: rmcp reads stdin lines unbounded, so viola hands it a capped reader

@row_add n=11 row=decision/statusline-command-trust area=input-validation rec=19,20
name: Statusline command trusted via strict modes
rule: statusline_command is trusted only because of the home strict-modes check.
why: Write access to it equals code execution.

@row_add n=12 row=decision/single-shell-out area=input-validation rec=21,22
name: statusline_command is the only shell-out
rule: The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded; it is the only shell-out, and a second one is never added.

@row_add n=13 row=absence/file-upload area=input-validation rec=23,24
capability: A file upload surface

@row_add n=14 row=absence/webhook area=input-validation rec=23,24
capability: A webhook surface

@row_add n=15 row=absence/database area=input-validation rec=23,24,25,26
capability: A database or query language
