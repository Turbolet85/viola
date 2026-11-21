@history_item n=1 marker=amd-boij6h33
**amd-boij6h33** — (2026-11-20-harness-prefix-and-identity-floor-ledger-rows, §4) `~ decision/identity-floor-always-stripped`: the floor is no longer "11 names"; a compile-time NON_IDENTITY_HOOK_NAMES list stands beside it. This is the chunk the cross-session row was cut from, so its landed rows are the base this chunk builds on. Restate no count of the floor.

@history_item n=2 marker=amd-n4b5ijaf
**amd-n4b5ijaf** — (same chunk, §4) `~ decision/env-names-only`: the capture arm's env-names file and verify's env_probe stamp are names-only. A new cross-session probe record or replay must stay names-only, with no env map passed to a span or event. PROVISIONAL (delegate, inputs#I15).

@history_item n=3 marker=amd-xu5dctrm
**amd-xu5dctrm** — (2026-11-17-trust-record-ledger-row, §6) `~ contract/diag-subject/verify-pty-probe` (five to six runs, A–F) and `~ contract/verify-process-exit/child-exit-status-and-duration` (eight to nine spawn pairs). This is the last precedent for a ledger row that moves verify's spawn count. A print-mode sender spawn would move "nine" again, and the subject row may need a new word.

@history_item n=4 marker=amd-zhy63jc5
**amd-zhy63jc5** — (2026-11-17-trust-record-ledger-row, §4) `~ contract/verify-spawn-subject/verify-pty-probe`: five to six interactive runs. This is the count register the sender's spawn (or a Run B change) has to keep true.

@history_item n=5 marker=amd-vnt75qwd
**amd-vnt75qwd** — (2026-09-29-verify-stamped-test-homes-and-harness) `verify-probe` joined the closed `subject` enum because every child spawn must log a process-start/process-exit pair at the call site, with `run_bounded` left unlogged. A new closed value needs a Decisions Log record, and CI verify runs only against the fake agent. Traps for the sender spawn: it must log a pair, its subject must be closed in `schemas/diag-line.v1.json`, and the real-CLI verify never runs in CI.

@history_item n=6 marker=amd-meyxwweu
**amd-meyxwweu** — (2026-11-14-hyprland-seam, §6) Precedent for adding a closed subject word. `~ contract/diag-line-fields/process-start` names the new subject, and `+ contract/diag-subject/hyprctl-probe` stands as its own row, because each subject word is its own contract row. If the sender takes a new subject, both rows move the same way.

@history_item n=7 marker=amd-qzec4puu
**amd-qzec4puu** — (2026-11-20-a-new-cli-version-stamped-automatically, §6) `~ contract/diag-line-fields/process-start` gained `auto-verify`, and `+ contract/diag-subject/auto-verify` was added. The sender also runs inside the automatic verify that `verify_new_cli` triggers, writing to the run log under that child. PROVISIONAL (delegate).

@history_item n=8 marker=amd-5psnrxhp
**amd-5psnrxhp** — (same chunk, §4) `+ contract/run-process-log/auto-verify-stop`: a stop during the automatic verify TERMs the child, then KILLs it after 30 s, and the start ends with no process-exit, stamp or session. The longer verify run that the sender adds must keep this stop shape.

@history_item n=9 marker=amd-nojl6uuq
**amd-nojl6uuq** — (same chunk, §3) `~ contract/config-json/known-keys`: five keys with `verify_new_cli`, default off. The automatic verify that this chunk's sender would join stays behind that key. Add no config key for the cross-session row.

@history_item n=10 marker=amd-lg5jrfgm
**amd-lg5jrfgm** — (2026-11-18-browser-tools-prompt-modal, §4) `~ contract/verify-stdout/step-line` (pass|absent|fail, MM = relied count), `~ contract/verify-stdout/summary-line` and `~ contract/verify-exit-codes/0`. The 29th relied row moves MM from 28 to 29. These rows hold MM by rule, so state no count, and the new row reads pass or fail, never absent.

@history_item n=11 marker=amd-o54uko65
**amd-o54uko65** — (2026-10-18-compact-post-condition-ledger-row) The 24th relied row moved MM, and Run B's sixth paste added no spawn pair or subject (still eight pairs). This is the precedent for a Run B addition that adds no pair. The sender, though, is a separate CLI spawn, which would add one.

@history_item n=12 marker=amd-zzlsyj35
**amd-zzlsyj35** — (2026-10-16-unwrapped-sessions-on-the-board) Verify went from seven to eight spawn pairs: Run B's single agents read reuses the `agents-probe` subject, so the closed enum did not move, and no line carries row text, a session id, cwd or path. This is the precedent for reusing an existing subject for a new verify spawn instead of widening the enum.

@history_item n=13 marker=amd-betgscce
**amd-betgscce** — (2026-10-14-resume-and-compact-ledger-rows) Run E (a `--resume` of Run B's session) raised verify to seven spawns and moved MM. A child of an interactive PTY run killed at a stop carries child_exit_status. A print-mode or `--version` child killed at a stop is unmeasured, which is a trap for a print-mode sender.

@history_item n=14 marker=amd-qd62nlqj
**amd-qd62nlqj** — (2026-10-13-owed-ledger-rows) Run B's `verify-pty-probe` pair carries one probe `--settings`, and no line carries the probe command, a payload or a path. This is the rule Run B's compiled `crossSessionInbound` accept literal falls under: it never reaches a line (see `decision/child-settings-never-logged`).

@history_item n=15 marker=amd-a3dzas4e
**amd-a3dzas4e** — (2026-11-05-linux-only-code-and-ci) The relied count reads all rows of `ALL` (no per-OS MM), and the probe `--settings` lost its Unix-only qualifier. A verify stop is SIGINT or SIGTERM. The new row joins `ALL` with one count on Linux.

@history_item n=16 marker=amd-qr4z6h75
**amd-qr4z6h75** — (2026-10-05-real-cli-verify-probes) Verify's interactive PTY runs were instrumented at the call site like the other spawns, gaining `verify-pty-probe`. In CI, verify drives the fake agent's interactive runs with flags. This is the precedent for the fake agent replaying the recorded cross-session payload.

@history_item n=17 marker=amd-7nwmoqnr
**amd-7nwmoqnr** — (2026-10-06-local-command-and-paste-framing-rows) The fake-agent verify flag list gained `--framing` for a new replayed row. The list is bound to test-plan §3's boot step 4, which moves in the same pass. A new replay flag for the cross-session payload moves both.

@history_item n=18 marker=amd-2kogdiq2
**amd-2kogdiq2** — (2026-10-05-dialog-rows-and-re-probe) Runs C and D raised verify to six spawn pairs with the subject enum unchanged. The capture arm stays uninstrumented, with `--answers` printing one product-built body. This is the capture-arm boundary a Run B hook reading of the prefixed prompt must respect.

@history_item n=19 marker=amd-7tsldkoh
**amd-7tsldkoh** — (2026-09-28-capability-ledger-and-viola-verify) Verify's outcome is exit 0/1 plus one stdout step line per row and a last `stamped` line. The hidden `hook --capture` arm is uninstrumented by design (the founder's ratification). This is the origin of the verify-stdout and capture-arm rows this chunk extends.

@history_item n=20 marker=amd-26guaqhc
**amd-26guaqhc** — (2026-10-13-interrupted-verify-cleanup) `detail:"interrupted"` was added: a stopped verify starts no further spawn and closes every pair it opened. The sender's spawn must be covered by that stop rule (`contract/verify-process-exit/interrupted`).

@history_item n=21 marker=amd-ykbpvuii
**amd-ykbpvuii** — (2026-10-31-exit-cause-code-catalogue) Verify writes its refusal code on process-exit (`contract/process-exit-detail/verify-refusal`), and a failing row carries no code. A red `cross-session-prefix: fail` (`no-prompt`) is a failing row, not a refusal, so it adds no detail word.

@history_item n=22 marker=amd-6734vgcg
**amd-6734vgcg** — (2026-10-26-home-and-code-bearing-file-integrity) Verify's home-tree check runs before its log opens and writes no line. A refused `update_stamps` ends with verify's own process-exit (`contract/viola-verify-refusal/*`, `decision/strict-modes-failure-lines`). Stamping the 29th row goes through that writer.

@history_item n=23 marker=amd-b7lxppqj
**amd-b7lxppqj** — (2026-10-04-dialog-answers-by-dialog-id) `parse-rejected{ledger-stamps}` gained `strict-modes-failed`, read as unverified, and CI verify runs at the recorded fixture version. This is the base of `decision/unreadable-ledger-is-unverified` and the run version gate rows, which a 29-row stamp is read through.

@history_item n=24 marker=amd-rxeb74yg
**amd-rxeb74yg** — (2026-09-29 registry migration) §6 child spawns: `$defs.subject.enum` closes `subject`, and verify's spawn exits carry child_exit_status + duration_ms (D-35). §4 E2 holds the R8 strip and identity floor. These are the closed-enum mechanics a sender spawn must follow.

@history_item n=25 marker=amd-7tvqnh5t
**amd-7tvqnh5t** — (2026-11-05-linux-only-code-and-ci) Rule: a closed word no build can emit leaves the schema with its emitter (`decision/unemittable-schema-words-removed`). Trap: if the last chunk's cut left a subject or parser word for the sender with no emitter, or if this chunk adds one and is cut again, the word goes with its code.

@history_item n=26 marker=amd-vzibbx6o
**amd-vzibbx6o** — (2026-11-14-budget-gate-removed, §1) `~ decision/diag-schema-additive-only` was narrowed to "must not rename or remove a field the product still emits". `amd-rg4iquwi` and `amd-2nptbhad` narrowed the twins `decision/log-fields-additive-only` and `decision/log-schema-additive-only` the same way. Adding or re-adding the cross-session sender's fields is additive and needs a Decisions Log record.

@history_item n=27 marker=amd-3cobhkpd
**amd-3cobhkpd** — (2026-11-17-trust-record-ledger-row, §1) `~ decision/claude-internals-not-a-contract` and `~ decision/refusal-telemetry` each gained a named bounded exception for verify's trust-record write. This is the precedent: any new reading or write into claude's internals for the cross-session probe (a settings literal, a sender's transcript) must be a named exception on these rows, ratified.

@history_item n=28 marker=amd-v67jn6xb
**amd-v67jn6xb** — (2026-11-18-session-folder-trusted-by-viola, §1) These are the current excepts of `decision/claude-internals-not-a-contract` and `decision/refusal-telemetry`: the transcript echo, verify's and the start's trust records. The sender's Run B start or a sender folder's trust would have to fit inside them or amend them.

@history_item n=29 marker=amd-22wdg6gb
**amd-22wdg6gb** — (2026-11-17-trust-record-ledger-row, §4) `+ decision/trust-record-write-uninstrumented`: verify's trust-record write is uninstrumented, the outcome riding the stamps. This is the precedent for a verify-side probe step whose outcome rides only the stamp, as the cross-session probe's result would.

@history_item n=30 marker=amd-6atx2ld7
**amd-6atx2ld7** — (2026-11-18-browser-tools-prompt-modal, §4) `+ decision/browser-tools-prompt-reading-uninstrumented`, refined by `amd-5dqcemjs`: a new verify reading with no telemetry of its own records that choice as a row. A diagnostic reading of the sender's tool calls by NAME, or of the send tool's result class, would need the same kind of row (a fork per scope).

@history_item n=31 marker=amd-lapxdzct
**amd-lapxdzct** — (2026-09-25-pty-wrapper-on-windows, D-34) process-start{claude-child} carries `env_kept`, and every stripped and kept name is logged, never a value (`contract/claude-child-process-start/env-*`). A sender child that verify spawns with a stripped environment logs names only.

@history_item n=32 marker=amd-rgx3hiwd
**amd-rgx3hiwd** — (2026-11-16T06-05-33-wrap) The founder ratified that StopFailure stands with no ledger row and no verify probe, its live reading owned elsewhere. This is the precedent that a CLI behaviour can be registered with no relied row. It bears on the fork if the `no-prompt` cause stays unsettled.

@history_item n=33 marker=amd-yjzki2j3
**amd-yjzki2j3** — (2026-09-24-three-os-ci-headless-harness-skeleton) The fake agent is a root-package `[[bin]]`, exempt from the print-macro ban by a crate-level allow (`decision/no-print-macros` except). Replaying the cross-session payload in the fake agent stays under that one exemption.
