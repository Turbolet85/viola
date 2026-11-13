@row_set n=1 row=contract/instance-unreachable-detail/unwrapped-name cell=promise seen=1 detector=none
value: The name has no live endpoint and no instance directory (absent or a regular file).
basis: fct-yeqzuggx
because: The strict-modes check now refuses a symbolic link at the instance directory first, so a symlink reads strict-modes-failed and the row's symlink clause is false.
