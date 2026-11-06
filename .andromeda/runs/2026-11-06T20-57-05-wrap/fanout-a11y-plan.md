@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §4 ARIA Patterns & Roles
change: In fct-4ktfiloi the tui row's automated reach reads «portable-pty outer-PTY boundary driver (portable-pty `=0.9.0`): zero viola bytes, keystrokes unblocked», the «inherited `=0.8.1`» qualifier gone.
sidecar: §4 tui tool-reach row — portable-pty `=0.8.1` retired; the outer-PTY boundary driver runs on `=0.9.0` (2026-11-06-version-done-check).
rationale: The report's Dependencies bullet (fct-xc746owh) moves portable-pty `=0.8.1` → `=0.9.0` and its Retired bullet (fct-uufjzm4k) retires the `=0.8.1` pin and its Windows rationale; the Outcome (fct-b2ryvv3f) has viola-pty 33/33 and the outer-PTY e2e binaries green on `=0.9.0`.
@proposal n=2 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: §3 → a11y-testing-tool-pick
change: In fct-dlpkde76 the tui bullet reads «portable-pty outer-PTY boundary driver on the workspace's exact `=0.9.0` (Decisions Log D-A11Y-13)», and its inherited-risk sub-bullet (`=0.8.1` of 2023-03-13 behind the maintained 0.9.0 line) is removed, the risk gone with the move.
sidecar: §3 a11y-testing-tool-pick — the tui driver on portable-pty `=0.9.0`; the `=0.8.1` inherited-risk note retired (2026-11-06-version-done-check).
rationale: The report's Dependencies bullet (fct-xc746owh) moves portable-pty to `=0.9.0`, the maintained line the risk note contrasted, and its Retired bullet (fct-uufjzm4k) retires the `=0.8.1` pin; the note now states a gap that no longer exists.
@remark
Detectors D-a11y-surface and D-a11y-obs-schema hold: the report adds no UI element (Coverage of new surfaces, fct-lxy7j3im: a11y n/a) and no schema or log field (fct-zmvefc57, fct-wo436rhb). The detail file a11y-scope-summary fct-twqx5x46 still names the portable-pty `=0.8.1` driver — a standing-group edit left to the cascade.
