group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pres»

- leaf: .claude/docs/security-summary.md:68   class: stale
  leaf-text: «PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin`, so no host value»
  source: .andromeda/security-plan.md:413 «launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root»
  replace-with: «PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`, so no host value»

- leaf: .claude/docs/security-summary.md:69   class: retired
  leaf-text: «Supply chain beyond cargo: the test-side `e2e-web/package-lock.json` (`@playwright/test` =1.63.0) has its own audit, `scripts/npm-audit.sh` (advisories + registry.npmjs.org-only sources) in `supply-chain` and weekly in `nightly.yml`; Node is a sha256-pinned official download (`scripts/install-node.sh`, ci.yml `NODE_PIN_*`), no toolchain action. The vendored»
  source: .andromeda/security-plan.md:286 «- The vendored Microsoft ConPTY binaries (`vendor/conpty/1.24.260710001/x64/{conpty.dll,OpenConsole.exe}`, nuget `Microsoft.Windows.Console.ConPTY` 1.24.260710001, licence MIT»
  replace-with: «Supply chain beyond cargo: the vendored»

- leaf: .claude/rules/security.md:30   class: retired
  leaf-text: «over the root and fuzz lockfiles; the test-side `e2e-web/package-lock.json` has its own audit, `scripts/npm-audit.sh` (npm advisories at every level + registry.npmjs.org-only sources; CI `supply-chain`, weekly `npm-advisories`); the vendored»
  source: .andromeda/security-plan.md:286 «embedded into the Windows x64 `viola` by `include_bytes!`) sit outside every cargo graph, so no lockfile audit reaches them. Their gate is `scripts/conpty-vendor.sh`»
  replace-with: «over the root and fuzz lockfiles; the vendored»

- leaf: .claude/rules/security.md:31   class: retired
  leaf-text: «rmcp features exactly `server` + `transport-io`; axum without `http2`.»
  source: .andromeda/security-plan.md:293 «  - `rmcp` denies `transport-streamable-http-server` and `auth`.»
  replace-with: «rmcp features exactly `server` + `transport-io`.»

- leaf: .claude/rules/security.md:35   class: retired
  leaf-text: «- Node is a sha256-pinned official download (`scripts/install-node.sh`; the `NODE_PIN_*` lines in ci.yml's workflow `env:`, parsed from the file text, are its only home), never a toolchain action or runner-image Node.»
  source: .andromeda/security-plan.md:327 «and `1.96` in the `msrv` job (selected with `RUSTUP_TOOLCHAIN=1.96`). No toolchain action is used.»
  replace-with: «»

- leaf: .claude/rules/security.md:36   class: stale
  leaf-text: «(`cc`, the `rust-toolchain.toml` channel, ci.yml's `test`-job line, `NODE_PIN_VERSION`)»
  source: .andromeda/security-plan.md:304 «its `tools` stage checks the host against CI's own pins — `cc`, the `rust-toolchain.toml` channel, and the cargo-nextest, cargo-mutants and cargo-llvm-cov versions it parses from ci.yml's `test`-job `tool:` line»
  replace-with: «(`cc`, the `rust-toolchain.toml` channel, ci.yml's `test`-job `tool:` line)»

- leaf: .claude/rules/security.md:36   class: stale
  leaf-text: «PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` — no host value crosses»
  source: .andromeda/security-plan.md:413 «launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root»
  replace-with: «PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin` — no host value crosses»
