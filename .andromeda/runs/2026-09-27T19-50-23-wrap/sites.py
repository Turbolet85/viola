import re, sys, pathlib
root = pathlib.Path(r"D:/dev/projects/viola/.andromeda")
docs = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
pats = {
    "browser-linux-only": r"browser-linux-only",
    "booted_wrapper": r"booted_wrapper",
    "browser-missing": r"browser-missing",
    "tool-pin-mismatch": r"tool-pin-mismatch",
    "test-runner-install": r"test-runner-install",
    "axe-core/playwright": r"axe-core/playwright",
    "bay-layout-type": r"bay-layout-type|<bay-",
    "Drivers": r"Drivers",
    "ubuntu-only-browser": r"(?i)(playwright|browser|a11y)[^\n]{0,80}(ubuntu[- ]only|linux[- ]only|ubuntu leg|ubuntu-latest only)",
    "runner-image-node": r"(?i)runner[- ]image node|setup-node|runner-image",
    "Browser caching": r"Browser caching",
    "npm-audit": r"(?i)npm audit|package-lock|npm lockfile",
    "wsl-provision": r"wsl-provision",
    "viola-root-watch": r"viola-root-watch|viola-pty-watch",
    "junit-os": r"junit-<os>|junit-playwright",
    "W125": r"W125",
    "nightly": r"nightly\.yml",
    "CI render": r"(?i)CI render",
    "D-A11Y-12": r"D-A11Y-12",
    "pre-push linux-tests": r"linux-tests",
    "10 s wait": r"10 s",
}
only = sys.argv[1:]
for name, p in pats.items():
    if only and name not in only:
        continue
    rx = re.compile(p)
    row = []
    for d in docs:
        lines = (root / f"{d}.md").read_text(encoding="utf-8").splitlines()
        hits = [i + 1 for i, l in enumerate(lines) if rx.search(l)]
        if hits:
            row.append(f"{d}:{len(hits)}@{','.join(map(str, hits[:12]))}")
    print(f"{name}: " + (" | ".join(row) if row else "0 hits"))
