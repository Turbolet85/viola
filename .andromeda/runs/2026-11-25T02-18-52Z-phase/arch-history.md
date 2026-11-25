@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns; Occupied Resources) — added decision/hyprland-seam-crate (viola-hyprland is the one compositor seam, its closed request set the only way to the compositor, sync, tokio-free, depending on viola-core alone) and registered name/crate/viola-hyprland and name/binary/viola-hypr-tap. Why it matters: the move's new form must enter as a member of this closed request set, through this crate and its tap, never as a second seam; the crate edge rule bounds where the form may live.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Standard Contracts; Conventions) — added decision/hyprctl-direct-spawn (direct spawn, no shell, closed argv, Lua dispatch form) and the five contract/hyprland-seam-errors/* rows (unsupported, unreachable, oversize, malformed, refused), plus the except on decision/unparseable-external-values-unknown (an unparseable hyprctl reply is the seam's malformed error). Why it matters: a new move form must keep the closed argv and the five closed errors; the fake's reply set must stay within what these errors and the real compositor answer.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns, decision/root-bin-consumes-normalised-events) — dropped «depends on all members» because viola-hyprland was then only a root dev-dependency while no verb called the seam. Why it matters: history of how the seam reached the root bin; the callers (open, up, revive --origin) are what later made it a product dependency.

@history_item n=4 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Standard Contracts, decision/hyprctl-direct-spawn rule and why) — widened the dispatch from hl.dsp.focus alone to «one Lua element, hl.dsp.focus or hl.dsp.window.move», only a closed address or an i32 workspace inserted; why: Hyprland 0.56.2 runs dispatch args as Lua, so only the measured forms are accepted (PROVISIONAL on the delegate's confirm, later ratified with the Epoch batch). Why it matters: this is the move this chunk fixes (it takes the active workspace with it, R4a 1→5); any new form (a silent or no-focus move) is a third dispatch form that must be measured and named in this row's rule and why, the same way.

@history_item n=5 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Standard Contracts) — added the `window` wrapper append, event data `{address, workspace, placement, origin}`, the snapshot's `window` field and its replay rule, list items' `window?`/`origin?`, and viola open's exit codes 0/1/2 (exit 0 `<name>  live  <placement>`, ok document with window and placement). Why it matters: the window step this chunk changes writes these records; placement is a closed word the open's output reports, and R2 of this chunk read placement only — the active-workspace clause is new and must not be confused with placement.

@history_item n=6 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Conventions; Occupied Resources; Established Decisions) — added contract/cli-verbs/open, contract/normalised-event-kinds/window, exit 1 for open's refusals, name/command/open, process-exit details terminal-not-found and not-live, the hidden run --origin and run --place (place without origin a usage error), and open into decision/no-start-over-live-instance. Why it matters: `viola open` is the first caller of the move; its hidden run flags are how the window step is reached, and any form difference per caller must stay inside these registered surfaces.

@history_item n=7 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Occupied Resources; Conventions) — occupied the process-exit detail words window-not-found, unsupported, unreachable, oversize, malformed, refused for viola focus (the seam's error words), added focus to exit 1, exit-21 strict-modes-failed and the cli-<name> role file, registered name/command/focus and contract/cli-verbs/focus. Why it matters: focus is the hl.dsp.focus caller of the same seam and the wrap that found the F4 defect (its CARRY placed it on this entry); a changed move must leave these seam-error words and focus's use of the seam unchanged.

@history_item n=8 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Established Decisions; ratification) — added decision/focus-has-no-driver-form (provisional, then standing on the operator's F-A arm 1), and a later entry ratified F-A, F-C and F-E (the six refusal pairs, the criterion). Why it matters: the window features stay human-only surfaces; a fixed move must not add a driver-facing window operation.

@history_item n=9 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Established Decisions; Conventions) — decision/lineage-label-only gained «viola up reads the parent only to order its re-opens»; decision/revive-resumes-in-place gained «under the hidden --origin the recorded workspace»; contract/cli-verbs/revive gained the hidden --origin conflict; added contract/cli-verbs/up and up's exit codes 0/1/21; exit 1 names up. Why it matters: `viola up` and `viola revive --origin` are the move's other two callers (each re-places a window on the recorded workspace); each caller's form decision at P3 must fit these rows.

