#!/usr/bin/env python3
"""The setup of the statusline readings (plan.md step 12).

It makes two session directories under the repository's ignored build directory, inside the tree the founder
trusted and outside every commit:

  <base>/a   its project settings name the rig's recorder as the status line, label `direct`
  <base>/b   no settings of its own

and fills the rig's private directory (outside the tree, made 0700 by the caller):

  live-recorder.sh         the recorder, a copy of this folder's, mode 0700
  statusline-source.json   the settings-shaped file a wrapped start's home is given as its source: the same
                           recorder, label `wrapped`
  rehearsal.stdin          a synthetic statusline payload for the fake-agent rehearsal
  <label>.needles          for each label given: the literals the pty host counts in the session's screen bytes

No JSON here crosses a command line's quoting: every document is written by `json.dumps`. The user's own
settings file is never opened.

  live-sl-setup.py <private dir> <base, relative to the repository root, under target/> <label>...

Run from the repository root.
"""
import json
import os
import shutil
import sys

NEEDLES = ("SLMARKdirect", "SLMARKwrapped")
SHELL_WORDS = ' "$0" "${BASH_VERSION:+bash}" "${ZSH_VERSION:+zsh}"'


def write(path, text, mode=0o600):
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    os.chmod(path, mode)


def status_line(recorder, label):
    return {"statusLine": {"type": "command", "command": recorder + " " + label + SHELL_WORDS}}


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    priv, base, labels = sys.argv[1], sys.argv[2], sys.argv[3:]
    if not os.path.isdir(priv):
        sys.exit("no private directory")
    if not base.startswith("target/sl-live-") or os.path.exists(base):
        sys.exit("the base is not a new target/sl-live-* directory")
    here = os.path.dirname(os.path.abspath(__file__))
    recorder = os.path.join(priv, "live-recorder.sh")
    shutil.copyfile(os.path.join(here, "live-recorder.sh"), recorder)
    os.chmod(recorder, 0o700)
    os.makedirs(os.path.join(base, "a", ".claude"))
    os.makedirs(os.path.join(base, "b"))
    write(os.path.join(base, "a", ".claude", "settings.json"), json.dumps(status_line(recorder, "direct")) + "\n")
    write(os.path.join(priv, "statusline-source.json"), json.dumps(status_line(recorder, "wrapped")) + "\n")
    payload = {
        "session_id": "s-rehearsal",
        "model": {"display_name": "synthetic"},
        "rate_limits": {"five_hour": {"used_percentage": 12, "resets_at": 1738425600}},
    }
    write(os.path.join(priv, "rehearsal.stdin"), json.dumps(payload))
    for label in labels:
        write(os.path.join(priv, label + ".needles"), "".join(n + "\n" for n in NEEDLES))
    print(json.dumps({"base": base, "dirs": sorted(os.listdir(base)), "labels": labels, "needles": list(NEEDLES)}))


if __name__ == "__main__":
    main()
