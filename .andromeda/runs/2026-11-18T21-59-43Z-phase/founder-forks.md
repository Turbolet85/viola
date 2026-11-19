# Founder forks — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin (P3 end, before P4)

Written 2026-11-18T22:40Z at the end of P3, per the overseer's directive 2 item 3 (inputs#I1). The phase stops here;
nothing below is started. Scope: `doc:2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/scope` rev 3;
research: `doc:2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/research` rev 1.

## (a) Size

- **Window:** I can't measure it from inside this session. The take-up, the row read (156 picker draws read by their checks) and the
  research reads ran here: an estimate puts it past half. P4 starts in a fresh window and resumes this run dir
  (Setup 3 (c), the `research` row present → P4).
- **One implement window?** Yes for this set, if the live verify runs through a rig arm (Q1 A):
  - the PREREQ gate entry
  - verify's browser-tools line and its fake-agent proof
  - a real-home verify arm and its rehearsal
  - a `/context` start
  - the two live starts

  That arm is the largest piece. Under Q1 B (a hand start) the build shrinks to the line, its proof and the gate
  entries.
- **The re-pin:** it belongs in this chunk's **operator pass**, not in implement and not in an entry of its own.
  The binary has to be built from the commit CI reads green, after the 24-row stamp, and placing a file under
  `~/.local/viola-product/` is no implement step. The pass builds, places and keeps the rollback copy, then stops
  before the switch (d).

## (b) Each live start's shape, before its count — all under your REAL home and config unless marked

### L1 — the 24-row `viola verify` of `~/.viola` on claude 2.1.325 by path

**Started by:** Q1. **Run from:** Q2. **Binary:** the implement tree's release build, `--home ~/.viola`.

**`claude` processes:** 9.
- 1 `--version`
- 1 print-mode probe
- 6 interactive runs, A to F
- Run B's one `claude agents --json` read
- plus the rig's own `--version` read by path when the rig starts it

**Bytes typed.** The rig grants no key. Every byte below is typed by verify itself. Each paste is
`ESC[200~ <text> ESC[201~ \r` (`crates/viola-pty/src/pump.rs:77-81`).
- **Run B:** seven pastes, in the order `src/cmd/verify/typed/turns.rs` holds them:
  - the probe prompt «viola verify probe: reply with the single word ok»
  - the long text
  - the tag-like text
  - the newline text
  - `/mcp disable viola-probe-absent`
  - `/compact`
  - `/clear`

  Then its STOP: `\x03`, a second `\x03` after 500 ms if the child still runs, then a kill.
- **Run C:** three pastes, the three dialog prompts of `crates/viola-agent-claude/src/ledger.rs:266-276`. The dialogs
  are answered by the probe hook's stdout, with no key typed. Ended by a kill, no key.
- **Run D:** one paste, `PLAN_PROMPT`. Ended by a kill, no key.
- **Runs A, E and F:** nothing typed. Each is ended by a kill.
- **The rig's own stop**, only at its deadline or a Ctrl-C: `\x03` into verify's terminal every 500 ms, then a group
  kill after 10 s.

**Written under `~/.viola`:**

| What is written | Afterwards |
|---|---|
| `ledger/stamps.json`: the 2.1.325 entry replaced whole, the other five versions kept | stays |
| `ledger/probes/<pid>/` (0700) | created and removed |
| `bin/<new key>/viola` (0700), the pinned copy of the build that ran verify | stays; a fourth key beside today's three |
| `diagnostics/`: verify's `cli` role log | to be confirmed at P4 from `src/obs.rs`, whether it writes without a `VIOLA_NAME` (the arm sets none) |

