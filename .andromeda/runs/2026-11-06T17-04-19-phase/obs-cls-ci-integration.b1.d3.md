@answer n=1 entity=ci-integration#1.1 v=yes
why: the `diag-<os>` upload and its scan-gated condition are named by the scope's gated `diag-<os>` uploads in step order (task item 7).
@answer n=2 entity=ci-integration#1.2 v=yes
why: the `harness-<os>` upload behind the `failure()` and scan-success gates is named by the scope's gated `harness-<os>` uploads (task item 7).
@answer n=3 entity=ci-integration#1.3 v=yes
why: the `junit-<os>` nextest JUnit upload is named by the scope and by task item 9 directly.
@answer n=4 entity=ci-integration#1.4 v=no standing=S4
why: the perf-artifact upload gating is the standing CI-gate upload convention (scan-gated, SHA-pinned); the scope names no perf upload and the chunk adds none.
@answer n=5 entity=ci-integration#1.5 v=no
why: the row records that v1 has no OTel trace exporter (N/A); the chunk adds no exporter and no rule of it is one the chunk's code must meet.
@answer n=6 entity=ci-integration#2.1 v=yes
why: the lint stage (clippy `disallowed-macros` and print bans, `scripts/lint-probes.sh`, G1, `cargo deny` with its Tokio, C-crate and banned-crate rules) is named by task items 5 and 8 and must be green.
@answer n=7 entity=ci-integration#2.2 v=no
why: the in-memory writer unit-test form is a re-check every change owes; the chunk adds no writer or writer test.
@answer n=8 entity=ci-integration#2.3 v=no standing=S1
why: the integration-test viola-home rule (homes under `target/e2e-home/`, covered by G2, G4, the scan and the upload) is the standing home convention S1.
@answer n=9 entity=ci-integration#2.4 v=yes
why: the E2E evidence through `agent-run logs` / `status` and the G2 and G4 gates is named by task items 7 and 10.
@answer n=10 entity=ci-integration#2.5 v=yes
why: the OS-gated test census in `tests/contract_os_gated_tests.rs` is the re-read check, and its fault on a `windows`-named gate predicate governs the Windows-remnant removal.
@answer n=11 entity=ci-integration#2.6 v=yes
why: `host.rs` (the `windows` test-host key) is edited, and task item 15 names the `run --mutants` survivor and build-failure rule that governs it.
@answer n=12 entity=ci-integration#3.1 v=yes
why: G1 (bare `#[instrument]`, with the PCRE2 ripgrep install step before G1 and G3) is named by task item 7 as a gate to re-run green.
@answer n=13 entity=ci-integration#3.2 v=yes
why: G2 (zero panics, probe and the `seam.rs` exemption) is named by task item 7 as a gate to re-run green.
@answer n=14 entity=ci-integration#3.3 v=yes
why: G4 (schema conformance against `diag-line.v1.json` and `diag-detail.v1.json`) is named by task item 7 as a gate to re-run green.
@answer n=15 entity=ci-integration#4.1 v=yes
why: the test-then-G2/G4-then-upload step order and `AGENT_RUN_KEEP_HOMES` in `ci.yml` are what task item 7's gates in their step order require to read the homes.
@answer n=16 entity=ci-integration#5.1 v=yes
why: the secret-scan timing after each job's end-to-end run and the kept homes are the scan that task item 7 names as a gate to re-run green.
