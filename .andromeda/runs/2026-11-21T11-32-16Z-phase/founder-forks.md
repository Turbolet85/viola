# Founder forks — «Cross-session prefix ledger row» (`2026-11-21-cross-session-prefix-ledger-row`), the P3 block

Written at the end of P3, before P4 (`inputs#I1` item 5). No start was made, nothing of the operator's real
`~/.claude*` was read, no dialog was raised. The phase stops here and **waits**: the cap, any key or byte into a CLI
dialog, every widening, what «verified» rests on and any read or write under his real home are his.

## §0 Window
This window ran the take-up (promoted `tx 3637`, the directive snapped `inputs#I1`), the row read (7 task writers,
156 picker draws, 7 history agents, every check passed first try, 1 158 of 4 771 rows picked) and P3 (research
`rev 1`, scope closed at `rev 3`). P4 opens on the answers; it may open in a fresh window (Setup 3 (c) resumes the
take-up at P4 from `research`).

## §1 The cause of `no-prompt`, as the bundle reads it (no start)
`HF` read `cross-session-prefix: fail`, `cross_session_probe` = `{outcome: no-prompt, prefix: null, latency_ms: null}`:
the sender exited 0, nothing arrived in 30 s, no dialog stood after. Read since, with no start
(research §Patterns detected; byte offsets there):

| Candidate | What the 2.1.328 bundle states | What only a start can tell |
|---|---|---|
| **C1 — the model did not call the send tool** | Nothing either way: the tool's own prompt tells the model to use `ListAgents` for targets and allows «an id … already in their message» as `to` — the sender's prompt does put Run B's id in its message. | Whether Haiku called it. |
| **C2 — cross-session sending is off in the sender** (new) | The send gate is off when the settings say `messagingBeyondOwnAgentsDisabled`, else when `CLAUDE_CODE_HARBOR_KITE` says so (verify strips that name), else by the remote flag `tengu_harbor_kite`, default **on**. Off, the tool still runs and answers «Cross-session messaging is not available in this session.»; the sender still exits 0. | The flag's value in the rig's private config. |
| **C3 — the deferred tool was not reached** | `SendMessage` is deferred only while tool search is on the wire; with it absent the tool is sent inline. `--allowedTools` is a **permission** list (help: «tool names to allow»), not an availability list (`--tools` is), so the one-tool allow-list removed no tool. Whether the tool-search call itself needs a permission `-p` denies is not settled by strings. | Whether tool search was on the Haiku request, and whether its fetch was made. |
| **C4 — the id did not resolve** | A target resolves against a roster read from `<config dir>/sessions`. A local row holds a socket, a pin, claimed session ids and a name; how a bare id is matched is not resolved by strings. The socket form (`uds:<path>`) needs no lookup but sits behind the same gate. Both sender and Run B run under the rig's one private config, and Run B was listed there (`agents-join: pass`). | Whether the id matched a row. |
| **C5 — Run B held or dropped it** (new) | Inbound policy: `accept` admits; a startup hold («delivered once the session finishes starting up»); a kill switch that refuses and drops; a permission-mode mismatch held unless `accept`; messages «drain at the receiver's next tool round». Run B's inbox WAS bound: `HF`'s `env_probe` lists `CLAUDE_CODE_MESSAGING_SOCKET` and `_TOKEN` on Run B's hook child (names only). A hold would ask for approval in a terminal, and `HF` saw no dialog after. | Whether a message reached Run B's inbox at all. |

**2.1.328 → 2.1.330:** every string above stands in both binaries, but one: the deferral function's
`toolSearchAbsent` default-parameter text stands in 2.1.328 and not in 2.1.330 (a re-minification or a change; not
resolved). **The row is stamped on 2.1.328**: the recorded set and the three default constants read it. 2.1.330 is
`mise latest` and the bare `claude` now; moving the proof to it is his word only (harness rule, 2026-10-10 ext. 2026-11-13).

**Net:** two of five candidates (C2, C5) are new since `HF`. The bundle makes C3-by-allow-list unlikely and C5-by-
closed-inbox unlikely, and settles none. Read 1 below splits C1/C3 (never called) from C2/C4/C5 (called); read 2
splits C2/C4 (refused) from C5 (delivered, not arrived).

