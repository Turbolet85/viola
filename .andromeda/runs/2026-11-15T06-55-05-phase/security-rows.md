# security-plan · the rows that bind 2026-11-15-live-round-on-the-trimmed-product — picked 178 of 595 (124 by every draw) · rule any · pin 1297

## threat-model-summary · ## Threat Model Summary
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/child-path-trust · Child processes trust PATH — Spawned child processes are whatever the calling process's PATH resolves to.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/human-typing-takes-wheel · Human typing takes the wheel — A byte on viola run's own stdin that is typing takes the wheel from the driver. · except: A sequence on the stdin classifier's closed non-editing list never counts as typing.
decision/mcp-tool-list-affordance · MCP tool list is an affordance — Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
decision/mcp-tool-list-not-enforcement · MCP tool list is an affordance — Leaving release, pause, link and unlink off the MCP tool list is an affordance, not an enforcement.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude.
decision/parent-identity-env-stripped · Parent identity variables stripped from the child — The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
decision/pinned-artefacts-never-deleted · Pinned binary and plugin folders kept — The pinned binary and the plugin folders are never deleted in v1.
decision/pinned-binary-placed-once · Pinned binary placed once — The pinned binary is placed once and never replaced, and the placement closes its write handle before the name appears. · why: an exec of a copy another start still holds open for write fails with ETXTBSY
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/run-pins-own-folder-first-on-path · Pinned folder first on the child's PATH — viola run puts the pinned copy's folder first on the child's PATH, while claude itself is resolved as a bare name on the calling process's PATH.
decision/stdin-closed-non-editing-list · Stdin classifier's closed non-editing list — Only a sequence on the closed non-editing list, matched by exact grammar, never counts as typing; every other stdin byte takes the wheel, and a reply no terminal has been measured to write is never added on a guess. · why: ratified; the list was widened once by ratification for measured terminal replies that took the wheel at a session's start
decision/verify-never-answers-dialogs · Verify never answers dialogs — viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key.
decision/verify-screen-read-against-literals · Verify screen read only against compiled literals — verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
decision/verify-signature-only · Verify reads signatures only — viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.
decision/verify-writes-no-dialog-byte · Verify never types into a dialog — No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output. · why: ratified
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/mcp-tools/list · MCP tools · list — A tool called by the driver model that takes no input, reading the board.
contract/send-refusal/human-typing · send refusal · human-typing — A typing byte on viola run's stdin takes the wheel for the human, and the next automation send is refused.
contract/wheel-event/human-input · wheel event · cause human-input — The wheel moved to holder human because a byte on viola run's stdin counted as typing.
decision/agents-probe-log-content-free · Agents probe log holds no content — An agents-probe spawn pair is logged only under an open log and holds no row text, id, cwd or path.
decision/agents-probe-log-redaction · Agents-probe logging redacted — Each agents-probe spawn pair is logged with no row text, id, cwd or path.
decision/cli-input-trust-boundary · CLI input trust boundary — CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype and tolerant config.json parsing.
decision/cwd-only-in-snapshot · Recorded cwd kept to the snapshot — The snapshot's recorded cwd, which can carry the OS username, stands only in the 0600 snapshot and never on a log line, error body or stdout.
decision/readers-tolerate-torn-lines · Readers tolerate torn lines — Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
decision/security-tier · Security tier — The security tier is Minimal (dependency audit, input validation on config and external payloads, error sanitization) with targeted elevations for IPC endpoint access control and server impersonation, bracketed-paste breakout in send text, and integrity of the viola home. · why: viola is local-only and single-user with no accounts, listener, database, stored credentials or regulated data, yet its IPC endpoint executes code and state files decide what code runs.
decision/spawn-cwd-snapshot-only · Spawn directory kept in the snapshot only — The start's spawn directory lives only in the 0600 snapshot, on no log line, error body or stdout, and is omitted when absent. · why: an absolute host path can carry the OS username
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/strict-modes-at-start · Strict-modes check at start — Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals. · why: Ratified live rulings, each given after the widening was shown.
contract/wrapper-channel-methods/hook-dialog · wrapper channel methods · hook.dialog — Carries a dialog from the hook to the wrapper and its decision back.

