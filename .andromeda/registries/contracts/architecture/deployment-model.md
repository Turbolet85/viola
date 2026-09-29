**Deployment model**
- Local-only. There is no Docker, Compose, Kubernetes or serverless.
- The single `viola` binary is installed on PATH. `viola run <name> -- claude <args>` pins a copy of itself in `~/.viola/bin/<version>-<hash>/` (on Windows x64 also the embedded ConPTY companions in its `conpty/` subdirectory, with `conpty.dll` pre-loaded from there before the spawn), writes the embedded plugin (pointing at that copy) to `~/.viola/plugin/<version>-<hash>/` and launches the child with `--plugin-dir`. `viola ui` is started by hand and serves 127.0.0.1.
