# Phase 0 — Input Validation Dialogue

**User input:**
Viola: a standalone, cross-platform (Windows first) bridge that lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified claude CLI in a pseudo-terminal, types into the driven session at turn boundaries, answers its dialogs through hooks, holds a one-driver wheel the human can take at any moment, and shows the active and linked sessions in a minimal local GUI. The full brief (decisions, measurements, open items, the first consumer) is refs/viola-brief.md; prior art is refs/viola-prior-art.md. The throwaway spike and prototype that proved the mechanisms live outside this repo in D:\dev\projects\additional\viola-lab and are reference material only, not this project's code.

**Clarifications (if any):**
- None — input parseable on first pass (refs/ read at full fidelity).

**Confirmed summary:**
- Type: Cross-platform CLI tool (single native binary) + Claude Code plugin (hooks + MCP) + minimal local web GUI
- Core idea: A bridge that lets one interactive Claude Code session drive another on the user's own subscription by wrapping the unmodified `claude` CLI in a pseudo-terminal.
- Key aspects: typed-input authority at turn boundaries + hook-answered dialogs; one-driver wheel the human can take; Windows first, no daemon, ndjson on disk, localhost GUI; subscription only, unmodified binary, budget governor in v1.

**User confirmation:** "yes, correct: slug viola"

**Slug:** viola

# Phase 3 — Quiz I Dialogue

**Pre-filled confirmations:**
- Platform: Hybrid (Rust binary run·send·wait·hook·mcp·ui + Claude Code plugin hooks+stdio MCP + local web GUI) — confirmed
- Primary Language: Rust — confirmed
- Growth Model: modular monolith (pty · channel · hook · mcp · ui · state), no daemon — confirmed
- Development Style: agent-driven — confirmed
- Viewer Surface and Reach: 127.0.0.1 web page, phone view later behind auth — confirmed
- Core Functionality: confirmed
- User: "yes, all correct"

**Core field answers (no research):**
- Q: Scale Intent — options: personal, startup, production
  A: "1, personal, with one amendment the founder made explicitly: decision D3 still holds. v1 serves the founder on their own subscription (no accounts, no hosting, GUI on 127.0.0.1), but it is cross-platform from the first commit: Windows first, the macOS and Linux code paths built and CI-tested on all three OSes against the fake agent, and the on-disk state parsed defensively from day one. Public distribution (signed binaries, winget/Homebrew/Scoop, a marketplace entry) is a later version, not v1."
  (This also settled field 14, Cross-Platform Release Bar; it was not asked separately.)
- Q: Target Users — options: just me, individual subscribers (public), developers building pipelines on viola
  A: "1, just me for v1: the founder's own overseer and builder pairs on Pulse, Conductor and viola itself. One boundary from D1 and R4 still binds: viola stays a general bridge, with nothing Andromeda-specific inside it. The Andromeda decision rights (brief Appendix A) belong to the driver's side, never to viola. Option 2 (individual subscribers) is the audience of the later public version."

**Process condition set by the founder (during field 9):**
- Research agents get input.md by path, not pasted inline: "Keep the path. One condition: every research agent reads input.md in full from disk, and its recommendation cites the brief sections it relied on. If a return shows no sign it read the file, re-run that agent with the file inline." The first field-9 return cited only the passed excerpts → re-run with the file inline. Every later return was audited: each cited sections beyond its excerpts.

**Research-derived field answers:**
- Q: Driven-Agent Coverage
  Stakes: whether the core is Claude-Code-specific or carries a driven-agent adapter seam
  Research findings (re-run, file inline):
    - Options: 1) Claude Code only all the way through, 2) Claude-first core + one `agent::claude` adapter module, no trait yet, 3) DrivenAgent trait + registry in v1, 4) several agents in v1 (Claude + Codex CLI 0.150 + Gemini CLI), 5) generic keystroke+screen adapter (agentapi-style)
    - Recommended: 2
    - Reasoning: "just me" rules out multi-agent now; D1/R4/R6 rule out hard-wiring; keeps the fast-moving contract (S3/S7/S8, S6/R8, §4) in one module the fake agent mirrors (§3.4); trait only once a second agent exists; neutral events keep R7 and avoid agentapi (prior-art §4)
  User response: "Accept 2."
  Final answer: option 2
