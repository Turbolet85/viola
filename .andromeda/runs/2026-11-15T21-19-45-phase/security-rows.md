# security-plan · the rows that bind 2026-11-15-api-error-turn-end — picked 156 of 597 (100 by every draw) · rule any · pin 1610

## threat-model-summary · ## Threat Model Summary
decision/child-output-tolerant · Child output parsed tolerantly — Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
decision/child-output-tolerant-unknown · Child output parsing fails to unknown — Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
decision/data-classes · Data classes — viola's data falls into user content logged and sent over the channel, integrity-sensitive config that decides what code runs, a transient environment credential never stored by viola, and operational metadata whose only PII is the OS username in paths.
decision/hook-fails-open · Hook fails open to the human — Any failure in viola hook, a failing security check included, fails open to the human: exit 0 with no body, nothing on stderr. · why: So the dialog renders for the human.
decision/hook-payload-is-content · Hook payload is content — Hook stdin payloads from Claude Code are parsed tolerantly and their upstream text is treated as content, never as commands.
decision/hook-strict-check-first · Hook strict-modes check before any read — Every hook path runs the instance's strict-modes check before anything under the home is read or opened; a refused or absent instance exits 0 with no config read, no stdin read, no line and no frame.
decision/mutation-testing-at-audit · Mutation testing at the epoch audit — No CI job runs mutation testing; it belongs to the epoch-boundary code audit.
decision/readers-tolerate-torn-lines · Readers tolerate torn lines — Readers across processes of state under the viola home tolerate torn lines: they skip and count them, and the next append heals a torn last line.
decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
decision/socket-in-owner-only-dir · Socket in a per-user owner-only directory — The endpoint socket sits in a per-user 0700 directory, never in world-writable tmp, and a socket directory another user owns, a loosened one or a symlink is refused with no bind and no frame.
decision/socket-in-per-user-dir · Sockets in a per-user directory — Each instance's Unix domain socket stands in a verified per-user 0700 directory under the XDG runtime directory or a per-uid folder in /tmp, never the /tmp root.
decision/sync-crate-tokio-ban · No tokio in sync crates — tokio is banned from the sync crates.
decision/sync-crates-tokio-free · Sync crates free of tokio — A tokio ban holds per sync crate, and the channel crate's Tokio client sits behind its tokio feature. · guard: deny-sync.toml; scripts/channel-tokio-free.sh
decision/tolerant-hook-parsing · Tolerant hook payload parsing — Hook payloads are parsed tolerantly, with no rejection of unknown fields, and drift is reported with its path.
decision/upstream-text-is-content · Upstream text is content, never commands — Upstream text is treated as content, never as commands; viola does not interpret it, and a permission decision rests with the driver model outside viola. · why: mechanism, not policy
contract/mcp-tools/wait · MCP tools · wait — A tool called by the driver model that reads the driven session's output.
contract/wrapper-channel-methods/hook-event · wrapper channel methods · hook.event — A notification from the hook to the wrapper.
contract/wrapper-channel-methods/send · wrapper channel methods · send — Types a prompt into the driven Claude session.
contract/wrapper-channel-methods/wait · wrapper channel methods · wait — Reads the driven session's conversation.
decision/client-reads-closed-values-only · Client screens the server's answer to closed values — A client takes a refusal or a string detail from the server only as a code word and a fault only through the sanitised fault, so nothing of the peer's but closed values reaches a CLI JSON document or an MCP result. · except: a code word outside the closed list stays visible by design
decision/flat-file-state-store · Flat-file state store — State is embedded flat files, ndjson append logs plus atomically replaced JSON snapshots, locked through sibling lock files under the viola home, with no database.
decision/no-abstract-namespace · No Linux abstract namespace — The Linux abstract namespace is never used for IPC sockets.
decision/pinned-artefacts-never-deleted · Pinned binary and plugin folders kept — The pinned binary and the plugin folders are never deleted in v1.
decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
decision/replayed-snapshot-yields-nothing · Replayed snapshot runs nothing and frames nothing — A snapshot replayed from the log carries no endpoint, pid or start time, so it yields no frame.
decision/strict-modes-at-start · Strict-modes check at start — Beyond the content hash in the folder name, the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode) before any log opens.
decision/verify-children-direct-spawn · Verify children spawned directly — viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals. · why: Ratified live rulings, each given after the widening was shown.
decision/verify-never-answers-dialogs · Verify never answers dialogs — viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key.
decision/verify-screen-read-against-literals · Verify screen read only against compiled literals — verify's children's screens are read only against compiled signature literals, and their row text leaves the process only as signature-only record fixtures.
decision/verify-signature-only · Verify reads signatures only — viola verify's interactive children are read only against the compiled signature literals, their row text leaving the process only as signature-only record fixtures.
decision/verify-writes-no-dialog-byte · Verify never types into a dialog — No verify run writes a byte into any dialog, and a start showing a modal is killed with no key; probe dialogs are answered only by the probe hook's output. · why: ratified
contract/cli-state-root/home-flag · CLI state root selection · --home — Chooses which state tree, snapshot, endpoint and ledger stamps are used.
contract/cli-state-root/viola-dir · CLI state root selection · VIOLA_DIR — Chooses which state tree, snapshot, endpoint and ledger stamps are used.

