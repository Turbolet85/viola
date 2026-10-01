import sys
R = ".andromeda/runs/2026-10-01T12-19-55-wrap/"
with open("viola-0.1.0/working-route.md", "rb") as f:
    lines = f.read().split(b"\n")
l66 = lines[65].decode("utf-8")
assert l66.startswith("Readiness gate and timing constants"), l66[:60]
wsl = "CARRY: chunk 2026-09-27-browser-verdict-reachability (overseer live ratification"
hr = "CARRY: chunk 2026-09-29-sideloaded-conpty (the operator's word at its wrap, the overseer agreeing), re-read by"
assert l66.count(wsl) == 1 and l66.count(hr) == 1
i, j = l66.index(wsl), l66.index(hr)
assert i < j
wsl_block = l66[i:j].rstrip(" ")
hr_block = l66[j:]
assert "  CARRY:" not in hr_block and "  CARRY:" not in wsl_block
with open(R + "route-a-head.txt", "rb") as f:
    head = f.read().decode("utf-8").rstrip("\n")
new = head + "  " + wsl_block + "  " + hr_block
with open(R + "route-a.txt", "wb") as f:
    f.write((new + "\n   ↓\n").encode("utf-8"))
# the removed tail of :66, for the Edit's old_string
with open(R + "route-a-removed.txt", "wb") as f:
    f.write(("  " + wsl_block + "  " + hr_block).encode("utf-8"))
print("wsl", len(wsl_block), "hr", len(hr_block), "new line", len(new))
