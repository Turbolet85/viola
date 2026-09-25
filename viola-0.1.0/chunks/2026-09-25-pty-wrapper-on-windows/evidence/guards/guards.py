"""Step 12 remove-the-guard runs: neutralise one guard, run its witness, restore the exact bytes.

Each case: (id, file, anchor, neutralised text, witness command). The anchor must match exactly once;
the restore is checked byte-for-byte before the next case runs. Readings go to the chunk's evidence/.
"""
import json
import pathlib
import subprocess
import sys
import time

ROOT = pathlib.Path('D:/dev/projects/viola')
EVIDENCE = ROOT / 'viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/evidence/guards'
EVIDENCE.mkdir(parents=True, exist_ok=True)
BASH = r'C:\Program Files\Git\bin\bash.exe'

CASES = [
    ('batch-refusal', 'crates/viola-agent-claude/src/lib.rs',
     '        _ => Err(Refusal::BatchScriptChild),',
     '        _ => Ok(found),',
     'cargo test -q -p viola-agent-claude resolve_program_refuses'),
    ('identity-floor', 'crates/viola-agent-claude/src/lib.rs',
     '        if !floor && persistent.contains(&k) {',
     '        if persistent.contains(&k) {',
     'cargo test -q -p viola-agent-claude plan_strip_strips_a_floor_name'),
    ('claude-prefix', 'crates/viola-agent-claude/src/lib.rs',
     '        if !k.starts_with("CLAUDE") {\n            continue;\n        }',
     '        if k.is_empty() {\n            continue;\n        }',
     'cargo test -q -p viola-agent-claude plan_strip_removes_identity'),
    ('exit-on-handle', 'crates/viola-pty/src/lib.rs',
     '            Ok(Worker::OutputDone) => output_done = true,',
     '            Ok(Worker::OutputDone) => {\n                return Ok(PumpEnd::Exited(Exit { code: None, source: ExitSource::HandleWait }));\n            }',
     'cargo test -q -p viola-pty pump_waits_for_the_handle_after_the_output_ends'),
    ('child-cwd', 'crates/viola-pty/src/lib.rs',
     '    cmd.cwd(&spec.cwd);\n',
     '',
     'cargo test -q --features fake-agent --test tui_passthrough tui_keys_reach'),
    ('raw-input-flags', 'crates/viola-pty/src/lib.rs',
     '    (mode & !COOKED_INPUT) | ENABLE_VIRTUAL_TERMINAL_INPUT',
     '    mode | ENABLE_VIRTUAL_TERMINAL_INPUT',
     'cargo test -q -p viola-pty spawn_runs_a_raw_child'),
    ('terminal-restore', 'crates/viola-pty/src/lib.rs',
     '                windows_sys::Win32::System::Console::SetConsoleMode(*handle, *mode);',
     '                let _ = (handle, mode);',
     'cargo test -q -p viola-pty spawn_runs_a_raw_child'),
    ('keep-child-input', 'crates/viola-pty/src/lib.rs',
     '        let _ = keep.send(writer);',
     '        drop(writer);\n        let _ = &keep;',
     'cargo test -q -p viola-pty pump_keeps_the_child_input_open'),
    ('ctrl-c-resend', 'crates/viola-e2e/src/harness/supervise.rs',
     '        if expired(next_ctrl_c) {\n            ctrl_c(input);',
     '        if expired(next_ctrl_c) && false {\n            ctrl_c(input);',
     'cargo test -q -p viola-e2e --lib stop_presses_ctrl_c_again'),
    ('ripgrep-sha', 'scripts/install-ripgrep.sh',
     '  if [ "$actual" != "$2" ]; then',
     '  if false; then',
     'bash scripts/install-ripgrep.sh --probe'),
    ('orphans-every-triple', 'scripts/orphans-check.sh',
     '      common=$(comm -12 <(printf \'%s\\n\' "$common") <(printf \'%s\\n\' "$found") | grep -v \'^$\' || true)',
     '      common=$(printf \'%s\\n%s\\n\' "$common" "$found" | sort -u | grep -v \'^$\' || true)',
     'bash scripts/orphans-check.sh --probe'),
]

only = set(sys.argv[1:])
results = []
for cid, rel, anchor, neutral, cmd in CASES:
    if only and cid not in only:
        continue
    path = ROOT / rel
    original = path.read_bytes()
    text = original.decode('utf-8')
    count = text.count(anchor)
    if count != 1:
        results.append({'id': cid, 'error': f'anchor matched {count} times'})
        print(cid, 'ANCHOR', count, flush=True)
        continue
    path.write_bytes(text.replace(anchor, neutral, 1).encode('utf-8'))
    assert path.read_bytes() != original, 'neutralising edit did not land'
    started = time.time()
    run = subprocess.run([BASH, '-o', 'pipefail', '-c', cmd], cwd=ROOT, capture_output=True,
                         text=True, encoding='utf-8', errors='replace', timeout=900)
    path.write_bytes(original)
    assert path.read_bytes() == original, 'restore failed'
    log = EVIDENCE / f'{cid}.neutralised.log'
    log.write_text(run.stdout + run.stderr, encoding='utf-8')
    row = {'id': cid, 'file': rel, 'witness': cmd, 'neutralised_exit': run.returncode,
           'red': run.returncode != 0, 'seconds': round(time.time() - started, 1),
           'log': log.name}
    results.append(row)
    print(json.dumps(row), flush=True)

out = EVIDENCE / 'readings.json'
prev = json.loads(out.read_text(encoding='utf-8')) if out.exists() else []
keep = [r for r in prev if r['id'] not in {r2['id'] for r2 in results}]
out.write_text(json.dumps(keep + results, indent=1), encoding='utf-8')