## authentication-authorization · ## Authentication & Authorization
decision/client-peer-euid-check · Client-side peer euid check — Past the connect, the client admits only a server of its own euid and refuses an unreadable peer, sending no frame.
decision/client-seam-check-order · Check order at every client seam — On every client seam the order is the socket-directory check, the connect, the peer euid decision, then the server verification against the snapshot.
decision/client-strict-modes-first · Client strict-modes check first — Before the first frame of any method, the client runs the instance's strict-modes check, a refusal writing no frame and no line.
decision/client-verifies-server · Client verifies the server — The client verifies the connected peer's pid and that process's start time against its one snapshot read; a mismatch, a replayed snapshot or a check that cannot be made sends no frame and never falls back to a default connect.
decision/client-verifies-server-before-frame · Client verifies the server before any frame — Before the first frame of any method, notifications included, every client checks the peer's euid equals its own and the peer pid and its OS start time match the snapshot's pid and started_at; a check that cannot be made is a refusal. · why: hook.event notifications carry prompt text, last_assistant_message and tool data, not only send, answer and hook.dialog
decision/socket-dir-checked · Socket directory checked — The socket's per-user 0700 directory is checked for owner, mode and not being a symlink before bind and before any client connect; a failed check refuses run, reports instance-unreachable to clients and fails hook open.
decision/strict-modes-before-read · Strict-modes check before trusted reads — Every process that reads a trusted file refuses, before its first read or open under the home, a trusted path another user can write, a home, instance directory or diagnostics folder with any group or other bit, or a symlinked diagnostics folder or file.
decision/strict-modes-before-snapshot-read · Strict-modes check before the snapshot is read — Every client runs the instance's strict-modes check before it reads the snapshot, and only a parsed snapshot gives reference values for server verification. · why: the snapshot's endpoint, pid and started_at are the reference values, so a client that skips the check verifies the server against values an attacker could have written
decision/strict-refusal-leaves-no-trace · A strict-modes refusal leaves no trace — A strict-modes refusal opens no log and writes no line, leaving the home's mode as found, the check running before any log opens. · why: ratified; opening a log sets an existing home to 0700, so a check made after it passes a widened home
contract/server-verify-refusal/hook · server-verification refusal · viola hook — Fails open: exit 0, no body, its hook-decision line carrying server-verify-failed.
contract/socket-dir-refusal/hook · socket-directory refusal · viola hook — Fails open: exit 0, empty streams, one hook-decision line of detail socket-dir-refused at WARN, no frame written.
contract/strict-modes-refusal/hook · strict-modes refusal · viola hook — Exits 0 at once with no body and no trace: no config.json read, no role file, no stdin read, no line, no frame, the home's mode left as found.
decision/checks-at-seams-not-channel-client · Security checks sit at the seams, not the channel client — No check sits inside the channel crate's client connect; the checks sit at the seams, and a test may write through that client past every check by design. · why: viola-channel has no viola-state edge
decision/one-socket-dir-check-for-bind-and-clients · One socket-directory check for bind and clients — The wrapper's bind and the three client seams share one check on the endpoint's parent directory before they bind or connect. The three seams are the CLI frame verbs, the hooks and the MCP frame tools. · why: Both sides of the socket read the same verdict, so a directory refused to a client is refused to a start, and the socket-dir-refused detail means one thing.
decision/only-matches-verdict-lets-a-frame-out · Only a Matches server verdict lets a frame out — Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a client write a frame; a mismatched pid or start time, or an unverifiable server, is refused before any frame. · why: The CLI verbs, the hooks and the MCP frame tools rely on it to reach only the recorded wrapper; server-verify-failed names the refusal.
decision/socket-directory-check · Socket-directory check before bind and connect — Before bind and again before any client connect, the socket directory is lstat'ed and must be owned by the effective user, grant no group or other bits, and not be a symlink. · guard: security_negatives_socket_dir
decision/strict-modes-check · Strict-modes check on trusted state — Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission. · guard: security_negatives_strict_modes, chaos_snapshot_replay
decision/strict-modes-read-rule · Strict-modes read rule — The home, instances/<name>/ and its diagnostics/ are also refused when they grant any group or other permission. · why: so a pre-existing home cannot expose events.ndjson or diagnostics/
decision/unreadable-peer-euid-is-refused · Unreadable peer euid refused, read silently — A peer whose euid cannot be read is refused, and reading a connected stream's peer euid or pid writes and logs nothing. · why: Server admission and the client server check both rest on it, and the negative tests expect no pid or uid in any output or line.
decision/unstattable-home-refused · An unstattable home is refused — An unstattable home (a path under a regular file) is a strict-modes refusal.
contract/ipc-client-refusals/server-verify-failed · IPC client refusals · server-verify-failed — Reported when the connected peer is not of the client's euid or cannot be read; no frame is sent.
contract/ipc-client-refusals/socket-dir-refused · IPC client refusals · socket-dir-refused — Reported when the recorded endpoint's socket directory fails its check before the connect; no frame is sent.

