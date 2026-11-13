# Founder forks — chunk 2026-11-13-verify-readiness-on-the-input-box (phase P4, 2026-11-13)

The founder is AWAY on the operator's word (the phase directive, inputs#I1 item 4: «The operator is away: raise no
dialog for a fork a measurement settles; a fork that is the founder's takes the most reversible option, marked
PROVISIONAL»). No dialog was raised. Each fork below is answered by that delegate rule, PROVISIONAL until his own word,
which supersedes it as one batch.

## F1 — the supply-chain red: how the three rmcp advisories leave CI's `supply-chain` job

The red: RUSTSEC-2026-0457 (rmcp OAuth client, fixed 3.7.0), -0461 (Streamable HTTP server transport, fixed 3.6.0),
-0462 (Streamable HTTP reqwest / unix-socket clients, fixed 3.9.0), all on rmcp 3.4.1; rustix 1.1.5's unsound
RUSTSEC-2026-0486 is a lockfile bump to 1.1.6 under either option. The release graph's rmcp features are exactly
`server` + `transport-io` (guard `scripts/rmcp-features.sh`); `auth` and `transport-streamable-http-server` are banned
by `deny.toml`'s `[[bans.features]]`.

1. **A reasoned `deny.toml` ignore per rmcp advisory, rmcp held at its ratified `>=3.4.1, <3.5` (recommended).** The
   security plan's own Critical CVE response for an advisory confined to a disabled feature
   (security-plan · decision/cve-sla-disabled-feature, decision/no-ignored-advisories): three ignore entries, each
   reason naming its disabled feature. No product code moves; the ratified range stands; removing the entries later is
   one edit. It makes stale the rows that read the ignore list empty (security-plan · decision/cargo-deny-gate-clean,
   test-plan · decision/no-advisory-ignored, test-plan · decision/no-ignored-advisories), which the wrap amends.
2. **Bump rmcp to `>=3.9.0, <3.10` (one minor pinned, the ratified form moved).** Measured offline: a scratch copy of
   HEAD resolves rmcp 3.4.1 → 3.9.0 (adding `const-oid 0.10.2`) and `cargo check -p viola-mcp -p viola-e2e
   --all-targets` is clean. `ignore = []` stays, as the three earlier advisory reds were closed. It moves the
   ratified range (security-plan · decision/exact-dependency-pins' except), changes the MCP server's dependency under
   the `CappedLines` stdin bound and the recorded `tools/list` exchange, and every rmcp trap the history names must be
   re-read.

**Delegate answer (PROVISIONAL, the directive's rule):** option 1 — the more reversible: no product code and no
ratified range moves, and it is the response the security plan already prescribes for this class.

## F2 — what «the input box is up» reads

The readiness reads two compiled footer literals («for agents», «(shift+tab to cycle)»); the founder ruled the second
literal in himself (chunk 2026-10-19-newer-claude-cli-rows, Fork 2). The recorded probe streams (inputs#I3, inputs#I4)
show the input box itself in every settled frame: a row of `─` alone, a row opening `❯`, a row of `─` alone.

1. **Add the box's own signature beside the two footer literals, recorded as its literal alone (recommended).** The
   box is up when a footer literal OR the box's own rows stand, and no modal; a recorded screen keeps a box row as its
   literal only (`─` run, `❯`), never the row's text — no more row text leaves the process than today. One shared
   predicate in `viola-agent-claude`, so `viola run`'s gate (the same `SIGNATURES`) takes it too. No new ledger row:
   the `input-box-signature` row reads the shared predicate (a new relied row would read every stamp, 2.1.296's
   included, unverified).
2. **Replace the footer literals by the box's signature.** Every stamped fixture set (2.1.287, 2.1.302) keeps only
   footer rows, so their recorded screens would stop reading ready until re-recorded live — a live re-record of every
   set.
3. **Keep the footer literals and add the notices' own texts as literals.** A notice's text varies with the account
   and the release; each new notice would red verify again.

**Delegate answer (PROVISIONAL, the directive's rule, and the most conservative form a boundary fork takes):** option
1 — additive, keeps every recorded set and stamp reading as today, and narrows what a recorded box row carries.
