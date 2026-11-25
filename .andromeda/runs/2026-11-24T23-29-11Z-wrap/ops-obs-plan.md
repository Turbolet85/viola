@row_set n=1 row=decision/adopt-start-logs-as-run cell=except seen=1 detector=none
value: The detail word not-resumed, on adopt's own self process-exit exit 1 when the resumed child ends before its session starts.
basis: fct-x4brbvla
because: adopt's start now writes process-exit{subject:"self", exit_code:1, detail:"not-resumed"} of its own, an exception the operator's answer adds by naming the case, so the row's «no detail word of its own» must carry it as its exception.
@row_set n=2 row=contract/process-exit-detail/exit-1-adopt cell=promise seen=1 fact=F1 detector=none
value: viola adopt's preflight ends exit 1 with one process-exit detail session-live and no child started; its strict-modes-failed refusal comes before any log and writes no line; a resumed child ending before its session starts gives exit 1 detail not-resumed after the child's process-exit.
basis: fct-x4brbvla
because: viola adopt gains a third exit-1 cause, the self process-exit detail not-resumed written after the claude-child process-exit, which the row naming adopt's exit-1 details must now list.
@row_none n=1 fact=F2 type=4
relied: Adopt's not-resumed check counts only a real snapshot with no agent session id as not resumed; a replayed, absent or unreadable snapshot reads resumed and the start ends exit 0.
