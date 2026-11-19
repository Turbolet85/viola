# Founder forks — «A live StopFailure payload» (chunk 2026-11-19-a-live-stopfailure-payload), P3 end

The phase stops here, before P4, on the overseer's directive (inputs#I1 item 4). One block: the facts, then the
founder's three questions, then two technical questions for the overseer. No live start was made; nothing of the
operator's real `~/.claude*` was read. Every static fact below is in `doc:2026-11-19-a-live-stopfailure-payload/research`
§Containment, read from the installed 2.1.328 bundle, the rig's code and the host's tools with no start.

## The block

**Window.** P1 to P3 ran in this one builder window (the row read: 7 tasks, 156 picker draws in 3 parts, 7 history
agents, all first-try). Its context use is not measurable exactly from inside it; estimated about half spent. P4 and
P5 fit in this window once the answers land; implement takes a fresh window.

**One implement window, no split — unless outcome A is taken (Q3).** The chunk is test-only work in `viola-e2e` plus
one fake-agent option: (1) C5's mend — `keep_record` refuses per version dir, `<out>/<label>/record/<version>/`,
never the label's whole `record` dir (`crates/viola-e2e/src/rig/isolate.rs:753-756`); (2) one new rig arm `SF`
(labels `SF` and spare `SF-2`), a `CX`-shaped wrapped start run inside a network-closed sandbox; (3) its rehearsal
over the fake agent; (4) the live round. Outcome A needs a product change to `viola verify` and a re-stamp, and
splits into a new entry placed directly after this one.

