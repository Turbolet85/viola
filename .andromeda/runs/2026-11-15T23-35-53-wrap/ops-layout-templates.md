@row_add n=1 row=decision/api-error-turn-end-reads-turn-ended area=surface-cli fact=F1 detector=D-design-cli-wording
name: API-error turn end reads as turn-ended
rule: A turn the CLI ends on an API error is returned by viola wait and the MCP wait tool as turn-ended with exit 0, as a Stop end is; the --json and MCP data carry only the closed cause word api-error, and no error text or error class ever reaches output.
why: The error text crosses into nothing; the cause is a closed set, the most conservative option of the open fork.
state: provisional
basis: fct-xbqeduib, fct-vkofdjrk, fct-klhffiim, fct-tf4ifap3
because: The chunk adds the API-error turn end with a closed cause word on wait --json and MCP wait while the human turn-ended line stays unchanged, and no row of the cli surface states that this end reads as turn-ended or that no error text reaches output.
@row_none n=1 fact=F2 type=4
relied: An API-error turn end clears the turn as a Stop does, so the board reads STATUS idle and the next send is accepted with no pause or release, under the existing board and send wording.
