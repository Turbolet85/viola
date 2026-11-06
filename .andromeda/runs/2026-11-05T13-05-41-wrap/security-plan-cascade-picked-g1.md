@account n=1 entity=threat-model-summary#1.3 v=proposed
why: it names the pinned executable as `viola(.exe)`, a Windows path form the Linux-only cut retired
@account n=2 entity=threat-model-summary#1.4 v=not_touched
why: the R8 strip still strips the inherited CLAUDE* names and the row names no registry, persistent set or case fold, so its claim holds after the change
@account n=3 entity=threat-model-summary#2.1 v=proposed
why: it names the Windows named-pipe endpoint, the shared pipe namespace, per-OS pipe DACL enforcement and the macOS no-pid carve-out, all retired
@account n=4 entity=threat-model-summary#8.1 v=proposed
why: its «on every OS» clause on the board scan's read rule is stale wording on a Linux-only product
@account n=5 entity=threat-model-summary#9.1 v=proposed
why: it names `viola(.exe)`, the Windows owner and DACL strict-modes form and `pin_companions`' ConPTY companion check, all removed
@account n=6 entity=threat-model-summary#11.1 v=proposed
why: it names the npm-shim to `claude.exe` resolution and the wrapped-OS (Unix) / Windows-passes-none split of Run B's settings, both removed
@account n=7 entity=threat-model-summary#13.1 v=proposed
why: it names the `windows-2025` / `macos-latest` matrix and per-OS jobs, now one `[ubuntu-latest]` leg, and is the place for F4's tag fact
