"""Regression probe for the PreToolUse write guard in .claude/settings.json.

Runs the guard command exactly as rendered (no substitution) under bash with a Write tool
payload on stdin for six paths, prints `<exit> <path>` per path, then the six exits
comma-joined as the last line. Expected: 2,2,2,2,0,0 (generated `target/` blocked, `src/` allowed).
Exit 1 when the guard entry is missing, 0 otherwise.
"""

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MATCHER = "Edit|MultiEdit|Write|NotebookEdit"
GIT_BASH = r"C:\Program Files\Git\bin\bash.exe"


def guard_command():
    settings = json.loads((ROOT / ".claude" / "settings.json").read_text(encoding="utf-8"))
    for entry in settings.get("hooks", {}).get("PreToolUse", []):
        if entry.get("matcher") != MATCHER:
            continue
        for hook in entry.get("hooks", []):
            if hook.get("type") == "command":
                return hook.get("command")
    return None


def bash():
    # On Windows a PATH `bash` may be WSL's System32 launcher, which is not the shell the hook runs in.
    found = shutil.which("bash")
    if os.name == "nt" and (found is None or "system32" in found.lower()):
        return GIT_BASH
    return found or "bash"


def main():
    command = guard_command()
    if not command:
        print("guard entry not found")
        return 1
    target = ROOT / "target" / "x.rs"
    paths = [str(target), target.as_posix(), "target\\x.rs", "target/x.rs", "src\\x.rs", "src/x.rs"]
    shell = bash()
    exits = []
    for path in paths:
        payload = json.dumps({"tool_name": "Write", "tool_input": {"file_path": path, "content": "x"}})
        result = subprocess.run([shell, "-c", command], input=payload, capture_output=True, text=True)
        exits.append(str(result.returncode))
        print(result.returncode, path)
    print(",".join(exits))
    return 0


if __name__ == "__main__":
    sys.exit(main())
