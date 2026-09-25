# Adaptation record — 0-pending wrap · session 13 · 2026-09-25

Path: Setup step 6, 0 pending, tree dirty only with bookkeeping (`session-handoff.md`, `friction-log.ndjson`).
Source: the operator's ROUTE-ADAPTATION request in this session (founder rulings 2026-09-25). It names both
entries, their order and anchor, and the epoch split, so the recorded direction satisfies the trajectory gate
(route-resolve §Edits + gradient). No P1 report and no fan-out on this path. P3 curation skipped: the
conversation carried no corrections.

## Items and dispositions

| # | Item | Disposition |
|---|---|---|
| 1 | Insert "CI chunk base and union verdict" (A) immediately before "Instance state and start order" | APPLIED — `working-route.md:34`, 3 CARRY blocks: (a) CI chunk base from the last master-flip pickaxe, (b) the union rule, (c) secret-scan residue scope |
| 2 | Insert "Local Linux pre-push gate" (B) after A | APPLIED — `working-route.md:36`, 1 CARRY block (host facts, CI pins, Linux-filesystem clone, pre-push agent-run gate + its mutation witness) |
| 3 | End Epoch 2 after "Wrapper channel"; the remaining four entries become a new epoch named by content | APPLIED — `### Epoch 2b — Windows slice I b: events and ledger` at `working-route.md:42`, per the epoch-growth valve's `{label}b` form, so the later epochs keep their numbers |
| 4 | Re-pin the handoff's union-rule note (overseer direction 2) onto A | APPLIED — A's second CARRY; B does not carry it |
| 5 | Handoff "Unowned observation" (secret-scan read `target/agent-run/chunk.diff` residue) | OWNED — A's third CARRY, source `chunks/2026-09-25-pty-wrapper-on-windows/report.md:124` |

## Measured, not relayed (before pinning)

- WSL: `wsl -l -v` lists `docker-desktop` (default, `*`) and `Ubuntu`, both version 2; in `wsl -d Ubuntu`:
  kernel `6.6.87.2-microsoft-standard-WSL2`, `nproc` 32, `free -g` total 31, no `rustup`/`cargo` on PATH.
- `crates/viola-pty/src/lib.rs`: `pub fn enter() -> Option<Self>` at :394 and :418 on HEAD fcca1ce (the run
  reported :395 / :420).
- `.github/workflows/ci.yml:166` `AGENT_RUN_CHUNK_BASE: ${{ github.event.pull_request.base.sha || github.event.before }}`;
  `fetch-depth: 0` at :155; tool pins `cargo-nextest@0.9.146,cargo-mutants@27.1.0` at :162.
- The pickaxe `git log -1 --format=%H -G ' · complete · ' -- .andromeda/master-route.md` reads fcca1ce; it is the
  form in `andromeda-phase/SKILL.md:117`.
- `rust-toolchain.toml` channel `1.98.1`.

## Not done here (left for the operator)

- The Epoch 2 header still reads `Windows slice I: wrapper, events, ledger`, though events and ledger moved to
  Epoch 2b. It was not renamed: the friction ledger keys epochs byte-exact on header text, so a rename would
  split Epoch 2's records. A rename is the operator's call.

## Verification

- `route.py cursor`: next `working-route.md:34 · CI chunk base and union verdict`, 42 of 53 markerless,
  half-promote 0, pending 0.
- `route.py epoch`: 9 headers; Epoch 2 = 7 entries (3 frozen complete, 4 markerless); Epoch 2b = 4 markerless.
- `route.py pins`: 33 freight blocks (29 before + 4), the new ones at :34 ×3 and :36 ×1; no UNPARSED lines.
- `git diff --stat viola-0.1.0/working-route.md`: 6 insertions, 1 deletion (the `↓` replaced by the header);
  `git ls-files --eol` reads `i/lf w/lf`.
