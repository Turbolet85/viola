@row_set n=1 row=decision/fake-agent-as-claude cell=except seen=2 detector=none
value: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
basis: fct-gpbtjh4c
because: The report confirms that the read's exact-frame case runs through the crate's own planted hyprctl stub rather than the root fake agent, and gives the reason, so the row's claim that the fake agent alone serves hyprctl needs this named exception.
@row_set n=2 row=decision/fakes-for-external-services cell=except seen=2 detector=none
value: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
basis: fct-gpbtjh4c
because: The report confirms that the crate's own planted hyprctl stub, not the fake agent binary, stands in for the real hyprctl in viola-hyprland's unit tests, and gives the reason, so the row's claim that the same binary replaces hyprctl needs this named exception.