## authentication-authorization · ## Authentication & Authorization
decision/collision-check-needs-no-server-verify · run's collision check is not server-verified — run's collision check judges a recorded wrapper by its pid, OS start time and heartbeat (a replayed snapshot by the beat alone) and opens no connection, so the peer-pid server check does not apply to it. · why: it writes no frame, so there is no serving process to verify
decision/pinned-viola-never-replaced · Pinned viola is never replaced — The pinned viola is never replaced: an absent copy is written to a 0700 temp file, synced, closed and placed without clobbering, never over a present name.
decision/start-checks-home-tree-before-log-opens · Start checks the home tree before its log opens — Run and revive's start check the whole home tree before the wrapper log opens: the home first, then the instance, ledger, bin and plugin folders and the pinned copy's own. A refusal exits 1 with no line or child. · why: Opening the log narrows the home to its owner, which would erase the state the check refuses; the revive and start tests expect the widened folder left as found.
decision/start-checks-socket-dir-before-bind · Socket directory checked before any bind, snapshot or child — A start, run or revive alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict. · why: When the directory is refused, the start negative test expects no child, no bind, no snapshot and the directory as found.
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/home-owner-only · Owner-only viola home — The viola home is owner-only (0700 directories, 0600 files, the pinned viola 0700), every replaced file getting its mode before any byte is written, never relying on the umask.
decision/leftover-socket-file-kept · Leftover socket file is not removed — A killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it. · why: by choice: a harmless file in a 0700 directory
decision/no-socket-overwrite · No socket overwrite — Binding never overwrites an existing socket; try_overwrite stays off.
decision/private-file-modes · Owner-only modes for the viola home — The home (and any --home) is created 0700 and every file 0600, never group- or world-writable. · except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/unverified-cli-gate · Unverified CLI gate — No non-null answer decision is sent without a viola verify stamp for the child's CLI version.

## input-validation · ## Input Validation
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/cli-home-resolution-order · CLI home resolution order — Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created. · why: A ratified widening of what chooses a CLI verb's home. · except: hook keeps its own shape check.
decision/config-cannot-disable-controls · Configuration cannot disable controls — No configuration value makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
decision/dialog-hold-config-only · Dialog hold set only in config — The dialog hold is read once at the wrapper's start from config.json only; no env var or flag sets it. · why: A ratified boundary widening.
decision/hook-capture-arm · Hook capture arm — The hidden capture arm, called only by verify's probe plugins, writes stdin raw to an exclusively claimed new 0600 file in an absolute existing directory, reads no VIOLA_* variable, opens no channel, writes nothing on failure, keeps stderr empty and exits 0. · why: A ratified boundary widening.
decision/hook-dialog-reply-read-bound · Hook dialog reply read bound — The hook's read of a dialog reply is bounded at the maximum dialog hold plus 5 s (3605 s), the wrapper replying null at its own configured hold's end.
decision/input-box-readiness-from-literals · Input box readiness read from compiled literals — The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a modal literal stands; one predicate decides for verify and for viola run's pre-send gate. · why: A boundary widening that was ratified.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-checked-against-ledger · MAX_FRAME checked against data — verify records the largest hook payload per event kind, stamped per CLI version; a stamped maximum within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/stamps-checked-under-lock · Stamps checked before and under lock — The ledger stamps file is checked before the ledger directory is created and again under its lock.
decision/verify-home-tree-check · verify checks the home tree first — viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
decision/verify-pty-output-caught · verify PTY output bounded and caught — verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.
decision/verify-reads-only-signatures · Verify reads PTY output by signatures — verify's PTY output is read only against the compiled signature literals.
decision/vt100-feed-isolated · Screen model feed isolated from passthrough — PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
contract/config-json/claude-env-keep · config.json · claude_env_keep — An array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$; a bad value is rejected whole and reported as parse-rejected; an identity-floor name in it has no effect.
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — A JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected line; no value makes the hold zero or unbounded.
decision/capture-arms-isolated · Hidden capture arms isolated — The hidden capture arm reads no VIOLA_* variable, opens no channel and always exits 0.
decision/config-bad-value-handling · Bad config values — Configuration is parsed tolerantly under MAX_FRAME with its version checked; a bad value takes its key's default or is rejected whole, and is reported as parse-rejected.
decision/config-tolerant-parse · Tolerant config parse — config.json is parsed tolerantly, unknown keys counted, its v checked and its read capped at MAX_FRAME; a bad value falls back to its default and is reported as parse-rejected.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/names-through-validated-type · Names through the validated name type — Every instance name taken from CLI arguments passes the validated name constructor.
decision/replayed-snapshot-fallbacks · Replayed snapshot fallbacks — Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, and the collision check judges liveness by the heartbeat alone.
decision/run-resolves-absolute-program · run resolves its child program itself — viola run resolves its child program itself and hands the PTY an absolute path: a given path as given, a bare name looked up on PATH; a program not found is ProgramNotFound.
decision/session-id-argv-closed-shape · Session id on child argv — A session id placed on a child's command line has the closed shape (36 characters, hyphens at 8, 13, 18, 23, hex elsewhere) and is one argv element of a direct spawn, never a shell, built only by one resume-args builder. · why: A ratified crossing of a logged value onto a child's command line.
decision/session-id-argv-no-shell · Session id as one argv element — The session id after --resume has the closed session-id shape and is one argv element of a direct spawn, never passed through a shell.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/verify-run-e-resume-target · verify Run E resume target — verify's Run E resumes only the session id of Run B's first SessionStart capture when it has the session-id shape, without fork; with none Run E spawns nothing and resume-session fails. · why: Ratified.
decision/vt100-feed-bounded-queue · Screen feed bounded queue — Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
decision/vt100-feed-under-panic-catch · Screen feed under a panic catch — The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues. · why: vt100 panics are reachable at real small sizes. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough

