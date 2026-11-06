@answer n=1 entity=established-decisions#1.1 v=no
why: the Cargo delta adds no crate and only viola-mcp enables tokio (untouched record), and the portable-pty bump re-read on 0.9.0 leaves viola-pty unchanged
@answer n=2 entity=established-decisions#2.1 v=yes mode=contradict
why: portable-pty moves from =0.8.1 to =0.9.0 with nix 0.28.0, so the pin, the owed Version done-check move and the portable-pty-psmux swap trigger (retired) no longer read as stated
@answer n=3 entity=established-decisions#2.2 v=yes mode=contradict
why: RUSTSEC-2026-0441 is closed (cargo tree -i pin-utils finds no package) and the deny.toml advisories ignore list is empty, so the red reading and its carry no longer stand
@answer n=4 entity=established-decisions#3.1 v=no
why: vt100 is not in the Cargo delta and the live 2.1.287 round failed only statusline-rate-limits, so the signature and quiet-period rows still pass
@answer n=5 entity=established-decisions#4.1 v=no
why: send.rs loses only the Git Bash rewritten-path warning, and the live 2.1.287 round failed only statusline-rate-limits, so the cited confirm-window, paste-newline and local-command rows still pass
@answer n=6 entity=established-decisions#5.1 v=no
why: the ndjson append, heal and lock layout and the read counts are unchanged, no log field or event kind was added, and the Windows mention is historical rationale
@answer n=7 entity=established-decisions#6.1 v=no
why: the replace helpers and their rationale are unchanged, tempfile is not in the Cargo.lock delta, and CI is green on 849aad18
@answer n=8 entity=established-decisions#7.1 v=no
why: pin.rs loses only EXE_SUFFIX joins, the pinned copy keeps its path under bin/ (re-witnessed live), and CI is green on 849aad18
@answer n=9 entity=established-decisions#7.2 v=no
why: the placed hook and the exec-at-once test are not named by any change, and CI is green on 849aad18
@answer n=10 entity=established-decisions#7.3 v=no
why: the write-handle close ordering is not named by any change, and the pinned-copy placement is re-witnessed unchanged on the live Linux dev host
@answer n=11 entity=established-decisions#8.1 v=no
why: interprocess 2.4.4, the endpoint naming and the dialog and wait contract are unchanged (untouched record), and no verb, flag or event kind was added
@answer n=12 entity=established-decisions#9.1 v=no
why: viola-channel is in no reached file and the workspace crate list is unchanged
@answer n=13 entity=established-decisions#10.1 v=no
why: viola-channel is in no reached file and the peer-pid check is unchanged
@answer n=14 entity=established-decisions#10.2 v=no
why: viola-channel is in no reached file and the peer-euid admission is unchanged
@answer n=15 entity=established-decisions#11.1 v=no
why: the wrapper accept_loop lies in no reached file and the channel and socket naming are unchanged
@answer n=16 entity=established-decisions#12.1 v=no
why: viola_state::liveness is in no reached file and session liveness is unchanged (untouched record)
@answer n=17 entity=established-decisions#13.1 v=no
why: strict.rs is reached, but the socket directory is re-witnessed 0700 and the socket 0600 live and the socket-dir negative tests pass on the changed tree (CI green on 849aad18)
@answer n=18 entity=established-decisions#14.1 v=no
why: the snapshot envelope and the replay read are unchanged (untouched record), and no snapshot reader was changed
@answer n=19 entity=established-decisions#15.1 v=no
why: hook.rs is reached only for remnant removal, the statusline serve and its replay read are unchanged, and HELP is byte-identical
@answer n=20 entity=established-decisions#16.1 v=no
why: rmcp 3.4.1 and the MCP tool set are unchanged (untouched record), and no MCP source file is reached
@answer n=21 entity=established-decisions#17.1 v=no
why: the CappedLines stdin cap and the rmcp transport features are named by no change
@answer n=22 entity=established-decisions#18.1 v=no
why: the check_instance call before live_endpoint is named by no change, and the strict-modes refusals pass in CI green on 849aad18
@answer n=23 entity=established-decisions#18.2 v=no
why: result::unreachable and its causes are named by no change, and no refusal variant was added
@answer n=24 entity=established-decisions#19.1 v=no
why: the oversize cut and its single parse-rejected line are named by no change, and the mcp stdin reader is in no reached file
@answer n=25 entity=established-decisions#19.2 v=no
why: mcp_tools.rs is reached only for EXE_SUFFIX binary joins, and the named test and its exit-1 one-line assertion hold on the changed tree (CI green on 849aad18)