- Q: GUI Control Scope in v1
  Stakes: whether the page is a code-execution surface in v1, localhost hardening, second-driver arbitration
  Research findings:
    - Options: 1) view-only (GET + SSE, Host allowlist), 2) view + brake (pause/unlink POSTs, token), 3) view + full wheel, 4) mission control
    - Recommended: 1
    - Reasoning: brief §3.2 part 4 "read-only first"; R2 + D4 make takeover a keystroke; `release` from the CLI passed in §6; keeps `ui` the §3.4 viewer; no code-execution surface (§3.4, §7); Host allowlist still needed because the feed carries prompt text and last_assistant_message (§4.1 S2)
  User response: "Accept 1."
  Final answer: option 1
- Q: Human Takeover Behavior (the Wheel)
  Stakes: interleaving of human keys with a bridge paste, keyboard responsiveness, automatic pause, release
  Research findings:
    - Options: 1) after-the-fact only, 2) byte-source claim + atomic send window + after-the-fact confirm, explicit release, 3) lock human keys for the whole driven turn, 4) explicit chord take/release, 5) vt100 empty-input-box check
    - Recommended: 2
    - Reasoning: D4/R2 rule out 3/4; §4.1 S1 shows paste+Enter is one prompt, so option 1 lets drafts merge; no screen parsing (R7, §4.1 Timing, prior-art §4); run knows every byte's source; CLI-only release matches the view-only GUI, R4, §6; no precedent (prior-art §5)
  User response: "Accept 2, with two facts measured today (2026-09-23, CLI 2.1.280, this very session driven by the prototype). Fold them into the wheel rule. (a) Harness-injected turns fire UserPromptSubmit: a subagent hand-back arrives as a prompt starting with "<agent-message from=" and a background-task notice as one starting with "<task-notification>". The prototype took both for a human take and paused automation twice. The prompt-submitted classifier must file them as harness turns, never a human take. (b) CLI-native modal screens bypass every hook. After a Stop, the CLI opened "Teach auto mode about your environment?", status still read idle, and the next paste+Enter answered that modal (it picked Yes and opened /auto-mode-setup) instead of reaching the model. So a turn boundary per the hooks is not proof the input box is ready. At minimum, send must confirm delivery: no matching prompt-submitted within a short window means the send is reported as not delivered, never assumed."
  Final answer: option 2 + (a) + (b)
- Q: Budget Governor Behavior (v1)
  Stakes: the policy answer to "ordinary, individual usage", mid-task stalls, statusline wrapping, missing figures
  Research findings:
    - Options: 1) advisory only, 2) soft stop at the turn boundary, 3) hard stop + pacing, 4) driver-side governor, 5) soft stop fail-closed
    - Recommended: 2
    - Reasoning: §7 Policy needs a governor that acts; reuses the wheel's refusal path; R3 turn boundary; 3/5 put policy in the bridge (R4, Appendix A boundary); §6 pipeline halts are normal; O8 + missing-data rule; wrap only viola-run sessions; O1 statusline renders through the wrapper
  User response: "Accept 2."
  Final answer: option 2
- Q: CLI Version Compatibility Posture
  Stakes: silent mis-answers after an auto-update, blocking on every release, fake-agent sync
  Research findings:
    - Options: 1) hard range gate, 2) warn and continue, 3) per-behaviour capability ledger + degrade to transport-only + `viola verify`, 4) post-condition checks only, 5) freeze the CLI
    - Recommended: 3
    - Reasoning: §4 design against the installed CLI; the S7 risk is per behaviour; slots into `agent::claude`; degrading never blocks (personal) and withholds rather than guesses (R4/R7); = first rung of the Appendix A rollout; verify produces fake-agent fixtures (§3.4); agentapi drift (prior-art §4); add post-condition checks in the probes
  User response: "Accept 3. Add three behaviours measured today to the ledger: harness-injected prompt prefixes (<agent-message, <task-notification>), the pasted_content id=... wrapper the CLI puts around a long paste (981 bytes wrapped, 749 not), and CLI-native modals that no hook reports."
  Final answer: option 3 + three measured behaviours

**Final summary confirmed:** yes — "yes, all good"