## data-protection · ## Data Protection
decision/code-bearing-files-rewritten-each-start · Code-bearing files rewritten at every start — The plugin files are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy. · why: A code-bearing file written at every start and handed to the CLI is a ratified boundary widening.
decision/exec-commands-absolute-pinned-path · Exec commands use an absolute path — Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive and verify, and the installing exe's canonical path in plugin install's `.mcp.json`.
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
decision/mcp-fixtures-via-test-only-tap · MCP fixtures recorded through a test-only tap — MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes. · guard: schemas/claude-mcp-fixture.v1.json
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands.
decision/pinned-binary-rehashed · Pinned binary re-hashed before use — The pinned viola binary is re-hashed before use.
decision/pinned-copy-content-hash · Pinned copy keyed and checked by SHA-256 — The pinned copy's <hash> is the first 16 hex characters of the SHA-256 of the exe bytes; before reuse the copy is re-hashed and a mismatch refuses the start with exit 1. · why: Being cryptographic, the hash catches substitution as well as accidental corruption. · guard: contract_content_hash
decision/plugin-files-rewritten-each-start · Plugin files rewritten every start — The plugin files are rewritten at every start.
decision/probe-artifacts-cleanup · Probe artifacts owner-only and removed — Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing. · except: a kill or a crash leaves them to be removed by hand
decision/probe-prompts-synthetic · Probe prompts are synthetic — Probe prompts used to record committed fixtures must be synthetic.
decision/screen-fixture-signature-rows-only · Screen fixtures keep only signature rows — A screen fixture keeps only signature rows (a row holding a compiled footer or modal literal whole, a box rule row as its rule chars alone, a prompt row as the prompt char alone), the rest blanked; another box row, a kept row the scrub would rewrite or a username split across a row seam is refused.
decision/verify-never-answers-import · Verify needs a pre-trusted folder — verify against the real claude runs from a user-trusted folder whose external CLAUDE.md import is already answered; verify never answers it.
decision/verify-never-answers-import-dialog · Real-CLI verify needs a prepared trusted folder — viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog.
decision/verify-probe-captures-transient · Verify probe captures are transient — viola verify's content-bearing probe captures are 0600 files in a 0700 per-pid probe directory, and only a scrubbed --record copy leaves it.
decision/verify-probe-dirs-cleanup · Verify probe directories removed by drop guards — Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists. · except: SIGKILL, a crash or a failed handler install leave the directories to be removed by hand.
decision/verify-probes-confined-to-own-dir · Verify probe effects stay in their own directory — Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file.
contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving absolute path.
contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
contract/fixture-refusal-line/form · verify's unclean-fixture refusal · line form — The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
contract/verify-refusal-codes/pinned-hash-mismatch · verify's refusal codes · pinned-hash-mismatch — One of the closed codes verify writes on its process-exit when refused.
contract/verify-refusal-codes/program-not-found · verify's refusal codes · program-not-found — One of the closed codes verify writes on its process-exit when refused.
contract/verify-refusal-codes/unclean-fixture · verify's refusal codes · unclean-fixture — One of the closed codes verify writes on its process-exit when refused; the fixture's file name enters no code.
contract/verify-refusal-codes/version-unreadable · verify's refusal codes · version-unreadable — One of the closed codes verify writes on its process-exit when refused.
decision/lost-placement-rehash · A lost placement re-hashes the standing copy — When a concurrent start placed the pinned copy first, the standing copy is re-hashed: equal bytes are reused, different bytes are a hash mismatch with the file never overwritten or deleted.
decision/no-non-cryptographic-hash-fallback · No non-cryptographic hash fallback — A non-cryptographic fallback for the pinned copy's content hash is banned.
decision/scratch-footer-reading-residual · Scratch footer reading is no standing permission — A scratch footer reading of a newer CLI is one tmpfs-home run start, no key typed, only scrubbed footer rows kept; it is no product path, and another such reading needs its own answer. · why: accepted residual
decision/verify-real-cli-transcripts-accepted · Verify's real-CLI transcripts accepted — verify's real interactive runs under a trusted parent leave the CLI's own synthetic-prompt session transcripts and run the user's global hooks; this residual is an accepted risk class.
decision/verify-trusted-runs-risk · Verify trusted runs accepted risk — verify's trusted runs B, C and D are real interactive turns whose transcripts the CLI persists and for which the user's global hooks run. · why: accepted risk
contract/verify-refusal-codes/strict-modes-failed · verify's refusal codes · strict-modes-failed — One of the closed codes verify writes on its process-exit when refused.

