import os

RUN = ".andromeda/runs/2026-11-25T00-52-39Z-wrap/"
WORD = ("the operator's own answer, relayed verbatim by overseer1 (dialog 2026-11-24T18:28:03Z), inputs#I12, "
        "confirmed on this wrap's one answer card by the overseer, inputs#I13")

FORKS = {
    "W1": "W1 arm 1 («Никогда, отказ с подсказкой»)",
    "W2": "W2 + W3 («Строгая форма id, только CLI»)",
    "W4": "W4 («Принять обе»)",
    "W5": "W5 arm a («Как любой старт»)",
}

# doc -> list of (amd id, heading after '## ', section, rows, fork)
E = {
    "architecture": [
        ("amd-zqfvjh3b", "2026-11-24-viola-adopt — Established Decisions: 4 row(s)", "Established Decisions",
         "decision/adopt-refuses-live-session", "W1"),
        ("amd-oq6673zw", "2026-11-24-viola-adopt — Occupied Resources: 3 row(s)", "Occupied Resources",
         "decision/env-strip-names-only, decision/env-strip-reads-names-only", "W5"),
    ],
    "security-plan": [
        ("amd-cmh23teh", "2026-11-24-viola-adopt — Authentication & Authorization: 3 row(s)",
         "Authentication & Authorization", "decision/adopt-signals-no-foreign-process", "W1"),
        ("amd-6foqdnvf", "2026-11-24-viola-adopt — Input Validation: 1 row(s)", "Input Validation",
         "decision/session-id-argv-closed-shape (its except)", "W2"),
        ("amd-3oykhjjv", "2026-11-24-viola-adopt — Threat Model Summary: 1 row(s)", "Threat Model Summary",
         "decision/nothing-written-under-claude-home", "W5"),
        ("amd-5sxvogei", "2026-11-24-viola-adopt — Secret Management: 3 row(s) (1 of 2)", "Secret Management",
         "decision/credential-strip-by-name-only, decision/env-not-config-channel, decision/no-stored-secrets", "W5"),
        ("amd-lr66lc35", "2026-11-24-viola-adopt — Secret Management: 1 row(s) (2 of 2)", "Secret Management",
         "decision/r8-claude-env-strip", "W5"),
        ("amd-moaoiixf", "2026-11-24-viola-adopt — Data Protection: 3 row(s)", "Data Protection",
         "decision/start-writes-trust-record-flag-only, decision/no-writes-under-claude-home, "
         "decision/viola-writes-nothing-in-claude-dir", "W5"),
    ],
    "design-system": [
        ("amd-go4loibi", "2026-11-24-viola-adopt — Surface: cli: 4 row(s) (2 of 2)", "Surface: cli",
         "contract/viola-adopt-refusals/session-live (its fixed pair)", "W4"),
    ],
    "obs-plan": [
        ("amd-6pluk4bc", "2026-11-24-viola-adopt — 6. Log Coverage: 1 row(s)", "6. Log Coverage",
         "contract/start-trust-log/trust-record", "W5"),
        ("amd-2whzt7ul", "2026-11-24-viola-adopt — 1. Obs Scope Summary: 1 row(s)", "1. Obs Scope Summary",
         "decision/r8-strip-logged-as-names-only", "W5"),
        ("amd-m3r652po", "2026-11-24-viola-adopt — 3. Observability Harness Contract: 1 row(s)",
         "3. Observability Harness Contract", "decision/env-vars-not-configuration", "W5"),
    ],
}

for doc, items in E.items():
    out = []
    for amd, head, section, rows, fork in items:
        out.append("@amendment marker=2026-11-24-viola-adopt")
        out.append(f"title: {section} — {rows.split(',')[0].split(' ')[0]} ratified ({fork})")
        out.append(f"section: {section}")
        out.append(f"change: was {rows} PROVISIONAL, per «{head}» {amd}; now ratified as the operator's, "
                   f"his fork {FORKS[fork]}; the row text stands as landed.")
        out.append(f"why: {WORD}.")
        out.append(f"ref: {RUN}")
        out.append("")
    with open(os.path.join(RUN, f"{doc}-entries.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(out))
print("written")
