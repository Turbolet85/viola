@row_retire n=1 row=contract/viola-hook-statusline/exit seen=1 detector=D-retired
why: viola hook statusline no longer runs a command under a deadline; the word is now an unknown hook event.
basis: fct-yeasicmk
because: The statusline arm and its 5 s deadline are removed (Retired (4)); the word's unknown-event exit stands in architecture contract/hook-to-kind-map/hook-statusline, so no heir is named here.
