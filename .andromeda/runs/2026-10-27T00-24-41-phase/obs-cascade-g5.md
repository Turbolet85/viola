# obs · cascade read · group 5 of 5

## Carried
- (.andromeda/obs-plan-detail/error-capture-reporting.md:8) The scope leaves a linker flag or profile key open as a P4 fork on the measurement's basis; any such change must keep the panic detail line's raw frames resolvable offline against the pinned copy, which the race fix must also keep the binary that runs.
  > The backtrace is raw, never-symbolised frames from `src/panic_frames.rs` — `RtlCaptureStackBackTrace` + `GetModuleHandleExW`/`GetModuleFileNameW` on Windows, `libc::backtrace` + `dladdr` on Unix — up to 62 strings `0x<ip> <module path> base=0x<base> +0x<offset>` (or `0x<ip> ?`), enough for an offline resolver against the pinned copy.
  (.andromeda/obs-plan.md:517)
  > - When an instance resolves, it also writes the payload and a raw, never-symbolised backtrace as one line to `detail-<process>.ndjson`, not gated by `RUST_BACKTRACE`, because env vars are not a configuration channel.
- (.andromeda/obs-plan-detail/error-capture-reporting.md:19) Any profile key the reproducible-build measurement leads the founder to change must keep `panic = "unwind"` in every profile, the release profile `scripts/release-check.sh` builds included, or every catch site becomes an abort.
  > **Unwinding is required:** every Cargo profile keeps `panic = "unwind"`. `panic = "abort"` turns every `catch_unwind` into an abort, so `hook` would die by SIGABRT instead of exiting 0. Gate G3 (§9) enforces this.
  (.andromeda/obs-plan.md:519)
  > every Cargo profile keeps `panic = "unwind"`, enforced by G3.
- (.andromeda/obs-plan-detail/ci-integration.md:51) G3 must keep passing over whatever profile key, rustflag or cargo config the measurement or a founder-ruled build change touches.
  > # G3: no abort panic strategy (profile `panic` key, `-C panic` rustflag, CARGO_PROFILE_*_PANIC env) in Cargo.toml, cargo config or workflows (§7); default regex engine; only rg exit 1 passes
  (.andromeda/obs-plan.md:610)
  > - **G3, no abort panic strategy:** no `panic` set to abort in Cargo.toml, cargo config or workflows; only rg exit 1 passes.
