# P5 review — 2026-11-01-unix-endpoint-and-home-hardening

The plan's render: `.andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/plan.md`, 364 lines, to open in an editor. No plan
stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-01-unix-endpoint-and-home-hardening`.

## Goal
Close the Unix half of the local trust boundary. The socket directory is verified by `lstat` before a bind and before a connect, and
left as found when it fails. The peer's euid is read on both sides: the wrapper drops a foreign peer before a frame, every Unix client
refuses a foreign server before its first frame. `viola list`'s home takes one rule. Harness `cleanup` reads what a client meets after a
force-kill.

## What each refusal reads
| | `run`, `revive`'s start | CLI frame verbs | MCP frame tools | `hook` | the wrapper |
|---|---|---|---|---|---|
| socket directory refused | exit 1, pair or document, `process-exit` | exit 21, `socket-dir-refused`, hint last, `process-exit` | `instance-unreachable`, same word, no line | exit 0, no body, no frame, `hook-decision` | — |
| peer of another euid | — | exit 21, `server-verify-failed`, its hint | the same word, no line | exit 0, no frame, `hook-decision` | dropped before a frame, no line, nothing on the terminal |

macOS: euid and directory checked; the pid and start-time comparison stays absent, the named carve-out, and no test reads it as a comparison.

## Top acceptance criteria
- A socket directory at 0755 or a symlink to a 0700 directory: `run` exit 1 with no bind and no child, frame verbs exit 21 with no frame, `hook` exit 0 with no frame, the directory as found; a foreign owner refused with an injected uid.
- The wrapper's and every client's decision refuse a foreign euid and a peer they cannot read; a refused stream reaches no dispatch; a same-user peer is served.
- `tests/security_negatives_socket_dir.rs` on `macos-latest`: the owed 0755 exit-cause case; `client_server_euid_is_this_user`: the euid read on macOS.
- `viola list` over a home another user may read: the home pair, exit 21, with no instance and with one, on all three OSes.
- `cleanup` after a force-killed wrapper reads `endpoint_gone:true`; measured `false` and exit 1 at HEAD (`.andromeda/runs/2026-11-01T15-28-14-phase/p3-cleanup-measure.md`).
- The five `run --perf` rows stay under `SPINE_DEADLINE`, both checks in the timed path (baseline max 2.42–3.0 ms).

One account cannot make a foreign euid or a foreign owner: those two are held at the injected-value decisions and at `accept_loop`'s decision argument, with no seam (test-plan §1).

## Touchpoints
24 files to modify, 1 new (`tests/security_negatives_socket_dir.rs`). Product: `viola-core` (the word), `viola-state` (`strict`, `board`), `viola-channel` (`client`, `tokio_client`, `server`, `lib`), `viola-mcp` (`tools`), `src/cmd/` (`run`, `client`, `hook`), `src/human/hints.rs`, `schemas/diag-line.v1.json`, the harness's `cleanup.rs`. `connect` has 8 callers and none is edited. No dependency moves.

## Test commands
25 entries: the lints (the Windows-target clippy included), `run --unit`, the 23-binary integration filter, the new binary (baseline: the predicted red, no such binary yet), the `run --e2e` filter, `run --perf`, `cargo deny`, the tokio-free check, the type check, the guard, the smoke with G2 and G4, `pre-push`, then hygiene, push and the CI read. Baselines: every new entry fired on the untouched tree; the guard's controls in `.andromeda/runs/2026-11-01T15-28-14-phase/p5-controls.md`. Live legs: none.

## The founder's forks (`inputs#I2`)
- Q1, the socket word: new word `socket-dir-refused`. **The founder's own word**, live, before 16:27Z, through the overseer's dialog, after the widening was shown to him.
- Q2, the euid word: reuse `server-verify-failed`. **The founder's own word**, the same.
- Q3, the board's rule on every OS: delegate the overseer, founder-delegated, so PROVISIONAL. The one Windows reading it moves: a read-granted home holding no instance is now refused.

