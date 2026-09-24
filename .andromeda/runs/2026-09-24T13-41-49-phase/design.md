# design extract

## No domain coverage
This chunk covers CI and test-harness gates only: fmt/clippy, MSRV, llvm-cov floors, proptest/fuzz replay, zero retries, the `viola-harness gate` verdict, zizmor concurrency, the `mutants.out/` upload and the `file_mode` mutant kill. None of it renders on the web-spa or the product `viola` CLI surface. The one design-to-CI rule, design-system §Typography "Assertions hold on the Linux fallback" (headless GUI checks with DejaVu), belongs to the browser job, and the scope defers that job to Epoch 8.
