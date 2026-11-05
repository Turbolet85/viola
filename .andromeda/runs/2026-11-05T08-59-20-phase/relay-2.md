# P5 → P4 dialog — the owner of an external-decay red found at a baseline run

Asked at phase P5 step 1 of chunk 2026-11-05-linux-only-code-and-ci (a red on an existing property, its owner named
before approval; a contestable fork returns to P4's form), one question, technical.

## The question, as asked

P5's baseline of `cargo deny --format json check` reads red on the untouched tree: RUSTSEC-2026-0441 (`pin-utils` is
unmaintained, dated 2026-10-01), reached only through the pinned `portable-pty =0.8.1` → `nix 0.25.1`. It's a Linux path
and external decay, not this chunk's subject, and CI's supply-chain job will read it red on this chunk's push. The rules
forbid widening the one ignore to reach green. Who owns this red?

1. Version done-check (Recommended) — a CARRY on «Version done-check» owns RUSTSEC-2026-0441, with this run as its origin;
   this chunk's acceptance reads every CI job green except supply-chain's advisories family, recorded red on that one
   advisory, and the plan's deny entry asserts the same.
2. Fold into this chunk — move portable-pty off 0.8.1 to a release whose graph drops pin-utils, measured at implement.
3. Ignore with a reason — add RUSTSEC-2026-0441 to deny.toml's ignore list; a widening, the founder's live word.

## The answer, verbatim

Chosen: «Version done-check (Recommended)».

Notes: «The overseer, technical answer (founder-delegated), 2026-11-05 11:51Z. The founder's standing rule folds a red
found now into the NEXT chunk's scope as an [inferred] item, and «Version done-check» is the next chunk. Word the CARRY
so it names the remedy: the 0.9.0 Windows garbage-read bug that justified the =0.8.1 pin leaves with this chunk's Windows
removal, so done-check measures moving portable-pty off 0.8.1 to a release whose graph drops pin-utils. No ignore-list
widening.»

## Authority

delegate the Viola overseer, 2026-11-05 11:51Z (a technical answer, founder-delegated). No widening: the ignore list stays
the one entry.
