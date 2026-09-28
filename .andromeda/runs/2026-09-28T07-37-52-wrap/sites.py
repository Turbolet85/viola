"""Locate each expected amendment's sites in the seven masters: pattern -> per-master hit lines."""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
MASTERS = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
PATTERNS = {
    "seam-env": r"FAKE_AGENT_PUMP_DELAY_MS|FAKE_AGENT_HOOK_PANIC",
    "one-seam": r"(?i)(the|one|only|sole) (test[- ])?seam|only env var outside|one exception",
    "perf-exports": r"perf/\*\.json|perf/hook-|perf-\*\.json|perf-<hook>|target/perf",
    "ci-count": r"(?i)\b8 jobs\b|15 check-runs|\b9 jobs\b|18 check-runs",
    "run-perf": r"--perf\b",
    "forced-panic": r"(?i)forced[- ]panic|FAKE_AGENT_HOOK_PANIC|panic seam",
    "g2": r"\bG2\b",
    "d28": r"D-28",
    "vector6": r"(?i)vector 6|cli_controls_not_disableable",
    "hyperfine": r"(?i)hyperfine",
    "pre-tool-use-row": r"pre-tool-use",
    "perf-upload": r"perf-<os>|diag-perf|perf-\$\{\{",
    "g2-script": r"g2-zero-panics|g2-probe",
}
for pid, pat in PATTERNS.items():
    rx = re.compile(pat)
    print(f"== {pid}: {pat}")
    for m in MASTERS:
        lines = (ROOT / ".andromeda" / f"{m}.md").read_text(encoding="utf-8").splitlines()
        hits = [(i + 1, len(l)) for i, l in enumerate(lines) if rx.search(l)]
        if hits:
            print(f"   {m}: {len(hits)} lines " + " ".join(f"{n}({c}c)" for n, c in hits[:40]))