## dependency-security · ## Dependency Security
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/nothing-runs-as-root · Nothing launched as root — No harness command, gate or pre-push stage launches anything as uid 0, and no provisioning script exists.
decision/serde-json-bounded-depth · serde_json recursion limit kept — cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit. · guard: scripts/deny-probes.sh
contract/release-check-output/test-only-feature · release-check output · release-check: FAILED — test-only feature {feature} in {target} — Printed when a release artifact's features include a test-only feature.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/code-files-rewritten-at-start · Code-bearing files rewritten at start — Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
decision/scan-gated-ci-uploads · CI uploads gated by the artifact canary scan — The CI test job runs the artifact canary scan against the NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass. · except: the nightly fuzz job's crash inputs, mutated from the committed synthetic corpus, leave CI unscanned; a non-synthetic seed must first be dropped
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/ci-artifact-canary-scan · CI artifact canary scan gates uploads — Every scan-gated CI upload waits for the artifact canary scan over test diagnostics, harness capture and test reports against the NEVER-log floor, which never prints or writes the matched bytes. · except: The harness capture's chunk.diff, repository source text by construction, is skipped by exact path and never uploaded.
decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.

## secret-management · ## Secret Management
decision/artifact-canary-scan · CI artifact canary scan — CI always runs the artifact canary secret scan over every file under the agent-run artifact directory, and the test-home uploads run only when it succeeds. · except: The mutants chunk diff is skipped by exact path and excluded from the harness upload, so it is never uploaded.
decision/claude-credentials-stripped-from-child · Inherited CLAUDE credentials stripped from the child — The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
decision/credential-strip-by-name-only · Credential strip reads names only — The environment strip handles variable names only and never reads a variable's value.
decision/identity-floor-eleven-names · Identity floor stays eleven names — The identity floor stays the eleven names; the plugin, project and env-file names are not identity names. · why: Ratified; the prefix rule already removes them when inherited outside the persistent set.
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/messaging-credentials-are-secret · Messaging token and socket are secrets — The inherited CLAUDE_CODE_MESSAGING_TOKEN and CLAUDE_CODE_MESSAGING_SOCKET count as secrets.
decision/r8-claude-env-strip · R8 CLAUDE environment strip — Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values. · why: Ratified.
decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
contract/artifact-canary-scan-classes/claude-stripped · artifact canary scan classes · claude-stripped — The scan fails on any of the twelve stripped CLAUDE* canary needles.
contract/claude-child-process-start/env-kept · process-start claude-child line · env_kept — Names the kept environment names, never a value.
contract/claude-child-process-start/env-stripped-known · process-start claude-child line · env_stripped_known — Names the stripped environment names, never a value.
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product.
decision/no-credential-literals · No credential literals in code — No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
decision/no-stored-secrets · No stored secrets — There are no stored secrets in v1, and environment variables are not a configuration channel. · except: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory
decision/no-uid-0-launch · No root launch — No child is ever launched as uid 0.

