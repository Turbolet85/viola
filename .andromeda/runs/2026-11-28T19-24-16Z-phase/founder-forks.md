# Founder forks — 2026-11-28-the-fleet-on-the-founder-s-host (phase P3; the operator away, no dialog raised)

## P3 block — written 2026-11-28T20:10:22Z (`date -u`)

**Window.** One phase window: take-up at `3a7ff587` (tx 5507), the row read (7 tasks, 159 pickers in 3 parts, 7
history agents; 2 re-spawns, each passed), research rev 1 (tx 5513), scope rev 4 (tx 5515). Read-only toward the
host throughout: no live start, no window touched, no compositor query of any kind (none run), no key to a real
terminal, no real `claude`, no live arm of any `scripts/r*.sh`, nothing read of `~/.claude*` or `~/.viola`. Inputs:
the directive (inputs#I1), the take-up order (inputs#I2).

### Measurements (each from the code, a test, the masters or a recorded fixture; research §Measurements)
- `viola open` (started): built — `foot` per instance running the pinned `viola run`, `--workspace`, the launch
  pair, parent and role, a 30 s live read-back (`OPEN_LIVE_BOUND`, `src/cmd/open.rs:30`).
- `viola up` (restored): built — selects a heartbeat older than this boot with no `session-end` after the newest
  `session-start`, or a named one (`pick`, `src/cmd/up.rs:119`); parents first (`parents_first`, `:140`); a pinned
  `viola revive` per instance in its own terminal (`up_argv`, `:457`); one closed word per line.
- `viola revive`: built — newest logged id (or `--id`), the recorded cwd and launch pair (`src/cmd/revive.rs:1-6`).
- Lineage registry: built, v2-09 verified; `up` reads the parent only to order re-opens.
- R4a: green once, 2026-11-24T17:05:59Z, real compositor over a planted fleet and the stand-in agent: it read order
  and placement (2/2); it read no real `claude`, no reboot, no session end. R4b: never fired; `plant | after` refuse
  as their first act (`scripts/r4-up.sh:13`), by your U6 arm 3.
- Against the three scripts' facts (inputs#I1): resume by own id **built**; resume by the folder's newest transcript
  **absent**; model and effort handed again **built**; a terminal per instance on a named workspace **built**; an
  opening message into a resumed session **absent**; a wait printing the last message **built** (`viola last`) and a
  pending dialog's text **partly** (by id in the human line, the text only under `wait --json`); a host line (free
  memory, disk) **absent**; a read-back that an instance came up **built**; an instance driven by another binary
  **not measured** (asked of the overseer below).
- «Driven from one console» beyond the two gated chunks: nothing left to build but F-K; your reading of the console
  over the whole fleet in one home stays the gated readings of v2-14 and v2-15.
- What a reboot leaves on disk, by the code's readers: each `instances/<name>/` (snapshot, events log, heartbeat,
  diagnostics) persists; the socket under `$XDG_RUNTIME_DIR/viola/` is gone (tmpfs); the `.lock` siblings persist and
  hold no lock (advisory); `up` selects on a heartbeat older than `/proc/stat`'s `btime`.

### Reached rows (all seven masters searched, `rec.py find --doc all --text T`)
Terms: `viola up` (69 hits in 53 rows), `lineage` (38 in 20), `prototype` (12 in 10), `refusals-never-block-human`
(1), `key line` (6), `jk move` (0), `q quit` (0), `up.sh` (0), `opening message` (0). Every playbook escalate row was
read first (4: rul-r74hshif, rul-p2zl2kgl, rul-o2tr55aj, rul-bznresq5).
- Reached by F-K: none — no master row holds the key line; `layout-templates decision/console-frame-floor`
  (dec-2yqgdjfn r3) read NOT reached: it names «the key line on the last row», not its words, and the new line
  fits 80 columns.
- Reached by F-R: `security-plan decision/refusals-never-block-human` (dec-qqb3py4s r1) — the named exception.
  Read not reached: `test-plan decision/console-actions-proven-live` (dec-7rxiy6xv), `design-system
  decision/console-actions-run-cli-verbs` (dec-qe2dfahr) — they name the console's verbs, not the refusal rule.