## Leans to confirm
- The hook's record is one `hook-decision` line, not the security plan's «a note in `diagnostics/`»: the form the server check already took.
- The wrapper's drop writes no line: no frame exists to key one on, and a new `event` needs its own entry.
- A failed create and a refused check read one code.
- `mcp`'s start, `plugin install` and `read_budget_stamped` keep the write rule on the home: the CARRY names the board.
- The security plan's «sync + Tokio server» reads as the one sync server the code holds.
- The hook's rows live in the new binary, not `tests/hook_fail_open.rs`, since they need a wrapper booted under a re-pointed root.
- The harness's rule reads a connect refused or a path absent; `cleanup` removes no leftover file.

## Capability: the concretization preview
`v1-44`, claimed by this chunk, method `integration` kept.
- **Now:** On each OS only the owning user (and SYSTEM on Windows) can connect to an instance endpoint; remote pipe clients and other users' peers are rejected; every client verifies it is talking to the recorded wrapper process before writing any frame, and on mismatch the CLI and MCP report unreachable while hooks fail open; a squatted name blocks start rather than being taken over; the missing macOS pid check is recorded as a known gap.
- **Proposed:** the same sentence, each clause naming its witness. Unix admission: the verified directory (`security_negatives_socket_dir`) and the wrapper's decision over an injected foreign euid (`peer_euid_admitted`'s table, `accept_loop`'s refusing unit), since a second account cannot be driven (test-plan §1). Windows: `security_negatives_channel` (remote rejection, DACL). Clients: `security_negatives_server_identity` (Linux and Windows), `hook_fail_open`, `handlers`; the euid at every Unix client, `client_server_euid_is_this_user` on Linux and macOS. Squat: `path1_run_start`, `a_held_endpoint_is_squatted_name`. macOS's gap: `server_verdict_rows`' no-comparison row and the named carve-out in the masters.

Not claimed: `v1-53` stays "Linux and macOS parity"'s. `v1-18` is verified already and takes a dated note at the wrap.

## Directive item 2: effort and draws
- The check line: `classify: 210 agents · 204 pass · failed EVERY_ENTITY 6 · yes 499 of 991 (draw 1 435 · draw 2 439 · draw 3 439 · one draw only 69) · closing tags 5`; the re-spawn's `classify: 6 agents · 6 pass · yes 36 of 106 (draw 1 7 · draw 2 16 · draw 3 13 · one draw only 36)`.
- The manifest: 70 agents at each of draws 1, 2 and 3, every one `model haiku`, `effort high`.
- `pick`'s trail: `draws {"asked": 3, "rendered": 3, "read": 3}` on each of the six masters. Nothing read `max` or fewer draws. `draws-dead:0`.

## Sampler
Forced, phase side (`sample: yes · phase · … · FORCED by the operator`); the wrap's run is forced too, per `inputs#I1` item 1.
- architecture — `sampler: E carried 109 · the cascade lost 7 (established-decisions#28.1, established-decisions#29.4, occupied-resources#22.1, occupied-resources#29.1, stack-and-technologies#1.1, stack-and-technologies#1.4, standard-contracts#17.1) · listed 1 (occupied-resources#35.1)`
- security-plan — `sampler: E carried 97 · the cascade lost 4 (data-protection#8.1, error-handling#1.18, input-validation#1.8, security-anti-patterns#4.2) · listed 3 (dependency-security#3.1, dependency-security#8.1, security-anti-patterns#3.1)`
- design-system — `sampler: E carried 31 · the cascade lost 2 (surface#12.1, surface#13.1) · listed 3 (brand-identity#2.1, color-palette#1.3, surface#16.5)`
- test-plan — `sampler: E carried 120 · the cascade lost 5 (ci-integration#1.11, ci-integration#1.12, ci-integration#1.2, ci-integration#1.9, test-data-fixtures#1.2) · listed 6 (integration-test-strategy#3.0, quality-gates-coverage-targets#1.1, quality-gates-coverage-targets#2.0, test-data-fixtures#2.3, test-scope-summary#7.1, test-strategy#2.1)`
- obs-plan — `sampler: E carried 113 · the cascade lost 5 (ci-integration#1.4, ci-integration#2.5, error-capture-reporting#1.2, metric-coverage#2.1, obs-scope-summary#26.1) · listed 8 (ci-integration#2.1, ci-integration#2.3, ci-integration#3.1, ci-integration#3.2, ci-integration#3.3, pii-scrubbing-compliance#1.10, pii-scrubbing-compliance#1.13, telemetry-strategy#1.1)`
- a11y-plan — `sampler: E carried 56 · the cascade lost 1 (aria-patterns-roles#1.1)`