## error-handling · ## Error Handling
contract/collision-check/already-live · start collision check · already-live — Printed and logged for a live holder and for a fresh replayed beat.
contract/collision-check/stale-heartbeat · start collision check · stale-heartbeat — Printed and logged for a stale holder of the name; stale heartbeat is exit 1's cause alone.
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arm never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.
decision/run-refuses-socket-dir-before-start · run refuses a bad socket directory first — viola run and revive's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/absolute-viola-path-in-commands · Hook and MCP commands use an absolute viola path — Every exec-form command in the embedded hooks.json or .mcp.json and in plugin install's .mcp.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH. · why: Claude Code runs these on every hook event and for MCP, so a PATH lookup hands code execution to whatever is earliest on the session's PATH
decision/compiled-classification-sources · Classification sources are compiled only — Screen signatures, harness prefixes and local-command lists come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/cryptographic-pin-hash · Pin hash is cryptographic — The pinned binary's hash is a truncated SHA-256, never a non-cryptographic hash.
decision/dialog-answers-need-verify-stamp · Dialog answers need a verify stamp — A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes. · why: a stamp is what switches on dialog answering
decision/e2e-home-keepers-linked-base · e2e-home keepers follow only a safe linked base — The two target/e2e-home keepers never follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700 and remove nothing.
decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/rehash-pinned-binary · Re-hash a pinned binary before reuse — An existing pinned viola binary is never reused without re-hashing it against the hash in its directory name.
decision/rewrite-plugin-files · Plugin files always rewritten — Existing plugin files are never reused without rewriting them.
decision/stamps-written-only-by-verify · Only viola verify writes the stamps file — No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600. · why: a stamp is what switches on dialog answering, PermissionRequest allow included
decision/version-gate-strict-stamps-read · Version gate reads stamps through the strict-modes check — A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check, unlocked and capped; a refusal, an unreadable or a malformed file is cli_verified:false.
decision/vt100-inside-catch-unwind · vt100 only inside catch_unwind — vt100 is never fed outside catch_unwind. · why: a parser panic must not take down the human's terminal path
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/instance-path-from-validated-name · Instance paths only from validated names — An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
decision/snapshot-written-only-by-wrapper · Only the instance's wrapper writes its snapshot — No process other than the instance's own wrapper writes instances/<name>/snapshot.json. · why: the endpoint in it decides where clients connect
decision/viola-ignores-e2e-home-backing · viola never touches the e2e-home backing — The viola binary itself never reads or writes the target/e2e-home backing by that name.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/e2e-home-link-guard · E2E home linked base guard — The e2e home keepers follow a linked base only when its target is absolute and reads as a real directory with no group or other bit, refuse a link to a link or a backing below the 9 GiB headroom floor with a fixed message naming no path, create the target 0700 and remove nothing. · why: a ratified boundary widening: on the dev host the path links outside the working directory
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/stamps-strict-check-first · Stamps touched only after strict-modes check — viola verify reads or writes the ledger stamps only after the strict-modes check passes, and a refusal reads and writes nothing.
decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix.
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check

# security-plan · its absences, names and overviews, whole

## security-plan-viola · (above the first heading)
overview/security-plan-viola — viola is a local CLI binary with a local IPC channel, a stdio MCP server and a Claude Code hook contract, with no public network surface. Its IPC, filesystem and PTY controls stand in for the auth, database and API concerns of a web stack.

## threat-model-summary · ## Threat Model Summary
absence/hosting — Hosting or containerised deployment of any kind
absence/model-api-calls — Model API calls of its own; the subscription credentials belong to the unmodified claude CLI
absence/outbound-network-calls — Outbound network calls made by viola itself
absence/public-network-listener — A public network listener or public internet API
absence/regulated-personal-data — Payment, health, children's or multi-user personal data of any kind
absence/release-and-deploy — A release, signing or deploy stage with release artefacts
absence/release-stage — A release, signing or deploy stage in v1
absence/remote-access — Remote or phone access to viola
absence/secrets — Secrets of any kind in v1
absence/third-party-personal-data — Collection of personal data from third parties
absence/websocket — WebSocket or polling endpoints

## authentication-authorization · ## Authentication & Authorization
absence/accounts-and-passwords — User accounts and passwords
absence/user-accounts-passwords — User accounts with passwords

## input-validation · ## Input Validation
absence/database — A database or query language
absence/file-upload — A file upload surface
absence/webhook — A webhook surface
overview/input-validation — Viola validates untrusted data at every point where it enters. Two validation regimes apply: tolerant parsing for external payloads and versioned-tolerant parsing for viola's own formats, with the security floors holding in both. Shared validators live in one core crate, so the CLI, the MCP server and the wrapper all call the same function.

## data-protection · ## Data Protection
absence/key-management — Keys at rest and their management

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
absence/stored-secrets — Stored secrets in v1

## secret-management · ## Secret Management
absence/secret-manager — A secret manager, and so any secret access auditing

## error-handling · ## Error Handling
absence/external-error-reporting — An external error reporting integration

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
absence/user-accounts — User accounts or login endpoints
