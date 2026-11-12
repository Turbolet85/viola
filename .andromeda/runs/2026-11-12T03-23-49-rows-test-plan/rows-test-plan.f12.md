@row_add n=1 row=decision/fixture-chain-exists-twice area=5-command-implementation rec=1
name: Fixture chain exists twice
rule: The home, fake_agent_path, stamped_home and booted_wrapper chain exists twice, as the sync root copy and the viola-e2e copy, against one contract.
why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
@row_add n=2 row=decision/coverage-floors area=5-command-implementation rec=2,3,4
name: Coverage floors
rule: CI enforces coverage floors of 85 lines, 95 functions and 80 regions, each floor its own breach, measured with the fake agent, viola-e2e, test support and fuzz trees ignored.
