# security extract

## Relevance
Relevant. This chunk carries out two bootstrap phases, security-plan §Bootstrap phases `logging-redaction-wire` (the consolidated NEVER-log floor) and `error-sanitization-wire`, for the two product crates at HEAD: the root `viola` bin and `viola-core`.

## Constraints
1. **The NEVER-log floor.** Per security-plan §Bootstrap phases → `logging-redaction-wire` (first bullet), no sink at HEAD may write any of these:
   - the GUI token, the launch URL or `.url` contents, or the `Cookie` header;
   - `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, or any other R8-stripped `CLAUDE*` value.

   The sinks covered are role/log files, stderr, `events.ndjson`, snapshots, `diagnostics/` and fixtures. §Secret Management (Storage) says the same. The R8 strip is not wired yet (see amendment history). Because of that, whether any existing span, `obs_event!` or panic-hook capture can pick up the wrapper's inherited `CLAUDE*` env today is a question for research.
2. **Content-bearing records go only to the detail files.** Per `logging-redaction-wire` (Scope bullet) and §Data Protection → Logs, only `instances/<name>/diagnostics/` may receive:
   - user content (prompts, `last_assistant_message`, plan and question text);
   - tool `input`;
   - the `statusline_command` string;
   - serde_path_to_error drift reports (paths and values).

   They must never reach stderr log or error lines or CLI `--json` error output. The same bullet exempts the contract payloads that carry content by design (`events.ndjson`, `wait`/`last` results, `statusline_command` in `snapshot.json` and `settings.json`). Do not over-redact those.
3. **anyhow chain handling.** Per §Bootstrap phases → `error-sanitization-wire` (anyhow bullet) and §Error Handling → Internal logging:
   - Chains stop at the `viola` bin's human-mode stderr.
   - A serde_json or serde_path_to_error source, or any other source whose `Display` can quote upstream text, is mapped to a fixed message before it joins the chain.
   - The full error is written only to `instances/<name>/diagnostics/`, and only when an instance resolves.

   The catch site in `main` needs a defined behaviour for the case where no instance resolves: stderr gets the fixed message only, and the full chain goes nowhere else.
4. **Fixed `Display` on error types.** Per §Error Handling → External responses and `error-sanitization-wire` (first bullet), the `thiserror` `Display` on every `<Crate>Error` enum uses fixed messages. `CoreError` is the one at HEAD. It must not interpolate upstream text or absolute paths, and variant fields that hold paths or payloads are excluded from `Display`.
5. **Operational metadata stays out of external errors.** Per `logging-redaction-wire` (Operational-metadata bullet, TMS §1), absolute paths (they carry the OS username, the only PII in scope) and internal type names never go into CLI `--json` error output. They may go to `diagnostics/`. This applies to the `ConfigRejection::Unreadable` report in the folded `read_diagnostics_level` fix: its fixed code must not name the `config.json` path. §Error Handling → Error format applies the same no-path rule to `state-unreadable`.
6. **File modes and the hook rule.**
   - Per `logging-redaction-wire` ("`diagnostics/` files are 0600") and §Data Protection → Logs, detail files are 0600 inside 0700 directories.
   - Per §Error Handling → Internal logging and §Anti-Patterns → Logging, `viola hook` never writes to stderr and never exits non-zero. If `hook` reaches the shared `main` catch site, the new routing must keep exit 0 with an empty stderr. Whether `hook` goes through that catch site at HEAD is a question for research.
7. **No switch can widen or disable redaction.** Per §Anti-Patterns → Universal (the `config.json` / `VIOLA_*` / CLI-flag ban) and §Input Validation (Configuration values row: `diagnostics_level` is the closed set `info | debug`), no config key, env var or flag can do this. The ban's list does not name redaction explicitly; the scope's extension to redaction follows its principle that env vars are not a configuration channel. `debug` level must not unlock any value on the floor.

## Patterns to follow
- **The detail sink.** Route full chains and drift reports through the shipped detail sink (`viola::obs::write_detail` / `detail_line`, 0600). This is the only destination security-plan §Data Protection → Logs and §Error Handling → Internal logging permit for full error detail.
- **Fixed messages at the source.** Map a serde source to a fixed message at the point where it would join the chain (per `error-sanitization-wire`). Do not scrub the rendered string afterwards.
- **Closed codes in external errors.** External error surfaces carry only closed codes and fixed strings (§Error Handling → External responses). This is the model for the `ConfigRejection` codes, and for the mechanism that later crates (`viola-channel`, `viola-mcp`, `viola-ui`) will reuse.
- **veil `Redact` ownership.** The veil `#[derive(Redact)]` layer, with the `toggle` feature banned, belongs to obs. security-plan §Dependency Security (the `deny.toml` additions heading) credits the `veil` feature ban to obs. security-plan §Data Protection → Logs does not name veil, although the scope cites it there. Take the derive's rules from obs-plan §6, not from this plan.

