from pathlib import Path

root = Path(__file__).resolve().parents[1]
required = [
    root / 'README.md',
    root / 'README_TH.md',
    root / 'docs' / 'README.md',
    root / 'docs' / 'th' / 'README.md',
]
required += list((root / 'docs').glob('*.md'))

missing = [str(path.relative_to(root)) for path in required if not path.exists()]
if missing:
    raise SystemExit('Missing files: ' + ', '.join(missing))

print(f'OK: {len(required)} required documentation files found')
