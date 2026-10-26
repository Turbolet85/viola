# Classified — security · bootstrap-phases.b1

## Answers
bootstrap-phases#1.1 · no · every phase names another chunk's code (viola-core validators, the viola-channel listener and client, verify's ledger row, the bin/ and plugin/ writers, viola-state home and strict-modes), none of which the tokio pin move reaches; the sha2 pin and the SQOS recipe are rows this chunk does not edit, witnessed only by the workspace-test re-run every change owes
bootstrap-phases#2.1 · no · the CI canary scan and its upload set are not reached: the chunk edits only the supply-chain job, which uploads nothing and is not scan-gated, and the nightly fuzz upload is outside the chunk
