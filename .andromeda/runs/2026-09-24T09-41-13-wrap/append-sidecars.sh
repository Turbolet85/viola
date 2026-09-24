cd /d/dev/projects/viola
T=C:/Users/turbo/.claude/skills/andromeda-tools/scripts
R=.andromeda/runs/2026-09-24T09-41-13-wrap
M=2026-09-24-supply-chain-and-workflow-gates
for d in architecture test-plan security-plan obs-plan; do
  n=$(wc -l < "$R/$d-entry.md")
  python -X utf8 $T/splice.py append --file .andromeda/$d-amendments.md --payload-file "$R/$d-entry.md" --lines "$n" --run-dir "$R" --marker "$M"
  echo "$d append exit=$?"
  echo "--- $d last line:"; tail -1 .andromeda/$d-amendments.md | cut -c1-160
done
