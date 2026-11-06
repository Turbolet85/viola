@answer n=1 entity=authentication-authorization#2.10 v=yes mode=stale
why: the session reached tests/security_negatives_socket_dir.rs, which this entity cites, and the report does not show its refusal claim holds on the changed file
@answer n=2 entity=authentication-authorization#2.11 v=no
why: the claim sits in viola_channel::Client and tests/channel_links.rs, neither of which the session reached; the reached src/cmd/client.rs is the CLI client module, not that channel client