The plan carries each lost item, marked `(sampler)` in its Implementation notes. Census: `entities:991 · classify-yes:501 · one-draw-yes:67 · false-yes:148 · lost-classify:4 · lost-strong:20`. Re-spawns: 6 classifiers on `EVERY_ENTITY`, 1 strong reader on `COVERED_DROP`, each passing its one re-spawn; `move`: moved 1, held 8. No death, no hard cascade failure, no agent did the orchestrator's work.

## Directive item 8: the store under the skill
- **Refusals:** one guard refusal, `gate.py run --dry-run` piped into `tail` to shorten my own re-run (my spelling, against SKILL.md Setup 1's run-bare rule), re-issued bare and passed. No tool refused a store operation.
- **Old paths:** no agent reached for one: a sweep of the workflows' transcripts over `verification-matrix.json`, `viola-0.1.0/requirements.md` and `viola-0.1.0/chunks/` read 0 of the 1 883 tool calls, each call's path, command or pattern (agents' prompt and file text not read by it). My own two probes missed, neither following a letter line: `rec.py get chk:chk-j3vkwokw` (`no live node`) and `select … from pins where title = …` (a Binder error); the `next` and `pins` views answered. The route's freight still spells `verification-matrix.json#v1-18`, carried text.
- **`rec.py check`:** `check: clean`, every family `store fresh`, with two `journal uncommitted` lines and `docs: carrier: 1235 file(s) standing, read by no tool until the stage's deletion`, each information.
- **`gate`, `planlint`, `inputs` on the store:** no row reads wrong. `planlint` reads the plan and research from the store; `inputs verify`'s `plan.md:N` rows hold on the render. `gate.py guard` prints `scope-record.md: absent — no companion is excused` over a chunk with no record yet, which reads true.
- **A re-put's facts:** the trail and the printed line hold them: scope rev 2 → 3 `1 new · 19 revised · 0 retired · 19 kept`; research rev 1 → 2 `0 new · 0 revised · 1 retired · 69 kept`; plan rev 1 → 2 `0 new · 13 revised · 0 retired · 105 kept`.
- **The render read at P5:** serves. The same file is both the render and the working copy, byte-equal to the store after the last `put`, and citations by line hold on it.

## Standing (directive items 3, 6, 9)
Every transcript and the run dir are kept. `df -h /tmp` read 23G free before the stamped-home measurement and is entry 1 of the block. Epoch 6's diagnosis and audit are untouched. HEAD stays three commits ahead of origin (`a25b7302`, `f8b0e884`, `6ba49f53`); they go out with this chunk's first push.

## Validation
Mechanical checks: all pass, re-run in full after the baseline write. `planlint`: 0 hits; check 12 (d)'s one listed claim (the exit set) holds. `inputs verify`: 2 entries, 0 unparsed, 0 uncited. Validation-1: aligned; P3's closure amended the scope once, for the Windows board. Size 189 lines of prose, the fence's 175 apart; within the band.

## Beyond the plan
| # | Proposal the word would approve | Owner |
|---|---|---|
| 1 | The new pair's wording (`unable: the socket directory for <name> cannot be trusted`; `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`) lands as a planning form, the founder's wording owed | the founder, the batch at Epoch 7's boundary |
| 2 | Q3, the board's rule on every OS, stands PROVISIONAL on the delegate's answer, the founder's own word owed | the founder, the same batch |
| 3 | A killed wrapper's leftover socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it, and no owner is proposed | none: the choice recorded, a harmless file in a 0700 directory |

Apply? (yes / review / cancel)
