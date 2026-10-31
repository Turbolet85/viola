# Scope — Sanitised error surfaces

**Marker:** `2026-10-31-sanitised-error-surfaces` · **Version:** viola-0.1.0 · **Epoch:** Epoch 6 — Security hardening
**Working entry (verbatim title + hint):** Sanitised error surfaces — codes and fixed messages only in CLI --json, MCP
errors and channel error data, no paths or upstream text

## Intent
Every error a `viola` process gives to a machine reader carries a code and a fixed message and nothing else: no absolute
path, no upstream text, no anyhow chain holding a serde source, no value of a stripped `CLAUDE*` variable. The chunk
makes that property hold on the three surfaces the entry names, and proves it across them, so that the set of error
bodies is one a reader can enumerate and check, not a set of separate chunks' local claims.

## What it builds
- **CLI `--json` errors** — every error document a verb prints under `--json` (the `{"v":1,"error":…,"detail":…}` form)
  holds a closed code and a code-shaped or null `detail`; a failure the verb cannot name is no free text. [inferred]
- **MCP errors** — every `isError` result of `viola mcp`'s tools `send · wait · last · answer · list` carries codes only
  (`refusal` / `detail`, or `instance-unreachable`, `wrapper-fault`, `invalid-params`), its text `refused: <code>`, and a
  handler fault is exactly `"internal error"` / `data: null`. [inferred]
- **Channel error data** — every JSON-RPC error a wrapper answers carries its fixed message, and its `error.data` a code
  or null; `-32603` is exactly `"internal error"` / `data: null`. [inferred]
- **`thiserror` messages** — every product `Display` impl that can reach one of the three surfaces is a fixed message,
  holding no path and no payload. [inferred]
- **The proof** — a witness per surface that drives the failure, with a path, an upstream text and a planted stripped
  `CLAUDE*` value in play, and an enumeration a reader can check: the error bodies of the three surfaces named, so a new
  one that carries free text is visible. [inferred]

## Boundaries (out of scope)
- The exit-cause code catalogue, and the revive start-refusal documents owed to it — "Exit-cause code catalogue".
- Socket, peer-credential and home hardening — "Unix endpoint and home hardening".
- Problem Details and the GUI token, cookie, launch URL and a11y artifact clauses of the cap: carried to
  `.andromeda/residuals.md` for 0.2.0 by the founder's ruling of 2026-10-16 (the notes of
  `verification-matrix.json#v1-47`); the CLI half stands unweakened.
