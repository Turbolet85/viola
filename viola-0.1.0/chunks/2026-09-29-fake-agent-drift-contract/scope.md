# Scope — 2026-09-29-fake-agent-drift-contract

**Working entry (working-route.md:63):** Fake-agent drift contract — fake agent's hook sequences and payloads equal
the recorded verify fixtures per CLI version, annotations forwarded, fixture schema check

**Epoch:** Epoch 2b — Windows slice I b: events and ledger. This is the epoch's LAST entry; Epoch 2b completes
with it (the founder's ruling of 2026-09-28 ~21:50: no split).

## What this chunk builds

- **The contract suite (test-plan §4, contract-test-against-sandbox trigger, S8):** a suite asserting the fake
  agent's behaviour matches the recorded `viola verify` fixtures. This guards against the fake agent drifting from
  the real CLI. Per CLI version present under `fixtures/claude/<cli-version>/`:
  - the fake agent's hook SEQUENCE (which events fire, in which order, for a scripted turn) equals the sequence
    the recorded fixture set implies;
  - the PAYLOAD bytes each hook command receives on stdin equal the recorded
    `<fixtures>/<cli-version>/<Event>.<variant>.json` (UserPromptSubmit: only the top-level `prompt` key set,
    per test-plan §7);
  - `annotations` are forwarded (S8).
- **The fixture schema check:** every recorded fixture file validates against the committed fixture schema.
  - `[premise-corrected: the schema is schemas/claude-fixture.v1.json, and tests/contract_fixture_hygiene.rs:46-59
    already walks every fixtures/claude/*/*.json against it, per version, with planted reds]` The check exists
    at HEAD. The scope's part is to keep it covering every recorded set; no second schema walk is built.
- `[premise-corrected: test-plan :866/:1100/:1337 place S8 annotations in the question answer path (PreToolUse
  updatedInput.answers + annotations), and arch :91 lands the S8 row with dialog answers; no PreToolUse fixture
  or answer verb exists at HEAD]` "Annotations forwarded" cannot be witnessed by this chunk. It is carried to
  the dialog-answers entry, where the question answer path and its fixtures land.
- One recorded CLI version exists today (`2.1.283`; `ls fixtures/claude`, and `claude --version` reads
  `2.1.283`). The suite iterates every version directory it finds, so a second recorded set is covered without
  an edit.
- **A live drift found at P3 (research §Mechanism re-derivations):** the fake agent's UserPromptSubmit payload
  is the fixture re-serialised without its trailing `\n` (`src/bin/viola-fake-agent.rs:185-194`). The recorder
  writes `to_string()` + `\n` (`src/cmd/verify.rs:319-320`). The contract witnesses this red, and the fix makes
  it green.

## Folded freight (the entry's four CARRY blocks — `route.py pins`: 4 blocks on :63)

1. **CARRY — hook `matcher` evaluation** (chunk 2026-09-24-fake-agent-and-test-data-fixtures): the fake agent runs
   every `type:"command"` hook registered for an event. Hook `matcher` evaluation was deferred to this entry,
   where recorded `tool_name`s exist.
   - This chunk makes the fake agent evaluate `matcher` the way Claude Code's hooks shape defines it. test-plan §7
     (:1365) names this chunk as the one it lands with.
   - `[premise-corrected: the four fixtures under fixtures/claude/2.1.283/ carry no tool_name, and verify's probe
     records only the four spine events (ledger.rs:63-68)]` No recorded `tool_name` exists at HEAD, so the
     CARRY's premise does not hold. Whether matcher evaluation is built now against synthetic tool-bearing
     fixtures, or re-carried to the dialog-answers entry, is a P4 fork.
2. **CARRY — the receipt `size` resize oracle** (chunk 2026-09-29-h2-conpty-resize-probe's wrap): test-plan §7 says
   the fake-agent receipt `size` is written "at start and again whenever the terminal size changed (the resize
   oracle)". The fake agent samples its size only at start and before a byte read after a change
   (`src/bin/viola-fake-agent.rs` `receipt_size`, read at HEAD `90aba7c`). So a resize with no later key writes
   no receipt. **Bring the receipt or the spec wording to one truth here.**
   - The coordinates were re-verified at HEAD `2d8bc53`: `receipt_size` is `:463-473`, called only at `:492` and
     `:494`. The mechanism claim also holds: `read` blocks, so a resize with no later byte writes no `size`
     line. The choice between code and spec wording is a P4 fork.
3. **CARRY — WSL `--install-deps` root boundary** (chunk 2026-09-27-browser-verdict-reachability; overseer live
   ratification, operator-only): before the WSL distro is next re-provisioned, `scripts/wsl-provision.sh
   --install-deps` must stop running user-writable code as root.
   - Root runs only `apt-get install` over the package list an unprivileged dry run produced (Playwright 1.63.0's
     `install-deps --dry-run` lists it and exits 1 while packages are missing — that chunk's
     `evidence/wsl-chromium-deps.md`). The list is checked against a committed allowlist.
   - The chunk that first re-provisions takes it. Until then it moves on with the first markerless entry.
   - This chunk re-provisions nothing: research's file lists hold neither `scripts/wsl-provision.sh` nor
     `.github/workflows/ci.yml`. So the CARRY moves on at this chunk's wrap, and the next markerless entry is
     Epoch 3's head.
4. **CARRY — the host's shared local integration reds** (chunk 2026-09-29-sideloaded-conpty; the operator's word at
   its wrap, the overseer agreeing): on this Windows dev host the local integration suite is red on the pre-chunk
   tree too. `agent-run run` and the pre-push windows-tests stage failed 45-51 of 199 on the `fb78ddc` control vs
   53-54 of 204 on that chunk. The failures: a 7 s start wait timing out, `viola hook` over its 1.0 s spine bound,
   and the stamped-home fixture's `viola never exited`. The cause is not established and not chased (overseer
   ruling: other sessions load this host).
   - This chunk judges its local run against a same-day control, and CI is the acceptance leg (that chunk's
     `evidence/entry-6-not-this-chunk.md`).
   - The chunk that establishes the cause or reads the local suite green retires it. Until then it moves on with
     the first markerless entry.

