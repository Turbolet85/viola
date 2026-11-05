@account n=1 entity=threat-model-summary#1.3 v=carried
why: it holds the Unix-only settings override and the statusline shell-out, which the chunk makes unconditional and which must stay the only shell-out
@account n=2 entity=threat-model-summary#1.5 v=not_in_scope
why: the snapshot's operational metadata and its cwd; the chunk adds or moves no snapshot field and no path toward any output
@account n=3 entity=threat-model-summary#2.1 v=carried
why: it states the per-OS IPC enforcement whose Windows arms and macOS carve-out leave while the Linux arms and closed causes must stay whole
@account n=4 entity=threat-model-summary#5.1 v=carried
why: hook statusline's shell-out and every hook path's strict-modes check sit in cfg(unix) arms the chunk makes unconditional, and hook's fail-open contract is re-asserted
@account n=5 entity=threat-model-summary#6.1 v=carried
why: the MCP tools' socket-dir check and peer euid decision are Unix client arms the chunk must keep whole on Linux
@account n=6 entity=threat-model-summary#8.1 v=carried
why: the CLI strict-modes checks and their exit 1 / exit 21 refusals lose their Windows arm and must refuse exactly as before on Linux
@account n=7 entity=threat-model-summary#9.1 v=carried
why: the home-tree strict-modes check loses its Windows owner-and-DACL arm and the pin_companions check leaves with sideload.rs
@account n=8 entity=threat-model-summary#11.1 v=carried
why: the npm-shim to claude.exe resolution and verify's Unix-only Run B settings are OS arms the chunk removes or makes unconditional
@account n=9 entity=threat-model-summary#12.1 v=carried
why: the chunk edits deny.toml, Cargo.toml and ci.yml, and the cargo deny, deny-sync, lockfile and rustup toolchain boundary must stay live with msrv green
@account n=10 entity=threat-model-summary#13.1 v=carried
why: it states the three-OS ci.yml matrix and the per-OS lint, perf and release jobs the chunk shrinks to the ubuntu leg