## Anti-patterns to avoid
- **anyhow chains with serde sources on stderr.** Never print a chain to stderr that still contains a serde_json or serde_path_to_error source. Never echo drift reports or chains into structured error output (§Anti-Patterns → Logging, first bullet).
- **Messaging token and socket values.** Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log, stderr or `diagnostics/` (§Anti-Patterns → Secrets, first bullet).
- **Other bans in §Anti-Patterns → Logging:**
  - no `diagnostics/` files readable by other users;
  - no absolute paths in any external error;
  - tracing-subscriber must be `>=0.3.20` if it is used.

## Contract bindings
- **security ↔ obs.** Obs owns the logger, the format and the redaction mechanism: skip-all `#[instrument]`, veil `Redact`, `schemas/diag-detail.v1.json` with `chain` and `drift_report`. Security owns only the floor that mechanism must satisfy (per §Bootstrap phases → `logging-redaction-wire` preamble: "Security floors that obs must satisfy"). The same preamble requires that §Data Protection → Logs, §Error Handling and §Anti-Patterns → Secrets / → Logging must not diverge from this consolidated list.
- **security ↔ tests.**
  - The mechanical proof of the floor (the secret-scan canary, the G1 bare-instrument check, the print bans) is the route entry "Observability gates", out of scope here. This chunk's tests should still plant a canary value and assert it is absent.
  - Test fixtures must use synthetic values only (§Data Protection → Repository fixtures; §Anti-Patterns → Data Protection, fixtures ban).
- **security ↔ arch.** The R8 `CLAUDE*` strip is owned by route entry "PTY wrapper on Windows" (per the amendment history). This chunk only makes sure the sinks cannot carry those values.

## Acceptance criteria contributions
- A test sets a canary value in `CLAUDE_CODE_MESSAGING_TOKEN` / `CLAUDE_CODE_MESSAGING_SOCKET` / another `CLAUDE*` var, then runs `viola` verbs, including an error path. The canary is absent from every role file, stderr, `events.ndjson`, snapshot and `diagnostics/` file. (per security-plan §Bootstrap phases → `logging-redaction-wire`; §Anti-Patterns → Secrets)
- A forced serde_json or serde_path_to_error failure that quotes a marker value is tested for an instance-resolved run:
  - the marker is absent from stderr and from `--json` error output;
  - the full chain or drift report, marker included, appears only in `instances/<name>/diagnostics/detail-<process>.ndjson`, and that file's mode is 0600 on Unix.

  (per security-plan §Bootstrap phases → `error-sanitization-wire`; §Anti-Patterns → Logging)
- Every `<Crate>Error` `Display` at HEAD, and the `ConfigRejection` reports, render with no absolute path, no home or username substring and no upstream text. A test builds each variant with a path- or payload-bearing field and asserts this. (per security-plan §Error Handling → External responses; `logging-redaction-wire` operational-metadata bullet)
- Setting `diagnostics_level: debug`, or any `VIOLA_*` env var or flag, does not bring any NEVER-log value into any sink. (per security-plan §Anti-Patterns → Universal, config/env/flag ban; §Input Validation Configuration values row)

## Relevant amendment history
- **2026-09-24-diagnostics-plane** (§Input Validation, Configuration values row + Constants):
  - The `config.json` row now names the closed `diagnostics_level` (info | debug; any other value falls back to info and is reported `parse-rejected`) and the `MAX_FRAME` cap. It places the read in root-bin `viola::obs`.
  - Why: that chunk shipped the read that this chunk's folded mutation fix (`read_diagnostics_level`, `Unreadable` branch) touches. The fix must keep the closed set and the fallback-to-info behaviour.
- **2026-09-24-three-os-ci-headless-harness-skeleton**, rejected entry (the interim `--home` and R8-strip Decisions-Log entries were not added):
  - The walking-skeleton `viola run` spawns the child with the full inherited environment. The R8 `CLAUDE*` strip is deferred to route entry "PTY wrapper on Windows", and §Secret Management stays as the target.
  - Why it matters here: until the strip lands, the `CLAUDE*` values are live in the wrapper process at HEAD. The floor must therefore be enforced at the sinks, not assumed from the strip.
