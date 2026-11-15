@row_retire n=1 row=decision/statusline-only-shell-out seen=1 detector=D-retired
why: The statusline shell-out and the settings override are gone and viola runs no shell at all.
basis: fct-vey5bl3d
because: The only shell-out was removed (Retired (4)); the ban on any shell-out stands in architecture decision/no-shell-out, so no heir is named here.
@row_retire n=2 row=decision/single-shell-out seen=1 detector=D-retired
why: The recorded command's shell-out is removed and viola runs no shell at all.
basis: fct-vey5bl3d
because: With the hook statusline shell-out gone the single exception to direct spawns is gone; the ban stands in architecture decision/no-shell-out.
@row_retire n=3 row=decision/statusline-shell-out seen=1 detector=D-retired
why: The recorded statusline command is no longer run through /bin/sh -c.
basis: fct-vey5bl3d
because: The shell-out was removed and viola runs no shell at all; the ban stands in architecture decision/no-shell-out.
