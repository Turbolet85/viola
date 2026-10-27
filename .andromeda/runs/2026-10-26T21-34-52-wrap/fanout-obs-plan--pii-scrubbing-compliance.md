proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 38
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Add one item after line 38: - A strict-modes refusal of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`) carries no pid, path or endpoint and writes no line: it prints only the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and asserts no `cli-<name>.ndjson` line, on all three OSes (chunk 2026-10-26-home-and-code-bearing-file-integrity); the `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: null` and no line (`binary(handlers)`).'
    sidecar: 'pii-scrubbing-compliance: verification gains the strict-modes refusal item (six frame verbs and the MCP frame tools: fixed pair or document, no pid or path, no line; digit_runs scan, three OSes), chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Changes › Symbols (`src/cmd/client.rs` refuse_untrusted, `viola-mcp` tools.rs frame 337-339) and Coverage of new surfaces (refuse_untrusted: PII n/a, fixed pair, no path, `digit_runs` read; tests `binary(security_negatives_strict_modes)` three OSes; MCP `detail: null`, no line); the file states the matching server-verification item at line 38 but none for the new strict-modes refusals.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:85
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 38
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Add one item after the item above: - The content sink is opened only after its strict-modes check: `src/obs.rs` `check_diagnostics` runs `strict::check_diagnostics` on the existing instance `diagnostics/` folder and on the role or `detail-<process>.ndjson` file in `open_role_file` and `write_detail`, before `ensure_private_dir` narrows anything; a symlink, a folder another user may read (Unix group/other bit; Windows a read grant to another SID), or a Unix file with a group/other bit (Windows: the write rule) refuses the open as a failed open: the writer ends at `std::io::sink` and the detail line is dropped, so High-class content never lands in a file another user may read (unit cases obs.rs 610-673).'
    sidecar: 'pii-scrubbing-compliance: the detail-file sink guard (check_diagnostics before every role and detail open; a refused path drops the line), chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Changes › Symbols (`src/obs.rs`: new private `check_diagnostics(dir, file)` 69-76 in `open_role_file` and `write_detail`; a refusal is an io::Error, the writer ends at sink, a detail line dropped; `viola_state::strict::check_diagnostics` 127-145 rules) and Coverage of new surfaces (diagnostics opens, validation symlink + modes); rows 13-15 of the file route High-class content to the detail file but state no guard on that sink.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20
note: D-obs-pii, D-obs-instrumentation and D-obs-stack find no drift in this file - the chunk adds no logging of user data (every new refusal writes no line by the founder word inputs#I3 Q2; state-recovered and the sideload_fallback word strict-modes-failed are codes only), no dependency, and no Retired gap is named here.
