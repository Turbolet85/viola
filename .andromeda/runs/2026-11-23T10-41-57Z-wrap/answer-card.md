# Answer card — wrap 2026-11-23T10-41-57Z-wrap (chunk 2026-11-23-model-and-effort-at-launch)

## Q1 — widenings inside the operator's own answers (rul-bznresq5)

The answers, each cited by its record (`inputs#I2`, relayed verbatim from overseer1's session, his dialog of
2026-11-23T08:36:42Z, as `inputs#I8` item 2 names them):
- **F2** — the two flags (`viola run --model` / `--effort`) and the two config keys (`default_model` / `default_effort`),
  the compiled default by its constant `DEFAULT_PAIR`.
- **F6's head** — the recorded pair crosses onto the revived child's argv, a second ratified crossing beside the logged
  session id.

The operations main reads as lying wholly inside them (12 lines):
- ops-architecture.md · decision/revive-resumes-in-place · established-decisions · fit: fits type 1 — «replays no launch» → «replays no launch spec but the recorded model and effort» (old: «…with the earlier bytes unchanged, replays no launch, and its first record is wheel…»; new: «…with the earlier bytes unchanged, replays no launch spec but the recorded model and effort, and its first record is wheel…»; the why kept). Exactly two words cross: model and effort.
- ops-architecture.md · contract/child-argv/launch-pair · occupied-resources · fit: fits type 3 — `--model <M> --effort <E>` after `--plugin-dir`, before the caller's arguments, every start and resume (F2)
- ops-architecture.md · contract/config-json/default-model · occupied-resources · fit: fits type 3 — the model-word shape, default the model half of `DEFAULT_PAIR` (F2; the one home of the default)
- ops-architecture.md · contract/config-json/default-effort · occupied-resources · fit: fits type 3 — one of the five effort words, default the effort half of `DEFAULT_PAIR` (F2)
- ops-architecture.md · contract/viola-cli-arguments/run-model · occupied-resources · fit: fits type 3 (F2)
- ops-architecture.md · contract/viola-cli-arguments/run-effort · occupied-resources · fit: fits type 3 (F2)
- ops-architecture.md · decision/recorded-pair-on-child-argv · occupied-resources · fit: fits type 1 — the recorded pair may cross onto the revived child's argv, re-parsed first (F6's head)
- ops-architecture.md · decision/default-launch-pair · occupied-resources · fit: fits type 1 — one named constant `DEFAULT_PAIR`, no environment variable sets the pair (F2)
- ops-main-architecture.md · decision/revive-in-place · established-decisions · fit: fits type 1 — keeps the parent, the role and the model and effort pair its real snapshot records (F6's head)
- ops-security-plan.md · decision/cli-input-trust-boundary · input-validation · fit: fits type 1 — the closed model-word and effort-word checks added (F2)
- ops-security-plan.md · decision/launch-flags-checked · input-validation · fit: fits type 1 — closed shapes, usage exit 2 before anything is read (F2)
- ops-security-plan.md · decision/launch-pair-argv-crossing · input-validation · fit: fits type 1 — four argv elements of a direct spawn, after `--plugin-dir`, before the caller's words; revive's recorded pair a second ratified crossing (F2, F6's head)

## Q2 — widenings on a delegate's word (rul-r74hshif) — the most conservative option, `inputs#I8` header

F5 (the pair into `list --json` / MCP `list`) and F6's rest (the `revive --model` / `--effort` override; a recorded half
that no longer parses takes the default half) are a delegate's word (`inputs#I8` item 2). They land PROVISIONAL — the
decision rows `state: provisional`, the contract rows marked by their sidecar Why — and stand in `founder-forks.md` for
the operator's own word (12 lines):
- ops-architecture.md · contract/list-payload/items · fit: fits type 3 (F5)
- ops-architecture.md · contract/list-item/model · fit: fits type 3 (F5)
- ops-architecture.md · contract/list-item/effort · fit: fits type 3 (F5)
- ops-architecture.md · decision/revive-pair-from-record · fit: fits type 1, state provisional (F6's rest)
- ops-architecture.md · contract/viola-cli-arguments/revive · fit: fits type 3 (F6's rest)
- ops-architecture.md · contract/cli-verbs/revive · fit: fits type 3 (F6's rest)
- ops-architecture.md · contract/viola-revive-exit-codes/2 · fit: fits type 3 (F6's rest)
- ops-security-plan.md · contract/revive-arguments/id · fit: fits type 3 (F6's rest)
- ops-security-plan.md · decision/board-launch-pair-re-parsed · fit: fits type 1, state provisional (F5)
- ops-security-plan.md · decision/launch-pair-kept-to-snapshot-and-event-log · fit: fits type 1, state provisional (F5)
- ops-test-plan.md · contract/viola-list-json/items · fit: fits type 3 (F5)
- ops-layout-templates.md · contract/viola-revive/usage · fit: fits type 3 (F6's rest)

MCP `list` beside W1: W1 (the 2026-11-23T05-33-20Z wrap) holds logged instance NAMES crossing into MCP `list`. The pair
is two closed words naming no instance, re-parsed at the board: not W1's class. It stands provisional beside W1 as its
own row of `founder-forks.md`; W1 is not widened and not answered here.

## Q3 — a cell that cannot hold a new member (`inputs#I8` item 4)

- ops-security-plan.md · decision/effort-word-check-in-agent-crate · fit: fits type 2, state provisional —
  security-plan decision/shared-validators-in-core's `except` (200 at the most) names two exceptions (the Hyprland
  Address validator, the transcript path shape check) and cannot hold the third (the effort-word check in the Claude
  adapter crate) under its bound. No name is dropped: the sibling row lands provisional and the fork stands in
  `founder-forks.md`.

## The answer

The overseer's answers, verbatim (the dialog of this wrap, snapped as relay-1.md):

- Q1: Confirm all 12 (Recommended) — overseer's confirm, 2026-11-23T11:17:56Z: I read the 12 lines of the answer card; each lies inside the operator's F2 answer (the two flags, the two keys, the compiled default) or his F6 head answer, whose chosen option itself named the closed shapes and the direct spawn; the exit 2 of a bad flag value is the standing usage rule. They land.
- Q2 / Q3: Proceed provisional (Recommended) — overseer's answer, 2026-11-23T11:17:56Z: proceed — the 12 delegate-word widenings and the cell-bound sibling row land PROVISIONAL and stand in founder-forks.md as W2 and W3 for the operator's word; W1 is neither widened nor answered. Quote on the last card what rows-land printed for the delegate-word adds, whole.
