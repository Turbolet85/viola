# Curation — 2026-10-04-confirmed-send-with-cl-1-records

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "under `* text=auto` a CRLF seed is stored LF; a byte-exact corpus dir needs its own `binary` line" (confidence 0.8)
                                              + verification-harness.md (extended): "`test(/…/)` matches names only; `binary(<file stem>)` selects a test file" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "A newly served channel method breaks the tests that pinned it unserved" (confidence 0.8)
  Filters: 1 dup (the receipt-after-reply wait — covered by testing.md 2026-09-24 "wait on the exact line a test asserts") · 0 task-specific · 0 conflict · 0 deferred
  No-other-home: all three (each 0.6 from measurement + detail, +0.2: no master, route annotation or ledger note carries the general fact)
  Extended: T2/verification-harness.md: "2026-09-27: A nextest `test(=name)` filter needs the test's full path…" + "`binary()` selects a file"
  Recurrence-despite-learning: host-win32.md 2026-09-28 (the Bash guard refuses a heredoc redirected to a file) — hit twice this session (`cat >> fuzz/Cargo.toml <<EOF`, `cat > scratchpad/s4.py <<PY`) → handoff Deferred learnings
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B

## Proofs
- testing.md CRLF seed — Proof: the operator pass's pre-CI commit `a6abfc2` stored `fuzz/corpus/paste_text/multi-line` as 33 bytes LF (`git show HEAD:… | od -c`); `git add` printed "CRLF will be replaced by LF"; folded in `306c4ae` (`.gitattributes` `fuzz/corpus/paste_text/** binary`, the blob re-read as 34 bytes with `\r\n`; `git ls-files --eol` `attr/-text`).
- verification-harness.md binary() — Proof: `run --integration --filter 'test(/channel_endpoint/)'` selected 2 of `tests/channel_endpoint.rs`'s 5 tests (the debug-level test's name lacks the token); `--filter 'binary(channel_endpoint)'` ran all 5.
- session-learnings.md unserved-method pin — Proof: implement's first full `bash scripts/agent-run.sh run` red, `channel_debug_level_keeps_request_content_out_of_the_role_files` asserting `reply["error"]["code"] == -32601` for `send`, which plan.md listed as "expect no change".
