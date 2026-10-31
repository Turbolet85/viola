## 2026-10-31-exit-cause-code-catalogue — exit 21's document carries its cause in `detail`
**Section:** .andromeda/layout-templates.md: §Output structure — `viola list --json`; §Output structure — `viola plugin install`; §Component — Primary content block 2 (Line form, Hint line, Exit codes as the typed tail)
**Change:**
- `viola list --json`'s failed strict-modes check of the home, `instances/` or any one instance: was `{"v":1,"error":"instance-unreachable","detail":null}`; now `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, exit 21, stderr empty.
- `viola plugin install --json`'s refused strict-modes check of the home or a `marketplace/` tree dir: was `"detail":null`; now `"detail":"strict-modes-failed"`, stderr empty.
- The frame verbs' exit-21 strict-modes refusal under `--json`: was `"detail":null`; now `"detail":"strict-modes-failed"`, stderr empty.
- A frame verb's failed or unmakeable server check under `--json`: was `"detail":null`; now `"detail":"server-verify-failed"`, stderr empty.
- Exit codes as the typed tail: was "the machine view carries the cause as a detail code once arch names those codes", keeping `"detail":null` for exit 21 until then; now the machine view of exit 21 carries the cause in its document's `detail`, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`.
**Why:** every exit-21 cause printed one document with `detail` null, so a driver could not tell the causes apart; the chunk built one document per cause and retired the null, moving no exit. The founder ruled it: his own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1). The overseer read these rows as lying inside his answers as delegate, PROVISIONAL (`inputs#I6`); the ratification stays the founder's.
**Kept:** `error` stays `instance-unreachable` and `v` stays 1 in every form. Exit 1's `{"v":1,"error":"<code>","detail":null}` document keeps its null. The human state pairs stand. macOS's server check stays on its recorded gap, a carve-out of the founder's answer (`inputs#I2` Q4) left with "Unix endpoint and home hardening".
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `cli` catch site's `internal-error` document under `--json`
**Section:** .andromeda/layout-templates.md: §Output structure — `viola wait` / `viola last`; §Output structure — `viola plugin install`; §Component — Primary content block 2 (Line form)
**Change:**
- Line form: was `error: internal error` once from the one catch site, with no hint, in every mode; now, with `--json` among the arguments before a `--`, `{"v":1,"error":"internal-error","detail":null}` on stdout in its place with stderr empty.
- `viola wait` / `viola last`: a failure or a caught panic, was the human line only; now also, under `--json`, that document on stdout with stderr empty.
- `viola plugin install`: a failed exe read, dir creation or file write, was the catch site's human line, exit 1; now also, under `--json`, that document on stdout with stderr empty.
**Why:** the chunk built it: the catch site reads `--json` from the arguments and prints the document in place of its stderr line. The founder ruled it: his own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1), exit 1 and the human line staying. The overseer read these rows as lying inside his answers as delegate, PROVISIONAL (`inputs#I6`); the ratification stays the founder's.
**Kept:** without `--json` the line stands, no hint. A `revive` start stays outside the catch site's list of verbs.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stale holder's code under `viola revive --json`
**Section:** .andromeda/layout-templates.md: §Output structure — `viola revive`
**Change:**
- The `--json` refusal codes: was `already-live` with no sibling; now `already-live` (a live holder or a fresh replayed beat) and `stale-heartbeat` (a stale holder), each one exit-1 document with stderr empty.
**Why:** the chunk's collision check prints and logs `stale-heartbeat` for a stale holder, retiring `already-live` as that holder's refusal.
**Kept:** without `--json` the pairs stay byte-identical.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `viola revive --json` post-preflight gap closed
**Section:** .andromeda/layout-templates.md: §Output structure — `viola revive`
**Change:**
- A refusal of the start `revive` shares with `run`, after a passed preflight: was no document as landed, owed to the working-route entry "Exit-cause code catalogue"; now one exit-1 document with stderr empty, `<code>` `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, or `internal-error` for a program not found and for an error of the start itself, whose chain the catch site still logs.
- A `pump_child` error prints no document, the child having held the terminal.
- The owed pointer to the working-route entry is gone.
**Why:** the chunk built the documents and the sentence "prints no document" stopped being true, closing the carry of chunk 2026-10-17-cli-machine-contract. The founder's answer makes `revive --json` the machine road to `run`'s start causes, `run` having no `--json`: his own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q4). The overseer read this row as lying inside his answers as delegate, PROVISIONAL (`inputs#I6`); the ratification stays the founder's. A document from `main` was rejected because a `pump_child` error reaches the same catch site after the child has held the terminal.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the unwrapped-name and endpoint-lost hints
**Section:** .andromeda/layout-templates.md: §Component — Primary content block 2 (Hint line)
**Change:**
- A frame verb's exit 21 on a name with no directory at its instance path: new text, `hint: <name> is not a wrapped session; only sessions started with viola run take commands`.
- A frame verb's exit 21 with its wrapper lost while the call is open: new text, `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`.
- Each is the last stderr line, in plain ASCII.
**Why:** the chunk built the unwrapped hint and added the endpoint-lost one, so each of these causes has its own hint. The directory reading and the words "went away during the call" are the delegate's: `inputs#I2` Q2 and Q3, delegate the overseer, PROVISIONAL. The body carries no PROVISIONAL mark; the record does.
**Kept:** the not-running hint stands word for word, and the bullet's "no hint" list did not move.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
