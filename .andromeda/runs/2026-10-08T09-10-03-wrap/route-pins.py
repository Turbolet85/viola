"""P5 of the 2026-10-08-first-live-test-and-self-drive wrap: annotations appended to three markerless entries.

Line 105 (the next entry) takes the trailing-CR carry and, verbatim from the frozen line 102, the two CARRY
blocks written for the Epoch 3 boundary audit. Line 115 takes the switch. Line 142 takes the other-terminals
carry. Every other line stays byte-identical (asserted); idempotent (a block already standing is skipped).
"""
import sys

P = "viola-0.1.0/working-route.md"
M = "2026-10-08-first-live-test-and-self-drive"
SEP = "  CARRY: "

CR = (
    "chunk " + M + " (the operator's direction at its wrap, 2026-10-08: fold it into the next entry): a driver "
    "text ending in one CR is delivered and answered, but filed `human` and not confirmed. The CLI submits it "
    "without the CR (measured at that chunk on live 2.1.287, `evidence/live-readings.ndjson`, `trailing-cr`), so "
    "the exact match on the typed text fails, the wheel moves to the human and `send` ends `not-delivered` / "
    "`no-prompt-submitted`, exit 13. Beside it, a text of only newlines ends the same refusal when the window "
    "closes, with the wheel unmoved (`only-newlines`). No remedy is chosen: stripping the trailing LF was the "
    "founder's ruling (2026-10-07T15:21Z), and whether a trailing CR is treated the same way is his to rule; no "
    "test at any tier pins either outcome"
)
SWITCH = (
    "chunk " + M + " (the founder's answer at its planning, 2026-10-08T06:07Z, relayed by the overseer: live test "
    "now, switch later; pinned here at its wrap, on the one entry its gap list names): the overseer's switch from "
    "the prototype to viola is not made, and `verification-matrix.json#v1-33` is unclaimed. That chunk's "
    "`evidence/gap-list.md` reads the overseer's driving guide against the product: 39 items, 10 that stop a "
    "switch, in five groups — `list` before every send (this entry); a dialog held until the overseer answers "
    "(the compiled 60 s deadline, no record of an expired hold); the screen read (CLI modals, the screen before a "
    "dialog, the context percentage); `allow <n>`; and the start line and three scripts outside this repository. "
    "No route entry names the second, third or fourth group: which of them the switch needs, and which chunk "
    "claims `v1-33` with its acceptance re-worded to the Linux dev host and a key typed in the session terminal, "
    "is the founder's at this entry's take-up. Measured at that chunk on live 2.1.287 through the product "
    "binary: the takeover by a typed key, three dialog kinds answered by id, and `/clear` between skills"
)
TERMS = (
    "chunk " + M + ": the wheel's closed non-editing list was measured on one terminal, foot 1.28.0 "
    "(`evidence/reply-probe-fixed.ndjson`, 23 queries). A terminal that answers the cursor-position query "
    "`CSI ? 6 n` (with `CSI ? r;c R`) or the kitty graphics query (an APC string) writes a reply that is still "
    "typing, and the CLI has a parser for both (read from the 2.1.287 binary, that chunk's research M14; foot "
    "answers neither, measured). hypothesis: on such a terminal a live session loses the wheel at its start, as "
    "start 6 did on foot. A shape joins the list only when measured, by its exact grammar, on the founder's word "
    "(F-W2)"
)
AUDIT_HEADS = (
    "chunk 2026-10-04-windows-boundary-mutation-workflow",
    "chunk 2026-10-07-test-homes-off-the-contended-volume",
)

with open(P, encoding="utf-8", newline="") as f:
    lines = f.read().split("\n")
before = list(lines)
frozen = lines[101]
assert frozen.startswith("[" + M + "] "), "line 102 is not the chunk's frozen line"
blocks = frozen.split("  CARRY: ")
audit = []
for head in AUDIT_HEADS:
    hit = [b for b in blocks[1:] if b.startswith(head)]
    assert len(hit) == 1, head
    b = hit[0]
    cut = b.find("  WATCH: ")
    audit.append(b[:cut] if cut >= 0 else b)

for idx, name in ((104, "Self-healing state"), (114, "The board: viola list"), (141, "Linux and macOS parity")):
    assert lines[idx].startswith(name + " — "), (idx + 1, lines[idx][:40])


def add(idx, block):
    if block in lines[idx]:
        print("skipped (stands) line", idx + 1, block[:50])
        return
    lines[idx] = lines[idx] + SEP + block
    print("appended line", idx + 1, len(block.encode()), "B", block[:60])


add(104, CR)
for b in audit:
    add(104, b)
add(114, SWITCH)
add(141, TERMS)

changed = [i + 1 for i, (a, b) in enumerate(zip(before, lines)) if a != b]
assert len(lines) == len(before) and set(changed) <= {105, 115, 142}, changed
with open(P, "w", encoding="utf-8", newline="") as f:
    f.write("\n".join(lines))
print("changed lines", changed, "· total lines", len(lines))