@history_item n=10 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Occupied Resources; Standard Contracts) — added name/command/up, contract/viola-cli-arguments/revive-origin (places the window on the workspace a real snapshot recorded, else none; no window step without it), process-exit boot-time-unread, up into cli-<name>, not-live and terminal-not-found, name/other/foot-spawns-ndjson (the fake foot's spawn record), decision/up-writes-no-state (no window address, pid or workspace in its output), and contract/instance-snapshot/cwd's second reader. Why it matters: revive --origin's window step is the same `Request::MoveToWorkspace` path; up's fake foot and its no-window-identity output are fakes and outputs this chunk's witnesses will touch.

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Standard Contracts, contract/instance-snapshot/cwd ratified) — the cwd row (up opening each terminal in the recorded cwd) ratified on the operator's W1 answer. Why it matters: a picked row; revive's and up's reads of the snapshot stand, so a move-form change must not alter what up and revive read.

@history_item n=12 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Standard Contracts) — contract/wrapper-event-appends/window now re-appends the window line after a switch's launch line when the snapshot holds a window; contract/wrapper-event-appends/launch appends at every switch; snapshot launch is the latest switch's pair. Why it matters: the window record is written on a second path (switch), so a change to what the window step records must keep the switch's re-append consistent.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (Design Philosophy; [Scale / Product]) — made viola Linux only, the 0.2.0 window features on Hyprland, the Windows and macOS code out of the tree (recoverable at multi-os-final). Why it matters: the move's form is measured for Hyprland alone (decision/window-features-target-hyprland, decision/linux-only); no other compositor or OS arm is owed.

@history_item n=14 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (Established Decisions; Standard Contracts; Conventions; Occupied Resources) — added decision/start-verifies-new-cli-version (config verify_new_cli runs viola verify once as a child after the endpoint bind, gating on the final verdict), decision/no-automatic-reverify-of-red (an entry holding every relied row that did not verify is never verified again automatically), decision/automatic-verify-attempt-lock, contract/config-json/verify-new-cli, name/other/ledger-verify-lock, the verify.lock except on decision/on-disk-file-naming, contract/instance-snapshot/cli-verified as the start's final verdict, and moved decision/run-start-order to «binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record». Why it matters: this is the code the folded item reads (src/run/auto_verify.rs, the start in src/cmd/run.rs); the 9-pass/19-fail stamp of 2.1.331 is kept because no-automatic-reverify-of-red forbids a retry, so any fix to where it runs or what it stamps on its own failure amends these rows (all ratified at Epoch 4's batch).

@history_item n=15 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Cross-cutting Patterns, decision/viola-home-resolution) — the rule now says viola run passes the home to its claude child only through VIOLA_DIR and to its automatic verify child only through --home. Why it matters: the automatic verify child's home and environment come from the start; where its probe runs (cwd) is the folded item's question and the start-to-verify hand-off is fixed here.

@history_item n=16 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (ratifications of Epoch 4 delegate confirms) — ratified the automatic-verify rows (start-verifies-new-cli-version, no-automatic-reverify-of-red, automatic-verify-attempt-lock, verify-new-cli, verify.lock, cli-verified, run-start-order), the trust-record rows (verify-child-set, verify-own-trust-record, trust-record-ledger-row, the env-strip exceptions, claude-env-keep, env-var prefix, no-environment-configuration, instance-snapshot/cwd, spawn-directory-confined-to-snapshot, trust-writer key-present/removed) and the browser-tools-prompt verified-row rows. Why it matters: these rows now stand on the founder's word, so changing any of them for the folded item is an amendment of ratified decisions that needs its own fork, not a quiet edit.

