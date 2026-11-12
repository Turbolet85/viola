@row_add n=1 row=contract/mutants-verdict/no-rust-delta area=quality-gates-coverage-targets rec=1,2
boundary: run --mutants verdict
item: no-rust-delta
promise: A closed mutants.verdict value: a diff naming no .rs path never reaches cargo-mutants and passes with tested 0, the diff and its file count.
@row_add n=2 row=contract/mutants-verdict/test-only-rust-delta area=quality-gates-coverage-targets rec=1,2
boundary: run --mutants verdict
item: test-only-rust-delta
promise: A closed mutants.verdict value: a diff whose .rs paths are all test targets passes only with this verdict, naming the diff, its file count and rust_files.
@row_add n=3 row=contract/mutants-verdict/counted area=quality-gates-coverage-targets rec=1,2
boundary: run --mutants verdict
item: counted
promise: A closed mutants.verdict value: any other diff naming a .rs path, a mixed one included, is counted; a counted run that produces no fresh outcomes.json is red.
@row_add n=4 row=contract/mutants-verdict/package area=quality-gates-coverage-targets rec=1,2
boundary: run --mutants verdict
item: package
promise: A closed mutants.verdict value: a whole member scored by run --mutants --package <member>, under the same missed == 0, timeout == 0 and unviable <= caught.
@row_add n=5 row=decision/perf-gate-spine-deadline area=quality-gates-coverage-targets rec=3,4,5
name: Hook perf gate at the spine deadline
rule: The five hook perf rows are required by name, and every timed hook row's hyperfine max must be below the product's SPINE_DEADLINE (1.0 s); a missing row is a breach, never a pass.
