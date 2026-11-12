@row_add n=1 row=decision/no-auth-library-local-boundary area=bootstrap-phases rec=1
name: Authentication narrowed to the local boundary
rule: Authentication is narrowed to the local boundary, and no auth library is taken.
@row_add n=2 row=decision/socket-owner-only-hardening area=bootstrap-phases rec=1
name: Owner-only channel socket
rule: The channel socket lives in a 0700 per-user socket directory, is made 0600 after the bind, and the wrapper admits a peer only after a peer euid check.
@row_add n=3 row=decision/client-checks-before-frame area=bootstrap-phases rec=1,28
name: Client checks before any frame
rule: Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).
@row_add n=4 row=decision/plaintext-state-os-permissions area=data-protection rec=3,1
name: Plaintext state protected by OS permissions
rule: Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files.
except: The pinned bin/<version>-<hash>/viola copy is 0700.
@row_add n=5 row=decision/strict-modes-check area=data-protection rec=3,1
name: Strict-modes check on trusted state
rule: A strict-modes check runs at startup over the home and trusted state paths, refusing a path readable by group or others, a symlink, or a non-directory where a directory is trusted.
@row_add n=6 row=decision/code-bearing-files-rewritten-each-start area=data-protection rec=3,1
name: Code-bearing files rewritten at every start
rule: The plugin files and the instance settings.json override are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy.
why: A code-bearing file written at every start and handed to the CLI is a ratified boundary widening.
@row_add n=7 row=decision/statusline-override-plain-path-only area=data-protection rec=3,1
name: Statusline override only for a plain pinned path
rule: The settings.json statusline override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings flag is passed.
why: A shell runs the command string unquoted.
@row_add n=8 row=decision/exec-commands-absolute-pinned-path area=data-protection rec=3,1
name: Exec commands use the absolute pinned path
rule: Every exec-form command viola writes for the CLI is the absolute pinned bin/<version>-<hash>/viola path, never a PATH lookup.
@row_add n=9 row=decision/pinned-copy-content-hash area=data-protection rec=3,1
name: Pinned copy keyed and checked by SHA-256
rule: The pinned copy's <hash> is the first 16 hex characters of the SHA-256 of the exe bytes; before reuse the copy is re-hashed and a mismatch refuses the start with exit 1.
why: Being cryptographic, the hash catches substitution as well as accidental corruption.
guard: contract_content_hash
@row_add n=10 row=decision/sha2-content-hash area=data-protection rec=3,1
name: sha2 is the SHA-256 implementation
rule: The SHA-256 implementation is the sha2 crate with default features off.
why: Pure Rust, no cc on the one target triple, so it passes the cargo-deny C-build ban.
guard: contract_content_hash
@row_add n=11 row=decision/no-non-cryptographic-hash-fallback area=data-protection rec=3
name: No non-cryptographic hash fallback
rule: A non-cryptographic fallback for the pinned copy's content hash is banned.
@row_add n=12 row=decision/same-user-tampering-out-of-scope area=data-protection rec=3
name: Same-user tampering outside the boundary
rule: Tampering with the pinned copy by the same OS user stays outside the v1 security boundary.
why: The same user could also rewrite the folder name.
@row_add n=13 row=decision/stamps-written-only-by-verify area=data-protection rec=3,1
name: Capability stamps written only by verify
rule: The ledger stamps file is written only by viola verify, under its lock, with mode 0600.
@row_add n=14 row=decision/marketplace-written-only-by-install area=data-protection rec=3
name: Marketplace tree written only by plugin install
rule: The marketplace tree is written only by viola plugin install, each file rewritten whole at 0600 and each dir 0700, with no hooks in any document and any other entry left as found; viola never reads the tree back.
@row_add n=15 row=decision/no-writes-under-claude-home area=data-protection rec=3,13
name: viola writes nothing under the Claude home
rule: viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands.
@row_add n=16 row=decision/verify-probe-captures-transient area=data-protection rec=3
name: Verify probe captures are transient
rule: viola verify's content-bearing probe captures are 0600 files in a 0700 per-pid probe directory, and only a scrubbed --record copy leaves it.
@row_add n=17 row=decision/verify-probe-dirs-cleanup area=data-protection rec=3
name: Verify probe directories removed by drop guards
rule: Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists.
except: SIGKILL, a crash or a failed handler install leave the directories to be removed by hand.
@row_add n=18 row=decision/verify-probes-confined-to-own-dir area=data-protection rec=3,13
name: Verify probe effects stay in their own directory
rule: Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.
@row_add n=19 row=decision/pinned-copy-no-clobber-placement area=data-protection rec=4,5,3
name: No-clobber placement of the pinned copy
rule: An absent pinned copy is placed by a no-clobber placement of a temp file whose mode is set before any byte, written, synced and closed first; a present name is never replaced, the target left untouched and the temp file removed.
why: The name never appears while a handle holds it open for write.
@row_add n=20 row=decision/lost-placement-rehash area=data-protection rec=6,3
name: A lost placement re-hashes the standing copy
rule: When a concurrent start placed the pinned copy first, the standing copy is re-hashed: equal bytes are reused, different bytes are a hash mismatch with the file never overwritten or deleted.
@row_add n=21 row=decision/two-keys-per-tree-accepted area=data-protection rec=7
name: One tree may key two pinned folders
rule: Clean release builds of one tree may key two bin and plugin folders; this is accepted with no action, the pinned binary's re-hash staying the guard.
@row_add n=22 row=contract/events-link-lines/data area=data-protection rec=8
boundary: events.ndjson link lines
item: data
promise: A link or unlink line carries data {driver, driven}: the two instance names only, no user content.
@row_add n=23 row=contract/events-link-lines/no-op-pairs area=data-protection rec=8
boundary: events.ndjson link lines
item: a repeated link or an absent pair's unlink
promise: A held pair's second link and an absent pair's unlink append nothing.
@row_add n=24 row=contract/channel-link-method/reply area=data-protection rec=8,29
boundary: the wrapper's link method
item: reply
promise: A successful link replies only {since, already_linked}, since a string and already_linked a boolean.
@row_add n=25 row=contract/channel-unlink-method/reply area=data-protection rec=8,29
boundary: the wrapper's unlink method
item: reply
promise: A successful unlink replies only an empty object {}.
@row_add n=26 row=decision/link-names-kept-out-of-process-log area=data-protection rec=9
name: Link names kept out of the process log
rule: The wrapper's link and unlink methods write no process-log line of their own and their request and response lines carry no name; viola link and unlink print only their own parsed arguments, never a name read back from a reply or log.
@row_add n=27 row=decision/diagnostics-open-fails-to-sink area=data-protection rec=10
name: A refused diagnostics open writes nowhere
rule: Diagnostics opens run the strict-modes check on the existing diagnostics folder and file before narrowing anything; a refusal is a failed open, the writer goes to a sink and the detail line is dropped.
@row_add n=28 row=decision/probe-prompts-synthetic area=data-protection rec=11
name: Probe prompts are synthetic
rule: Probe prompts used to record committed fixtures must be synthetic.
@row_add n=29 row=decision/fixture-scrub-and-refusal area=data-protection rec=11
name: Recorded fixtures scrubbed or refused whole
rule: Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
except: A workspace.repo naming this repository's own remote keeps its owner handle, neither rewritten nor refused.
@row_add n=30 row=decision/screen-fixture-signature-rows-only area=data-protection rec=11
name: Screen fixtures keep only signature rows
rule: A screen fixture keeps only the rows holding a compiled signature literal, the rest blanked; a kept row the scrub would rewrite, or a username split across a row seam, refuses the recording.
@row_add n=31 row=contract/fixture-refusal-line/form area=data-protection rec=11
boundary: verify's unclean-fixture refusal
item: line form
promise: The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
@row_add n=32 row=contract/fixture-refusal-codes/home-path area=data-protection rec=11
boundary: verify's unclean-fixture refusal codes
item: home-path
promise: A closed check code of the fixture refusal, for a surviving home path.
@row_add n=33 row=contract/fixture-refusal-codes/absolute-path area=data-protection rec=11
boundary: verify's unclean-fixture refusal codes
item: absolute-path
promise: A closed check code of the fixture refusal, for a surviving absolute path.
@row_add n=34 row=contract/fixture-refusal-codes/username area=data-protection rec=11
boundary: verify's unclean-fixture refusal codes
item: username
promise: A closed check code of the fixture refusal, for a surviving user word.
@row_add n=35 row=contract/fixture-refusal-codes/email area=data-protection rec=11
boundary: verify's unclean-fixture refusal codes
item: email
promise: A closed check code of the fixture refusal, for a surviving email-shaped token.
@row_add n=36 row=decision/mcp-fixtures-via-test-only-tap area=data-protection rec=11
name: MCP fixtures recorded through a test-only tap
rule: MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes.
guard: schemas/claude-mcp-fixture.v1.json
@row_add n=37 row=contract/verify-refusal-codes/program-not-found area=data-protection rec=12
boundary: verify's refusal codes
item: program-not-found
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=38 row=contract/verify-refusal-codes/pinned-hash-mismatch area=data-protection rec=12
boundary: verify's refusal codes
item: pinned-hash-mismatch
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=39 row=contract/verify-refusal-codes/version-unreadable area=data-protection rec=12
boundary: verify's refusal codes
item: version-unreadable
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=40 row=contract/verify-refusal-codes/strict-modes-failed area=data-protection rec=12
boundary: verify's refusal codes
item: strict-modes-failed
promise: One of the closed codes verify writes on its process-exit when refused.
@row_add n=41 row=contract/verify-refusal-codes/unclean-fixture area=data-protection rec=12
boundary: verify's refusal codes
item: unclean-fixture
promise: One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.
@row_add n=42 row=absence/key-management area=data-protection rec=13
capability: Keys at rest and their management
@row_add n=43 row=decision/events-rotation-keeps-modes-and-offsets area=data-protection rec=13
name: Event log rotation keeps modes and offsets
rule: Any rotation scheme for events.ndjson keeps 0600/0700 permissions and keeps offsets valid.
@row_add n=44 row=decision/verify-real-cli-transcripts-accepted area=data-protection rec=13
name: Verify's real-CLI transcripts accepted
rule: verify's real interactive runs under a trusted parent leave the CLI's own synthetic-prompt session transcripts and run the user's global hooks; this residual is an accepted risk class.
@row_add n=45 row=decision/plugin-install-live-round-rig area=data-protection rec=13
name: Live plugin-install rounds isolated
rule: A live round of viola plugin install runs the CLI's commands at local scope from a project that is its own git repo outside the work tree, passing the trust dialog only by keys into its own pty, never a desktop key.
@row_add n=46 row=decision/verify-never-answers-import-dialog area=data-protection rec=13
name: Real-CLI verify needs a prepared trusted folder
rule: viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog.
@row_add n=47 row=decision/event-log-growth-availability-risk area=data-protection rec=13
name: Event log growth accepted as availability risk
rule: Unbounded growth of events.ndjson exhausting local disk is an accepted risk until retention is resolved.
why: The impact is on availability only, with no confidentiality or integrity impact.
@row_add n=48 row=decision/serde-json-unbounded-depth-banned area=dependency-security rec=14
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned in cargo-deny.
why: Every parse of external bytes keeps serde_json's default recursion limit.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth
@row_add n=49 row=decision/no-stale-license-exceptions area=dependency-security rec=15
name: No license exception for an absent crate
rule: A license exception naming a crate absent from the graph is dropped, never kept.
why: Such an exception narrows nothing and widens nothing.
@row_add n=50 row=decision/c-building-crates-banned area=dependency-security rec=16
name: C-building crates banned
rule: Crates that build C are banned from the dependency graph by cargo-deny.
guard: scripts/deny-probes.sh
@row_add n=51 row=decision/no-tokio-in-sync-crates area=dependency-security rec=16
name: No tokio in the sync crates
rule: tokio is banned from each sync crate's graph, checked per sync crate as the sole root.
except: viola-channel's Tokio client sits behind its tokio feature, its graph without the feature checked to hold no tokio by a cargo tree probe.
guard: scripts/deny-probes.sh; scripts/channel-tokio-free.sh
@row_add n=52 row=decision/cargo-deny-gate-clean area=dependency-security rec=17
name: cargo-deny gate with no advisory ignores
rule: CI's supply-chain gate runs cargo deny over advisories, bans, licenses and sources with the advisory ignore list empty.
@row_add n=53 row=decision/exact-third-party-pins area=dependency-security rec=18
name: Exact third-party version pins
rule: The workspace pins every third-party dependency version exactly.
@row_add n=54 row=decision/fuzz-workspace-exempt-from-root-deny area=dependency-security rec=19
name: Fuzz workspace outside the root deny graph
rule: fuzz is a separate cargo workspace with its own lockfile, outside cargo-deny's root graph, so the C-build ban does not apply to it.
why: Ratified as a test-only exemption.
except: Holds only while the fuzz crate is never linked into a shipped binary and never joins the root workspace.
@row_add n=55 row=decision/fuzz-lockfile-audited-separately area=dependency-security rec=19
name: Fuzz lockfile audited separately
rule: The fuzz lockfile is audited separately for advisories and sources in CI and for advisories weekly.
@row_add n=56 row=decision/g2-zero-panics-single-exemption area=dependency-security rec=20
name: Zero-panics gate with one exemption
rule: CI's zero-panics gate counts every role-file panic line; any further exemption needs its own Decisions Log entry.
except: A panic whose location is exactly the hook seam's forced-panic file and a line number, compared as a whole string; the perf arm exempts nothing.
guard: G2 --probe
@row_add n=57 row=decision/actions-pinned-by-sha area=dependency-security rec=21
name: CI actions pinned by commit SHA
rule: Every GitHub Action CI uses is pinned by full commit SHA.
@row_add n=58 row=decision/checkout-no-persisted-credentials area=dependency-security rec=21
name: Checkout without persisted credentials
rule: The CI checkout action runs with persist-credentials false.
@row_add n=59 row=decision/no-repo-secret-scanner area=bootstrap-phases rec=2
name: No repository secret scanner in v1
rule: v1 runs no secret scanner over the repository.
why: There are no secrets in the repo or CI.
@row_add n=60 row=decision/ci-artifact-canary-scan area=bootstrap-phases rec=2
name: CI artifact canary scan gates uploads
rule: Every scan-gated CI upload waits for the artifact canary scan over test diagnostics, harness capture and test reports against the NEVER-log floor, which never prints or writes the matched bytes.
except: The harness capture's chunk.diff, repository source text by construction, is skipped by exact path and never uploaded.
@row_add n=61 row=decision/no-mutants-out-upload area=bootstrap-phases rec=2
name: No mutants.out upload
rule: CI does not upload mutants.out.
why: It held absolute argv paths and test output.
@row_add n=62 row=decision/fuzz-artifacts-upload-unscanned area=bootstrap-phases rec=2
name: Fuzz crash artifacts the one unscanned upload
rule: The nightly fuzz job's crash-input upload is the one CI upload left unscanned; a non-synthetic corpus seed must first drop that upload.
why: Ratified as admissible by content: the inputs are mutated from the committed synthetic corpus.
@row_add n=63 row=decision/test-homes-gitignored area=bootstrap-phases rec=2
name: Test homes kept out of git
rule: Local --home test directories are git-ignored so recorded state never lands in git.
@row_add n=64 row=decision/hook-fails-open area=error-handling rec=22
name: hook fails open
rule: viola hook never writes to stderr and never exits non-zero; a failing security check still ends in exit 0 with no body.
why: So the dialog renders for the human.
@row_add n=65 row=decision/hook-strict-modes-first area=error-handling rec=22
name: hook checks the instance before anything
rule: The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
except: The two capture arms run ahead of that check.
@row_add n=66 row=contract/hook-decision-detail/oversize-stdin area=error-handling rec=22
boundary: hook-decision detail
item: oversize-stdin
promise: hook fails open with this detail when stdin exceeds its cap.
@row_add n=67 row=contract/hook-decision-detail/malformed-json area=error-handling rec=22
boundary: hook-decision detail
item: malformed-json
promise: hook fails open with this detail when stdin is not one JSON object.
@row_add n=68 row=contract/hook-decision-detail/channel-unreachable area=error-handling rec=22,25
boundary: hook-decision detail
item: channel-unreachable
promise: hook fails open with this detail when the channel cannot be reached, a replayed or absent snapshot included.
@row_add n=69 row=contract/hook-decision-detail/server-verify-failed area=error-handling rec=22,23
boundary: hook-decision detail
item: server-verify-failed
promise: hook fails open with this detail when the peer euid or server verification fails; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
@row_add n=70 row=contract/hook-decision-detail/socket-dir-refused area=error-handling rec=22,24
boundary: hook-decision detail
item: socket-dir-refused
promise: hook fails open with this detail, in one line at WARN, when the socket directory is refused; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
@row_add n=71 row=decision/hook-refusals-write-no-frame area=error-handling rec=23,24,25
name: A refused hook writes no frame
rule: A hook event or dialog refused by the server check, the socket-directory check or a replayed or absent snapshot writes no frame and exits 0 with empty streams; a refused SessionEnd is still appended directly.
@row_none n=1 rec=26 type=4
@row_add n=72 row=decision/mcp-frame-tool-refusals area=error-handling rec=27,28
name: Refused MCP frame tools send nothing
rule: A viola mcp frame tool refused by the strict-modes, liveness, socket-directory or server check sends no frame, writes no log line and returns the instance-unreachable result.
@row_add n=73 row=contract/mcp-frame-tool-result/unreachable area=error-handling rec=27,28,39
boundary: viola mcp frame tool results
item: instance-unreachable
promise: A refused frame tool returns isError with error instance-unreachable, a detail cause word, and text `refused: instance-unreachable`.
@row_add n=74 row=contract/channel-fault/-32603 area=error-handling rec=29
boundary: wrapper channel faults
item: -32603
promise: A failed link or unlink record answers -32603 with the fixed message `internal error` and data null.
@row_add n=75 row=contract/link-verbs-exit-codes/20 area=error-handling rec=29
boundary: viola link / unlink exit codes
item: 20
promise: A wrapper error reply, a -32602 included, is a wrapper fault, exit 20.
@row_add n=76 row=contract/link-verbs-exit-codes/1 area=error-handling rec=29
boundary: viola link / unlink exit codes
item: 1
promise: A Refused reply or an ok of a shape other than the verb's own is an internal error, exit 1.
@row_add n=77 row=decision/channel-faults-sanitised area=error-handling rec=30,31
name: Channel faults sanitised by the clients
rule: Clients read a wrapper fault only with an integer code; each known code reads its fixed message with data null.
except: Two -32602 bodies keep data: the unsupported-version body with an integer data.supported and a 1-64 character data.wrapper, and the body whose data.reason is release-from-driver.
@row_add n=78 row=contract/channel-fault/unknown-code area=error-handling rec=32
boundary: wrapper faults as read by clients
item: a code outside the five
promise: Reads {"code": <code>, "message": "wrapper fault", "data": null}; `wrapper fault` is a fixed word on --json and MCP, and the human line reads `error: wrapper fault  <code>`.
@row_add n=79 row=decision/reply-code-words-validated area=error-handling rec=33,34
name: Reply code words validated
rule: A refusal or detail word read from a reply must be 1 to 64 bytes of a-z, 0-9 or -, a lower-case letter first; a failing refusal reads unknown, a failing string detail reads null, and a non-string detail is no known reply.
except: A valid code word outside the closed list stays visible in the document and in `refused: <word>`, its exit unknown's 14.
@row_add n=80 row=contract/channel-response-detail/unsupported-version area=error-handling rec=35
boundary: channel-response detail
item: unsupported-version
promise: The closed detail word for the unsupported-version -32602 fault, written on the server's and the client's line.
@row_add n=81 row=contract/channel-response-detail/invalid-params area=error-handling rec=35
boundary: channel-response detail
item: invalid-params
promise: The closed detail word for the invalid-params -32602 fault, written on the server's and the client's line.
@row_add n=82 row=contract/channel-response-detail/release-from-driver area=error-handling rec=35
boundary: channel-response detail
item: release-from-driver
promise: The closed detail word for the release-from-driver -32602 fault, written on the server's and the client's line.
@row_add n=83 row=decision/refused-peer-dropped-before-frame area=error-handling rec=36,1
name: A refused peer is dropped before any frame
rule: A connection whose peer euid is refused is dropped on the accept thread before any worker or frame: the peer reads end-of-file and the wrapper writes no reply, no line and nothing on the terminal.
@row_add n=84 row=decision/internal-error-held-at-unit-layer area=error-handling rec=37
name: Internal errors tested at the unit layer
rule: No -32603 is driven in a scanned home, by failpoint or counted panic; the unit layer holds that path.
guard: answer_a_panicking_dispatch_is_internal_error_and_closes
@row_add n=85 row=decision/cli-frame-verbs-strict-modes-first area=error-handling rec=38
name: CLI frame verbs check the instance first
rule: CLI send, wait, last, answer, pause, release, link and unlink run the instance strict-modes check before any frame; a refusal exits 21, opens nothing under the home and writes no line.
@row_add n=86 row=contract/cli-instance-unreachable/json-document area=error-handling rec=39,38
boundary: CLI exit 21
item: --json document
promise: Exit 21 under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"<cause>"}, the cause one of a closed set of six.
@row_add n=87 row=contract/instance-unreachable-detail/strict-modes-failed area=error-handling rec=39
boundary: instance-unreachable detail
item: strict-modes-failed
promise: A cause of exit 21 and MCP's unreachable result: the instance failed the strict-modes check.
@row_add n=88 row=contract/instance-unreachable-detail/instance-dead area=error-handling rec=39,49
boundary: instance-unreachable detail
item: instance-dead
promise: A cause of exit 21 and MCP's unreachable result: the instance directory has no live endpoint, a stale wrapper included.
@row_add n=89 row=contract/instance-unreachable-detail/unwrapped-name area=error-handling rec=39
boundary: instance-unreachable detail
item: unwrapped-name
promise: A cause of exit 21 and MCP's unreachable result: the name has no instance directory.
@row_add n=90 row=contract/instance-unreachable-detail/endpoint-lost area=error-handling rec=39
boundary: instance-unreachable detail
item: endpoint-lost
promise: A cause of exit 21 and MCP's unreachable result.
@row_add n=91 row=contract/instance-unreachable-detail/server-verify-failed area=error-handling rec=39,40
boundary: instance-unreachable detail
item: server-verify-failed
promise: A cause of exit 21 and MCP's unreachable result: the server check failed; a peer euid refusal has no word of its own and reads this.
@row_add n=92 row=contract/instance-unreachable-detail/socket-dir-refused area=error-handling rec=39,42
boundary: instance-unreachable detail
item: socket-dir-refused
promise: A cause of exit 21 and MCP's unreachable result: the socket directory was refused; on a CLI frame verb, during connect, one process-exit at WARN and no channel-request.
@row_none n=2 rec=41 type=4
@row_add n=93 row=contract/cli-stderr-hints/unwrapped-name-endpoint-lost area=error-handling rec=43
boundary: CLI stderr hints
item: unwrapped-name and endpoint-lost hints
promise: The hints come last on stderr in plain ASCII, their <name> the parsed ViolaName.
@row_add n=94 row=decision/socket-dir-refusal-names-only-name area=error-handling rec=44
name: Socket-directory refusal discloses no path
rule: The socket-directory refusal names only the parsed ViolaName, and its lines hold no path, pid or uid.
@row_add n=95 row=decision/unusable-response-line-on-stderr area=error-handling rec=45
name: Unusable answer or send text goes to stderr
rule: The exit-2 paths of viola answer and viola send write a fixed unusable-response or unusable-text line to stderr whatever --json says, and no document.
why: A named exception to the --json error document.
@row_add n=96 row=decision/home-tree-strict-modes-before-log area=error-handling rec=46
name: Home tree checked before any log opens
rule: viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
@row_add n=97 row=decision/run-refuses-socket-dir-before-start area=error-handling rec=47
name: run refuses a bad socket directory first
rule: viola run and revive's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.
@row_add n=98 row=contract/cli-json-error/internal-error area=error-handling rec=48
boundary: CLI --json error documents
item: internal-error
promise: Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.
@row_add n=99 row=contract/collision-check/stale-heartbeat area=error-handling rec=49
boundary: start collision check
item: stale-heartbeat
promise: Printed and logged for a stale holder of the name; stale heartbeat is exit 1's cause alone.
@row_add n=100 row=contract/collision-check/already-live area=error-handling rec=49
boundary: start collision check
item: already-live
promise: Printed and logged for a live holder and for a fresh replayed beat.
@row_add n=101 row=decision/error-messages-interpolate-nothing area=error-handling rec=50,51,55
name: Error messages interpolate nothing
rule: Product error types carry fixed Display messages with no path or payload interpolated.
guard: thiserror_messages_interpolate_nothing
@row_none n=3 rec=52 type=5
@row_add n=102 row=decision/error-enum-convention-exceptions area=error-handling rec=53
name: Named exceptions to the error-enum convention
rule: ServeError, StampsError, PinError, StampsReadError and Refused diverge from the error-enum convention as named exceptions.
why: Ratified as named exceptions.
@row_none n=4 rec=54 type=4
@row_add n=103 row=decision/every-error-writer-named area=error-handling rec=55
name: Every product error writer named
rule: Every error body is enumerated with its writer and witness, with named exclusions; an unnamed product error writer fails the census.
guard: product_error_writers_are_each_named
