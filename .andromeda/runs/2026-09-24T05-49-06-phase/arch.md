# arch extract

## Relevance
Relevant. This chunk creates the workspace, toolchain pin, CI workflow, repository layout, the first new crate and bins, and new env vars and paths. All of these fall under arch §Infrastructure Patterns, §Module Boundaries and §Occupied Resources.

## Constraints
- **Workspace shape:** architecture §Established Decisions [Module Boundaries] and §Infrastructure Patterns "Build system" require:
  - a flat `crates/` layout;
  - a root `Cargo.toml` that is both the `viola` bin package and the `[workspace]`;
  - `resolver = "3"` and edition 2024;
  - `publish = false` on every member, including the root (also §Inherited Defaults "Publishability");
  - every third-party version pinned once in `[workspace.dependencies]`.

  `cargo install --path .` needs the bin at the root.
- **Toolchain floor conflicts with the plan.** architecture §Stack and Technologies and §Inherited Defaults state `rust-version = "1.89"` (MSRV 1.89, host 1.95). The project directory tree in §Infrastructure Patterns shows `rust-toolchain.toml` as `channel = "stable"`. The scope (F-19) sets the floor at 1.96 and pins one exact stable version. This chunk's 1.96 floor and exact pin supersede the plan, so arch §Stack, §Inherited Defaults and the directory-tree comment need an amendment at wrap.
- **Crate naming and new members:** architecture §Occupied Resources "Workspace crates" lists exactly `viola` plus `viola-core`, `-pty`, `-channel`, `-state`, `-agent-claude`, `-mcp` and `-ui`. §Conventions "Naming patterns" requires `viola-<area>` kebab-case names. The new `viola-e2e` crate follows the naming rule but is not in the Occupied list. It must be recorded at wrap. The same goes for the bins `viola-harness` and `viola-fake-agent` and the root package's `fake-agent` feature.
- **Error and dependency discipline:** architecture §Conventions "Rust error types" and §Established Decisions [Error Handling] require one `thiserror` enum per crate (`<Crate>Error`) and allow `anyhow` only in the root `viola` bin crate. If `viola-harness` (in `viola-e2e`) uses anyhow, it breaks that rule. §Cross-cutting Patterns "Tokio containment" requires std threads and blocking I/O for new code. The harness and fake agent should not pull in tokio.
- **Cross-platform from the first commit:** architecture §Design Philosophy ("Cross-platform from the first commit, Windows first"), §Established Decisions [Scale / Product] and [CI/CD], and §Cross-cutting Patterns "Cross-platform discipline" require:
  - children spawned directly, with no shell-out through `sh`, `bash` or `cmd`;
  - every OS-specific branch compiled and tested on its CI runner;
  - no reliance on POSIX-only behaviour.

  The `.sh` and `.ps1` shims are developer entry points. The Rust harness itself must spawn processes directly.
- **Test isolation by home:** architecture §Cross-cutting Patterns "Config management" and §Established Decisions [CI/CD] require each test to run in its own viola home, selected by `--home`, then the grandparent of `VIOLA_DIR`, then the default. The harness homes under `target/e2e-home/` are that isolation point. Tests never share an endpoint, a log or `budget.json`.
- **Env var and path namespace:** architecture §Conventions "Environment variables" requires the `VIOLA_` prefix. §Cross-cutting Patterns "Config management" says env vars are not a product configuration channel. The scope's `AGENT_RUN_CHUNK_BASE`, `AGENT_RUN_KEEP_HOMES` and `NEXTEST_PROFILE` are therefore acceptable only as test-harness and CI plumbing, never read by the product `viola` binary. They and the `target/agent-run/` and `target/e2e-home/` paths should be recorded in §Occupied Resources at wrap.

