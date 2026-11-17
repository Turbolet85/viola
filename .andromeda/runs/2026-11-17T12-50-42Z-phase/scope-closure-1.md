old: at `4ef8f4b9`, nothing wider (the overseer's phase directive, inputs#I1 item 1 [inferred]).
new: at `4ef8f4b9`, nothing wider (the overseer's phase directive, inputs#I1 item 1).
old: - [inferred] The red's cause lies in the advisory database, not in this repository's history: no manifest, lock, deny
new: - The red's cause lies in the advisory database, not in this repository's history: no manifest, lock, deny
old:   at 2026-11-17T08:47:19Z (the CARRY's measured claim, kept verbatim in substance; P3 re-reads it).
new:   at 2026-11-17T08:47:19Z (re-derived at P3: `git diff --stat 2dc9b58d 0651903c` over the manifests, locks, deny configs and `.github/` prints nothing, nor does `0651903c..4ef8f4b9`; `cargo deny check advisories` at cargo-deny 0.20.2 reproduces the one error on this host).
old: - [inferred] Every config and job that reads advisories is re-measured at P3 from the code and the failing run's log:
new: - [premise-corrected: `deny-sync.toml`'s runs are `check bans` only and read no advisory; the fuzz audit reads the root `deny.toml` and its graph holds no chrono] Every config and job that reads advisories, measured at P3 (research «Measured»): the root graph (`cargo deny --format json check` in CI `supply-chain`, `cargo deny check advisories` in `nightly.yml`) fails on RUSTSEC-2026-0543 and one ignore for it makes it green; the fuzz audits (`supply-chain`'s `check advisories sources`, `nightly.yml`'s `check advisories`) read the root `deny.toml`, pass now and pass with the ignore, which there reads one more `advisory-not-detected` warning; the five per-sync-crate `deny-sync.toml` runs read bans only. Originally listed:
old:   amendment and edits no master (the CARRY; inputs#I1 item 3 [inferred]).
new:   amendment and edits no master (the CARRY; inputs#I1 item 3).
old:   once a safe upgrade exists, if the tool reads that) (inputs#I1 item 4 [inferred]).
new:   once a safe upgrade exists, if the tool reads that) (inputs#I1 item 4).
old: - [inferred] Uses the tie (`HARNESS_TIED_TEST_CHILD`, `ROOT_TIED_TEST_CHILD`) and the census the last chunk built; no
new: - [premise-corrected: the last chunk's census (`tests/cross_killed_test.rs`) walks the root `tests/` alone and counts `Command::new(` a path ending `VIOLA`, so it reaches no viola-e2e `BootOptions`; and the stamping verify's SIGKILL tie leaves its `<cwd>/.viola-verify-<pid>*` dirs, its Run A dir and its `ledger/probes/<pid>/` behind, since a SIGKILL runs no drop guard — research «Measured»] Uses the tie the last chunk built, with the stamping verify's parent-death signal SIGTERM so its own stop removes its dirs and ends its children (the supervisor's stays SIGKILL), and a census of viola-e2e's own boots beside the root one; no
old:   claimed so the wrap can lift the narrowing, never weaker than its text.
new:   claimed so the wrap can lift the narrowing, never weaker than its text. [premise-corrected: v2-37 reads `verified`, claimed by chunk 2026-11-17-booted-wrapper-does-not-outlive-its-test, and the ledger tool refuses a verified cap's claim; its new witnesses ride a dated ledger note at P5 naming them, and the narrowing's lift is the wrap's]
old: - No live `claude` start; no mutation run (inputs#I1 item 6 [inferred]).
new: - No live `claude` start; no mutation run (inputs#I1 item 6).
old:   short `/tmp` root (inputs#I1 item 8 [inferred]).
new:   short `/tmp` root (inputs#I1 item 8).
old:   select at least one test (inputs#I1 item 7 [inferred]).
new:   select at least one test (inputs#I1 item 7).
old:   ci#44084919060 completed/failure — failed 1: supply-chain (failure). [inferred] Its failing subject IS this chunk's
new:   ci#44084919060 completed/failure — failed 1: supply-chain (failure). Its failing subject IS this chunk's
old:   part A: folded here, resolved by P3 against that run's log.
new:   part A: folded here; VERIFIED against that run's log (job 134044722633): its one error diagnostic is `unmaintained` RUSTSEC-2026-0543, no patched version; the fuzz step did not run, the root step failing first.
