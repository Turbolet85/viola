#!/usr/bin/env python3
"""The ten synthetic controls of `live-pty-arrow.py`'s key rule (plan.md step 1).

It loads the rig by path, feeds synthetic streams through its screen and its key rule the way its loop does (a read
after quiet output; a key only at a read made after the output that followed the previous key), and prints one line
per control and, last, `controls: <k> of 10 as expected`. Exit 0 only when all ten hold. Nothing is written.

  rig-controls.py
"""
import importlib.util
import os
import sys

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("live_pty_arrow", os.path.join(HERE, "live-pty-arrow.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)

NAMES = {rig.DOWN: "down", rig.UP: "up", rig.ENTER: "enter"}
TRUST = "Yes, I trust this folder"


def frame(options, pointer, extra=()):
    """A whole dialog frame: a cleared screen, a title, the options with `❯` on row `pointer`, then `extra` rows."""
    rows = ["Do you trust the files in this folder?", ""]
    for i, text in enumerate(options):
        rows.append((" ❯ " if i == pointer else "   ") + text)
    rows.extend(extra)
    return ("\x1b[2J\x1b[H" + "\r\n".join(rows) + "\r\n").encode("utf-8")


INPUT_BOX = "\r\n".join(["╭" + "─" * 40 + "╮", "│ > " + " " * 37 + "│", "╰" + "─" * 40 + "╯"]).encode("utf-8")


class Run:
    def __init__(self):
        self.screen = rig.Screen(rig.ROWS, rig.COLS)
        self.rule = rig.KeyRule()
        self.redrawn = False
        self.keys = []
        self.word = None

    def feed(self, data, step=None):
        for i in range(0, len(data), step or len(data) or 1):
            self.screen.feed(data[i : i + (step or len(data))])
        self.redrawn = True

    def read(self):
        self.word, key = self.rule.read(self.screen.lines(), self.redrawn)
        if key is not None:
            self.keys.append(NAMES[key])
            self.redrawn = False
        return key


def c1():
    r = Run()
    r.feed(frame([TRUST, "No, exit"], 0))
    r.read()
    return r.keys, r.word, r.keys == ["enter"]


def c2():
    r = Run()
    r.feed(frame([TRUST, "No, exit"], 0), step=3)
    r.read()
    return r.keys, r.word, r.keys == ["enter"]


def c3():
    r = Run()
    r.feed(frame(["No, exit", TRUST], 0))
    r.read()
    return r.keys, r.word, r.keys == ["down"] and r.rule.default_offset == -1


def c4():
    r = Run()
    r.feed(frame([TRUST, "No, exit"], 1))
    r.read()
    return r.keys, r.word, r.keys == ["up"] and r.rule.default_offset == 1


def c5():
    r = Run()
    options = ["No, exit", "Not now", TRUST]
    for pointer in (0, 1, 2):
        r.feed(frame(options, pointer))
        r.read()
    return r.keys, r.word, r.keys == ["down", "down", "enter"] and r.rule.default_offset == -2


def c6():
    r = Run()
    options = ["No, exit", "Not now", TRUST]
    for _ in range(4):
        r.feed(frame(options, 0))
        r.read()
    return r.keys, r.word, r.keys == ["down", "down"] and r.word == "unreached"


def c7():
    r = Run()
    r.feed(frame([TRUST, "No, exit"], 0, extra=["", "   Yes, allow external imports"]))
    r.read()
    return r.keys, r.word, r.keys == [] and r.word == "other-modal"


def c8():
    erased_ed = Run()
    erased_ed.feed(frame([TRUST, "No, exit"], 0) + b"\x1b[2J")
    erased_ed.read()
    erased_el = Run()
    erased_el.feed(frame([TRUST, "No, exit"], 0) + b"\x1b[3;1H\x1b[2K")
    erased_el.read()
    ok = erased_ed.keys == [] and erased_ed.word == "absent" and erased_el.keys == [] and erased_el.word == "absent"
    return erased_ed.keys + erased_el.keys, erased_ed.word + "/" + erased_el.word, ok


def c9():
    r = Run()
    r.feed(b"\x1b[2J\x1b[H" + INPUT_BOX)
    r.read()
    return r.keys, r.word, r.keys == [] and r.word == "absent"


def c10():
    r = Run()
    dialog = frame([TRUST, "No, exit"], 0)
    lines = dialog.count(b"\r\n")
    r.feed(dialog + b"\x1b[1A\x1b[2K" * lines + b"\r" + INPUT_BOX)
    r.read()
    return r.keys, r.word, r.keys == [] and r.word == "absent"


CONTROLS = [
    ("the dialog with the confirm row selected, whole", c1),
    ("the same fed in 3-byte chunks", c2),
    ("the pointer one row above the trust row", c3),
    ("the pointer one row below the trust row", c4),
    ("the pointer two rows above, redrawn after each arrow", c5),
    ("two arrows typed and the pointer still elsewhere", c6),
    ("another modal beside the dialog", c7),
    ("the dialog erased by ED 2, and by EL 2", c8),
    ("an input box alone", c9),
    ("a frame redrawn by cursor-up and EL", c10),
]


def main():
    held = 0
    for n, (name, control) in enumerate(CONTROLS, 1):
        keys, word, ok = control()
        held += ok
        print(f"control {n} · {name}: keys [{', '.join(keys)}] · verdict {word} · {'as expected' if ok else 'NOT as expected'}")
    print(f"controls: {held} of {len(CONTROLS)} as expected")
    sys.exit(0 if held == len(CONTROLS) else 1)


if __name__ == "__main__":
    main()
