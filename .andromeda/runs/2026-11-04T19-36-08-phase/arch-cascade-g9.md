@carried n=1 label=architecture/crosscut-config fact=fct-k6htnrfj
why: any Unix-only test seam the parity suite needs must join this closed list, compiled only under the fake-agent feature, and the v1-40 witness's SGR bytes are decided only by the three colour variables it names.
quote: The only other variables any `viola` build reads are the two test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (exact value `1`, a forced fail-open `hook` panic), both compiled only under `cfg(feature = "fake-agent")` and absent from release builds, and, in every build, `NO_COLOR`, `TERM` and `COLORTERM`, read by viola's own colour layer (`src/human/style.rs`, `viola list`'s human mode only) to decide whether and how deep its SGR bytes are written (the founder's own word, chunk 2026-10-17-cli-output-discipline): they configure nothing, disable no control and widen no redaction.
@carried n=1 run=2 label=architecture fact=fct-dkudywpi
quote: There are no secrets in v1, and environment variables are not a configuration channel: a dependency or feature that reads one breaks this rule like a direct read does.
@carried n=2 label=architecture/crosscut-config fact=fct-k6htnrfj
why: the Unix R8 pass-list is a per-OS behaviour that a parity test must read on the Unix runners as Unix-only, without making it a Windows behaviour or an OS-only schema word.
quote: and `claude_env_keep` (the Unix R8 pass-list of `CLAUDE*` names, default empty)
@carried n=3 label=architecture/crosscut-capability-ledger fact=fct-wfge3rsz
why: the chunk reads v1-53's live Linux leg against what earlier chunks measured live on the Linux dev host, and this row records that measurement while the CI runners read the behaviour under the fake agent only.
quote: `viola revive`'s: `claude --resume <id>` reopens the same session and fires a SessionStart of source `resume` with that id (as measured on 2.1.287 on the Linux dev host at chunk 2026-10-10-viola-revive, `evidence/live-revive.md`, three live starts; the CI runners read it under the fake agent only).
@carried n=3 run=2 label=architecture fact=fct-igbvtfv6
quote: The first live run was on the Linux dev host, by the founder's ruling R-L1 (2026-10-07, relayed by the overseer), at chunk 2026-10-08-first-live-test-and-self-drive (`claude` 2.1.287 in foot 1.28.0 under Hyprland 0.56.2); Windows live behaviour stays proven on the CI runner under the fake agent until an interactive Windows host exists.
@carried n=4 label=architecture/crosscut-capability-ledger fact=fct-wfge3rsz
why: the statusline rows are relied on Unix only, so the relied set differs per OS, and a parity test stamping a test home for the fake agent must read its own OS's relied set and leave the Windows legs owed where they stand.
quote: Their rows landed at chunk 2026-10-13-owed-ledger-rows: `statusline-override`, `statusline-shell` and `statusline-rate-limits`, probed by Run B's probe `--settings` ([CLI Version Compatibility]), relied on Unix only and stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  21 pass  0 fail`, `evidence/live-round.md`), with the two `StatusLine` fixtures, so `budget-paused` now rests on a stamped row; the Windows legs stay owed to "Windows-only live measurements" (the operator's direction at the 2026-10-10-statusline-pass-through wrap). The dialog answer mapping (the S3/S7/S8 decision bodies) is gated by its own rows (`question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency`) inside the relied-set `cli_verified` (`LedgerRow::relied()`: 24 rows on Unix, 21 on Windows; [CLI Version Compatibility]).
@carried n=4 run=2 label=architecture fact=fct-fa2klbw5
quote: Nothing shells out through `sh`, `bash` or `cmd`, except `hook statusline` running the user's own statusline command (see Hook contract).
@carried n=5 label=architecture/crosscut-bounded-readers fact=fct-qip7sr3o
why: every product reader of external bytes that the openpty wrapper path adds must be bounded in its own statement or named in an EXCEPTIONS row of tests/contract_bounded_readers.rs.
quote: Every product reader of external bytes is bounded in the same statement (`take`) or named in an `EXCEPTIONS` row; none reads an external byte source to its end unbounded.
@carried n=5 run=2 label=architecture fact=fct-tpj3dxid
quote: Every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounded_readers.rs` enumerates the boundaries and censuses the product sources.
@carried n=5 run=3 label=architecture fact=fct-me2myqzf
quote: New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O.
@carried n=6 label=architecture/crosscut-sanitised-error-surfaces fact=fct-7ok4646z
why: the chunk re-asserts that every --json document carries the same closed words on every OS, and this rule keeps those words closed, with no path or upstream text on any OS.
quote: CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text; a peer's `message`, `data`, `refusal` or `detail` reaches no document or text but as a closed value, and a code word outside the closed list stays visible by design.
@carried n=7 label=architecture/crosscut-sanitised-error-surfaces fact=fct-7ok4646z
why: every exit-21 cause and exit-1 code the Unix suite reaches, socket-dir-refused included, must be the same row of tests/contract_error_bodies.rs on every OS, with no OS-only code word.
quote: Every exit-21 cause and exit-1 code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text.
@carried n=8 label=architecture/crosscut-sanitised-error-surfaces fact=fct-7ok4646z
why: the socket-directory refusal belongs to the Unix endpoint, so its human pair is a Unix surface that the parity suite must read as written.
quote: The `unable:` / `hint:` pair for a socket directory names only the parsed `ViolaName`, no path, pid or uid, is plain ASCII with `hint:` last and no SGR, its hint `send_hint`'s cause `socket-dir` in `src/human/hints.rs`.
@binding n=1
**Config management (closed variable list) ↔ Conventions (environment-variable naming) and the scope's test-seam boundary**: a Unix-only test seam the parity suite needs joins the closed list of variables a viola build reads, compiled only under `cfg(feature = "fake-agent")` and absent from release builds (architecture/crosscut-config fct-k6htnrfj). It must fit the naming rule's ratified test-seam exceptions (architecture fct-qqxg3ika), take a Decisions Log entry, and never force a refusing peer-euid decision.
@binding n=2
**Config management (colour variables) ↔ Standard Contracts (Session liveness) and Occupied Resources (environment variables)**: the v1-40 integration-tier witness reads SGR dim under the outer PTY. Its bytes are decided only by `NO_COLOR`, `TERM` and `COLORTERM`, which the test sets in the environment it starts `viola list` with, never viola (architecture/crosscut-config fct-k6htnrfj). The board width comes from `viola_pty::host_size()` (architecture fct-lak6evbp), and the row is stale only past a 5 s beat (architecture fct-arftgre4).
@binding n=3
**Capability ledger (live Linux records) ↔ Design Philosophy (first live run on Linux)**: v1-53's live Linux leg is read only against the live measurements on the Linux dev host recorded in the ledger rows (architecture/crosscut-capability-ledger fct-wfge3rsz) and the first live run (architecture fct-igbvtfv6). The Unix CI runners read the same behaviours under the fake agent only, and the chunk starts no live `claude`.
@binding n=4
**Capability ledger (Unix-only relied rows) ↔ Occupied Resources (per-session settings override) and Standard Contracts (hook statusline)**: the relied set is 24 rows on Unix and 21 on Windows (architecture/crosscut-capability-ledger fct-wfge3rsz), because the settings override is written only on Unix (architecture fct-klpb7auf) and the `/bin/sh -c` shell-out happens only on Unix (architecture fct-horlp4ay). A parity test reading them is a per-OS test that carries its one-OS reason where it stands, and no Windows pin moves.
@binding n=5
**Bounded readers ↔ Established Decisions ([PTY] seam, Tokio containment)**: the openpty backend's read path behind the `pty` seam (architecture fct-omwrdnlc) adds no product reader of external bytes unless it is bounded in its own statement or named in `EXCEPTIONS` (architecture/crosscut-bounded-readers fct-qip7sr3o). It uses std threads and blocking I/O in the sync `viola-pty` crate (architecture fct-me2myqzf).
@binding n=6
**Sanitised error surfaces ↔ Standard Contracts (CLI `--json` output) and the scope's one-schema rule**: the closed `UnreachableCause` words, `socket-dir-refused` among them (architecture fct-ksmkl4jd), and `viola revive`'s refusal codes (architecture fct-54jxzo44) read the same on every OS as rows of `tests/contract_error_bodies.rs` (architecture/crosscut-sanitised-error-surfaces fct-7ok4646z). A cause that one OS cannot reach still adds no OS-only word.
