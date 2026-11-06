@answer n=1 entity=obs-scope-summary#1.1 v=no
why: the session adds no product surface, verb, flag or env var, so the count of 5 telemetry surface entries holds (task untouched 1).
@answer n=2 entity=obs-scope-summary#1.2 v=no
why: the Cargo.toml change is only the portable-pty pin, so no crate is added or removed and the count of 6 holds.
@answer n=3 entity=obs-scope-summary#1.3 v=no
why: the text already states Linux only and CI on ubuntu-latest, which matches the session's ubuntu-latest-only matrix (task item 13).
@answer n=4 entity=obs-scope-summary#1.4 v=no
why: the session adds or removes no must-trace path or edge flow, and the statusline-rate-limits edge flow still exists, only its reading changes.
@answer n=5 entity=obs-scope-summary#1.5 v=no
why: the trigger count of 18 is unchanged, and the multi-platform exporter trigger is a re-read with no rule moved (task item 13).
@answer n=6 entity=obs-scope-summary#2.1 v=yes mode=stale
why: tests/support/home.rs is reached with plain_drive retired, and the tests' `<home>/diagnostics/` root is built in that helper, so its reading can have moved.
@answer n=7 entity=obs-scope-summary#2.2 v=no
why: the mcp stderr-when-unset rule lives in the mcp server, which the session does not change, and the reached mcp tests changed only for plain binary names.
@answer n=8 entity=obs-scope-summary#2.3 v=no
why: no event kind is added or removed (task untouched 5), so the event enum extensions stay as stated.
@answer n=9 entity=obs-scope-summary#2.4 v=no
why: the corr propagation is unchanged (task untouched 5), and the hook.rs change is the exe-suffix and Git Bash cleanup, not corr timing.
@answer n=10 entity=obs-scope-summary#3.1 v=no
why: the clap 4.6.7, vt100 0.16.2, thiserror and anyhow pins are unmoved, since the Cargo.lock moves are portable-pty, nix, the serial family and pin-utils only.
@answer n=11 entity=obs-scope-summary#3.2 v=no
why: the crate's instrumentability is unchanged, as the session moves no surface of it beyond the cleanup.
@answer n=12 entity=obs-scope-summary#3.3 v=no
why: the session's error changes are the thiserror census wording and the send stderr warning removal, so the anyhow-only dispatch boundary and chain rule still hold.
@answer n=13 entity=obs-scope-summary#4.1 v=no
why: viola-core is not among the reached files and its pure type-crate role is unchanged.
@answer n=14 entity=obs-scope-summary#4.2 v=no
why: the not-instrumentable status of viola-core is unchanged, and no reached file alters it.
@answer n=15 entity=obs-scope-summary#4.3 v=no
why: the six UnreachableCause words stay as confirmed (task item 8 keeps the wording and only lifts marks), and this text carries no PROVISIONAL mark.
@answer n=16 entity=obs-scope-summary#5.1 v=no
why: the text names portable-pty with no pin, and the dependency stays the PTY seam source at =0.9.0.
@answer n=17 entity=obs-scope-summary#5.2 v=no
why: the viola-pty instrumentability is unchanged, since pty_backend stays openpty.
@answer n=18 entity=obs-scope-summary#5.3 v=no
why: the openpty OS API stays boundary-only and PTY bytes stay unlogged, as pty_backend stays openpty (task item 1).
@answer n=19 entity=obs-scope-summary#6.1 v=no
why: the IPC method set is unchanged, and send loses only its stderr warning (task untouched 7).
@answer n=20 entity=obs-scope-summary#6.2 v=no
why: the sync-only instrumentation rule holds, since the lock moves add no Tokio and the crate's instrumentability is unchanged.
@answer n=21 entity=obs-scope-summary#6.3 v=no
why: the Tokio ban holds, as the Cargo.lock moves are nix, the serial family and pin-utils, and none adds Tokio.
@answer n=22 entity=obs-scope-summary#7.1 v=no
why: the ndjson, atomic-write-file and sysinfo 0.39.6 pins are unmoved, and no Cargo.lock move names them.
@answer n=23 entity=obs-scope-summary#7.2 v=no
why: the instrumentability of this crate is unchanged, and the session moves none of its surface.
@answer n=24 entity=obs-scope-summary#7.3 v=no
why: the sync-only rule holds, since no Tokio is added to this crate's graph.
@answer n=25 entity=obs-scope-summary#8.1 v=yes mode=stale
why: tests/contract_fake_agent_drift.rs and src/bin/viola-fake-agent.rs are reached, and the new dialog_drift_names_a_changed_annotation contract test moves the serde_path_to_error drift-report reading this entity cites.
