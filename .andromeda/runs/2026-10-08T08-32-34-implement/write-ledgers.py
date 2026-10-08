"""The records of the live work of this run (plan.md steps 6 to 10), written from the driver's journal and the
readings taken in this session: codes, counts and booleans only.

  write-ledgers.py <evidence dir>

Creates live-run.ndjson (refused when it exists); appends to live-sessions.ndjson and live-readings.ndjson.
"""
import json
import os
import sys

D = "2026-10-08T"
SHA = "2bf1b8ab19e16c8e"


def dump(rows):
    return "".join(json.dumps(r, separators=(",", ":"), ensure_ascii=False) + "\n" for r in rows)


def step(i, at, **kw):
    r = {"kind": "step", "id": i, "session": "start-7", "at": D + at + "Z"}
    r.update(kw)
    return r


def dialog(i, at, did, kind, **kw):
    r = step(
        i,
        at,
        raised=True,
        kind_raised=kind,
        dialog_id=did,
        answer_exit=0,
        decision_emitted=True,
        deadline_hit=False,
        wheel_records=0,
        keys_typed=0,
        turn_ended=True,
    )
    r.update(kw)
    return r


RUN = [
    step(
        "snapshot", "08:39:39", cli_version="2.1.287", cli_verified=True, build="release-check", build_sha256=SHA,
        instance="livetest2", cols=210, rows=45, font_size=8,
        start_records=["wheel:start", "budget-gate", "session-start:startup"],
        seconds_after_session_start=5, wheel_at_snapshot="driver", wheel_human_records=0, modal=False,
    ),
    step(
        "skill-sent", "08:39:51.637", exit=0, confirmed=True, result="read-back", cursor=454,
        records=["send-issued", "prompt-submitted", "send-confirmed"],
    ),
    step(
        "permission-1", "08:39:54.990", during="skill-sent", raised=True, kind_raised="permission", dialog_id=1,
        tool="Bash", rule="auto-skill-script-or-read", behavior="allow", answer_exit=0, decision_emitted=True,
        deadline_hit=False, answered_ms_after_raise=3,
    ),
    step("turn-ended", "08:41:55.391", exit=0, woke="turn-ended", cursor=6879, records=["turn-ended"], seconds=124),
    step(
        "last-read", "08:42:08.040", exit=0, non_empty=True, holds_closing_question=True, text_bytes=2198, words=324,
    ),
    step(
        "review-answered", "08:42:08.066", exit=0, confirmed=True, result="read-back", cursor=6879, turn_ended=True,
        last_is_the_word=True, records=["send-issued", "prompt-submitted", "send-confirmed", "turn-ended"],
    ),
    step(
        "clear", "08:42:16.363", exit=0, confirmed=True, result="read-back", cursor=7494, new_session=True,
        session_start_cause="clear", new_agent_session_id=True, prompt_submitted_records=0,
        records=["send-issued", "session-end", "session-start", "send-confirmed"],
    ),
    step(
        "skill-2", "08:42:23.647", exit=0, confirmed=True, result="read-back", cursor=8067, turn_ended=True,
        dialogs_by_themselves=3, seconds=203,
    ),
    step(
        "permission-2", "08:42:28.521", during="skill-2", raised=True, kind_raised="permission", dialog_id=2,
        tool="Bash", rule="auto-deny", behavior="deny", answer_exit=0, decision_emitted=True, deadline_hit=False,
    ),
    step(
        "permission-3", "08:42:34.401", during="skill-2", raised=True, kind_raised="permission", dialog_id=3,
        tool="Bash", rule="auto-deny", behavior="deny", answer_exit=0, decision_emitted=True, deadline_hit=False,
    ),
    step(
        "permission-4", "08:42:42.010", during="skill-2", raised=True, kind_raised="permission", dialog_id=4,
        tool="Bash", rule="auto-deny", behavior="deny", answer_exit=0, decision_emitted=True, deadline_hit=False,
    ),
    step(
        "skill-2-review", "08:45:46.283", exit=0, confirmed=True, result="read-back", cursor=17394, turn_ended=True,
        last_is_the_word=True,
    ),
    dialog(
        "dialog-question", "08:45:53.531", 5, "question", send_exit=0, send_confirmed=True, asked_as_planned=True,
        raised_s_after_send=1.6, last_is_the_chosen_word=True,
    ),
    dialog(
        "dialog-permission", "08:45:56.353", 6, "permission", send_exit=0, send_confirmed=True, behavior="deny",
        tool="Bash", command_is_true=True, raised_s_after_send=1.6,
    ),
    dialog(
        "dialog-plan", "08:46:37.377", 7, "plan", send_exit=0, send_confirmed=True, behavior="approve",
        raised_s_after_send=10.4, last_is_the_word=True, plan_file_beside_home=0, plan_file_under_cli_default_dir=1,
    ),
    step(
        "takeover", "08:47:04.983", wheel_after_focus=0, compositor="own", guard_before=True, guard_at_key=True,
        key_bytes=1, holder="human", cause="human-input", new_wheel_records=1, snapshot_wheel="human", send_exit=10,
        refusal="human-typing", hint_lines=1, hint_is_last_line=True, hint_names_release=False, send_issued=False,
        records=["wheel", "send-refused"], release_run=False, pause_run=False,
    ),
    step(
        "close", "08:47:28.504", child_fd_count=23, wrapper_gone_s=0.03, child_gone_s=0.36, session_end_line_s=0.03,
        session_end_before_child_gone=True, records=["session-end"], windows_on_own_instance=0,
    ),
]

