@verdict n=1 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=14 v=contradiction fact=fct-vtox22ts
quote: This must hold on `[windows-2025, macos-latest, ubuntu-latest]`
why: the cli surface still binds its a11y checks to three OSes (windows-2025, macos-latest, ubuntu-latest) and names NVDA/VoiceOver as manual targets, where the proposal states Linux only with CI on ubuntu-latest and the Windows and macOS code removed