## input-validation · ## Input Validation
decision/bounded-readers-census · Every product reader bounded or named — Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable for the pinned copy are not external input.
decision/channel-closed-types · Channel closed types — Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
decision/events-reader-bounded-lines · Event log reader bounded lines — The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
decision/home-strict-modes-first · Home strict-modes check first — The home is resolved, canonicalised and passed through the strict-modes check before any state file is read, and a refusal reads and writes nothing.
decision/hook-instance-check-first · Hook instance check before anything — Hook event and dialog paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
decision/hook-security-fields-closed · Hook security fields in closed enums — Hook payloads are parsed tolerantly as one JSON object, but the fields that decide security map into closed enums.
decision/hook-stdin-never-truncated · Hook stdin over-limit detected — Hook stdin is capped at MAX_FRAME and an over-limit payload is detected, never silently truncated.
decision/hook-unknown-event-silent-exit · Hook unknown event exits silently — The hook event argument maps into a closed set of nine events and no other word; an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing read or written anywhere.
decision/hooks-fail-open · Hooks fail open — Every hook failure fails open: exit 0 with no body.
decision/invalid-hook-frames-dropped · Invalid hook frames dropped — An invalid hook.dialog frame is not registered and an invalid hook.event frame is not appended.
decision/max-frame-cap · One external read cap — MAX_FRAME is 16 MiB, one constant in the core crate, and caps every external read. · except: viola's reads of its own executable are no external input
decision/max-frame-shared-cap · One shared MAX_FRAME cap — One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps. · why: Sized above the largest expected permission input (Write-tool file bodies).
decision/shared-validators-in-core · Shared validators in the core crate — Input validators live in viola-core and every surface (CLI, MCP, wrapper) calls the same one. · why: so the CLI, MCP and the wrapper apply one identical check · except: The Hyprland seam's window Address validator lives in the seam crate.
decision/state-readers-never-write · State file readers never write — A reader of viola's own state files never writes, caps each ndjson line at MAX_FRAME and counts an over-long, non-object or unterminated line as torn.
decision/state-reads-after-strict-modes · State files read after strict modes — Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
decision/stop-background-tasks-count-only · Stop background tasks carried as a count only — Stop's background_tasks is carried only as its length, no element read; null when absent, null or not an array, and a non-array also writes a drift entry. · why: Ratified with the option-text widening.
decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
contract/channel-frames/hook-event-revalidation · channel frames · hook.event — kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; an invalid event is not appended.
contract/hook-stdin/not-one-object · hook stdin · malformed-json — A payload that is not one JSON object fails open with malformed-json.
contract/hook-stdin/oversize · hook stdin · oversize payload — A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.
decision/channel-frame-cap-before-read · Channel frame cap before reading — A channel frame is capped at MAX_FRAME before the line is read, and an over-cap frame closes the connection.
decision/channel-json-default-depth-limit · Channel JSON default depth limit — Channel frames are parsed with serde_json's default recursion limit of 128, never unbounded depth.
decision/classified-snapshot-read-only · Classified snapshot read only — No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
decision/client-reply-code-words-only · Client reads reply refusals as code words only — A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
decision/home-strict-modes-before-read · Home strict-modes check before any read — --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.
decision/hook-security-fields-closed-enums · Hook security fields in closed enums — Hook parsing is tolerant, but the fields that decide security (dialog kind and decision body) map into closed enums; a malformed dialog payload fails open with exit 0 and no body, and stdout takes one write.
decision/parser-fuzz-scope · Parser surfaces in fuzz scope — Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
decision/replayed-snapshot-fallbacks · Replayed snapshot fallbacks — Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, and the collision check judges liveness by the heartbeat alone.

