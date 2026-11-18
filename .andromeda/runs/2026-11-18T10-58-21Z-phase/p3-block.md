# P3 block — 2026-11-18-session-folder-trusted-by-viola (directive item 5, inputs#I1)

No live start was made. Findings come from the code at `2cbaa752` (research §Files inspected) and from the two sealed
P3 blocks the directive names (not re-derived). The 2.1.325 install was read by path:
`~/.local/share/mise/installs/claude/2.1.325/claude --version` → `2.1.325 (Claude Code)`.

## What P3 found that changes the plan
- **The writer stands in part.** `src/trust_record.rs` already holds the CLI lock (taken only while free), the atomic
  replace that keeps the file's mode, the read-back and at most 3 attempts. It already refuses an absent file (never
  created), a link, a non-regular file, a foreign owner, an oversize file and a malformed document, and it reads
  `lost` after the third failed read-back. The product write reuses it.
- **Three parts of the bound are new code:**
  - the CLI's trust read: the exact key, then the parent walk (`holds_record` reads the exact key only);
  - no record for the home folder;
  - an entry that stands at the key without a true flag. The writer reads it `key-present` and writes nothing (F-T1
    below).
- **The fake agent** reads the record by the exact key only, so a case for «a parent already trusts» needs its read
  widened to the walk. That puts `cli_verify` in the gate block.
- **One write site.** `revive` calls the same `start` as `run` (`src/cmd/revive.rs:154`). The write goes after
  `version_gate` (`src/cmd/run.rs:289`) and before the bind. It is keyed on `launch.spawn_dir`, which is revive's
  recorded cwd, never `current_dir()`.
- **Directive item 4, verified** (`src/run/version_gate.rs`, its doc at line 3 and its test note at 286–287): a stamp
  of the 22 rows before `trust-record` reads unverified. On such a home `run` is transport-only and writes NO trust
  record; the dialog stays the human's until that home's next live verify. Nothing is re-pinned, and the product the
  overseer drives with is untouched.
- **The no-write rows are four, not two.** One is arch `decision/verify-own-trust-record` («run and revive write no
  trust record»). Three are security: `decision/no-writes-under-claude-home`, `decision/viola-writes-nothing-in-claude-dir`
  and `decision/nothing-written-under-claude-home`.
- **The value reader rows are nine.** Their `except:` names verify alone as the reader of `CLAUDE_CONFIG_DIR`'s value.
  Security: `credential-strip-by-name-only`, `env-not-config-channel`, `r8-claude-env-strip`, `no-stored-secrets`.
  Arch: `env-strip-names-only`, `env-strip-reads-names-only`, `no-environment-configuration`, `env-var-viola-prefix`,
  `contract/config-json/claude-env-keep`.
- All 13 rows above become Expected amendments, beside the security Decisions Log entry for the widening.

## (a) Size against one implement window
- **My window at this point:** not measured by a tool. I estimate it at about 60 % used: P2's 170 agents ran outside it,
  but the rules, two whole rows files, five source files and both sealed blocks came in. For reference, the last two
  phases ran to about 60–68 % and their implements to about 61 %.
- **What the entry holds:**
  1. The trust read (exact key, the walk, the home rule) and F-T1's form, in `viola-agent-claude::trust`.
  2. The product entry in `src/trust_record.rs`, sharing the checks and attempts.
  3. The call in `start`, with one closed word per start (a new obs field or event, plus `schemas/diag-line.v1.json` and
     an obs Decisions Log entry).
  4. `trust_config_file` shared.
  5. The fake agent's walk.
  6. About 12 e2e cases in `tests/run_cli.rs` and `tests/cli_revive.rs`: written; each refusal writes nothing; a parent
     trusts; unverified; the home folder; revive.
  7. Two rig arms with rehearsals.
  8. The live starts.
- **My judgment: it fits ONE implement window. No split.**
  - Items 1–6 are about the size of `2026-11-17-trust-record-ledger-row`'s writer half, with the writer already built.
  - Items 7–8 are one composite arm and one plain arm.
  - The risk is S1's shape below, because it carries a live verify.
