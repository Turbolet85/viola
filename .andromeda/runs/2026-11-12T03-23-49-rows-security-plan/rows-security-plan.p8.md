@row_add n=1 row=decision/server-verified-before-any-frame area=security-anti-patterns rec=1
name: Server verified before any channel frame
rule: No client writes any channel frame before verifying the server's euid, pid and process start time against the instance snapshot; an unreadable euid is refused and a mismatch or an unmakeable check sends no frame.
@row_add n=2 row=decision/snapshot-strict-checked-before-trust area=security-anti-patterns rec=1
name: Snapshot reference values only after strict-modes check
rule: The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
@row_add n=3 row=decision/test-removal-own-dirs-only area=security-anti-patterns rec=2
name: Test-side removal only of test-created dirs
rule: A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix.
@row_add n=4 row=decision/e2e-home-link-guard area=security-anti-patterns rec=2
name: E2E home linked base guard
rule: The e2e home keepers follow a linked base only when its target is absolute and reads as a real directory with no group or other bit, refuse a link to a link or a backing below the 9 GiB headroom floor with a fixed message naming no path, create the target 0700 and remove nothing.
why: a ratified boundary widening: on the dev host the path links outside the working directory
@row_add n=5 row=decision/dialog-answers-need-verify-stamp area=security-anti-patterns rec=3
name: Dialog answers need a verify stamp
rule: A dialog gets a non-null decision only when the ledger holds a verify stamp for the child's CLI version with every relied ledger row passing; otherwise the hook prints nothing.
@row_add n=6 row=decision/stamps-written-only-by-verify area=security-anti-patterns rec=3
name: Only verify writes the stamps
rule: No process other than viola verify writes the ledger stamps file.
why: a stamp is what switches on dialog answering, including permission allow
@row_add n=7 row=contract/cli-exit-codes/12 area=security-anti-patterns rec=3
boundary: CLI exit codes
item: 12
promise: answer refused unverified-cli: no verify stamp covers every relied ledger row for the child's CLI version.
@row_add n=8 row=decision/stamps-strict-check-first area=security-anti-patterns rec=4
name: Stamps touched only after strict-modes check
rule: viola verify reads or writes the ledger stamps only after the strict-modes check passes, and a refusal reads and writes nothing.
@row_add n=9 row=decision/strict-check-before-narrowing area=security-anti-patterns rec=4
name: Strict-modes check before any narrowing
rule: The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks.
why: a widened directory narrowed to 0700 first would pass the check
@row_add n=10 row=decision/no-control-switch-off area=security-anti-patterns rec=5
name: No switch turns off a control
rule: No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation.
why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence
except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
@row_add n=11 row=decision/test-seams-fake-agent-only area=security-anti-patterns rec=5
name: Test seams only under the fake-agent feature
rule: An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
@row_add n=12 row=decision/colour-facts-choose-sgr-only area=security-anti-patterns rec=5
name: Terminal colour facts choose SGR only
rule: NO_COLOR, TERM and COLORTERM are read only by viola list's human mode to choose whether and how deep SGR bytes are written; they change no control, widen no redaction and reach no log line, error or refusal.
why: ratified
@row_add n=13 row=decision/no-seam-forcing-peer-refusal area=security-anti-patterns rec=6
name: No seam to force a peer-euid refusal
rule: No env, flag or feature seam forces a refusing peer-euid decision; the decisions are proven over injected values and each client seam's wiring stays the named weakest proof.
why: one account can make no foreign peer
@row_none n=1 rec=7 type=5
@row_none n=2 rec=8 type=5
@row_add n=14 row=decision/statusline-only-shell-out area=threat-model-summary rec=9
name: Statusline command is the only shell-out
rule: The user-written statusline command run through a shell is the only shell-out in viola, and the per-session settings override holds the pinned hook command, never the user's command.
@row_add n=15 row=decision/parent-identity-env-stripped area=threat-model-summary rec=10
name: Parent identity variables stripped from the child
rule: The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
@row_add n=16 row=absence/model-api-calls area=threat-model-summary rec=10
capability: Model API calls of its own; the subscription credentials belong to the unmodified claude CLI
@row_add n=17 row=absence/secrets area=threat-model-summary rec=10,23
capability: Secrets of any kind in v1
@row_add n=18 row=decision/spawn-cwd-snapshot-only area=threat-model-summary rec=11
name: Spawn directory kept in the snapshot only
rule: The start's spawn directory lives only in the 0600 snapshot, on no log line, error body or stdout, and is omitted when absent.
why: an absolute host path can carry the OS username
@row_add n=19 row=decision/same-user-trust-boundary area=threat-model-summary rec=12
name: Same OS user trust boundary
rule: The wrapper channel trusts every process of the same OS user and no other; there is no per-connection authentication and the caller's self-reported from is not authenticated.
@row_add n=20 row=decision/socket-in-owner-only-dir area=threat-model-summary rec=12
name: Socket in a per-user owner-only directory
rule: The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
@row_add n=21 row=decision/exclusive-endpoint-bind area=threat-model-summary rec=12
name: Exclusive endpoint bind
rule: viola run binds its endpoint exclusively, so a squatted name blocks startup with exit 1.
@row_add n=22 row=contract/wrapper-channel-methods/send area=threat-model-summary rec=12
boundary: wrapper channel methods
item: send
promise: Types a prompt into the driven Claude session.
@row_add n=23 row=contract/wrapper-channel-methods/wait area=threat-model-summary rec=12
boundary: wrapper channel methods
item: wait
promise: Reads the driven session's conversation.
@row_add n=24 row=contract/wrapper-channel-methods/last area=threat-model-summary rec=12
boundary: wrapper channel methods
item: last
promise: Reads the driven session's latest reply.
@row_add n=25 row=contract/wrapper-channel-methods/answer area=threat-model-summary rec=12
boundary: wrapper channel methods
item: answer
promise: Answers a dialog, including allowing a PermissionRequest with behavior allow.
@row_add n=26 row=contract/wrapper-channel-methods/pause area=threat-model-summary rec=12
boundary: wrapper channel methods
item: pause
promise: A method of the wrapper channel, JSON-RPC 2.0 over ndjson.
@row_add n=27 row=contract/wrapper-channel-methods/release area=threat-model-summary rec=12
boundary: wrapper channel methods
item: release
promise: A method of the wrapper channel, JSON-RPC 2.0 over ndjson.
@row_add n=28 row=contract/wrapper-channel-methods/link area=threat-model-summary rec=12
boundary: wrapper channel methods
item: link
promise: Adds a driver link, whose effect is a link line in the event log and the snapshot's links only; the self-reported from is not read.
@row_add n=29 row=contract/wrapper-channel-methods/unlink area=threat-model-summary rec=12
boundary: wrapper channel methods
item: unlink
promise: Removes a driver link, whose effect is an unlink line in the event log and the snapshot's links only; the self-reported from is not read.
@row_add n=30 row=contract/wrapper-channel-methods/hook-dialog area=threat-model-summary rec=12
boundary: wrapper channel methods
item: hook.dialog
promise: Carries a dialog from the hook to the wrapper and its decision back.
@row_add n=31 row=contract/wrapper-channel-methods/hook-event area=threat-model-summary rec=12
boundary: wrapper channel methods
item: hook.event
promise: A notification from the hook to the wrapper.
@row_add n=32 row=decision/client-reads-closed-values-only area=threat-model-summary rec=12
name: Client screens the server's answer to closed values
rule: A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result.
except: a code word outside the closed list stays visible by design
@row_add n=33 row=contract/instance-unreachable-detail/strict-modes-failed area=threat-model-summary rec=12
boundary: instance-unreachable detail
item: strict-modes-failed
promise: The instance's strict-modes check refused; no frame was sent.
@row_add n=34 row=contract/instance-unreachable-detail/instance-dead area=threat-model-summary rec=12
boundary: instance-unreachable detail
item: instance-dead
promise: No endpoint with the instance directory standing, or a failed connect.
@row_add n=35 row=contract/instance-unreachable-detail/unwrapped-name area=threat-model-summary rec=12
boundary: instance-unreachable detail
item: unwrapped-name
promise: No endpoint with anything else at that path: absent, a regular file or a symlink.
@row_add n=36 row=contract/instance-unreachable-detail/endpoint-lost area=threat-model-summary rec=12
boundary: instance-unreachable detail
item: endpoint-lost
promise: The wrapper was lost with the call open.
@row_add n=37 row=contract/instance-unreachable-detail/server-verify-failed area=threat-model-summary rec=12
boundary: instance-unreachable detail
item: server-verify-failed
promise: The server or peer euid verification refused; no frame was sent.
@row_add n=38 row=contract/instance-unreachable-detail/socket-dir-refused area=threat-model-summary rec=12
boundary: instance-unreachable detail
item: socket-dir-refused
promise: The socket directory of the recorded endpoint was refused; no frame was sent.
@row_add n=39 row=decision/bridge-send-one-bracketed-paste area=threat-model-summary rec=13
name: Bridge send is one bracketed paste
rule: A bridge send is one bracketed-paste write plus CR of the typed text, with every CR LF pair and every other CR as one LF and without trailing CR and LF characters.
@row_add n=40 row=contract/send-refusal/empty-text area=threat-model-summary rec=13
boundary: send refusal
item: not-delivered / empty-text
promise: A text whose typed text is empty is refused and nothing is written.
@row_add n=41 row=decision/delivery-confirmation-exact-match area=threat-model-summary rec=13
name: Delivery confirmed by exact equality
rule: Delivery confirmation compares the submitted prompt text with the typed text by exact equality, with no tolerance on the submitted side.
@row_add n=42 row=decision/stdin-closed-non-editing-list area=threat-model-summary rec=14
name: Stdin classifier's closed non-editing list
rule: Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess.
why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start
@row_add n=43 row=contract/send-refusal/human-typing area=threat-model-summary rec=14
boundary: send refusal
item: human-typing
promise: A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
@row_add n=44 row=contract/wheel-event/human-input area=threat-model-summary rec=14
boundary: wheel event
item: cause human-input
promise: The wheel moved to holder human because a byte on viola run's stdin counted as typing.
@row_add n=45 row=decision/tolerant-hook-parsing area=threat-model-summary rec=15
name: Tolerant hook payload parsing
rule: Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
@row_add n=46 row=decision/upstream-text-is-content area=threat-model-summary rec=15,16
name: Upstream text is content, never commands
rule: Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola.
why: mechanism, not policy
@row_add n=47 row=decision/hook-fails-open area=threat-model-summary rec=15
name: Hook fails open to the human
rule: Any hook failure fails open to the human: exit 0 with no body.
@row_add n=48 row=decision/hook-strict-check-first area=threat-model-summary rec=15
name: Hook strict-modes check before any read
rule: Every hook path runs the instance's strict-modes check before anything under the home is read or opened; a refused or absent instance exits 0 with no config read, no stdin read, no line and no frame.
@row_add n=49 row=decision/replayed-snapshot-yields-nothing area=threat-model-summary rec=15,19
name: Replayed snapshot runs nothing and frames nothing
rule: A snapshot replayed from the log carries no endpoint, pid, start time or statusline command, so it yields no frame and runs no command.
@row_add n=50 row=contract/mcp-tools/send area=threat-model-summary rec=16
boundary: MCP tools
item: send
promise: A tool called by the driver model, input checked by its schema and the ViolaName check.
@row_add n=51 row=contract/mcp-tools/wait area=threat-model-summary rec=16
boundary: MCP tools
item: wait
promise: A tool called by the driver model that reads the driven session's output.
@row_add n=52 row=contract/mcp-tools/last area=threat-model-summary rec=16
boundary: MCP tools
item: last
promise: A tool called by the driver model that reads the driven session's latest reply.
@row_add n=53 row=contract/mcp-tools/answer area=threat-model-summary rec=16
boundary: MCP tools
item: answer
promise: A tool called by the driver model that answers the driven session's dialog, including a PermissionRequest.
@row_add n=54 row=contract/mcp-tools/list area=threat-model-summary rec=16
boundary: MCP tools
item: list
promise: A tool called by the driver model, input checked by its schema.
@row_add n=55 row=decision/mcp-tool-list-not-enforcement area=threat-model-summary rec=16
name: MCP tool list is an affordance
rule: Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
@row_add n=56 row=decision/mcp-stdin-line-cap area=threat-model-summary rec=17
name: MCP stdin lines capped at the frame limit
rule: An over-long driver stdin line is refused whole and ends the MCP session after the earlier lines are answered, never truncated, skipped or dispatched in part, and its warning holds no byte of it.
@row_add n=57 row=decision/state-tree-strict-modes-before-use area=threat-model-summary rec=18,19
name: State tree strict-modes check before first use
rule: run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
@row_add n=58 row=contract/cli-exit-codes/21 area=threat-model-summary rec=18
boundary: CLI exit codes
item: 21
promise: A frame verb refused because the instance failed the strict-modes check.
@row_add n=59 row=decision/list-home-read-rule area=threat-model-summary rec=18
name: Board scan refuses a readable home
rule: viola list's board scan holds the home to the read rule, so a home another user may read is refused whole, with or without an instance.
@row_add n=60 row=contract/internal-error-output/json area=threat-model-summary rec=18
boundary: catch-site internal error output
item: --json
promise: Where the command prints at the catch site, --json before any -- gives {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, in place of the human line error: internal error.
@row_add n=61 row=decision/readers-tolerate-torn-lines area=threat-model-summary rec=19
name: Readers tolerate torn lines
rule: Readers across processes skip and count torn lines, and the next append heals a torn last line.
@row_add n=62 row=decision/pinned-artefacts-never-deleted area=threat-model-summary rec=19
name: Pinned binary and plugin folders kept
rule: The pinned binary and plugin folders are never deleted in v1.
@row_add n=63 row=decision/marketplace-off-integrity-set area=threat-model-summary rec=19
name: Marketplace off the integrity set
rule: The marketplace directory written by plugin install is off the strict-modes integrity set and never read back by viola.
@row_add n=64 row=decision/pinned-binary-placed-once area=threat-model-summary rec=20
name: Pinned binary placed once
rule: The pinned binary is placed once and never replaced, and the placement closes its write handle before the name appears.
why: an exec of a copy another start still holds open for write fails with ETXTBSY
@row_add n=65 row=decision/run-pins-own-folder-first-on-path area=threat-model-summary rec=21
name: Pinned folder first on the child's PATH
rule: viola run puts the pinned copy's folder first on the child's PATH, while claude itself is resolved as a bare name on the calling process's PATH.
@row_add n=66 row=decision/verify-children-direct-spawn area=threat-model-summary rec=21
name: Verify children spawned directly
rule: viola verify's interactive children are each a direct spawn, never through a shell, under the parent-identity strip at a fixed 80x24, and paste only compiled literals.
why: ratified widenings
@row_add n=67 row=decision/verify-writes-no-dialog-byte area=threat-model-summary rec=21
name: Verify never types into a dialog
rule: No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output.
why: ratified
@row_add n=68 row=decision/nothing-written-under-claude-home area=threat-model-summary rec=21
name: Nothing written under the Claude home
rule: viola writes nothing under ~/.claude.
@row_add n=69 row=decision/agents-probe-log-content-free area=threat-model-summary rec=21
name: Agents probe log holds no content
rule: An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
@row_add n=70 row=decision/verify-screen-read-against-literals area=threat-model-summary rec=21
name: Verify screen read only against compiled literals
rule: verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
@row_add n=71 row=decision/child-output-tolerant-unknown area=threat-model-summary rec=21
name: Child output parsing fails to unknown
rule: Child output parsing is tolerant, and a failure becomes unknown.
@row_add n=72 row=decision/cargo-deny-gate area=threat-model-summary rec=22
name: Cargo deny gate on the root graph
rule: cargo deny checks advisories, licences, sources and C-crate, telemetry-crate and feature bans over the root lockfile.
except: the test-only fuzz workspace sits outside it by an operator-ratified exemption, its lockfile getting its own advisory and source audit
@row_add n=73 row=decision/sync-crates-tokio-free area=threat-model-summary rec=22
name: Sync crates free of tokio
rule: A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature.
guard: deny-sync.toml; scripts/channel-tokio-free.sh
@row_add n=74 row=decision/actions-sha-pinned area=threat-model-summary rec=22
name: Third-party actions SHA-pinned
rule: Third-party GitHub Actions are SHA-pinned.
@row_add n=75 row=decision/toolchains-by-rustup area=threat-model-summary rec=22
name: Toolchains installed by rustup
rule: Every toolchain is installed by rustup, with no toolchain action.
@row_add n=76 row=decision/crates-unpublished area=threat-model-summary rec=22
name: Crates not published
rule: Every crate is publish = false.
@row_add n=77 row=absence/release-stage area=threat-model-summary rec=22,23
capability: A release, signing or deploy stage in v1
@row_add n=78 row=decision/rmcp-features-exact area=threat-model-summary rec=23
name: rmcp features pinned in the release graph
rule: rmcp is present in viola's release graph with exactly the features server and transport-io.
guard: scripts/rmcp-features.sh
@row_add n=79 row=decision/release-build-no-test-binaries area=threat-model-summary rec=23
name: Release build refuses test-only binaries
rule: The CI release build refuses any test-only binary.
guard: scripts/release-check.sh
@row_add n=80 row=decision/mutation-testing-at-audit area=threat-model-summary rec=23
name: Mutation testing at the epoch audit
rule: No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
@row_add n=81 row=decision/real-cli-local-only area=threat-model-summary rec=23
name: Real claude CLI only locally
rule: The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
