# Merge decisions — viola-0.1.0 route

_One line per suggestion: `{validator} {kind} · {decision} · {reason / adjustment}`. Founder route rules (user-confirmed
binding): chunk 1 = 3-OS CI + headless harness; infra before feature code; thin Windows slice next; heavy security
ACL work after the slice; SQOS spike before the channel client; SHA-256 choice before `bin/`._

## Security (10)
- security Reorder `Bounded inputs` → before PTY wrapper · adjusted · no standalone validator chunk (founder "no crate without a real consumer"); validators land in their consumer chunks instead — MAX_FRAME in Wrapper channel, control-character/paste rule in Confirmed send, parser-panic degrade added to Readiness gate; the Epoch 6 chunk stays as the all-boundary sweep
- security Reorder `Home and code-bearing file integrity` → after Instance state · rejected · founder rule: heavy security ACL work after the slice (surfaced at P4)
- security Reorder `Windows endpoint admission` + `Server verification` → after Wrapper channel · rejected · founder rule: heavy ACL work after the slice; the slice's client is SQOS-opened, and no unprotected-connect fallback exists (F-43) (surfaced at P4)
- security Reorder `Sanitised errors and never-log floor` → Epoch 1 · adjusted · merged with obs Insert: the NEVER-log floor lands in Foundation as "Log redaction and never-log floor"; the Epoch 6 chunk narrows to "Sanitised error surfaces" (CLI --json / MCP / channel error data, built in Epochs 2–5)
- security Reorder `Unix endpoint and home hardening` → before Linux live confirmation · rejected · intent F-53 EXPECT orders the live Linux run before the Unix-specific chunk (H:180-181)
- security Rewrite `Wrapper channel` Unix socket directory · adjusted · "(Unix: per-user socket directory)" — amendment 2's location lands with the channel; 0700 enforcement stays in Epoch 8
- security Rewrite `Diagnostics plane` owner-only detail files · applied
- security Rewrite `viola ui loopback server` cookie gating · adjusted · "cookie-gated /api and SSE"; "no CORS" dropped for the 25-word limit (per security-plan §API Security, cited at phase)
- security Rewrite `Strip-bay live page` text-only rendering · adjusted · "text-only fields"
- security Rewrite `Statusline pass-through` per-start settings.json rewrite · applied (trimmed)

