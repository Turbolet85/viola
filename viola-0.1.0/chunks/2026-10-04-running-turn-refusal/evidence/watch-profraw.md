# WATCH — the coverage `.profraw` merge refusal (scope.md, folded freight)

Recorded by /implement P2 before any re-run, per the watch rule (a red of the subject is recorded with the test, the
run and its first diagnostic line; a later green never erases it).

## Pre-push runs of this chunk

| # | run | stage | verdict | subject |
|---|---|---|---|---|
| 1 | implement P2, gate entry 17 (run dir `2026-10-04T22-00-28-implement`), 2026-10-04 ~22:07Z, uncommitted tree on `eb37914` | `linux-tests` | `"ok":false` | **RED — recurrence** |
| 2 | implement P2, full block after the fix, same run dir | `linux-tests` | `"ok":true` | green — post-fix 1/3 (coverage 1472/0, playwright 1/0, gate no breaches) |
| 3 | implement P2, `--entry 17`, same run dir | `linux-tests` | `"ok":true` | green — post-fix 2/3 (coverage 1472/0; 0 corrupt-profile lines in the log) |
| 4 | implement P2, `--entry 17`, same run dir, consecutive with 3 | `linux-tests` | `"ok":true` | green — post-fix 3/3 (coverage 1472/0; 0 corrupt-profile lines in the log) |
| 5 | operator pass, entry 17 by hand, before the pre-CI commit | `linux-tests` | `"ok":true` | green — post-fix 4/4 (coverage 1472/0; 0 corrupt-profile lines) |

**Fix and closure rule.** Folded on the overseer's word (founder-delegated, fold-now; `scope-record.md`):
`src/bin/viola-fake-agent.rs`'s `Agent` holds a `hooks` lock for each hook's whole run. On `\x03` and at stdin EOF,
`close_hooks` takes it, so the agent waits for a hook still exiting, and marks it closed, so no step starts another
before the agent exits. Per the same word, the WATCH closes only on a tally of 3 consecutive green pre-push runs after
the fix. Runs 2–4 are that tally at /implement; the wrap reads it. The operator pass's pre-push runs add to it.

## Run 1 — the recurrence

- Suite `coverage`: 1472 passed, 2 failed: `llvm-cov-exit-1`, `llvm-cov-summary-missing` (archived `target/run-archive/345`).
  Every test passed; the refusal is the merge after them.
- First diagnostic line: `warning: target/llvm-cov-target/viola-673807-6045272833459289092_15.profraw: invalid
  instrumentation profile data (file header is corrupt)`, then `error: no profile can be merged`.
- The file is TRUNCATED, not garbled: 77 744 B, magic and header intact, where every sibling of the same binary
  signature (`6045272833459289092`, the `viola` binary — a sibling's functions are `viola::cmd::hook::*`) is
  81 128 B. Decoding the header (raw profile version 10, 783 records of 64 B, 1898 counters) reproduces 81 128 B
  exactly; the cut falls inside the trailing names section, so every counter survived.
- **Which process.** The surviving counters, matched against a complete sibling's function names by name hash,
  identify pid 673807 as a `viola hook` handling a `UserPromptSubmit`: `role_of`, `cmd::dispatch`,
  `cmd::hook::{hook, handle, decided, deliver}`, `claude::hook::normalise` with its `origin` / `tags` / `pastes`
  paths, `Client::notify` (`hook.event`), `HookEvent::kind`. It had delivered its event and was at process exit
  (the profile is dumped by the runtime's exit handler) when it was killed.
- **Which test** (JUnit timestamps of the archived run against the file's mtime, 22:06:56.405Z):
  `hook_events::hook_prompts_arrive_normalised_with_their_origin` ended ~0.12 s after it, and its five prompts
  exercise exactly the tag-escape and paste normalisation the counters show
  (`hook_events::hook_every_registered_event_lands_as_one_line_of_its_kind` also ends inside the window).
- **Mechanism** (read in source, not yet forced open):
  - The test fires its prompts as ungated `--script` steps. It waits only until the last `prompt-submitted` line is
    in `events.ndjson`, which the wrapper appends when the hook's `hook.event` notification arrives, before the
    hook process has exited. Then it calls `Wrapper::stop_keep` (Ctrl-C) at once.
  - The fake agent runs `--script` / `--inject-harness-turn` steps on a detached thread
    (`src/bin/viola-fake-agent.rs` `main`: `std::thread::spawn(move || runner.run_steps(&steps))`). On `\x03`,
    `read_stdin` returns and `main` exits without joining it.
  - The fake agent is the session leader of the wrapper's PTY. Its exit sends SIGHUP to the foreground process
    group, and the hook child it spawned (`run_hook`, no process group of its own) is in that group. A hook still
    dumping its profile is cut short: a truncated `.profraw`. One killed a little earlier leaves no profile at all,
    and that loss is silent.
- **Prior hypothesis falsified.** The watch's hypothesis named a harness `cleanup` Ctrl-C reaching a WRAPPER
  mid-write. The truncated profile is a hook's, and the kill is the fake agent's session-leader exit, not the
  wrapper's.
- Not this chunk's code: the test, the fake agent and the hook path are all untouched by its diff. The race
  predates it.
