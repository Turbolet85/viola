@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Surface: cli → Platform
change: fct-7z5o76y5 reads «Linux terminals (the core on any current distribution, CI on `ubuntu-latest`)»; the Windows 10/11 (Windows Terminal, conhost) and macOS terminals leave, recoverable at the tag `multi-os-final`.
sidecar: §Surface: cli Platform narrowed to Linux terminals — the OS target «Windows first, macOS and Linux» retired by the founder's rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6), chunk 2026-11-05-linux-only-code-and-ci.
rationale: report fct-tulramlm (Retired) retires the OS target «Windows first, macOS and Linux» for «Linux only: the core on any current distribution (CI on ubuntu-latest)»; fct-pdxuthnr lists design-system §Surface: cli Platform among the expected amendments.
@proposal n=2 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Surface: cli → Toolkit / Framework
change: fct-2wyyc34q is removed: no Windows VT enabling step and no windows-sys dependency remain; the toolkit is clap, std `IsTerminal` and the hand-written SGR module.
sidecar: §Surface: cli Toolkit loses the windows-sys `SetConsoleMode` VT-enable bullet — the Windows VT enable and the windows-sys dependency removed (chunk 2026-11-05-linux-only-code-and-ci).
rationale: report fct-xxutheix removes `src/human/style.rs`'s VT enable (the `vt` fact); fct-www7rgz5 removes `windows-sys`; fct-tulramlm retires «the Windows VT enable» and «the windows-sys dependency».
@proposal n=3 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-kbhpzsoq
section: §Surface: cli → ### Tokens (platform-specific)
change: the colour decision order ends at rung 6 «stdout is not a terminal»; rung 7 «Windows VT enable failed» (fct-jijjnoyy) is removed.
sidecar: Colour decision order loses rung 7 «Windows VT enable failed»; rungs 1–6 unchanged (chunk 2026-11-05-linux-only-code-and-ci, deviation 8).
rationale: report fct-tulramlm retires «the Windows VT enable (decision rung 7 «Windows VT enable failed»)»; fct-kbhpzsoq (deviation 8): `human/style.rs` lost the `vt` fact, rungs 1–6 and the depth rule unchanged.
@proposal n=4 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Surface: cli → ### Tokens (platform-specific)
change: the depth rule's 256-colour line (fct-vzhczpnh) reads «256-colour if `TERM` contains `256color`;» — the clause «or on Windows once VT enabling has succeeded» is dropped.
sidecar: Colour depth rule loses the Windows depth branch (256-colour once VT enabling succeeded) — retired with the Windows VT enable (chunk 2026-11-05-linux-only-code-and-ci).
rationale: report fct-tulramlm retires «the Windows depth branch»; fct-xxutheix removes `src/human/style.rs`'s Windows depth rows (`colorterm_on_windows` → `colorterm_without_term`).
@proposal n=5 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Surface: cli → ### Platform-Specific Notes
change: the «conhost without VT falls back to no colour» note (fct-jphpatxv) is removed; no Windows console remains a platform.
sidecar: Platform-Specific Notes lose the conhost-without-VT note — Windows terminals and the VT enable retired (chunk 2026-11-05-linux-only-code-and-ci).
rationale: report fct-tulramlm retires the Windows VT enable and the Windows OS target; fct-pdxuthnr names Platform-Specific Notes among design-system's expected amendments.
@proposal n=6 detector=D-design-cli-wording severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: §Surface: cli → ### Component Patterns
change: `revive`'s `--json` refusal codes (fct-2t54hjhs) are ten — `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; `batch-script-child` is dropped.
sidecar: revive's closed --json refusal codes 11 → 10, `batch-script-child` removed with `AgentError::BatchScriptChild` (chunk 2026-11-05-linux-only-code-and-ci, inputs#I2).
rationale: report fct-tjt6dmfu: revive's closed `--json` refusal codes 11 → 10 (`batch-script-child` gone); fct-tulramlm retires `batch-script-child`; fct-25zti4qi: revive's codes ten, pinned in tests/cross_exit_causes.rs.
@proposal n=7 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Typography
change: fct-prvstcem reads «Bahnschrift is Microsoft's DIN 1451, a Windows face; Viola targets Linux only, so the DIN choice reaches a render through the Linux stack» then its DIN rationale unchanged — the clause «It ships with Windows 10 1709+, and Windows is the first target» is dropped.
sidecar: Typography rationale drops «Windows is the first target» — the OS target is Linux only (founder rulings 2026-11-04, chunk 2026-11-05-linux-only-code-and-ci).
rationale: report fct-tulramlm retires «Windows first, macOS and Linux» for Linux only, the 0.2.0 window features on Hyprland; fct-tjt6dmfu os-target class «Windows first» is stale (design 2 sites).
@proposal n=8 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Typography
change: fct-ithdbbjj reads «Both faces are Windows-only and Viola targets Linux only (the 0.2.0 window features on Hyprland), so the Linux fallback stack is the rendering a 0.2.0 frontend draws; the stacks end at the generic `sans-serif` / `monospace`, never `system-ui`» — «D3 makes macOS and Linux supported» is dropped.
sidecar: Typography's «D3 makes macOS and Linux supported» re-worded to the Linux-only target; the Linux stack is the rendered one (chunk 2026-11-05-linux-only-code-and-ci).
rationale: report fct-tulramlm retires the three-OS target; fct-tjt6dmfu os-target class «Windows, macOS and Linux» is stale, re-worded to the Linux statement, the claim it carries kept.
@proposal n=9 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: §Typography
change: fct-nqpspgzt reads «Each role token's stack ends at `sans-serif` / `monospace`; strip and tape columns are sized in `ch` of `--font-field`, so the Linux render (DejaVu Sans Mono, wider than Cascadia Mono) is the case the layout must hold without horizontal scroll» — «names a face per OS (Windows, macOS, Linux)» is dropped.
sidecar: Typography's per-OS stack sentence re-worded to the Linux render on a Linux-only product (chunk 2026-11-05-linux-only-code-and-ci).
rationale: report fct-tjt6dmfu os-target class («Windows, macOS and Linux») stale on a Linux-only product; fct-tulramlm retires the Windows and macOS target.
@remark
detail-file hits for the cascade (surface: fct-45kqhafb .cmd/.bat pair and batch-script-child, fct-sx3u7aez relied 21 on Windows, fct-a4ss54r5 / fct-pxdkcmfq macOS no-peer-pid, fct-kn56sdzi GetConsoleScreenBufferInfo, fct-ypei347w macos-latest, fct-mep5dn6i, fct-sedcqxww, fct-bctn63fi, fct-nmng4ymy «every OS»; typography: fct-64jvg77y / fct-gt53qtga / fct-ui2v5h36 per-OS columns); body label fct-fjvugaic «Per-OS fallback stacks» follows the typography detail's outcome; fct-7u3hdiu2 Git Bash note left, its warning stays per report fct-xxutheix (runs on Linux).
