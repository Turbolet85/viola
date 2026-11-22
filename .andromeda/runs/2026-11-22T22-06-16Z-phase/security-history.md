@history_item n=1 marker=amd-psva565i
**amd-psva565i** — (2026-10-10-session-links) the wrapper's `LinkSet::restore` became a second start-time pass of `events::read_from`; `apply_link_line` uses a link line's `driver`/`driven` only when both are strings and does not re-parse them (row decision/link-restore-trusts-written-names); the replay's result holds the logged link set, so a revive over a replayed snapshot reads the links. Precedent for a new `lineage` kind read back at start and through the replay, and for "links drawn apart": the logged links are trusted only because the wrapper wrote names that passed `ViolaName::try_new`, so a display read must name-check them itself.

@history_item n=2 marker=amd-zqh3rwyd
**amd-zqh3rwyd** — (2026-10-26-explicit-session-links) `link`/`unlink` served behind the IPC controls; names through `ViolaName::try_new`, a foreign `driven` or self-pair `-32602`; neither method reads the self-reported `from` (rows contract/wrapper-channel-methods/link, /unlink, decision/all-or-nothing-channel-access). A new channel method was a boundary widening needing the founder's word — a parent declared over the channel would be the same class; a parent edge must not ride on `from`.

@history_item n=3 marker=amd-e54gcl5j
**amd-e54gcl5j** — (2026-10-26-explicit-session-links) link lines carry `data {driver, driven}` only, a repeated link / absent unlink appends nothing (contracts/events-link-lines/*); the methods write no process-log line holding a name, and `viola link`/`unlink` print only their own parsed arguments, never a name read back from a reply or the log (decision/link-names-kept-out-of-process-log). Binds how this chunk shows links (name-checked before display) and what a `lineage` line may carry.

@history_item n=4 marker=amd-akjbvlc5
**amd-akjbvlc5** — (2026-10-26-explicit-session-links) `link`/`unlink` run `check_instance` before any role file, log or snapshot read, refusal exit 21, no line. Trap recorded: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a widened home — any new reader/writer this chunk adds must check first.

@history_item n=5 marker=amd-uqpqhlss
**amd-uqpqhlss** — (2026-10-26-home-and-code-bearing-file-integrity) all eight CLI frame verbs share `refuse_untrusted`'s instance check before `client::start`; refusal exit 21 with the instance pair, nothing opened. The order a new or changed verb (a `run` parent flag resolving another instance) must follow.

@history_item n=6 marker=amd-xhxx3ana
**amd-xhxx3ana** — (2026-11-15 wrap) the served link/unlink halt confirm ratified by the founder (companions amd-kniw5rix, amd-geyhcvbq). The link surface this chunk draws stands on the founder's word; changing what a link means is his.

@history_item n=7 marker=amd-cccya7ze
**amd-cccya7ze** — (2026-10-11-the-board-viola-list) `viola list` joined the strict-modes entry points (whole-board refusal on the first failure, exit 21, no log, no line); `board::read` became a product caller of the classified read and replay after `check_instance`; `events::read_from` gained `board::log_pass`; list rows escape every control as `\xHH` via `escape_row`, every row field from viola's own state. The board the fleet trees extend; rows decision/whole-board-refusal, list-scan-checks-before-open, human-output-escapes-controls.

@history_item n=8 marker=amd-ho4nohh4
**amd-ho4nohh4** — (2026-11-01-unix-endpoint-and-home-hardening) `board::scan` holds the home to `check_read` and `instances/` to `check_path`; a 0750 home refused as `ScanRefused::Home` with or without an instance (rows decision/list-home-read-rule, board-home-read-rule). A tree view must keep the scan's check order and no-open.

@history_item n=9 marker=amd-ijqhczgw
**amd-ijqhczgw** — (2026-11-06-version-done-check) the board's read rule confirmed by the founder (was OPEN on a delegate's answer). Standing; not for this chunk to loosen.

@history_item n=10 marker=amd-zh2jq57t
**amd-zh2jq57t** — (2026-10-13 wrap) the whole-board refusal and its no-line shape ratified as the founder's. Any new list refusal for a lineage read inherits this shape.

@history_item n=11 marker=amd-okzja6o7
**amd-okzja6o7** — (2026-10-17-cli-output-discipline) `NO_COLOR`/`TERM`/`COLORTERM` admitted, read only by `list`'s human mode, compared only, deciding SGR depth around already-escaped rows; `COLUMNS`, `CLICOLOR(_FORCE)`, `CI` not read, width from `viola_pty::host_size()`; NEVER another env read without its own Decisions Log entry (rows decision/terminal-colour-facts, colour-env-only-compared, no-width-or-colour-override-env, new-env-read-needs-log-entry). "Plain when piped" must use stdout's `IsTerminal`, never a new env var.

@history_item n=12 marker=amd-5vfzlnsc
**amd-5vfzlnsc** — (2026-11-05-linux-only-code-and-ci) the colour decision is none under `--json`, a passthrough role, `NO_COLOR`, `TERM=dumb` or a non-terminal stdout. The existing non-terminal rule the piped tree form rests on.

@history_item n=13 marker=amd-d74wzrrq
**amd-d74wzrrq** — (2026-10-16-unwrapped-sessions-on-the-board) `list`/`revive` spawn `claude agents --json`, bounded and tolerantly parsed; an unwrapped row's name is never a `ViolaName` or target and is printed through `escape_row` (rows decision/unwrapped-name-never-target, unwrapped-session-name-not-target, agents-row-name-not-a-target). An unwrapped row can never be a lineage parent node or a link endpoint in the tree.

@history_item n=14 marker=amd-wrds3kue
**amd-wrds3kue** — (2026-10-17-mcp-server-for-drivers) the MCP `list` tool goes through CLI `list`'s board path (`list::board_document`); unwrapped names never targets there. A `list --json` payload change reaches the MCP list too.

@history_item n=15 marker=amd-wodlxpqi
**amd-wodlxpqi** — (2026-10-10-self-healing-state) the events reader counts `Skipped{unknown_kinds, unknown_fields, torn_lines}` unlogged; an unknown kind is not returned; a reader never writes; the next append heals a torn last line with one `state-recovered` line (rows decision/readers-tolerate-torn-lines, events-reader-bounded-lines, torn-last-line-healed-on-append, state-readers-never-write). A new `lineage` kind is unknown to older builds' readers and must parse under the same bounds.

@history_item n=16 marker=amd-b7itse67
**amd-b7itse67** — (2026-10-10-viola-revive) revive reads its own state after `check_instance`; the classified read and replay's first product caller; revive's arguments (`<name>` by the shared parser, `--id` closed shape). Where "parent kept by revive" must read the lineage line: after the check, through the replay.

@history_item n=17 marker=amd-vi2zwy7z
**amd-vi2zwy7z** — (2026-10-10-viola-revive) the recorded `cwd` used only as revive's spawn directory, ratified as a boundary widening; snapshots hold the optional `cwd` (OS-username-bearing) in the 0600 snapshot only. Precedent: a launch-time value carried into a revived start is a widening needing the founder's word, and identity-bearing values stay out of logs.

@history_item n=18 marker=amd-wcbqjrb7
**amd-wcbqjrb7** — (2026-11-18-session-folder-trusted-by-viola) decision/cwd-only-in-snapshot and decision/spawn-cwd-snapshot-only now allow the cwd as the trust-record key in the Claude config, still never on a log line, error body or stdout. A parent's identity written to events.ndjson must hold no such path.

@history_item n=19 marker=amd-cu4fmc7c
**amd-cu4fmc7c** — (2026-11-18-session-folder-trusted-by-viola) revive's recorded cwd rows gained a second use (trust-record key), each new use of a recorded value amending the "used only as" rows. A parent carried by revive is a new use the same rows' pattern must name.

@history_item n=20 marker=amd-hje724bm
**amd-hje724bm** — (2026-10-10-budget-governor) revive's check runs before the wrapper log opens; a refusal opens no log and writes no line, since an opened log narrows the home and erases the refused state. Order constraint for any lineage read in revive's preflight.

@history_item n=21 marker=amd-ambqnrib
**amd-ambqnrib** — (2026-10-20 wrap) revive's refused strict-modes check writing no line ratified by the founder (row decision/strict-refusal-leaves-no-trace).

@history_item n=22 marker=amd-xuzwgrgf
**amd-xuzwgrgf** — (2026-10-26-home-and-code-bearing-file-integrity) `run` and revive's start run `check_home_tree` through `checked_exe` before the wrapper log opens; refusal exit 1, no log, no line, no child (rows decision/start-checks-home-tree-before-log-opens, state-tree-strict-modes-before-use, strict-modes-at-start). The lineage line at launch must land after this check, in the wrapper's own events.ndjson.

@history_item n=23 marker=amd-cus6gsms
**amd-cus6gsms** — (2026-10-26-home-and-code-bearing-file-integrity) revive's preflight reads once through `read_snapshot_or_replay` and hands that read to both the collision check and the recorded-directory reading. A parent read in revive should reuse that one read, not add a second replay.

@history_item n=24 marker=amd-24x45dat
**amd-24x45dat** — (2026-10-26-home-and-code-bearing-file-integrity) every snapshot reader moved onto `read_snapshot_or_replay`; a replayed snapshot carries no endpoint/pid/started_at and yields no frame (ratified widening: processes read events.ndjson on a new basis). What a lineage field in the replay may and may not enable.

@history_item n=25 marker=amd-pikr4h2t
**amd-pikr4h2t** — (2026-11-14-status-line-interception-removed) decision/replayed-snapshot-yields-nothing reworded (no endpoint, pid or start time, so no frame); contracts cli-state-root/home-flag and /viola-dir name what the home selects. A lineage field added to the snapshot or replay would extend these lists.

@history_item n=26 marker=amd-vzhwfuz2
**amd-vzhwfuz2** — (2026-11-14-status-line-interception-removed) decision/replayed-snapshot-inert reworded to endpoint, pid and start time. Same constraint from the input-validation side.

@history_item n=27 marker=amd-5665sqdu
**amd-5665sqdu** — (2026-11-14-status-line-interception-removed) decision/snapshot-written-only-by-wrapper's why narrowed to the endpoint. Only the instance's own wrapper writes its snapshot; a parent recorded on a child must not be written by the parent's process.

@history_item n=28 marker=amd-g3esiqyy
**amd-g3esiqyy** — (2026-10-31-exit-cause-code-catalogue) `revive --json` documents every refusal of its start with closed exit-1 codes, never a path or pid (contracts revive-json/*, revive-refusal/no-session). A new revive refusal about lineage would be a new closed code — a fork.

@history_item n=29 marker=amd-sjhr4scg
**amd-sjhr4scg** — (2026-10-17-cli-machine-contract) CLI home resolution: `--home`, else the `VIOLA_NAME`/`VIOLA_DIR` instance home when the pair checks, else the default, canonicalised (row decision/cli-home-resolution-order); revive's `--json` closed codes. The launching session's `VIOLA_NAME`/`VIOLA_DIR` is the existing identity a starting wrapper could see of its parent.

@history_item n=30 marker=amd-3oiym264
**amd-3oiym264** — (2026-10-18-plugin-precedence-live-readings) the `VIOLA_DIR` home step ratified by the founder as a widening of what chooses a verb's home. Reading the parent's identity from the same variables for another purpose (a parent record) is a further widening.

@history_item n=31 marker=amd-zzwsrnpk
**amd-zzwsrnpk** — (2026-10-04-the-wheel) `from` taken from `VIOLA_NAME` only, self-reported (`from_trust:"self-reported"`), a driver's string-`from` release refused (rows decision/from-field-authenticates-nothing, channel-from-not-identity). A declared parent is self-reported the same way and must authenticate nothing, change no wheel.

@history_item n=32 marker=amd-esb55l5a
**amd-esb55l5a** — (2026-10-08-first-live-test-and-self-drive) the human-terminal stdin vector: the closed non-editing list, widened once by measured replies (rows decision/human-typing-takes-wheel, stdin-closed-non-editing-list, contracts send-refusal/human-typing, wheel-event/human-input). The wheel rules a parent edge must leave untouched.

@history_item n=33 marker=amd-xsehduha
**amd-xsehduha** — (2026-11-19-context-as-a-local-command) contract/wrapper-channel-methods/send amended (unconfirmable local commands). Whether a send is accepted must not depend on a parent edge.

@history_item n=34 marker=amd-ldkwh6ak
**amd-ldkwh6ak** — (2026-09-25-pty-wrapper-on-windows) R8 as prefix rule + `claude_env_keep` (≤32 names, rejected whole) + identity floor with both messaging variables (contract/config-json/claude-env-keep). The parent-identity variables a launch-time declaration must not read values of or store.

@history_item n=35 marker=amd-6dirgg7r
**amd-6dirgg7r** — (2026-11-05-linux-only-code-and-ci) the R8 strip matches names exactly; identity floor always stripped (rows decision/r8-claude-env-strip, claude-credentials-stripped-from-child, parent-identity-env-stripped). The launching session's own identity is stripped from the child; using it to declare a parent would cross this boundary.

@history_item n=36 marker=amd-fkdvldwn
**amd-fkdvldwn** — (2026-11-20-harness-prefix-and-identity-floor-ledger-rows) identity floor grew to thirteen names; decision/non-identity-hook-names took the four hook names off the floor. The current floor a parent-identity measurement reads against.

@history_item n=37 marker=amd-ntq53yr2
**amd-ntq53yr2** — (2026-10-07-live-rows-and-paste-shapes-on-the-dev-host) two scratch measurements across the credential boundary ratified once each; standing rule: another names probe or peer message needs its own showing. P3's measurement of what a starting wrapper sees of its parent must stay names-only and may need the founder's word.

@history_item n=38 marker=amd-4sakddpi
**amd-4sakddpi** — (2026-11-18-session-folder-trusted-by-viola) the one env-value read exception (CLAUDE_CONFIG_DIR when kept) widened from verify to run/revive's start (rows credential-strip-by-name-only, env-not-config-channel, no-stored-secrets); companion amd-omnnx7wq for r8-claude-env-strip. Precedent: any new env value read at launch amends these four rows' excepts.

@history_item n=39 marker=amd-rgbf2n4h
**amd-rgbf2n4h** — (2026-09-26-local-linux-pre-push-gate) origin of the env carve-out ban: no seam or env read without its own Decisions Log entry, env vars are not a configuration channel (rows decision/controls-not-switchable, no-control-switch-off). A `VIOLA_PARENT`-style variable would be such a new read.

@history_item n=40 marker=amd-ugrtlmgv
**amd-ugrtlmgv** — (2026-10-31-exit-cause-code-catalogue) `without_endpoint` reads only the instance directory: `instance-dead` vs `unwrapped-name`, opening nothing, writing no line (contracts instance-unreachable-detail/*). The existing per-name reading a tree node's state can reuse.

@history_item n=41 marker=amd-kqvzb3vp
**amd-kqvzb3vp** — (2026-11-13-rows-proposal-rule-work) contract/instance-unreachable-detail/unwrapped-name lost its symlink clause: a symlinked instance dir is strict-modes-failed.

@history_item n=42 marker=amd-tsvxhgce
**amd-tsvxhgce** — (2026-10-31-exit-cause-code-catalogue) exit 21 names a closed cause in `detail`, holding no path, pid or upstream text (contract/strict-modes-refusal/list-json reads `strict-modes-failed`). A new list cause is a new closed word — a fork.

@history_item n=43 marker=amd-3luna2mx
**amd-3luna2mx** — (2026-10-31-exit-cause-code-catalogue) the `unwrapped-name`/`endpoint-lost` hints last on stderr in ASCII, `<name>` the parsed `ViolaName` (contract/cli-stderr-hints/unwrapped-name-endpoint-lost). Names printed by a hint come only from parsed names.

@history_item n=44 marker=amd-sdvrxace
**amd-sdvrxace** — (2026-10-31-exit-cause-code-catalogue) the catch site's `--json` internal-error document; run, revive's start, mcp and hook silent there (contract/cli-json-error/internal-error).

@history_item n=45 marker=amd-a37wdtds
**amd-a37wdtds** — (2026-10-31-exit-cause-code-catalogue) `collision_check` prints `stale-heartbeat` vs `already-live` (contracts collision-check/*). A reused name's start passes through this check; "a reused name never inherits an old parent" must hold past it.

@history_item n=46 marker=amd-hubagvb3
**amd-hubagvb3** — (2026-11-01-unix-endpoint-and-home-hardening) at a start `socket_dir_ready` checks before any snapshot, child or bind, shared by revive's start; one `socket-dir-refused` code (contract/socket-dir-refusal/run-and-revive-start, decision/run-refuses-socket-dir-before-start). Ordering the lineage write must sit inside.

@history_item n=47 marker=amd-ap3enen5
**amd-ap3enen5** — (2026-10-26-home-and-code-bearing-file-integrity) every client checks the instance before reading the snapshot; refusal no frame, no line (rows decision/client-checks-before-frame, strict-modes-before-snapshot-read, snapshot-strict-checked-before-trust).

@history_item n=48 marker=amd-zafz4vgj
**amd-zafz4vgj** — (2026-10-26-server-verification-before-any-frame) the closed `ServerVerdict`; only `Matches` writes a frame (row decision/only-matches-verdict-lets-a-frame-out).

@history_item n=49 marker=amd-yrfueg5n
**amd-yrfueg5n** — (2026-10-26-server-verification-before-any-frame) a failed server check: no frame; no line or output holds a pid, start time, path or endpoint (row decision/no-identity-values-in-refusals). A parent's pid or start time must not surface in any output.

@history_item n=50 marker=amd-mk6igznz
**amd-mk6igznz** — (2026-10-26-home-and-code-bearing-file-integrity) `check_instance`'s set (home, instance and `diagnostics/` to the read rule, `instances/`, snapshot, events.ndjson to the write rule) and the `Readable`/`Symlink` refusals (rows decision/strict-modes-read-rule, strict-modes-before-read, strict-modes-check).

@history_item n=51 marker=amd-zoeaxb4o
**amd-zoeaxb4o** — (2026-11-05-linux-only-code-and-ci) strict-modes is owner and mode alone (`mode & 0o077` read rule); an unstattable home is a refusal (row decision/unstattable-home-refused).

@history_item n=52 marker=amd-jas4miib
**amd-jas4miib** — (2026-10-26-home-and-code-bearing-file-integrity) ban: never run the check after a create, mode set or log open that narrows the checked path (row decision/strict-check-before-narrowing).

@history_item n=53 marker=amd-fj2ck37p
**amd-fj2ck37p** — (2026-09-27-instance-state-and-start-order) the shared `replace_private` helper sets the mode before any byte (rows decision/home-owner-only, private-file-modes, owner-only-file-modes).

@history_item n=54 marker=amd-4sxz5z6g
**amd-4sxz5z6g** — (2026-11-17-trust-record-ledger-row) decision/explicit-mode-before-write gained its one except (the trust-record writer). Any new file this chunk writes takes the rule with no new except.

@history_item n=55 marker=amd-e4fxmjmx
**amd-e4fxmjmx** — (2026-11-13-rows-proposal-rule-work) decision/logs-owner-only: event log, heartbeat and diagnostics 0600 in 0700 dirs, append open never follows a symlink. The lineage line appends through this open.

@history_item n=56 marker=amd-p7254a3f
**amd-p7254a3f** — (2026-11-14-status-line-interception-removed) decision/max-frame-shared-cap and home-strict-modes-before-read reworded to their current consumer lists. A new read (a parent's snapshot or log) joins the MAX_FRAME list and the census.

@history_item n=57 marker=amd-o677no2k
**amd-o677no2k** — (2026-10-27-bounded-inputs-at-every-boundary) `tests/contract_bounded_readers.rs` census: every product reader bounded in its statement or named (row decision/bounded-readers-census). A new reader of a parent's state fails the census unless bounded.

@history_item n=58 marker=amd-w2b5ogmy
**amd-w2b5ogmy** — (2026-10-27-bounded-inputs-at-every-boundary) serde_json `unbounded_depth` banned in deny.toml with probe and test (rows decision/serde-json-bounded-depth, serde-json-unbounded-depth-banned).

@history_item n=59 marker=amd-6wkya775
**amd-6wkya775** — (2026-10-31-sanitised-error-surfaces) `tests/contract_error_bodies.rs`: every error writer named, `thiserror` messages interpolate nothing (rows decision/every-error-writer-named, error-messages-interpolate-nothing). A new refusal or error document must be added to the census.

@history_item n=60 marker=amd-bt2fw3dh
**amd-bt2fw3dh** — (2026-10-31-sanitised-error-surfaces) clients screen a peer's reply to closed values via `sanitised_fault` and `is_code_word` (rows decision/client-reads-closed-values-only, client-reply-code-words-only). Any parent/link value read from a reply must pass the same screen.

@history_item n=61 marker=amd-uc6w5fo7
**amd-uc6w5fo7** — (2026-11-14-hyprland-seam) decision/shared-validators-in-core gained the seam-crate except; decision/parser-fuzz-scope widened to the hyprctl parsers. A new lineage parser is a parser surface for fuzz/property scope.

@history_item n=62 marker=amd-l2ajoemi
**amd-l2ajoemi** — (2026-11-16-local-command-delivery) decision/shared-validators-in-core except widened to the transcript path check. Precedent: a validator outside viola-core must be named in the except; parent names go through the one `ViolaName` check (rows decision/cli-names-through-name-check, names-through-validated-type, instance-path-from-validated-name).

@history_item n=63 marker=amd-ow5es6l4
**amd-ow5es6l4** — (2026-11-16-local-command-delivery) decision/transcript-path-redacted added: an OS-username-bearing path appears in no event data, log line or reply. Same class as what a lineage record may hold of a parent.

@history_item n=64 marker=amd-y27ejyen
**amd-y27ejyen** — (2026-11-14-status-line-interception-removed) decision/user-content-only-to-diagnostics rule current; except lists events.ndjson as content-carrying by design. A lineage line holds names and closed words, no content.

@history_item n=65 marker=amd-5ndzdd2g
**amd-5ndzdd2g** — (2026-09-28-capability-ledger-and-viola-verify) the real CLI runs only locally, verify in CI only against the fake agent (rows decision/real-cli-local-only, ci-no-secrets-no-real-claude). The scope's no-live-start directive matches; lineage witnesses run on the fake agent.

@history_item n=66 marker=amd-wxkn7twf
**amd-wxkn7twf** — (2026-09-28-mutation-testing-to-the-epoch-boundary) mutation testing left CI for the epoch-boundary audit (row decision/mutation-testing-at-audit). No mutation run in this phase, as the scope says.

@history_item n=67 marker=amd-ucp5wit6
**amd-ucp5wit6** — (2026-09-24-quality-gates) the nightly fuzz job on the pinned fuzz nightly (row decision/scheduled-fuzzing). Where a new parser's fuzz target would run.
