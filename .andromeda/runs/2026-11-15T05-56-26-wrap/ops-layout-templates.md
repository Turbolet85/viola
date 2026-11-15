@row_add n=1 row=absence/status-line-interception area=surface-cli fact=F1 detector=D-layout-surface
capability: Status-line interception: a wrapped session's claude status line is exactly as unwrapped
basis: fct-vey5bl3d
because: The chunk retired the status-line interception whole (no settings override, no --settings on run or revive, no status-line source read, no shell, viola hook statusline an unknown event), so the cli surface records the absence of that capability.
