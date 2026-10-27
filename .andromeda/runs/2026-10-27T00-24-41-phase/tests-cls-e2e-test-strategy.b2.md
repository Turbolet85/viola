# Classified — tests · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · Path 5 is a Not-needed path (Paths 2–6), so its cleanup is that scenario's own
e2e-test-strategy#7.1 · no · Path 6 is a Not-needed path (Paths 2–6), the chunk changes no budget, send or statusline behaviour
e2e-test-strategy#7.2 · no · Path 6's verification signals read the budget gate and the statusline arm, both Not-needed
e2e-test-strategy#7.3 · no · Path 6's as-landed cases read the budget, refusal and statusline behaviour, all Not-needed
e2e-test-strategy#7.4 · no · Path 6 is a Not-needed path, so its harness cleanup is that scenario's own
e2e-test-strategy#8.1 · yes · it states the one-instance Path 7 workaround the task says stands, and step 4's fresh-home viola verify --home H3 runs the absent-copy arm of pin_exe
e2e-test-strategy#8.2 · yes · the fresh-home verify of step 4 must still exit 0 with the stamped last line of its summary once the absent-copy call changes
e2e-test-strategy#8.3 · no · standing S3 · every test's fresh home and its Drop-guard teardown (harness cleanup, TempDir drop)
e2e-test-strategy#9.1 · no · E2's R8 env strip is an untouched scenario (Not needed), the edit moves the pinned copy's write, not the child's env
e2e-test-strategy#9.2 · no · E2's child-fd rule is an untouched scenario (Not needed), and portable-pty closes every fd above 2 at spawn
e2e-test-strategy#9.3 · no · E2 is an untouched scenario (Not needed), so its TempDir drop is that scenario's own
e2e-test-strategy#10.1 · no · E3 link / unlink is an untouched scenario (Not needed), the chunk adds no link record, verb or restore
e2e-test-strategy#11.1 · no · E5 replay recovery is an untouched scenario (Not needed), the edit touches neither the snapshot read nor the collision check
e2e-test-strategy#12.1 · yes · its --home strict-modes refusals witness the home-tree check the chunk re-asserts unchanged, its MCP and hook matrices are Not needed
e2e-test-strategy#13.1 · no · the events helper's checks after every E2E test are a convention each new E2E test owes, and the chunk names no error body or diagnostics line of its own
e2e-test-strategy#14.1 · yes · the run exit-1 rows for the home-tree refusal before the log and for the pinned exe's SHA-256 re-hash mismatch witness the two rules the chunk re-asserts
e2e-test-strategy#15.1 · yes · the umask-independent row (the pinned bin/<version>-<hash>/viola is 0700, never group- or world-writable) is the private-write rule the fix keeps through fs.rs
e2e-test-strategy#16.1 · no · the viola list rows and board are Not needed, the chunk changes no list output or claude agents --json join
e2e-test-strategy#17.1 · no · none of its listed chaos cases reads the absent-copy write or the start's pin call the chunk changes
