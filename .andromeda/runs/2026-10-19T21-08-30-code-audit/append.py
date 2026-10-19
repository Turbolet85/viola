#!/usr/bin/env python3
"""P3 append: the pinned recipe (audit-pass.md §Ledger I/O), verbatim logic, run by path. usage: append.py <record.json>"""
import json, os, sys
p = '.andromeda/code-metrics.ndjson'
raw = open(p, 'rb').read() if os.path.isfile(p) else b''
crlf = raw.count(b'\r\n')
crlf and crlf != raw.count(b'\n') and sys.exit('code-metrics.ndjson is MIXED - setup upgrade U11 repairs it; nothing appended')
rec = json.load(open(sys.argv[1], encoding='utf-8'))
open(p, 'ab').write(json.dumps(rec, ensure_ascii=False).encode('utf-8') + (b'\r\n' if crlf else b'\n'))
