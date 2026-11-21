# Founder forks — «Cross-session prefix ledger row» (`2026-11-21-cross-session-prefix-ledger-row`), Revision 1

Written at P4 of the plan revision after the red `XS` (run `2026-11-21T14-00-53Z-phase`), under `inputs#I8` item 6: a
master row the uds form reaches that §4 of the take-up's block (`.andromeda/runs/2026-11-21T11-32-16Z-phase/founder-forks.md`)
did not name is a fork here. No start was made; nothing of the operator's real `~/.claude*` or `~/.viola` was read.

## X1 — where verify gets the runtime dir's name

**What was read (no start).** 2.1.328 names its own inbox socket `(XDG_RUNTIME_DIR, else the OS temp dir)/cc-socks/<its
pid>.sock`, and only when that path is over 103 bytes `/tmp/cc-socks-<uid>/<pid>.sock` (the installed bundle read as
strings, offset ≈341.971 M). The send tool accepts a `uds:` path whose dir is `/run/user/<uid>/cc-socks`,
`/tmp/cc-socks[-<uid>]`, or the sender's own socket dir (offset ≈308.894 M).

**What verify does today.** It reads no `XDG_RUNTIME_DIR`. Run B inherits it because the R8 strip removes only `CLAUDE*`
names (`plan_strip`, `crates/viola-agent-claude/src/lib.rs:76-96`), and the rig's `VR` arm hands verify the real one
(`crates/viola-e2e/src/rig/isolate.rs:31`). So «the env name verify already hands its children» (`inputs#I8` item 3) is
an inheritance, not a read. Composing the address from the runtime dir's NAME is a new environment read in a product
build.

**The rows it reaches, none named in §4:** architecture `decision/env-var-viola-prefix` and
`decision/no-environment-configuration` (each lists `XDG_RUNTIME_DIR` «for the socket dir» only); security-plan
`decision/no-stored-secrets` (the same exception) and `decision/new-env-read-needs-log-entry` (no environment read is
added without its Decisions Log entry). All four were picked in the take-up's union (`arch-rows.md:101, 256`,
`security-rows.md:143, 156`).

Options, in order:
- **(a) verify reads `XDG_RUNTIME_DIR`'s value once (Recommended).** It reads the value at `measure`'s start, which is
  the value Run B inherits. It holds the value in memory and uses it only to compose Run B's address
  (`uds:<value>/cc-socks/<pid>.sock`). The value reaches only the sender's argv, and through it the sender's prompt to
  the model. It never reaches a log line, the stamp, a fixture or an error. It configures nothing, and its absence
  disables no control: with no value, a relative one, a refused byte, or a composed path over 103 bytes, verify spawns
  no sender, and the row reads `fail` (`not-run`), never a pass.
  *Why it wins:* it is the CLI's own source for the name, so the address follows the CLI wherever the user's session
  runs. It also matches his words «the runtime dir's name».
  *Cost:* the four rows each add the reading beside the channel's (a named reading added, never folded into a class),
  and the Decisions Log entry the fourth row asks for: his.
- **(b) verify composes from its own euid: `/run/user/<euid>/cc-socks/<pid>.sock`, no env read.** No env row moves.
  The address matches the CLI only where `XDG_RUNTIME_DIR` is `/run/user/<uid>` (this host, and the rig's `VR` arm).
  Anywhere else the start reads red (the connect finds no socket, `send` · `refused`), never a false pass. It still
  writes a `/run/user` convention into verify as a compiled literal, which is a reading of its own.

**Held:** the plan writes (a) at step 12, behind this fork. The word at P5 answers X1 beside the plan. An answer of
(b) re-plans steps 12 and 14 and the expected amendments before `XS-2`. Neither is a widening beyond the composed
address he ratified (`inputs#I8` item 1).

## Read as not reached by the uds form (no fork)
- security-plan `decision/per-user-socket-directory` and test-plan `contract/channel-endpoint/socket-path`: viola's own
  socket dir; verify neither binds nor reads under `cc-socks/`.
- obs-plan: no new event, subject or field; the address goes into no line.
- architecture §Occupied Resources: `cc-socks/` is the CLI's, not a resource viola holds.

## The answer — appended at P5, 2026-11-21 (`inputs#I9`)
X1: **(a)**. The operator's own word, in his dialog in the overseer's session, read at 2026-11-21T17:27:44Z and relayed
verbatim: «Читать XDG_RUNTIME_DIR (Recommended)». Verify reads the value once, in memory, for the address alone. He was
shown that it reaches the sender argv, and through the prompt the model, and never a log, stamp or fixture; he was also
shown the four master rows and the Decisions Log entry. Step 12 is no longer held.