**Provenance (stated at the end of Quiz II, then corrected):** every answer in this run — from the Phase 0 confirmation onward, including the slug, all Quiz I and Quiz II answers, the research read-in-full condition and every amendment — was typed by the founder's overseer session driving this session through the viola prototype, under the founder's delegation: "overseer (founder-delegated)". The founder's one direct ruling is the Scale Intent answer (personal; D3 holds; public release later), relayed by the overseer.

# Phase 6 — Quiz II Dialogue

All answers in this phase: overseer (founder-delegated).

**Pre-filled confirmations:**
- Language Rust stable 1.95; clap 4.6.7; CLI conventions; MCP refusals as isError; hook contract; exec-form command hooks (http, mcp_tool excluded); tolerant external JSON; view-only GUI surface; GitHub Actions 3-OS CI vs fake agent; N/A hosting/mobile/LLM; deferrals to design/security/tests/obs/later — confirmed
- Changed: "Correct, with one naming fix: the product and binary are viola, so the instance variable is VIOLA_NAME (plus VIOLA_DIR and VIOLA_BIN, as the prototype used), not BRIDGE_NAME. BRIDGE_NAME was the brief's working name. The same goes for any bridge-prefixed identifier."

**Fork decisions:**
- Q: Concurrency Model and Server Stack
  Stakes: whether the hot `viola hook` path carries an async runtime; I/O styles in `channel`; usable edge libraries
  Research basis: research-targeted.md § Backend Framework Comparison — Tokio 1.53.1, axum 0.8.9, rmcp 3.4.1, interprocess 2.4.4, tiny_http 0.12.0 (stale), smol not viable; crates.io 2026-09-23 + Quiz I Growth/Wheel/GUI + hook contract
  Research findings:
    - Options: 1) all-async Tokio+axum+rmcp, 2) sync core tiny_http + hand-rolled MCP, 3) hybrid sync run/hook/send + Tokio only in mcp/ui/wait
    - Recommended: 3
    - Reasoning: hook must exit fast without VIOLA_NAME (R5), block for dialog answers (S3/S7), fit SessionEnd 1.5 s; PTY handles blocking, exit on process (§4.1 Windows details); wheel send window is a thread Mutex; Tokio only where rmcp/axum demand; no daemon (§3.4); option 1 taxes hook (O6); option 2 bets on stale tiny_http
    - Downstream forks: Q6, Q7, Q8, Q11, Q13, Q15 — KEYSTONE (warning shown)
  User response: "Accept 3."
  Final answer: hybrid
- Q: PTY Layer
  Stakes: whether ConPTY correctness is borrowed or owned; fixes; crates.io publishability; platform code to maintain
  Research basis: research-targeted.md § PTY layer — portable-pty 0.8.1, wezterm git main, portable-pty-psmux 0.9.7, windows-sys 0.61.2, nix 0.31.3; 0.9.0 excluded (#6783); crates.io 2026-09-23 + brief §3.4, §4.1, O1/O5, §6
  Research findings:
    - Options: 1) portable-pty 0.8.1 pinned, 2) wezterm git main, 3) portable-pty-psmux 0.9.7, 4) own ConPTY + nix
    - Recommended: 1
    - Reasoning: only option with evidence on this host (S1, O1, §6); own code is unsafe FFI across 3 OSes before need; git dep closes public door; psmux chases a non-reproduced artifact; fits Q2 sync pump; gaps owned behind a `pty` seam; swap triggers named
    - Downstream forks: Q4 only if passthrough — not keystone
  User response: "Accept 1. One more data point for it: this arch session itself runs under the prototype on portable-pty 0.8.1, pass-through in Windows Terminal, about an hour so far with no PTY defect."
  Final answer: portable-pty =0.8.1 behind a `pty` seam
