# Review feedback 2 (overseer, founder-delegated)

Confirm the plugin folder keyed by <version>-<hash>. For unconfirmable, use (a), keyed on the ledger, never on a leading slash. Skills are expanded into a prompt and fire UserPromptSubmit like any prompt (measured today: /andromeda-arch did), so a slash send that is not on the ledger's local-command list is an ordinary send: unconfirmed means not-delivered. The ledger lists known local commands, each with its post-condition or "none". Measured post-condition met: ok, confirmed. Post-condition "none" or not yet measured on this CLI version (/remote-control today): ok {confirmed:false, detail:unconfirmable}. Nothing else is ever unconfirmable.

## Context: option (a) as offered
(a) an unknown local command is `ok` with a caveat, e.g. `{"ok":{"confirmed":false,"detail":"unconfirmable"}}`, and anything else unconfirmed stays `not-delivered`.
