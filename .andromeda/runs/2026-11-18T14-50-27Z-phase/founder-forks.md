# Founder forks — 2026-11-18-browser-tools-prompt-modal «Browser-tools prompt read as a modal»

Written 2026-11-18T15:26:09Z at the END of P3, before P4 (directive `inputs#I1` item 5). The operator is away, so no
dialog was raised. The overseer carries this block to the founder. The phase stops here: P4 waits on the word. It
plans every other step on the fake agent meanwhile, but no plan is written before the word.

## Block 1 — size and the live starts (one block, the founder's cap and keys)

### What P3 found that the questions rest on
- The literal: the dialog's own labels in the 2.1.325 bundle, `cancelLabel:"No, keep browser tools off"` (the
  selected row) and `confirmLabel:"Yes, use my browser"`, under the title `Claude in Chrome extension detected`. The
  rig's kept first screens B, C, D and F (local, not in the repository) show the same rows. One literal goes into
  the compiled modal set. Once there, it moves verify's settle and `run`'s pre-send gate together, because both use
  one predicate. It also takes a new closed first-screen word, proposed `browser-tools`.
- Which earlier step arms the prompt (carry `cry-idxm4ycw` (1)), READ STATICALLY: the print-mode probe cannot. The
  scan that writes `cachedChromeExtensionInstalled` runs only behind a check for an interactive start. Run A is the
  one earlier step that can write it. **Unmeasured, live only:** whether Run A's scan finishes before Run A is killed
  at its trust dialog (459–741 ms).
- The flag (carry (2)), READ STATICALLY IN PART: `tengu_chrome_auto_enable` is a GrowthBook value the account's
  server delivers. It defaults to off and is cached in the config under `cachedGrowthBookFeatures`. Whether the CLI
  reads the live value or the cache is not traced. **Measured indirectly:** S0R-3 showed it on for your account
  today. **Live only:** whether it is still on at the start.
- A home stamped at 23 rows, read by a binary built from this tree: the 24th row is absent, so the version reads
  **unverified** (the same shape as the 22 → 23 step). `run` there is transport-only: the gate reads no screen rows
  (a quiet screen is ready), dialog answers are refused `unverified-cli`, and no trust record is written. Nothing is
  re-pinned. The pinned product the overseer drives with is untouched.
- Product consequence on a host where the prompt stands unanswered: a 24-row `viola verify` settles at once on the
  modal (no longer `none` at 8.5 s) and pastes nothing, so rows B–F still fail. `run`'s gate refuses at once and no
  longer waits out its bound. The version stays unverified there until a person answers the prompt once in a real
  session. Viola answers nothing in it and writes nothing to turn it off.

### Q0 (the row's shape; the live arms follow it) — what does the 24th row read on a host where the prompt never stands?
The prompt stands only where an extension is detected and the server flag is on. On most hosts no run can raise it.
1. **(Recommended) One row, three stamped words: `pass` · `absent` · `fail`.** `pass`: some interactive run's first
   screen read `browser-tools` and nothing was typed. `absent`: no run met the prompt; the literal was not exercised
   there, and the row counts toward verified. `fail`: anything else (a browser-tools screen typed into, or one read
   as another word). Why: it is honest about what was measured (`absent` is never written as `pass`), and it does not
   leave every host without the extension unverified. The cost: the literal is measured only on hosts that raise
   the prompt, which is why the live arm below matters.
2. A strict relied row that needs the prompt to stand. Why not: every host without the extension, CI's fake agent
   aside, reads unverified, and `run` drops to transport-only there.