- Q: Screen Model for Readiness and Modal Detection
  Stakes: hold a send before it lands in a hookless modal vs only report it afterwards
  Research basis: research-targeted.md § PTY layer screen-model bullet (vt100 0.16.2, alacritty_terminal 0.26.0) + research-broad + Quiz I Wheel (b), CLI Version ledger + Q2, Q3
  Research findings:
    - Options: 1) vt100 0.16.2 pre-send readiness gate, 2) alacritty_terminal 0.26.0, 3) no screen model
    - Recommended: 1
    - Reasoning: in a hookless modal the bytes answer it (measured (b)); option 3 would let viola answer a dialog by keys (breaks R4, R7); gate after quiet screen (§4.1 Timing), input-box signature present + no modal signature, else hold → not-delivered/input-not-ready; readiness only, never content (prior-art §4); signatures in agent::claude ledger; unverified → confirmation only
    - Downstream forks: none
  Audit note: cited ids overlapped the prompt's example ids; passed on content not in the prompt (R7 "keys only for user turns", R4 "never decides what to answer"); later prompts used neutral examples
  User response: "Accept 1."
  Final answer: vt100 0.16.2 gate
- Q: State Store
  Stakes: persistence of the audit log and small mutable state with many writer processes and no daemon
  Research basis: research-targeted.md § Database Comparison (atomic-write-file 0.3.1, tempfile 3.27.0, File::lock, serde_json 1.0.151, notify 8.2.0, rusqlite 0.40.2, redb excluded) + research-broad + Quiz I Scale/GUI/Wheel/Budget + Q2
  Research findings:
    - Options: 1) ndjson + atomic snapshots + File::lock, 2) SQLite WAL (rusqlite), 3) hybrid ndjson + rebuildable SQLite index
    - Recommended: 1
    - Reasoning: R4, §7 audit trail, §3.4 viewer reads disk, prior-art §4 healing; transactions buy little (wheel in wrapper memory, budget last-writer-wins, rare verbs); no runtime or C in Q2's sync core (D3); option 3 = later upgrade path
    - Downstream forks: Q6, Q10, Q11, Q12 — KEYSTONE (warning shown)
  User response: "Accept 1."
  Final answer: ndjson + snapshots + File::lock
- Q: Wrapper IPC Channel
  Stakes: local IPC between wrapper and short-lived clients
  Research basis: determined by Q2 (accepted text named interprocess 2.4.4 sync local sockets) + research-targeted.md § Message Queue / Event System — presented for confirmation without a separate research agent (Phase 6 guide: near-mechanical given prior answers)
  Findings: interprocess local_socket, one endpoint per run, hashed names; Tokio-native endpoints conflict with Q2; file mailbox fails the hook round trip
  User response: "Confirmed."
  Final answer: interprocess 2.4.4 local_socket behind a `channel` seam
- Q: Hook Transport
  Stakes: which hook events block the driven turn; decisions, ordering, latency, SessionEnd budget
  Research basis: research-targeted.md § Hook transport + hooks docs re-checked 2026-09-23 (parallel hooks; async cannot block; async output next turn; SessionEnd budget ≤ 60 s; UserPromptSubmit 30 s)
  Research findings:
    - Options: 1) all sync, 2) template B (async PostToolUse/Notification, catch-all sync PreToolUse), 3) tiered split with scoped matchers, 4) maximal async
    - Recommended: 3
    - Reasoning: async hooks cannot decide (S3/S7/S8 stay sync); ordering spine must stay sync (amendments (a),(b), R2, S5, S2); scoped PreToolUse matcher removes per-tool-call tax (O6); async events must print nothing (R1/R4/R7); SessionEnd own deadline (D4) with run fallback (§4.1); tier map is a ledger row
    - Downstream forks: Q10 only — not keystone
  User response: "Accept 3, with a correction to its premise. The plugin need not run in every session. The prototype loads it only into wrapped sessions: viola run passes --plugin-dir to the child, and this arch session runs exactly that way today. Per brief 3.2 both the driver and the driven run under viola run. Make that the default: unwrapped sessions carry no viola hooks at all. A user-level plugin install stays optional, for the MCP verbs in an unwrapped driver, and its hooks keep the fast exit when VIOLA_NAME is absent."
  Final answer: tiered split with scoped matchers; plugin only in wrapped sessions by default
