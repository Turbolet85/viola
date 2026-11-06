@task_prose part=chunk
Closes viola-0.1.0: brings every gate green on Linux (fmt, clippy, check, test, cargo deny check, deny probes, tokio-free checks, release check, CI's seven jobs), the supply-chain red cleared by moving portable-pty off =0.8.1 to a release whose graph drops pin-utils (RUSTSEC-2026-0441), with no ignore-list widening.
Removes or justifies the Windows remnants left in code (tests, viola-agent-claude, the e2e harness EXE_SUFFIX joins, coverage comments, plain_drive, the windows test-host key), the masters following.
Re-measures at HEAD and re-words the masters where they disagree: trycmd/assert_cmd absence, PTY_SIDELOAD_TEST_CASE registration, the one-enum-per-crate divergences, the thiserror derive census (10 vs 9), obs-plan's process-start row, and the channel-handle no-leak property on Linux (/proc/<child pid>/fd under the fake agent), then re-words the security plan's handle-inheritance line off recorded-not-established.
Runs up to two live sessions on the headless pty rig (v1-53's one live viola run on Linux, a driver driving a wrapped builder through viola), reads a dialog hold near the 3600 s ceiling, and records the founder's proved-or-deferred word on 17 unclaimed caps and the founder items (Measurement B, the wheel list, the socket-directory wording) in the verification matrix.
No new verb, flag, env var, channel method or frontend surface.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace.dependencies portable-pty pin (=0.8.1 moved)
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_file n=4 path=crates/viola-pty
@task_file n=5 path=tests/cli_output_plain.rs mark=edits
what: the Windows remnants at lines 104 and 116
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the Windows remnant at line 2
@task_file n=7 path=crates/viola-e2e/src/harness/session.rs mark=edits
what: the Windows remnant at line 52 and its EXE_SUFFIX joins
@task_file n=8 path=crates/viola-e2e/src/harness mark=edits
what: the EXE_SUFFIX joins
@task_file n=9 path=src/cmd/mod.rs mark=edits
what: plain_drive
@task_file n=10 path=tests/contract_os_gated_tests.rs mark=re-reads
what: the OS-gated test census (39 OS-gated test functions, 2 gated files) that v1-53's claim reads
@task_file n=11 path=.github/workflows/ci.yml mark=re-reads
what: CI's seven jobs read green at wrap, the supply-chain job included
@task_item n=1 part=needs
The supply-chain fix: portable-pty moves off =0.8.1 to a release whose dependency graph drops pin-utils (RUSTSEC-2026-0441 via nix 0.25.1), cargo deny check advisories and CI's supply-chain job reading green, with no new advisories ignore entry; the dependency-security advisories configuration, its one expected ignore (RUSTSEC-2017-0008, serial via portable-pty) re-checked because the portable-pty pin moves.
@task_item n=2 part=needs
Indirect, through the portable-pty bump: the pinning rule (every third-party root-graph version pinned exactly in workspace.dependencies, Cargo.lock committed) and the critical-CVE response path (bump, or swap along the named portable-pty-psmux 0.9.7 path) govern how the new version is chosen and recorded.
@task_item n=3 part=needs
Indirect, through the new portable-pty graph: the deny.toml licence allow list stays exact (any new non-listed licence crate needs its own per-crate exception and Decisions Log entry), sources stay crates.io only, the C-building crate bans and the sync-crate tokio bans (deny-sync, channel-tokio-free, deny probes) must still pass with the new transitive crates.
@task_item n=4 part=needs
Channel handle inheritance: the measurement that the wrapped child's /proc/<child pid>/fd holds no channel socket or listener on Linux under the fake agent, and the ban on inheritable channel handles; any portable-pty move must keep the close-on-exec property; the Data Protection handle-inheritance line is re-worded by this chunk off recorded-not-established.
@task_item n=5 part=needs
Indirect, through crates/viola-pty's spawn after the bump: viola run must still hand portable-pty an absolute child path it resolved itself (child executable resolution), and the vt100 feed must stay under catch_unwind with byte passthrough unblocked.
@task_item n=6 part=needs
Error Handling's product error enum list (PtyError hand-written; AgentError, ChannelError, StateError, PinError, StampsError, StampsReadError, Refused, McpError, ServeError via thiserror) with fixed-message Display impls: the one-enum-per-crate folds or ratifications and the thiserror derive census re-measure (10 vs 9) may change the enum set the list names; a PtyError change from the portable-pty bump keeps no upstream text or path in Display.
@task_item n=7 part=needs
The gates the chunk must bring green: CI's cargo deny job over all four families plus the fuzz/Cargo.lock advisories and sources step, the release check (scripts/release-check.sh refusing test-only features fake-agent and test-support, its probe 5/5), zizmor on the workflows, the G2 zero-panics gate with its one FAKE_AGENT_HOOK_PANIC exemption, and the artifact canary secret scan gating uploads.
@task_item n=8 part=needs
The live sessions (at most two, headless pty rig, no human key, no window): the real claude resolved from the calling PATH as an accepted same-user risk; a driver driving a wrapped builder over MCP or the CLI through the channel's existing peer and server-verification checks; every recorded reading kept free of CLAUDE* stripped values, user content and absolute paths (NEVER-log floor).
@task_item n=9 part=needs
The accepted residual for a scratch reading of a newer CLI: such a live start is no standing permission, each further reading needs its own answer; the dev-host prerequisite for a real-CLI run from a trusted folder.
@task_item n=10 part=needs
The dialog hold ceiling (3600 s, PROVISIONAL until a hold near it is read live): config values never make the dialog hold zero or unbounded; the hold is read, not reconfigured.
@task_item n=11 part=needs
The human-terminal vector: the stdin classifier's closed non-editing list (the wheel) is extended only by a reply measured live by its exact grammar on the founder's word, never on a guess (the CSI ? 6 n and kitty graphics hypothesis).
@task_item n=12 part=needs
Test-only env markers: whether PTY_SIDELOAD_TEST_CASE still exists at HEAD and takes an Occupied Resources row beside MCP_SERVE_TEST_CHILD; the rule that no new test seam or environment read is added without its own Decisions Log entry, and that test seams are compiled out of release and switch off no control.
@task_item n=13 part=needs
The mutation audit's survivor: an always-admit peer_admitted read at the three client seams (the client's own-euid peer admission in server verification) stays untested by construction; no seam may be added to reach it, matrix#v1-44 and test-plan section 1 holding the note.
@task_item n=14 part=needs
Measurement B (one tree keying two bin/<key>/ folders, unequal .note.gnu.build-id, .symtab, .strtab): the pinned binary keyed by its content hash, re-hashed (SHA-256) before reuse; the chunk records the founder item and acts on it only by his word.
@task_item n=15 part=needs
The founder's word owed on the socket-directory pair's wording (the per-user 0700 directory, $XDG_RUNTIME_DIR/viola/ or /tmp/viola-<uid>/) and on the board's read rule (list rows, unwrapped rows from claude agents --json printed through escape_row).
@task_item n=16 part=needs
v1-13 and v1-36 dispositions: link, unlink and release stay off the MCP tool list (an affordance, not an enforcement) and no new channel method, verb or listener is added; the clauses are deferred or re-worded, not built.
@task_item n=17 part=needs
Indirect, through the e2e harness edits under crates/viola-e2e/src/harness: a test-side removal under target/e2e-home takes only a dir the test side created, its keepers never following a linked base unless absolute and owner-only.
@task_item n=18 part=needs
The supply-chain trust-boundary line in the Threat Model Summary and the CI/CD line (one-leg ubuntu-latest matrix): the portable-pty pin named there and the Linux-only CI they state must agree with the code after the bump and the remnant removal.
@task_standing n=1
convention: tests and live runs use their own --home; every child of the local gate is launched under env -i with exactly HOME and PATH, no host environment value crossing into a child
home: Secret Management
@task_standing n=2
convention: no log line, diagnostic, evidence record or CI artifact carries a CLAUDE* stripped value, user content, an absolute path or tool input; diagnostics files are 0600
home: Bootstrap phases (logging-redaction-wire)
@task_standing n=3
convention: every CI upload waits on the artifact canary secret scan, which never prints or writes the matched bytes
home: secret-management
@task_standing n=4
convention: a test-only seam or environment read is compiled out of every release build, switches off no control, and needs its own Decisions Log entry
home: Security Anti-Patterns (Universal)
@task_standing n=5
convention: any change to the CI workflows keeps actions pinned by full SHA with a version comment, workflow-level permissions empty and job-level contents read, and event values reaching steps only through env
home: dependency-security
@task_item n=1 part=not_needed
Paste text and dialog free text validation (validate_paste_text, control-character, empty-text): the chunk adds no send or answer surface and changes no validator.
@task_item n=2 part=not_needed
Hook stdin parsing, the hook statusline shell-out and the statusline source read: no hook code is changed.
@task_item n=3 part=not_needed
MCP stdin line capping and MCP tool input checking: the MCP server's reader and handlers are not changed.
@task_item n=4 part=not_needed
Channel JSON-RPC error codes and MCP isError structuredContent code words: no reply shape changes.
@task_item n=5 part=not_needed
The v1.x release prerequisites (dist, attestations and signing, cargo-auditable, self_update signatures, no rust-cache in the release workflow): 0.1.0 has no release.
@task_item n=6 part=not_needed
viola verify's record fixtures, its probe captures and its trusted runs B to E: the chunk runs no verify recording.
@task_item n=7 part=not_needed
The live round of viola plugin install and its folder-trust dialog: the chunk runs no plugin install.
@task_item n=8 part=not_needed
events.ndjson retention and the local-availability accepted risk: an open arch item, not this chunk's.
@task_item n=9 part=not_needed
Driver-originated release refusal (release with from) and the from param as identity: no channel method changes.
@task_item n=10 part=not_needed
Session id as a child argv value for revive and the claude agents --json output parser: not touched.
@task_item n=11 part=not_needed
Terminal colour facts (NO_COLOR, TERM, COLORTERM) in list's human mode: not touched.
@task_item n=12 part=not_needed
Account, password, RBAC, TLS, HSTS, certificate pinning, SQL, upload and webhook rows: all N/A for a local tool.
@task_item n=13 part=not_needed
The ledger stamps gate for non-null dialog answers (unverified-cli, read_stamps_strict): the chunk answers no dialog with a decision and changes no stamp write.
