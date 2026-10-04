# Print-mode dialog probe — the plan's STOP condition, measured (2026-10-04, this host, Linux)

The plan's step 5 / step 7 and its Constraints rest on one premise: a print-mode (`claude -p`) turn of the installed CLI can
raise the dialog-tier hooks — PreToolUse `AskUserQuestion` / `ExitPlanMode` and PermissionRequest — so `viola verify` can
measure S3 / S7 / S8 / dialog concurrency at capture grain and `--record` can write the tool-bearing fixtures. The
Constraints say: "if print mode cannot raise a dialog hook or exercise concurrency, implement STOPS and surfaces, never
improvises a PTY probe or an invented post-condition". Before any of steps 3-18 was written, that premise was measured
with a throwaway probe outside the repository.

## The probe
- CLI: `claude` 2.1.287 (`claude --version` → `2.1.287 (Claude Code)`), the mise install on PATH.
- Environment: every inherited `CLAUDE*` variable removed (`env -u …`, the ten this session carried), as `viola verify`'s
  R8 strip does.
- Plugin (`--plugin-dir`, scratch dir outside the repo): exec-form hooks on `PreToolUse` (matcher
  `AskUserQuestion|ExitPlanMode`), `PermissionRequest` (no matcher) and `Stop`, each running a scratch capture script
  that files the stdin payload with its arrival time, sleeps 3 s (so two overlapping dialog hooks would show as
  overlapping intervals — the concurrency witness), then exits 0 with no body.
- Every run: `--model haiku --no-session-persistence`, a fresh empty working directory.

## Runs and readings
1. `-p "<call AskUserQuestion …; then ExitPlanMode …; then run: touch probe.txt>"` (default permission mode) → exit 0.
   Model: "The dialog tools you mentioned (AskUserQuestion, ExitPlanMode) aren't available in this environment". The
   command ran (`probe.txt` created) with NO PermissionRequest hook. Captures: `Stop` only.
2. The same ask with `--permission-mode plan` (ExitPlanMode's own mode), command `mkdir probedir` → exit 0. Model:
   "`ExitPlanMode` and `AskUserQuestion` aren't available in this session. They aren't in my tool list, and a tool search
   for both returned nothing." No command ran. Captures: `Stop` only.
3. The tools named explicitly, `--tools "AskUserQuestion,ExitPlanMode,Bash"`, command `curl -sI https://example.com` →
   exit 0. Model: "Looking at my available tools, I don't see `AskUserQuestion` or `ExitPlanMode`". No command ran.
   Captures: `Stop` only.

## Verdict
- On 2.1.287, print mode exposes neither `AskUserQuestion` nor `ExitPlanMode`: no flag tried (default, `--permission-mode
  plan`, `--tools`) put them in the tool list. A print-mode probe cannot raise a `question` or `plan` dialog hook, so it
  cannot measure S3 / S7 / S8 nor record `PreToolUse.ask-user-question.json` / `PreToolUse.exit-plan-mode.json`.
- No PermissionRequest hook fired in any run (run 1's command ran unprompted; runs 2-3 ran none), so no
  `PermissionRequest.<tool>.json` was captured either.
- With no dialog hook raised, dialog concurrency cannot be exercised.
- `--help` lists `--permission-prompts <target>` ("who answers permission prompts … non-interactive mode"); it was not
  tried — it routes the permission prompt to a tool or to none, not through the dialog-tier hooks, and trying it would
  be designing a new probe (the plan's rejected improvisation).
- The plan's STOP fires: implement stopped after steps 1-2 (the two CI reds), wrote none of steps 3-18, and fired no
  recording leg.
