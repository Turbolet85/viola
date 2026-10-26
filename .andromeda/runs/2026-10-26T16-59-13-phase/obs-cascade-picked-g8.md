# obs · picked accounting · group 8 of 8

## Picked
pii-scrubbing-compliance#1.4 · not in scope · prompt and assistant text; the chunk changes no payload type and no send, wait or last content path
pii-scrubbing-compliance#1.6 · not in scope · drift reports come from the hook payload parse in `src/cmd/hook.rs`, a unit the chunk leaves alone (it edits only the `VIOLA_DIR` snapshot read); no parse seam is added
pii-scrubbing-compliance#1.7 · carried · `src/cmd/hook/statusline.rs` is reached whole (its `statusline.shell_out` span and its snapshot read of `statusline_command`), and `run`'s trusted-file checks cover the command's source
pii-scrubbing-compliance#1.8 · carried · the new checks run over home paths (`instances/<name>/`, `bin/`, `plugin/`, `ui/`, `conpty/`), and their refusals must keep every home-level line path-free
pii-scrubbing-compliance#1.9 · carried · the chunk edits `pin_exe` (re-hash before reuse, a hash-mismatch refusal) and `live_endpoint`; `pinned_bin` and `endpoint` stay out of every line
pii-scrubbing-compliance#1.10 · not in scope · standing S3 · a `detail` code is closed kebab-case and never holds a path or pid
pii-scrubbing-compliance#1.13 · not in scope · standing S3 · a `detail` code is closed kebab-case and never holds a path or pid
pii-scrubbing-compliance#2.1 · carried · the scan's 0600 assertion over every `diagnostics/` file is the owner-only home the chunk makes whole, and the server-identity scan rules over the same verbs the new check precedes
ci-integration#1.4 · not in scope · the perf artifacts' upload and scan gating; no upload or job is added, a re-check every change owes
ci-integration#2.3 · carried · the chunk's Windows `RUNNER_TEMP` negative (CARRY 2) and the seeded-home rework (CARRYs 6, 13) place test homes, and the row's one carve-out must record any home outside `target/e2e-home/`
ci-integration#2.4 · not in scope · standing S5 · zero unlogged panics over home-level role files (G2), no retry-once policy
ci-integration#3.2 · not in scope · standing S5 · zero unlogged panics over home-level role files (G2), no retry-once policy
ci-integration#3.3 · not in scope · standing S2 · every new field or `detail` code is in the §6 catalog and validates against `schemas/diag-line.v1.json` (G4, default-deny)
ci-integration#4.1 · carried · its step-1 home rule names the one home outside `target/e2e-home/`, the same count the chunk's test homes must keep
slo-invariants-telemetry-budgets#1.1 · carried · every new refused check before a role-file open (CLI frame verbs, hook's nine events, mcp frame tools, verify, run) is a no-line exemption the list must record
slo-invariants-telemetry-budgets#2.1 · carried · the strict-modes check added on every hook's path runs inside the timed `session-end` row
slo-invariants-telemetry-budgets#2.2 · carried · the new check sits on the spine hooks' timed path before the same snapshot read, bounded by the same gate and failing open
slo-invariants-telemetry-budgets#2.4 · carried · a perf home the new check refused would time the no-open arm and pass the gate vacuously
obs-anti-patterns#1.1 · carried · the chunk's new refusals (pinned-exe mismatch, the newly checked verbs) write through `human::refuse`, whose recorded caller counts move with each new site