3. A ledger row that is not relied on: stamped as a measurement only, outside the verified check. Why not: the
   literal moves `run`'s gate whether or not the row is relied on, so a dependence would stand outside the check
   that is meant to hold every dependence (CLAUDE.md's ledger invariant).

### (a) Window fit
- **One implement window. No split recommended.** The work: one literal and one first-screen word; the 24th row and
  its check; the 23 → 24 counts in 9 files (research §Sweeps); a fake-agent browser-tools screen (so `cli_verify`
  joins the gate block); a gate case proving nothing was sent; the rig's two arms and a no-key stop; then the live
  starts. Research lists 19 files to modify and no new file.
- My own window at this point is deep. The take-up, the 156-agent row read and P3 all ran in it, and I estimate it
  is past half used. P4 and P5 should run in a fresh window that resumes this run dir.
- If a split is wanted, name it plainly: **(i)** «Browser-tools prompt read as a modal» keeps the literal, the row,
  the fake agent, the gate and the counts, proven on CI. **(ii)** a new entry, «Browser-tools prompt read live»,
  takes the rig arms and the live starts.

### (b) The live starts on the standing rig, on 2.1.325 by path (`~/.local/share/mise/installs/claude/2.1.325/claude`; `mise ls claude` reads 2.1.327 as `latest`, not used)
**Shape first:**
- **`BP`, the arm where the prompt MUST stand:** `viola verify` (24 rows, the rig's `V` shape) under the rig's
  private config with the four kept first-run markers and **without** `claudeInChromeDefaultEnabled` (the S0R-3
  seed). The config the rig makes holds no detection cache and no cached features.
  - What else must hold for the CLI to show the prompt on this host: the Chrome extension still installed and
    detectable; the server flag on for your account; an OAuth login whose scope the CLI accepts; no organisation
    policy against it; Run A's scan writing the cache before its kill (else the first prompt moves to a later run).
  - What the start reads if the prompt does not show: B–F `box`, the 24th row `absent`, so the predicate below reads
    RED with its cause named. No keys are added for that.
  - The rig adds one closed word, `cache-a` (`written` · `absent`): it records whether the private config holds
    `cachedChromeExtensionInstalled` after Run A, read by key name, never a value. That measures carry item (1) live.
- **`BK`, its control:** the same start with the S0B seed (`claudeInChromeDefaultEnabled: false`). The box is up on
  B–F and the 24th row reads `absent`.
- **A live 24-row `viola verify` of your real HOME is NOT part of this chunk.** It stays the carry `cry-bb5cy7aq` on
  «The 2.1.325 recorded fixture set». `BP` and `BK` are 24-row verifies under the rig's private home only.
- **`claude` processes per start, by the code's run list and the S0R-3 / S0B records:**
  - `BK`: **10** — the rig's own `--version` by path, verify's `--version`, the print probe, Runs A–F (6), and Run B's
    one `claude agents --json` read.
  - `BP`: **8** — the rig's `--version`, verify's `--version`, the print probe, and A, B, C, D, F. Run E does not
    start (no resume target), and the agents read is not made (Run B's turn never settles).
- **Every byte the rig types: NONE.** No key goes into the modal, the trust dialog or the box. Verify itself pastes
  only into a settled box, and none settles on `BP` B–F. `keys: none` is printed beside the stop.
- **The stop with a modal standing is the no-key group kill, never Ctrl-C.** Today the rig's stop presses Ctrl-C
  every 500 ms and kills the group after 10 s (`rig/mod.rs:41-43`). `BP` and `BK` take a stop that sends no byte and
  kills the process group at once. Verify ends its own runs by a kill with no key. The rig's stop fires only if
  verify overruns its bound.
- **One ledger, one count:** a fresh `evidence/prompt-starts.ndjson`. `BP` runs first under `--cap 1`, then `BK`
  under `--cap 2` (the running total), each with `--spare 1`.
- **Recommended cap: 2 base starts + 1 spare (`BP-2`).** Each start is ledgered with the clock's time before it is
  made. A start refused for the token window is uncounted.
- **What one spare is for:** only a cause `BP` itself showed. Example: B–F `box` with `cache-a: absent` would say Run
  A's scan did not finish before its kill. The mend then lands through a plan revision first, for instance seeding
  the CLI's own detection key `cachedChromeExtensionInstalled: true` (the value the CLI writes itself; nothing copied
  from your config). The spare is never spent on an inference. A third start is yours.

### (c) The predicate of each live entry, able to FAIL
- **`BP` GREEN** only when all of these hold:
  - at least one of `screen-b` · `screen-c` · `screen-d` · `screen-f` reads `browser-tools`, and none reads `none`;
  - the 24th row reads `pass`;
  - `keys: none`;
  - every teardown word stands: `census: 0` · `credentials: removed` · `scratch: absent` · `record-left: absent` ·
    `user-config` not `changed-by-rig` · `stop:` printed · `cache-a:` printed.

  RED otherwise. RED on `box` throughout (the prompt did not stand). RED on any teardown word missing.
- **`BK` GREEN** only when all of these hold:
  - `failing: none` · `rows: 24` · `pass-count: 24/24`;
  - `screen-b`…`screen-f` read `box`;
  - the 24th row reads `absent`;
  - `keys: none`;
  - every teardown word above stands.

  RED otherwise, a missing teardown word included.
- A RED live entry stops implement before the operator pass (directive item 7).

### (d) Your real config
Your real `~/.claude.json` is read only by the rig's mark witness (`user-config`). Nothing of it is copied but key
NAMES. The private config the rig seeds holds only the four markers and, on `BK`, the one browser-tools key at the
value the CLI itself writes on «keep them off».

### The word asked, in one answer
**Q0** (1 · 2 · 3) · **window** (one, or the split i/ii) · **live** (`BP` + `BK`, cap 2 + 1 spare as recommended, or
another cap) · whether the `cache-a` word and the no-key stop may be built into the rig.