@history_item n=17 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify gained interactive PTY runs: Run A untrusted in an OS-temp dir (killed), Run B trusted in `<cwd>/.viola-verify-<pid>/`; measured on 2.1.288 that trust is inherited from a trusted parent and the external-imports approval is keyed on the git root; the probe dirs and their removal were registered. Why it matters: Run B (and later C, D, E) take trust from the cwd verify runs in, which is exactly why an automatic verify run from an untrusted scratch folder stamped red — the folded item's cause sits in this placement.

@history_item n=18 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D run in `<cwd>/.viola-verify-<pid>-dialogs/` and `-plan/`, their rows failing when no dialog is raised. Why it matters: more of the relied rows depend on verify's cwd being trusted; part of the 19 fails a scratch-folder verify produced.

@history_item n=19 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E runs in a guarded `<cwd>/.viola-verify-<pid>-resume/` and resumes Run B's first session; a verify is six claude starts. Why it matters: another cwd-placed run whose row reads red when Run B cannot start in an untrusted folder; any change to where the automatic verify runs moves these dirs.

@history_item n=20 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify's child set became six runs (Run F after Run E), added decision/verify-own-trust-record (verify writes a record for its own temp folder only, never creating an absent config, removing it after), decision/trust-record-ledger-row, the closed contract/trust-writer-words/* set, the trust-record value in contract/ledger-stamps/rows and content, and the CLAUDE_CONFIG_DIR read exceptions; a later entry of the same chunk reworded the row as what the probe measures per version, its one live reading on 2.1.325 being fail. Why it matters: verify already writes trust for its own folder; a fix that lets the automatic verify run trusted is bounded by this one ruled exception to writing into Claude's config.

@history_item n=21 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — retired «run and revive write no trust record»: decision/run-start-order gained «on a verified CLI writes the trust record of an untrusted spawn folder»; contract/instance-snapshot/cwd and decision/spawn-directory-confined-to-snapshot name the trust key; key-present and removed are verify's only (a start reads them as lost); run and revive read a kept CLAUDE_CONFIG_DIR. Why it matters: the start trusts the spawn folder only on a verified verdict, so an automatic verify that stamps red because the folder is untrusted locks the start out of the very trust step that would fix it — the trap the folded item names.

@history_item n=22 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — added the rig-only hidden --record-first-screens (scrubbed whole-row first screens of a failed verify) as excepts on decision/verify-record-only-clean, decision/recorded-fixtures-scrubbed, decision/screen-fixtures-signature-rows-only, decision/run-never-reads-screen-content and contract/viola-cli-arguments/verify, and the stamp's measured.first_screen / first_settle_ms. Why it matters: first_screen (trust, imports, browser-tools, box, none, exited) is the measured reading that can show a verify ran in an untrusted folder; a witness for the folded item can read it rather than new screen text.

@history_item n=23 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — a version is verified when every relied row counts (pass, or absent on browser-tools-prompt alone): decision/version-verified-all-relied-rows, contract/ledger-stamps/verified-version, rows, content, first-screen gained browser-tools, verify's step and summary lines, decision/capability-ledger and decision/cli-capability-ledger, and decision/browser-tools-prompt-ledger-row added. Why it matters: the 2.1.331 stamp read «9 pass · 19 fail · 1 absent»; the verdict and the summary count follow these rows, and the folded item's acceptance must read them as stated.

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (statusline-rate-limits) — recorded that a ledger row read red on an unchanged CLI because the account's payload differed (`23 pass  1 fail` on the rig home), ruled a finding: the row read the environment, not the CLI. Why it matters: direct precedent for a stamp recording what its surroundings made it read rather than the CLI — the class of defect the folded item's untrusted-folder stamp belongs to.

