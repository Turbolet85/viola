@fact id=fct-irhgihut expect_rev=3
old: │   ├── conpty.rs               # Windows x64: the embedded ConPTY companions and their four pins (the vendor script parses this text)
new: │   ├── (conpty.rs)             # removed with the Windows code; recoverable at the tag multi-os-final
old: #   `sideload` (Windows): the System32 DLL-search restriction + the absolute-path conpty.dll pre-load)
new: #   (`sideload`, the Windows DLL-search module: removed; recoverable at the tag multi-os-final))
old: │   ├── conpty-vendor.sh        # re-vendor vendor/conpty/ from the pinned nupkg (+ --verify: sha256 + byte compare + signer; --probe)
new: │   ├── (conpty-vendor.sh)      # removed with the Windows code; recoverable at the tag multi-os-final
old: ├── vendor/conpty/<version>/x64/ # the committed Microsoft conpty.dll + OpenConsole.exe (binary per .gitattributes)
new: ├── (vendor/conpty/)           # removed with the Windows code; recoverable at the tag multi-os-final
old: │       └── windows-mutants.yml # workflow_dispatch only (no inputs), dispatched at the boundary audit, never a gate:
new: │       └── (windows-mutants.yml) # removed with the Windows code; recoverable at the tag multi-os-final:
old: #   windows-2025 run --mutants --package … --file … per labelled matrix item, over that
new: #   the boundary audit's Windows leg; no workflow replaces it,
old: #   item's cfg(windows) files (the root package split by file)
new: #   and the audit runs `run --mutants` on Linux alone
