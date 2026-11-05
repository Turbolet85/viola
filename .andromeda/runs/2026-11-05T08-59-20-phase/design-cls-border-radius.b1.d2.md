@answer n=1 entity=border-radius#1.1 v=no
why: the radius-sm 0px token is a frontend value for the deferred 0.2.0 web surface; the chunk's terminal code, tests and CI read no radius value
@answer n=2 entity=border-radius#1.2 v=no
why: the radius-md 0px token is a frontend value for the deferred 0.2.0 web surface; the chunk renders no radius and the removal reaches none
@answer n=3 entity=border-radius#1.3 v=no
why: the radius-lg 0px token is a frontend value for the deferred 0.2.0 web surface; the chunk renders no radius and the removal reaches none
@answer n=4 entity=border-radius#1.4 v=no
why: the radius-full row only bans a 9999px value and marks it unused; a frontend design-intent rule the Windows and macOS removal does not touch
