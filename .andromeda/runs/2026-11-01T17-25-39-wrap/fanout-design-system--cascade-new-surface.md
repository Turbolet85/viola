proposals:
  - detector: D-new-detail
    severity: warning
    fact: F5
    file: .andromeda/design-system-detail/surface.md
    near: 140
    section: "## Surface: cli › ### Component Patterns"
    change: "- **A peer the wrapper refuses:** dropped before any frame, with no output of its own: nothing on the terminal the child holds, no line written, no wheel moved; the client reads end-of-file."
    sidecar: "Surface: cli gains the wrapper's silent drop of a refused peer: no terminal output, no line, no wheel moved, end-of-file at the client."
    rationale: "The report states the drop before any frame, reading end-of-file, writing no line and nothing on the terminal (report.md:13), that it writes nothing to the outer terminal and moves no wheel (report.md:75, report.md:120); no standing item of the file holds it, and the item it follows already states viola writes nothing past the terminal the child holds."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
