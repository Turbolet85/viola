@row_add n=1 row=decision/no-stored-secrets area=secret-management rec=1
name: No stored secrets
rule: The product stores no secrets.
@row_add n=2 row=decision/env-not-config-channel area=secret-management rec=1
name: Environment variables are not configuration
rule: Environment variables are never a configuration channel for the product.
@row_add n=3 row=decision/claude-credentials-stripped-from-child area=secret-management rec=1
name: Inherited CLAUDE credentials stripped from the child
rule: The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
@row_add n=4 row=decision/credential-strip-by-name-only area=secret-management rec=1
name: Credential strip reads names only
rule: The environment strip handles variable names only and never reads a variable's value.
@row_add n=5 row=decision/hermetic-child-environment area=secret-management rec=2
name: Hermetic child environment in the pre-push gate
rule: The local pre-push gate launches every child under env -i with exactly HOME and PATH, so no host environment value crosses into a child.
except: Any assignment whose value comes from the host is a boundary widening that needs its own Decisions Log entry.
@row_add n=6 row=decision/tests-use-own-home area=secret-management rec=2
name: Tests use their own home
rule: Tests run against their own --home, never the user's.
@row_add n=7 row=decision/no-uid-0-launch area=secret-management rec=2
name: No root launch
rule: No child is ever launched as uid 0.
@row_add n=8 row=decision/no-credential-literals area=secret-management rec=3
name: No credential literals in code
rule: No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
@row_add n=9 row=decision/no-repo-secret-scanner area=secret-management rec=4
name: No repository secret scanner
rule: No repository secret scanner runs; revisit when the release workflow adds signing credentials.
why: The repo and CI hold no secrets, and the security research catalogues no secret scanner.
@row_add n=10 row=decision/artifact-canary-scan area=secret-management rec=5,10
name: CI artifact canary scan
rule: CI always runs the artifact canary secret scan over every file under the agent-run artifact directory, and the test-home uploads run only when it succeeds.
except: The mutants chunk diff is skipped by exact path and excluded from the harness upload, so it is never uploaded.
@row_add n=11 row=contract/artifact-canary-scan-classes/claude-stripped area=secret-management rec=6,5
boundary: artifact canary scan classes
item: claude-stripped
promise: The scan fails on any of the twelve stripped CLAUDE* canary needles.
@row_add n=12 row=contract/artifact-canary-scan-classes/cookie-header area=secret-management rec=7,5
boundary: artifact canary scan classes
item: cookie header
promise: The scan fails on a cookie: header, matched case-insensitively.
@row_add n=13 row=contract/artifact-canary-scan-classes/content-canary area=secret-management rec=8,5
boundary: artifact canary scan classes
item: content canary
promise: The content canary fails the scan only in home-level role files.
@row_add n=14 row=contract/artifact-canary-scan-classes/diagnostics-mode area=secret-management rec=9,5
boundary: artifact canary scan classes
item: diagnostics file mode
promise: The scan fails on a diagnostics file whose mode is not 0600.
@row_add n=15 row=decision/canary-scan-never-echoes-matches area=secret-management rec=10
name: Canary scan never echoes matches
rule: The artifact canary scan never prints or writes the matched bytes.
@row_none n=1 rec=11 type=S
@row_none n=2 rec=12 type=0
what: a not-applicable placeholder line for API key and database password rotation
@row_none n=3 rec=13 type=0
what: a not-applicable placeholder line for JWT signing key rotation
@row_none n=4 rec=14 type=0
what: a not-applicable placeholder line for TLS certificate rotation
@row_add n=16 row=absence/secret-manager area=secret-management rec=15
capability: A secret manager, and so any secret access auditing
@row_none n=5 rec=16 type=S
@row_add n=17 row=decision/messaging-credentials-are-secret area=secret-management rec=17,16
name: Messaging token and socket are secrets
rule: The inherited CLAUDE_CODE_MESSAGING_TOKEN and CLAUDE_CODE_MESSAGING_SOCKET count as secrets.
@row_add n=18 row=decision/zipsign-key-ci-secret-only area=secret-management rec=18,16
name: Release signing key held only in CI
rule: The zipsign private key for self_update signatures counts as a secret and is held only as a CI secret in the release workflow.
@row_none n=6 rec=19 type=S
@row_none n=7 rec=20 type=S
@row_none n=8 rec=21 type=S
@row_add n=19 row=decision/sanitised-external-errors area=error-handling rec=22,23
name: Sanitised external error responses
rule: Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
@row_add n=20 row=decision/fixed-error-display-messages area=error-handling rec=24,22
name: Fixed error display messages
rule: Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
@row_add n=21 row=contract/channel-errors/-32603 area=error-handling rec=25
boundary: channel JSON-RPC errors
item: -32603
promise: An internal error carries the fixed message "internal error" with data null.
@row_add n=22 row=contract/mcp-iserror/instance-unreachable area=error-handling rec=26
boundary: MCP isError structuredContent
item: error instance-unreachable
promise: An unreachable instance reads error instance-unreachable with detail one of its closed causes.
@row_add n=23 row=contract/mcp-iserror/wrapper-fault area=error-handling rec=26
boundary: MCP isError structuredContent
item: error wrapper-fault
promise: A wrapper fault reads error wrapper-fault with a sanitised code, message and data, never the peer's own.
@row_add n=24 row=contract/mcp-iserror/invalid-params area=error-handling rec=26
boundary: MCP isError structuredContent
item: error invalid-params
promise: Invalid parameters read error invalid-params with a closed field word.
@row_add n=25 row=contract/instance-unreachable-cause/strict-modes-failed area=error-handling rec=26
boundary: instance-unreachable detail causes
item: strict-modes-failed
promise: A closed cause of instance-unreachable.
@row_add n=26 row=contract/instance-unreachable-cause/instance-dead area=error-handling rec=26
boundary: instance-unreachable detail causes
item: instance-dead
promise: A closed cause of instance-unreachable.
@row_add n=27 row=contract/instance-unreachable-cause/unwrapped-name area=error-handling rec=26
boundary: instance-unreachable detail causes
item: unwrapped-name
promise: A closed cause of instance-unreachable.
@row_add n=28 row=contract/instance-unreachable-cause/endpoint-lost area=error-handling rec=26
boundary: instance-unreachable detail causes
item: endpoint-lost
promise: A closed cause of instance-unreachable.
@row_add n=29 row=contract/instance-unreachable-cause/server-verify-failed area=error-handling rec=26
boundary: instance-unreachable detail causes
item: server-verify-failed
promise: A closed cause of instance-unreachable.
@row_add n=30 row=contract/instance-unreachable-cause/socket-dir-refused area=error-handling rec=26
boundary: instance-unreachable detail causes
item: socket-dir-refused
promise: A closed cause of instance-unreachable.
@row_add n=31 row=contract/mcp-iserror/refusal area=error-handling rec=26
boundary: MCP isError structuredContent
item: refusal
promise: A refusal that is no code word reads unknown, alike on MCP and CLI --json.
@row_add n=32 row=contract/mcp-iserror/detail area=error-handling rec=26
boundary: MCP isError structuredContent
item: detail
promise: A string detail that is no known detail reads null, alike on MCP and CLI --json.
@row_add n=33 row=contract/mcp-iserror/text area=error-handling rec=26
boundary: MCP isError text
item: refused text
promise: Every isError text reads refused: <code>.
@row_add n=34 row=contract/mcp-errors/-32603 area=error-handling rec=26
boundary: MCP JSON-RPC errors
item: -32603
promise: Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error -32603 internal error with data null.
@row_none n=9 rec=27 type=S
@row_add n=35 row=decision/full-error-detail-to-diagnostics area=error-handling rec=28
name: Full error detail only in diagnostics
rule: Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
@row_add n=36 row=decision/stderr-never-log-floor area=error-handling rec=28
name: stderr honours the never-log floor
rule: stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
@row_add n=37 row=decision/hook-silent-exit-zero area=error-handling rec=29
name: hook is silent and always exits 0
rule: The hook verb and its hidden capture arms never write to stderr and always exit 0; a failing security check ends in exit 0 with no body.
why: So the dialog still renders for the human.
@row_none n=10 rec=30 type=S
@row_add n=38 row=contract/channel-errors/-32600 area=error-handling rec=31
boundary: channel JSON-RPC errors
item: -32600
promise: A frame over MAX_FRAME gets the JSON-RPC error -32600.
@row_add n=39 row=contract/channel-errors/-32602 area=error-handling rec=31
boundary: channel JSON-RPC errors
item: -32602
promise: A release call carrying from gets the JSON-RPC error -32602.
@row_add n=40 row=contract/channel-refusal/control-character area=error-handling rec=31
boundary: channel result.refusal
item: not-delivered/control-character
promise: A text holding a refused control character is answered with result.refusal not-delivered/control-character.
@row_add n=41 row=contract/channel-refusal/empty-text area=error-handling rec=31
boundary: channel result.refusal
item: not-delivered/empty-text
promise: A text whose typed text is empty is answered with result.refusal not-delivered/empty-text.
@row_add n=42 row=absence/external-error-reporting area=error-handling rec=32
capability: An external error reporting integration
@row_add n=43 row=absence/outbound-network-calls area=error-handling rec=32
capability: Outbound network calls of any kind
@row_add n=44 row=decision/logging-bans-before-reporting area=error-handling rec=32
name: Logging bans apply to any external reporter
rule: If an external error reporter is ever added, the logging bans apply before transmission.
@row_none n=11 rec=33 type=S
@row_none n=12 rec=34 type=S
@row_none n=13 rec=35 type=S
@row_add n=45 row=decision/no-socket-in-tmp-root area=security-anti-patterns-never-do-these rec=36
name: No Unix socket at the temp root
rule: A Unix socket is never placed directly in /tmp or at the TMPDIR root; it always lives in a verified 0700 per-user directory.
why: The Kea DHCP CVE-2025-32801/32802/32803 precedent.
@row_add n=46 row=decision/socket-mode-not-sole-control area=security-anti-patterns-never-do-these rec=37
name: Socket file mode is not the only control
rule: The socket file's 0600 mode is never the only Unix access control.
why: unix(7) says socket permissions may be ignored.
@row_add n=47 row=decision/chmod-socket-after-bind area=security-anti-patterns-never-do-these rec=37
name: Socket mode set by chmod after bind
rule: The socket mode is set by chmod after the bind, never through the listener's mode option.
why: Set while macOS, where that option fails the bind, was a target, and kept.
@row_add n=48 row=decision/no-abstract-namespace-sockets area=security-anti-patterns-never-do-these rec=38
name: No abstract-namespace sockets
rule: Linux abstract-namespace sockets are never used.
why: They have no filesystem permissions.
@row_add n=49 row=decision/verify-server-before-frame area=security-anti-patterns-never-do-these rec=39
name: Verify the server before any frame
rule: No channel frame is written before the server is verified against a snapshot that passed the full strict-modes check (socket directory, pid, start time and euid) in the same process.
@row_add n=50 row=decision/channel-from-not-identity area=security-anti-patterns-never-do-these rec=40
name: Channel from is not an identity
rule: The channel from param is self-reported, never an authenticated identity, and serves link records and the release affordance guard only.
@row_add n=51 row=absence/user-accounts area=security-anti-patterns-never-do-these rec=41
capability: User accounts or login endpoints
@row_add n=52 row=decision/server-side-revalidation area=security-anti-patterns-never-do-these rec=42
name: Wrapper re-validates every input
rule: MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
@row_add n=53 row=decision/instance-path-from-validated-name area=security-anti-patterns-never-do-these rec=43
name: Instance paths only from validated names
rule: An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
@row_add n=54 row=decision/refuse-control-characters area=security-anti-patterns-never-do-these rec=44
name: Refuse control characters, never strip them
rule: A send text or answer free text holding any C0 control other than LF, CR and TAB, DEL or any C1 control is rejected with control-character and never typed into the PTY nor silently stripped.
except: send typing CR LF pairs and lone CRs as LF and dropping trailing CR and LF after validation is not stripping.
@row_add n=55 row=decision/never-refuse-line-breaks area=security-anti-patterns-never-do-these rec=44
name: Line breaks and tabs are allowed
rule: LF, CR and TAB are never refused, and a text with content is never refused for holding them.
why: Multi-line relays, plans and answers are normal.
@row_add n=56 row=decision/empty-text-refused area=security-anti-patterns-never-do-these rec=44
name: Empty typed text is refused
rule: A text whose typed text is empty is refused with empty-text, since there is nothing to type.
@row_add n=57 row=decision/control-check-on-decoded-chars area=security-anti-patterns-never-do-these rec=45
name: Control-character check on decoded characters
rule: The control-character check runs over decoded characters, never raw bytes.
why: Valid UTF-8 continuation bytes would otherwise be misread as C1 controls.
@row_add n=58 row=decision/reads-capped-at-max-frame area=security-anti-patterns-never-do-these rec=46
name: Reads capped at MAX_FRAME
rule: Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
@row_add n=59 row=decision/over-long-line-ends-session area=security-anti-patterns-never-do-these rec=46
name: Over-long line ends the session
rule: An over-long MCP stdin line ends the session; it is never truncated or skipped.
@row_add n=60 row=decision/no-serde-unbounded-depth area=security-anti-patterns-never-do-these rec=47
name: No unbounded JSON depth
rule: serde_json's unbounded_depth feature is never enabled.
guard: deny.toml ban
@row_add n=61 row=decision/closed-types-for-protocol-fields area=security-anti-patterns-never-do-these rec=48
name: Closed types for protocol fields
rule: behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
@row_add n=62 row=decision/agents-row-name-not-a-target area=security-anti-patterns-never-do-these rec=49
name: claude agents names are not targets
rule: A claude agents --json row name is never used as a ViolaName or target.
@row_add n=63 row=absence/database area=security-anti-patterns-never-do-these rec=50
capability: A database or query language
@row_add n=64 row=decision/owner-only-file-modes area=security-anti-patterns-never-do-these rec=51
name: Owner-only modes under the viola home
rule: The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask.
except: The pinned versioned viola binary is 0700, and never group- or world-writable.
@row_add n=65 row=decision/statusline-only-from-strict-home area=security-anti-patterns-never-do-these rec=52
name: statusline command only from a strict home
rule: statusline_command is never run from a snapshot whose home or instance directory fails the strict-modes check.
why: Write access there equals code execution.
@row_add n=66 row=decision/rehash-pinned-binary area=security-anti-patterns-never-do-these rec=53
name: Re-hash a pinned binary before reuse
rule: An existing pinned viola binary is never reused without re-hashing it against the hash in its directory name.
@row_add n=67 row=decision/cryptographic-pin-hash area=security-anti-patterns-never-do-these rec=53
name: Pin hash is cryptographic
rule: The pinned binary's hash is a truncated SHA-256, never a non-cryptographic hash.
@row_add n=68 row=decision/rewrite-plugin-files area=security-anti-patterns-never-do-these rec=53
name: Plugin files always rewritten
rule: Existing plugin files are never reused without rewriting them.
@row_add n=69 row=decision/non-inheritable-channel-handles area=security-anti-patterns-never-do-these rec=54
name: Channel handles not inheritable
rule: Channel handles are never made inheritable.
@row_add n=70 row=decision/fixture-hygiene area=security-anti-patterns-never-do-these rec=55
name: Recorded fixture hygiene
rule: Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
except: A workspace.repo naming this repository's own remote is kept.