## §2 F1 — the diagnostic reading (a widening he has not seen; `inputs#I1` item 3)
What is read, from where, what is kept, where written:
- **Read 1 — the sender's tool calls by NAME.** The sender gets a capture plugin (`--plugin-dir`, its own 0700 dir
  inside the print-probe dir) whose `PreToolUse` and `PostToolUse` hooks run the existing hidden `hook --capture` arm
  (raw payload files, 0600, removed with the probe dir — as Runs B–D's captures are today). verify reads in memory only
  each `PreToolUse` payload's `tool_name`, mapped to the closed set `send` · `search` · `list` · `other`.
- **Read 2 — the send tool's result CLASS.** From the `SendMessage` `PostToolUse` payload, only whether its
  `tool_response` reads a success: the closed set `delivered` · `refused` · `none`. No text read.
- **Kept:** two keys more in the stamp's `cross_session_probe`: `sender_tools` (the closed words, in order, deduped)
  and `send_result` (one closed word). Nothing else; no sender payload ever goes into `--record`'s fixtures.
- **No-text proof:** a unit case feeds the classifier payloads carrying a canary in `tool_input.message`,
  `tool_response` and a model-text field, and asserts the stamp bytes hold no canary byte; the stamp key set is pinned
  by a literal.
- **On disk, transiently:** the sender's raw payloads include `tool_input` (`to` = Run B's id, `message` = the
  compiled probe text) and `tool_response` (the CLI's text) in the 0600 probe dir until its removal — the same
  residual class as Runs B–D's captures today.

Options, in order:
- **(a) Both reads (Recommended).** One start can then name C1/C3 vs C2/C4 vs C5. Cost: the sender's argv gains
  `--plugin-dir`; the capture arm's rows gain a sender plugin; the stamp gains two closed-word keys. Rows: listed in §4.
- **(b) Read 1 only.** Splits «never called» from «called»; a called-but-failed red stays C2/C4/C5. Cost: the same
  argv; one more start likely.
- **(c) No reading.** The design as `HF` ran it. A red again names nothing; the next start is a guess.

## §3 F2 — the answer path (how the row gets to `pass`)
- **(A) Diagnose, then mend, then re-record (Recommended).** `S1` runs the founder's path (A) with F1's reads. Green at
  29/29 records the set at once and the chunk is done. Red names the cause; implement stops; a revision plans the one
  mend the reading names; `S2` re-records. Fails only if the model's behaviour varies run to run (a `send` reading in
  one start, none in the next): the spare is for that.
- **(B) Mend on a guess first** (e.g. the sender's prompt also asks for the tool-search fetch, or `to` = Run B's socket
  path). Set aside: a red still names nothing, and the socket form makes verify read the path of Run B's socket under
  the real runtime dir, a read viola does not make today.
