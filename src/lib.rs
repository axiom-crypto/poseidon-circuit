//! Native Poseidon implementation.
//! Forked from Scroll's Poseidon Halo2 circuit library to update `ff` to `v0.13`.

#![feature(slice_group_by)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![feature(trait_alias)]

//pub mod hash;
pub mod poseidon;

//pub use halo2_proofs::halo2curves::bn256::Fr as Bn256Fr;
//pub use hash::{Hashable, HASHABLE_DOMAIN_SPEC};

/// a default step can be compatible with codehash circuit
pub const DEFAULT_STEP: usize = 62;
