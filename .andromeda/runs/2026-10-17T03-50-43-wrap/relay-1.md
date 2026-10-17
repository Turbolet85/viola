# P2 escalation dialog — the questions put and the answers given (verbatim)

Asked by the wrap at P2 (Escalate + HALT), 2026-10-17, over `fanout-results.md`. Answered through the dialog; each
answer's note names the Viola overseer as its author.

## Q1 — Boundary widening?
Question: "Q1 — Boundary widening? Every CLI verb (run and revive included) now reads VIOLA_DIR to choose its home. It
takes VIOLA_DIR only when VIOLA_NAME + VIOLA_DIR pass instance_of's shape check, else falls through to the default; the
home is then canonicalised once it exists. Only hook read VIOLA_DIR before. The plan leaned 'no widening' (the masters
already name it as target state), and the overseer's I2 answer, a delegate's, covers canonicalisation. Proposals:
security-plan 2, architecture 6 and 7, test-plan 9's order. How should the masters record it?"
Options: "Landed, not ratified (Recommended)" · "No widening — routine" · "Founder ratifies now".
Answer: "Landed, not ratified (Recommended)" — notes: "Overseer at 2026-10-17T04:05:25Z: a boundary widening is never
settled by a delegate and the founder is away overnight — record it as built, marked NOT ratified; the overseer carries
it to his next batch for his own word."

## Q2 — Who owns the revive --json gap?
Question: "Q2 — Who owns the revive --json gap? After a passed preflight, run::start can still refuse
(batch-script-child, pinned-hash-mismatch, squatted-name, claude not on PATH → internal-error). Each exits 1 with no
--json document, and none is among v1-39's five codes. v1-39 is 'implemented' with the gap surfaced beside it. What
does P7 do with v1-39, and who owns the gap?"
Options: "Refine + CARRY (Recommended)" · "Un-claim v1-39" · "Hold chunk pending".
Answer: "Un-claim v1-39" — notes: "Overseer, a ledger call at 2026-10-17T04:05:25Z: narrowing v1-39 to the five
preflight codes would read verified while every-exit-1-refusal stays unmet — the never-weaken rule. v1-39 returns to
the pool with a dated note naming the gap; pin the post-preflight documents as a CARRY on "Exit-cause code catalogue",
which then claims v1-39 with the gap closed."

## Q3 — revive's machine form
Question: "Q3 — revive's machine form. security-plan 1 and 3 escalate by detector severity. revive gains --json on both
arms (an output of closed codes only). Also, after a passed strict-modes check and under a VIOLA_NAME, revive --list
now opens and writes the caller's cli-<name>.ndjson (process-start / process-exit), as `viola list` already does; a
refused check still writes nothing. This retires obs-plan's exemption 6 (item kept as a retired marker so 7's number
holds). Apply as routine?"
Options: "Apply as routine (Recommended)" · "Treat as widening".
Answer: "Apply as routine (Recommended)" — notes: "Overseer, technical: the --list log write is the cli role's
existing rule after the check, as list and verify already do — no input class added."