- Reached by U-M arm 2 or U-S arm 2/3 (only if chosen): `architecture contract/cli-verbs/up` (ctr-uv2g64ih),
  `layout-templates contract/cli-verbs/up` (ctr-ccsi2k25), `architecture decision/up-writes-no-state`
  (dec-4xolnmqs), `design-system contract/cli-verb-output/up` (ctr-z7n7sjhl) and the nine
  `contract/viola-up-outcome/*` rows, `security-plan decision/open-terminal-direct-spawn` (dec-uyp3bfs7), the
  security rule «upstream text is content, never a viola command» (CLAUDE.md invariant) and `obs-plan` rows on what
  crosses a log; a newest-transcript pick reaches `security-plan`'s «viola writes nothing under ~/.claude» and the
  transcript-path redaction rows. Read not reached by every arm 1: each `up` / `lineage` row above stands as built.
- Reached by U-B arm 1: `test-plan decision/window-reopen-proven-live` (dec-b7dyjzkw), `test-plan
  decision/live-rig-clean-start` (provisional), the cwd rows his U2b answered (`security-plan
  decision/cwd-only-in-snapshot`, `decision/spawn-cwd-snapshot-only`). Read not reached: the R8 strip rows (no
  credential is copied under any arm).

### His forks (each a product choice or a widening; options priced, the recommendation first and marked)

**U-R — what «restored after a reboot» restores, and by what act.**
1. **One `viola up` you type after a reboot, as built (Recommended).** No new code or crossing; `up.sh`'s reopen
   is `viola up`. Cost: nothing restores until you type it.
2. Per instance, `viola revive <name>` by hand. No new code; more typing; no parents-first order.
3. Automatic at login (a user unit or a compositor autostart line running `viola up`). A new crossing: viola writes
   into your session startup; a boundary widening, its own route entry and his ratification.

**U-M — does viola carry an opening message into a resumed session (`up.sh` sends one).**
1. **No: viola carries none; the driver sends its own `viola send` after `up`'s `live` line (Recommended).** No
   new code; mechanism, not policy; the overseer sessions you restart send their builders' openers themselves.
2. `viola up` takes a message (a `--file`) typed into each resumed session after it reads live. A new input class
   into a resumed session (upstream-bound text from a new verb), a widening; rows on `up`'s verb, outcome words,
   its writes-no-state rule and the send path change. About one implement window.

**U-S — what replaces each script, and what is deliberately NOT replaced.**
1. **As built (Recommended):** `vstart.sh` → `viola open` (start) and `viola revive` / `viola adopt <id>`
   (resume); `up.sh` → `viola up` (+ U-M); `pwait.sh` → `viola wait` then `viola last`, the dialog text read from
   `viola wait --json`. Not replaced: the resume by «the folder's newest transcript» (an id is typed, or viola's own
   log holds it) and the host line (free memory and disk — not a viola fact). No new code.
2. As 1, plus `viola wait`'s human line printing a pending dialog's question and option text. Upstream text on a
   human-mode stdout line, escaped; a product choice on the wait contract's rows. A small step.
3. As 1, plus `viola adopt --newest` picking the newest transcript of a folder. A new read of Claude's transcript
   folder under `~/.claude/projects/` — a widening of what viola reads there. A small step.

**U-B — what R4b reads, given your U6 arm 3 (no credential copy, no code that copies one).**
1. **R4b reads your real fleet after your own reboot (Recommended):** you reboot and type `viola up`; the script,
   fired once on your word, reads only the board (`list --json`, no claude on PATH) and each window record, and
   starts, copies and types nothing. A new live step on your host reading your real `~/.viola`; its rehearsal form
   cannot reach it. This is what the script would be revised to.
2. A planted real parent and child under a private config: needs a credential in that config — refused by U6 arm 3.
3. No R4b script: v2-12 and v2-28 close on your own manual confirmation of the reboot. No code; no witness line.
- U1's graceful-shutdown reading (does a clean shutdown write a session end) rides U-B arm 1 as a read of the
  events logs after your reboot; under arm 3 it stays unmeasured.

**Split.** The doable part fits one implement window under every arm 1 (F-K, a revised R4b in no-start and
rehearsal forms); arm 2 of U-M would add about one. No split proposed.

