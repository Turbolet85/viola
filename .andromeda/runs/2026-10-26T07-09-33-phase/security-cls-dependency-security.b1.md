# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY binaries and their conpty-vendor.sh gate are on the task's Not-needed list, and no reached file is a ConPTY file
dependency-security#2.1 · yes · the server check's Tokio side (viola-channel tokio_client.rs) and its sync side (client.rs) must keep the deny-sync.toml ban and scripts/channel-tokio-free.sh green
dependency-security#3.1 · no · the fuzz/ exemption is on the task's Not-needed list, and the chunk reaches no fuzz/ file and names no dependency
dependency-security#4.1 · no · src/conpty.rs is in no reached file, and the ConPTY sideload is on the task's Not-needed list
dependency-security#5.1 · no · the windows-mutants workflow is no reached file, and the chunk adds no matrix item or upload
dependency-security#6.1 · no · G2's only exemption is the seam's forced panic in src/cmd/hook/seam.rs, a file the task does not reach, and the chunk names no further exemption
dependency-security#7.1 · no · the action SHA pins bind workflow files, none of which the task names (its Not-needed list has SHA-pinned actions), and the chunk adds no CI job
