@row_set n=1 row=decision/verify-lines-carry-no-content cell=rule seen=1 detector=none
value: No verify log line carries row text, a prompt, a session id, the probe command or a path.
basis: fct-vey5bl3d
because: Run B's status-line probe and its captures were removed (Retired (6)), so verify reads no status line payload and the rule's list loses it.
@row_set n=2 row=contract/replayed-snapshot/live-fields cell=promise seen=1 detector=none
value: A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, and run's collision check reads the heartbeat alone.
basis: fct-yeasicmk
because: The hook statusline arm was removed (Retired (4)), so the promise loses «hook statusline prints nothing»; fit read the proposed heir as standing under this row, so the row is set rather than retired and re-added.
