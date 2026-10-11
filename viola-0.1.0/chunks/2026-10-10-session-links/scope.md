# Scope — 2026-10-10-session-links · Session links

**Working entry** (`viola-0.1.0/working-route.md:121`, the head of the markerless tail, in
`### Epoch 4 — Session state & governance`): Session links — driver-to-driven link on first cross-instance send or
answer, explicit link/unlink, log-derived link set with since, link result payload — plus two CARRY blocks (folded
below; completeness check `route.py pins` → the run dir's trail: two `:121` rows, 521 and 737 chars, no
abstention). The entry carries no `BLOCKED-ON`, no `PREREQ` and no `WATCH`: the chunk has no acceptance gate.

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- no mutation run, no witness run and no workflow dispatch is planned;
- a ledger row, a `viola verify` child, a re-stamp and any live `claude` start are the founder's, and no cap is
  given. The review card states what the entry needs of them, each option priced, as steps that wait for his
  word. The fake agent rehearses;
- the folded test is in scope, with a control that fails without it: `answer`, `wait`, `last` and `pause` are
  not gated by a closed budget gate;
- a widening goes on the card as itself, unsettled, and the plan holds only that item, never the whole operator
  pass;
- a cut is proposed at the card if the chunk does not fit one window;
- the CI read is the `ci.py conclusion` call.

**CI read at Setup** (`ci.py conclusion`, one sha: the last flip commit is HEAD): `f02d5486` green, 15 of 15
checks, wall 454 s, run `ci#38094715870`. No red and no `not green` to disposition.

**Premise closure** (P3, at HEAD `f02d54865815`): every `[inferred]` bullet below was read against the extracts
and the code. A closed bullet carries no tag; a corrected one carries `[premise-corrected: …]`. What no master
sentence settles is listed under "Unworded shapes" for the plan's fork round.

## What the chunk builds

1. **Two event kinds.** `link` and `unlink` join `viola_core::EventKind` (`ALL`, `from_name`, `data_keys`), each
   with the `data` keys `{driver, driven}`. The log reader `viola_state::events::read_from` then returns such a
   line where today it counts it under `unknown_kinds`.
2. **The replay's link half.** `viola_state::replay` derives `links` from the `link` / `unlink` lines of
   `events.ndjson`, each item `{driver, driven, since}`. `since` is the time of the first `link` line of the pair
   after any `unlink` of it (architecture §Established Decisions, [Links]).
3. **The implicit link.** A `send` or an `answer` that reaches a wrapper with a string `from` naming another
   instance appends one `link` line to that wrapper's own `events.ndjson`, only when the pair
   `from → this instance` is not already in its link set. A repeated send or answer adds no line. The snapshot's
   `links` is rewritten on each change of the set (architecture §Standard Contracts, Instance snapshot). `answer`
   already carries `from` on the wire (`src/cmd/answer.rs:120`, `src/run/dialog.rs:323`). A `from` equal to the
   wrapper's own name is no link: [Links] words the link as made "to another instance".
4. **Two channel methods.** `link` and `unlink`, params `{driver, driven}`, served by the driven instance's
   wrapper. Both names go through `ViolaName::try_new` on the wrapper as on the client (security-plan §Input
   Validation, row "Channel frames (IPC ndjson)"). An explicit `link` of an existing pair adds no line; `unlink`
   appends an `unlink` line. A `link` / `unlink` is not refused for a driver's `from` (security-plan
   §Authentication & Authorization, row "Driver-originated `release`"); architecture's list of methods that read
   `from` holds neither.
5. **Two CLI verbs.** `viola link <driver> <driven>` and `viola unlink <driver> <driven>`, in the human, plain
   and `--json` modes. The human lines are design-system §Surface: cli → Component Patterns, component 4
   "Wheel, link and gate verbs": the padded verb word, `<driver> -> <driven>` in ASCII, then for `link` the word
   `since` and a UTC clock time in the `HH:MM:SS` form with `Z`, or for an existing pair the words
   `already linked` with no time. Link output is plain at every colour depth.
6. **The link result payload.** The machine result of `link` carries the facts its human line prints: the
   first-linked time, and whether the pair already existed (requirement `v1-13`, the `link` half; intent F-13).
   Architecture's channel contract and [Links] both still read the result as the empty object, so the payload's
   keys are worded by no master: a fork for the plan and an amendment for the wrap. It is additive (no `v`
   bump).
7. **Tests.** The CLI and log half of test-plan §6 Scenario E3; Scenario E5's `link` step (the replay reads a
   link the wrapper wrote); the folded budget case below; the raw-channel `link` / `unlink` name-validation
   negative of test-plan §6 Security control negatives (found by the tests extract; the methods land here, so
   its case does); unit and crate-level cases for the kind table and the replay. E3's `/api/links` and
   Playwright signals are not built here (Boundaries).
8. **The set at a wrapper's start** (found at research). A wrapper that starts over an existing log, a revived
   one included, starts with the link set its log gives, as it already rebuilds the newest turn and the highest
   dialog id from the log before its endpoint serves (`src/cmd/run.rs:272`, `src/run/wait.rs:112`). The first
   snapshot of a start holds that set, never an empty one.

## Folded freight

- **CARRY, chunk 2026-10-10-self-healing-state** (the operator's direction at its wrap, that chunk's
  input `I4`). Items 1, 2 and E5's `link` step in item 7 are this block. Coordinates re-verified at take-up:
  `crates/viola-core/src/lib.rs` declares `ALL: [Self; 13]` and a 13-row `KIND_TABLE`, and its case
  `kind_table_a_text_that_is_no_kind_parses_to_nothing` lists `"link"` and `"unlink"` as texts that are no kind
  (that case changes with item 1); `viola_state::replay`'s recovered state holds `links: Vec<Value>`.
  The mechanism claim is re-derived at HEAD: `events.rs:201` reads a line's kind through
  `EventKind::from_name`, `events.rs:261` counts a line it does not know under `unknown_kinds` and steps over
  it, and `replay.rs:57` matches three kinds and leaves `links` at its default.
