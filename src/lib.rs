#![recursion_limit = "1024"]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(not(feature = "clippy"), allow(unknown_lints))]
#![feature(exact_size_is_empty)]
#![feature(drain_filter)]
#![feature(hashmap_internals)]
#![feature(fnbox)]
#![feature(integer_atomics)]
#![feature(vec_remove_item)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate byteorder;
extern crate bytes;
extern crate crc;
extern crate crossbeam;
extern crate fasthash;
extern crate flate2;
extern crate memmap;
extern crate num_traits;
extern crate smallvec;
extern crate thread_local;
extern crate unicode_reader;

pub mod core;
pub mod error;
