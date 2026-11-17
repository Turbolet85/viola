@history_item n=1 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — (Surface: cli: 1 row) changed contract/viola-send-output/read-back from «the matching prompt-submitted» to equality or equality less a trailing run of space, TAB, CR, LF (PROVISIONAL then). This chunk carries the live reading of which whitespace the CLI trims at submit, and that reading decides whether the row ever widens to a leading trim.

@history_item n=2 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Surface: cli: 1 row) dropped the provisional mark from contract/viola-send-output/read-back. The trailing-only tolerance is ratified by the founder. The row stays as written unless this chunk's reading lands and the founder places a change; the scope's Boundaries keep the tolerance trailing-only until then.

@history_item n=3 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the send trailing-only tolerance ratified by the founder) names «Standing live rig with a screen model» as the owner of the leading-trim («either end») reading. It is the layout-side record of the carry (cry-vdpx2jqz) that this chunk takes.

@history_item n=4 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — ~ contract/viola-send-output/unconfirmable (the /cost, /usage and /mcp cases) and + read-back-mcp. These are the latest send-outcome rows a live send through the rig would observe; the rig must read them as they stand, with no product change.

@history_item n=5 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2) ~ contract/viola-send-output/unable: exit 11 is retired and never reused, so a send refusal exits 13 or 10/12/14. A rig that classifies send exits must not expect 11.

@history_item n=6 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses control-character and empty-text texts itself before any frame, and the wrapper repeats both refusals (rows contract/send-refusals/*, decision/wrapper-repeats-send-refusals). Whether the TTY issue line prints on a client-side refusal was left unstated because nothing measured it, which a screen-model rig could now read.

@history_item n=7 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the first live test reworded the input-not-ready hint to the founder's text (row contract/viola-send-output/input-not-ready-hint). This is a precedent that live rounds pin the shipped wording, and the hint is a line a live send through the rig may print.

@history_item n=8 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back line's second trigger: /clear is confirmed by its new-session post-condition. The read-back row the rig observes covers more than prompt equality.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — /compact is read back by its compacted-session post-condition inside a 40 s PROVISIONAL window, and verify runs at 24 rows. This is the timing a live /compact through the rig meets.

@history_item n=10 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output structure, usage `[--record <DIR>] [-- <program> [args…]]` and its refusal pairs, a dirty recording among them (rows contract/viola-verify-output/*, contract/viola-verify-refusals/*). The rig's live `verify --record` (the record-home carry) runs through this surface.

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added the modal-signature screen row, the `--help` paragraph (run from a folder trusted in Claude Code; an unapproved external CLAUDE.md import blocks the probe) and the named fixture-not-clean refusal (`home-path`, `absolute-path`, `username`, `email`). This bears directly on the rig's trust-dialog handling and its scratch-root and record-home isolation: a record under a home path is refused.

@history_item n=12 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify at 14 rows, Kept: verify never answers the CLI-native external-import dialog, and the probe's own dialogs are answered by its hook. This is a constraint on how the rig reads and answers modal and trust dialogs, as distinct from verify.

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify stopped by SIGINT/SIGTERM prints a fixed pair, exits 1 and writes no summary or stamp (row contract/viola-verify/summary). The rig's kill deadline and exit-path cleanup meet this form when they stop a live verify.

@history_item n=14 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `MM` is no longer a fixed number and follows the ledger's length. A rig that reads verify's step lines must not hard-code the count.

@history_item n=15 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` is the relied count (`LedgerRow::relied()`), and exit 0 means every relied row passed (row contract/viola-verify-output/exit-0).

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS) 24 relied rows on every build, and the stop is SIGINT/SIGTERM only. Linux is the one target the rig needs to cover.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the .cmd/.bat refusal leaves revive and verify) verify's exit-1 pairs are now not-found, version, fixture, stop and home. These are the refusals a rig-driven verify can meet.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree refusals of verify and the run and revive starts) a home, `ledger/`, `bin/`, `plugin/` or pinned copy that is not private is refused, exit 1, with no log and no child, and viola never changes permissions (rows contract/run-revive-start/home-tree-refused, contract/viola-verify-refusals/home-not-private). The rig's private scratch home and stamped-home copy must be owner-only, or every start is refused.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the six frame verbs' exit-21 instance pair) the frame verbs refuse widened instance state files with exit 21 before any log. Rig-owned instance directories have to keep owner-only modes.

@history_item n=20 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory refusal of the `run` and `revive` starts) a socket directory that cannot be trusted is refused with exit 1 before any snapshot, child or bind (row contract/run-revive-start/socket-dir-refused). The scope places test sockets under the short `/tmp` root, so the rig's socket directory must be owned by the user and private.

@history_item n=21 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (exit 21's sixth cause, `socket-dir-refused`, at the frame verbs) rows contract/frame-verb-hints/socket-dir-refused and contract/instance-unreachable-detail/socket-dir-refused: the same trap for every frame verb the rig drives.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's document carries its cause in `detail`) exit 21's `--json` detail is a closed cause list. A rig that reads `--json` refusals can tell the causes apart by that field (row contract/instance-unreachable-detail/*).

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `cli` catch site's `internal-error` document under `--json`) a caught failure under `--json` prints one stdout document with stderr empty (row contract/cli-json-mode/one-document).

@history_item n=24 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the exit-21 not-running hint now applies only when the instance is not running, and a second, server-verify cause was added. A rig that drives wrapped instances meets exit 21 with one of several hints (row contract/frame-verb-hints/instance-not-running).

@history_item n=25 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (`link` / `unlink` join the exit-21 `unable` lines) the not-running hint row now covers wait, last, link, unlink and send.

@history_item n=26 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — set wait/last's lines: every non-dialog kind takes the turn-ended form, `dialog unknown`, `waiting:` only on a terminal stderr, and the exit-21 pair (rows contract/viola-wait-output/*). The scheduled-wake carry reads `turn-ended` through `viola wait`.

@history_item n=27 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — added the `dialog-expired` wait line, and every kind without a dialog_id takes the turn-ended form (rows contract/viola-wait-output/dialog-expired, turn-ended). A rig that holds modal dialogs live can observe the expired hold.

@history_item n=28 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built, with `DIALOG unknown` cells (row contract/viola-list-board/dialog-unknown). `viola list` is the hint target the rig's exit-21 lines point at.

@history_item n=29 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `--json` is the machine view of every data verb, revive included (row decision/json-on-every-data-verb). It is the machine surface the rig should read rather than scraping human lines.

@history_item n=30 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the grouped `--help` table over served verbs, and the board's styling and unwrapped-name cut use the width from `host_size()`, never `COLUMNS` (rows contract/cli-verbs/help, decision/truncate-only-unwrapped-names, decision/plain-output-modes). A rig pty's size and TERM/NO_COLOR therefore shape what the screen model reads. The product gains no verb, so the help row stays unchanged.

@history_item n=31 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the `handoff:  link, unlink` help row built) the help table's five served rows. The rig adds no verb, so `help_lists_only_served_verbs` must stay green.

@history_item n=32 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut the web-spa surface to a stub, leaving the cli surface to stand alone (the origin of absence/web-surface). The rig adds no surface.

@history_item n=33 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — census 0.2.0: no web surface (absence/web-surface), flat verbs with no interactive prompt or TUI in a verb, and the Unix qualifiers dropped on the socket-directory refusals. The rig must add no verb and no surface to the product.

@history_item n=34 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master into rows, each verified against the code, in every area. Every picked row this chunk reads comes from that migration.
