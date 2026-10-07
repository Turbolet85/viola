# The founder's card — what `send` does while the paste hint stands (R-L4)

One question, raised once through AskUserQuestion; it reaches the founder through the overseer, who returns his
words (inputs#I4). Nothing is built from the answer in this chunk: the gate, `GATE_MAX_WAIT`, `QUIET_PERIOD` and
`send_hint` stay as they are, and the chosen remedy's build is the route's at the wrap.

## The numbers (all on `claude` 2.1.287; `hint-window.md` has every reading)
- After a long paste (a text the CLI wraps; the prior chunk read the threshold statically as over 800 characters or
  over 2 lines) the footer shows `paste again to expand` in place of the input-box literal for **8.0 s from the
  paste**: nine timings, 8.000 s to 8.023 s. A second long paste restarts the timer; a short paste does not.
- The part of it after the turn's Stop is 8.0 s less the turn's length: **3.8 s to 7.0 s** in the nine timings
  (3.764, 4.176, 5.511, 5.746, 6.378, 6.466, 6.5, 6.619, 6.968).
- **Verified home, a `send` 1 ms after the Stop:** exit 13, `not-delivered` / `input-not-ready`, 0.63 s after it was
  issued, nothing typed. The refusal comes at the first quiet instant; the 5 s maximum plays no part.
- **The hint text's advice, run:** `viola wait <name>` with no cursor, issued in the window, woke on nothing and ran
  to its own 20 s deadline; with no `--timeout-ms` it has none. `wait --after <the cursor before the turn>` returns
  at once, inside the window, and a send right after it is refused again.
- **Unstamped home, the same `send`:** typed under the hint, submitted by the CLI as its own prompt and confirmed in
  0.64 s; a second one 2.2 s after the Stop the same. Short texts only.
- Unmeasured: a long text, or the same text a second time, pasted under the hint.

## The four options the route names, each priced from the numbers
The masters named under each option are the builder's expectation; the wrap's fan-out decides them.

### 1. Wait on a quiet literal-less screen
On a verified CLI a quiet screen with no input-box literal and no modal literal keeps the gate waiting, up to the
maximum wait, instead of refusing at once. `viola verify`'s own Run B already waits this way.
- **Code:** `Screen::verdict` (`crates/viola-agent-claude/src/screen.rs:140-151`, the last arm) and its cases; the
  gate's own cases (`src/run/gate.rs`); the two fake-agent cases of `tests/cli_send.rs`
  (`send_under_the_paste_hint_on_a_verified_cli`), whose `hint` case would turn from refused to delivered.
- **What it covers at the 5 s bound as it stands:** counted from the send, not the Stop. A send issued right at the
  Stop is covered when the window after the Stop is under 5 s: 2 of the 9 timings (3.8 s, 4.2 s). The other seven
  would still be refused, 5 s later than today.
- **Masters:** architecture §Established Decisions → [Screen Model]; test-plan §4 and §6; obs-plan §4 (the gate
  span's outcomes keep their two values).
- **`send`'s human lines and the `input-not-ready` hint:** unchanged in text; the refusal is printed later (up to
  5 s) and less often.
- **A human keystroke in the window:** neither delayed nor refused. The wait is on the automation side only; a key
  goes to the child as today and takes the wheel.
- **`send`'s longest blocking time:** unchanged, 15 s (5 s of gate, 10 s of confirmation). The masters hold no
  number for the MCP client's tool-call timeout: architecture names it only as the bound the MCP `wait` default
  must stay below.
- **The fake agent's 8 000 ms hold cap:** the hold counts from the Stop, and the longest measured window after a
  Stop is 6.968 s, so the cap covers it. A CI case that waits a hold out must stay under its kill line: the
  `hint` case took 7.4 s with a 3 000 ms hold, under the `mutants` profile's 10 s.

### 2. A longer bound
`GATE_MAX_WAIT` moves from 5 s to a value over the window.
- **Alone it changes nothing measured here:** the live refusal came 0.63 s after the send, at the first quiet
  instant, never at the bound. It has an effect only together with option 1.
- **With option 1, what it covers:** a bound of 8.5 s covers all nine timings, since the hint never outlasts 8.0 s
  from its paste and a send cannot be issued before the Stop.
- **Code, beyond option 1's:** the constant (`screen.rs:14`, pinned at `:184`); the `quiet-period` row reads it as
  the bound its measured settles must be within (`crates/viola-agent-claude/src/ledger.rs:760-761`), and verify's
  first settle uses it (`src/cmd/verify/typed.rs:487`). A moved bound loosens a stamped row's check, so the honest
  price is one by-path `viola verify` of the stamped version: 5 live starts.
- **Masters:** architecture [Screen Model] and [CLI Version Compatibility] (the `quiet-period` row); test-plan §4;
  security-plan §Input Validation stays as it is.
- **`send`'s human lines and the hint:** unchanged in text.
- **A human keystroke in the window:** neither delayed nor refused, as in option 1.
- **`send`'s longest blocking time:** the bound plus 10 s: 18.5 s at 8.5 s. A send against a screen that never goes
  quiet is refused after 8.5 s instead of 5 s.
- **The fake agent's cap:** an 8.5 s bound is over the 8 000 ms cap, so the fake agent can hold a window the gate
  waits out but cannot hold one that outlasts the bound; the "still refused" case would need the cap raised or a
  second mechanism.

### 3. Type under the hint
On a verified CLI the hint screen counts as ready: the text is pasted while `paste again to expand` stands.
- **What the numbers say:** on the unstamped home the real CLI took a short text under the hint as a new prompt,
  twice, confirmed in 0.64 s. A long text and a repeated text are unmeasured, and the footer's own words suggest a
  second paste can expand the first.
- **Code:** the hint's literal becomes a compiled signature (`screen.rs:30-33`) that `Screen::verdict` reads. That
  is a new relied-on CLI behaviour, so by the ledger rule it is an eighteenth row with a `viola verify` probe: a
  probe, a recorded variant, a fake-agent replay, the 75 count-literal lines in six files, and a 5-start record
  round (research M9).
- **Masters:** architecture [Screen Model], [CLI Version Compatibility] and §Cross-cutting Patterns → Capability
  ledger; security-plan §Input Validation (text typed with no input box on the screen); test-plan §3, §4, §7.
- **`send`'s human lines and the hint:** unchanged in text; `input-not-ready` no longer follows a long paste.
- **A human keystroke in the window:** neither delayed nor refused.
- **`send`'s longest blocking time:** unchanged, 15 s; no wait is added.
- **The fake agent's cap:** not in play; the fake agent would need to submit a paste under its held screen, which
  it does not do today (it reads its stdin again only after the hold).

### 4. Stay as it is
A `send` in the window is refused at once; the driver retries.
- **Code:** none.
- **What the numbers say:** the refusal is fast (0.63 s) and nothing is typed, but the hint line tells the driver to
  `viola wait <name>, then send again`, and that wait either has nothing to wake on (20 s to its deadline, or no
  deadline) or returns inside the window. What ends the window is time: at most 8.0 s from the paste.
- **Masters:** none, unless the hint line's wording is changed to say so (design-system §Surface: cli → Component
  Patterns 2; layout-templates §Surface: cli › Output structure — `viola send`). A wording change is its own small
  build.
- **`send`'s human lines and the hint:** stay byte for byte, as recorded live:
  ```
  [/ ] unable         hint-v  not-delivered  input-not-ready
  hint: hint-v was not ready for input; viola wait hint-v, then send again
  ```
- **A human keystroke in the window:** neither delayed nor refused.
- **`send`'s longest blocking time:** unchanged, 15 s.
- **The fake agent's cap:** the two standing cases replay this behaviour as they are.

## Beside the options: the `wait` finding
Whatever is chosen, a no-cursor `viola wait` issued after a turn has already ended wakes on nothing. The hint line
names that verb for a cause whose turn has always already ended. Under options 1 with 2, and 3, the line is no
longer printed for this cause; under option 1 alone it still is for most windows; under option 4 it stays, and its
advice is worth what the 20 s reading shows.

## Found beside the hint, not part of this choice
A `send` whose text ends in a newline is delivered and runs a turn, yet ends `not-delivered` /
`no-prompt-submitted` with the wheel moved to the human, on both homes: the CLI drops a pasted text's last newline
before the hook sees it (`live-shape-red-green.md`, STOP 5). It goes to the wrap as its own item.
