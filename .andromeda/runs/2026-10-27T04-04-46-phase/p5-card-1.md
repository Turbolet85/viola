# P5 review — 2026-10-27-bounded-inputs-at-every-boundary

**Plan:** `viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/plan.md` (462 lines; the prose outside the
`[[gate]]` fence about 250) · **Research:** `…/research.md` · **Scope:** `…/scope.md` (P3's closure: one bullet
premise-corrected, six verified)

## Goal
Every input that crosses into a `viola` process is bounded where it enters, and the bound set is checkable. Most of it
stands (24 of the 31 product reader sites read through `take(…)`; every `instances/<name>` join takes a `ViolaName`;
`validate_paste_text` runs at all six send and answer sites on both sides). The chunk closes what does not:
1. **`viola mcp`'s stdin is bounded** — rmcp 3.4.1 reads each line with an unbounded `read_until` (`inputs#I2`). A
   line-capped reader of viola's own (`crates/viola-mcp/src/stdin.rs`) passes a line of at most `MAX_FRAME` bytes, its
   `\n` counted, whole; a longer one passes no byte and ends the session as stdin's close does, with one
   `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line (the wrapper channel's own rule for an over-long line).
2. **The last unbounded compare read under the home** (`replace_private_shared`) reads at most `bytes.len() + 1`.
3. **The depth limit gets a gate** — a `deny.toml` ban on serde_json's `unbounded_depth` with a firing probe.
4. **The panic degrade's passthrough half is witnessed** in `tests/chaos_feed_panic.rs`.
5. **The enumeration** — `tests/contract_bounded_readers.rs`: every external reader named with its cap, depth, type
   rule and witness, plus a census of the product sources that reads red on an unnamed or unbounded reader call.

## Top acceptance criteria
- `verification-matrix.json#v1-46` claimed, proved by `tests/contract_bounded_readers.rs` green with every boundary
  row's named witness green in the block
- A `viola mcp` line past `MAX_FRAME` ends the session after the earlier lines are answered: exit 0, stderr empty, one
  codes-only `mcp-stdin` / `oversize` line
- `bash scripts/deny-probes.sh` ends `deny-probes: 14/14 banned, control clean`; `cargo deny check` clean
- After a forced vt100 panic: `send` refused `input-not-ready`, a key reaches the child, no viola literal on the outer
  terminal on every OS, the panicking bytes passed through on Unix

## Touchpoints
- New: `crates/viola-mcp/src/stdin.rs` · `tests/contract_bounded_readers.rs`
- Modify: `crates/viola-mcp/src/lib.rs` · `crates/viola-state/src/fs.rs` · `schemas/diag-line.v1.json` · `deny.toml`
  · `scripts/deny-probes.sh` · `tests/chaos_feed_panic.rs` · `crates/viola-e2e/tests/mcp_tools.rs`

## Test commands (27 entries)
Lints (fmt, clippy, the Windows-target clippy, G1, lint-probes) · unit (the suite; the two new by-name filters) ·
integration (eleven existing binaries the boundary rows name) · E2E (the five existing binaries; the new
`contract_bounded_readers` on its own) · probes (`cargo deny check`, `deny-probes.sh`, viola-state's sync ban,
`rmcp-features.sh`, the guard at base `4b0bf776`) · the harness smoke with G2 and G4 · pre-push · the operator's
hygiene, push and `ci.py` entries. Baselines on HEAD `4b0bf776`: 16 green; 2 red as the plan predicts (deny-probes
`13/13` before step 6; the new binary's entry before step 8), each with its control (`p5-controls.md`: deny-probes
must-pass `14/14` in a clone with step 6 planted; the guard clean / allowed / red-on-three-outside). No live leg, no
`run --perf` (no timed hook path moves), no fuzz target.

## Concretization preview — `v1-46`
- **Current acceptance:** «Every external reader is size-capped, depth-limited and closed-typed; names are validated
  before any path join; a malformed resume-cursor pair drops the whole resume header; the paste rule holds client-side
  and authoritatively in the wrapper; a terminal-parser panic degrades to input-not-ready without harming
  passthrough.» (notes: the resume-cursor clause carried to 0.2.0 by the founder's ruling of 2026-10-16)
- **Proposed:** the same sentence less the resume-cursor clause (per its notes), made concrete as:
  `tests/contract_bounded_readers.rs` green — every boundary row's bounded read stands in its file and its named
  witness exists, every product reader call outside the named own-executable exceptions reads through `take` in its
  own statement, and `deny.toml` bans serde_json's `unbounded_depth`; `deny-probes.sh` `14/14`; the `viola mcp` line
  past `MAX_FRAME` ending the session with one `parse-rejected{mcp-stdin, oversize}`
  (`mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`); the paste rule's client and wrapper witnesses
  green; `chaos_feed_panic_refuses_send_and_keeps_passthrough` asserting the `input-not-ready` refusal, a key reaching
  the child, no viola literal on the outer terminal on every OS and the child's panicking bytes on the outer terminal
  on Unix.
- **Where it is narrower than the sentence, named so you can judge it:** "without harming passthrough" for the
  child's OUTPUT is asserted byte for byte on Linux and macOS only; on `windows-2025` the case asserts no viola byte on
  the terminal and the key reaching the child, but not the child's bytes, because ConPTY re-renders a wide character
  on a 1-column screen with no guaranteed form (research §Open questions). The Windows reading is the Beyond-the-plan
  row below, never left unowned.

## P2 notes
- Not a sampled run (`cascade-sample.txt`: `sample: no`, draw 2 of 10): no `sampler:` line.
- design-cascade-g1 halted on `BODY_UNDER_ENTITY` after its one retry; overseer1's ruling (`inputs#I2`) ran a one-off
  second re-spawn, which passed. The fallback (accept and note) was not needed. Observed (the F364 concern): the
  ruled re-spawn turned five sibling brand-identity items (`design-system.md:10`, `:43`, `:21`, `:14`, `:15`) into
  runs under the brand-identity item, not the one row named — the check prints only its first row, so the other four
  were the same defect; every quote stands, their five claim sentences are gone, the accounting is byte-identical.
  Nothing binding is lost: the plan takes its design and security facts from the design and security extracts.
- One cross-master contradiction surfaced and resolved at synthesis: architecture's [Screen Model] detail text names
  `oversize` for every vt100 poisoning episode; the code and the security and obs plans give `panicked` for a caught
  panic. The plan carries it as an Expected amendment of architecture.

## Expected amendments (wrap)
security-plan §Input Validation (the `viola mcp` stdin row, the Constants list) and §Dependencies (the ban, the probe
count `13/13` → `14/14`) · obs-plan §6 (`mcp-stdin` in the `parser` catalog, a Decisions Log entry) and §1 / §4 ·
architecture [MCP] (the stdin bound) and [Screen Model] (the detail correction) · test-plan §5 / §6 / §9 · a11y-plan
§1 (the passthrough witness, its Windows byte reading owed).

## Beyond the plan
- A CARRY on «Windows-only live measurements»: after a contained vt100 feed panic under a 1-column ConPTY, read how
  the child's panicking wide character (`e4 b8 ad`) reaches the outer terminal on a Windows host, and give
  `tests/chaos_feed_panic.rs` its Windows byte oracle from that reading (this chunk asserts the bytes on Unix and only
  viola's literal absence on Windows).
