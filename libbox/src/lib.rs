extern crate specs;
extern crate time;
#[cfg(feature = "client")]
#[macro_use] extern crate glium;
extern crate nalgebra;
extern crate ncollide;

extern crate rustc_serialize;

mod common;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
