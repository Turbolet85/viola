# security · Security Anti-Patterns — category read

## Carried
- (.andromeda/security-plan.md:482) The three surfaces must never carry a drift report or an anyhow chain, and a chain printed to stderr must hold no serde source; this is the ban the chunk's sanitised bodies and its `thiserror` messages must satisfy.
  > - NEVER echo serde_path_to_error drift reports or anyhow chains into MCP `structuredContent` or channel `error.data`, and NEVER print an anyhow chain to stderr that still contains a serde_json or serde_path_to_error source (its `Display` quotes input values such as tool `input`).
- (.andromeda/security-plan.md:487) Every error body the enumeration names, on each of the three surfaces, must hold no absolute path, and each surface's witness must drive its failure with a path in play to prove it.
  > - NEVER expose absolute paths in any external error.
- (.andromeda/security-plan.md:476) No stripped `CLAUDE*` value may reach any error output or its full-detail file; the witnesses' planted value must prove this, and a planted value must be synthetic and never a real credential.
  > - NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log or stderr, `events.ndjson`, snapshots, `diagnostics/` or fixtures (consolidated floor: Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan.md:486) Any full error detail the chunk moves off a surface must land only in a `diagnostics/` file no other user can read; this is the owner-readable leg of the candidate claim `v1-47`.
  > - NEVER create `diagnostics/` files readable by other users.
- (.andromeda/security-plan.md:483) Sanitising must not give `viola hook` an error surface: it stays silent on stderr and exits zero when a channel error or a security check fails.
  > - NEVER write to stderr from `viola hook`, and NEVER exit non-zero from it, including on security-check failures.
- (.andromeda/security-plan.md:510) A sanitised refusal must still go only to automation and must leave the hook's fail-open path intact, so no change to an error body may turn into a block on the human.
  > - NEVER let a security refusal block the human. Refusals go only to automation (`send`/`answer`), and hooks fail open.
- (.andromeda/security-plan-detail/security-anti-patterns.md:39) A witness must drive its failures without a new seam or environment read unless that seam has its own Decisions Log entry; the two existing seams stay test-only, and a colour fact must reach no error or refusal body.
  > The two carve-outs are the test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s; Decisions Log 2026-09-27) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`, a forced fail-open `hook` panic; Decisions Log 2026-09-28): each exists only under `cfg(feature = "fake-agent")`, is absent from every release build and switches off nothing (§Input Validation). The terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build (the founder's own word, chunk 2026-10-17-cli-output-discipline), only by `viola list`'s human mode and only to choose whether and how deep SGR bytes are written: they switch off no control (no escaping, no `validate_paste_text`, no check), widen no redaction and are not a configuration channel (§Input Validation). NEVER add another such seam or environment read, let these reach a release build, or let a colour fact change a control or reach a log line, error or refusal, without its own Decisions Log entry.
- (.andromeda/security-plan-detail/security-anti-patterns.md:32) The enumeration of CLI error bodies must keep the code and exit of this refusal unchanged, and the hook's side of it must stay silent.
  > the hook prints nothing and `answer` exits 12 `unverified-cli`.
- (.andromeda/security-plan-detail/security-anti-patterns.md:33) A fixed message given to this refusal must not change what it does: it still reads and writes nothing.
  > a refusal (`StampsError::StrictModes`) reads and writes nothing.
- (.andromeda/security-plan.md:452) The enumeration must keep `control-character` as the code of this refusal, and a text holding a refused character must still be refused and never stripped.
  > and NEVER strip them silently. Reject with `control-character`.
- (.andromeda/security-plan.md:465) Folding `Refusal` into `AgentError`, or ratifying it as an exception, must keep the refusal of a `.cmd`/`.bat` child in force, with a fixed message.
  > - NEVER spawn a `.cmd`/`.bat` PTY child through portable-pty. Resolve to a real `.exe` (portable-pty 0.8.1 bypasses std's BatBadBut escaping).
- (.andromeda/security-plan.md:454) The folded PREREQ's cap-driving cases must prove these bounds at their edge: the child pipe drain's cap, and a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes, which must end the session and never be truncated or skipped.
  > - NEVER call `read_line` on a channel stream or `read_to_end` on hook stdin or child output without `Read::take(MAX_FRAME)`, and NEVER hand rmcp a `viola mcp` stdin reader that does not cap each line at `MAX_FRAME`, nor truncate or skip an over-long line instead of ending the session.
- (.andromeda/security-plan.md:472) The same PREREQ case must hold this limit on `viola mcp` stdin, and the error it drives must carry no byte of the refused line.
  > - NEVER skip request size limits: `MAX_FRAME` on the channel and on each `viola mcp` stdin line.
- (.andromeda/security-plan-detail/security-anti-patterns.md:23) Any new witness or cap-driving case that lays a test home must take its removal through the test side's own creations, never by a wider sweep.
  > NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created: the root fixture chain's owner sweep and a test home's drop (by `owner.json` record, by its own `TempDir`) and harness `cleanup` (by the `viola-session-` name).

## Bindings
- **Logging bans ↔ body §Error Handling**: the bans on chains, drift reports and paths are the prohibition side, and the fixed `-32603` and MCP `isError` shapes of §Error Handling are the required form; a body must satisfy both, (.andromeda/security-plan.md:482) with (.andromeda/security-plan.md:413)
- **Absolute-path ban ↔ body §Bootstrap phases `logging-redaction-wire`**: the floor names the three surfaces and its Scope line keeps `wait`/`last` contract payloads outside it, so the enumeration covers error bodies only, (.andromeda/security-plan.md:487) with (.andromeda/security-plan.md:346)
- **Stripped-value ban ↔ obs-plan §9 artifact canary scan**: the secret-scan leg of `v1-47` reads the `claude-stripped` needles of `viola-harness secret-scan`; the claim holds only if that scan proves the floor at this chunk's reads, (.andromeda/security-plan.md:476) with (.andromeda/security-plan.md:381)
- **Hook silence ↔ body §Error Handling**: the hook's exit 0 with no body must hold for every failure the chunk re-words, (.andromeda/security-plan.md:483) with (.andromeda/security-plan.md:418)
- **Seam ban ↔ body §Input Validation test-seam rows**: a new failure-driving seam must meet the same terms as the two rows, test-only and with its own Decisions Log entry, (.andromeda/security-plan-detail/security-anti-patterns.md:39) with (.andromeda/security-plan.md:152)
- **`.cmd`/`.bat` ban ↔ architecture [Error Handling] one-enum rule**: the architecture owns the fold-or-ratify question for `Refusal`, and this ban owns what the refusal must keep doing whichever way it goes, (.andromeda/security-plan.md:465)
- **stdin cap ↔ test-plan §4 viola-mcp and §6 Contract suite**: the test plan names this entry owner of the cap-driving cases, and this ban sets what those cases must prove, (.andromeda/security-plan.md:454) with (.andromeda/security-plan.md:142)