### For the overseer (technical, or a fact this directive does not state)
- Which home and binary the prototype-driven instances run on (a separate `--home`, or `~/.viola` under the older
  binary) — it decides whether `viola up` and the console see them and what moving them onto this binary means.
- Technical forks taken as leans unless the overseer says otherwise: R4b stays in `scripts/r4-up.sh` (its `after`
  arm), its rehearsal over the stand-ins as R4a's; test homes under `target/e2e-home`; the planted fleet R4a's shape.

### The three lists, as they now stand
- **A — buildable and provable NOW over the fakes:** F-K (`CONSOLE_KEYS` to the 79-column line, the two pinned
  literals, the 80-column run read whole); F-R (a wrap amendment, no code); R4b's script revised to the arm you
  choose, in its rehearsal form only (its live arm refusing until your word); any small step U-M arm 2 or U-S arm 2/3
  adds, only on your word.
- **B — built, proven only by your live reading (GATED, yours to time):** v2-12's R4b and U1's reading; v2-30's
  «runs on» the launch pair; v2-31's next-turn half (R6a, R6b); R7's two readings (focus after the restore, a failed
  restore live); v2-28 whole, your confirmation; your console readings (v2-14, v2-15) over the whole fleet.
- **C — cannot be built until you are at the host:** the reboot and every start or restore on your host; retiring
  `vstart.sh`, `up.sh`, `pwait.sh` (outside this repository, your act); moving your live fleet onto this binary with
  the overseers wrapped and their parents declared.

### Why the phase stops here
The doable part beyond F-K and F-R stands on U-B (and on U-M / U-S where their arm 2 or 3 is chosen) — product
choices and widenings, never built on a delegate's word. The phase stops at this block; P4 resumes on your answers,
relayed verbatim.

## His answers — appended 2026-11-28T20:21:49Z (`date -u`), cited inputs#I5
Each his own word, read by the overseer at 2026-11-28T20:15:31Z and 20:16:41Z and relayed (inputs#I5); the scripts'
owner's facts and THE BOUND, inputs#I3; the prototype store, inputs#I4.
- THE SCOPE (the fifth fork): «Сузить до флота Viola (Recommended)» — Viola's own sessions; retiring the scripts and
  moving the other projects' sessions a later entry or act.
- U-R: «Одна viola up (Recommended)» — arm 1. U-M: «Нет, шлёт драйвер (Recommended)» — arm 1. U-S: «Как построено
  (Recommended)» — arm 1. U-B: «Ваш реальный флот (Recommended)» — arm 1.

## P4 block — a fork his U-B answer leaves open (written 2026-11-28T20:21:49Z; no dialog raised)

**U-B2 — what R4b must read for v2-12 to verify on it.** v2-12's acceptance reads, of R4b: «a parent and its child
resumed on their logged ids in their recorded workspaces, parent first». U-B's arm reads «only the board and each
window record». The board carries no session id (architecture `contract/list-payload/items`: `{name, wrapped,
liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window?,
origin?}`); the window record carries the workspace, the placement and, on its events-log line, its time — so
placement and parents-first ARE read inside U-B's arm, the resumed id is NOT. And with one session under the product
today (inputs#I4) no parent edge stands to be read.
- Q1, the logged id:
  1. **R4b also reads each live instance's two newest `session-start` lines in its own events log — the same id, the
     newer one after this boot — and prints words only (Recommended).** One read beyond «only the board and each window
     record»; v2-12 then reads whole live. Built behind a stop rule (plan step 8) until your word.
  2. R4b as you answered it; the id clause of v2-12 rests on the fakes and your own confirmation at the reading — the
     acceptance reworded to say so, a narrowing that is yours.
- Q2, one session today:
  1. **Green needs at least one parent edge among the live instances (the acceptance as written, Recommended):** R4b
     reads red `order=unread` until a Viola parent and its child stand under the product before your reboot — the
     moment is yours.
  2. Green on one session: v2-12's «parent first» narrowed to «where the home holds a parent edge» — a weakening,
     yours alone.
- While unanswered: the plan builds R4b inside U-B's arm (board and window records), its green needing a parent edge
  (Q2 arm 1, the acceptance unweakened), and the id read held (Q1 arm 1 behind its stop rule).