- **(C) A sender with no model** (viola writes into Run B's socket itself). Set aside: it needs the auth key the
  registry holds, a NEVER-log-floor secret.
- **(D) Leave v2-25 out of this version** (the cap deferred). Set aside unless he chooses it: the harness-prefix
  reliance stays unmeasured.

## §4 Every master row the change reaches (all seven searched: the union's rows files by search, the masters by `rec.py find`)
**Reached — the ground the last chunk's wrap cut, none landed** (`rec.py find` for `crossSessionInbound` ·
`verify-sender` · `cross_session_probe` · `cross-session-prefix` · `misaddress` over architecture, security-plan,
obs-plan and test-plan: 0 hits):
- architecture — `decision/verify-child-set` (the sender; Run B's cross-session turn; W1) ·
  `contract/ledger-stamps/content` (the `cross_session_probe` block; with F1 its two keys) ·
  `decision/verify-transcripts-accepted-residual` (points to the new residual row) ·
  `decision/harness-prompts-not-human` and `decision/harness-prompt-prefixes` (the reliance the row measures; words
  unchanged, cited) · `decision/capability-ledger` (a 29th relied row; no count stated) ·
  `decision/start-verifies-new-cli-version` and `contract/config-json/verify-new-cli` (the automatic verify now runs
  the sender with no human, in the user's own config).
- security-plan — `decision/verify-children-direct-spawn` (the sender, a direct non-PTY spawn under the R8 strip; Run
  B's `--settings` accept literal; W1, W4) · `decision/verify-trusted-runs-risk` (Run B's open inbound window for its
  life; W4) · a NEW residual row beside `decision/verify-echo-probe-residuals`: W3 the misaddressing residual in a real
  config, the automatic verify included; W4 what the accept literal cannot override (managed policy, a tightening repo
  or local setting, an invalid value, the CLI's receive switch) — each a red; T5 the sender's leavings, unmeasured (a
  registry entry, an input-history line, a socket under `$XDG_RUNTIME_DIR/cc-socks/` while it runs) ·
  `decision/verify-real-cli-transcripts-accepted` (points to it) · `decision/live-rig-kept-stamp-owner-only` (the new
  labels) · `decision/probe-prompts-synthetic` (the sender's compiled texts) · `decision/fixture-scrub-and-refusal`
  (the cross-session fixture class).
- obs-plan — new `contract/verify-spawn-subject/verify-sender` and `contract/diag-subject/verify-sender`;
  `contract/process-log/child-and-probe-lifecycle` (names the sender).
- test-plan — `decision/hook-payloads-recorded-by-verify` · `decision/fake-agent-matches-verify-fixtures` ·
  `decision/fake-agent-replay-for-stamps` · `decision/one-verify-stamp-per-build` (the cross-session variant) ·
  `decision/standing-live-rig` (the new labels) · `decision/ledger-row-verify-probe` (its probe, held).
- **With F1 (a) or (b) only:** security-plan `decision/hook-capture-arm` and `decision/verify-probe-captures-transient`
  (a sender capture plugin, its transient payloads) · architecture `decision/probe-capture-arms-inert` (held, a third
  probe plugin kind) — the widening of §2.

**Read as not reached:** security-plan `decision/harness-prefix-literals` (the four literals stay) · architecture
`decision/send-confirmed-after-the-fact` · obs-plan `decision/send-text-never-logged` (the sender's text is a compiled
probe literal, not a driver send) · design-system `contract/auto-verify-line/*` and every `contract/viola-verify-output/*`
row, layout-templates' too (a 29th step line keeps the form; no row states a count) · a11y-plan
`decision/time-bounds-fall-on-the-driver` and `decision/human-keystrokes-never-blocked` (the automatic verify's window
lengthens; no word moves).

## §5 Every live start's shape (before the count)
Both starts are the rig's `VR` arm on claude **2.1.328 by path**, in the rig's scratch home under its private Claude
config; labels new to this chunk (proposed `XS`, `XS-2`; `HF-2` would overwrite `HF`'s kept stamp).
- **Processes:** `viola-live-rig live XS --cli 2.1.328 --cap {cap} --spare {spare} --ledger <file>` → `viola verify
  --record --home <scratch>` → its version probe, print probe, Runs A–F (six PTY children), the one agents read in
  Run B, and the **sender**: `claude -p <compiled prompt + Run B's id> --model haiku --no-session-persistence
  --allowedTools SendMessage` (+ `--plugin-dir <sender capture plugin>` under F1), a direct spawn, the R8 strip, cwd the
  print-probe dir, bound 120 s, spawned between `/compact`'s answer and `/clear` (lean T1, standing).
- **Every byte typed:** Run B's compiled pastes as today; nothing is typed for the cross-session turn (the sender's
  prompt is argv). No byte into any dialog.
- **Every dialog and its answer:** Run A's trust — killed, no key; Runs C/D — answered only by the hidden capture
  answers arm, as today; the browser-tools prompt — left; **a held-message approval in Run B — never answered**: the
  box guard then reads not-up, `/clear` is not pasted and Run B ends by a kill, no key (lean T2, standing).
- **The stop:** the rig's census kills only its own rows; the scratch home is removed.
- **Files written, and where:** the scratch home (removed); the kept stamp `target/live-rig/cross-stamp.json` (`XS`) /
  `cross-stamp-2.json` (`XS-2`), 0600; the rig ledger line; under `--record` on a 29/29 pass only, the re-recorded
  `fixtures/claude/2.1.328/` (32 → 33 files, `UserPromptSubmit.cross-session-1.json` added); the sender's unmeasured
  leavings (§4, T5) — its registry entry and input-history line inside the private config (removed with the scratch),
  a socket in the real `$XDG_RUNTIME_DIR/cc-socks/` while it runs.
- **The wheel:** no `viola run` wrapper stands in a verify; no typed key or answered dialog moves a wheel.
- **What each start reads** (names, counts, closed words — never a text): the step lines; the kept stamp's rows and
  `cross_session_probe` (`outcome`, `prefix` id, `latency_ms`, with F1 `sender_tools` and `send_result`); the arrived
  prompt's raw start against the four compiled literals, in memory.
- **Predicates that can FAIL:** `S1` — `cross-session-prefix: pass`, `pass-count: 29/29`, kept stamp `29 ok`; red:
  `fail` with its outcome word (`no-prompt` · `unprefixed` · `sender-failed` · `sender-deadline`). F1's words are
  READINGS recorded, never atoms. `S2` — the same three.
- **On disk before / after each start (no-start gate entries, placed so a resumed round re-fires them):** ABSENT
  `target/live-rig/cross-stamp.json` before `S1` (`cross-stamp-2.json` before `S2`); PRESENT and unchanged
  `floor-stamp.json` and `floor-stamp-2.json` (`HF`/`HG`'s kept products, sha read before and after, left as found);
  ABSENT the rig scratch base; `claude --version` by path reads 2.1.328 and `mise ls claude` re-read; the build's hash
  read right before; `hostwatch.py read --last 15` QUIET (a CONTENDED reading naming a neighbour stops and is re-read);
  AFTER: the kept stamp read, the scratch absent, no socket of a rig-started pid left under `cc-socks/`.

## §6 Versions, stamps and the fixture set
A 29th relied id makes every stamp without it read unverified: the committed tests' fake-agent stamp is re-made by the
plan; the live home's 2.1.328 and 2.1.330 stamps — already four relied ids short (CARRY cry-fqwjpnqt) — go five
short, until the product re-pin step re-stamps them (no change here; the pinned product reads its own 24-row stamp).
Only `2.1.328`'s fixture set changes. The older sets (`2.1.283` … `2.1.325`) hold no cross-session variant; whether the
ledger-probe contract reads them under the relied set is a P4 read.

## §7 The capability, and what «verified» would rest on
v2-25, its acceptance as the matrix holds it with the label moved `HF` → `XS`: `S1` prints
`cross-session-prefix: pass`, `cross-session: harness`, `pass-count: 29/29`, its kept stamp reads `29 ok`; the recorded
`UserPromptSubmit.cross-session-1.json` is replayed by the fake agent in CI at `29 pass 0 fail`; under the accept literal
the row still reads `fail` when no prompt arrives or it arrives unprefixed
(`cross_session_prefix_keeps_its_red_under_the_accept_settings`). A red start verifies nothing.

## §8 Cap, spare, window
- **Recommended cap 2 base starts + 1 spare.** `S1` diagnoses and, green, records. `S2` only on a red that names a cause,
  after its mend. The spare: a confounded reading (the model's run-to-run variance, C1) or a mend that needs its own
  proof. A start the rig refuses for the token window or the host is uncounted, and the round stops.
- **One implement window holds it only if `S1` reads green.** A red stops implement at `S1`; the mend is a revision and
  `S2` rides that revision's implement window.

## §9 The answers this block waits for
1. F1 — (a) / (b) / (c).
2. F2 — (A) / (B) / (C) / (D).
3. The cap and spare (recommended 2 + 1) and the labels (`XS`, `XS-2`).
4. The version: 2.1.328 (recommended), or 2.1.330.
5. Confirm W1, W3, W4 and T5 stand as he answered them for the cut design (this plan writes them again, `inputs#I1`
   Facts), and that §4's rows are the whole reach.

## The answers — appended at P4, 2026-11-21T12:11:13Z (`inputs#I2`, the relay copied verbatim below its heading)

Authority: items 1–4 are the operator's own words, given in one dialog and read at 2026-11-21T12:10:30Z, relayed verbatim by the overseer; items 5–12 are the overseer's technical answers.

## The operator's own words — given in one dialog and read at 2026-11-21T12:10:30Z

1. **F1, the diagnostic reading: «Оба чтения, только имена (Recommended)».** Your option (a), IN THE NARROWED FORM
   he was shown and nothing wider: the sender's hooks write to disk ONLY the tool's name and, for the send, one
   boolean result — no arguments, no model text, no raw payload — and the stamp's `cross_session_probe` gains two
   closed-word keys: the tool names called (`send` · `search` · `list` · `other`) and the send's result class
   (`delivered` · `refused` · `none` · `unread`). He was told it also runs under the automatic verify in a
   user's real config, and that the raw-payload form was set aside because it would outlive a SIGKILL in the
   user's home. This is his ratification of that widening of the capture arm and of the stamp's content, over
   the rows your §4 names for F1, and nothing wider.
2. **The cap: «2 базовых + 1 запасной (Recommended)».** TWO base starts and ONE spare on one ledger, labels `XS`
   and `XS-2`, each in the shape shown to him: 2.1.328 by path, the rig's scratch home and private config, no
   key and no byte into any CLI dialog, one read of his credentials file by the rig, nothing written under his
   real `~/.claude*` or `~/.viola`, the sender's socket in the real `$XDG_RUNTIME_DIR/cc-socks/` while it runs,
   the sender `claude -p` on Haiku. `XS` (`--cap 1`) diagnoses and, green at 29/29, records. `XS-2` (`--cap 2`)
   opens ONLY after a red that NAMES a cause and its mend through the revision door. The spare: the model's
   run-to-run variance, a rig refusal before the ledger line, or a host CONTENDED by a neighbour. A red stops
   implement. A third base start is his.
3. **The version: «2.1.328 (Recommended)».** The row is stamped on 2.1.328 alone; no 2.1.330 start; the one
   fixture set is re-recorded (32 → 33 files) on a 29/29 pass only.
4. **His earlier answers: «Да, в силе (Recommended)».** Path (A) the print-mode sender with its one allowed tool,
   the `accept` literal on Run B, and the accepted residuals W1, W3, W4 and T5 stand for this chunk as he gave
   them, over the rows your §4 names; the plan writes them again. No widening beyond those and item 1.

## Technical answers (the overseer's)

5. **F2: path (A)** — diagnose, mend, re-record. (B) and (C) are declined for the reasons your block gives; (D)
   was offered to him and not chosen.
6. **The names-only capture mode is built as you proposed in your answer to my two questions:** a mode of the
   capture arm that never writes a payload — one small 0600 file of closed words, as the env-names file is. Its
   proof is the canary test over `tool_input`, `tool_response` and a model-text field PLUS a case that the mode
   leaves no payload file in the probe dir at all (so a SIGKILL leaves closed words only). Say in the plan what
   file a SIGKILL leaves and that its content is closed words; the residual row's words carry it.
7. **Read 2's field is unmeasured until `XS`:** the classifier reads a structured boolean only, in the shapes
   the tool's code states; any other or absent shape reads `unread`, never `refused`; a failed input validation
   that fires no PostToolUse reads `none`. No text match, no compiled literal. `delivered` means the tool
   reported success, not that Run B received a prompt — the row's predicate stays the arrived prompt's prefix.
   Every F1 word is a READING recorded, never an expect atom.
8. **`XS` must be able to name the cause in ONE start:** the plan writes, before the round, the table from
   readings to cause (which `sender_tools` / `send_result` / `outcome` triple reads C1, C2-or-C4, C3, C5) and
   which triples stay ambiguous — an ambiguous red is said as one, and what start would split it is his.
9. **Labels and kept products:** `XS` → `target/live-rig/cross-stamp.json`, `XS-2` → `cross-stamp-2.json`; the
   security-plan kept-stamp row grows by the two label NAMES (confirmed by the overseer under his standing
   word; the rule unchanged). The no-start entries of your §5 stand, each an entry or true by construction WITH
   its code line; `floor-stamp.json` and `floor-stamp-2.json` read unchanged before and after.
10. **The older fixture sets** (2.1.283 … 2.1.325) under a 29th relied id: read at P4; if the ledger-probe
    contract then reads them red, that is a fork of its own in `founder-forks.md`, not a fold.
11. **Order:** the fake-agent cases, the keeps-its-red case, the canary and no-payload cases and every control
    green before the round; an entry a LATER plan step falsifies is keyed to the round leg; the round's span
    holds round and live entries only. A row NOT in your §4 that P4 finds reached is a fork written to
    `founder-forks.md`.
12. **The seam after P5.** When the plan is approved the phase ENDS and nothing else starts. On an account-limit
    refusal, stop where you are.