- Q: MCP Server Implementation
  Stakes: the driver-facing MCP surface
  Research basis: determined by Q2 (accepted text named rmcp 3.4.1 inside `mcp`) + research-targeted.md § MCP server — presented for confirmation without a separate research agent
  Findings: rmcp 3.4.1 server + transport-io; churn risk → pin minor, thin adapter; rust-mcp-sdk and hand-rolled rejected
  User response: "Confirmed, with two tools added: answer (question, permission and plan answers, which is how a driver answers dialogs under R7) and list (sessions, state, wheel). Brief 3.2 named send, wait and last before the hook-answered dialogs existed. The prototype driver uses answer and list in every session. release stays CLI-only."
  Final answer: rmcp 3.4.1; tools send · wait · last · answer · list
- Q: Distribution and Install (v1) — audience-sensitive
  Stakes: how binary and plugin reach the founder's machines and find each other; version drift; PATH on Windows
  Research basis: research-targeted.md § Deployment & Infrastructure + plugin packaging facts (docs 2026-09-23) + viola-lab prototype run.rs (reference: VIOLA_BIN = current_exe(), viola's folder first on PATH, --plugin-dir) + Scale amendment + Q7 correction
  Research findings:
    - Options: 1) cargo install + hand-kept plugin, PATH lookup, 2) cargo install + plugin compiled into the binary bound to current_exe(), 3) dist 0.33.0 + cargo-auditable 0.7.6, 4) binary inside plugin bin/
    - Recommended: 2
    - Reasoning: personal / just me with Rust 1.95 on host (§4); viola builds viola, Windows locks a running exe, versions coexist; current_exe() binds hooks/MCP to the owning binary (R5, §3.4); carries out the Q7 correction; exec commands avoid Git Bash rewriting (§6)
    - Downstream forks: Q10, Q12 — KEYSTONE (warning shown)
  User response: "Accept 2. It is already measured today: I rebuilt the prototype three times under this running session (a running viola.exe cannot be overwritten on Windows, so the old one was renamed aside). The wrapper kept the old binary while every hook, found through PATH, ran the new one. This time that was wanted, since it hot-fixed the wheel classifier, but it proves the mixed-version case is real."
  Final answer: cargo install + plugin compiled in, bound to current_exe(); known trade-off: hook fixes apply to sessions started after a rebuild
- Q: Wrapper Channel Protocol and Framing
  Stakes: wire format on each wrapper socket; refusals; request/answer pairing; mixed versions; match with log and MCP framing
  Research basis: research-targeted.md § API Style → channel protocol + MCP conventions + § Framing (serde 1.0.229, serde_json 1.0.151, tokio-util 0.7.19) + Q2, Q5, Q7, Q8, Q9 + CLI Version posture
  Research findings:
    - Options: 1) JSON-RPC 2.0 envelope over ndjson, refusals as typed results, 2) custom serde tagged enum, 3) JSON-RPC with refusals as error codes, 4) length-prefixed framing
    - Recommended: 1
    - Reasoning: JSON-RPC has Q7's two frame kinds; refusals in result keep the four reasons first-class and MCP maps to isError (R4); -32601 + v give typed cross-version failure; no handshake (O6); ndjson matches Q5 and §3.4; S1/S3/S7/S8/§6 payloads byte-exact as escaped strings
    - Downstream forks: Q12, Q15 (+ Q13) — KEYSTONE (warning shown)
  User response: "Accept 1."
  Final answer: JSON-RPC 2.0 over ndjson
- Q: GUI Live Feed and Server-side Source
  Stakes: transport of the view-only feed and how the server learns about changes; liveness
  Research basis: determined by Quiz I GUI Control Scope ("GET routes plus an SSE event feed") + Q2 + Q5 + research-targeted.md § Push Notification / Real-time — presented for confirmation; liveness sub-choice filled by the orchestrator and flagged for checking
  User response: "Confirmed, with one change to the liveness sub-choice. A bare pid check can be fooled by pid reuse, so match the pid together with the process start time recorded in the snapshot. Keep the prototype's heartbeat as the primary signal: the wrapper touches a heartbeat file every second, and a stale beat means gone. The pid check and claude agents --json only enrich that."
  Final answer: SSE via axum tailing ndjson; heartbeat primary, pid + start time and agents --json enrich