SESSIONS = [
    {
        "kind": "census", "when": "after", "of": "readings", "at": D + "08:49:22Z", "wrapper_gone": True,
        "child_gone": True, "windows_on_own_instance": 0, "probe_cwd_processes": 0, "viola_verify_dirs_at_root": 11,
        "own_left": 0,
    },
    {
        "kind": "lock-read", "when": "before-step-5c", "at": D + "08:36:55Z", "desktop_lock_shell": True,
        "desktop_lock_compositor": True, "shell_relaunched": False, "own_compositor": "up, 0 windows",
    },
    {
        "kind": "lock-read", "when": "before-start-7", "at": D + "08:39:33Z", "desktop_lock_shell": True,
        "desktop_lock_compositor": True, "shell_relaunched": False, "own_compositor": "up, 0 windows",
    },
    {
        "kind": "lock-read", "when": "before-start-8", "at": D + "08:48:14Z", "desktop_lock_shell": True,
        "desktop_lock_compositor": True, "shell_relaunched": False, "own_compositor": "up, 0 windows",
    },
    {
        "kind": "compositor", "event": "end", "n": 1, "at": D + "08:49:30Z", "own_instance": True,
        "windows_before_end": 0, "ended_by": "TERM to its pid", "gone": True, "up_s": 4998,
        "desktop_lock_shell": True, "desktop_lock_compositor": True, "shell_first_answer": "true",
        "shell_relaunched": False, "first_reading_s": 2.1, "second_reading_s": 15.1, "own_left": 0,
    },
]


def reading(i, session, **kw):
    r = {"kind": "reading", "id": i, "session": session, "outcome": "measured"}
    r.update(kw)
    return r


def fidelity(i, **kw):
    r = {"kind": "fidelity", "id": i, "session": "start-7", "outcome": "measured"}
    r.update(kw)
    return r


