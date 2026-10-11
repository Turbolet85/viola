import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-11T00:57:13Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-session-links",
    "skill": "andromeda-wrap-session",
    "step": "curation",
}
RUN = ".andromeda/runs/2026-10-11T00-33-08-wrap/"


def rec(letter, kind, **fields):
    out = dict(BASE)
    out["kind"] = kind
    out["id"] = TS + "-" + letter
    out.update(fields)
    return out


def recurrence(letter, what, artifact):
    return rec(
        letter,
        "friction",
        type="recall.corpus-recurrence",
        what=what,
        impact={"iterations": 0},
        artifacts=[artifact],
        evidence=RUN + "curation.md",
    )


records = [
    rec(
        "a",
        "step",
        outcome="ok",
        counts={"dialogue_rounds": 0},
        consumed=[
            {
                "artifact": "conversation",
                "quality": "thin",
                "note": "the conversation of implement and of this wrap's Phase 1 was gone at the resume; its eleven "
                "candidates were read from resume-point.md, each with its evidence",
            },
            {"artifact": "report", "quality": "ok"},
        ],
        produced=[
            {
                "artifact": "conversation",
                "signals": ["exact-hit:3"],
                "note": "three candidates totalled exactly 0.6: two were carried by the no-other-home signal and "
                "applied as Tier 2 entries, one was rejected by the lean default because the route carries its fact",
            }
        ],
        problem=None,
    ),
    rec(
        "b",
        "friction",
        type="ambiguity.filter-borderline",
        what="the no-other-home signal's applicability was open for the schema-check candidate: docs/commands.md "
        "already says the local G2, G4 and scan need kept homes, and a docs leaf is not among the homes the filter "
        "lists; the signal was applied",
        impact={"extra_reads": 1},
        artifacts=[".claude/rules/verification-harness.md"],
        evidence=RUN + "curation.md",
    ),
    recurrence(
        "c",
        "host-linux.md Transports: one inline python heredoc for a source edit at implement, and inline python "
        "heredocs for multi-line reads in the wrap's second session, past the entry that sends them to a script file",
        ".claude/rules/host-linux.md",
    ),
    recurrence(
        "d",
        "a time typed from memory into a record ahead of the clock reading, caught and replaced before it was pasted; "
        "the deferred-learnings list already carries this class",
        ".claude/session-handoff.md",
    ),
    recurrence(
        "e",
        "host-linux.md Paths: a cd at the head of a compound in the session's first Bash call, past the entry that "
        "allows it only in a subshell",
        ".claude/rules/host-linux.md",
    ),
    recurrence(
        "f",
        "host-linux.md Exit codes: a verdict tool's output read through a line filter once, past the entry that says "
        "to read it bare",
        ".claude/rules/host-linux.md",
    ),
    recurrence(
        "g",
        "multi-KB master lines read through a scratch script once more; the deferred-learnings list already carries "
        "this class",
        ".claude/session-handoff.md",
    ),
]

print("\n".join(json.dumps(r, ensure_ascii=False) for r in records))
