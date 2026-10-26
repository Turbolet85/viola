# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the session adds no process-start field (§8 untouched) and the link and unlink process-start lines in `cli-<driven>` carry the listed fields
log-coverage#1.2 · no · the session adds no process-exit field (§8 untouched) and the new server-verify-failed line uses the listed `exit_code` and `during:connect`
log-coverage#1.3 · no · the request row holds because the pair carries `method:"link"|"unlink"` with no names (F3), and F7 concerns only the conn-less channel-response line
log-coverage#1.4 · yes · add-to · F7: an unserved method's conn-less channel-response line carries no `method` key, because the channel logs `method` only for names on its closed method-label list
log-coverage#1.5 · no · the change serves link and unlink only, so no send path or send field changed and §8 adds no field
log-coverage#1.6 · no · no dialog path or dialog field changed and the hook path is untouched (§10)
log-coverage#1.7 · no · no hook path changed (§10 untouched), so the hook-invoked and hook-decision fields hold
log-coverage#1.8 · no · no panic path or hook changed (§7 untouched)
log-coverage#1.9 · no · the session changes no release path and link and unlink read no `from` (F2)
log-coverage#1.10 · no · no state-recovered field changed (§8) and the unlink event append uses the existing append, whose torn-line heal the row already lists
log-coverage#1.11 · no · no parser or detail added and a bad link or unlink name is answered `-32602` (F2), not a parser rejection
log-coverage#2.1 · no · the change adds no exit-1 refusal or detail, a refused or other-shape ok maps to the listed `internal-error`, and the 13-value detail enum is unchanged (§8)
log-coverage#2.2 · yes · stale · the six CLI frame verbs become eight, since link and unlink also write `process-exit` exit 21 `server-verify-failed` at WARN into `cli-<driven>` on a server mismatch
log-coverage#2.3 · no · link and unlink wrapper errors, `-32602` included, reach exit 20 with the existing `wrapper-fault` detail and no new exit-20 detail is added (Touches: exit classes reach existing levels)
log-coverage#2.4 · no · no hook path changed (§10 untouched), so the fail-open codes stand
log-coverage#2.5 · no · no parser or detail added and link and unlink name refusals are `-32602` channel errors
log-coverage#2.6 · no · the new link and unlink refusals are `-32602` channel errors, not RefusalReason values, so the refusal-detail list is unchanged
log-coverage#2.7 · yes · add-to · F5: link and unlink's refused driven-instance check exits 21 with no line in any log, so they join this list of no-line strict-modes refusals
log-coverage#3.1 · no · link and unlink spawn no child and run no `claude agents` read, and the agents-probe and statusline-shell writers are unchanged (§4 statusline and budget code untouched)
