proposals:
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [CLI Conventions]'
    change: 'Retire "One verb has no machine form yet: `viola revive` takes no `--json` … its `--json` is owed to the working-route entry "CLI machine contract"" (architecture.md:98) and say instead: `viola revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract); a strict-modes or preflight refusal then prints `{"v":1,"error":"<code>","detail":null}` (code `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) in place of its `unable:`/`hint:` pair, exit 1, and `--list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order; without `--json` the pairs and rows are byte-identical; `run::start`''s own refusals after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, internal error) print no document.'
    sidecar: 'CLI Conventions: revive takes --json on both arms (error document with five codes, --list ok sessions payload); the owed clause to "CLI machine contract" retired; the post-preflight run::start refusals named as printing no document.'
    rationale: 'Report Changes → Symbols / APIs (src/cmd/revive.rs: ReviveArgs gains --json on both arms, Refusal::write, ListRow; src/cmd/run.rs collision_check json) and Counts / qualifiers moved (pattern revive-nojson, architecture.md:98 stale; owner-title :98); Spec claims disproved names the run::start refusals that print no document; Expected amendments lists this site.'
    basis: 'src/cmd/revive.rs:269-275; src/cmd/client.rs:120-124'
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources → Binary, subcommands and exit codes'
    change: 'In the subcommands row (architecture.md:300) replace "it takes no `--json`" with: it takes `--json` on both arms, and a refused strict-modes check or preflight refusal under `--json` is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the five codes) with stderr empty; `<name> --list --json` prints the `ok.sessions` document, or the `no-session` document with exit 1.'
    sidecar: 'Occupied Resources subcommands row: revive takes --json (its exit-1 error documents and --list ok document).'
    rationale: 'Same retired claim as the [CLI Conventions] decision, restated in the registry; report Counts / qualifiers moved, pattern revive-nojson lists architecture.md:300 stale.'
    basis: 'src/cmd/revive.rs:269-275'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Conventions → CLI exit codes'
    change: 'In the exit-1 row (architecture.md:129), where revive''s strict-modes refusal and its four preflight refusals are each "one `unable:` / `hint:` pair on stderr", add that under `--json` each is instead the `{"v":1,"error":"<code>","detail":null}` document (`strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`) on stdout with the same exit 1 and the same log lines, and that `run::start`''s own refusals reached after a passed preflight print no document.'
    sidecar: 'Conventions CLI exit codes: revive''s exit-1 refusals name their --json documents.'
    rationale: 'Report Changes → Symbols / APIs (Refusal::write(name, json), Refusal::detail codes plus already-live from collision_check; refused_document); Expected amendments names "the revive lines name their documents" for §Conventions.'
    basis: 'src/cmd/client.rs:120-124'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources → Filesystem (viola home = `<user home>/.viola/`)'
    change: 'In the `instances/<ViolaName>/…` row, where `viola revive` (the start arm and `--list`) refusing `check_instance` is "exit 1 with the state-files `unable:` / `hint:` pair", add: or, under `--json`, the `strict-modes-failed` document on stdout — still no log opened, no line written and the home''s mode left as found.'
    sidecar: 'Filesystem instances row: revive''s strict-modes refusal under --json is the strict-modes-failed document, still opening nothing.'
    rationale: 'Report Changes → Symbols / APIs (src/cmd/revive.rs: strict-modes-failed among Refusal::detail; "A refused check still opens nothing and writes no line, on both arms"); the human-pair-only claim retired at [CLI Conventions] is restated here.'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → CLI `--json` output'
    change: 'Register revive''s documents beside the `error` outcomes: `viola revive --json` refusing its strict-modes check or its preflight prints `{"v":1,"error":"<code>","detail":null}` (exit 1; code one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; written by `client::refused_document`, codes only, no path, pid or id), and `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…},…]}}` in log order (exit 0), or the `no-session` document (exit 1) on an empty chain; a refusal of `run::start` after a passed preflight prints no document.'
    sidecar: 'Standard Contracts CLI --json output: revive''s exit-1 error document (five codes) and --list''s ok.sessions payload registered; the post-preflight gap stated.'
    rationale: 'Report Changes → Symbols / APIs (src/cmd/revive.rs, src/cmd/client.rs refused_document) adds a new CLI output document; Counts / qualifiers moved, pattern revive-doc: 0 sites ("the CLI --json output contract states no such document yet"); Expected amendments lists it; Spec claims disproved bears on the "every exit-1 refusal" wording.'
    basis: 'src/cmd/client.rs:120-124; src/cmd/revive.rs:269-275'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'In the `VIOLA_DIR` row (architecture.md:321) retire "As landed, `hook` is its only reader: the CLI verbs do not read it yet" and say: as landed (chunk 2026-10-17-cli-machine-contract) `hook` and every CLI verb, `run` and `revive` included, read it — the verbs through `resolved_home`, which takes the `VIOLA_DIR` grandparent only when `VIOLA_NAME` and `VIOLA_DIR` pass `instance_of`''s shape check (a pair that fails falls through to the default, never refuses); the `mcp` verb is not built.'
    sidecar: 'Environment variables VIOLA_DIR: readers now hook and every CLI verb (run and revive included) via resolved_home; the "hook is its only reader" clause retired.'
    rationale: 'Report Changes → Schema / config ("VIOLA_DIR gains readers: every CLI verb (through resolved_home), run and revive included") and Symbols / APIs (src/cmd/mod.rs resolve_home, resolved_home, instance_of); pattern viola-dir-step lists architecture.md:321 stale.'
    basis: 'src/cmd/mod.rs:208-219; src/cmd/mod.rs:221-239'
  - detector: D-arch-resources
    severity: warning
    section: '§Cross-cutting Patterns → Config management'
    change: 'In the Viola home bullet (architecture.md:398) replace the "As landed … only `hook` takes the `VIOLA_DIR` step … owed to the working-route entry "CLI machine contract"" clause with: as landed (chunk 2026-10-17-cli-machine-contract) `resolve_home(flag, name, dir, user_home)` in `src/cmd/mod.rs` resolves `--home` as given → the `instance_of` home when both `VIOLA_NAME` and `VIOLA_DIR` pass → `<user home>/.viola`, a failing pair falling through; `resolved_home` reads the two variables and `std::env::home_dir()` once at every verb arm; `canonical_home` canonicalises a home that exists (`std::fs::canonicalize`, a Windows `\\?\C:\…` written back as `C:\…`, other verbatim forms kept) and keeps `std::path::absolute`''s spelling for one that does not, creating nothing; `run::start` canonicalises after `open_wrapper_log` and before the collision check, pin and bind; the statusline-source rule compares the home with `canonical_home(default_home(user_home))`, and the `home_dir()` read it shares is `resolved_home`''s.'
    sidecar: 'Config management: the VIOLA_DIR step for the CLI verbs and the home canonicalisation (plain drive form on Windows) recorded as landed; the owed clause retired.'
    rationale: 'Report Changes → Symbols / APIs (src/cmd/mod.rs resolve_home order, resolved_home at every arm, canonical_home / plain_drive; src/cmd/run.rs start and statusline_source); pattern viola-dir-step architecture.md:398 @c291 stale, owner-title :398 stale, canonical pattern; Expected amendments lists this site. Same retired "hook is the only VIOLA_DIR reader" claim as the VIOLA_DIR registry row.'
    basis: 'src/cmd/mod.rs:221-239; src/cmd/mod.rs:241-249'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Filesystem (viola home = `<user home>/.viola/`)'
    change: 'In the `diagnostics/` row (architecture.md:341) retire "and `viola revive --list`, open no process log and write no line" (a refused check still opens none) and add `revive --list` to the `cli` producers: under a parsed `VIOLA_NAME` and only after its strict-modes check passed, it writes `process-start` and `process-exit{subject:"self"}` (`exit_code` 0 with no detail, or `exit_code` 1 `detail:"no-session"` on an empty chain) into the caller''s own `cli-<VIOLA_NAME>.ndjson`; with no name it opens no `cli-*.ndjson`.'
    sidecar: 'Filesystem diagnostics/: revive --list is a cli producer under a VIOLA_NAME (caller''s cli-<name>.ndjson, process-start / process-exit); the "opens no process log" clause retired.'
    rationale: 'Report Changes → Symbols / APIs (src/cmd/revive.rs --list opens the caller''s cli-<VIOLA_NAME>.ndjson through client::start; dispatch''s Revive arm detail sink own_name(), ObsProcess::Cli); pattern list-nolog architecture.md:341 stale, and the cli producers fact on the same row; Expected amendments lists it.'
    basis: 'src/cmd/revive.rs:244-246'
  - detector: D-arch-resources
    severity: warning
    section: '§Conventions → CLI exit codes'
    change: 'In the exit-1 row (architecture.md:129) replace "`role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli`" with: `role_of` files `revive` followed (after any `--home` pair) by `--list` before a `--` under `cli` and every other `revive` beside `run` (a `--list` after `--` is the child''s word), so `revive --list` joins `send` … `verify` at the `cli` catch site (`error: internal error`, `process-exit{detail:"internal-error"}`); and replace "`viola revive --list` has two refusals … each the stderr pair and exit 1 with no log line" with: the strict-modes refusal writes no line, and the `no-session` refusal under a parsed `VIOLA_NAME` writes `process-exit{subject:"self", exit_code:1, detail:"no-session"}` into the caller''s `cli-<VIOLA_NAME>.ndjson` (each the stderr pair, or under `--json` its document).'
    sidecar: 'Conventions CLI exit codes: role_of files revive --list under cli; revive --list''s no-session refusal logs its process-exit in the caller''s cli log.'
    rationale: 'Report Changes → Symbols / APIs (src/main.rs role_of; src/cmd/revive.rs --list logging); pattern role-of architecture.md:129 stale ×2 and list-nolog''s claim restated on the same row.'
    basis: 'src/main.rs:100-107; src/cmd/revive.rs:244-246'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Repository'
    change: 'Register `schemas/config.v1.json` beside the other schema rows: the tolerant (`additionalProperties: true`) test-side schema of `config.json`, properties `v` (const 1), `diagnostics_level` (`info`|`debug`), `claude_env_keep` (at most 32 strings `^CLAUDE[A-Z0-9_]*$`), `five_hour` / `seven_day` (integers 0-100) — the five `CONFIG_KEYS` of `src/obs.rs`, pinned equal as a set by `config_keys_are_the_config_schema_properties`; readers unchanged (chunk 2026-10-17-cli-machine-contract).'
    sidecar: 'Repository: schemas/config.v1.json registered (tolerant config.json schema, its properties the five CONFIG_KEYS).'
    rationale: 'Report Changes → Files (schemas/config.v1.json new) and Schema / config; pattern config-schema reports 0 architecture sites, so the new repository file is unregistered; Expected amendments lists it for §Occupied Resources.'
    basis: 'schemas/config.v1.json:1-18; src/obs.rs:335-353'
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'In the `schemas/` row add `config.v1.json` (the tolerant `config.json` schema, test-side) to the listed schemas.'
    sidecar: 'Project directory structure: schemas/ row lists config.v1.json.'
    rationale: 'Same new repository file as the §Occupied Resources → Repository registration; the keyed row enumerates the schemas directory''s files and omits it (report Changes → Files).'
    basis: 'schemas/config.v1.json:1-18'
    dependent-of: D-arch-resources
note: 'No architecture drift for state-recovered.scanned: architecture names no state-recovered field (the row is obs-owned, schemas/diag-line.v1.json). Dependencies none, crates none: D-arch-decisions found no new library or runtime. The revive --json post-preflight gap (run::start refusals print no document, against matrix#v1-39 five-code acceptance) is stated as landed in the proposals; its owner stays P2/P7''s, so the Standard Contracts wording may need revisiting once that is decided.'
