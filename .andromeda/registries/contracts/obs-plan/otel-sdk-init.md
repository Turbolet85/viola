### OTel SDK init

- **SDK packages:** **No OTel SDK in v1.** The OTel API role is filled by **tracing 0.1.44** (the facade in every instrumentable crate) and **tracing-subscriber 0.3.23** (the formatter and sink).
  - Researched and deferred to the root-bin edge only: `opentelemetry` / `opentelemetry_sdk` 0.33.0 and the `tracing-opentelemetry` 0.34.0 bridge. With no exporter they would add only dependency and cargo-mutants surface, and bridged span IDs would have no consumer.
  - If either is ever adopted, the Resource must be built with `Resource::builder_empty()`, never `Resource::builder()`: the latter's `EnvResourceDetector` reads `OTEL_*` env vars.
  - Cargo entry for every instrumentable crate: `tracing = "0.1.44"`.
  - Cargo entry for the root bin only: `tracing-subscriber = { version = "0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }`. `ansi` and `tracing-log` are off (D-11). The `chrono` feature is off too: it only enables `ChronoUtc` / `ChronoLocal`, and `MillisUtc` uses the root bin's direct `chrono` dependency (D-26).
- **Init order** (one `viola_obs_init(role)` in the root bin, run in obs-scope §3 order):
  1. The **first statement in `main`** is `std::panic::set_hook(viola_panic_hook)`. The hook writes to a `OnceLock<Arc<File>>` that is empty until step 4, so it never falls back to stderr.
  2. clap parse.
  3. Resolve the home (`--home` → grandparent of `VIOLA_DIR` → `~/.viola/`), the instance and the role (`run` / `hook` / `mcp` / `ui` / `cli`). The instance source is fixed per role:
     - `run`: its own `ViolaName` argument. An inherited `VIOLA_NAME` (a `viola run` started from inside a wrapped session) never overrides it, so a wrapper never writes into another instance's `run-<name>.ndjson`;
     - `hook` and `mcp`: `VIOLA_NAME` only;
     - `cli`: the caller's `VIOLA_NAME`, then the target argument (Trace context propagation);
     - `ui`: no instance. Its file name needs `<port>`, and the GUI port can come from `config.json` (security Data Classifications, config values). For `ui` only, the step-5 `config.json` parse therefore runs here and its result is reused at step 5. `<port>` resolves flag → `config.json` → `47319` (arch precedence flags > config > defaults). A `ui` panic during that parse falls in the §7 pre-init window (D-29).
  4. **Home strict-modes first, diagnostics init after** (overseer fix pass 2, B2). When viola creates the home (a not-yet-existing `--home`, as the tests harness passes), it creates it first: 0700 on Unix, and on Windows the explicit protected user + SYSTEM DACL that security requires for a `--home` outside `%USERPROFILE%`. The role's security strict-modes check then runs on the home (owner, mode / DACL, not a symlink; security `~/.viola/` access control). Nothing under the home, `diagnostics/` included, is created or opened before that check passes. A refused home gets no diagnostics line: the role exits with its security outcome (`hook` fails open with exit 0), and a panic in this window falls in the §7 pre-init window (D-29). Roles with no strict-modes check in security still get the diagnostics checks below. Only then create `<home>/diagnostics/` with `DirBuilderExt::mode(0o700)` on Unix; on Windows it inherits the already-verified home DACL. Before the first write, `diagnostics/` and every opened diagnostics file (role file, and later the detail file) get the same checks as the home: on Unix, `lstat` owner == `geteuid()`, `mode & 0o077 == 0`, not a symlink (files opened with `O_NOFOLLOW`); on Windows, owner and DACL. A failed check is handled like a failed open (below), so a pre-existing foreign, permissive or symlinked diagnostics file is never written. Open the role file with `OpenOptions::new().append(true).create(true)` and `OpenOptionsExt::mode(0o600)` on Unix. If the open fails, the writer becomes `std::io::sink` (or stderr for `mcp` only, D-09). The instance detail file `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` is **not** opened here. It is opened lazily, with the same append / 0600 / 0700 rules, by the first content-bearing record (chain, drift report), and the panic hook opens it itself when it is still closed. A failed detail open drops only the detail line, never the home-level line, and writes nothing to stderr. The error-free `hook` path therefore keeps exactly one log `open` (D-29).
  5. Parse `<home>/config.json` once for the process. `diagnostics_level` falls back to `"info"` when the file is missing or unreadable or the value is unknown. Then install the subscriber with an explicit writer (below). No subscriber exists during the parse, so its skipped-key count and failure `detail` are held in memory. A panic during the parse is still logged, because the role file exists after step 4.
  6. `obs_event!(ProcessStart, …)`, then `parse-rejected{parser:"config-json", detail, count}` if the step-5 parse failed or skipped keys (D-28).

  Per-role anchors:
  - `run`: steps 1–6 finish **before** `run.collision_check`, so exit-1 causes are logged. The panic hook is live before `pty.spawn`.
  - `ui`: before `axum::serve` binds `127.0.0.1:<port>`.
  - `mcp`: before the rmcp 3.4.1 stdio transport starts.
  - `hook`: one `config.json` read plus exactly one log `open`, then appends, which fits the `max < 1.0 s` gate.
- **Init body sketch (≤ 5 lines):**
  ```rust
  let sub = tracing_subscriber::fmt().json().flatten_event(true).with_current_span(false)
      .with_span_list(false).with_ansi(false).log_internal_errors(false).with_max_level(tracing::Level::DEBUG)
      .with_timer(MillisUtc).with_writer(diag_writer /* BoxMakeWriter over Arc<File> | sink */)
      .finish().with(viola_targets(cfg.diagnostics_level)); // filter::Targets = the only configurable level gate
  tracing::subscriber::set_global_default(sub)?;
  ```
  `MillisUtc` is a crate-local `FormatTime` that writes `chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)`. It is needed because the default `SystemTime` timer writes microseconds and `ChronoUtc::rfc_3339()` writes `+00:00` with automatic precision (obs-research, tracing-subscriber, finding 1).
