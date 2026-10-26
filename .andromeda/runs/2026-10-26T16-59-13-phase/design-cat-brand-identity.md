# design · Brand Identity — category read

## Carried
- (.andromeda/design-system-detail/brand-identity.md:20) Every human-TTY refusal this chunk adds (the strict-modes check now made before the home is read on `send`, `wait`, `last`, `answer`, `pause`, `release` and at `run`'s start) must stay a static line at expression 0.2: no redraw, no spinner, colour only as a second cue and never on a refusal.
  > cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners.
- (.andromeda/design-system-detail/brand-identity.md:21) The added refusals under `--json`, a pipe, `NO_COLOR` or `TERM=dumb` must carry no colour, glyph or cursor control, and a check failing once `run`'s or a passed `revive`'s child has started (pinned-binary re-hash, plugin rewrite, any later home read) must print nothing to the terminal.
  > cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
- (.andromeda/design-system.md:21) A refused strict-modes, DACL or re-hash check must surface as `unable` plus a short fixed reason word, never filler or a guessed cause.
  > **Standard phraseology and "unable" plus a reason.** Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal. There is no filler and no guessing.
- (.andromeda/design-system.md:19) A `ledger/stamps.json` that fails the strict-modes check this chunk adds to `verify`'s stamps read must never print `verified`: an unread stamp is an unstamped build, shown as `unverified-cli`.
  > **Verified aircraft type.** A CLI version stamped by `viola verify` prints `verified`. An unstamped build flies transport only as `unverified-cli`.

## Bindings
- **0.0 passthrough row ↔ body Surface: cli colour decision order and Platform-Specific Notes**: the 0.0 row's "prints nothing" for `run` (.andromeda/design-system-detail/brand-identity.md:21) is the same rule as decision-order steps 2–3 (.andromeda/design-system.md:246) and `hook` writing nothing to stderr and always exiting 0 (.andromeda/design-system.md:355), so the hook client's new strict-modes check must fail open silently.
- **"unable" plus a reason ↔ body Exit-code phraseology**: a frame verb's refused check prints exit 21 `unable  instance-unreachable` then the cause's hint, or the `--json` document with stderr empty (.andromeda/design-system.md:338), carrying no path or pid (.andromeda/design-system.md:382).
- **Verified aircraft type ↔ security.md stamps read**: the `unverified-cli` word (.andromeda/design-system.md:19) is what a refused stamps read must render as on the board (.andromeda/design-system.md:282).
