@row_set n=1 row=contract/instance-unreachable-detail/unwrapped-name cell=promise seen=1 detector=none
value: A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with anything else at the instance path (absent or a regular file); the reading opens nothing and writes no line.
basis: fct-yeqzuggx
because: The strict-modes check now refuses a symbolic link at the instance directory before any reading, so a symlink reads strict-modes-failed and the row's symlink clause is false.