## data-protection · ## Data Protection
decision/code-bearing-files-rewritten-each-start · Code-bearing files rewritten at every start — The plugin files are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy. · why: A code-bearing file written at every start and handed to the CLI is a ratified boundary widening.
decision/diagnostics-open-fails-to-sink · A refused diagnostics open writes nowhere — Diagnostics opens run the strict-modes check on the existing diagnostics folder and file before narrowing anything; a refusal is a failed open, the writer goes to a sink and the detail line is dropped.
decision/drift-reports-diagnostics-only · Drift reports only in diagnostics — Deserialisation drift reports are written only to the diagnostics directory. · why: drift reports can quote tool input
decision/exec-commands-absolute-pinned-path · Exec commands use an absolute path — Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive and verify, and the installing exe's canonical path in plugin install's `.mcp.json`.
decision/logs-owner-only · Logs owner-only — The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
decision/marketplace-written-only-by-install · Marketplace tree written only by plugin install — The marketplace tree is written only by viola plugin install, each file rewritten whole at 0600 and each dir 0700, with no hooks in any document and any other entry left as found; viola never reads the tree back.
decision/plugin-files-rewritten-each-start · Plugin files rewritten every start — The plugin files are rewritten at every start.
decision/plaintext-state-os-permissions · Plaintext state protected by OS permissions — Files under the viola home are plaintext, protected by OS permissions only: 0700 directories and 0600 files. · except: The pinned bin/<version>-<hash>/viola copy is 0700.
decision/probe-artifacts-cleanup · Probe artifacts owner-only and removed — Probe captures and dirs are owner-only and removed when verify passes, fails or is stopped, each run removing only what it created, never by pattern or listing. · except: a kill or a crash leaves them to be removed by hand
decision/probe-prompts-synthetic · Probe prompts are synthetic — Probe prompts used to record committed fixtures must be synthetic.
decision/state-files-os-permissions · State files protected by OS permissions — State files are plaintext by design, protected only by OS permissions: 0700 directories, 0600 files and the pinned binary 0700.
decision/verify-never-answers-import · Verify needs a pre-trusted folder — verify against the real claude runs from a user-trusted folder whose external CLAUDE.md import is already answered; verify never answers it.
decision/verify-never-answers-import-dialog · Real-CLI verify needs a prepared trusted folder — viola verify against the real claude runs from a folder the user trusts whose external CLAUDE.md import is already answered; verify never answers that dialog.
decision/verify-probe-dirs-cleanup · Verify probe directories removed by drop guards — Every probe directory of viola verify is 0700 and removed when verify passes, fails or is stopped by SIGINT or SIGTERM; a guard removes only what its own run created, never by a pattern or a listing, and no remover of an earlier run's directories exists. · except: SIGKILL, a crash or a failed handler install leave the directories to be removed by hand.
decision/verify-probes-confined-to-own-dir · Verify probe effects stay in their own directory — Verify's dialog probe's one allowed command and the plan probe's plan file land inside the probe's own directory, never under ~/.claude/plans.

