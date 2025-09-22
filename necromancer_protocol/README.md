# necromancer_protocol &emsp; [![Latest Version]][crates.io] [![Docs version]][docs.rs]

`necromancer_protocol` implements low-level data structures for Blackmagic Design ATEM video
switchers' control protocol.

**WARNING: This is still a work in progress.**

This needs a client implementation on top of this code to manage state, which will be published
soon.

## Cargo features

* `clap`: Adds clap `ValueEnum` derive macros to some enums.
* `palette`: Adds helpers for using `palette`, enabled by default.
* `serde`: Adds Serde serialisation and deserialisation derive macros to some types.

[crates.io]: https://crates.io/crates/necromancer_protocol
[Docs version]: https://img.shields.io/docsrs/necromancer_protocol.svg
[docs.rs]: https://docs.rs/necromancer_protocol/
[Latest Version]: https://img.shields.io/crates/v/necromancer_protocol.svg
