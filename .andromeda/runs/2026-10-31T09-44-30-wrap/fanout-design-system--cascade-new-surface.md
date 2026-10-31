proposals:
  - detector: D-new-detail
    severity: warning
    fact: F7
    file: .andromeda/design-system-detail/surface.md
    near: 156
    section: "## Surface: cli › ### Component Patterns"
    change: "- Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text."
    sidecar: "Surface: cli, exit codes: the shape of the `--json` failures the witness drives recorded, chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "The report's Outcome reads each CLI `--json` failure the witness drives as one document, literal exit, stderr empty, no ESC, clean, met by `cli_error_documents_hold_no_path_or_upstream_text`; no standing text of the file states this shape across the failures."
note: "F8 is no home here: its census of error writers belongs to test-plan's §6 Contract suite, where the report's `new-binaries` fact says no master names the binary yet."