- Q: Validation and Parsing Strategy (own formats)
  Stakes: strict vs versioned-tolerant own formats; newtypes; derive validation
  Research basis: research-targeted.md § Validation Library (serde 1.0.229, nutype 0.8.0, garde 0.23.0, validator 0.21.0, serde_with, schemars, jsonschema) + Quiz I GUI/Budget/Scale + Q5, Q9, Q10 + R4, R5, §7, prior-art §4
  Research findings:
    - Options: 1) strict own formats + nutype, 2) versioned-tolerant own formats + nutype + strict version gate, 3) option 2 + garde/validator
    - Recommended: 2
    - Reasoning: mixed versions measured (Q9); strict parsing breaks Q10's typed refusal; logs readable across upgrades by a pure-reader ui; R4, §7, prior-art §4 healing; strictness on own inputs via ViolaName/Percent
    - Downstream forks: Q15 only — not keystone
  User response: "Accept 2."
  Final answer: versioned-tolerant + nutype + version gate
- Q: Timestamp Type (low-stakes, asked directly without research per the Phase 6 guide)
  Options: 1) chrono 0.4.45 (already via rmcp), 2) jiff 0.2.37; convention RFC 3339 UTC ms either way
  Basis: prior artifacts Q8, Q10, Q12 + research-targeted.md § Validation → time fields
  User response: "Accept 1."
  Final answer: chrono 0.4.45
- Q: Module Boundary Enforcement
  Stakes: compiler vs reviewer catching violating imports; repo structure; CI shape
  Research basis: research-targeted.md § Module Boundary Enforcement (matklad workspace, clippy disallow lists, cargo_pup 0.1.8 nightly, modou 0.4.0, cargo-deny 0.20.2, cargo-modules 0.27.0) + Quiz I Dev Style/Driven-Agent/Growth + Q2, Q9, Q10
  Research findings:
    - Options: 1) Cargo workspace flat crates/ (root = bin + workspace), 2) three-crate split, 3) single crate + clippy disallow, 4) single crate + modou
    - Recommended: 1
    - Reasoning: agent-driven → boundary must be a compile error (D1/R4, prior-art §4); compiler enforces Q2 (only mcp/ui list tokio; channel tokio feature; cargo check step on 3 OSes, D3/§3.4); root must be a package for Q9's cargo install; version.workspace for Q9/Q10 version stamps
    - Downstream forks: Q15 only — not keystone
  User response: "Accept 1."
  Final answer: Cargo workspace
- Q: Error Handling Pattern (grounded in prior answers, presented without a separate research agent)
  Options: 1) thiserror 2.0.20 per crate + anyhow 1.0.104 at the bin edge, 2) snafu 0.9.2, 3) option 1 + miette 7.6.0; fixed part: RefusalReason in viola-core with Unknown catch-all, result.refusal vs error.code, CLI exit codes, MCP isError, hooks fail open
  Basis: research-targeted.md § API Style → error handling + Q8, Q10, Q12, Q13
  User response: "Accept 1."
  Final answer: thiserror per crate + anyhow at the edge

**Final summary confirmed:** yes — "All good. One provenance note for the Established Decisions and the dialogue log: since the Quiz I review, the answers here were typed by the founder's overseer session driving this one through the viola prototype, under the founder's delegation. Record the amendments as "overseer (founder-delegated)". The founder's own direct rulings are the Scale Intent amendment (D3 holds, public release later) and the choice of option 1 there."

# Phase 8 — User Review (overseer, founder-delegated)

- Round 1: review-feedback-1.md (six items) → 21 of 21 patches applied. Harness had rewritten `<` to `<\` in the Phase 7 hand-back for the two harness prefixes; restored before saving the draft.
- Round 2: review-feedback-2.md (plugin folder keyed by <version>-<hash> confirmed; unconfirmable = option (a), keyed on the ledger) → 6 of 6 patches applied.
- Round 3: "Looks good, proceed."

# Phase 10-12

- Phase 10: 15 iterations (hard cap), all substantive, 101 patches applied, 0 failed. Iteration 7 revised the Q7 statement that the user-level plugin keeps hooks (it now carries only .mcp.json, to avoid duplicate hook.dialog in wrapped sessions) — flagged to the overseer for confirmation.
- Phase 11: 8 issues (0 high), 9 patch blocks applied; one DUPLICATION (unconfirmable exception stated 5x) left unpatched by design.
- Phase 12: .andromeda/architecture.md (iteration log stripped), input.md, project.yaml written.
