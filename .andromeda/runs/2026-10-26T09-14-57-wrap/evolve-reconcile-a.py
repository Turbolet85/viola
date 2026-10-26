import json
TS = "2026-10-26T10:58:26Z"
B = {"v": 1, "ts": TS, "version": "viola-0.1.0", "epoch": "Epoch 6 — Security hardening",
     "chunk": "2026-10-26-server-verification-before-any-frame", "skill": "andromeda-wrap-session", "step": "reconcile"}
R = ".andromeda/runs/2026-10-26T09-14-57-wrap/"
rs = [
    dict(B, kind="step", id=TS + "-a", outcome="halted-resolved",
         counts={"retries": 5, "dialogue_rounds": 2, "halted": 1},
         consumed=[
             {"artifact": "report", "quality": "thin", "note": "carried no endpoint name form for the impostor binders and no name for the handlers test's held-child variable: 2 proposals rejected for source"},
             {"artifact": "spec:architecture", "quality": "ok"}, {"artifact": "spec:security-plan", "quality": "ok"},
             {"artifact": "spec:design-system", "quality": "ok"}, {"artifact": "spec:layout-templates", "quality": "ok"},
             {"artifact": "spec:test-plan", "quality": "ok"}, {"artifact": "spec:obs-plan", "quality": "ok"},
             {"artifact": "spec:a11y-plan", "quality": "ok"}, {"artifact": "drift-base", "quality": "ok"},
             {"artifact": "wrap-playbook", "quality": "ok", "note": "no rule for a check-7 contradiction against a line the set retires; escalated (Q1)"}],
         produced=[
             {"artifact": "spec edits + sidecar appends", "signals": ["cascade-wide", "escalation-shaped", "hand-removal-f354a:1", "blank-before-f354e:7", "guard-refused:2"],
              "note": "F354 (a) first live use: apply-security-plan.p2 r2 read 'refused · a hand removal · input-validation.md:88–95', exactly those 8 lines removed by the Edit tool, detail.py check 0 defects after; F354 (e) first live use: splice.py append --blank-before on all 7 sidecars, each printing 'a blank line put before the block (line N is not blank)', last line the entry's Ref; guard: a dry-run's stdout to a file and a detail check piped into grep, each re-issued bare"},
             {"artifact": "fanout-results", "signals": ["rejected-for-source:2", "rejected-for-coordinate:0"]}],
         problem=[
             {"nature": "process", "solution": "workaround", "note": "the ruled re-spawn of apply-security-plan.p2 wrote '## Patterns (none)'; the step-2 sweep read the landed payload's patterns from its moved file .raw-apply-security-plan.p2-2.md"},
             {"nature": "process", "solution": "removed-cause", "note": "a scratch python append to cascade-dispositions.md raised on an illegal newline argument after open() had truncated the file; the file was rewritten whole with the Write tool from the window, the Leaves section added"}]),
    dict(B, kind="friction", id=TS + "-b", type="retry.detector-respawn",
         what="stage1: 6 of 47 category detectors failed check (YAML_FORM 4: multi-line values off the list shape; OWN_FILE 2: a section cited no origin line names); one re-spawn each, 6 of 6 pass",
         impact={"retries": 1}, artifacts=["fanout-security-plan--error-handling.md", "fanout-design-system--surface.md", "fanout-test-plan--e2e-test-strategy.md", "fanout-test-plan--integration-test-strategy.md", "fanout-test-plan--unit-test-strategy.md", "fanout-a11y-plan--cognitive-accessibility.md"],
         evidence=R + "fanout-2026-10-26-server-verification-before-any-frame.json"),
    dict(B, kind="friction", id=TS + "-c", type="retry.detector-respawn",
         what="strong: 5 of 8 strong readers failed SPLIT (block-scalar change: values, 15-131 lines off the list shape); one re-spawn each, 5 of 5 pass",
         impact={"retries": 1}, artifacts=["fanout-architecture--cascade.md", "fanout-security-plan--cascade.md", "fanout-obs-plan--cascade-g1.md", "fanout-obs-plan--cascade-g2.md", "fanout-a11y-plan--cascade.md"],
         evidence=R + "fanout-2026-10-26-server-verification-before-any-frame.json"),
    dict(B, kind="friction", id=TS + "-d", type="ambiguity.playbook-no-match",
         what="4 check-7 'contradiction' verdicts (security cascade 13, 22; test g1 23; obs g2 1) each stood against a standing line this chunk retires, rewritten by a companion proposal of the same set; no playbook rule; escalated, resolved apply-with-companion; the proposed rule declined at n=1 — recorded here for overseer1, who owns the letter (inputs#I9)",
         impact={"dialogue_rounds": 1}, artifacts=["fanout-results.md"], evidence=R + "fanout-results.md"),
    dict(B, kind="friction", id=TS + "-e", type="input.report-insufficient",
         what="two architecture registry proposals (the impostor binders as creators of the test-only viola-test-chan- endpoints; the handlers test's held-child env var) rested on facts the report did not carry (the endpoint name form, the variable's name) and were rejected for source; the registry gap stands for a later wrap's report",
         impact={"extra_reads": 0}, artifacts=["report.md", "fanout-architecture.md"], evidence=R + "fanout-results.md"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
