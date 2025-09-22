# Decompiling notes

> [!TIP]
> If you want to decompile device firmware, [`blackmagic-firmware-re`][0] could help.

`BMDSwicherAPI` is the main library for controlling ATEM switches. It is available for macOS on
`aarch64` and `x86_64`, and for Windows on `x86_64`:

- macOS: `/Library/Application Support/Blackmagic Design/Switchers/BMDSwitcherAPI.bundle/Contents/MacOS/BMDSwitcherAPI` (fat binary)
- Windows: `${PROGRRAMFILES(X86)}/Blackmagic Design/Blackmagic ATEM Switchers/BMDSwitcherAPI64.dll` (`x86_64`)

I normally work with the binary in Ghidra. "Auto-analyse" picks up a lot of the program's structure,
but will need some help/corrections.

## `BEPAtom` classes

The ATEM wire protocol atoms (commands / messages) are in classes with names starting with
`BEPAtom`.

The first field of an atom is an 8-byte [`BEPAtomHeader`](#bepatomheader), which indicates the
atom's length and type.

Atoms are in host byte order (normally little-endian) inside of `BMDSwitcherAPI`, and big-endian
when over the network. [`PrepareForHost()` and `PrepareForNetwork()`](#standard-methods) convert
endianness.

Structs are _not_ normally packed, so are word-aligned and contain lots of padding.

Both the library and ATEM switches generally do not initialise memory (ie: `memset(0)`), so padding
may contain arbitrary data. This can be a problem when testing an atom round-trip decode/encodes.

### `BEPAtomHeader`

All atoms have a standard header (expressed here as a C/C++ `struct`):

```cpp
#include <stdint.h>

struct BEPAtomHeader {
    // 0x00: Length of the atom, including this header.
    // If this is set to 8, then the atom has no payload.
    uint16_t length;

    // 0x02: Padding to word-align the fourcc field.
    uint16_t _padding;

    // 0x04: Atom type identifier, normally printable ASCII.
    uint32_t fourcc;

    // 0x08
};

static_assert(sizeof(struct BEPAtomHeader) == 8, "");
```

### Example atom structure (`BEPAtomDoTransitionCut`)

A typical atom's structure looks something like:

```cpp
struct BEPAtomDoTransitionCut {
    // 0x00: atom header.
    // { length = 0xC, fourcc = 0x44437574 (DCut) }
    struct BEPAtomHeader header;

    // 0x08: mix effect block to issue the cut command to.
    uint8_t me;
    
    // 0x09: padding to word-align the structure.
    undefined3 _padding;
}

static_assert(sizeof(struct BEPAtomDoTransitionCut) == 12, "");

// 000c00004443757400000000 => BEPAtomDoTransitionCut { me = 0 }
```

> [!TIP]
> The Blackmagic SDK _also_ uses FourCCs to identify switcher events, which may not match the FourCC
> used in the wire protocol.

### Standard methods

<dl>

<dt>

`BEPAtom*::Initialise()`

</dt>

<dd>

Sets the length of the atom. Variable-length structures may take an additional parameter with the
number of entries.

This lets you put a name to a FourCC.

</dd>

<dt>

`BEPAtom*::GetCreateSize()`

</dt>

<dd>

Get the length of the atom.

Like `Initialise()`, variable-length structures may take an additional parameter with the number of
entries.

</dd>

<dt>

`BEPAtom*::Coalesce(other)`

</dt>

<dd>

If `this` and `other` refer to the same entity (eg: mixer channel), update `this` with the values
from `other`.
  
This can be useful to figure out which memory in the structure is _actually used_ (and is not
padding), but sometimes the library just uses `memcpy()`.

</dd>

<dt>

`BEPAtom*::PrepareForHost()`, `BEPAtom*::PrepareForNetwork()`

</dt>

<dd>

Convert device byte order (big-endian) and structure layout to host byte order (normally
little-endian) and structure layout, and the reverse.

`PrepareForHost()` and `PrepareForNetwork()` are normally identical.

Endian flip operations can be hard to read in the decompiler output (lots of bitwise operations).
In the disassembler output, they're normally just a `bswap`/`rol` (`x86`) or `rev` (`aarch64`)
instruction; which should also hint about how wide the field is.

Cross-references to `PrepareForHost()` indicate the atom is recieved from the ATEM,
`PrepareForNetwork()` indicates the atom is sent to the ATEM.

</dd>

</dl>

### Field getters and setters

<dl>

<dt>

`BEPAtom*::GetPtr_{name}()`

</dt>

<dd>

Get a pointer to the first entry in an array field `{name}`.

</dd>

<dt>

`BEPAtom*::GetLen_{name}()`

</dt>

<dd>

Get the number of entries in the array field `{name}`.

</dd>

<dt>

`BEPAtom*::GetString_{name}(char* out, uint64_t len)`

</dt>

<dd>

Get the string field `{name}` as a null-terminated string. Present where a command contains a
string.

The function will normally contain a maximum length check.

</dd>

<dt>

`BEPAtom*::SetString_{name}(char* in)`

</dt>

<dd>

Set the string field `{name}` to the null-terminated string `in`.

</dd>

</dl>

### `BEPStruct`

`BEPAtom*` can contain `BEPStruct*` fields. This is used for array fields of complex types.

They contain similarly-named standard methods to `BEPAtom*` classes (`PrepareFor`, `GetPtr_`,
`GetLen_`, `GetString_`, `SetString_`).

[0]: https://github.com/micolous/blackmagic-firmware-re/

## `CBMDSwitcher`

This is the main class of the API, which implements the `IBMDSwitcher` interface, and holds many
utility functions.

There are other `CBMDSwitcher{entity}` clases which implement `IBMDSwitcher{entity}` interfaces.

## Version variants

`BMDSwitcher` has a bunch of alternative versions of classes named like `CBMDSwitcher_v4_0`. These
are thin wrappers around the latest version of the ABI.

Applications call into the library requesting an object instance by an IID encoded in the IDL via
COM (on Windows) or calling `CreateBMDSwitcherDiscovery()` provided in the SDK's header files (on
macOS).

In this way, the library attempts to retain binary compatibility with applications built against
older versions of the SDK.

The library instanciates objects for _all_ versions of the API.

However, this does not extend to communication with the hardware: `BMDSwitcherAPI` only supports
**one** version of the wire protocol.
