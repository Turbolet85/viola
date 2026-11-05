@answer n=1 entity=secret-management#1.1 v=yes
why: the R8 persistent-set rule whose Windows registry reader the chunk removes (task item 6), and the Unix config.json branch it keeps
@answer n=2 entity=secret-management#2.1 v=no standing=S3
why: every test runs on its own --home and every pre-push child runs under env -i with exactly HOME and PATH, the convention S3 lists