## dependency-security · ## Dependency Security
decision/no-tokio-in-sync-crates · No tokio in the sync crates — tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root. · guard: deny-sync.toml; channel-tokio-free.sh; deny-probes.sh · except: viola-channel's Tokio client sits behind its tokio feature; its graph without the feature is checked to hold no tokio by the cargo tree form instead.
decision/serde-json-bounded-depth · serde_json recursion limit kept — cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit. · guard: scripts/deny-probes.sh
decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned in cargo-deny. · why: Every parse of external bytes keeps serde_json's default recursion limit. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml; deny-probes row serde_json-unbounded-depth

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases
decision/bounded-external-readers · Bounded external readers — Every external reader is wrapped by a size bound at a maximum frame size.
decision/code-files-rewritten-at-start · Code-bearing files rewritten at start — Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
decision/diagnostics-owner-only · Diagnostics files owner-only — Files under the instance diagnostics are mode 0600.
decision/error-display-no-upstream-text · Error messages carry no upstream text — The Display output of the product error enums never interpolates upstream text or absolute paths.
decision/serde-errors-mapped-to-fixed-message · Upstream-quoting errors mapped to fixed messages — An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics. · why: serde errors quote the offending value
decision/user-content-only-to-diagnostics · User content only in diagnostics — Outside the content-carrying contract payloads, user content, tool input and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data. · except: events.ndjson, wait and last results and hook.dialog carry content by design
decision/local-boundary-no-auth-library · Local boundary instead of an auth library — Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
decision/owner-only-home-and-pinned-binary · Owner-only home and pinned binary re-hash — The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
contract/mcp-structured-content/codes · MCP structuredContent · refusal, detail and error codes — structuredContent carries the channel ok payload on success, and only refusal, detail and error codes on a refusal, an unreachable instance, a wrapper fault or a refused input.

## bootstrap-phases · ## Bootstrap phases (derive for route / setup-project) · twin bootstrap-phases-derive-for-route-setup-project
decision/client-checks-before-frame · Client checks before any frame — Every client reads the instance snapshot only after the instance strict-modes check, then makes the liveness pre-check, the socket-directory check on the recorded endpoint's parent, the connect, the peer euid decision and the server check (pid and start time against the snapshot).

## secret-management · ## Secret Management
decision/no-credential-literals · No credential literals in code — No token, key or credential literal appears in source or in the embedded plugin files, which carry only exec paths.
decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.