**Written under `~/.claude*`** (read before and after by NAMES only, matched by the run's own paths, never by the
shared file's hash, host-linux 2026-11-17):

| What is written | Afterwards |
|---|---|
| `~/.claude.json` `projects` entries the CLI makes for each run's cwd: A and F under `$TMPDIR/viola-verify-*`; B–E under `<cwd>/.viola-verify-<pid>*` | stay (two such keys already stand from earlier live verifies) |
| verify's own Run F trust record | written and removed by verify |
| whatever `/mcp disable viola-probe-absent` writes: the 2.1.325 toggle writer writes `disabledMcpServers` on the current project's entry, and whether an absent name reaches it is the unmeasured item | read on Run B's own project entry, key names before and after |
| transcripts under `~/.claude/projects/` for Runs B, C and D (E adds none) | stay |
| the print probe's project dir for its `ledger/probes/<pid>/` cwd (a `memory/`, no transcript) | stays |
| input-history lines for the typed prompts: `/mcp disable` adds one (security-plan); the rest are not counted | stay |
| every other path the CLI writes under `~/.claude/` | unmeasured; read as a before/after listing of names |

### L2 — `/context`'s post-condition (C6), one wrapped `viola run` start that sends `/context` once

- **`claude` processes:** 1, plus the rig's `--version` read.
- **Bytes typed:** the send's own paste `ESC[200~/context ESC[201~ \r` through the wrapper. Then the stop:
  `\x03` every 500 ms until the child exits, a group kill after 10 s.
- **Under the rig's private config (Q3 A):** nothing is written under the real `~/.claude*` or `~/.viola`.
- **Under the real home (Q3 B):** a `projects` entry for its cwd, one transcript, one history line,
  `instances/<name>/` under `~/.viola` (removed or kept at your word), and the start's trust-record step on your real
  config.

### The `/mcp disable` reading

It needs no start of its own. It rides L1, as the key-name reads above.

## (c) A predicate each can FAIL, and what a RED leaves

### P0 — the PREREQ gate, before L1

`jq -e 'has("claudeInChromeDefaultEnabled")' ~/.claude.json`. Exit 0 → go; exit 1 → RED, no start made.
- **Read at P3:** present. Its presence shows a decision recorded on this host. It does not show that a person
  answered this prompt, because `/config`'s «Claude in Chrome enabled by default» toggle and the extension's install
  upsell write the same key.
- **A RED leaves:** nothing.

### L1 — passes only on all three

- verify exits 0
- its last line is `stamped 2.1.325  24 pass  0 fail`
- the stamp's 2.1.325 `rows` holds all 24 relied ids, each `pass`, or `absent` on `browser-tools-prompt` alone

**A RED leaves** (`src/cmd/verify.rs:280` stamps before `:299` counts the fails):
- the 2.1.325 entry replaced by its failing rows. The product that drives this builder then reads 2.1.325
  UNVERIFIED: answers are refused (exit 12) and `/clear`, `/compact` and `/mcp` sends read unconfirmable, until a
  verify passes. Only `viola verify` may write `stamps.json`, so no hand restore exists; a spare start is the only
  repair.
- the new pinned key under `bin/`
- every residual in the table above

### L1's `/mcp disable` reading

It reads one of `written: disabledMcpServers` · `written: <other names>` · `none`. RED when Run B's own project entry
cannot be found by its path (`unread`).

### L2

It reads one closed word for the post-condition:
- a stdout tag in the transcript
- `prompt-submitted`
- none within the bound

RED when the send is not read back, or the start ends with no word. Under Q3 A a RED leaves nothing in your home.

## (d) The re-pin, the operator pass, stopping before the switch

**Built:** `cargo build --release --locked -p viola`, from the chunk's operator pre-CI commit, once CI reads that
sha green. The sha is named at the pass. `df` is read first: 203 G free under `/home` at Setup.

**Placed:**
1. Today's binary is renamed to `~/.local/viola-product/bin/viola.2026-11-15`, the NEW rollback copy (the
   `aa6bb5ab` build placed 2026-11-15, md5 beginning `50c9be2a`). Its md5 is read before and after.
2. The new binary is copied into the same directory under a temp name at mode 0700 and renamed to `viola`.
3. `viola.2026-10-19` is left untouched.

**Rollback:** rename `viola.2026-11-15` back to `viola`.

**Stamps the new binary reads:**
- **2.1.325:** verified only if L1 read 24 counting.
- **2.1.296, .302, .312, .319, .323:** unverified, each lacking `local-command-echo`, `trust-record` and
  `browser-tools-prompt`.
