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
  holds a closed code and a `detail` that is null, a closed word, or `wrapper-fault`'s `{code, message, data}` with every field fixed or closed; a failure the verb cannot name is no free text. [premise-corrected: the contract's `wrapper-fault` `detail` is an object (architecture §Standard Contracts, CLI `--json` output), and `fault_detail` and `reply_of` copy the peer's `message`, `data`, `refusal` and `detail` (`src/cmd/client.rs:76`, `:86`): the chunk screens them]
- **MCP errors** — every `isError` result of `viola mcp`'s tools `send · wait · last · answer · list` carries codes only
  (`refusal` / `detail`, or `instance-unreachable`, `wrapper-fault`, `invalid-params`), its text `refused: <code>`, and a
  handler fault is exactly `"internal error"` / `data: null`. [premise-corrected: `crates/viola-mcp/src/result.rs:65` and `:80` carry the same peer text onto the result and into the text `refused: <reason>`: the chunk screens them through one shared reader; the connection's own faults verified fixed, `unknown tool` and rmcp's]
- **Channel error data** — every JSON-RPC error a wrapper answers carries its fixed message, and its `error.data` a code
  or null; `-32603` is exactly `"internal error"` / `data: null`. [premise-corrected: the contract's `-32602` carries closed objects, `{supported, wrapper}` and `{"reason":"release-from-driver"}` (`crates/viola-channel/src/lib.rs:95`, pinned by literals): the rule is closed fields and no free text, the wire form unchanged and `v` unchanged]
- **`thiserror` messages** — every product `Display` impl that can reach one of the three surfaces is a fixed message,
  holding no path and no payload. [verified: 0 of 36 `#[error(` attributes interpolate, research.md §Sweeps; the census holds it]
- **The proof** — a witness per surface that drives the failure, with a path, an upstream text and a planted stripped
  `CLAUDE*` value in play, and an enumeration a reader can check: the error bodies of the three surfaces named, so a new
  one that carries free text is visible. [verified as owed: no such table exists, and test-plan §6's after-every-test error-body check was never built (`tests/support/events.rs`); the check lands there, read by the witnesses]

## Boundaries (out of scope)
- The exit-cause code catalogue, and the revive start-refusal documents owed to it — "Exit-cause code catalogue".
- Socket, peer-credential and home hardening — "Unix endpoint and home hardening".
- Problem Details and the GUI token, cookie, launch URL and a11y artifact clauses of the cap: carried to
  `.andromeda/residuals.md` for 0.2.0 by the founder's ruling of 2026-10-16 (the notes of
  `verification-matrix.json#v1-47`); the CLI half stands unweakened.
- Any frontend surface — deferred to 0.2.0.
- Windows-only live readings — "Windows-only live measurements".
- Testing past what the plan lists (inputs#I1 item 5).
- The human exit-2 lines and clap's usage render: stderr text with no `--json` document (the design table's row 2), named in the enumeration and left as they are. [verified: `src/main.rs:58`, `src/cmd/send.rs:187`]

## Capability
- Candidate claim: `verification-matrix.json#v1-47` (Sanitised errors and the never-log floor), its web clauses out per
  its own notes. [verified: both other legs stand at HEAD and are named as witnesses, the drift report's detail-only write (`src/cmd/hook.rs:787`) with the owner-only opens (`src/obs.rs:660`) and the `secret-scan` step (`.github/workflows/ci.yml:104`); no working-route line names `v1-47` (`command grep -n 'v1-47' viola-0.1.0/working-route.md`, 0 lines)]

## Folded freight and observations
- CARRY (chunk 2026-09-29-verify-stamped-test-homes-and-harness, operator word at its wrap: "carry the Refusal-enum
  line"): `viola-agent-claude` still holds a second thiserror enum, `Refusal` (`BatchScriptChild`, `NotFound`, the
  `resolve_program` refusal), beside `AgentError` (which now holds `Malformed` and `StampsMalformed`); architecture
  [Error Handling] names it the one open divergence from the one-enum rule — fold it into `AgentError` or have it
  ratified as an exception. [premise-corrected: `AgentError` holds five variants, not two, and the masters now name three open divergences, not one (`Refusal`, `ServeError`, `StampsError`), while the census reads ten `thiserror` enums and no `CoreError` (research.md §Sweeps). This freight owns `Refusal`, folded here; the rest is named at P5]
- CARRY (chunk 2026-10-04-confirmed-send-with-cl-1-records): it added `ProtocolError::InvalidParams` (`-32602`
  `"invalid params"`, `data: null`) beside the unsupported-version and `release-from-driver` causes of the same code; the
  `channel-response` line carries only `error_code`, and the client's `fault_of` labels every `-32602`
  unsupported-version, so `viola.compat.v_rejected` over-counts (obs-plan §5) until the line tells the causes apart.
  [verified: `fault_of` labels every `-32602` `UnsupportedVersion` (`crates/viola-channel/src/client.rs:215`), the label reaching only the code and the level; the line's `detail` is already admitted on `channel-response`, and the chunk writes the cause there, a closed word on both sides]
- PREREQ (chunk 2026-10-27-bounded-inputs-at-every-boundary, its wrap directive item 4,
  `chunks/2026-10-27-bounded-inputs-at-every-boundary/inputs/I4-wrap-bounded-inputs-directive.md.txt`; a missing test for
  a clause a chunk landed folds into the next chunk): that chunk landed two clauses no test drives to their edge:
  1. a `viola mcp` stdin line past `MAX_FRAME` that lands before `initialize` completes ends `serve` with
     `ServeError::Initialize`, and no test drives it; [premise-corrected: exit 11 is the unit test child's own code for that error (`crates/viola-mcp/src/lib.rs:123`); the `viola mcp` process exits 1 with both streams empty (`src/cmd/mcp.rs:36`, `src/main.rs:137`), so the test reads both, and the masters' "exit 11" is corrected at the wrap]
  2. five `BOUNDARIES` rows of `tests/contract_bounded_readers.rs` name a witness that reads the bounded read but drives
     nothing past its cap — the status-line command's stdout, the child pipe drain shared by `claude --version` and
     `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` —
     owed a cap-driving case each (test-plan §4 viola-mcp and §6 Contract suite name this entry their owner).
  [verified: each of the five rows' witnesses reads no byte past its cap (`tests/contract_bounded_readers.rs:91`, `:109`, `:118`, `:127`, `:217`)]
- watch: Path 1's CI timeout — `path1_run_start_one_home_in_order` killed at the ci profile's 20 s line on
  `test (ubuntu-latest)` at `ci#40425866096`; measured profile on the Linux dev host ≈ 6.07 s, the `stamped_home` fixture
  4.88 s or 80 %, the four product starts 0.93 s, the test's own file I/O 0.16 s; CI green at 13.468 s and 8.219 s, red
  at 20.089 s (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md`); green at 15.010 s
  (`SLOW [> 10.000s]`) on `test (ubuntu-latest)` at `ci#40458923095` over `607792cb`, no cause claimed for the slower time
  (`chunks/2026-10-27-bounded-inputs-at-every-boundary/evidence/operator-pass.md`); the kill line does not move, and a
  recurrence reopens the owner question for the founder (2/3 with Setup 5a's reading: green at 14.205 s at `ci#40467224735`; since
  2026-10-27-pinned-copy-race-and-reproducible-builds; retires: a recurrence, or 3 green runs). Tallied on this chunk's
  CI reads per the directive (inputs#I1 item 7).
- The WATCH's causal claim, its marker kept verbatim: "read as runner variance and not a product red — the
  operator's technical answer, the overseer, founder-delegated, PROVISIONAL"
  (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/inputs/I6-wrap-pinned-copy-directive.md.txt` item 3).
  [verified against its recorded runs, not HEAD: a runner-only observation whose non-reproduction here is its expected state; the cause stays PROVISIONAL as written, and this chunk changes no part of Path 1's start sequence]
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
  bookkeeping list reading the journal as dirty work, a health row off). [verified: inputs#I1 holds these items as written]
