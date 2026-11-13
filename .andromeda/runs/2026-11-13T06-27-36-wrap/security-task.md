@task_prose part=session
The readiness check on the claude screen now reads the input box as up when a footer literal stands OR the box's own rows stand (a rule row of at least 40 box-drawing chars directly above a row opening the prompt char), and never while a modal literal is on screen; one predicate decides for verify's guards, the input-box-signature ledger row and viola run's pre-send readiness gate.
The two new box chars are compiled signature literals beside the two footer and two modal literals; the founder ratified this live as a boundary widening (F2).
A recorded screen fixture keeps a box rule row as its rule chars alone and a prompt row as the prompt char alone; the fixture walk refuses any box row holding more, and a recording refuses as unclean-fixture where the live prompt row holds a home path, the user word or an email.
cargo-deny's advisory ignore list goes from empty to three reasoned entries (RUSTSEC-2026-0457, -0461, -0462), each in an rmcp feature that is banned or off in the release graph, standing until rmcp leaves its below-3.5 range; the founder ratified them (F1). rustix moved 1.1.5 to 1.1.6 in the lockfile alone, clearing RUSTSEC-2026-0486; CI supply-chain is green again.
verify's typed probe now records how many guards passed on the box alone (box_only_guards) in the stamps file's measured block; the live proof ran on the newest installed claude CLI.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the screen model's compiled signature literals and the readiness verdict over the vt100 screen — the verify signature reading and viola run's readiness gate
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: verify's ledger — the input-box-signature check, the recorded screen fixture's signature rows, the fixture scrub and unclean-fixture refusal, and the stamps file verify alone writes
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of verify's ledger — signature rows kept in a recorded fixture, the fixture refusal on a box prompt row holding the home, the stamps' measured block
@task_file n=4 path=src/cmd/verify/typed/turns.rs
what: verify's typed-run guards reading the screen only against compiled signature literals
@task_file n=5 path=src/cmd/verify/typed/pty.rs
what: verify's PTY settle and box wait over the real claude CLI, ending on a compiled literal or a modal literal
@task_file n=6 path=src/cmd/verify/typed/runs.rs
what: verify's trusted runs against the real claude CLI, their ready guard
@task_file n=7 path=src/run/gate.rs
what: viola run's pre-send readiness gate on a verified CLI (one test case added, no code changed)
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene walk — a screen fixture keeps only signature rows, any other row refused
@task_file n=9 path=deny.toml
what: the cargo-deny gate's config — advisory ignores, feature bans, licence allow list, sources
@task_file n=10 path=Cargo.lock
what: the root dependency graph's lockfile audited by the cargo-deny gate
@task_file n=11 path=fuzz/fuzz_targets/vt100_feed.rs
what: the screen model feed fuzz target in the fuzz workspace on its pinned nightly
@task_item n=1 part=touches
decision/cargo-deny-gate-clean (cargo-deny gate with no advisory ignores): the gate now holds three reasoned advisory ignores; it is still green (advisories ok, bans ok, licenses ok, sources ok) but no longer ignore-free.
@task_item n=2 part=touches
decision/no-ignored-advisories (No ignored advisories): broken as written — three rmcp advisories are ignored, each with a reason naming its banned or disabled feature, founder-ratified (F1).
@task_item n=3 part=touches
decision/cve-sla-disabled-feature (Critical CVE response for a disabled feature): the rule this session applied — its response for a CVE in a disabled feature is now the reasoned ignore until rmcp leaves its range; check the row says so.
@task_item n=4 part=touches
The rmcp ban rows (decision/rmcp-banned-features, decision/rmcp-http-server-and-auth-banned, decision/no-rmcp-http-or-auth-features, decision/rmcp-feature-set, decision/rmcp-features-exact): indirect — each ignore's reason rests on the auth feature and the Streamable HTTP server and client transports staying banned or off; the feature set itself is unchanged (server plus transport-io, rmcp 3.4.1).
@task_item n=5 part=touches
decision/data-protection screen-fixture-signature-rows-only (Screen fixtures keep only signature rows): a signature row now includes a box rule row kept as its rule chars alone and a prompt row kept as the prompt char alone, never the text after it.
@task_item n=6 part=touches
decision/fixture-scrub-and-refusal, decision/fixture-recording-scrubbed and decision/fixture-hygiene: the scrub-or-refuse rule now reaches the box rows' seams — a recording refuses whole where the live prompt row beside a kept rule row holds a home path, the user word or an email.
@task_item n=7 part=touches
contract/verify-refusal-codes/unclean-fixture and contract/fixture-refusal-codes/home-path, username, email: the same refusal codes now fire on an identifying input-box prompt row; codes and line form unchanged.
@task_item n=8 part=touches
decision/verify-screen-read-against-literals, decision/verify-signature-only, decision/verify-reads-only-signatures: verify still reads only compiled literals, but the set grows by the box's rule and prompt chars and a box-only screen now reads ready.
@task_item n=9 part=touches
decision/compiled-classification-sources (Classification sources are compiled only): the two new box chars and the 40-char rule minimum are compiled constants; check the row's list of sources holds.
@task_item n=10 part=touches
decision/unverified-cli-gate and decision/version-gate-strict-stamps-read: indirect — viola run's pre-send readiness gate on a verified CLI now reads a screen ready on the box's rows alone (founder-ratified boundary widening, F2).
@task_item n=11 part=touches
decision/bridge-send-one-bracketed-paste and decision/send-text-is-one-bracketed-paste: indirect — the send is gated by the readiness verdict that now accepts the box alone; the paste itself is unchanged.
@task_item n=12 part=touches
The screen model feed rows (decision/vt100-feed-isolated, decision/vt100-feed-bounded-queue, decision/vt100-feed-under-panic-catch, decision/vt100-inside-catch-unwind): indirect — the readiness verdict over the feed's rows changed; the feed, its queue and its panic catch are unchanged.
@task_item n=13 part=touches
decision/stamps-written-only-by-verify: the stamps file's measured typed-probe block gains box_only_guards, written by verify alone; no version bump, run reads no field of it.
@task_item n=14 part=touches
decision/manual-dependency-updates and decision/exact-dependency-pins: rustix moved 1.1.5 to 1.1.6 in the lockfile alone by a precise update to clear RUSTSEC-2026-0486; no manifest pin changed, tempfile stays exactly pinned.
@task_item n=15 part=touches
The fuzz rows (decision/parser-fuzz-scope, decision/fuzz-nightly-pin, decision/fuzz-workspace-exempt-from-root-deny, decision/scheduled-fuzzing): the screen feed fuzz target changed with the signature type; it was compile-checked on the pinned nightly offline, as no gate entry builds the fuzz workspace.
@task_item n=16 part=touches
decision/ci-cargo-deny-gate and decision/scheduled-advisory-run: the supply-chain job's red is closed by this session; the job reads the new ignore list.
@task_item n=17 part=touches
decision/scratch-footer-reading-residual (Scratch footer reading is no standing permission): readiness is no longer read from footer literals alone; check the row still holds with the box arm.
@task_fact n=1 slug=dependency-security
fact: cargo-deny ignores exactly three advisories, RUSTSEC-2026-0457 (rmcp auth feature, banned), RUSTSEC-2026-0461 (Streamable HTTP server transport, banned) and RUSTSEC-2026-0462 (Streamable HTTP clients, off in the release graph), each with a reason naming its disabled feature, standing until rmcp leaves its below-3.5 range; ratified by the founder.
@task_fact n=2 slug=dependency-security
fact: rustix stands at 1.1.6 or later in the root graph; 1.1.5 (RUSTSEC-2026-0486) is in no graph and needs no ignore.
@task_fact n=3 slug=input-validation
fact: the input box reads up only on compiled literals — a footer literal, or a rule row of at least 40 rule chars directly above a row opening the prompt char — and never while a modal literal stands; one predicate decides for verify and for viola run's pre-send gate, a boundary widening ratified by the founder.
@task_fact n=4 slug=data-protection
fact: a recorded screen fixture keeps a box rule row as its rule chars alone and a prompt row as the prompt char alone; the fixture walk refuses a box row holding anything more.
@task_fact n=5 slug=data-protection
fact: a recording refuses as unclean-fixture where the input box's live prompt row holds a home path, the user word or an email, because a kept rule row's seam is that neighbouring row's whole text.
@task_item n=1 part=untouched
Authentication and authorization over the IPC channel — peer euid checks, socket directory and socket modes, server verification, strict-modes refusals: no change.
@task_item n=2 part=untouched
Secret management — the CLAUDE credential strip, hermetic child environment, artifact canary scan classes: no change.
@task_item n=3 part=untouched
MCP tools, channel frames, wrapper channel methods and their error and refusal contracts: no change.
@task_item n=4 part=untouched
Paste text and dialog free-text character sets, control-character refusal and typed text normalisation: no change.
@task_item n=5 part=untouched
decision/harness-prefix-literals (four raw-start literals): a different set of literals; its count of four stands.
@task_item n=6 part=untouched
Statusline, budget file, config.json, revive, link and unlink rows and their contracts: no change.
@task_item n=7 part=untouched
Release, attestation, workflow permissions, action SHA pins and zizmor rows: no workflow or release step changed.
@task_item n=8 part=untouched
The absences (hosting, network listener, outbound calls, secrets, accounts, database): no capability added or removed.
