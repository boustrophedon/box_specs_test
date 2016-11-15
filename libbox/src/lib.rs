extern crate specs;
extern crate time;
#[cfg(feature = "client")]
extern crate glium;

mod common;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
