proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 20
    section: "## 2. Telemetry Strategy"
    change: "In the panic bullet's bounded-exemption list, extend the refused-instance-check clause so it reads: and a refused instance check on either `viola revive` arm (start or `--list`), on `viola hook statusline` or on `viola link` / `viola unlink` (their one shared `check_instance`, run before `client::start` opens the `cli-<driven>` role file), or a refused `viola list` scan, which opens none either; the rest of the bullet unchanged."
    sidecar: "telemetry-strategy §2 panic exemptions: the refused instance check of the new `viola link` / `viola unlink` verbs joins the no-role-file exemptions beside `revive`, `hook statusline` and `list` (chunk 2026-10-26-explicit-session-links)."
    rationale: "The report's Changes (Symbols / APIs, `link_call` step 1) puts `check_instance` before `client::start`, so before any role file or log opens, a refusal leaving no line in any log; a panic in that window has no home-level role file, so it is one more bounded exemption of the zero-unlogged-panics rule; the report's Expected amendments name the obs refusing-verb enumerations (this file's line 20 among them) as the obs detector's to extend, and §10 exemption 7 / the pre-init window for the verbs' refused check."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41
note: "D-obs-stack and D-obs-pii read clean for this file (no dependency added; the verbs log codes only, no names in the wrapper's process log); no line of this file states the retired hold; the Vector 7 trigger row (line 33) maps sections only and needs no verb list, but the §10 detail file owns the matching exemption-list edit for the two verbs."
