@answer n=1 entity=test-strategy#1.1 v=yes
why: the unit tier runs on all 3 OSes and names the macOS-compiled-out cases and gates that the chunk removes with the macOS arms
@answer n=2 entity=test-strategy#1.2 v=no
why: the integration tier requires a suite per contract, and the Linux suites keep that whatever the removed Windows and macOS arms are
@answer n=3 entity=test-strategy#1.3 v=no
why: the E2E critical-path requirement names no OS and no unit the change edits
@answer n=4 entity=test-strategy#1.4 v=yes
why: it names the time-boxed cargo fuzz job as an ubuntu nightly job, which the scope keeps on ubuntu in nightly.yml
@answer n=5 entity=test-strategy#1.5 v=yes
why: it names the per-OS perf CI job, whose Windows and macOS legs the change removes from ci.yml
@answer n=6 entity=test-strategy#1.6 v=no
why: the chaos fault list is OS-neutral, and the SIGSTOP freeze it names is Linux behaviour the chunk leaves unchanged
@answer n=7 entity=test-strategy#1.7 v=yes
why: it names the euid wiring of the three client seams, which the scope requires to stay held by no committed test
@answer n=8 entity=test-strategy#1.8 v=no
why: the fixture and replay contract is Linux wire behaviour that no removed unit touches, and its Unix statusline gate is not named by the scope
@answer n=9 entity=test-strategy#2.1 v=yes
why: it places the icacls set and read-back and the Windows debugger attach in tests/support/home.rs and frozen.rs, both Windows arms the change removes
@answer n=10 entity=test-strategy#2.2 v=yes
why: tests/contract_os_gated_tests.rs is a file the change edits by its census, and the entity states its contract_ binary role
@answer n=11 entity=test-strategy#3.1 v=yes
why: it names the Windows listener's remote-client rejection and DACL denial and the macOS-gated server identity suite, both removed or re-pointed by the chunk
@answer n=12 entity=test-strategy#3.2 v=no
why: the paste injection tier names no OS and no unit the change edits
@answer n=13 entity=test-strategy#3.3 v=no
why: the hook stdin tier names no OS and no unit the change edits
@answer n=14 entity=test-strategy#3.4 v=no
why: the MCP E2E tier names no OS and no unit the change edits
@answer n=15 entity=test-strategy#3.5 v=no
why: the CLI and env tier names no OS and no unit the change edits
@answer n=16 entity=test-strategy#3.6 v=yes
why: it names the Windows --home protected DACL read back through icacls in the filesystem strict-modes suite, an arm the change removes
@answer n=17 entity=test-strategy#3.7 v=no
why: the child spawn tier names no OS and no Windows-only child refusal
@answer n=18 entity=test-strategy#3.8 v=yes
why: it names cargo deny, the sole-root tokio ban and the nightly advisories over the removed Windows dependencies and their deny.toml entries
@answer n=19 entity=test-strategy#3.9 v=yes
why: it names the §9 fuzz job, which the scope keeps on ubuntu in nightly.yml
@answer n=20 entity=test-strategy#3.10 v=no
why: the chaos tier location names no OS and no unit the change edits
@answer n=21 entity=test-strategy#3.11 v=no
why: the performance-budget tier names the §10 budgets, which the change does not edit
@answer n=22 entity=test-strategy#3.12 v=no
why: the cross-surface tier names no OS and no unit the change edits
@answer n=23 entity=test-strategy#3.13 v=yes
why: it names the multi-os-compat CI matrix builds and the contract_os_gated_tests.rs census, both edited by the change
@answer n=24 entity=test-strategy#3.14 v=no
why: the multi-version compat tier names no OS and no unit the change edits
@answer n=25 entity=test-strategy#3.15 v=no
why: the contract-against-sandbox tier names no OS and no unit the change edits
