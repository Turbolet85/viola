@row_set n=1 row=decision/screen-fixture-signature-rows-only cell=rule seen=1 fact=F4 detector=none
value: A screen fixture keeps only signature rows (a row holding a compiled footer or modal literal whole, a box rule row as its rule chars alone, a prompt row as the prompt char alone), the rest blanked; another box row, a kept row the scrub would rewrite or a username split across a row seam is refused.
basis: fct-w5at72sz
because: The recorded-screen form now keeps the input box's rule and prompt rows as their literals alone and the fixture walk refuses a box row holding anything more, which the row's rule must say.
@row_set n=2 row=decision/ci-cargo-deny-gate cell=rule seen=1 detector=D-security-deps
value: CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory not on its reasoned ignore list, any yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
basis: fct-w5at72sz
because: deny.toml's advisory ignore list went from empty to three reasoned entries, so the gate no longer fails on every advisory.
@row_retire n=1 row=decision/cargo-deny-gate-clean seen=1 heir=decision/ci-cargo-deny-gate detector=D-security-deps
why: The advisory ignore list is no longer empty, so both the row's name and rule are false; the standing CI cargo-deny gate row carries the gate.
basis: fct-w5at72sz
because: deny.toml now ignores three rmcp advisories, contradicting a gate held with an empty ignore list.
@row_retire n=2 row=decision/no-ignored-advisories seen=1 heir=decision/rmcp-advisory-ignores detector=D-security-deps
why: cargo-deny now ignores three advisories, so a row named and ruled as ignoring none is false.
basis: fct-w5at72sz
because: The three reasoned rmcp ignores were added and ratified, and the heir row states the advisory policy as it now stands.
@row_add n=1 row=decision/rmcp-advisory-ignores area=dependency-security fact=F1 detector=D-security-deps
name: Reasoned advisory ignores for disabled rmcp features
rule: cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off in the release graph, its reason naming it, standing until rmcp leaves its below-3.5 range.
why: Ratified, under the Critical CVE response for an advisory confined to a disabled feature.
basis: fct-w5at72sz
because: The chunk added three reasoned advisory ignores to deny.toml, ratified by the founder, a dependency-security decision future work must respect.
@row_add n=2 row=decision/rustix-advisory-floor area=dependency-security fact=F2 detector=D-security-deps
name: rustix version floor
rule: The root lockfile must resolve rustix >=1.1.6, so the advisory against 1.1.5 is cleared by the version and needs no ignore.
why: RUSTSEC-2026-0486
basis: fct-w7nqjwnx
because: rustix was bumped from 1.1.5 to 1.1.6 to clear RUSTSEC-2026-0486 without an ignore, a floor like the section's other advisory floors.
@row_add n=3 row=decision/input-box-readiness-from-literals area=input-validation fact=F3 detector=D-security-input
name: Input box readiness read from compiled literals
rule: The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a modal literal stands; one predicate decides for verify and for viola run's pre-send gate.
why: A boundary widening that was ratified.
basis: fct-xdv7p2qy
because: The readiness predicate gained the input box's own rows beside the footer literals for verify and viola run's gate, a ratified widening no row of the section states.
@row_none n=1 fact=F5 type=4
relied: A screen recording is refused as unclean-fixture where the input box's live prompt row holds a home path, the user word or an email, since a kept rule row's seam is that row's whole text; the standing seam refusal of the screen-fixture row covers it.