- If a split is wanted after all, the seam is: Part A is items 1–6 plus the S2 arm, which claims nothing; Part B is S1
  and the claim. I do not recommend it. Its placement would be the founder's.

## (b) The live starts on the standing rig — by shape first
**Common to both arms:**
- **CLI:** 2.1.325 by path. Its install dir goes first on the rig's `PATH`, and `--version` is read by path before the
  first start. 2.1.326 / 2.1.327 (mise `latest`) are not used.
- **Config:** a private `CLAUDE_CONFIG_DIR` (`<scratch>/config/`), with the credentials copied 0600 and removed at
  teardown.
- **Seed:** `hasCompletedOnboarding`, the used-config markers, and **`claudeInChromeDefaultEnabled: false`**.
  - The browser-tools key is seeded for BOTH arms. In S1 the config's verify children start first, so the `viola run`
    child is a later start in the same config, the start the prompt was measured to arm.
- **Sentinel:** one OTHER project's entry, `projects[<scratch>/sentinel]` = `{"hasTrustDialogAccepted": true,
  "allowedTools": []}`. The folder is created, so its key is a real path.
- **The run folder `cwd/`:** fresh, NOT a git root, with no record. Its ancestors are the scratch root, `/tmp` and `/`;
  none holds a record.

**S1 — the proof: one rig start of a new composite arm.**
- **Why composite.** The run home needs a stamp with all 23 rows. Only `viola verify` writes `ledger/stamps.json`
  (arch `contract/ledger-stamps/writer`), and the rig removes its scratch, its home's stamps with it, at every start's
  end. So no stamp survives from S0B, and no other writer may make one. **S1 therefore carries one live verify of its
  own.**
- **Step 1, the verify.** In `<scratch>/vcwd/`, git-initialised and seeded trusted at its root's key, as the `V` arm
  is: `viola --home <vhome> verify -- <2.1.325>`. That is 1 print probe, 6 interactive children and Run B's one `claude
  agents --json` read. Run F writes and removes its own temp record.
  - **Step 1's gate:** the rig reads the stamp's words, rows only. Unless all 23 rows read `pass`, step 2 does not
    start and S1 reads `unstamped`. That is a reading lost to its shape, not a red of the product.
- **Step 2, the run.** The sentinel entry is serialised and kept in memory right before this step. Then, in `cwd/`:
  `viola --home <vhome> run rig-s1 -- <2.1.325> --model haiku`. The vhome keeps `CLAUDE_CONFIG_DIR`, so the child and
  viola both read `<scratch>/config/.claude.json`.
  - At the first settled frame the rig reads the screen word (`trust` · `imports` · `box` · `none`), the
    `projects[<cwd>]` entry's key names, the sentinel's bytes, and the run log's trust word.
- **Mechanism works:** `box`, the rig reads `ready`, and no key is typed.
- **Mechanism fails:** `trust`. The run log's word names why: `cli-unverified`, `already-trusted`, `config-*`,
  `lock-held` or `lost`, or `written` with the CLI not honouring it.

**S2 — the control: one rig start of a new plain arm.**
- Plain `claude` (2.1.325 by path, `--model haiku`) in a fresh `cwd/` of the same shape, under the same seed and
  sentinel. No viola and no verify.
- **Expected:** `trust`, the known positive that this folder shape raises the dialog when nothing trusts it.
- **Not `trust`:** the shape cannot tell viola's write apart, and S1 proves nothing.

**Count per CLI version (2.1.325 only):**
- S1 is 1 rig start: 8 `claude` processes (1 print probe, 6 interactive verify children, 1 wrapped child) plus 1
  `claude agents --json` read.
- S2 is 1 rig start: 1 `claude`.
- **The cap I propose:** S1 1, plus 1 spare spent only on a cause S1 itself showed (directive item 6); S2 1, no spare.
- None on 2.1.326 / 2.1.327.

**Every byte the rig types:**
- **S1:** none into any dialog or box.
  - verify pastes only its own seven ratified compiled literals into its Run B. Those are verify's bytes, not the
    rig's, typed only into a settled box.
  - **Stop:** with `box` read, the standing Ctrl-C into the rig's own pty (every 500 ms, group kill at 10 s). With
    `trust` read, the no-key group kill, never Ctrl-C while a dialog stands.
  - `x` + DEL into the ready box only on the founder's word. My lean: none.
- **S2:** none.
  - **Stop:** the no-key group kill (`killed-no-key`) whatever the screen, since a dialog is expected to stand.
- **The `keys:` grant to ask:** `none` for both.

## (c) The predicates — each reading can FAIL
- **S1 RED when any of these holds:**
  - the first settled screen reads `trust`;
  - the sentinel entry's serialised bytes after the run differ from those before it by a byte;
  - the written record is not exactly the one key: `projects[<cwd>]` read at the first settled frame holds any key
    besides `hasTrustDialogAccepted: true`, or that key is not `true`;
  - the run log's trust word is not `written`;
  - step 1's gate read `unstamped` (lost to its shape, so the spare may be spent on it).
- **S2 RED when** the first settled screen does NOT read `trust`.
- **Both RED when** a teardown word is missing or wrong: `census: 0`, `credentials: removed`, `scratch: absent`, and
  `user-config` not `changed-by-rig`.
- **One residual, said in these words:** whether the real CLI adds keys to `projects[<cwd>]` before its first settled
  frame is UNMEASURED. If it does, S1 reads RED on a key the CLI wrote, not viola. That is a false red, never a false
  green. The exact bytes of viola's own write are proven on the fake agent (the e2e case reads the file right after
  the start, before any child write).

## (d) What viola's write does in S1, and to which file
- viola writes only `<scratch>/config/.claude.json`, the rig's private config, reached through the vhome's kept
  `CLAUDE_CONFIG_DIR`.
- It writes one entry, `projects[<canonical cwd>] = {"hasTrustDialogAccepted": true}` (the key is the cwd: no git
  root). No other key changes and no entry is removed.
- It writes under the CLI's lock when that lock is free, with an atomic replace keeping the file's mode, a read-back,
  and at most 3 attempts.
- It writes nothing for the home folder, and nothing when the exact key or a parent already trusts.
- The founder's real `~/.claude.json` is never opened by viola in any start. It is read only by the rig's mark
  witness, after a moved stamp: it searches for rig-made needles (the scratch paths, the session id), keeps no bytes,
  and gives one closed word. The host's sessions rewrite that file about once a minute, so marks are witnessed, never
  its hash.
- S2 runs no viola and writes nothing.

## Forks
- **The founder's, through the overseer in one dialog:**
  - **the cap:** S1 1 + 1 spare, S2 1;
  - **the keys:** `none` for both (or `x` + DEL in S1's box);
  - **the split:** none recommended;
  - **S1 carries a live verify of its own** (7 children before the run), because no other stamp writer may exist. A
    named cost, his to accept.
- **F-T1 (technical, by relay): an entry already at the folder's key without `hasTrustDialogAccepted: true`.**
  - My lean (A): set that one field inside the existing entry and keep its other keys. That is the founder's «ONLY
    `projects[<key>].hasTrustDialogAccepted`» read literally, and `trust-writer-words/key-present` is narrowed to «a
    true flag already stands».
  - (B): leave it, write nothing, and the dialog stays the human's.
- **F-T2 (technical, by relay): the run log's closed word.** My lean: a new `ObsEvent` `trust-record{detail}` written
  once per start inside `run.start`. Its `detail` is one of the writer's words less `removed`, plus `already-trusted`,
  `home-folder` and `cli-unverified`. It takes an obs Decisions Log entry and the schema.
- **A residual already measured** (sealed block 2, (ii)): the CLI still shows the dialog for a folder trusted only by
  an ancestor that holds project or local settings (`gpt()`). There viola reads «a parent trusts», writes nothing, and
  the dialog stays the human's. It is named in the row, not mended.
