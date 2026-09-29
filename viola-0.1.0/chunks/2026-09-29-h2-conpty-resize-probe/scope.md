# Scope — 2026-09-29-h2-conpty-resize-probe · H2 ConPTY resize probe

**Source:** `viola-0.1.0/working-route.md:57` (Epoch 2b — Windows slice I b: events and ledger), taken up 2026-09-29.
**Working entry (verbatim title + hint):** H2 ConPTY resize probe — the key lost within ~50 ms of a ConPTY resize
reproduced on Windows a set number of times, localised, then fixed or documented

## Take-up direction (overseer relay, 2026-09-29, at this chunk's `/andromeda-phase` invocation)
- "CI on 90aba7c is green." — agrees with Setup 5a below.
- "The evidence is in the :53 chunk h2-ci-red.md: the recorder stopped after start and byte 78, so the RESIZE itself
  never reached the child, not only the key after it. Localise that first." — the coordinate re-verified: working-route
  :53 is `2026-09-28-capability-ledger-and-viola-verify`, and its `evidence/h2-ci-red.md` is present and quotes
  `child report stopped at ["start pid=6880 raw=true size=100x30", "byte 78"]`. The mechanism claim is folded below as
  its own `[inferred]` bullet, and "localise first" orders the work (see §What it builds).
- "The WSL --install-deps CARRY binds only if this chunk re-provisions." — folded as a conditional (§Folded freight 3).
- "The playbook-rule proposal in the handoff is closed: playbook.md:44-46 already rules it." — re-verified:
  `.andromeda/playbook.md:44-46` is the `Verbatim upstream copy (other masters)` pattern, `verdict: routine`. It is not
  this chunk's work. The wrap does not carry the proposal forward.

## CI verdict read at Setup (Setup 5a)
- `90aba7c` (HEAD, the setup-project upgrade U02/U07) · **green** · checks 15/15 · wall 256 s · ci#36489570285
  completed/success.
