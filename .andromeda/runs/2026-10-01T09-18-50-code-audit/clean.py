"""Exit housekeeping: delete THIS run's raw tool output (size discipline). Scope: paths inside this run dir only —
every target is resolved and refused unless it sits strictly under the run dir."""
import glob, os, shutil

RUN = os.path.realpath(os.path.dirname(os.path.abspath(__file__)))
DIRS = ['jscpd', 'rca', '__pycache__'] + [os.path.basename(p) for p in glob.glob(os.path.join(RUN, 'mutants-*')) if os.path.isdir(p)]
FILES = [os.path.basename(p) for p in glob.glob(os.path.join(RUN, 'mutants-*.std*.txt'))] + [
    'tokei.json', 'tokei-err.txt', 'jscpd-stdout.txt', 'rca-stdout.txt', 'zero-ref-raw.json', 'machete.txt',
    'cov-stdout.txt', 'cov-stderr.txt', 'cov-exit.txt', 'cov-run1-stdout.txt', 'cov-run1-stderr.txt', 'cov-run1-exit.txt']
done = []
for name in DIRS + FILES:
    p = os.path.realpath(os.path.join(RUN, name))
    if os.path.dirname(p) != RUN:
        raise SystemExit(f'refused: {p} is not directly inside the run dir')
    if os.path.isdir(p):
        shutil.rmtree(p); done.append(name + '/')
    elif os.path.isfile(p):
        os.remove(p); done.append(name)
print(len(done), 'removed:', ' '.join(sorted(done)))