## error-handling · ## Error Handling
decision/full-error-detail-to-diagnostics · Full error detail only in diagnostics — Full error detail goes only to the instance's 0600 diagnostics files or, for short-lived CLI verbs, to stderr.
decision/hook-refusals-write-no-frame · A refused hook writes no frame — A hook event or dialog refused by the server check, the socket-directory check or a replayed or absent snapshot writes no frame and exits 0 with empty streams; a refused SessionEnd is still appended directly.
decision/hook-silent-exit-zero · hook is silent and always exits 0 — The hook verb and its hidden capture arm never write to stderr and always exit 0; a failing security check ends in exit 0 with no body. · why: So the dialog still renders for the human.
decision/hook-strict-modes-first · hook checks the instance before anything — The hook event and dialog paths run the instance strict-modes check first; a refused or absent instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append. · except: The capture arm runs ahead of that check.
decision/sanitised-external-errors · Sanitised external error responses — Errors returned to channel clients, CLI --json and MCP drivers carry no anyhow context chains, absolute paths, upstream text, serde_path_to_error paths or values, or internal type names.
decision/stderr-never-log-floor · stderr honours the never-log floor — stderr never carries upstream text, tool input, drift-report values or any other value on the NEVER-log floor.
contract/hook-decision-detail/channel-unreachable · hook-decision detail · channel-unreachable — hook fails open with this detail when the channel cannot be reached, a replayed or absent snapshot included.
contract/hook-decision-detail/malformed-json · hook-decision detail · malformed-json — hook fails open with this detail when stdin is not one JSON object.
contract/hook-decision-detail/oversize-stdin · hook-decision detail · oversize-stdin — hook fails open with this detail when stdin exceeds its cap.
contract/hook-decision-detail/server-verify-failed · hook-decision detail · server-verify-failed — hook fails open with this detail when the peer euid or server verification fails; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
contract/hook-decision-detail/socket-dir-refused · hook-decision detail · socket-dir-refused — hook fails open with this detail, in one line at WARN, when the socket directory is refused; on the dialog path after hook-invoked{corr:null}, with decision_emitted:false.
decision/diagnostics-strict-modes-fail-open · Diagnostics files failing strict modes fail open — A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — Product error types carry fixed Display messages with no path or payload interpolated. · guard: thiserror_messages_interpolate_nothing
decision/fixed-error-display-messages · Fixed error display messages — Every product error type's Display uses fixed messages, and variant fields holding paths or payloads are excluded from Display.
contract/cli-json-error/internal-error · CLI --json error documents · internal-error — Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, mcp and hook stay silent there.

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns
decision/absolute-viola-path-in-commands · Hook and MCP commands use an absolute viola path — Every exec-form command in the embedded hooks.json or .mcp.json and in plugin install's .mcp.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH. · why: Claude Code runs these on every hook event and for MCP, so a PATH lookup hands code execution to whatever is earliest on the session's PATH
decision/closed-types-for-protocol-fields · Closed types for protocol fields — behavior, dialog_id, the wheel holder and the dialog kind are closed enums or integers, never free strings, even under tolerant parsing.
decision/compiled-classification-sources · Classification sources are compiled only — Screen signatures, harness prefixes and local-command lists come only from compiled viola-agent-claude ledger rows, never from runtime or upstream text.
decision/escape-terminal-controls · Escape C0/C1 controls in human output — wait/last human-mode output and list rows taken from claude agents --json are never printed to a terminal without escaping C0/C1 controls. · why: the same class as RUSTSEC-2025-0055
decision/hook-silent-and-exit-zero · viola hook writes no stderr and exits zero — viola hook never writes to stderr and never exits non-zero, including on security-check failures.
decision/instance-path-from-validated-name · Instance paths only from validated names — An instance directory is joined only with a string that came out of ViolaName validation, including MCP target, link and unlink params and notify events.
decision/new-env-read-needs-log-entry · New seam or environment read needs a log entry — No further test-only seam or environment read is added without its own Decisions Log entry.
decision/no-parser-error-chains-outward · No parser error chains in external output — serde_path_to_error drift reports and anyhow chains never reach MCP structuredContent or channel error.data, and no anyhow chain still holding a serde_json or serde_path_to_error source is printed to stderr. · why: their Display quotes input values such as tool input
decision/no-socket-in-tmp-root · No Unix socket at the temp root — A Unix socket is never placed directly in /tmp or at the TMPDIR root; it always lives in a verified 0700 per-user directory. · why: The Kea DHCP CVE-2025-32801/32802/32803 precedent.
decision/private-diagnostics-files · Private diagnostics files — diagnostics/ files are never created readable by other users.
decision/reads-capped-at-max-frame · Reads capped at MAX_FRAME — Every channel line read, hook stdin read, child output read and MCP stdin line is capped at MAX_FRAME.
decision/refusals-never-block-human · Security refusals never block the human — Security refusals go only to automation (send/answer), never to the human, and hooks fail open.
decision/request-size-limits · Request size limits — MAX_FRAME is enforced on the channel and on each viola mcp stdin line; request size limits are never skipped.
decision/rewrite-plugin-files · Plugin files always rewritten — Existing plugin files are never reused without rewriting them.
decision/server-side-revalidation · Wrapper re-validates every input — MCP schemas and client-side CLI checks are never trusted alone; the wrapper re-runs name validation, closed-enum deserialisation and paste-text validation.
decision/upstream-text-is-never-a-command · Upstream text is never a command (R1) — Upstream text (prompts, last_assistant_message, plan text, question and option text, tool input) is never interpreted as a viola command or config.
decision/verify-server-before-frame · Verify the server before any frame — No channel frame is written before the server is verified against a snapshot that passed the full strict-modes check (socket directory, pid, start time and euid) in the same process.
decision/controls-not-switchable · No config, env var or flag switches off a control — config.json, a VIOLA_* environment variable or a CLI flag never switches off any security control (the socket-directory check, the peer or server verification, the home strict-modes check, validate_paste_text). · why: environment variables are not a configuration channel · except: the carve-outs, the two test-only seams and the terminal colour facts, switch off nothing
decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
decision/snapshot-written-only-by-wrapper · Only the instance's wrapper writes its snapshot — No process other than the instance's own wrapper writes instances/<name>/snapshot.json. · why: the endpoint in it decides where clients connect

