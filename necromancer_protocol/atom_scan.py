#!/usr/bin/env python3

import os
from os.path import abspath, dirname, join
from re import compile

_ATOM_NAME = compile(r"BEPAtom(.+)10Initialise")
_BASE_PATH = dirname(abspath(__name__))
_ATOMS_TXT = join(_BASE_PATH, "atoms.txt")
_SOURCE_ATOM_PATH = join(_BASE_PATH, "src", "atom")
_MARKDOWN_CODE_BLOCK = compile(r"//[^\n]+`([^`\n]{5,})`")

re_atoms: set[str] = set()
with open(_ATOMS_TXT, "rt") as f:
    # Read in a list of Initialise functions from BMDSwitcherAPI
    for line in f:
        line = line.strip()
        if line.startswith("#") or "Initialise" not in line:
            continue

        m = _ATOM_NAME.search(line)
        assert m is not None

        re_atoms.add(m.group(1))

print(f"Atoms from function list: ({len(re_atoms)})")

# Read in a list of potential atom names in the source code.
source_atoms = set()

for dirpath, dirnames, filenames in os.walk(_SOURCE_ATOM_PATH):
    for fn in filenames:
        if not fn.endswith(".rs"):
            continue

        # Read in things that look like tokens
        with open(join(dirpath, fn), "rt") as f:
            b = f.read()
            for block in _MARKDOWN_CODE_BLOCK.finditer(b):
                source_atoms.add(block.group(1))

print(f"Atom-like references in source code: ({len(source_atoms)})")

# Find re_atoms that are not in source_atoms
missing_atoms = re_atoms.difference(source_atoms)
print(f"Missing atoms: ({len(missing_atoms)})")
print(sorted(list(missing_atoms)))
