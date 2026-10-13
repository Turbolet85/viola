# audit-pass.md, Ledger I/O, the pinned append recipe (verbatim logic), run by path from the repository root:
#   python3 .andromeda/runs/2026-10-12T06-54-29-code-audit/append.py .andromeda/runs/2026-10-12T06-54-29-code-audit/record.json
import json, os, sys
p = '.andromeda/code-metrics.ndjson'
raw = open(p, 'rb').read() if os.path.isfile(p) else b''
crlf = raw.count(b'\r\n')
crlf and crlf != raw.count(b'\n') and sys.exit('code-metrics.ndjson is MIXED - setup upgrade U11 repairs it; nothing appended')
rec = json.load(open(sys.argv[1], encoding='utf-8'))
open(p, 'ab').write(json.dumps(rec, ensure_ascii=False).encode('utf-8') + (b'\r\n' if crlf else b'\n'))