## Design (9)
- design Insert `Strip and readback primitives` · applied · bootstrap-first primitives (DS web-spa components 1–2); the cocked state moves there from the page chunk
- design Reorder `CLI output discipline` → before Capability ledger · adjusted · split: new "CLI output tokens" chunk before the ledger (its first consumer, the `verify` step lines); F-40's three-OS invariants stay in Epoch 5 "CLI output discipline"
- design Rewrite `CLI output discipline` token table / streams / grouped help · adjusted · tokens + stream split in "CLI output tokens"; grouped --help in "CLI output discipline"
- design Rewrite `Confirmed send` readback mirror · adjusted · "RB readback mirror with per-reason hints"
- design Rewrite `The board` BAY header + SGR cues · applied (trimmed)
- design Rewrite `viola ui loopback server` launch line · adjusted · launch file kept; launch-line phraseology left to design cli pattern 5 at phase (word limit)
- design Rewrite `Design token bundle` token test + Linux-render font stacks · applied (light-DOM elements moved to primitives)
- design Rewrite `Strip-bay live page` expandable lines, line cap, follow rule · adjusted · "silent expandable tape"; the line cap and follow rule stay in layout-templates, cited at phase
- design Rewrite `401 access strip` → Access and error strips (401 / 503 / TAPE stopped) · applied (dedups a11y's strip request)

## Tests (8)
- tests Insert `Test runner and coverage tooling` before chunk 1 · adjusted · founder rule: chunk 1 = 3-OS CI + headless harness, so no chunk precedes it; nextest folded into chunk 1, coverage tooling into Quality gates, the Playwright pin into Web test toolchain
- tests Insert `Fake-agent drift contract` after Capability ledger · applied · recorded fixtures exist only after verify (TP §6-§7); F-03's claim moves here
- tests Reorder Unix chunks (Linux live, Unix hardening, parity) → before Web UI · DEFERRED TO USER · real dependency: Playwright runs ubuntu-only and boots harness `viola run` sessions (TP §3, §9); it conflicts with the intent's (f)-before-(g) order
- tests Rewrite `Fake-agent contract suite` → Fake agent and test-data fixtures · applied (trimmed)
- tests Rewrite chunk 1 toolchain · adjusted · "pinned stable toolchain with 1.96 floor" (F-19: floor + pinned stable); artifact upload left to TP §9 at phase
- tests Rewrite `Quality gates` lint + MSRV, perf moved out · applied · the perf job needs hooks to exist (TP §3 `gate` treats a missing perf artifact as a breach)
- tests Rewrite `Hooks to normalised events` perf-gated deadlines · adjusted · "perf-job-gated hook deadlines" — the perf job is born here, so v1-01 and v1-07 claims move to this chunk
- tests Rewrite `Strip-bay live page` Playwright path halves · adjusted · "Playwright path specs"

## Obs (8)
- obs Insert `Log redaction wire` in Epoch 1 · applied · merged with security's sanitised-errors reorder into "Log redaction and never-log floor"
- obs Rewrite `Diagnostics plane` service identity + owner checks · adjusted · service identity applied; owner/mode/symlink checks on diagnostics files stay with Home integrity (F-17), per the founder ACL-after-slice rule
- obs Rewrite `Supply-chain and workflow gates` telemetry + redaction-toggle bans · applied
- obs Rewrite `Observability gates` G1/G3 · applied (trimmed)
- obs Rewrite `Quality gates` perf inside per-OS E2E job · rejected · intent F-01 EXPECT "the perf gates run in their own job" (XF F1 resolved there)
- obs Rewrite `viola ui loopback server` path-only request lines · applied
- obs Rewrite `Resumable SSE feed` stream lifecycle + liveness transitions · applied
- obs Rewrite `Linux and macOS parity` obs gates + identical schema · applied

## A11y (8)
- a11y Insert `Web a11y CI gate` in Epoch 8 · adjusted · no new chunk: the lint step and SC coverage report fold into "Web test toolchain and a11y harness" (AY bootstrap order puts a11y-ci-gate-wire early), so the verdict chunk can prove "every criterion tagged"
- a11y Reorder `Contrast, forced colours, reduced motion` → before A11y verdict · applied
- a11y Rewrite `Web test toolchain and a11y harness` SR-pass template + scrubbed rows · adjusted · scrubbed rows applied; the SR-pass template moves to Version done-check (founder pass)
- a11y Rewrite `Strip-bay live page` announcer + 503/TAPE strips · adjusted · polite announcer in the page; strips in "Access and error strips" (dedup with design)
- a11y Rewrite `A11y verdict` conformance label + focus theft · adjusted · "no focus theft" applied; SC IDs / conformance label not inlined (NO_DOMAIN_CONTENT), cited from AY §3 at phase
- a11y Rewrite `Contrast` SC IDs · rejected · domain content inline; cite a11y-plan §6 instead
- a11y Rewrite `The wheel` focus/mouse/resize · applied
- a11y Rewrite `Version done-check` founder SR pass · adjusted · "founder screen-reader pass recorded" (supplemental, never gating)

## Tally
Applied 17 · adjusted 20 · rejected 5 · deferred 1 (43 suggestions).

## Coverage-map deltas
- v1-03 claim → Fake-agent drift contract (E2); advanced by Fake agent and test-data fixtures (E1).
- v1-01, v1-07 claims → Hooks to normalised events (E2), where the perf job is born; advanced by the Foundation gate chunks.
- v1-40 advanced by CLI output tokens (E2); claim stays CLI output discipline (E5).
- v1-47 advanced by Log redaction and never-log floor (E1) + Sanitised error surfaces (E6); claim stays viola ui loopback server (E7).
- v1-49 advanced by Strip and readback primitives (E7); v1-52 claim → Access and error strips (E7).
