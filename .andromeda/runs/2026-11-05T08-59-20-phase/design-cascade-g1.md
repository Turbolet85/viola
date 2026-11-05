@carried n=1 label=design-system/brand-identity fact=fct-e37xe7yc
why: The removal of the Windows VT arm from the SGR module and of the Windows arms from the `run` / `revive` path must leave every one of these conditions printing no colour, no glyph, no cursor control and no spinner on Linux, and `viola run` / a passed `viola revive` printing nothing while the child runs, because the scope re-asserts that no Linux behaviour moves.
quote: | cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
@carried n=1 run=2 label=design-system fact=fct-4m7ag2lh
quote: viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
@carried n=1 run=3 label=design-system fact=fct-jphnedpy
quote: **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
@carried n=2 label=design-system fact=fct-tmafcfek
why: The design-system amendment for a Windows terminal behaviour is expected and lands in the live amendments file with its sidecar entry carrying the founder's ruling, which the chunk's plan must name as written by the wrap.
quote: History: design-system-amendments.md (live); the log moved verbatim to design-system-amendments-archive.md (U35).
@binding n=1
**brand-identity's 0.0 expression row ↔ the cli Toolkit and colour decision order (body, surface)**: the 0.0 row's no-colour outcome under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` and `viola run` (design-system/brand-identity fct-e37xe7yc) must keep being produced by the decision order's remaining steps once its Windows step and the Windows VT arm leave (design-system fct-w37zqv3v) (design-system fct-2wyyc34q), with the 256-colour depth rule losing only its Windows clause (design-system fct-vzhczpnh).
@binding n=2
**the cli Platform and Platform-Specific Notes ↔ architecture's D3 and the Design Decisions Log**: the Linux-only OS target lands as one amendment set with the code (design-system fct-7z5o76y5) (design-system fct-jphpatxv) (design-system fct-7u3hdiu2), its design-system entry in the live amendments file carrying the founder's ruling (design-system fct-tmafcfek), written by the wrap and named by this chunk's plan as expected.
@binding n=3
**brand-identity's `viola run` passthrough ↔ the `run` / `revive` start refusals (body, surface)**: removing the ConPTY and `.cmd` / `.bat` arms from the start path must keep `run` and a passed `revive` silent while the child runs (design-system/brand-identity fct-e37xe7yc) and leave the closed start-refusal codes unchanged on Linux (design-system fct-2t54hjhs).
