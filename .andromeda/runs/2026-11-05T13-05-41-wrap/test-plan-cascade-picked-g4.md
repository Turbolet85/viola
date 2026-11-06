@account n=1 entity=e2e-test-strategy#1.1 v=proposed
why: it pins the dim-stale witness to ubuntu-latest and macos-latest under cfg(unix) and names a windows-2025 attribute-set arm, while the report ungates that test and retires both other legs
@account n=2 entity=e2e-test-strategy#1.3 v=proposed
why: its tui mode reads ConPTY / openpty and its panic witness splits Unix from every OS, while pty_backend is always openpty on a Linux-only product
@account n=3 entity=e2e-test-strategy#2.2 v=proposed
why: it states no settings.json is written on Windows and names a cfg!(windows) branch, both removed with the Windows code
@account n=4 entity=e2e-test-strategy#3.1 v=proposed
why: it claims Path 2 coverage on all three CI OSes, while CI is now ubuntu-latest alone
@account n=5 entity=e2e-test-strategy#3.3 v=proposed
why: it claims all three CI OSes and owes a Windows reading to the retired Windows-only live measurements entry, plus a Windows not-measured item
@account n=6 entity=e2e-test-strategy#4.1 v=proposed
why: it claims Path 3 coverage on all three CI OSes, while CI is now ubuntu-latest alone
@account n=7 entity=e2e-test-strategy#4.5 v=proposed
why: it names a Windows OpenProcess + TerminateProcess arm and every OS, while tests/run_cli.rs keeps kill -KILL only
@account n=8 entity=e2e-test-strategy#5.1 v=proposed
why: it claims Path 4 coverage on all three CI OSes, while CI is now ubuntu-latest alone
@account n=9 entity=e2e-test-strategy#6.1 v=proposed
why: it claims all three CI OSes and a windows-2025 focus-case arm, both retired with the Windows leg
@account n=10 entity=e2e-test-strategy#7.1 v=not_touched
why: its two Unix qualifiers (the planted source read 0600, the 0770 chmod) still hold on the Linux-only build and the report moves no step of it
@account n=11 entity=e2e-test-strategy#7.2 v=proposed
why: it says shell_argv returns None on Windows and the arm runs no command there, while shell_argv now returns [&str; 3] on every build
@account n=12 entity=e2e-test-strategy#7.3 v=proposed
why: it reads green on the three CI runners with windows-2025 included and states a cfg(not(unix)) arm that prints nothing, both retired
@account n=13 entity=e2e-test-strategy#8.3 v=proposed
why: it states a three-CI-OS green as current, which is now a dated reading of the retired matrix
@account n=14 entity=e2e-test-strategy#9.2 v=proposed
why: it asserts Windows handle non-inheritance and limits the claude_env_keep survival to Unix, while the Windows arms and registry reader are retired