READINGS = [
    reading(
        "send-under-hint", "start-7", at=D + "08:46:13.188Z", exit=0, confirmed=True, text_bytes=32,
        wheel_records=0, long_paste_text_bytes=2564, long_paste_exit=0, long_paste_confirmed=True,
        send_began_ms_after_paste_issued=4248, send_confirmed_duration_ms=4081, bound_ms=8500, inside_bound=True,
        issued_ms_after_paste_issued=8319, turn_ended=True, last_is_the_word=True,
    ),
    reading(
        "ends-in-newlines", "start-7", at=D + "08:46:27.400Z", exit=0, confirmed=True, text_bytes=31,
        sent_bytes=33, trailing_lf=2, send_refused_records=0, wheel_records=0, prompt_origin="driver",
        turn_ended=True, last_is_the_word=True,
    ),
    reading(
        "clear-then-newline", "start-7", at=D + "08:46:28.384Z", exit=0, confirmed=True, text_bytes=6, sent_bytes=7,
        trailing_lf=1, new_session=True, session_start_cause="clear", new_agent_session_id=True,
        prompt_submitted_records=0, wheel_records=0,
    ),
    reading(
        "only-newlines", "start-8", at=D + "08:48:31.735Z", exit=13, confirmed=False, text_bytes=0, sent_bytes=2,
        refusal="not-delivered", detail="no-prompt-submitted", seconds=10.0, hint_lines=1,
        records=["send-issued", "send-refused"], prompt_submitted_records=0, wheel_records=0, wheel_moved=False,
        snapshot_wheel="driver",
    ),
    reading(
        "trailing-cr", "start-8", at=D + "08:48:54.698Z", exit=13, confirmed=False, text_bytes=32, sent_bytes=32,
        trailing_cr=1, refusal="not-delivered", detail="no-prompt-submitted", seconds=10.0, hint_lines=1,
        records=["send-issued", "prompt-submitted", "wheel", "turn-ended", "send-refused"],
        prompt_submitted_records=1, prompt_origin="human", prompt_text_bytes=31,
        prompt_equals_sent_text_without_cr=True, wheel_records=1, wheel_moved=True, wheel_holder="human",
        wheel_cause="human-input", wheel_ms_after_send_issued=33, turn_ended=True, snapshot_wheel="human",
        falsifies="a driver text ending in CR is delivered and answered, reported not delivered, and takes the wheel",
    ),
    fidelity(
        "start-order", live=["wheel:start", "budget-gate", "session-start:startup"],
        fake=["wheel:start", "budget-gate", "session-start:startup"], equal=True,
        note="start 8 read the same three; start 6's extra wheel record is gone with the closed list extended",
    ),
    fidelity(
        "claude-child-start", pty_backend="openpty", env_stripped_count=0, stripped_names=[],
        kept_claude_names=[], kept_claude_names_count=0, recorded_reading_names=12,
        child_viola_names=["VIOLA_BIN", "VIOLA_DIR", "VIOLA_NAME"],
        note="names only, read from the live child's environment while it was alive; the window's declared environment holds no CLAUDE name, so the 12-name reading is not comparable from this start either",
    ),
    fidelity("endpoint-kind", live="unix-socket", fake="unix-socket", equal=True),
    fidelity(
        "child-fds", live_count=23, live=[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 19, 22, 24, 26, 27, 28, 30, 43],
        fake=[0, 1, 2, 3, 4], equal=False, live_count_at_start=29, live_count_start_8_at_close=32,
        note="numbers only, read right before the close; the count moves within one session",
    ),
    fidelity(
        "session-end-order", session_end_before_child_gone=True, session_end_source="hook",
        hook_detail="channel-unreachable", wrapper_gone_s=0.03, child_gone_s=0.36, session_end_line_s=0.03,
        close="TERM to the window foot process", start_8_child_gone_s=0.38, start_8_session_end_before_child_gone=True,
    ),
]


def main():
    ev = sys.argv[1]
    run = os.path.join(ev, "live-run.ndjson")
    if os.path.exists(run):
        sys.exit("live-run.ndjson exists")
    with open(run, "w", encoding="utf-8", newline="\n") as f:
        f.write(dump(RUN))
    with open(os.path.join(ev, "live-sessions.ndjson"), "a", encoding="utf-8", newline="\n") as f:
        f.write(dump(SESSIONS))
    with open(os.path.join(ev, "live-readings.ndjson"), "a", encoding="utf-8", newline="\n") as f:
        f.write(dump(READINGS))
    print("live-run %d · live-sessions +%d · live-readings +%d" % (len(RUN), len(SESSIONS), len(READINGS)))


if __name__ == "__main__":
    main()
