## 2026-10-31-exit-cause-code-catalogue — exit 21's document names its cause
**Section:** §Surface: cli (design-system.md: Tokens → Streams; Component Patterns 1 `viola list` and 2 `viola send`; Exit-code phraseology) · surface.md: cli-list (the Refusal item), cli-send (the strict-modes hint row), cli-exit-codes (row 21 and the server-verification item)
**Change:**
- Row 21's `--json` cell and the phraseology's code 21: was one document, `{"v":1,"error":"instance-unreachable","detail":null}`, its per-cause `detail` code awaiting an arch amendment; now `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, `<cause>` one of five closed words `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, stderr empty.
- The refused strict-modes check of `list`, `link`, `unlink`, `plugin install` and the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`): was the bare `instance-unreachable` document; now with `detail` `strict-modes-failed`. The board's Refusal item and pattern 1's as-built line now state it: the board's only refusal, one stdout document, stderr empty.
- The server-verification item: was `detail` null; now `detail` `server-verify-failed`.
- Pattern 2: was a per-cause `detail` code pending, owner the arch amendment naming them; now exit 21's document names its cause in `detail`, one of five closed words.
- Streams: was the typed `refusal` / `error` fields being the whole answer; now those fields and their `detail`, exit 21's cause being its `detail`.
**Why:** one document could not tell a same-user driver which of exit 21's causes it met; the chunk put the cause in the document. Crossing that boundary is the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1). That these rows lie inside his answer is the overseer's confirm at this wrap's halt, as delegate, PROVISIONAL (`inputs#I6`); the ratification stays the founder's.
**Kept:** `v` stays 1, `error` stays `instance-unreachable` and no exit moved; the human `unable` lines and fixed pairs stand as written.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `endpoint-lost` hint and the four hint causes
**Section:** §Surface: cli · surface.md: cli-send (the `instance-unreachable` hint list, the server-verification hint item), cli-exit-codes (row 21's human cell)
**Change:**
- A new hint row, a wrapper lost with the call open (`endpoint-lost`): `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII. It carries a gap marker: this wording PROVISIONAL, owner the wording of the `endpoint-lost` hint.
- Row 21's human cell: was not running / unwrapped / server verification; now not running / unwrapped / endpoint lost / server verification.
- The server-verification hint item: was the hint cause `server-verify` beside `not-running`; now beside `not-running`, `unwrapped-name` and `endpoint-lost`, with `not-running` named the hint key of the log's `instance-dead`.
**Why:** `instance-unreachable` keeps one hint per cause because the code is shared, and a wrapper lost mid-call is a cause of its own. The chunk built the unwrapped hint and added this one. The wording is the delegate's, the overseer's answer at the P4 fork, PROVISIONAL (`inputs#I2` Q3), and stays open until its owner settles it.
**Kept:** `not-running` stays the hint cause's key, not renamed to the log's `instance-dead`; the unwrapped hint's words did not move.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stale holder's own code
**Section:** §Surface: cli · surface.md: cli-send (the exit-1 start refusals of `viola run`), cli-exit-codes (the stale-instance item)
**Change:**
- A new row under the two holder refusals: these two print and log their own codes, a stale holder `stale-heartbeat`, a live holder and a fresh replayed beat `already-live`. Was the stale-heartbeat cause having no code yet.
- A new item: a frame verb against a stale instance reads `instance-dead`; stale heartbeat is exit 1's cause alone.
**Why:** the chunk gave a stale holder its own printed and logged code and retired `already-live` as a stale holder's refusal; a frame verb finds no live endpoint for a stale wrapper, so it never meets that code. The row landing over the file's "no code yet" line is the overseer's answer to this wrap's halt, Q4, as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** both holder refusals' human pairs stand as written.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the pending arch block spent, the log's codes and the two carve-outs
**Section:** §Surface: cli · surface.md: cli-send (the "Hints are human-mode only" item)
**Change:**
- Was a pending-an-arch-amendment block: arch fixing `detail` null for exit 21 and, for exit 1, only `viola revive`'s strict-modes and preflight refusals, `--json` keeping arch's shape until arch named the codes. Now exit 21's document names its cause in `detail`, one of five closed words (row 21); exit 1's, `detail` null, is printed by `viola revive` for every refusal of the start it shares with `run` and, as `internal-error`, by the `cli` catch site (row 1).
- obs-plan D-20's log detail codes gain `stale-heartbeat`, `unwrapped-name` and `endpoint-lost`, beside `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and for `viola revive` `no-session` and `cwd-missing`.
- The server-verification refusal's `--json` document: was `detail:null`, as the not-running cause's; now reads the same word as its log line.
- Was the stale-heartbeat and unwrapped-name causes having no code yet; now the socket-directory cause and macOS's server check stay without these codes, owner the route entry "Unix endpoint and home hardening".
**Why:** the block's wait is over: `detail` carries the cause by the founder's own word (`inputs#I2` Q1). The claim with its named carve-outs is his too, live, through the overseer's dialog (`inputs#I2` Q4), and the two causes left out stay with that route entry. The socket-directory row is also the delegate's, the overseer's P5 approval, PROVISIONAL (`inputs#I3`): the code holds no such refusal, its failure reaching the catch site as an anyhow error. The overseer's confirm at this wrap's halt is as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** `viola revive`'s strict-modes refusal still writes no log line, so it carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the catch site's `internal-error` document
**Section:** §Surface: cli (design-system.md: Tokens → Streams; Exit-code phraseology) · surface.md: cli-send (the no-hint fault item), cli-exit-codes (row 1's `--json` cell and the catch-site items)
**Change:**
- Row 1's `--json` cell and the phraseology's code 1: was no document at the catch site; now `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error` under `--json`, stderr then empty. Without `--json` the catch site prints that line, byte for byte.
- The no-hint fault item: was the `--json` document's `detail` being row 20's closed object for both faults; now row 20's closed object for the wrapper fault and null for the internal error.
- New: the catch site counts `--json` only among the arguments before a `--`, read from argv.
- New, in the items and in Streams: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
**Why:** a verb asked for one machine document had none where its error reached the catch site. The founder's own word, live, through the overseer's dialog, moved it, exit 1 and the human line staying (`inputs#I2` Q1). The document's item landing over row 1's "every other exit 1 has no document" is the overseer's answer to this wrap's halt, Q4, and the confirm of the rewritten lines its Q1, both as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** the human line is unchanged byte for byte, and neither fault gains a hint.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `revive --json`'s documents and its ten codes
**Section:** §Surface: cli (design-system.md: Tokens → Streams; Component Patterns 5 `viola verify` / `viola run`; Exit-code phraseology) · surface.md: cli-verify-run (the `revive` item and three new items), cli-exit-codes (row 1's `--json` cell)
**Change:**
- Was one document for a refused strict-modes check or a preflight refusal, five codes, and none for a refusal `run`'s start makes after a passed preflight, owed to the working-route entry "Exit-cause code catalogue". Now one stdout exit-1 document `{"v":1,"error":"<code>","detail":null}` for every refusal of the start it shares with `run`, no hint, stderr empty; the owed clause is gone.
- Its closed codes, was five, now ten: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`. `run` itself prints none of these documents.
- New: a program not found prints `internal-error`, a cause with no human pair; an anyhow error of its start prints the same document, and the catch site logs its chain and its line.
- New, in the item, row 1 and Streams: a `pump_child` error prints no document, the child having held the terminal and viola writing nothing past it.
**Why:** the chunk closed the post-preflight gap its route carried: "a refusal `run`'s start makes after a passed preflight prints no document" stopped being true. `run`'s causes reaching a machine only through `revive --json` is inside the founder's own answer, live, through the overseer's dialog (`inputs#I2` Q4). The codes and `internal-error` items landing over the file's five codes and no-document clause are the overseer's answer to this wrap's halt, Q4, and the confirm of the rewritten lines its Q1, both as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** a `pump_child` error stays without a document: a document printed from `main` when the start returned was weighed and rejected, since that error reaches the same catch site after the child has held the terminal.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — how a frame verb reads no endpoint and a lost call
**Section:** §Surface: cli · surface.md: cli-exit-codes (two new items)
**Change:**
- With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name`. It carries a gap marker: this reading PROVISIONAL, owner whether an unwrapped name reads by its instance directory.
- A failed connect is `instance-dead`; a wrapper lost with the call open is `endpoint-lost`.
**Why:** the five causes need a rule for which one a verb names, and the chunk built this one, the strict-modes check first at every caller. Reading an unwrapped name by its instance directory is the delegate's word, the overseer's at the P4 fork, PROVISIONAL (`inputs#I2` Q2), and stays open until its owner settles it. These items landing over the file's "no code yet" and `detail:null` lines are the overseer's answer to this wrap's halt, Q4, as delegate, PROVISIONAL (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
