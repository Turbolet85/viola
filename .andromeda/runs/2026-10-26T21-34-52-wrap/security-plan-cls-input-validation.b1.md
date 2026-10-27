# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · yes · stale · send.rs runs check_instance first (F3) and the MCP tools run it before live_endpoint (F13), so the exit-13 refusal reading moves; the validator is unchanged
input-validation#1.2 · yes · stale · answer.rs runs check_instance first (F3), so the checked-first and exit-13 reading of viola answer moves; the validator is unchanged
input-validation#1.3 · no · viola-channel framing and the wrapper's method params lie in no reached file, and the report lists channel framing and the release refusal as unchanged
input-validation#1.4 · yes · stale · the hook event and dialog paths now check the instance before stdin, the diagnostics level and obs init (F11), so a refused instance writes no oversize or parse line
input-validation#1.5 · yes · stale · the MCP frame tools in reached tools.rs now run check_instance before live_endpoint (F13), so a refusal can return instance-unreachable ahead of the invalid-params shape
input-validation#1.6 · yes · contradict · the ratified interim gaps for hook, send, wait, last, answer, pause, release, the MCP tools and verify's stamps read are retired, so the exception clause no longer holds
input-validation#1.7 · yes · stale · the diagnostics_level read in reached src/obs.rs now follows the hook instance check (F11), so a refused instance reads no config.json and writes no line
input-validation#1.8 · no · the persistent-environment names row lies in src/run/env.rs, which no reached file touches, and the report lists that row as unchanged
input-validation#1.9 · yes · stale · the seam sits in reached src/cmd/run.rs (fn hold_pump_start), whose start now runs check_home_tree before its log and reads the collision check from the replay (F1, F10)
input-validation#1.10 · yes · stale · the seam is called from hook() in reached src/cmd/hook.rs, now behind the instance check before obs init (F11) with diagnostics opens checked first (F7)
input-validation#1.11 · no · the colour facts lie in src/human/style.rs and src/cmd/list.rs, neither reached, and the report lists the colour facts as unchanged
input-validation#1.12 · yes · contradict · live_endpoint, hook connect, statusline serve and the collision check moved onto read_snapshot_or_replay (F10), so the stays-on-read_snapshot rule and the ratified gaps no longer hold, and update_stamps checks before its create and read (F8)
input-validation#1.13 · no · tailing lies in viola-state tail code that no reached file names, and the session names no tail path or name rule change
input-validation#1.14 · yes · stale · revive's preflight in reached src/cmd/revive.rs now takes one snapshot read, and verify's start in reached src/cmd/verify.rs runs check_home_tree before its cli log (F2), so the fail-open reading is not shown to hold
input-validation#1.15 · no · the tee and feed thread lie in src/run/gate.rs and the screen model in viola-agent-claude, neither reached, and viola-pty and the dependency lock are unchanged
input-validation#1.16 · yes · stale · revive.rs (reached) now shares one preflight snapshot read between the collision check and the recorded-directory read, so its reading count and the cwd read move
input-validation#1.17 · yes · stale · revive.rs and verify.rs (reached) changed at their starts (F1, F2) and no reading there is shown to hold, though the resume_args argv shape is unchanged
input-validation#1.18 · yes · stale · run's start now runs check_home_tree before its wrapper log (F1), and the statusline source read sits in reached src/cmd/run.rs, so its place in the start order is not shown to hold
input-validation#1.19 · yes · stale · statusline.rs (reached) now reads the replay in serve (F10, a replay carries no statusline_command) and the instance check moved from statusline.rs to hook.rs
