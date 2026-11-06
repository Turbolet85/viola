@proposal n=1 detector=D-retired severity=warning file=telemetry-strategy anchor=fct-n6l5o3fm basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 2. Telemetry Strategy
change: end the row's last sentence at «The "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN», dropping «on Linux, Windows and macOS (macOS compares no peer pid, the recorded gap, so there the word means an euid mismatch alone)»; the rest of the row unchanged
sidecar: telemetry-strategy Vector 4 row — the server-verify-failed line's Windows and macOS scope and the macOS no-peer-pid carve-out retired (Linux only, founder rulings 2026-11-04, inputs#I1)
rationale: the report's Retired bullet removes the macOS no-peer-pid carve-out with Windows and macOS; Symbols / APIs: server_verdict / verify_server lose peer_pid_reported and the macOS Matches arm, peer_euid_admitted is observed == Some(own)
@proposal n=2 detector=D-retired severity=warning file=telemetry-strategy anchor=fct-ne5xp5cg basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 2. Telemetry Strategy
change: drop the item «§4 Scenario 1 / §6 `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses: the child on the inbox ConPTY, exit unchanged)» from the Vector 7 row's mapping; every other item of the row unchanged
sidecar: telemetry-strategy Vector 7 row — the conpty companion strict-modes mapping (process-start.sideload_fallback strict-modes-failed) retired with the ConPTY sideload (inputs#I1, inputs#I2)
rationale: the report retires the ConPTY sideload, sideload_fallback and pin_companions; Schema / config: the sideload_fallback property leaves process-start in schemas/diag-line.v1.json
@proposal n=3 detector=D-retired severity=warning file=telemetry-strategy anchor=fct-6isebxud basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 2. Telemetry Strategy
change: the row reads «| multi-platform-exporter-compat | §3 Log file location (0600/0700), §3 Logging stack (concurrent-append check on ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 `diag-<os>` / `harness-<os>` artifacts (their `matrix.os` names on the one `ubuntu-latest` leg) |»
sidecar: telemetry-strategy multi-platform-exporter-compat row — home DACL clause, windows-2025 / macos-latest legs and the junit per-OS artifacts retired; ubuntu-latest alone (inputs#I1)
rationale: Harness / gate surface: ci.yml keeps strategy.matrix.os [ubuntu-latest] with every job and artifact name in its matrix.os form; Retired: the windows-2025 and macos-latest legs, the three junit-<os> artifacts and the Windows home owner/DACL arm
@remark
Out of group 2: the §1 body trigger multi-platform-exporter-compat (fct-mjdbrsjn three legs, fct-dao4nifd «on every OS») and Vector 7 signal fct-wrd7y3z2 (home DACL, conpty companion) carry the same retired claims in obs-scope-summary; whether the multi-platform-exporter-compat trigger stands at all on a one-OS target is the orchestrator's call; F1 and F2 belong to obs-scope-summary and log-coverage, not this group.
