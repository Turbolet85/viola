## Output Protocol

Rules for iteration agents working on `D:/dev/projects/viola/.andromeda/runs/2026-09-23T21-50-50-security/security-plan-draft.md`:

1. **Output format.** Give patches (old → new) plus a changelog. Do NOT reproduce the full document.
2. **Patch format.** Each patch looks like this:
   ```
   ### Patch N: <short description>
   **Old:**
   <exact text copied verbatim from the current draft, long enough to be unique>
   **New:**
   <replacement text>
   ```
   The **Old** text must match the draft byte for byte, including backticks, `§` signs, en dashes and table pipes. If you cannot quote it exactly, do not write the patch.
3. **Changelog.** Write one line per change: `[Iteration N] [substantive|cosmetic] description`. A change is substantive only if it changes a control, a requirement, a version, a threat mapping, a ban, or what a downstream reader can derive. Wording, ordering and formatting changes are cosmetic.
4. **Prohibited:**
   - Reproducing the full document.
   - Restructuring sections without a stated defect. Keep the template order: Threat Model Summary → Auth & Authz → Input Validation → Data Protection → API Security → Dependency Security → Bootstrap phases → Secret Management → Error Handling → Anti-Patterns → Decisions Log.
   - Labelling cosmetic changes as substantive.
   - Editing the Threat Model Summary except to restore verbatim fidelity with `threat-assessment.md`.
   - Adding tools or versions that are not in `security-research.md`.
   - Adding content owned by other specialists: test cases (tests' domain), design tokens or component patterns (design's domain), observability instrumentation schemas (obs' domain), accessibility attribute rules (a11y's domain), or picks of observability and error-reporting platforms.
5. **Decisions Log.** Any patch that changes a security decision (tier, boundary mechanism, arch amendment, accepted risk) must come with a companion patch that updates the `## Security Decisions Log`.
6. **No issues.** If you find nothing, output "No patches" and add a single cosmetic changelog line saying so.

## Analysis Protocol

Work through these steps in order. Do not start writing patches until step 4.

1. **Read once, note nothing.** Read the whole draft (about 607 lines) from start to end. Then re-read each section you intend to patch just before you write its **Old** block. Do not copy **Old** text from memory.

2. **Walk these cross-references.** Every broken link below is a substantive defect.
   - **TMS Vector → mitigation.** The TMS lists nine attack-surface vectors:
     - Local IPC
     - CLI `send` text into the PTY
     - Loopback HTTP
     - Hook stdin
     - MCP over stdio
     - CLI arguments, flags and environment
     - Filesystem state under `~/.viola/`
     - Child process spawning / PATH resolution
     - Supply chain

     Each vector needs at least one of these: an Auth & Authz row, an Input Validation row, an API Security row, a Data Protection bullet, a Dependency Security control, a Security Anti-Patterns ban, or an explicit entry under Decisions Log **Accepted risks**. A vector with none of these is dangling.
   - **Six §6 elevations → enforced controls.** The six elevations are:
     - IPC access control and impersonation
     - bracketed-paste breakout
     - GUI output encoding
     - GUI cross-user and cross-origin readability
     - v1.x brake auth
     - integrity of `~/.viola/`

     Each must trace to a control that is concrete **on every OS the plan names** (Windows, Linux, macOS). An elevation that only has prose, or that is enforced on Unix only, is under-served.
   - **Tool/version across 3 sites.** Compare the plan body, the TMS verbatim block, and `security-research.md`. The TMS is a verbatim copy, so if the TMS and the body disagree, patch the body and never the TMS (see the "Do not chase" list).
   - **`(See § X)` pointers.** Check that each of these five points at a subsection that exists and holds the bans the pointer promises:
     - Input Validation → § Input
     - API Security → § API
     - Dependency Security → § Code Patterns
     - Secret Management → § Secrets
     - Error Handling → § Logging

     Also check the inline pointers "(see API Security)" and "(Authentication & Authorization, `~/.viola/` access control)".
   - **Decisions Log open questions ↔ sections.** There are three open questions: the SQOS `Stream::try_from(OwnedHandle)` spike, `events.ndjson` retention, and the SHA-256 crate. The section that depends on each one must reference it. A MUST control that silently depends on an open question is a defect.
   - **The 8 arch amendments ↔ plan body.** Each amendment must be stated as a requirement in the body section that owns it. The body must not contradict the amendment.
   - **NEVER-log ↔ data classifications.** Every raw form in TMS §1 must be covered by one consolidated NEVER-log floor that obs can consume. That means user content, tool `input`, config, the transient credentials, and operational metadata.
   - **Same control, several sections.** When a control is described in more than one section, the descriptions must agree. Examples: the Windows home DACL (Auth & Authz, Data Protection, Secret Management) and the strict-modes check (Auth & Authz, Input Validation, Data Protection, Anti-Patterns, Universal).

