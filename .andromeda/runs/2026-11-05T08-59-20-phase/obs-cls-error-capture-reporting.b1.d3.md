@answer n=1 entity=error-capture-reporting#1.1 v=yes
why: the Windows RtlCaptureStackBackTrace arm of src/panic_frames.rs is removed, so the Unix libc::backtrace and dladdr capture loses its cfg gate and must keep its 62-string raw frames
@answer n=2 entity=error-capture-reporting#1.2 v=yes
why: the Windows write-rule arm of check_diagnostics is removed, so the Unix read rule, symlink refusal and absent-path pass of the file check must keep holding
@answer n=3 entity=error-capture-reporting#2.1 v=yes
why: Cargo.toml is edited and G3 runs over it, so panic = "unwind" and the catch-site exit mapping (exit 1, exit 0, internal-error) must stay intact
@answer n=4 entity=error-capture-reporting#3.1 v=yes
why: the pre-init instance checks refuse with exit 21 instance-unreachable and strict-modes-failed, which the scope names as unchanged Linux values, and the check's Windows arm is removed
@answer n=5 entity=error-capture-reporting#4.1 v=yes
why: the scope names G2's single seam exemption and the e2e-home coverage rule (items 15 and 18), and the entity states the outside-scan home that G2 must keep skipping plus the parse-rejected detail value
@answer n=6 entity=error-capture-reporting#4.2 v=yes
why: the Windows byte reading and its cfg(windows) arm leave with the removal, and the Unix passthrough assertion of the feed-panic test must keep holding
@answer n=7 entity=error-capture-reporting#5.1 v=yes
why: viola mcp frame tools are named by the scope's wait/last and MCP server-verification paths, and this entity states the strict-modes-failed refusal they must keep
@answer n=8 entity=error-capture-reporting#5.2 v=yes
why: the MCP frame socket-dir refusal sits on the server-verification path the scope names, and its socket-dir-refused detail is a Linux value the chunk keeps
@answer n=9 entity=error-capture-reporting#5.3 v=yes
why: the MCP ServerVerify mapping of a refused peer_admitted or failed server_verified is the server-verification path whose macOS carve-out the chunk removes
@answer n=10 entity=error-capture-reporting#5.4 v=yes
why: the entity states exit 0 and exit 1 and the parse-rejected oversize detail for mcp stdin, all in the typed exit set and detail catalog the scope keeps fixed
@answer n=11 entity=error-capture-reporting#5.5 v=yes
why: the entity states exit 1 and the parse-rejected mcp-stdin oversize line before initialize, a Linux exit and detail value the scope keeps fixed
@answer n=12 entity=error-capture-reporting#6.1 v=yes
why: sanitised_fault is the reader of a peer's wire fault, and its code and message mapping is a Linux wire contract the scope keeps fixed
@answer n=13 entity=error-capture-reporting#6.2 v=yes
why: the entity states the closed-word screen and the exit 14 unknown mapping, both in the typed exit set and closed-word catalog the scope keeps fixed
@answer n=14 entity=error-capture-reporting#6.3 v=yes
why: the thiserror::Error derive count of 9 spans src and crates, and the whole-reached Windows-only files (conpty.rs, sideload.rs) may hold derives the removal would move
@answer n=15 entity=error-capture-reporting#6.4 v=no
why: the fixed NotADirectory message on Refused names no exit, closed word or wire contract the scope keeps, and no removed Windows arm or reached file carries it
@answer n=16 entity=error-capture-reporting#6.5 v=yes
why: the entity states exit 2 for the answer and send UNUSABLE lines, a value in the typed exit set the scope keeps fixed
@answer n=17 entity=error-capture-reporting#6.6 v=yes
why: the entity states the exit 21 instance-unreachable document and its six closed UnreachableCause words, which the scope's MCP paths and detail catalog keep unchanged