- **2.1.326, 2.1.327:** no entry → unverified.
- **Inference, not measured:** today's product keeps reading 2.1.325 verified after a passing L1, since the new entry
  holds every row it relied on.

**The session that drives this builder, at the switch:**
- **Keeps:**
  - its running wrapper and pinned copy `~/.viola/bin/<old key>/` until it is restarted
  - its instance name `viola-builder`, its events log and its snapshots
- **Loses at the restart** (`/exit` through the product, then your launcher):
  - its live `claude` child and the conversation in it, unless the launcher resumes it (`viola revive`)
- **Gains:**
  - hooks and MCP pointing at the new pinned path
  - the new start's trust-record step, writing the trust flag for an untrusted spawn folder in your real config. The
    builder's folder already reads `hasTrustDialogAccepted` by key name, so no write is expected there.
- **The switch itself:** yours and the overseer's.

## The four forks already put to you (one line each)

1. **The cap:** answered below by the recommendation (2 base + 1 spare).
2. **Where `/context` is read:** Q3 below.
3. **The reads of your real Claude config:** key NAMES only, before and after L1, matched by the run's own paths (b).
   Nothing here adds a read beyond that, save P0's one `has()`.
4. **Verify's words on the prompt:** Q4 below.

## NEW forks

**Q1. How is L1 started?**
- **A (Recommended):** a new rig arm, `VH`, verify under the real `~/.viola` and the real config.
  - **What it does:** no private `CLAUDE_CONFIG_DIR`, no credentials copy. It keeps the rig's ledger line, cap,
    version-by-path, token window, census and stop.
  - **Cost:** the largest build of the chunk. It is also a widening of the rig's private-config-only clause
    (security-plan; `decision/live-rig-seed-from-bundle-logic`), a wrap amendment under your word.
- **B:** a hand start by the operator, `<release build> --home ~/.viola verify` under `env -u CLAUDE_CONFIG_DIR`
  with the 2.1.325 dir first on `PATH`.
  - **Cost:** no build, but no ledger line, cap count or census from the rig. The ledger is written by hand before
    the start.

**Q2. L1's cwd?**
- **A (Recommended):** the repo root `~/dev/projects/viola`.
  - **Why:** its `projects` entry reads `hasTrustDialogAccepted` by key name, and `/.viola-verify-*/` is gitignored.
  - **Cost:** the run dirs are made inside the repository, removed on every exit but SIGKILL.
- **B:** a fresh folder.
  - **Cost:** untrusted, so Run B meets the trust dialog and L1 reads RED, unless its trust record is written into
    your real config first. That is a further write.

**Q3. Where is `/context` read?**
- **A (Recommended):** under the rig's private config, a wrapped start of the R2 shape.
  - **Why:** its post-condition is the CLI's behaviour, not your config's.
  - **Cost:** a small arm change; nothing touches your home.
- **B:** under the real home.
  - **Cost:** the residuals listed under L2.

**Q4. Verify's line when a run meets the prompt?**
- **A (Recommended):** one stderr line after the step lines,
  `note: a run met the CLI's browser-tools prompt; answer it once in a claude session, then verify again`.
  - **Effect:** stdout, the stamp and the exit are unchanged, and no step-line count moves.
- **B:** the same words on stdout before the summary.
  - **Cost:** it adds a stdout line that the `stamped …` last-line contracts and their tests read past.

**Q5. C5's two unmeasured items: when the extension cache is written, and whether the CLI reads the flag's live
value or its cache.**
- **A (Recommended):** left unmeasured. P0 keeps the prompt off L1, so this chunk has no reading of them. They are
  named as carried to the route with no owner entry.
- **B:** a VP-shaped start under a private config to read them.
  - **Cost:** one more base start.

## Recommended cap

**2 base starts:**
- L1 under the real home
- L2 under the private config (Q3 A)

**1 spare,** for L1 only: a second verify after a RED, the only repair of the replaced 2.1.325 entry (c). P0 spends
no start. Under Q3 B the count is the same, with L2's residuals added.
