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
    lines = [l.strip() for l in filename.open().readlines()]

@click.command()
@click.argument("blocklist_file", required=True, type=Path)
def main(blocklist_file):
    blocklist_file = blocklist_file.absolute()
    if not blocklist_file.is_file():
        throw(f"{blocklist_file} is not a file")

    split_blocklist_file(blocklist_file)


if __name__ == "__main__":
    main()
