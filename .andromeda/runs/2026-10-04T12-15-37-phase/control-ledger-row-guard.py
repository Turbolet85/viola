"""Known-verdict controls for the plan's ledger-row guard (gate 26): its grep -cE pattern, fed synthetic diff lines."""
import subprocess

PATTERN = r'^\+.*(Self::[A-Z][A-Za-z]+ => "[a-z-]+",|pub const ALL: \[Self; )'
CASES = [
    ("must count: a new row id arm", '+            Self::QuestionUpdatedInput => "question-updated-input",\n', "1"),
    ("must count: a resized ALL", "+    pub const ALL: [Self; 10] = [\n", "1"),
    ("must not count: an event_name arm", '+        HookEvent::PreToolUse => "PreToolUse",\n', "0"),
    ("must not count: a removed row line", '-            Self::StopMessage => "stop-message",\n', "0"),
]
for label, line, want in CASES:
    out = subprocess.run(["grep", "-cE", PATTERN], input=line, capture_output=True, text=True)
    got = out.stdout.strip()
    print(f"{label}: count {got} exit {out.returncode} -> {'ok' if got == want else 'WRONG'}")
