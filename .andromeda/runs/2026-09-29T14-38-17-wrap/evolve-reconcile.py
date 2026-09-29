import json

TS = "2026-09-29T14:49:56Z"
EPOCH = "Epoch 2b — Windows slice I b: events and ledger"
BASE = {"v": 1, "ts": TS, "version": "viola-0.1.0", "epoch": EPOCH,
        "chunk": "2026-09-29-fake-agent-drift-contract", "skill": "andromeda-wrap-session", "step": "reconcile"}
BS = chr(92)

step = dict(BASE, kind="step", id=TS + "-a", outcome="ok",
            counts={"retries": 2, "dialogue_rounds": 0, "halted": 0},
            consumed=[
                {"artifact": "report", "quality": "ok", "note": "every detector ran on it alone; the plan's expected amendments 2, 3 (three sites) and 5 were carried by the report and raised by the orchestrator at check 5 (a detector blind class: existing elements gaining a treatment), not a report gap"},
                {"artifact": "spec:architecture", "quality": "ok"},
                {"artifact": "spec:security-plan", "quality": "ok"},
                {"artifact": "spec:design-system", "quality": "ok"},
                {"artifact": "spec:layout-templates", "quality": "ok"},
                {"artifact": "spec:test-plan", "quality": "ok"},
                {"artifact": "spec:obs-plan", "quality": "ok"},
                {"artifact": "spec:a11y-plan", "quality": "ok"},
                {"artifact": "drift-base", "quality": "ok"},
                {"artifact": "wrap-playbook", "quality": "ok", "note": "Boundary widening + what-ratifies-it governed A1-A3/R1-R2; the founder's live answer on disk (given after the widening was shown) resolved them without a new halt"},
            ],
            produced=[
                {"artifact": "spec:architecture", "signals": ["escalation-shaped"], "note": "3 sites"},
                {"artifact": "spec:security-plan", "signals": ["escalation-shaped"], "note": "Code Patterns + Decisions Log entry"},
                {"artifact": "spec:test-plan", "signals": ["cascade-wide"], "note": "9 body edits + a Decisions Log entry; proposals 3 routine, orchestrator-raised 7; leaves re-derived: rules/events.md, rules/verification-harness.md, docs/services/viola-agent-claude.md, docs/tests-summary.md, docs/security-summary.md"},
                {"artifact": "sidecar:architecture", "signals": []},
                {"artifact": "sidecar:security-plan", "signals": []},
                {"artifact": "sidecar:test-plan", "signals": [], "note": "2 entries"},
            ],
            problem=None)

friction = dict(BASE, kind="friction", id=TS + "-b", type="contract.proposal-format",
                what="the architecture doc-agent's return quoted the spec's harness-prefix tag literals and the harness flagged it as a harness-envelope-tag, neutralising every quoted tag's '<' to '<" + BS + "' inside the YAML change lines; the applied text was re-derived from the report instead of the change lines, so nothing mangled landed",
                impact={"retries": 0},
                artifacts=[".andromeda/architecture.md"],
                evidence=".andromeda/runs/2026-09-29T14-38-17-wrap/fanout-results.md")

print("\n".join(json.dumps(r, ensure_ascii=False) for r in (step, friction)))