**The containment chosen (no fork: no root, no system-wide change, no change to the operator's config).** `bwrap
--unshare-net --dev-bind / / --tmpfs /run/systemd/resolve -- <viola run …>`, unprivileged (bwrap 0.13.1, not
setuid; `unprivileged_userns_clone` = 1). Measured with no start: uid 1000 kept, only `lo` inside, a TCP connect
to the API host fails, and the host resolver's socket hidden so no name lookup leaves the host (with plain `unshare
--net` a lookup still crossed through systemd-resolved). `ANTHROPIC_BASE_URL` at a closed port is rejected: it moves
only the Messages API, while the CLI's other calls (OAuth profile, flags, telemetry, update check) still leave the
host. What the sandbox still lets the child reach: filesystem Unix sockets its paths name (viola's own socket
directory, needed; the session bus and the like under the kept `XDG_RUNTIME_DIR`).

**Credentials.** The contained child neither authenticates nor is refused: no request leaves the host. It reads the
rig's 0600 copy from its private config; the rig reads the real `~/.claude/.credentials.json` once (its token-window
check) and stamps `~/.claude.json` before and after, read-only, as every earlier start; nothing is written under
the real `~/.claude*` or `~/.viola`; no byte goes into any CLI dialog. A contained start spends no account usage.

**What 2.1.328 is expected to do (static, a reading the start records).** The turn's request fails at the connect;
the CLI retries (10 by default, reconnect budget 0 offline), ends the turn with an API-error message whose `error`
word is `server_error`, and fires StopFailure once with it; viola maps `server_error` to cause `server-error`
(`crates/viola-agent-claude/src/hook.rs:235-255`). A turn must be in flight: StopFailure fires only from a turn's
request.

**The live start's SHAPE (`SF`; its spare `SF-2` identical).**
- Processes: the rig → `bwrap` → `viola --home <scratch vhome> run sf-probe -- <2.1.328 claude by path>` → the
  claude child and its hooks (`viola hook …` from the plugin; one `viola-live-rig capture` hook for StopFailure in
  the private config's settings.json) — all inside the sandbox; and, outside it, one `viola --home <scratch vhome>
  send sf-probe --json --file <scratch>/texts/1.txt` from the rig.
- Every byte typed: the rig types NO key (unless Q2 folds C3: then one `x` and one DEL into the ready box, inside the
  existing grant, before the send). The one send types, through viola, the compiled literal `reply with the single
  word ok` (the rig's `MT_PROMPT`, already compiled) and its Enter.
- The stop: after the turn end is read (or at the arm's bound), Ctrl-C into the rig's own pty, a no-key group kill
  past `STOP_KILL_AFTER` — the rig's standing stop; the watchdog `--deadline 600`.
- Every file written: the scratch under the rig's default scratch base (`/tmp/viola-rig-<uid>/<run>/`: private
  config with the 0600 credentials copy, seeded markers and the capture hook; cwd; scratch viola home; capture file;
  the text file) — removed whole at teardown; kept frames under `target/live-rig/<run>/` (gitignored); one start
  line and one summary line in the ledger file the plan names under `target/live-rig/`; nothing kept as a record
  (outcome B), nothing under the real home.

**Its predicates — each can fail, the red reading named.**
1. `net-ns: own` — the child's `/proc/<pid>/ns/net` differs from the rig's (measured inside `net:[4026533305]`
   against `net:[4026531833]` outside). Red: `shared`.
2. `box: up` and `modal: none` on the first frame before any send. Red: `refused: shape` (an offline modal or
   preflight screen) — the start is spent with no payload.
3. `send-1`: `prompt-submitted` read back with the literal. Red: no read-back within the bound.
4. `turn-end: stop-failure` — a `turn-ended` line carrying `cause`. Red: `stop` (a Stop with no cause: the API was
   reached, the start was NOT contained) or `none` (no turn end within the bound).
5. `cause` equals the map of the captured `error` word (the capture keeps the word only when it is one of the CLI's
   13, else `other`). Red: a mismatch.
6. `unknown-value-lines: 0` in the hook role file when the captured word is one of the 13. Red: any such line.
   Recorded as readings, not atoms: the word itself (`server_error` expected, `unknown` possible) and the
   turn-end delay in seconds.
7. The standing teardown: census 0, credentials removed, scratch absent, user config `unchanged` or `moved` never
   `changed-by-rig`. Red: any other.

**Disk, absent or present, each read by a no-start gate entry right before its start (placed so a round resumed at
one entry still fires it), and after.**
- Before `SF`: the 2.1.328 install present (`~/.local/share/mise/installs/claude/2.1.328/claude`, executable);
  `bwrap` on `PATH` and its sandbox reading `net-ns: own` on a no-op child; the rig's census of leftovers 0; the
  scratch base holding no earlier run; the built `viola` hash read right before the start (the hard-link re-point,
  verification-harness 2026-10-20); the ledger file's base-start count below the cap; `target/live-rig/SF/` absent
  (no kept record expected; its presence would mean an earlier round ran).
- After `SF`: an entry reading the ledger's summary line for the start (predicates 1-7 as recorded words), and the
  census of leftovers 0 again. A census entry after the last live leg is keyed to the operator leg (inputs#I1 item 6).
- Before C5's proof: `target/live-rig/R4-2/record/2.1.328/` and `target/live-rig/R4/record-2.1.325/` stand as today;
  C5 is proven by unit and rehearsal cases with no live start.

**Recommended cap: 1 base start (`SF`) + 1 spare (`SF-2`).** The spare is for a start spent before the reading by
a cause outside it — predicate 2's offline first frame, the host's token window or a host stall — never for a red
on predicates 1, 4, 5 or 6, which are readings and stop the round. No other live start: C5 needs none; C3 rides
`SF` (Q2); C4 is acted on only at its next red.

**The exception's two outcomes (its settlement is the founder's).**
- **A — a ledger row with its probe.** Needs from the reading: a green `SF` (predicates 1-7), the payload's key set
  and its `error` word on 2.1.328. Then, in its own new entry: a 25th relied row in `viola-agent-claude`'s ledger,
  a `viola verify` probe that provokes a StopFailure — a seventh interactive verify run inside a network-closed
  sandbox, a product dependency on `bwrap`/user namespaces and a widening of verify's six ratified children; a
  `StopFailure.<variant>.json` recorded through `verify --record`'s scrub; the fake's replay; every CLI version
  re-stamped (until then the new row reads absent and leaves the version unverified, so dialog answers stop — on a
  host without user namespaces, for good).
- **B — the exception kept on his word.** Needs from the reading: a green `SF`, predicate 5's match and predicate 6's
  zero on the live word. Then only the wrap's amendments: the two architecture rows' exception text and the
  security and test rows cite the live reading (2.1.328, the date, the start). No product change.

## The founder's questions

**Q1 — the live cap for this chunk.**
1. **1 base start `SF` + 1 spare `SF-2` (Recommended)** — one reading of the payload; the spare only for a start
   spent before the reading. Cost: up to two contained starts of about five minutes each, no account usage.
2. 1 base start, no spare — cheapest; a start lost to the offline first frame or a stall leaves the carry open to a
   later entry.
3. 2 base starts + 1 spare — a second reading of the word (e.g. under `unshare` without the resolver hidden, a
   `ENOTFOUND` versus refused case); costs one more start for a word the static read already expects to be the same.

**Q2 — fold C3 (the placeholder hint's cell attribute) into `SF`.**
1. **Fold it (Recommended)** — before the send, `SF` reads the empty box's hint cells' attribute as one closed word,
   types one `x`, reads that cell's attribute, types one DEL (inside the existing x-and-DEL grant), then sends; no
   extra start. Cost: two keys into the rig's own pty and a screen-model reading of cell attributes (never text).
2. Leave C3 out — `SF` types no key; C3 stays PROVISIONAL and is named leaving with its next reader.

**Q3 — when and how the exception is settled.**
1. **On the reading, at this chunk's wrap: B if `SF` reads green on predicates 1-7, otherwise kept PROVISIONAL with
   the red named (Recommended)** — the settlement rests on the live word, as C1 asks; A stays open as a later entry
   only if he asks for it then.
2. A now — this chunk records the reading; a new entry «StopFailure as a verify ledger row» is minted after it for
   the product half (the verify probe, the re-stamp).
3. B now, regardless of the reading — the reading is recorded, the exception kept on his word as it stands.

## Technical questions for the overseer

- **T1.** Does CI's `ubuntu-latest` carry `bwrap` and admit an unprivileged user namespace? The lean, if unknown:
  the CI rehearsal runs `SF` unwrapped over the fake agent, and a local no-start gate entry rehearses it wrapped on
  this host before the live start.
- **T2.** `bwrap` (resolver socket hidden) over `unshare --user --map-current-user --net` (names still resolve
  through the host resolver) as the rig's sandbox: the lean is `bwrap`, a test-only host dependency of the rig alone.

## Answers (appended at P4; the relay `~/dev/projects/additional/viola-overseer/relay-stopfailure-p3.md`, snapped as inputs#I3)

The founder's answers, given in a dialog and read at 2026-11-19T16:30:15Z, his own words:
- **Q1:** «1 + 1 запасной (Recommended)» — one base start `SF` and one spare `SF-2`; the spare only for a start lost
  before the reading to a cause outside it, never on a red of predicates 1, 4, 5 or 6; each start ledgered with
  `date -u` before it is made; a token-window refusal uncounted (stop and say so); a second base start is his.
- **Q2:** «Сложить в SF (Recommended)» — one `x` and one DEL into the ready box before the send, cell attributes
  only, never text; an attribute reading that cannot be taken is `unread` with its reason, a reading, not a red.
- **Q3:** «По чтению, на wrap: B при зелёном (Recommended)» — green on predicates 1-7 → outcome B at this chunk's
  wrap, the master rows citing the live reading; red → PROVISIONAL with the red named; outcome A not planned here.

The overseer's technical answers:
- **T2:** `bwrap` with the resolver socket hidden, a test-only host dependency of the rig, named in the plan with its
  no-start entry (`net-ns: own` on a no-op child); the `SF` arm refused before its ledger line where `bwrap` is
  missing or the namespace refused.
- **T1:** plan as if CI cannot run the sandbox (hypothesis): the CI rehearsal runs `SF` unwrapped; a local no-start
  entry rehearses it wrapped before the live start; no CI test needs `bwrap`, a wrapped test skips by a printed probe.
- Items 6-9: predicate 4's `stop` means the API was reached — not contained, the round stops, nothing retried; the
  plan names what the child can still write outside the scratch and the witness that reads it (`lacks user-config:
  unread` a stopping red, `changed-by-others` a reading); C5 lands before the live round; `SF`'s deadline covers the
  retries with room, the expected delay and the bound named; the phase ends at the plan's approval.

## P4 round — two forks the answers above do not cover (the plan stops before P5 on them)

Plan rev 2 (`doc:2026-11-19-a-live-stopfailure-payload/plan`) is written up to both. §C.4 and §C.5 are HELD
behind his word, and each has a fallback written in its Implementation notes. Nothing below adds a live start.

**F1 — the rig reading the StopFailure payload's error word (a boundary widening; playbook «Boundary widening —
what ratifies it», rul-p2zl2kgl).** Predicate 5 as shown in the P3 block («the capture keeps the word only when it
is one of the CLI's 13, else `other`») needs the rig to read a VALUE of a Claude payload. Today the rig reads only
key names and one source word of its own UserPromptSubmit capture
(`(security-plan · decision/live-rig-prompt-capture-closed)`), and the Claude-shape exception reads «Test-only, no
value» (`(architecture · decision/claude-shapes-in-one-crate)`). The P3 block showed predicate 5 but did not show
it as a widening, so his Q1 and Q3 answers do not ratify it.
1. **Name it (Recommended).** The rig's capture of a StopFailure payload keeps its key names (`[a-z_]{1,40}`,
   others counted), the error word only when it is one of the CLI's 13 compiled words (else `other`), and the byte
   lengths of `error_details` and `last_assistant_message` — never a text. Predicate 5 stands as he answered Q3:
   the turn end's cause equals the map of the captured word. Cost: one test-only capture, and the two rows above
   each gain the named reading at the wrap, as his answer.
2. **No capture.** Predicate 5 reads from the product path alone: the `cause` word and predicate 6's line count.
   `server-error` names the word exactly (its one preimage, `server_error`); `auth-failed` names five words as a
   class; `api-error` with 0 lines means `unknown` or no word, with 1 line a word outside the 13. Cost: no
   widening, but predicate 5 changes from what his Q3 answer was keyed to, and an `auth-failed` or `api-error`
   reading names a class, not the word.

**F2 — where C3's `x` and DEL go (a departure from his Q2 answer).** He answered «one `x` and one DEL into the
ready box … before the send». In a wrapped start, any editing byte since the last turn boundary moves the wheel to
the human, and the automated send is then refused `human-typing`
(`(architecture · decision/byte-source-wheel-claim)`). Keys before the send would refuse `SF`'s one send and lose
the payload reading.
1. **After the turn end (Recommended).** Once `SF`'s StopFailure turn end is read, read the box's hint-cell
   attributes, type one `x`, read its cell's attributes, type one DEL, then the rig's stop. Cost: none to the
   payload reading. The box after an API-error turn may draw differently from a fresh box; if it holds no hint, the
   word reads `unread` with its reason, a reading, not a red (his Q2 rule).
2. **Before the send, then one `viola release` by the rig.** Keys as he answered, then the rig hands the wheel back
   with the human verb before the send. Cost: one more product act the rig makes as the human, not shown before. A
   release that does not take reads as a refused send, and the payload reading is lost.
3. **Leave C3 out.** `SF` types no key. C3 stays PROVISIONAL and is named leaving with its next reader.

## P4 round answers (appended; the relay `~/dev/projects/additional/viola-overseer/relay-stopfailure-forks.md`, snapped as inputs#I4)

The founder's own words, given in a dialog and read at 2026-11-19T16:55:47Z:
- **F1:** «Назвать чтение (Recommended)» — the capture keeps key names, the error word only when it is one of the
  CLI's 13 compiled words (else `other`), and the byte lengths of `error_details` and `last_assistant_message`,
  never a text; predicate 5 stands. His ratification of that one named reading, nothing wider; the wrap's answer
  card cites these words for `decision/live-rig-prompt-capture-closed` (security-plan) and
  `decision/claude-shapes-in-one-crate` (architecture).
- **F2:** «После конца хода (Recommended)» — the `x` and the DEL after `SF`'s StopFailure turn end has been read:
  the hint cells' attributes, one `x`, its cell's attributes, one DEL, then the rig's stop; attributes only, never
  text; no hint after an API-error turn reads `unread` with its reason, a reading, the carry kept open with its next
  reader named.

The overseer's technical word: the capture's closed set is a compiled list in the rig, and a rehearsal over the
fake agent proves both arms (a word of the 13 kept as itself, any other value kept as `other`) with a payload whose
word and texts are needles the kept file must lack; the host is the operator's own — nothing heavier than this
chunk's gates, read before each start, a neighbour named stops; on `yes` the phase ends, no wake or timer.
