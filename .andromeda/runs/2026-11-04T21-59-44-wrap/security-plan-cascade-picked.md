@account n=1 entity=authentication-authorization#3.10 v=proposed
why: the carve-out is unchanged, but the new census now names the tests it keeps off macOS under one reason, NO_PEER_PID, which the row's stack reference should cite as its standing witness
@account n=2 entity=input-validation#1.12 v=not_touched
why: TERM is still only compared and still decides SGR alone; the new outer-PTY witness is a test-plan §6 and a11y-plan fact, and this row's How names product code only
@account n=3 entity=bootstrap-phases#2.1 v=not_touched
why: no workflow, scan class, skip or upload moved; a green ci#41918694243 with the secret scan inside it confirms the row and adds nothing to it
@account n=4 entity=secret-management#2.1 v=not_touched
why: pre-push re-ran green with its env -i launcher unchanged, and scripts/junit-os-sets.py is an operator-pass tool that is not part of pre-push, so the row's single-launcher claim still holds
@account n=5 entity=error-handling#1.35 v=not_touched
why: tests/cross_exit_causes.rs gained only comment and reason lines (12, 268, 283), and Cargo.toml is unchanged, so its cross_ binary, required-features and the four named tests still hold
