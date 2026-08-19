//! Mob spawn - inbound `0x03C6`, `MobEnterField`.
//!
//! Stub. Routing and the 11-byte head are read in `research/mob-spawn.md`; what is missing
//! is the movement-path framing inside `FUN_14046fba0`. This module is where the builder
//! goes, kept out of `opcode.rs` so the character-record work cannot collide with it.
