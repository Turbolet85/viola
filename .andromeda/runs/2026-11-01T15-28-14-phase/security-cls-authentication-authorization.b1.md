# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · password hashing: no accounts or passwords exist, so the chunk does not reach it
authentication-authorization#1.2 · yes · the OS check that gates the whole surface is the Unix peer euid and socket-directory checks this chunk builds
authentication-authorization#1.3 · yes · the Windows listener's SDDL and accept_remote(false) sit in the shared bind code the scope keeps as they stand
authentication-authorization#1.4 · yes · the Unix socket directory's 0700, owner and not-a-symlink check, the chmod 0600 after bind, and the ban on try_overwrite and the abstract namespace are the chunk's Needs
authentication-authorization#1.5 · yes · the wrapper's accept path drops a peer whose euid differs before a frame is read, named in the Needs
authentication-authorization#1.6 · yes · the Unix client check (peer euid equal to geteuid, before the first frame of any method) is the check this chunk adds; the Windows client open does not move
authentication-authorization#1.7 · no · the release-from-driver guard is a params check on a frame; the chunk changes no frame content and its new checks run before any frame
authentication-authorization#1.8 · yes · the Unix strict-modes rules of check_path and check_instance, and the home check of board::scan the chunk edits and whose 0750 split it must end; the file-creation modes are S9's and are not counted here
authentication-authorization#1.9 · yes · the home-tree strict-modes check at run and revive start, which the chunk applies on Unix, with run's refusal on a failed directory check as its named case
authentication-authorization#2.1 · yes · the Windows listener's remote-client rejection bit, a shared bind arm the scope keeps as it stands, which this witness proves on windows-2025
authentication-authorization#2.2 · yes · the protected DACL's effect (anonymous open denied), the SDDL the scope keeps as it stands in the shared bind code
authentication-authorization#3.1 · yes · the macOS carve-out: no peer pid and SERVER_PID_REPORTED false, which the chunk records; the Linux pid reading stays beside the Unix euid check
authentication-authorization#3.2 · yes · the verdict is Matches with no comparison where no peer pid is reported, the exact rule of the macOS carve-out
authentication-authorization#3.3 · yes · the no-endpoint causes instance-dead and unwrapped-name are values in the closed instance-unreachable detail set that the new refusal causes join
authentication-authorization#3.4 · yes · link and unlink run the Unix client euid check before their one frame, after the strict-modes check, which the chunk adds to every Unix client
authentication-authorization#3.5 · yes · the CLI frame verbs' exit 21 and server-verify-failed document is the shape a peer euid mismatch takes on the same surface
authentication-authorization#3.6 · yes · the MCP frame's mapping of a failed server check to instance-unreachable with no frame or line is the surface a Unix euid mismatch takes in viola-mcp
authentication-authorization#3.7 · no · the exiting Windows wrapper's pipe is a Windows-only case; the Windows arms do not move and its CARRY sits in another route entry
authentication-authorization#3.8 · yes · the macOS 0755 socket-directory case with exit 21, plus the euid check and directory refusal code, are owed to this chunk
authentication-authorization#3.9 · yes · the link and unlink refusal tests on a widened instance and on an impostor server must keep holding under the chunk's home check and client euid check
