#!/usr/bin/env python3
"""Scan the `necromancer_protocol` source code for unknown atoms.

`atoms.txt` is a dump of `Initialise` functions from `BMDSwitcherAPI`.
"""

import os
from os.path import abspath, dirname, join
from re import compile

_ATOM_NAME = compile(r"BEPAtom(.+)10Initialise")
_BASE_PATH = dirname(abspath(__name__))
_ATOMS_TXT = join(_BASE_PATH, "atoms.txt")
_SOURCE_ATOM_PATH = join(_BASE_PATH, "src", "atom")
_MARKDOWN_CODE_BLOCK = compile(r"//[^\n]+`([^`\n]{7,})`")
_UNKNOWN_ATOM = compile(r"\| `([^`\n]{7,})` \|")

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

print(f"{len(re_atoms)} atoms in function list")

# Read in a list of potential atom names in the source code.
source_atoms: set[str] = set()
unimplemented_atoms: set[str] = set()
sources: int = 0

print("Parsing source code for atom references...")
for dirpath, dirnames, filenames in os.walk(_SOURCE_ATOM_PATH):
    for fn in filenames:
        if not fn.endswith(".rs"):
            continue

        # Read in things that look like tokens
        with open(join(dirpath, fn), "rt") as f:
            sources += 1
            b = f.read()
            for block in _MARKDOWN_CODE_BLOCK.finditer(b):
                source_atoms.add(block.group(1))
            for atom in _UNKNOWN_ATOM.finditer(b):
                unimplemented_atoms.add(atom.group(1))

print(f"Read {sources} source files")
print(f"{len(source_atoms)} atom-like references in source code")
print(f"{len(unimplemented_atoms)} unimplemented atoms:")
print(sorted(list(unimplemented_atoms)))

# Find re_atoms that are not in source_atoms
unreferenced_atoms = re_atoms.difference(source_atoms)
print(f"{len(unreferenced_atoms)} unreferenced atoms:")
print(sorted(list(unreferenced_atoms)))
