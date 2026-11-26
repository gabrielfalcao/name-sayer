import click
import string
import sys
import os
from pathlib import Path
from collections import defaultdict
from itertools import groupby
from operator import itemgetter


def eprint(*message):
    sys.stderr.flush()
    for msg in message:
        sys.stderr.write(msg)
    sys.stderr.write('\n')
    sys.stderr.flush()

def throw(*message):
    eprint(*message)
    sys.exit(1)

def split_blocklist_file(filename: Path):
    alphabetic = defaultdict(list)
    parents, file = list(map(Path, os.path.split(filename)))
    base, extension = os.path.splitext(filename)
    extension = extension.strip('.')
    target_filenames = defaultdict(list)
    lines = filter(bool, [l.strip() for l in filename.open().readlines()])
    sorted_lines = sorted(lines, key=itemgetter(0))

    for key, group in groupby(sorted_lines, key=itemgetter(0)):
        group = sorted(group)
        # coded_key = hex(ord(key))
        coded_key = key
        alphabetic[key].append(group)
        new_filename = parents.joinpath(f'{base}.{coded_key}.{extension}')
        with new_filename.open('w') as fd:
            fd.write('\n'.join(group))
            eprint(f'wrote {new_filename}')

@click.command()
@click.argument("blocklist_file", required=True, type=Path)
def main(blocklist_file):
    blocklist_file = blocklist_file.absolute()
    if not blocklist_file.is_file():
        throw(f"{blocklist_file} is not a file")

    split_blocklist_file(blocklist_file)


if __name__ == "__main__":
    main()