## Folded from the overseer's direction at take-up (not on the entry)

5. `[inferred]` **Harness origin for the cross-session-message tag** — the overseer's claim, kept verbatim: "the
   hook classifier must treat a prompt that starts with the escaped or plain cross-session-message tag as harness
   origin, never the human. Measured by overseer1 on andromeda-worker, where 3 wheel flips on 09-28/29 came from
   Claude SendMessage prompts. The prototype got the same fix today in hook.rs harness_injected."
   - At HEAD the classifier is `crates/viola-agent-claude/src/hook.rs` `prompt_origin` over
     `HARNESS_PREFIXES = ["<agent-message from=", "<task-notification>"]`. Its doc says "a prefix the user typed
     arrives escaped (`<\task-notification>`), so it never classifies". A test case pins `<\task-notification>x` →
     `human`.
   - `[premise-corrected: the prototype's D:/dev/projects/additional/viola-lab/prototype/src/hook.rs:84-93
     (outside this repo) matches <\cross-session-message and <cross-session-message; its comment says the
     message "arrives escaped as <\cross-session-message (measured 2026-09-28/29 on andromeda-worker, overseer1's
     F115)"; nothing in this repository records it]` Closed as follows:
     - The tag's literal is `cross-session-message`. The measured injected form is the ESCAPED one,
       `<\cross-session-message from="…" from-name="…">`. That remains a HYPOTHESIS here: it is relayed, with an
       out-of-repo source and no recorded fixture.
     - An escaped prefix collides with the present escaped-means-typed design. A human who types the plain tag
       at a prompt's start arrives as the same escaped bytes (brief M4; `hook.rs:149-150`). So filing the
       escaped form as `harness` also files such a typed prompt as `harness`. That is a security boundary
       question (P4 fork), never a presumption.
     - No payload witnesses it: `verify`'s `claude -p` probe records only the four spine events. The
       harness-prefix ledger row is owned by "First live test and self-drive" (:76).
   - **Witness (required, must not pass vacuously):** a test that fails on the pre-fix classifier. A prompt that
     starts with the cross-session-message tag classifies `harness` (and never flips the wheel, events.md). The
     same tag anywhere but the start stays `human`.

## CI verdict read at Setup (5a)

- `2d8bc53` (the last wrap's flip = HEAD): **verdict not yet available**. ci#36571737771 is in progress; 15/15
  checks were registered, and the oldest still-running check (`test (ubuntu-latest)`) was at 157 s. No wall-clock
  yet.
  - The overseer relays: "10/15 green so far with no red; I relay any red before P4". A red relayed before P4
    folds here per promotion.md's CI arm.

## Boundaries

- **In:**
  - the fake agent (`src/bin/viola-fake-agent.rs`) and its contract suite;
  - the fixture schema check;
  - the matcher evaluation;
  - the receipt-size truth;
  - the harness-origin classifier fix in `viola-agent-claude` (item 5);
  - the spec wording those touch (test-plan §7, via a wrap amendment where the spec moves).
- **Out:**
  - new live `viola verify` recordings against the real `claude` beyond what a witness needs (live refresh is
    local and operator-run);
  - the fake agent's `--vt100`, `statusline` and `agents --json` deferrals (CARRYs pinned to :66 / :81 / :87);
  - the wheel itself (the wheel flip consequence is asserted only as far as the `origin` field reaches);
  - the DA1 headless stall (:76).