- `6c14191` (the last wrap's flip) · **green** · checks 15/15 · wall 238 s · ci#36486472673 completed/success.
- No red or `not green` to disposition.

## What it builds
A probe, not a feature: it turns H2 from a two-sighting runner-only flake into a measured fact about viola-pty's
Windows seam. Then it either fixes H2 in the seam or documents it as a known platform limit. It goes in three steps,
in this order.

1. **Localise first.** Split the resize's arrival from the key's arrival, so a red says WHICH one was lost.
   - Today the child reports its size only when the second byte arrives (`crates/viola-pty/src/lib.rs:454-459`,
     `byte {:02x} size={}x{}`). So the red of ci#36436266196 cannot tell "resize never applied" from "key never
     delivered" from "both". [verified at HEAD: `pty_child_entry` has no report line of its own for a size change]
   - The recorder gains an independent resize observation in the child (a size-change line, observed without a key).
     The test side records its own sequence as well (resize returned · key written · key flushed). A kill-proof report
     then places a loss at one of: the resize never reached the console · the resize applied and the key was lost · the
     key reached the child before the size changed.
     [verified: the child can watch its size with the existing `host_size()` (`lib.rs:332`, `GetConsoleScreenBufferInfo`)
     with no new feature, since workspace `windows-sys` already enables `Win32_System_Console` (`Cargo.toml:201`). The
     child and the rig live inside `#[cfg(test)] mod tests` (`lib.rs:361`), so nothing reaches a release build. The
     fake agent's `size` receipt is key-gated too (`src/bin/viola-fake-agent.rs:463-473`), so no existing oracle
     observes a resize apart from a key (research.md). Poll vs input-record read stays P4's choice.]
   - The rig drains and discards ConPTY's output (`lib.rs:552-556`), so a terminal query ConPTY writes after a resize
     goes unanswered. microsoft/terminal PR #19535 (merged 2025-11-18, marked for inbox servicing) makes ConPTY
     request the cursor position (`ESC[6n`) after each resize and wait for the reply. [inferred — hypothesis H2-CPR,
     from the fetched PR pages, not measured on either build. Step 1 counts DSR requests on the drain.]
2. **Reproduce a set number of times** on Windows, with a count fixed in the plan before the run (the entry's "a set
   number of times"). A reproduction is an observed loss, localised by step 1's recorder. A green sample is not a
   reproduction.
   - The venue is the windows-2025 CI runner, not this host. Both sightings are runner-only: ci#36296402785
     job 108555954166 (under llvm-cov, 10 s wait) and ci#36436266196 job 108974874287 (7 s `CHILD_WITHIN`). The host
     read 60/60, 20/20 and 30/30 green (`chunks/2026-09-27-epoch-2-cleanup/evidence/pty-forced-window.md`;
     `chunks/2026-09-27-instance-state-and-start-order/evidence/ci-red-36296402785.md`). How to raise the hit rate
     (iterations, load, the forced-window `hold` child) and whether the probe runs in the standing CI or a dedicated
     probe job is P4's fork. [verified against the recorded runs: both sightings are under `run --coverage`
     (llvm-cov), the only Windows nextest run CI has. The rate is 2 of at most 32 ci.yml runs since `054ebe4`, about
     1 in 16, so sampling the standing CI cannot reach a set count within a chunk. Builds: runner Windows Server 2025
     10.0.26100, image `windows-2025-vs2026` 20260922.246.2; host 10.0.26200.9457, conhost 10.0.26100.8875
     (research.md).]
   - **P4 forks answered (operator + overseer, 2026-09-29; validation-1, intent-incomplete):** the venue is temporary
     measurement pushes that loop the recorder test inside the windows-2025 `test` job, removed in a follow-up commit,
     with no standing CI change. The count is 3 localised losses within at most 3 pushes of 200 iterations, stopping at
     the first push that reaches it. A budget that ends short is reported as not reproduced with its rate bound, never
     retried.
   - H1 stays falsified as recorded: a key that reaches ConPTY after a resize while the child is not reading still
     arrives, 20/20 on the host (`pty-forced-window.md`). This chunk does not re-open it unless step 1's recorder
     places a runner loss there.
3. **Fix or document.**
   - **Fix:** where the loss is in viola's own seam (`viola-pty` `PortablePty::resize` / the writer, or the `run` pump's
     resize-then-write ordering), fix it there, with a witness that cannot pass vacuously (it must observe both the
     resize and the key).
   - **Document:** where the loss is below viola (ConPTY / conhost, the rstudio/rstudio#18884 class), record it as a
     measured platform limit, with the reproduction count and the localisation.
     [verified: the homes exist. `.claude/docs/gotchas.md` already carries two ConPTY entries,
     `.claude/docs/services/viola-pty.md` carries the seam's ConPTY notes, and architecture's [PTY] decision records
     measured facts inline (an amendment only through the wrap). Which of them is P4's choice.]
   - Either way, the founder's product question on H2 stays OPEN beside the probe (entry CARRY 1): viola forwards both
     resizes and keys, and "the human always wins". This chunk hands the founder the measurement. It does not answer
     the product question.

## Boundaries
- In: `crates/viola-pty` (the seam, its real-PTY tests and the child recorder); the `run` pump's resize/write path only
  if step 1 or 3 places the loss there; the CI workflow only for a Windows probe job or loop, if P4 chooses one.
- Out: the founder's H2 product decision; Unix PTY behaviour (H2 is a ConPTY class; the recorder change stays
  cross-platform and must keep the three-OS tests green); portable-pty's version (`=0.8.1` is pinned — a bump is not
  this chunk's unless the fix is proven to need it, and then it goes to P4 as a fork).
- Invariant to keep: the human always wins — nothing added here may delay, drop or reorder a human keystroke to protect
  a resize.

## Folded freight (every annotation on working-route:57; `route.py pins`: 3 blocks, 0 abstentions)
1. **CARRY (328 chars) — founder ruling 2026-09-28 16:54**, relayed by the Viola overseer and placed by the operator at
   the `2026-09-28-capability-ledger-and-viola-verify` wrap as the second of two head-of-queue entries: "reproduce H2 on
   Windows N times, localise it, fix or document it; the founder's product question on H2 stays open beside this probe".
   → §What it builds, steps 1-3. Relayed; no artifact on disk behind the ruling beyond that wrap's route insert.
2. **CARRY (482 chars) — the red record:** `chunks/2026-09-28-capability-ledger-and-viola-verify/evidence/h2-ci-red.md`
   (verified present): ci#36436266196 job 108974874287 (`test (windows-2025)`, head `8cc9f14`),
   `viola-pty tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code`. The report stopped at
   `byte 78`, and `byte 79 size=120x40` never arrived within `CHILD_WITHIN` 7 s. Upstream class rstudio/rstudio#18884.
   - [premise-corrected: `lib.rs:454-459` writes the size only after the second `read_exact` returns, so a missing
     `byte 79` line says nothing about the resize] Overseer relay, 2026-09-29, verbatim: "the recorder stopped after
     start and byte 78, so the RESIZE itself never reached the child, not only the key after it." The recorded red
     stands. Closed against its witness, ci#36436266196 job 108974874287 still reads `child report stopped at ["start
     pid=6880 raw=true size=100x30", "byte 78"]`. The inference drawn from it does not follow: the report cannot tell
     "the resize never applied" from "it applied and the key was lost". Whether the resize reached the child stays an
     open HYPOTHESIS, and step 1 decides it on a runner loss. Localising it first stands as the overseer directed.
3. **CARRY (987 chars) — WSL `--install-deps` hardening** (chunk `2026-09-27-browser-verdict-reachability`, overseer
   live ratification, operator-only): before the WSL distro is next re-provisioned, `scripts/wsl-provision.sh
   --install-deps` must stop running user-writable code as root. Root would run only `apt-get install` over the list an
   unprivileged `install-deps --dry-run` produced, checked against a committed allowlist.
   **Binds only if this chunk re-provisions** (overseer, 2026-09-29). [verified at P3: research.md's file list names
   no `scripts/wsl-provision.sh`, and names `ci.yml` only on a CI-probe branch that adds a job, not the `test`-job
   tool line] This chunk does not re-provision:
   nothing in steps 1-3 touches `scripts/wsl-provision.sh` or ci.yml's `test`-job tool line
   (`scripts/wsl-provision.sh` last changed 2026-09-27, `5f0a809`). If the plan comes to touch either, the CARRY binds
   and becomes a plan task. Otherwise the wrap moves it forward with the next markerless entry, unchanged.