## Patterns to follow
- The fake agent answers `--version` the same way the real CLI does, so `run`'s version gate can parse it (architecture §Established Decisions [CLI Version Compatibility]). Per [CI/CD], it is substituted by naming its executable; npm-shim resolution applies only to the real `claude` shim.
- The one-line JSON record follows §Conventions "Data model conventions":
  - one complete JSON object plus `\n` per single `write` call;
  - RFC 3339 UTC timestamps with milliseconds and a `Z` suffix (chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`);
  - absent optional fields omitted on write (`skip_serializing_if = "Option::is_none"`);
  - snake_case field names.
- The harness JSON envelope `{"v":1,…}` follows the §Conventions "Protocol versioning" pattern: an integer `v` starting at 1, where added fields need no bump.
- Workspace layout (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `crates/`, `.github/workflows/ci.yml`) follows the §Infrastructure Patterns "Project directory structure". `scripts/` and `.config/nextest.toml` are additions to that tree and belong in the arch tree amendment. The scope assigns that amendment to the *Workspace tree and code-graph planes* chunk, and this chunk's wrap records them.
- CI matrix `windows-2025`, `macos-latest`, `ubuntu-latest` on native runners, triggered on push and pull_request, with no cross-compilers, and the real `claude` never run in CI (architecture §Infrastructure Patterns "CI/CD approach").

## Anti-patterns to avoid
- **Don't use the plan's CI setup as written.** architecture §Infrastructure Patterns "CI/CD approach" names `dtolnay/rust-toolchain@stable` and `Swatinem/rust-cache@v2.9.2`, which are tag and branch references. The scope requires every `uses:` pinned by full commit SHA and the toolchain read from `rust-toolchain.toml`. The plan's refs are outdated here and must be amended, not copied.
- **Don't create a product crate without a consumer.** Do not stand up `viola-core`, `-pty` or any other §Occupied Resources crate early just to match the tree (architecture §Established Decisions [Module Boundaries] against the scope's founder rule). Arch lists them as target state, not as chunk-1 deliverables.
- **Don't write to stdout or stderr from a product role while it hosts a child.** Per architecture §Cross-cutting Patterns "Diagnostic output channels", `hook` never writes to stderr and `run` writes nothing to the terminal but the child's output. The per-role JSON line goes to a file, never to stdout or stderr, and the harness reads it from disk.

## Contract bindings
- **arch ↔ obs, diagnostics path:** architecture §Occupied Resources "Filesystem" reserves `instances/<ViolaName>/diagnostics/` (format owned by obs). The scope writes `<home>/diagnostics/<role>.ndjson` at home level. Either the path is reconciled or the plan gets a home-level `diagnostics/` entry. The line's `timestamp` field name also differs from arch's `ts` convention (§Conventions "Data model conventions"). Obs owns the log format per §Cross-cutting Patterns "Diagnostic output channels", so it is flagged here, not decided.
- **arch ↔ tests, fake agent and harness:** architecture §Established Decisions [CI/CD] and [Deferred] give the test framework and fake-agent harness to tests. Arch supplies only the substitution mechanism (`viola run <name> -- <fake agent>`), `--home` isolation and the `--version` answer. `fixtures/claude/<cli-version>/` (§Occupied Resources "Repository") belongs to the next chunk.
- **arch ↔ security, CI hardening:** the SHA pins, `permissions: {}` and the toolchain floor come from security §Dependency Security. The `cargo deny` tokio and C-crate bans (§Infrastructure Patterns "Build system") belong to the Supply-chain chunk.
- **arch ↔ all domains, workspace naming:** `viola-e2e`, `viola-harness` and `viola-fake-agent` become names every domain's content refers to once recorded.

## Acceptance criteria contributions
- Root `Cargo.toml` is both the `viola` bin package and `[workspace] members = ["crates/*"]`, with `resolver = "3"` and edition 2024. Every member, including the root, sets `publish = false`. `Cargo.lock` is committed. (per architecture §Infrastructure Patterns "Build system", §Established Decisions [Module Boundaries])
- No `crates/viola-{core,pty,channel,state,agent-claude,mcp,ui}` directory exists unless it has a consumer and a test. Every new crate is named `viola-<area>`. (per architecture §Conventions "Naming patterns", §Occupied Resources "Workspace crates")
- `anyhow` and `tokio` appear in no manifest other than the root `viola` package's. Specifically, `viola-e2e` lists neither as a normal dependency. (per architecture §Conventions "Rust error types", §Cross-cutting Patterns "Tokio containment")
- `viola-fake-agent` is not in the default `cargo build --release` output (it sits behind the `fake-agent` feature) and answers `--version` with a parseable version line. (per architecture §Established Decisions [CLI Version Compatibility], [CI/CD])

## Relevant amendment history
(none). `architecture-amendments.md` does not exist yet, which is normal for a fresh project. At wrap, this chunk needs a first amendment covering:
- the rust-version floor, 1.89 → 1.96;
- `rust-toolchain.toml` as an exact pin instead of `stable`;
- SHA-pinned CI actions;
- the new crate `viola-e2e` and its bins;
- the `fake-agent` feature;
- the `scripts/` and `.config/nextest.toml` tree entries;
- the `target/agent-run/` and `target/e2e-home/` paths;
- the `AGENT_RUN_*` env vars.
