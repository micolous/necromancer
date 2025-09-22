# necromancer

> [!WARNING]
> **This is still a work in progress.**
>
> Version 0.0.0 does nothing but reserve the package name on crates.io.
>
> A real release will follow soon... it's just taking a while! :)

`necromancer` will be a pure-Rust re-implementation of the Blackmagic Design ATEM control
protocol.

It is divided up into two crates, which will share a version number:

* [`necromancer`](./necromancer/): high-level client with state machine.
* [`necromancer_protocol`](./necromancer_protocol/): low-level data structures.
