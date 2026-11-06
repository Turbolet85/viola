@amendment marker=2026-11-05-linux-only-code-and-ci
title: handle inheritance without the Windows mechanism
section: §Data Protection (body: Handle inheritance; the inheritable-handles ban)
change:
  - Handle inheritance: was «portable-pty 0.8.1 calls `CreateProcessW` with `bInheritHandles = 0`, and interprocess listeners stay `inheritable(false)`, so channel handles never leak into the `claude` child»; now channel handles never leak into the `claude` child, on Linux the property resting on close-on-exec descriptors, recorded, not established, its measurement owed to the working-route entry "Version done-check". The swap clause stands.
  - The ban: was «NEVER make channel handles inheritable, and NEVER adopt a portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE`»; now «NEVER make channel handles inheritable».
why: the Windows mechanics left with the Windows code (the founder's own rulings of 2026-11-04, inputs#I1, inputs#I5 item 6), and the report states no Linux mechanism; escalated as Q4 and resolved by the overseer's technical answer, founder-delegated, 2026-11-06 16:25Z: keep the property, mark close-on-exec on Linux as recorded and not established, drop the `CreateProcessW` clause, and owe the measurement (the wrapped child's `/proc/<child pid>/fd` holding no channel socket or listener, under the fake agent) to a CARRY on «Version done-check».
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the one-leg CI jobs and the bind rule's reason
section: §Dependency Security (body: the `release` and `perf` job lines); §Authentication & Authorization (body: the socket-mode rule)
change:
  - The `release` and `perf` job lines read «The `release` job» and «The `perf` job»; was «The per-OS …», the matrix being `[ubuntu-latest]`.
  - The socket-mode rule keeps chmod after the bind and never `ListenerOptionsExt::mode`; its reason reads a rule set while macOS, where the mode returns `Unsupported` and fails the bind, was a target, and kept; was the macOS failure stated as current.
why: the CI legs and the macOS target are retired (the founder's own rulings of 2026-11-04, inputs#I1); a residual census after the cascade sweep found this wording no sweep pattern named (`cascade-dispositions.md` §Residual census).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
