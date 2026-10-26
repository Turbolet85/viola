# Classified — obs · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · standing S7 · the secret-scan floor over every `diagnostics/` file (no `CLAUDE*` value, no content canary in a home-level line, 0600)
pii-scrubbing-compliance#1.2 · no · standing S7 · the secret-scan floor over every `diagnostics/` file (no `CLAUDE*` value, no content canary in a home-level line, 0600)
pii-scrubbing-compliance#1.3 · no · v1 has no signing credentials in its runtime, and the chunk adds no CI job, secret or upload (Not needed: CI artifact retention and upload)
pii-scrubbing-compliance#1.4 · yes · the verbs send, wait, last, answer, pause and release are reached whole and gain a pre-frame check with process-exit lines, so prompt text and last_assistant_message stay out of every home-level line (codes only)
pii-scrubbing-compliance#1.5 · no · Not needed: dialog content rules (no dialog flow changes), and the hook's refused arm logs nothing because its check precedes the open
pii-scrubbing-compliance#1.6 · yes · `snapshot.rs` and `replay.rs` are reached whole and hold the drift seam behind every checked snapshot read, and the readers' move onto `read_snapshot_or_replay` is an open P4 fork over that call
pii-scrubbing-compliance#1.7 · yes · `src/cmd/hook/statusline.rs` is reached whole, so its `statusline.shell_out` span must keep `statusline_command` out of home-level logs and record only `shell_exit_status` and `duration_ms`
pii-scrubbing-compliance#1.8 · yes · the reached `obs.rs` and the new refusal and exit lines (run's pinned-exe mismatch, the strict-modes `process-exit`, `plugin install`'s marketplace refusals) must stay path-free
pii-scrubbing-compliance#1.9 · yes · run's pinned-exe mismatch refusal, the run start sequence and the edited `pin_exe` must never log `pinned_bin` or an endpoint (Needs: the `pinned_bin` path is never logged)
pii-scrubbing-compliance#1.10 · yes · the new `strict-modes-failed` refusals carry codes only, with no pid, endpoint or start time (Needs), so typed `pid` and `started_at` never enter a detail code
pii-scrubbing-compliance#1.11 · no · no `claude agents --json` read is added (Not needed: parse-rejected handling), and the row rules live in `src/run/agents.rs`, which the chunk does not reach
pii-scrubbing-compliance#1.12 · no · `link` and `unlink` already run `check_instance` first (Not needed: E3), and their files and the wrapper's links module are not reached
pii-scrubbing-compliance#1.13 · yes · the newly refused CLI verbs add the closed detail `strict-modes-failed` at exit 21, which the closed kebab-case catalog must record (Needs)
pii-scrubbing-compliance#1.14 · no · the chunk adds no recorded fixture, the scrub is tests-owned, and obs adds no fixture content
pii-scrubbing-compliance#2.1 · yes · the scope's build-failure check for `diagnostics/` files at 0600, DACL-protected on Windows, is the 0600 clause this verification rules over
