@answer n=1 entity=obs-scope-summary#17.1 v=no
why: the session adds only operator-pass and test files, so the CI/CD Platform source named by the arch excerpt is unchanged
@answer n=2 entity=obs-scope-summary#17.2 v=no
why: junit-os-sets.py is an operator-pass reader and never a CI step, so the CI/CD Platform still emits no product telemetry
@answer n=3 entity=obs-scope-summary#17.3 v=no
why: the session moved no fixture and the not-product-runtime reason still holds
@answer n=4 entity=obs-scope-summary#18.1 v=no
why: the tracing spans and the per-process panic hook are src code the session did not change
@answer n=5 entity=obs-scope-summary#18.2 v=no
why: the CLI frontend stays N/A, since the list colour witness adds no TUI, prompt or pager
@answer n=6 entity=obs-scope-summary#18.3 v=no
why: the file sink and the refused-check outcomes are product behaviour the session did not change
@answer n=7 entity=obs-scope-summary#18.4 v=no
why: service identity and CARGO_PKG_VERSION are unchanged and Cargo.toml did not move
@answer n=8 entity=obs-scope-summary#18.5 v=yes mode=add-to
why: the macOS no-peer-pid carve-out stands unchanged in behaviour but is now counted as 43 cases that run on ubuntu-latest and not on macos-latest
@answer n=9 entity=obs-scope-summary#19.1 v=no
why: the run start-sequence spans and the viola-pty seam operations are product code the session did not change
@answer n=10 entity=obs-scope-summary#19.2 v=no
why: the terminal still carries the child's screen, and the session added only a test witness under a PTY
@answer n=11 entity=obs-scope-summary#19.3 v=no
why: run still writes nothing to the terminal and its diagnostics file sink is unchanged product behaviour
@answer n=12 entity=obs-scope-summary#19.4 v=no
why: service identity with process run is unchanged
@answer n=13 entity=obs-scope-summary#19.5 v=yes mode=add-to
why: the child-exit-on-process-handle claim stands, and the new Unix witness run_child_ends_when_its_wrapper_is_terminated now proves the child ends when its wrapper is SIGKILLed
@answer n=14 entity=obs-scope-summary#20.1 v=no
why: the hook spans and the catch_unwind panic emission are product code the session did not change
@answer n=15 entity=obs-scope-summary#20.2 v=no
why: the hook frontend stays N/A and no output surface changed
@answer n=16 entity=obs-scope-summary#20.3 v=no
why: the hook file sink and the stdout decision body are unchanged product behaviour, and only test files moved
@answer n=17 entity=obs-scope-summary#20.4 v=no
why: hook service identity and the silent no-op without VIOLA_NAME are unchanged
@answer n=18 entity=obs-scope-summary#20.5 v=no
why: the append-mode writes and the perf budgets are unchanged, and no perf row or gate command changed
@answer n=19 entity=obs-scope-summary#21.1 v=no
why: the channel spans and the corr fields are product code the session did not change
@answer n=20 entity=obs-scope-summary#21.2 v=no
why: the channel frontend stays N/A and no output surface changed
@answer n=21 entity=obs-scope-summary#21.3 v=no
why: the exporter is still the log file of whichever process is on each side, unchanged
@answer n=22 entity=obs-scope-summary#21.4 v=no
why: service identity and the peer version in params.sender are unchanged
@answer n=23 entity=obs-scope-summary#21.5 v=no
why: the peer-euid refusal in accept_loop and the JSON-RPC rules are product code the session did not change
@answer n=24 entity=obs-scope-summary#22.1 v=no
why: the MCP tool-call spans are product code the session did not change, and list adds no stderr writer
@answer n=25 entity=obs-scope-summary#22.2 v=no
why: the MCP frontend stays N/A and no output surface changed