- **CARRY, chunk 2026-10-10-budget-governor** (the operator's answer at its wrap, that chunk's input `I11`,
  question 3; restated at this take-up, `inputs#I1`). Owed here: a root case in `tests/cli_budget.rs` that drives
  `answer`, `wait`, `last` and `pause` under a closed budget gate, the gate fed through `viola hook statusline`,
  no booted pair needed. It carries a control that fails without the property: in the same closed gate an
  automated `send` is refused `budget-paused`, so a gate that never closed cannot pass the case. Coordinates
  re-verified: `tests/cli_budget.rs` holds `e5_the_replay_reads_what_the_wrapper_wrote` and seven `path6_*` cases.
  One coordinate mis-aims: the block says the masters name this entry, and neither sentence does by title.
  Test-plan §6 Path 6 reads "its case is owed to the route", and a11y-plan §8 reads "no case drives one of them
  under a closed gate yet"; "Session links" is named three times in test-plan, all three for the replay and E5.
  Both sentences are the wrap's to amend when the case lands. The mechanism claim is re-derived at HEAD: the
  gate's product call sites are five (`start` at `src/cmd/run.rs:536`, `evaluate` at `src/cmd/run.rs:126` and
  `src/run/send.rs:347`, `lift` at `src/run/wheel.rs:214`, `paused` at `src/run/wheel.rs:222`; a read of each
  call under `src/`, test modules left out), none in `answer`, `wait`, `last` or `pause`, and no case drives
  one of the four under a closed gate.

## Held for the review card, unsettled

- **A widening, as itself.** `[premise-corrected: three parts, not one; sixth gap in its row and eighth dated
  gap overall; no ratification on disk]` Serving `link` / `unlink` and adding their two client verbs crosses the
  local trust boundary in three places the security plan names, and none of the seven dated gaps covers any:
  (a) the wrapper first serves two channel methods (they answer `-32601` today; serving `answer` and
  `hook.dialog` was recorded as a channel-boundary widening the founder ruled, F1); (b) `viola link` /
  `viola unlink` would write their frame after the liveness-only pre-check, with no server identity
  (security-plan §Authentication & Authorization, row "IPC client-side server verification (anti-impersonation)");
  (c) the same two verbs would read `snapshot.json`'s `endpoint`, `pid` and `started_at` without the strict-modes
  check (same section, row "`~/.viola/` access control"). `unlink` occurs 0 times in
  `security-plan-amendments.md` and in its archive (`grep -c -i unlink` over both), so no record ratifies it. It
  goes on the card as itself, each option priced, and the plan holds that one item behind the founder's own word
  and nothing else of the chunk or of the operator pass (`inputs#I1`, note 4).
- **The founder-only steps.** The chunk needs no ledger row, no `viola verify` child, no re-stamp and no live
  `claude` start: a link is recorded from viola's own `from` parameter, which the CLI and the wrapper already
  carry, and the chunk touches no file of `crates/viola-agent-claude`, `fixtures/claude` or `src/cmd/verify`.
  The card says so, with the one optional live reading priced (`inputs#I1`, note 2).

## Unworded shapes (for the plan's fork round or its leans)

- whether a `send` or an `answer` the wrapper refuses still records the link;
- the keys of the `link` result, and whether `unlink` carries any;
- an `unlink` of a pair that is not in the set;
- a `driven` that is not the serving wrapper's own name, and a `driver` equal to `driven`;
- which of the two names files the verb's `cli` role log and fills the name field of its exit-21 line;
- the `source` of a `link` / `unlink` line (architecture closes `source` to `hook` · `wrapper` · `cli`; the code's
  `Source` holds the first two).

**How they were settled.** The first three were asked as forks at P4 and answered by the operator
(`inputs#I2`): a refused call records no link; the `link` result is `{since, already_linked}` and `unlink`'s
stays the empty object; an `unlink` of an absent pair appends nothing. The last three the plan took as leans
(the two bad shapes are `-32602`, the verb's instance is the driven one, the `source` is `wrapper`), and the
operator confirmed them with the approval word at the P5 review (`inputs#I3`). The same word sends the widening
and its three options to the founder and has Part A land alone until his word is relayed.

## Boundaries

- Not built here: `viola list` (no link column in v1; "The board: viola list"), `/api/links` and the `ui` server
  ("viola ui loopback server"), the page's transfer markers (the Epoch 8 entries), any MCP tool (the MCP list
  carries no link control; "MCP server for drivers"), the reserved v1.x brake routes, and the grouped
  `viola --help` (its `handoff:` group is worded by design-system and built by no verb yet; "CLI output
  discipline" owns it).
- No mutation run, no witness run, no workflow dispatch (`inputs#I1`, note 1).
- Requirement `v1-36` ends "shown by the board and the page", two surfaces this chunk does not build, so the
  chunk advances it and cannot prove it whole: no claim.
- `[premise-corrected: the ledger entry's acceptance also names the MCP result]` Requirement `v1-13` names
  `release --budget` and `link`, and its acceptance reads "the machine-readable and MCP results carry the same
  facts". No MCP surface exists before "MCP server for drivers", so this chunk advances `v1-13` and cannot prove
  it whole: no claim. The `release --budget` half landed with chunk 2026-10-10-budget-governor.