@history_item n=25 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed viola verify, the stamps envelope and its one writer update_stamps: a failing row still stamps and exits 1; merge replaces only its own version's entry. Why it matters: «a failing stamp is kept» is this designed behaviour; the folded item's fork (not stamping on verify's own precondition failure) amends the writer's contract (decision/stamps-merge-own-version-only, decision/stamps-single-writer, contract/viola-verify-exit-codes/1-relied-row-failed).

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer; the home-tree check) — update_stamps runs check_stamps before creating ledger/ and again under its lock, a refusal reading and writing nothing (StampsError::StrictModes); check_home_tree runs before verify's and every start's log, a refusal exit 1 with the home pair and no line. Why it matters: these are the existing «precondition failed, write nothing» paths of verify and the start; a new precondition refusal for the automatic verify should follow their shape (decision/stamps-writer-checks-before-and-under-lock, decision/verify-refuses-untrusted-home-or-stamps, contract/verify-refusals/home-tree).

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes) — verify's process-exit under a parsed VIOLA_NAME carries its Ended::Refused code beside interrupted; a failing row is no refusal; verify takes no --json. Why it matters: a new automatic-verify refusal (an untrusted folder, a modal) would be a new closed word in this set (contract/verify-process-exit/*), passing is_code_word and the error-body census.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (batch-script-child retired) — verify's Ended::Refused codes reduced to five (program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture) beside interrupted; revive --json's codes to ten. Why it matters: the current closed sets any new refusal word for the folded item extends; a word no build emits leaves with its emitter.

@history_item n=29 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — decision/verify-removes-only-own-dirs and decision/verify-own-trust-record gained the SIGKILL residual (verify's one-key trust record left, removed by hand). Why it matters: an automatic verify started by a start inherits this residual; moving where it runs must keep its cleanup and the named residual true.

@history_item n=30 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the snapshot's cwd; the child's second cwd source) — the snapshot gained cwd; a revived start spawns in the recorded cwd, read after the strict-modes check, else cwd-missing; program resolution and the version gate never read it. Why it matters: decision/child-cwd-explicit and decision/revive-program-resolution bound where a start runs things; the automatic verify's place relative to the spawn folder must respect that the gate reads no recorded directory.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (one read per revive) — a revive reads its snapshot once through the replay for its holder check and its recorded directory, one state-recovered line per revive. Why it matters: decision/revive-reads-snapshot-once; revive --origin reads the recorded workspace from the same snapshot, so a caller-side change of the move must not add a second read.

@history_item n=32 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's refused socket directory) — run and revive's start refuse a socket directory through socket_dir_ready before any snapshot, child or bind: exit 1, the pair or the socket-dir-refused document, one process-exit line. Why it matters: contract/run-start-refusals/socket-dir-refused; the endpoint bind precedes the automatic verify in the start order, so the folded item's change sits after these refusals.

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start; the catch site's document) — revive --json prints a closed exit-1 document for every refusal of the start it shares with run, and a cli verb under --json prints the internal-error document at the catch site while run, revive's start, mcp and hook stay silent. Why it matters: a new start refusal from the automatic verify reaches revive --json and adopt --json as a code in these closed lists (contract/viola-revive-json/*, contract/cli-json-output/*).

@history_item n=34 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Standard Contracts; Conventions; Occupied Resources) — adopt's start joined the silent roles at the internal-error catch site, adopt --json gained its refusal codes, adopt logs into run-<name>.ndjson, and adopt's start reads a kept CLAUDE_CONFIG_DIR like run and revive (later ratified, W5). Why it matters: adopt is a third start through run's start, so the automatic verify and its trust step run there too; a change to the start order reaches adopt.

@history_item n=35 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — decision/no-document-after-child-held-terminal gained its one except (adopt --json's not-resumed end prints after the child released the terminal); adopt --json gained not-resumed. Why it matters: a picked row; any new start-time document for the folded item comes before the child holds the terminal, never after.

@history_item n=36 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — decision/screen-signatures-compiled and decision/vt100-readiness-gate read a standing modal (a quoted label not refusing), with the except naming the residual. Why it matters: «a modal» is one of the folded item's preconditions for the automatic verify; how a modal is recognised is fixed by these rows.

@history_item n=37 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — the readiness predicate reads the input box by its footer literal or its own rule and prompt rows; verify's box wait ends on the box or a modal literal; screen fixtures keep box rows as their literals; stamps measure box_only_guards. Why it matters: verify's settle on a first screen (box versus trust modal) is the reading that separates a trusted start from an untrusted one in the folded item's witness.