## security-anti-patterns · ## Security Anti-Patterns (NEVER do these) · twin security-anti-patterns-never-do-these
decision/server-verified-before-any-frame · Server verified before any channel frame — No client writes any channel frame before verifying the server's euid, pid and process start time against the instance snapshot; an unreadable euid is refused and a mismatch or an unmakeable check sends no frame.
decision/snapshot-strict-checked-before-trust · Snapshot reference values only after strict-modes check — The endpoint, pid and start time a client verifies against are taken only from a snapshot that passed the strict-modes check in the same process.
decision/strict-check-before-narrowing · Strict-modes check before any narrowing — The strict-modes check never runs after a create, a mode set or a log open that narrows the path it checks. · why: a widened directory narrowed to 0700 first would pass the check
decision/no-control-switch-off · No switch turns off a control — No config file, environment variable or CLI flag switches off the socket-directory check, the peer or server verification, the home strict-modes check or the paste text validation. · why: environment variables are not a configuration channel, and the home variable is set by a child any same-session agent can influence · except: the two fake-agent test seams for pump delay and hook panic, present only under the fake-agent feature and switching off nothing
decision/no-seam-forcing-peer-refusal · No seam to force a peer-euid refusal — No env, flag or feature seam forces a refusing peer-euid decision; the decisions are proven over injected values and each client seam's wiring stays the named weakest proof. · why: one account can make no foreign peer
decision/test-seams-fake-agent-only · Test seams only under the fake-agent feature — An environment test seam exists only under the fake-agent feature, is absent from every release build and switches off nothing; another seam or environment read needs its own Decisions Log entry.

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