- Any frontend surface — deferred to 0.2.0.
- Windows-only live readings — "Windows-only live measurements".
- Testing past what the plan lists (inputs#I1 item 5).

## Capability
- Candidate claim: `verification-matrix.json#v1-47` (Sanitised errors and the never-log floor), its web clauses out per
  its own notes. [inferred: P4 confirms the claim lean against the cap's requirement and the route. The acceptance also
  asks that content and drift reports reach only owner-readable detail files and that a secret scan after every
  end-to-end test proves the floor; P3 reads whether each stands at HEAD, and a leg this chunk cannot prove is no claim.
  No working-route line names `v1-47` (`command grep -n 'v1-47' viola-0.1.0/working-route.md`, 0 lines)]

## Folded freight and observations
- CARRY (chunk 2026-09-29-verify-stamped-test-homes-and-harness, operator word at its wrap: "carry the Refusal-enum
  line"): `viola-agent-claude` still holds a second thiserror enum, `Refusal` (`BatchScriptChild`, `NotFound`, the
  `resolve_program` refusal), beside `AgentError` (which now holds `Malformed` and `StampsMalformed`); architecture
  [Error Handling] names it the one open divergence from the one-enum rule — fold it into `AgentError` or have it
  ratified as an exception. [inferred: coordinate read at HEAD, `crates/viola-agent-claude/src/lib.rs:195` and `:209`;
  `AgentError` holds five variants now, not two; P3 reads every caller]
- CARRY (chunk 2026-10-04-confirmed-send-with-cl-1-records): it added `ProtocolError::InvalidParams` (`-32602`
  `"invalid params"`, `data: null`) beside the unsupported-version and `release-from-driver` causes of the same code; the
  `channel-response` line carries only `error_code`, and the client's `fault_of` labels every `-32602`
  unsupported-version, so `viola.compat.v_rejected` over-counts (obs-plan §5) until the line tells the causes apart.
  [inferred: coordinates read at HEAD, `crates/viola-channel/src/lib.rs:62`, `crates/viola-channel/src/client.rs:210`,
  `.andromeda/obs-plan-detail/metric-coverage.md:28`; P3 closes the mechanism, and the line's change is a contract read
  against obs-plan]
- PREREQ (chunk 2026-10-27-bounded-inputs-at-every-boundary, its wrap directive item 4,
  `chunks/2026-10-27-bounded-inputs-at-every-boundary/inputs/I4-wrap-bounded-inputs-directive.md.txt`; a missing test for
  a clause a chunk landed folds into the next chunk): that chunk landed two clauses no test drives to their edge:
  1. a `viola mcp` stdin line past `MAX_FRAME` that lands before `initialize` completes ends `serve` with
     `ServeError::Initialize` (exit 11), and no test drives it;
  2. five `BOUNDARIES` rows of `tests/contract_bounded_readers.rs` name a witness that reads the bounded read but drives
     nothing past its cap — the status-line command's stdout, the child pipe drain shared by `claude --version` and
     `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` —
     owed a cap-driving case each (test-plan §4 viola-mcp and §6 Contract suite name this entry their owner).
  [inferred: the rows named are read at HEAD by name in `tests/contract_bounded_readers.rs`; P3 reads each witness]
- watch: Path 1's CI timeout — `path1_run_start_one_home_in_order` killed at the ci profile's 20 s line on
  `test (ubuntu-latest)` at `ci#40425866096`; measured profile on the Linux dev host ≈ 6.07 s, the `stamped_home` fixture
  4.88 s or 80 %, the four product starts 0.93 s, the test's own file I/O 0.16 s; CI green at 13.468 s and 8.219 s, red
  at 20.089 s (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md`); green at 15.010 s
  (`SLOW [> 10.000s]`) on `test (ubuntu-latest)` at `ci#40458923095` over `607792cb`, no cause claimed for the slower time
  (`chunks/2026-10-27-bounded-inputs-at-every-boundary/evidence/operator-pass.md`); the kill line does not move, and a
  recurrence reopens the owner question for the founder (1/3; since
  2026-10-27-pinned-copy-race-and-reproducible-builds; retires: a recurrence, or 3 green runs). Tallied on this chunk's
  CI reads per the directive (inputs#I1 item 7).
- [inferred] The WATCH's causal claim, its marker kept verbatim: "read as runner variance and not a product red — the
  operator's technical answer, the overseer, founder-delegated, PROVISIONAL"
  (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/inputs/I6-wrap-pinned-copy-directive.md.txt` item 3).
- CI (Setup 5a): `5273ab8c68e8` — verdict: green · checks 15/15 · wall 709 s · `ci#40467224735` completed/success.
  `7bf5c32add0e`, `2cedb584b3ad`, `8ebed5bc0c81` — verdict: not on the remote (the records door's three local commits,
  which go out with this chunk's normal pushes, inputs#I1 item 9); noted. No red to disposition.

## Operator directive
- The operator's requests for this phase (inputs#I1, the overseer, founder-delegated): the sampler forced on this phase
  and on this chunk's wrap; every subagent and Workflow transcript and the run dir kept; a hard cascade failure, agents
  writing nothing, or agents doing the orchestrator's work stops the run and is reported with the run dir, with no
  re-spawn around it; a boundary widening, a live-session cap or a route split is the founder's live word, asked as a P4
  dialog; no epoch boundary inside Epoch 6; `df -h /tmp` read before a stamped-home gate; `target/` cleaned on 2026-10-27,
  so the next gate block rebuilds cold; no testing past what the plan lists; anything the records store changes under a
  skill is reported at once (an append refused or slow, a guard refusal on `.andromeda/db/` or the old ledger, a
  bookkeeping list reading the journal as dirty work, a health row off). [inferred]
