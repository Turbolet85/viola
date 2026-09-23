## Output Protocol

You are an iteration agent improving the viola architecture document (`architecture-draft.md`). Follow these rules exactly.

1. **Output patches and a changelog only.** Do not reproduce the full document.
2. **Patch format.** Number each patch and use this form:

   ### Patch N: <short description>
   **Old:**
   ```
   <exact text copied verbatim from the current document, long enough to be unique>
   ```
   **New:**
   ```
   <replacement text>
   ```

   The **Old** text must match the document character for character, including backticks, em-dashes, `·` separators and list markers, or the patch cannot be applied. Keep each patch as small as it can be while staying unambiguous. To add text, quote the adjacent anchor line as **Old** and give that line plus the new content as **New**.
3. **Changelog.** Give one line per patch in the form `[Iteration N] [substantive|cosmetic] description`.
   - **Substantive** means the patch changes or adds a technical fact, contract, decision, rationale, constraint, version, name or resource.
   - **Cosmetic** means wording, formatting or ordering only.
4. **Prohibited:**
   - reproducing the full document;
   - restructuring or renaming sections without a concrete defect behind it;
   - labelling a cosmetic change as substantive;
   - inventing measured facts (CLI versions, spike results, ledger measurements) that the document does not already support. If a fact is missing and cannot be derived, add an explicit, named open item (for example "confirmation window: default N s, ledger-tunable") instead of a fabricated measurement;
   - adding specialist-owned choices: frontend framework, CSS or component library, logger crate, test framework, auth library.
5. **If you find no issues:** output "No patches", then a single changelog line `[Iteration N] [cosmetic] No issues found`.

Keep these document-level invariants when you patch:
- Mechanism, not policy (R4).
- The human always wins the wheel.
- No daemon.
- `viola hook` always exits 0 and never produces exit 2.
- Tokio is confined to `viola-mcp` and `viola-ui`.
- Claude-specific shapes live only in `viola-agent-claude`.
- Every undocumented CLI behaviour is a capability-ledger row.

## Analysis Protocol

Work through these steps in order. Do not jump from scanning straight to patching.

1. **Read once, note nothing.** Read the whole document (Design Philosophy through Existing Scopes) before you form any finding. Several facts appear only once, late in the document. Examples: `publish = false` appears only in Inherited Defaults, and the endpoint hash recipe appears only in Conventions → Data model.

2. **Walk these cross-references explicitly.** Each one is a place where this document states the same fact in more than one section. List every site, then compare them.
   - **Verbs.** Compare these six lists entry by entry: the clap row in Stack; "[API Style]" request list; Conventions → "Channel methods"; Occupied Resources → "Subcommands"; the MCP tool list (Stack row, "[MCP]", Occupied Resources → "Claude Code integration names"); and the `src/cmd/` tree comment. For each verb, is there a decision that says what it does? For each verb missing from MCP, is there a rationale?
   - **Hook events.** Compare the "[Hook Transport]" tiers, Occupied Resources → "Hook event arguments", Occupied Resources → "Registered hook events", and Conventions → "Normalised event kinds". Every hook must map to a normalised kind or state explicitly that it produces no event.
   - **Tokio.** Compare the Stack concurrency row, "[Concurrency / Backend Framework]", "[Module Boundaries]", `deny.toml` in Build system, CI job 3, Cross-cutting → "Tokio containment" and Inherited Defaults → Framework. All of them must name the same crate set and the same `wait` caveat.
   - **Dependencies → crates.** Every Stack technology must land in exactly one crate in "Crate dependency direction". That list must agree with the directory-tree comments, for example "tailing" under `viola-state`.
   - **Env vars.** Every variable in Occupied Resources → "Environment variables" needs a named reader in some Decision or Cross-cutting bullet.
   - **Filesystem and repository paths.** Match each path in Occupied Resources → Filesystem/Repository against a writer and a reader in the Decisions and the directory tree.
   - **Versioning vocabulary.** Check that `v`, `proto`, `sender`, `writer` and "major version" carry one meaning across Conventions → "Protocol versioning", Standard Contracts and Cross-cutting → "Mixed-version tolerance".
   - **Refusal and error mapping.** Line up `RefusalReason`, CLI exit codes `10–14/20/21`, the MCP `isError` shape and CLI `--json`. Each outcome must have exactly one representation on each surface.
   - **Inherited Defaults drift.** Every Inherited Defaults bullet must summarise a fact stated in a Decision or Convention. It must not drop a caveat, and it must not be the only place a fact appears.