3. **Check each dimension below against its anchor example.** Anchors were verified byte-for-byte against the current draft during refinement. Treat them as confirmed starting points, then look for siblings of the same defect class.

4. **Out-of-scope discipline.** Some findings would require one of the following:
   - writing test cases (tests' domain)
   - observability instrumentation schemas (obs' domain)
   - accessibility attribute names or conformance claims (a11y's domain)
   - design tokens, component patterns or typography picks (design's domain)
   - naming a concrete observability or error-reporting platform

   Do NOT patch those. Instead, check that the security plan states the boundary requirement: what must hold, not how it is wired. Patch only if the boundary itself is unstated. The security plan defines WHAT to protect, log and verify; other specialists define HOW.

5. **Prioritize and budget.** Spend patches in this order:
   1. Dimensions tagged `[priority: high]` in bucket 1: Dimensions 1–3.
   2. Bucket 2: Dimensions 4–5, only if budget remains.
   3. Bucket 3 (`[priority: medium]`, Dimensions 6–7): only one-line fixes that cannot be deferred.

   Inside a bucket, fix whatever would break implementation or a downstream plan before anything cosmetic.

6. **Do not chase these verified non-issues.** Each would waste a patch:
   - TMS Infrastructure says "dist 0.33.0 release workflow with cargo-auditable 0.7.6", while the Decisions Log uses `>=` floors. The TMS line matches `threat-assessment.md` byte for byte (TMS §1–§5 were diffed and differ only in headings and blank lines). Do not patch the TMS. At most, make sure the body and Decisions Log use floors.
   - The `> **Tool-version syntax.**` blockquote in Dependency Security, which names gitleaks and govulncheck, is template boilerplate from the phase-3 output template. Leave it alone.
   - These plan versions were pre-checked against `security-research.md` and match. Do not re-patch them:
     - interprocess 2.4.4, windows-sys 0.61.2, sysinfo 0.39.6
     - getrandom 0.4.3, constant_time_eq 0.6.0, atomic-write-file 0.3.1
     - axum 0.8.9, tower-http 0.7.1, rmcp 3.4.1, schemars 1.2.2
     - nutype 0.8.0, serde 1.0.229, serde_json 1.0.151, serde_path_to_error 0.1.20
     - chrono 0.4.45, clap 4.6.7, notify 8.2.0, vt100 0.16.2, portable-pty 0.8.1, portable-pty-psmux 0.9.7
     - bytes `>=1.11.1`, tracing-subscriber `>=0.3.20`, Rust `>=1.96`
     - the two action SHAs
     - cargo-deny `>=0.20.2`, zizmor `>=1.30.1`, cargo-audit `>=0.22.2`

## Analysis Dimensions

### 1. Downstream Readiness [priority: high]

- **obs.** The Logging section is skipped at Minimal tier. obs therefore gets its floors from the `logging-redaction-wire` bullet, Error Handling, Data Protection § Logs, and Anti-Patterns § Secrets / § Logging. Do these sources list the same forbidden values?
  - Build the union: GUI token, `Cookie` header, `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, other R8-stripped `CLAUDE*` values, and tool `input` / serde_path_to_error values outside `diagnostics/`.
  - Check whether one consolidated list exists that the others cross-reference.
- **design.** From API Security's "Frontend output-encoding constraints" row plus the 401 Problem Details, can design derive what the page must do in these cases?
  - The `?t=` exchange fails.
  - `/api/*` returns 401 `urn:viola:problem:unauthorized` after the browser session ended.
  - The user opens `http://127.0.0.1:<port>/` without `?t=`.

  The security plan must state the boundary (for example, never echo the token or the `.url` contents into the page or into `detail`). Rendering the recovery UX belongs to design.
- **route / setup-project.**
  - Can route order the 8 arch amendments against the chunks from what is written? In particular, the Unix endpoint move (amendment 2) and the v1 GUI cookie (amendment 1) must land before the `viola-channel` and `viola-ui` chunks.
  - Does every Bootstrap phase name artefacts setup-project can materialise (the `deny.toml` blocks, the `ci.yml` `schedule:`, `permissions` edits)?
  - Are any MUST controls missing from every phase? Examples: the SHA-256 re-hash, the Windows `--home` DACL.
- **tests (scope only; do not write tests).** Are the parser surfaces that need fuzz or property coverage named in one place, so tests can scope them without reverse-engineering Anti-Patterns? The surfaces are `validate_paste_text`, the `Last-Event-ID` parser, channel ndjson framing with `MAX_FRAME`, the hook stdin parser, and the vt100 `catch_unwind` path.

**Anchor example:** Bootstrap phases, `logging-redaction-wire` bullet vs. Security Anti-Patterns § Secrets

> "Never write the GUI token, the `Cookie` header or the stripped `CLAUDE_CODE_MESSAGING_TOKEN` to any log or diagnostic."

> "NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into `events.ndjson`, snapshots, `diagnostics/` or fixtures."

**Issue:** The bootstrap floor that obs is told to satisfy ("Security floors that obs must satisfy") lists only `CLAUDE_CODE_MESSAGING_TOKEN`. It leaves out `CLAUDE_CODE_MESSAGING_SOCKET` and the other R8-stripped `CLAUDE*` values, which the Anti-Patterns ban covers.

It also leaves out the rule that drift reports that "can quote tool `input`" go only to `diagnostics/` (Data Protection § Logs).

Search evidence:
- `MESSAGING_SOCKET` appears only at TMS line 42, Secret Management (2 places) and § Secrets. It is absent from the `logging-redaction-wire` block.
- `R8` and `CLAUDE\*` are also absent from that block.
- The block was read end-to-end (5 bullets).

**Why this matters:** obs will build its redaction list from the phase item addressed to it and ship a logger that can emit the messaging socket path and other parent-identity variables. The ban then exists only in a section obs was never pointed to.

**Adversarial:** Suppose obs implements exactly the `logging-redaction-wire` bullets and nothing else. Which TMS §1 data class would then reach a log line that is not in `diagnostics/`? Is there any sentence in the plan that obs was explicitly directed to read and that would have stopped it?

### 2. Threat → Control Traceability and TMS Fidelity [priority: high]

- Walk all nine TMS §2 vectors through the Analysis Protocol step 2 mapping. For each vector, name the plan row or ban that mitigates it, or the Accepted-risks line that accepts it. Candidates for dangling:
  - **Child process spawning / PATH resolution:** `claude agents --json` spawned by `list`, `mcp` and `ui`.
  - **Unbounded `events.ndjson` growth.** It is only "an arch open item". Is local disk exhaustion accepted as a risk anywhere, or only deferred?
  - **`heartbeat`.** It is a cross-process reader in TMS §2 Filesystem. Does any Input Validation row (for example "Own state files on read") give it the `MAX_FRAME` line cap and the strict-modes precondition?
- Walk each §1 data class to a protection:
  - `permission.input`: Bash commands and Write-tool file bodies.
  - `budget.json` usage data.
  - `/api/sessions` unwrapped identifiers.
  - `instances/<name>/settings.json`, the per-session statusline override.

  Is any class left with no named control, such as file mode, cookie gate or exclusion from external errors?
- TMS fidelity. The TMS "Security tier" justification compresses `threat-assessment.md` §6.
  - The TMS reads "with no decided per-OS enforcement and predictable names in shared namespaces".
  - The source reads "no decided per-OS enforcement of its same-OS-user boundary, and its names are predictable in shared namespaces (Windows pipes, `/tmp`)", and adds "Each of these traces to Section 2."

  Restoring verbatim text is the only permitted TMS edit. Do it only if budget remains after bucket 1 defects.

**Anchor example:** Threat Model Summary, Attack surface, "Child process spawning and PATH resolution" vector

> "`list`, `mcp` and `ui` spawn `claude agents --json`, resolving `claude` from their own PATH with npm-shim → `claude.exe` resolution."

> "**Trust boundary:** Whatever the calling process's PATH resolves to. Output parsing is tolerant, and failure becomes `unknown`."

**Issue:** The plan body covers this vector only partly:
- The shim is resolved to `.exe` (Input Validation, "Child executable resolution").
- Output is parsed with `Read::take(MAX_FRAME)` ("Child process output").
- `hooks.json` / `.mcp.json` must not use PATH (Universal).

Nothing states the control, or the accepted risk, for `claude` itself being resolved from an attacker-influenced PATH in `list`, `mcp` and `ui`. `mcp` runs inside the driver session, whose PATH an agent can influence.

Search evidence:
- `PATH` appears in the plan body (after line 178) only in the Universal `hooks.json` ban.
- `resolv|shim|claude agents` finds only the two Input Validation rows and the § Input / § Data Protection bans, none of which address `claude` lookup.
- Decisions Log **Accepted risks** was read end-to-end (4 bullets: ambient same-user trust, driver prompt-injection, squatted endpoint DoS, plaintext). None mentions PATH.

**Why this matters:** /implement will write `Command::new("claude")` for `claude agents --json` with no guidance. Security review of the `viola-ui` or `viola-mcp` chunk cannot tell whether that is accepted (same-user PATH equals same-user trust) or an omission. Either state the resolution rule or add an Accepted-risks line, with a Decisions Log companion patch.

**Adversarial:** A same-session agent can set PATH for a `viola mcp` it launches. The `claude` it resolves is spawned by viola and its output feeds `/api/sessions` and `list`. What stops a planted `claude` from injecting rows? Is the only barrier "an unwrapped row's `name` is never a `ViolaName`", and is that barrier stated for the MCP `list` tool as well as for the GUI?

### 3. Tool Anchoring (Catalog ↔ Plan) [priority: high]

- The SHA-256 re-hash of `bin/<version>-<hash>/` is a MUST control, yet `security-research.md` catalogues no hash crate. The Output Protocol forbids adding uncatalogued tools, so do NOT name a crate. Instead check three things:
  - Does the Data Protection bullet that requires the re-hash cross-reference the open question?
  - Does a Bootstrap phase own the step (`auth-scaffolding-baseline` lists `viola-state` home creation but no hash step)?
  - Are the pick constraints stated as requirements: pure Rust, passes the cargo-deny C-build ban, recorded in the Decisions Log before the chunk that writes `bin/`?
- Pin syntax. Dependency Security says "`[workspace.dependencies]` pins every third-party version (portable-pty `=0.8.1`, rmcp `~3.4`)".
  - `~3.4` is a range, not an exact pin, and it admits 3.4.0, below the catalogued 3.4.1.
  - Is the rmcp floor tied to `Cargo.lock`, or is "pins every" inaccurate for rmcp?
  - Do Dependency Security, the Bootstrap `dep-audit-tooling-install` item and the Decisions Log v1.x prerequisites all use `>=` floors for the external CLIs (dist, cargo-auditable, cargo-audit, cargo-deny, zizmor)?
- Justifications. `security-research.md` lists tempfile 3.27.0 as atomic-write-file's "drop-in alternative". Does the plan say why atomic-write-file was chosen, or does it assert the choice without tracing it to research?

**Anchor example:** Data Protection, At rest, Code-bearing artefacts, together with Security Decisions Log, Open questions

> "Before reusing an existing `bin/<version>-<hash>/viola(.exe)`, `viola run` re-hashes the file with SHA-256 and compares the truncated result with `<hash>`. On a mismatch it refuses to start (exit 1)."

> "SHA-256 crate: no hash crate appears in security-research.md. Pick one at implementation time that passes the cargo-deny C-build ban (pure Rust), and record it here."

**Issue:** A refuse-to-start integrity control depends on an unresolved dependency. The Data Protection bullet does not point to the open question.

Search evidence:
- `SHA-256|sha2|sha256|hash crate` in the plan finds Data Protection, the § Data Protection ban, amendment 8, the open question, and two round-1 Decisions Log lines. None names a crate or a Bootstrap phase.
- The Bootstrap phases block was read end-to-end, and no hash step appears.
- `security-research.md` has 0 hits for `SHA-256`, `sha2` and `sha-2`.

**Why this matters:** setup-project cannot add the dependency, and route cannot place it in a phase. /implement will pick a crate unaided in the middle of a chunk and may pull in a C-building crate (such as `ring`) that fails `cargo deny check`, or silently fall back to a non-cryptographic hash. The § Data Protection ban exists to prevent exactly that fallback.

**Adversarial:** Suppose the implementer, lacking a catalogued crate, reuses the FNV-1a helper that already builds endpoint names. Which sentence in the plan would a phase-loop distiller surface to stop them? Is that sentence in a section the `viola-state` chunk distiller actually reads?

### 4. GUI Credential Lifecycle, Scope and Recovery [priority: high] [trigger: GUI per-launch token exchanged for a browser-session cookie `viola_<port>`, HttpOnly, SameSite=Strict, no Max-Age]

_There are no accounts. "Auth recovery" here means lost cookie, lost launch line, suspected token leak, and a stale `.url` file after a crash._

- **Cookie scope across ports and users.** Cookies for host `127.0.0.1` go to every port on that host, and `SameSite=Strict` treats all `127.0.0.1` ports as same-site.
  - Does the plan address the token leaking to any other loopback HTTP server the user's browser requests? That includes one run by another OS user, which would then replay the token against `/api/events`. This is the exact reader the "GUI cross-user and cross-origin readability" elevation targets.
  - If the leak is accepted, is it in Accepted risks with a Decisions Log entry?
- **Exchange failure versus the ungated `/`.**
  - The principal-check row says "`/`, `/assets/*`, `/health` and `/ready` stay ungated".
  - Error Handling says "`urn:viola:problem:unauthorized` (401): missing or invalid `viola_<port>` cookie or `?t=` token."

  Is `GET /?t=<wrong>` a 401 or the page? Do the `Cache-Control: no-store` and `Referrer-Policy` headers apply to the failure response too? Is the `?t=` comparison explicitly constant-time? (The § Authentication ban covers "token or cookie", so check that the exchange row states it.)
- **Recovery consolidation.** "_Authentication Recovery Flows: omitted … Recovery means restarting `viola ui`._" The recovery facts are spread across the Session/token rotation row, Secret Management "Per-incident" and the stderr launch line.
  - Is it stated anywhere that no recovery shortcut may exist (no token-reprint route, no token or `.url` path in the 401 `detail`)?
  - Is the fate of a stale `.url` after a crash stated? It is "removed on graceful shutdown" only.

**Anchor example:** Authentication & Authorization, "Token / session storage" row

> "**Cookie name:** includes the port, because cookies are not port-scoped and two `viola ui` launches on different ports must not overwrite each other."

**Issue:** The plan knows cookies are not port-scoped but draws only the collision conclusion. It never draws the leakage conclusion: the browser attaches `viola_<port>=<token>` to same-site requests to any `127.0.0.1:<other port>`. Loopback ports are open to every local user (Universal: "Loopback TCP is reachable by every local user").

Search evidence:
- `port-scoped` has a single hit (this row).
- `other port` and `another local user` have 0 hits.
- `cross-user` appears only in the Auth & Authz intro, which quotes the elevation name.
- `every local` appears only in the Universal ban, which assumes the cookie check restores the boundary.

**Why this matters:** Amendment 1 justifies the v1 cookie as the fix for other OS users reading prompts and tool `input`. If the cookie can be harvested by another user's loopback server, the elevation's control is incomplete, and nothing tells /implement or design to limit the exposure or to record the gap.

**Adversarial:** Another local user runs a server on `127.0.0.1:8080`. The victim, with `viola ui` open, visits `http://127.0.0.1:8080/` directly (a top-level navigation, so `SameSite=Strict` still sends the cookie). What in the plan stops that server from logging `Cookie: viola_47319=…` and then reading the full SSE feed until the browser closes? Which Decisions Log entry records the answer?

### 5. Tier Calibration and Elevation Coverage [priority: high]

- **Under-served elevations.** For each of the six elevations, is the control concrete on Windows as well as on Unix? Windows is the primary host: the TMS example path is `C:/Users/<user>/.viola`, and macOS is CI-only.
  - "Integrity of `~/.viola/`": the strict-modes check is defined only inside the Auth & Authz **Unix:** bullet. Yet the Input Validation CLI row, the § Data Protection `statusline_command` ban and the Universal "NEVER let `config.json` … switch off … the home strict-modes check" ban treat it as a cross-OS control. What is the Windows check at the start of `run`, `ui`, `mcp` and `hook statusline`?
- **Over-engineering.** Does each above-Minimal control trace to a named elevation? Check the Windows SQOS + `server_process_id()` + sysinfo start-time verification, the 7-day CVE SLA, the v1.x Azure Artifact Signing / zipsign entries under "What counts as secret", and the Code Patterns bans on the v1.x release workflow. A control with no stated driver is Standard-tier rigour that has leaked in.
- **SKIP markers.**
  - `## Logging & Monitoring` is correctly absent.
  - `### Supply chain integrity` is correctly absent, with v1.x prerequisites only in the Decisions Log.
  - Is any Standard-tier material still present as body text instead of Decisions Log deferrals?

**Anchor example:** Data Protection, At rest, Files (`~/.viola/`), vs. Authentication & Authorization, "`~/.viola/` access control" row

> "Windows: user-only DACL."

> "**Windows:** `%USERPROFILE%` inheritance (user + SYSTEM + Administrators) is accepted."

**Issue:** The first statement is false for the default home: it inherits the Administrators ACE. Secret Management and the Token row repeat "the home's user-only DACL on Windows". In addition, the Auth & Authz row gives Windows only a creation-time DACL for a `--home` outside `%USERPROFILE%`. It gives no startup verification equivalent to the Unix strict-modes check.

Search evidence:
- `strict-modes` has hits in Input Validation (2), Data Protection, Bootstrap, Error Handling, § Data Protection and Universal. None defines a Windows procedure.
- `StrictModes` has a single hit, inside the **Unix:** bullet.
- `world-writable|group-` has a single hit, also in the Unix bullet.
- `Windows|DACL|USERPROFILE` hits in the Auth & Authz row cover only inheritance acceptance and `--home` creation.

**Why this matters:** `hook statusline` executes `statusline_command` from `snapshot.json`, and "write access there equals code execution". On Windows the plan gives /implement no check to write, so `#[cfg(windows)]` will likely become `Ok(())`. That silently satisfies the Universal "never switch off" ban while the elevation goes unenforced on the primary platform.

**Adversarial:** A `--home` created before this plan landed, or an existing directory with a later ACL change granting `Users:(M)`, is reused on Windows. Which control catches it before `hook statusline` shells out? If none does, does the Decisions Log say Windows home integrity rests on creation-time ACLs only?

### 6. Anti-Pattern Relevance and Centralisation [priority: medium]

- The Anti-Patterns header claims to be the "single source of truth for all security bans". Are the bans that protect the "GUI output encoding" elevation actually in § API or § Code Patterns? Or do they live only inline in a section body?
- Is every ban grounded in viola's stack?
  - `X-Forwarded-For` / `Forwarded`: nothing in the plan reads a client IP (`forwarded|client ip|remote addr|ConnectInfo|peer addr|ip address` hits only the ban itself). Is it filler, or should it be reframed as a real constraint, such as "never make an access decision on client address"?
  - Code Patterns bans v1.x release-workflow artefacts (`rust-cache` restore, `self_update` without `signatures`) that do not exist in v1 and duplicate the Decisions Log prerequisites. Do they belong in the Decisions Log only?
- **Universal has 7 bans.** Most name viola artefacts: `hooks.json` PATH, `ledger/stamps.json`, `snapshot.json` writers, `VIOLA_DIR`. Would they be read by every chunk, or do some belong under § Data Protection or § Code Patterns? After any move, do at least 5 genuinely cross-cutting principles remain (such as "never let a security refusal block the human")?

**Anchor example:** Security Anti-Patterns header vs. API Security, "Frontend output-encoding constraints (for design)" row

> "_[ALL tiers — single source of truth for all security bans]_"

> "Never use `innerHTML`, `v-html` or `dangerouslySetInnerHTML`."

**Issue:** The output-encoding bans exist only inline in the API Security table: `innerHTML` / `v-html` / `dangerouslySetInnerHTML`, no Markdown-to-HTML, no inline `<script>`, no `eval`. They are not in any Anti-Patterns subsection.

Search evidence:
- `innerHTML|v-html|dangerously|markdown|eval|inline` hits only that row and the Decisions Log's sanitizer note.
- § API was read end-to-end (8 bans), with none on frontend rendering.
- § Code Patterns was read end-to-end, also with none.

**Why this matters:** Distillers and design treat Anti-Patterns as the ban list. The most direct XSS-to-feed-exfiltration control for the GUI's untrusted-text elevation is missing from it, so a `viola-ui` frontend chunk distiller that pulls only § API bans would drop it.

**Adversarial:** A phase-loop distiller for the `viola-ui` frontend chunk extracts only `## Security Anti-Patterns` § API and § Universal. Which of the GUI output-encoding requirements survive into that chunk's context? Would /implement render assistant Markdown through a library by default?

### 7. API Hardening Completeness [priority: medium] [trigger: loopback HTTP GUI `viola-ui` (axum 0.8.9 + tower-http 0.7.1) is a §2 attack-surface vector]

- **Resource bounds.** The Rate limiting row justifies "None in v1" only on credential-guessing grounds. Is there any bound on:
  - concurrent SSE `/api/events` streams per launch,
  - notify watchers and tail file handles per stream,
  - idle or slow connections from other local users, who can reach the port but never send a complete request?

  Bounds must come from axum/tower-http as catalogued. Do not add an uncatalogued rate-limit crate.
- **Compression.** The row reads "`CompressionLayer` excludes `text/event-stream`. Re-test the exclusion under tower-http 0.7's 406 semantics for `identity;q=0` and `*`." Is that an implementable predicate, or a to-do that /implement cannot turn into code? The § API ban "NEVER compress `text/event-stream`" depends on it.
- **Host allowlist edge cases.** Does "compare it exactly" say what happens with a missing `Host` (HTTP/1.0), `LOCALHOST:<port>` letter case, or a trailing-dot host? The launch URL and cookie are set on `127.0.0.1`. Is allowing `localhost:<port>` consistent with the cookie and `Origin` checks?

**Anchor example:** API Security, "Rate limiting" row

> "None in v1. The surface is single-user, loopback-only and GET-only, and the only credential is a 256-bit per-launch token, which cannot be brute-forced."

**Issue:** The justification answers brute force only. Elsewhere the plan says the loopback port is reachable by every local OS user (Universal ban; amendment 1), yet it sets no connection or stream bound. Each authenticated SSE stream also tails every `instances/<name>/events.ndjson`, which is never rotated.

Search evidence:
- `concurren` and `connections` have 0 hits.
- `limit|bound|exhaust|file handle` after line 178 finds only `MAX_FRAME` rows, request-size rows, the `events.ndjson` retention question and the Decisions Log line "GUI/IPC rate limiting: not needed at this tier". None bounds GUI connections or streams.

**Why this matters:** /implement will use axum's defaults, which have no per-listener connection cap. A local process can hold many connections open or, with the cookie, open many SSE streams. The failure mode is availability of the human's view, and the plan gives no stance either way. Either state a bound or record the accepted risk.

**Adversarial:** Another local user opens thousands of TCP connections to `127.0.0.1:47319` and never finishes the request line. The Host middleware never runs, because no headers arrive. What in the plan keeps `viola ui` serving the real user? If nothing does, where is that recorded as accepted?
