# P5 review card 1 — 2026-10-26-explicit-session-links: "Explicit session links"

Plan: `viola-0.1.0/chunks/2026-10-26-explicit-session-links/plan.md` (12 steps, 11 acceptance criteria, 26 gate entries).
Run dir: `.andromeda/runs/2026-10-26T12-53-39-phase/`.

## The founder's word it rests on
«Сначала instance check (Рекомендую)» — his own answer to the P4 fork, live by dialog at 14:41Z, relayed verbatim
(`inputs#I3`): the wrapper serves `link` / `unlink`; `viola link` / `viola unlink` run `check_instance` on the home and
the driven instance before any log or snapshot read, then the liveness pre-check and the server check like `send`; a
refused check exits 21 with no log and no line; no dated gap.

## Goal
Serve the two methods over the existing `LinkSet` (`link` → `{since, already_linked}`, `unlink` → `{}`, an absent pair
appends nothing, `-32602` for a bad name, a foreign `driven`, `driver` = `driven`), and build the two verbs in the form
above, the three design lines and the `handoff:  link, unlink` help row. No ledger row, no verify child, no live
`claude`, no MCP tool.

## Top acceptance criteria
- `link` twice → one `link` line, the same `since`, `already_linked` false then true; `unlink` → one `unlink` line the
  wrapper wrote (its first product path), `{}`, the snapshot's `links` empty; restart restores the set the log ends on.
- Raw frames with `../x`, `BUILDER`, `a/b`, `""`, 33 chars, a number, a foreign `driven`, `driver` = `driven` →
  `-32602`, no line.
- Linux and Windows: an impostor on the endpoint gets no frame from either verb, exit 21 in both modes; the control
  reaches the recorded wrapper.
- Unix: a 0770 instance dir → both verbs exit 21 before any log (instance-state pair / `instance-unreachable` document),
  no `cli-builder.ndjson` line, no frame, the mode as found.
- The three lines byte for byte, plain ASCII; `viola --help` gains the `handoff:` row.

## Touchpoints
New: `src/cmd/link.rs`, `src/cmd/unlink.rs`, `tests/channel_links.rs`. Modify: `src/run/links.rs`, `src/cmd/run.rs`,
`src/cmd/mod.rs`, `src/human/verbs.rs`, `src/human.rs`, `tests/channel_endpoint.rs`, `tests/cli_output_plain.rs`,
`tests/security_negatives_server_identity.rs`, `tests/cli_links.rs`, `.config/nextest.toml`.

## Test commands (baselines read at P5, HEAD 8fc3d1df12f9)
`df -h /tmp` (25G free) · fmt · clippy · Windows-target clippy · G1 · lint-probes · orphans-check (11/11) · unit ·
nextest `cli_links` (5/5), `channel_links` (absent: exit 94, vacuous), `channel_endpoint` (5/5), `cli_output_plain`
(6/6), `security_negatives_server_identity` (19/19) · `cargo deny` · the preservation guard (its three clone controls
in `p5-controls.md`) · boot / status / G2 / G4 / cleanup · `run --perf` (max 2.5 ms) · `pre-push` (3056 passed) · the
operator's hygiene, push and CI entries.

## Leans taken (each from an artifact; say if one is wrong)
- `since` prints time-only `HH:MM:SSZ`, a non-instant escaped — per layout-templates §Output structure (the wireframe
  shows no date).
- `driver` = `driven` refused by the wrapper's `-32602` only (exit 20 at the CLI) — the operator's answer
  (`chunks/2026-10-10-session-links/inputs/I2-relay-2.md.txt`).
- E3's driver is not booted: its name is self-reported and the landed step-2 cases boot `builder` alone — the test-plan
  E3 wording is an expected amendment.
- CARRY 2 (re-parse a logged link name before it is shown): the lines print the verb's own parsed `ViolaName`
  arguments and the reply's `since` / `already_linked`, so no name read back from the log reaches a line in this chunk;
  the rule binds the first surface that does.
- The verbs' refusal on a failed instance check is exit 21 (as `list` / `plugin install`, test-plan §5 On-disk), not
  `revive`'s exit 1 — as the founder's option text stated.

## Concretization previews
None: this chunk claims no capability. Two caps it advances cannot be proven by any chunk under the masters as they
stand; after the word, step 4 writes one dated `notes` line on each (no claim, no status change):
- `v1-13` — current acceptance: "Every human output line of the budget release and of link is derivable from its
  machine result, and the machine-readable and MCP results carry the same facts." Architecture [MCP] keeps `release`,
  `link` and `unlink` CLI-only, so no MCP result exists for either verb. This chunk builds the `link` half's CLI side
  whole.
- `v1-36` — current acceptance ends "… the link set with its first-linked time is derived from the log and shown
  identically by the board" (the page clause left 0.1.0 on 2026-10-16). design-system and layout-templates keep
  `viola list` without a link column in v1. This chunk builds the explicit `link` / `unlink` clause.

## Sampler (forced sample, the operator's word, `inputs#I2` item 1)
- architecture: `sampler: E carried 67 · the cascade lost 3 (established-decisions#17.1, stack-and-technologies#1.1, standard-contracts#6.1)`
- security-plan: `sampler: E carried 57 · the cascade lost 5 (authentication-authorization#1.2, authentication-authorization#1.4, authentication-authorization#1.5, dependency-security#2.1, dependency-security#6.1)`
- design-system: `sampler: E carried 17 · the cascade lost 2 (color-palette#5.60, motion#1.1)`
- test-plan: `sampler: E carried 82 · the cascade lost 4 (ci-integration#1.9, quality-gates-coverage-targets#2.0, quality-gates-coverage-targets#3.1, test-scope-summary#6.1)`
- obs-plan: `sampler: E carried 77 · the cascade lost 4 (metric-coverage#1.1, metric-coverage#3.1, slo-invariants-telemetry-budgets#2.3, span-trace-coverage#4.1)`
- a11y-plan: `sampler: E carried 25 · the cascade lost 1 (cognitive-accessibility#5.1)`
The plan's `(sampler)` items are in its Provenance (eight carried).

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | `v1-13` and `v1-36` each need the founder's disposition of the clause no chunk can prove (the MCP result; the board showing links): a deferral with its reason, or the acceptance re-worded. Their CLI halves are built by this chunk. | a CARRY onto the existing working-route entry "Version done-check" (Epoch 8), which judges every cap `verified` or `deferred` |
