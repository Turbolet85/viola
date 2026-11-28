import json
ts = "2026-11-28T15:09:55Z"
env = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 6 — The fleet console",
       "chunk": "2026-11-28-console-actions", "skill": "andromeda-implement", "step": "code"}
rs = [
    dict(env, kind="step", id=ts + "-a", outcome="ok",
         counts={"extra_reads": 11, "soft_exit": 0},
         consumed=[
             {"artifact": "plan", "quality": "thin",
              "note": "step 9's refused-action case presumes a dead instance keeps a row (a gone instance has none); its scroll case needs ~22 wrapped rows at the floor"},
             {"artifact": "research", "quality": "ok", "note": "every listed path existed; src/human.rs (the re-export) was the one file outside the lists"},
             {"artifact": "scope", "quality": "ok"},
             {"artifact": "source", "quality": "ok"}],
         produced=[{"artifact": "source", "signals": ["style-inferred", "guard-refused:2"],
                    "note": "the Bash guard refused a cd into a skill dir and a cat heredoc to a file; each re-issued once (Read tool, Write-tool script)"}],
         problem=[
             {"nature": "product-logic", "solution": "workaround",
              "note": "plan step 9 named pause on a dead instance for exit 21, but a gone instance draws no row to select: the case freezes the wrapper (SIGSTOP) until its beat is stale, which pause refuses instance-dead, exit 21"},
             {"nature": "product-logic", "solution": "workaround",
              "note": "a scroll at the 80x24 floor needs 22+ live wrapped rows; the scroll case runs at 80x5 over three booted wrappers, reading the shown/hidden counts each console-frame carries below the floor too"}]),
    dict(env, kind="friction", id=ts + "-b", type="input.plan-step-ambiguous",
         what="plan step 9's console_refused_action case assumed a dead instance keeps a selectable row; the board drops gone instances, so the case was rebuilt over a frozen (stale) wrapper",
         impact={"reformulations": 1}, artifacts=["plan"],
         evidence="tests/tui_console.rs::console_refused_action_logs_the_verbs_exit_code"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
