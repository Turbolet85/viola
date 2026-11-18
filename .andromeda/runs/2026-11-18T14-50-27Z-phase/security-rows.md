# security-plan · the rows that bind 2026-11-18-browser-tools-prompt-modal — picked 121 of 614 (91 by every draw) · rule any · pin 2742

## threat-model-summary · ## Threat Model Summary
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/nothing-written-under-claude-home · Nothing written under the Claude home — viola writes nothing under ~/.claude. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
decision/parent-identity-env-stripped · Parent identity variables stripped from the child — The messaging token, socket and other CLAUDE parent-identity variables are stripped from the child's environment, and viola never stores a credential.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/state-tree-strict-modes-before-use · State tree strict-modes check before first use — run, revive's start and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals. · why: Ratified live rulings, each given after the widening was shown.
decision/verify-never-answers-dialogs · Verify never answers dialogs — viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key.
decision/verify-screen-read-against-literals · Verify screen read only against compiled literals — verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures. · except: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
decision/verify-signature-only · Verify reads signatures only — viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures. · except: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
decision/verify-writes-no-dialog-byte · Verify never types into a dialog — No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output. · why: ratified
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/flat-file-state-store · Flat-file state store — State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/internal-error-output/json · catch-site internal error output · --json — Where the command prints at the catch site, --json before any -- gives {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, in place of the human line error: internal error.
contract/wrapper-channel-methods/send · wrapper channel methods · send — Types a prompt into the driven Claude session.

## authentication-authorization · ## Authentication & Authorization
decision/explicit-mode-before-write · Mode set explicitly before any byte — A written file's mode is set explicitly before any byte is written, never relying on the umask or a mode carried over from the replaced file; if the mode cannot be set, the write is discarded. · except: The trust record writer replaces Claude's config file keeping that file's own mode.
decision/home-owner-only · Owner-only viola home — The viola home is owner-only (0700 directories, 0600 files, the pinned viola 0700), every replaced file getting its mode before any byte is written, never relying on the umask.
decision/live-round-rig-keys-own-pty · Live-round rig keys only into its own pty — The founder's word anew per chunk binds only the live rig's closed argv grant: at most two down-arrows and Enter only on a read trust row, one x and one DEL in a ready box. Its stop, Ctrl-C into its own pty or a no-key group kill, needs no word; other keys write nothing; viola types into no dialog. · why: A boundary widening bounded as the founder bounded it; a rig is no viola, so the product's own pty seam stays unchanged.
decision/one-replace-helper · One shared helper replaces files — Every replaced file goes through one shared helper that writes a private temp file in the same directory, syncs it and persists it into place.
decision/private-file-modes · Owner-only modes for the viola home — The home (and any --home) is created 0700 and every file 0600, never group- or world-writable. · except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
decision/unverified-cli-gate · Unverified CLI gate — No non-null answer decision is sent without a viola verify stamp for the child's CLI version. · except: StopFailure is registered with no capability ledger row and no verify probe, owned by the 2.1.325 recorded fixture set.
contract/strict-modes-refusal/verify · strict-modes refusal · viola verify — Exit 1 with the home unable:/hint: pair, no step line and no summary; a refused stamps update reads and writes nothing.
contract/strict-modes-refusal/version-gate · strict-modes refusal · viola run's version gate — A refused stamps read is cli_verified:false with parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}.
decision/unstattable-home-refused · An unstattable home is refused — An unstattable home (a path under a regular file) is a strict-modes refusal.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/child-output-bounded · Child output bounded and tolerant — Child process output is read under MAX_FRAME and parsed tolerantly, and a failed claude agents read fails open.
decision/claude-resolved-from-path · claude resolved from PATH — The claude executable is resolved from the calling process's PATH and its output is treated as untrusted and bounded. · why: a same-user trust and accepted risk
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/input-box-readiness-from-literals · Input box readiness read from compiled literals — The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a modal literal stands; one predicate decides for verify and for viola run's pre-send gate. · why: A boundary widening that was ratified.
decision/live-rig-prompt-capture-closed · Live rig's prompt capture keeps closed words only — The live rig's own UserPromptSubmit capture reads the hook payload under MAX_FRAME with the default JSON depth and writes one owner-only line of key names matching [a-z_]{1,40}, others counted, and one closed source word, never a value; a non-object or over-long payload writes unread. · why: A ratified boundary widening.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/refusals-hold-no-paths-pids-ids · Refusals hold no paths, pids or ids — No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
decision/stamps-checked-under-lock · Stamps checked before and under lock — The ledger stamps file is checked before the ledger directory is created and again under its lock.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
decision/verify-home-tree-check · verify checks the home tree first — viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
decision/verify-pty-output-caught · verify PTY output bounded and caught — verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.
decision/verify-reads-only-signatures · Verify reads PTY output by signatures — verify's PTY output is read only against the compiled signature literals.
decision/verify-stamp-first-screen-closed-words · Verify stamp's first screen in closed words — The verify stamp records each run's first screen only as closed words, whole-number settle walls and closed fact rows of literal ids and row families, never screen text. · guard: merge_stamp_writes_the_six_first_screens_their_settles_and_facts
decision/vt100-feed-bounded-queue · Screen feed bounded queue — Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
decision/vt100-feed-isolated · Screen model feed isolated from passthrough — PTY bytes reach the human first; the screen model is fed under catch_unwind through a bounded queue, a panic or dropped copy poisons it to input-not-ready, and a full queue never blocks passthrough.
decision/vt100-feed-under-panic-catch · Screen feed under a panic catch — The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues. · why: vt100 panics are reachable at real small sizes. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/claude-agents-read-fail-open · claude agents read bounded, fail open — The claude agents --json child is a direct spawn under the R8 strip, stdout capped and killed at 5 s, parsed tolerantly into rows or closed oversize or malformed; a failed read leaves list's board unchanged and passes revive.
decision/config-cannot-disable-controls · Configuration cannot disable controls — No configuration value makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep has no effect.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
contract/config-json/claude-env-keep · config.json · claude_env_keep — An array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$; a bad value is rejected whole and reported as parse-rejected; an identity-floor name in it has no effect.

## data-protection · ## Data Protection
decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
decision/live-session-screens-kept-local · Live session screens kept local — A live session's screen frames are kept only under a gitignored local path, never in a work dir, a run dir or a commit; what lands is a closed-word summary per frame. · why: A real session's screen holds the user's own status-line output and the repository is public.
decision/no-writes-under-claude-home · viola writes nothing under the Claude home — viola itself writes nothing under ~/.claude or into ~/.claude.json and spawns no claude for plugin install; the user runs the printed claude plugin commands. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
decision/plaintext-state-os-permissions · Plaintext state protected by OS permissions — Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files. · except: The pinned bin/<version>-<hash>/viola copy is 0700.
decision/probe-artifacts-cleanup · Probe artifacts owner-only and removed — Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing. · except: a kill or a crash leaves them, and verify's own trust record in the Claude config, to be removed by hand
decision/screen-fixture-signature-rows-only · Screen fixtures keep only signature rows — A screen fixture keeps only signature rows (a row holding a compiled footer or modal literal whole, a box rule row as its rule chars alone, a prompt row as the prompt char alone), the rest blanked; another box row, a kept row the scrub would rewrite or a username split across a row seam is refused. · except: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
decision/state-files-os-permissions · State files protected by OS permissions — State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
decision/verify-never-answers-import · Verify needs a pre-trusted folder — verify against the real claude runs from a user-trusted folder whose external CLAUDE.md import is already answered; verify never answers it.
decision/verify-never-answers-import-dialog · Real-CLI verify needs a prepared trusted folder — viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog.
decision/verify-probe-captures-transient · Verify probe captures are transient — viola verify's content-bearing probe captures are 0600 files in a 0700 per-pid probe directory, and only a scrubbed --record copy leaves it.
decision/verify-probe-dirs-cleanup · Verify probe directories removed by drop guards — Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists. · except: SIGKILL, a crash or a failed handler install leave the directories, and verify's own trust record in the Claude config, to be removed by hand.
decision/verify-probes-confined-to-own-dir · Verify probe effects stay in their own directory — Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.
decision/viola-writes-nothing-in-claude-dir · viola writes nothing in Claude's config — viola itself writes nothing under the user's Claude Code config directory or its global config file. · except: verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.
decision/live-plugin-install-round · Live plugin install round — A live plugin install round runs the CLI's four plugin commands at local scope outside the work tree, passing folder trust by keys into its own pty only, never a desktop key. · why: accepted residual
decision/no-keys-at-rest · No keys at rest — Key management is omitted for the Minimal tier: there are no keys at rest.
decision/plugin-install-live-round-rig · Live plugin-install rounds isolated — A live round of viola plugin install runs the CLI's commands at local scope from a project that is its own git repo outside the work tree, passing the trust dialog only by keys into its own pty, never a desktop key.
contract/verify-refusal-codes/strict-modes-failed · verify's refusal codes · strict-modes-failed — One of the closed codes verify writes on its process-exit when refused.
contract/verify-refusal-codes/version-unreadable · verify's refusal codes · version-unreadable — One of the closed codes verify writes on its process-exit when refused.

## dependency-security · ## Dependency Security
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/no-paths-in-error-output · No paths or type names in error output — Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data. · why: the OS username is the only PII in scope
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/code-files-rewritten-at-start · Code-bearing files rewritten at start — Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
decision/never-log-claude-values · CLAUDE values never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design

## secret-management · ## Secret Management
decision/claude-credentials-stripped-from-child · Inherited CLAUDE credentials stripped from the child — The inherited CLAUDE* credentials stay only in the viola run wrapper's environment, are removed from the child by an exact-name strip that always strips the identity floor, and are never serialised into events, snapshots, diagnostics or the ledger.
decision/credential-strip-by-name-only · Credential strip reads names only — The environment strip handles variable names only and never reads a variable's value. · except: viola verify and the start of run and revive read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/hermetic-child-environment · Hermetic child environment in the pre-push gate — The local pre-push gate launches every child under env -i with exactly HOME and PATH, so no host environment value crosses into a child. · except: Any assignment whose value comes from the host is a boundary widening that needs its own Decisions Log entry.
decision/inherited-credentials-never-serialised · Inherited credentials never serialised — Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
decision/live-rig-credentials-copy · Live rig credentials copy transient and unrecorded — The live rig copies the claude OAuth credentials file owner-only into its private scratch, reads only its expiry, refuses a start with under 20 minutes of token, removes the copy on every exit path, and writes no credential, host path or config value into any ledger, frame or summary. · guard: rig_leaves_no_credentials_copy_on_any_exit_path; rig_keeps_no_credential_or_host_path_in_any_record
decision/live-rig-isolated-child · Live rig child isolated from the user's config — The live rig runs its child under a private CLAUDE_CONFIG_DIR with an environment from a closed kept list of host names plus its own, and reads the user's ~/.claude.json only after a moved stamp, searching it for rig-made needles, dropping the bytes and keeping one closed word. · guard: rig_reads_the_user_config_before_and_after
decision/live-rig-seed-from-bundle-logic · Live rig's child config seeded from bundle logic only — The live rig seeds its child's private config with no value read from the user's config: its used-config markers and the browser-tools prompt turned off come from the CLI bundle's own logic.
decision/r8-claude-env-strip · R8 CLAUDE environment strip — Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values. · why: Ratified. · except: verify and run and revive's start read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps the name, to choose the Claude config holding the trust record; else the user HOME's .claude.json.
decision/env-not-config-channel · Environment variables are not configuration — Environment variables are never a configuration channel for the product. · except: viola verify and the start of run and revive read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.
decision/messaging-credentials-are-secret · Messaging token and socket are secrets — The inherited CLAUDE_CODE_MESSAGING_TOKEN and CLAUDE_CODE_MESSAGING_SOCKET count as secrets.
decision/no-uid-0-launch · No root launch — No child is ever launched as uid 0.
decision/pre-push-empty-environment-launch · Pre-push children launched with an empty environment — The pre-push gate launches every child natively with an emptied environment carrying exactly HOME (from the passwd entry, never its own $HOME) and PATH of constant dirs, so no host value crosses; it has no other launcher and no uid-0 launch. · why: Keeps inherited CLAUDE* credentials out of every child. · except: A host-valued assignment is a boundary widening needing its own Decisions Log entry.

## error-handling · ## Error Handling
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
decision/home-tree-strict-modes-before-log · Home tree checked before any log opens — viola run, revive's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/compiled-classification-sources · Classification sources are compiled only — Screen signatures, harness prefixes and local-command names come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text; a typed text is a local command only by its compiled name, alone or followed by one space and arguments.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/dialog-answers-need-verify-stamp · Dialog answers need a verify stamp — A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row passes. · why: a stamp is what switches on dialog answering
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/no-absolute-paths-in-external-errors · No absolute paths in external errors — No external error ever exposes an absolute path.
decision/owner-only-file-modes · Owner-only modes under the viola home — The viola home and everything under it are created with explicit modes, directories 0700 and files 0600, never the default umask. · except: The pinned versioned viola binary is 0700, and never group- or world-writable.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/stamps-written-only-by-verify · Only viola verify writes the stamps file — No process other than viola verify ever writes ledger/stamps.json; viola verify writes it under its lock, with 0600. · why: a stamp is what switches on dialog answering, PermissionRequest allow included
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/version-gate-strict-stamps-read · Version gate reads stamps through the strict-modes check — A hook.dialog gets a non-null decision only when viola run's version gate read ledger/stamps.json through the strict-modes check, unlocked and capped; a refusal, an unreadable or a malformed file is cli_verified:false.
decision/vt100-inside-catch-unwind · vt100 only inside catch_unwind — vt100 is never fed outside catch_unwind. · why: a parser panic must not take down the human's terminal path
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-messaging-secrets-in-outputs · No messaging secrets in any output — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and any R8-stripped CLAUDE* value are never written to any log or stderr, events.ndjson, snapshots, diagnostics/ or fixtures.
decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/stamps-strict-check-first · Stamps touched only after strict-modes check — viola verify reads or writes the ledger stamps only after the strict-modes check passes, and a refusal reads and writes nothing.
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.
contract/cli-exit-codes/12 · CLI exit codes · 12 — answer refused unverified-cli: no verify stamp covers every relied ledger row for the child's CLI version.

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