- (.andromeda/obs-plan-detail/ci-integration.md:39) A path remap the measurement may motivate changes what `panic_location` reads; the seam's forced panic must still read exactly `src/cmd/hook/seam.rs:<digits>`, compared as a whole string, and no build change may inject CI or env metadata into a product line.
  > One exemption, ratified by the founder live on 2026-09-28 (security-plan Decisions Log): a line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` — the `fake-agent` seam's forced panic — is not counted; the file is compared as a whole string, never a prefix, suffix, regex or home path.
  (.andromeda/obs-plan.md:606)
  > - **G2, zero panics:** `scripts/g2-zero-panics.sh`, fail-closed, `--probe` first, in the `test` and `perf` jobs, over the home-level role files only; one exemption, the `fake-agent` seam's forced panic at exactly `src/cmd/hook/seam.rs:<digits>`, compared as a whole string.
  (.andromeda/obs-plan.md:612)
  > **CI-specific resource attributes:** none are read by the product. Env vars are not a configuration channel, so `deployment.environment`, `ci.run.id` and `git.commit.sha` are **never** injected into viola log lines.
  (.andromeda/obs-plan.md:743)
  > - NEVER inject CI env metadata into product log lines. The product never reads env as configuration.
  (.andromeda/obs-plan.md:758)
  > - NEVER resolve service identity at runtime from env vars (`OTEL_SERVICE_NAME`, `OTEL_RESOURCE_ATTRIBUTES`). `"viola"` and `env!("CARGO_PKG_VERSION")` come from viola-core constants.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) An io error the absent-copy write raises across the `viola-state` seam is logged once with a closed code and no path, its chain only in the detail file; a path remap must keep `panic_location` workspace-relative.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
  (.andromeda/obs-plan.md:533)
  > - Module-boundary errors (Standard+): every typed error that crosses a crate seam ends in a logged `process-exit{detail}`, `channel-response{result_class:"error", error_code}`, `hook-decision{detail}` or `send-refused`. Each carries codes only, at `WARN`/`ERROR` per Section 6.
  (.andromeda/obs-plan.md:643)
  > - **Module-boundary error logging:** every typed error that crosses a crate seam is logged once at `WARN`/`ERROR` with a closed `detail` / `error_code`. The full chain goes only to the instance detail file.
  (.andromeda/obs-plan.md:735)
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) The path the fix writes is the pinned copy, `bin/<key>/viola(.exe)`: no line the fix, a lost-race outcome or the witness adds may carry it.
  > | Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
  (.andromeda/obs-plan.md:557)
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged;
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:44) Whether CI's three OSes run the clean-build comparison is P4's; a comparison artifact the plan adds holds build paths and section dumps, so it is scanned before upload or dropped, and the build-id or symbol differences stay in the chunk's evidence.
  > The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped (operator ratification 2026-09-24, logged for founder review).
  (.andromeda/obs-plan.md:574)
  > - Unscanned uploads are admissible by content only: a member that could carry a host path or log content is scanned before upload, or dropped;
- (.andromeda/obs-plan-detail/error-capture-reporting.md:25) The fix may add a lock, wait or directory under the home on the pin path in `run` and `verify`; it must run after their home-tree checks, inside the post-init window, so a refused tree still writes no line.
  > `viola verify`'s home-tree check before its `cli` role file opens, and the home-tree check of `viola run` and `viola revive`'s start arm before the wrapper log opens; each of their refusals writes no line.
  (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:18)
  > on `viola verify`'s home tree (exit 1, the home pair) or on the home tree of `viola run` or `viola revive`'s start arm (exit 1, the home or the instance pair): it opens no role file and writes no line, so a panic there writes none, by §7's rule.
  (.andromeda/obs-plan.md:529)
  > - Before step 4 of init no file exists, so the panic hook writes nothing; the window holds clap parsing, home resolution and the instance and home checks that run before any role file is opened.
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:8) The two-sided race witness fixes a measured flake (6 of 24 boots), so it runs with no retry-once policy, waits on hook events and `events.ndjson` offsets, and a red it shows makes the chunk red until its cause is known.
  > Retry-once policies for panic handling are forbidden, in parallel with the tests' `retries = 0`.
  (.andromeda/obs-plan.md:662)
  > **Cross-input parallel:** the tests' zero-flakiness budget (`retries = 0`) ↔ obs' zero-unlogged-panics invariant. Retry policies mask real signals in both cases.
  (.andromeda/obs-plan.md:748)
  > - NEVER add retry-once policies for panics or flaky telemetry assertions.
  (.andromeda/obs-plan.md:645)
  > - **Deterministic emission:** events key on hook events and `events.ndjson` offsets. Nothing is derived from screen content, and no log-based wait relies on sleeps (creator explicit).
  (.andromeda/obs-plan.md:658)
  > - **Action threshold:** any nonzero count makes the chunk red until the root cause is fixed within that chunk (tests quarantine rule).
- (.andromeda/obs-plan-detail/ci-integration.md:22) The chunk edits `viola-state`, a sync crate; any wait, lock or dependency the absent-copy fix adds must keep its graph Tokio-free and inside the `cargo deny` bans.
  > `cargo check` of sync crates without Tokio; `cargo deny check` (ubuntu) incl. veil `toggle` ban and bans on `opentelemetry-otlp` / `opentelemetry-stdout` / `sentry` / `tracing-appender` / tracing-subscriber `env-filter` feature
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) The chunk edits `run`'s and `verify`'s pin calls, whose tampered-copy refusal goes through `human::refuse`; a refusal site the fix adds or removes (a lost race that cannot be settled) must be re-counted by the rule beside the recorded numbers, and any refusal it adds writes through `src/human.rs`, never a print macro.
  > callers `run` 7 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1, the one `refuse_untrusted` site `link` / `unlink` and the six frame verbs share; the rule: the `human::refuse(` call sites under `src/`)
  (.andromeda/obs-plan.md:717)
  > - NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates: they bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing; enforced by the clippy `print_stdout`, `print_stderr` and `dbg_macro` lints at `deny`, except `viola-harness`, the fake agent and the output modules that own `--json`, human CLI stderr and the hook decision body (below).

## Bindings
- **error-capture-reporting ↔ ci-integration**: the catch sites rest on `panic = "unwind"` in every profile (.andromeda/obs-plan-detail/error-capture-reporting.md:19), and G3 is what proves it over any profile key, rustflag or cargo config a build change touches (.andromeda/obs-plan-detail/ci-integration.md:51).
- **pii-scrubbing-compliance ↔ ci-integration ↔ slo-invariants-telemetry-budgets**: a path remap moves `panic_location`, which must stay workspace-relative (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) and must keep reading the seam's exact whole string G2 exempts (.andromeda/obs-plan-detail/ci-integration.md:39), or the zero-panics count G2 measures goes red.
- **pii-scrubbing-compliance ↔ error-capture-reporting**: the pin-write error crossing the `viola-state` seam is logged with a closed code and no `pinned_bin` (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18), its chain only in the detail file, beside the raw-frame detail line resolved against that same pinned copy (.andromeda/obs-plan-detail/error-capture-reporting.md:8).
- **error-capture-reporting ↔ slo-invariants-telemetry-budgets**: a home-tree refusal before the log writes no line on `run` and `verify` (.andromeda/obs-plan-detail/error-capture-reporting.md:25), which is exemption 8 of the zero-unlogged-panics invariant (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:18); the fix's added home writes come after both.
- **slo-invariants-telemetry-budgets ↔ the test plan**: the race witness's no-retry rule (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:8) is the obs side of the tests' `retries = 0` (.andromeda/obs-plan.md:662).
- **obs-anti-patterns ↔ the security plan**: the `human::refuse(` counts `run` 7 · `verify` 6 (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) record the tampered-copy start refusal the security plan's hash-before-reuse rule keeps unchanged.
