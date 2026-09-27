# design extract

## No domain coverage
This chunk builds only developer and CI tooling: WSL2 provisioning, a Linux clone sync, and an `agent-run` pre-push gate in `scripts/` and `crates/viola-e2e/src/harness/`, plus docs. Its scope says product-crate behaviour does not change. design-system.md applies only to viola's product surfaces. Those are the web-spa (`viola ui`, §Surface: web-spa) and the output of the `viola` binary itself (§Surface: cli). The plan has nothing on agent-run, the harness, nextest or mutants output, and there is no amendment history (design-system-amendments.md does not exist).
