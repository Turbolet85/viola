import json
ts = "2026-10-04T17:16:27Z"
E = "Epoch 3 — Windows slice II: driving verbs and live proof"
base = {"v": 1, "ts": ts, "version": "viola-0.1.0", "epoch": E,
        "chunk": "2026-10-04-dialog-answers-by-dialog-id", "skill": "wrap-session", "step": "reconcile"}
rs = []
rs.append(dict(base, id=ts + "-a", kind="step", outcome="halted-resolved",
    counts={"retries": 2, "dialogue_rounds": 1, "halted": 1},
    consumed=[
        {"artifact": "report", "quality": "thin", "note": "carried every Change the 68 proposals rested on; did not carry the perf suite's green count (6->7) or name RECORDED_CLI_VERSION, so the orchestrator re-read perf.rs:357 and the plan"},
        {"artifact": "spec:architecture", "quality": "ok"}, {"artifact": "spec:security-plan", "quality": "ok"},
        {"artifact": "spec:design-system", "quality": "ok"}, {"artifact": "spec:layout-templates", "quality": "ok"},
        {"artifact": "spec:test-plan", "quality": "ok"}, {"artifact": "spec:obs-plan", "quality": "ok"},
        {"artifact": "spec:a11y-plan", "quality": "ok"},
        {"artifact": "drift-base", "quality": "ok"},
        {"artifact": "wrap-playbook", "quality": "ok", "note": "Boundary widening + what-ratifies-it governed E1/E2; Registry over-reach rejected A22"}],
    produced=[
        {"artifact": "spec:architecture", "signals": ["cascade-wide", "escalation-shaped"], "note": "28 proposals + 1 raise; 3 keyed-contract files edited, registry check clean"},
        {"artifact": "spec:security-plan", "signals": ["escalation-shaped"], "note": "13 proposals, the sixth dated gap and R2 recorded as the founder's"},
        {"artifact": "spec:test-plan", "signals": ["cascade-wide"], "note": "19 proposals + 1 raise + 1 sweep-found duplicate (:408)"},
        {"artifact": "spec:obs-plan", "signals": [], "note": "6 proposals + 1 raise (:737 version)"},
        {"artifact": "spec:layout-templates", "signals": []}, {"artifact": "spec:a11y-plan", "signals": []},
        {"artifact": "sidecar:architecture", "signals": []}, {"artifact": "sidecar:security-plan", "signals": []},
        {"artifact": "sidecar:test-plan", "signals": []}, {"artifact": "sidecar:obs-plan", "signals": []},
        {"artifact": "sidecar:layout-templates", "signals": []}, {"artifact": "sidecar:a11y-plan", "signals": []},
        {"artifact": "cascade-sweep", "signals": ["control-fired-all-20"], "note": "every pattern's known-positive control fired; 6 zero-row patterns"}],
    problem=[
        {"nature": "environment", "solution": "workaround", "note": "a cat heredoc appending to fanout-results.md was blocked by the project's Bash guard; the text went in through the Edit tool"},
        {"nature": "environment", "solution": "workaround", "note": "grep -o with a 250-char context window failed on the host grep (ugrep complexity limit); used a scratchpad python context printer"},
        {"nature": "process", "solution": "workaround", "note": "three counts the report does not carry (perf suite 7 passed, PERF_ROWS re-export, MUTANTS_PROGRESS arms) confirmed by reading crates/viola-e2e source before applying"}]))
rs.append(dict(base, id=ts + "-b", kind="friction", type="contract.false-positive-proposal",
    what="D-arch-decisions proposed insta for arch Stack, which carries no test-library row; rejected under Registry over-reach",
    impact={"iterations": 1}, artifacts=["architecture.md:38"], evidence=".andromeda/runs/2026-10-04T16-53-44-wrap/fanout-results.md"))
rs.append(dict(base, id=ts + "-c", kind="friction", type="contract.cascade-miss",
    what="two pre-pass restatements no detector proposed: obs-plan:737 (CI verify at 2.1.283) found by the orchestrator's expected-amendment grep, test-plan:408 (contract suite vs verify fixtures only) found by the sweep",
    impact={"extra_reads": 2}, artifacts=["obs-plan.md:737", "test-plan.md:408"], evidence=".andromeda/runs/2026-10-04T16-53-44-wrap/cascade-dispositions.md"))
rs.append(dict(base, id=ts + "-d", kind="friction", type="input.report-insufficient",
    what="report Counts moved perf rows 4->5 but not the perf suite's green count (6 passed -> 7) the test-plan key states; re-derived from perf.rs:357",
    impact={"extra_reads": 1}, artifacts=["5-command-implementation.md:51"]))
rs.append(dict(base, id=ts + "-e", kind="friction", type="contract.in-pass-correction",
    what="cascade-patterns.toml first carried two ids over 16 chars (cascade.py refused, exit 2); test-plan:989 first said the RUNNER_TEMP case 'stays owed' with no owner, a re-read added ':111'",
    impact={"retries": 1}, artifacts=["cascade-patterns.toml", "test-plan.md:989"]))
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