3. **Check each dimension below, using its anchor example as a calibration of the kind of defect to find.** An anchor is one verified instance, not the whole finding. Look for siblings of the same defect elsewhere in the document.

4. **Out-of-scope discipline.** Some findings would require patching in another specialist's material:
   - test cases (tests' domain);
   - OTel span, metric or trace schemas (obs' domain);
   - `aria-` attribute names or WCAG conformance claims (a11y's domain);
   - design tokens, component patterns or typography (design's domain);
   - threat-model, tier or compliance content (security's domain);
   - naming a concrete logger, test framework, auth library, frontend framework, CSS tool or component library.

   Do NOT patch any of these. Instead, verify that the document states the boundary requirement: the "what must hold", not the "how it's wired". Patch only if that boundary itself is missing.

5. **Prioritise and budget.**
   - Patch `[priority: high]` dimensions 1–5 first. These are downstream-blocking: a specialist or `/implement` would stop or guess.
   - Patch dimensions 6–7 next. These are implementation-misleading.
   - Patch dimension 8 only if the fix is one line and cannot be deferred.
   - Before emitting any patch, re-check that its **Old** text is byte-exact and that the **New** text does not break an invariant listed above.

## Analysis Dimensions

### 1. Decision Completeness [priority: high]
- **Driver loop around dialogs.** A sync-tier PreToolUse (`AskUserQuestion|ExitPlanMode`) or PermissionRequest hook blocks the turn until someone sends `answer`. The only documented blocking primitive for the driver is `wait`, and it returns on `turn-ended`. Does any decision say that `wait` also returns on `question`, `permission` or `plan`? Does any decision say how the driver learns a dialog is pending and which dialog an `answer` targets?
- **Verbs with no decision.** `pause` and `last` appear in every verb list, but does any Established Decision say what either one does? For `pause`: does it take the wheel for the human, trigger a manual budget pause, or something else? How does it relate to `release` and `release --budget`, and why is it absent from MCP? For `last`: does it return the `Stop` payload's `last_assistant_message`, and from the log or from memory?
- **Unvalued blocking waits.** Check each of these:
  - the "confirmation window" in "[Delivery Confirmation]";
  - how long a sync hook blocks on `hook.dialog` before failing open, and the `timeout` that `hooks.json` sets;
  - how long `wait` blocks.

  For each one, state a default or a named open item, whether config or a ledger row owns it, and what the caller receives on expiry. A hook timeout must still fail open.
- **Wheel state machine.** Who holds the wheel when `viola run` starts? What does `send` return while a turn is running (before `turn-ended`): queue, block, or `not-delivered` with some `detail`?

**Anchor example:** Established Decisions, [Message Broker / IPC]

> "`hook` blocks on the channel for a dialog answer. `wait` blocks until `turn-ended`, so nothing polls."

**Issue:** A pending AskUserQuestion keeps the turn open, so `turn-ended` never fires until `answer` arrives. A driver parked in `wait` therefore never learns that an answer is needed.

Search evidence:
- Every `wait` occurrence was checked (Stack lines for concurrency, clap and MCP; [PTY]; [Screen Model]; [Message Broker / IPC]; [API Style]; [MCP]; Channel methods; Subcommands; MCP tools; `src/cmd` tree; Tokio containment). None extends `wait` to dialog events.
- `pending` returns zero hits.
- `dialog_id|request_id|tool_use_id` returns zero hits, so nothing states how `answer` identifies its dialog.

**Why this matters:** The first `/implement` of the driver loop either deadlocks until the MCP client times out or invents its own polling, which violates "nothing polls". The per-feature scope for `answer` cannot be written without re-asking.

**Adversarial:** A driver calls `wait` on the MCP surface, and the driven session raises a PermissionRequest. If nothing wakes the driver, which timeout fires first: the hook's `hooks.json` timeout (which fails open, so the dialog lands on the human) or the MCP client's? And did R7 ("driver answers dialogs") just silently fail?

### 2. Cross-reference Integrity [priority: high]
- **Hook → normalised kind map.** Registered hooks are SessionStart, UserPromptSubmit, PreToolUse, PermissionRequest, Stop, SessionEnd, Notification, PostToolUse and PostToolUseFailure. Which normalised kind does each one emit? SessionStart, Notification, PostToolUse and PostToolUseFailure have no listed kind. Add the missing kinds (with non-Claude names) or state "no event" per hook. Also check `statusline` in "Hook event arguments". It must print the user's statusline output, yet "Hook contract" says "No decision is exit 0 with an empty stdout". Is `hook statusline` exempt from the stdout rules, or should it be a separate subcommand?
- **Dependency placement.** `vt100` appears only in the Stack row and "[Screen Model]", and `jsonschema` only in the Stack row. Neither is placed in "Crate dependency direction". The tree gives `viola-state` "tailing", but `notify` is listed only under `viola-ui`. `sysinfo` is listed only under `viola-ui`, but the heartbeat `stale` check "from the snapshot" and CLI `list` also need liveness. Put each dependency in exactly one crate, and make the tree comments agree.
- **Env vars and TL;DR-only facts.**
  - `VIOLA_DIR` appears only in "[Naming]" and Occupied Resources, with no reader. "Identity by instance" says lookups use `VIOLA_NAME`. Name a reader or drop the variable.
  - `publish = false` appears only in Inherited Defaults. Promote it to a Decision or to Build system.
  - Occupied Resources calls port 47319 a "derived default", while Config says "default 47319". Derived from what?

**Anchor example:** Conventions, Naming patterns → Normalised event kinds

> "`turn-ended`, `prompt-submitted`, `question`, `permission`, `plan`, `session-end`, `link`, `unlink`. New kinds are added only in `viola-core`, never with Claude-specific names."

**Issue:** "[Delivery Confirmation]" says "For `/clear`, that post-condition is a SessionStart with source `clear` and a new `session_id`." The wrapper that confirms `/clear` receives hook traffic only as normalised `hook.event` frames, and no kind exists for a session start.

Search evidence: `SessionStart|session-start` hits only the [Delivery Confirmation] line, the [Hook Transport] tiers, the ledger `/clear` row, "Hook event arguments" and "Registered hook events". None of these assigns a normalised kind.

**Why this matters:** `viola-agent-claude` must either emit a Claude-shaped event (which breaks the isolation invariant) or drop SessionStart. If it drops it, every `/clear` send, which drivers issue between pipeline skills, reports `not-delivered`.

**Adversarial:** Suppose `viola-core` gains no session-start kind, and an implementer passes the raw `source: "clear"` payload through `data`. Which Standard Contracts rule is broken? ("`data` … never embeds a raw Claude payload")

### 3. Internal Consistency [priority: high]
- **Tokio crate set.** Do all seven Tokio sites (see Analysis Protocol 2) name the same set?
  - CI job 3 checks `viola-channel -p viola-pty -p viola-state`.
  - `deny.toml` also bans `viola-core` and `viola-agent-claude`, and `viola-agent-claude` is the crate on the hot `hook` path.
  - The `wait` caveat exists in the Stack row and in Cross-cutting, but not in "[Module Boundaries]" or Inherited Defaults.

  CLI `wait` lives in the root bin, so resolve "if it goes async" one way and align every site.
- **Versioning terms.**
  - Standard Contracts → Snapshot envelope says "a snapshot whose `v` major version is unreadable", but Conventions define `v` as a single integer.
  - `/api/info` carries `"proto":1` beside `"v":1`, and `proto` is defined nowhere.
  - Is `writer` in the snapshot envelope the same concept as `sender` in channel `params`?
- **"[MCP]" rationale vs tool list.** It justifies only `release` being CLI-only. `pause`, `link` and `unlink` are also absent from MCP with no stated reason.

**Anchor example:** Stack and Technologies, Backend framework row

> "Tokio 1.53.1 built only inside `mcp`, `ui` (and `wait` if it goes async)"

**Issue:** "[Module Boundaries]" states "Only `viola-mcp` and `viola-ui` list `tokio`, and `viola-channel`'s Tokio client sits behind a `tokio` feature only `viola-mcp` enables". Inherited Defaults says "Tokio 1.53.1 only in `mcp`/`ui`". An async CLI `wait` in `src/cmd/` would need either the root bin to list `tokio` or `viola-channel[tokio]`. Both contradict "[Module Boundaries]". The document holds two incompatible rules.

**Why this matters:** `/implement` of `wait` picks one reading. The `deny.toml` ban and CI job 3 then either fail the build or silently stop guarding the rule the KEYSTONE decision depends on.

**Adversarial:** A contributor adds `tokio` to `viola-agent-claude` for a timeout helper. CI job 3 does not check that crate. Is the only thing that catches it `cargo deny` on ubuntu? What would a Windows-only `cfg` dependency do to that guarantee?

### 4. Downstream Readiness (bidirectional) [priority: high]
- **Tests.** Can the test suite run in an isolated viola home? The Filesystem section fixes it at `<user home>/.viola/`, and Config forbids env vars as a configuration channel. How is the fake agent substituted in `viola run <name> -- claude <args>`, given that "npm-shim → `claude.exe` resolution" is a ledger row? Add an arch-level injection point (for example a `--home` flag or a documented test-only override) without naming a test framework.
- **Obs.** Where do diagnostics go when `viola hook` must exit 0 and async tiers print nothing on stdout? Is stderr allowed for hook? Is there a per-instance diagnostic file? State the requirement without naming a logger.
- **Security.** Is the trust model of `\\.\pipe\viola-<h12>` and `$TMPDIR/viola-<h12>.sock` stated? Any local process that connects can `send` prompts and `answer` PermissionRequest dialogs. Elevate this to a Decision or Cross-cutting bullet that security can consume.
- **Design / a11y / scope template.** `/` is described only as "the view page", and `/api/sessions` and `/api/links` `items` shapes are empty. Is there a specialist-facing list of what the page must show? That means session rows (wrapped vs read-only unwrapped, `stale`), wheel holder, budget age, links and `skipped` counts, plus the modalities (view-only, live SSE regions).

**Anchor example:** Occupied Resources, Filesystem heading, together with Cross-cutting Patterns → Config management

> "**Filesystem (viola home = `<user home>/.viola/`)**"

> "There are no secrets in v1, and environment variables are not a configuration channel."

**Issue:** No mechanism is given for pointing viola, and every hook it spawns, at a different home. CI "runs the workspace test suite against the fake agent" on shared runners, and parallel tests would collide on one `~/.viola/`.

Search evidence:
- Case-insensitive `home|--home|viola_home` hits only path templates, the `/ready` check key and the `/api/info` field. There is no flag or override.
- `fake agent` hits (Design Philosophy, Stack CI row, the ledger paragraph, "[CI/CD]", Repository, CI job 5, Inherited Defaults) never say how the agent is launched or where its state lives.
- `stderr` returns zero hits, and `ACL|trust|permissions|0600` hits only the GUI brake, "[Deferred]" and "Untrusted upstream text".

**Why this matters:** The tests specialist cannot write an isolation strategy without inventing an arch-level surface. The obs specialist has no permitted output channel for `hook`. Security has no stated socket trust boundary.

**Adversarial:** Two CI test cases both run `viola run builder -- <fake agent>` in parallel on the same runner. The endpoint hash is over `ViolaName` + viola home. Do they share one pipe name, one `events.ndjson` and one `budget.json`? Which test fails nondeterministically?

### 5. Specialist Content Boundary [priority: high]
- **Security.** "[GUI Control Scope]" and "[Deferred]" hand brake auth to security, yet "[GUI Control Scope]" already fixes the mechanism. Rephrase it as a reserved surface plus the requirement that must hold (state-changing routes must not be reachable cross-origin or without per-launch proof), and leave the mechanism to security.
- **Tests.** "[Deferred]" gives "the test framework, fake-agent harness and `viola verify` probe design" to tests. Do the Stack row "Fixture schema check | jsonschema 0.57.0 (optional)" and CI job 5 stay at contract level (fixture location, replay contract), or do they pre-pick harness tooling?
- **Design and obs.** Does anything imply a frontend build or asset pipeline beyond "embedded in the binary"? Is any logger crate named? (`tracing|logger` returns zero hits. Keep it that way when you patch Dimension 4.)

**Anchor example:** Established Decisions, [GUI Control Scope]

> "A brake (pause/unlink POSTs with Host + Origin checks and a per-launch token) is the v1.x step, owned by the security specialist."

**Issue:** The sentence names the auth mechanism (Origin check + per-launch token) in the same breath as assigning ownership to security. Occupied Resources then reserves `POST /api/sessions/{name}/pause` and `POST /api/sessions/{name}/unlink`, which is legitimate arch surface. The mechanism belongs to security.

**Why this matters:** The security specialist either rubber-stamps an unreviewed auth design or has to contradict arch.md, which creates a two-source conflict for the v1.x scope.

**Adversarial:** Suppose security later picks a different mechanism, for example an OS-user-bound socket instead of a token. Which arch.md sentences become wrong, and would `/implement` of v1.x follow arch or security?

### 6. API Contract Completeness [priority: high]
- **SSE resume across instances.** `/api/events` fans out every instance, but the browser returns one `Last-Event-ID`. Specify how the other instances resume: a composite cursor, replay-from-snapshot, or a per-instance query. Also specify what happens when an offset exceeds the file length, and what happens when a new instance appears mid-stream.
- **Channel `params`/`ok` shapes.** Only `send` and `hook.event` have example frames. Give the shapes for `wait`, `last`, `list`, `answer`, `pause`, `release`, `link`, `unlink` and `hook.dialog`. In particular:
  - the `from` field that "[Links]" requires, which is absent from the `send` example;
  - the dialog identifier `answer` carries;
  - the request and response of `hook.dialog`;
  - whether `list` is really a per-wrapper channel request (sent to which wrapper?) or a disk and `claude agents --json` read performed by the caller.
- **GUI `items` shapes.** Define the element fields for `/api/sessions` (liveness `live`/`stale`/gone, `status`, wrapped flag, wheel holder, budget reading + age) and `/api/links` (`driver`, `driven`). Also give an input-schema summary for the five MCP tools.

**Anchor example:** Standard Contracts, SSE feed

> "- `id:` is `<ViolaName>:<byte offset of the line in events.ndjson>`. The browser's `Last-Event-ID` on reconnect resumes the tail from that offset."

**Issue:** Each instance has its own `events.ndjson` (Standard Contracts: "`events.ndjson`, per instance"). One `Last-Event-ID` names one instance's offset, so on reconnect every other instance's position is lost. "Resumes the tail from that offset" is therefore defined for one file only.

**Why this matters:** `viola-ui`'s SSE handler will either replay every other instance from byte 0 (duplicate rows in the page) or from EOF (silently dropped `question`/`permission` events). The GUI then misreports exactly the dialog state a human watches for.

**Adversarial:** Instance `builder` restarts and its `events.ndjson` is replaced or truncated. What does a resume with `builder:48213` do if the file is now 900 bytes long? Does it error, clamp or replay?

### 7. Conventions Specificity [priority: high]
- **`v` in GUI bodies.** Conventions require `v` in every GUI JSON body, but the RFC 9457 example has no `v`. `/ready` returns 503 with its own shape while `urn:viola:problem:state-unreadable` is also a 503. Say which body `/ready` and `/health` use, whether Problem Details carry `v`, and each route's `Content-Type` (only `application/problem+json` and `text/event-stream` are stated).
- **Closed `detail` set.** "`detail` values are kebab-case strings, for example `input-not-ready` and `no-prompt-submitted`". List the closed set per `RefusalReason`, and state whether readers must tolerate unknown `detail` values the way `unknown` tolerates unknown reasons.
- **Exit 20/21 across surfaces.** How does "instance unreachable" (exit 21, no endpoint, so no channel frame) appear in MCP (an `isError` refusal? a JSON-RPC error with which code?) and in CLI `--json`? The same question applies to exit 20.

**Anchor example:** Conventions, Protocol versioning, against Standard Contracts, GUI HTTP error

> "an integer `v` starting at `1` in every channel `params`, ndjson event line, snapshot and GUI JSON body."

> "{"type":"urn:viola:problem:host-not-allowed","title":"Host not allowed","status":403,"

**Issue:** The Problem Details example is a GUI JSON body and carries no `v`, while `/health`, `/ready`, `/api/info` and the list envelope all do. Either Problem Details are exempt (which should be said) or the example is wrong.

**Why this matters:** `viola-ui` and the page's error handling will disagree about whether `v` is mandatory. A strict page-side reader that treats a missing `v` as corrupt would hide the 403 host-not-allowed message it exists to show.

**Adversarial:** A driver's MCP `send` targets an instance whose wrapper just exited. There is no channel endpoint, so no `result.refusal` exists to map. What does the MCP tool return, and can the driver distinguish "gone" from `not-delivered`?

### 8. Complexity Calibration [priority: medium]
- **Unbounded disk growth from "no daemon".**
  - Every dev rebuild adds a directory under `bin/<version>-<hash>/` and one under `plugin/<version>-<hash>/`.
  - `events.ndjson` only grows, and SSE ids are byte offsets into it.
  - `instances/<ViolaName>/` directories from crashed sessions are never reclaimed.

  Add minimal rules: who may delete a pinned copy, and when, without breaking a running session that still calls it? Is rotation allowed given byte-offset ids? Do not introduce a daemon.
- **Name collision.** What happens when a second `viola run` uses a `ViolaName` whose endpoint is live, or whose heartbeat is `stale`? Is it refused, does it take over, or is it undefined?
- **v1 weight.** Does the "Release (v1.x, not v1)" Stack row (dist 0.33.0, cargo-auditable 0.7.6, self_update 1.3.0) belong in the v1 Stack table, or in Project Intent → Scale path, where it is already repeated? Are cargo-modules and the optional jsonschema earning their place in a personal v1?

**Anchor example:** Occupied Resources, Filesystem

> "- `bin/<version>-<hash>/viola(.exe)`: the pinned copy of the binary that `viola run` creates if it is absent, where `<hash>` is a short hash of the exe's content."

**Issue:** "[Deployment / Distribution]" explains that "`CARGO_PKG_VERSION` stays `0.1.0` across development rebuilds", so every rebuild creates a new pinned directory, and the document gives no cleanup rule.

Search evidence:
- `rotat|prune|cleanup|clean up|garbage|reclaim|delete` hits only "garbage-read bug wezterm#6783".
- `collision|collide|already running|duplicate` hits only the version-collision sentence in "[Deployment / Distribution]".

**Why this matters:** In agent-driven development with frequent rebuilds, `~/.viola/bin/` accumulates one full binary per build. An ad-hoc cleanup added at implementation time could delete a copy that a live session's `hooks.json` still execs. Hooks would then fail to spawn, and every dialog would fall through to the human.

**Adversarial:** A user manually deletes old `bin/0.1.0-*` directories while a week-old `viola run` is still live. The pinned copy is gone. Does Claude Code report hook spawn failures, and does "hooks fail open" still hold when the hook binary does not exist at all?
