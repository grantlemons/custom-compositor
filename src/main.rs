#[allow(unused_imports)]
use smithay::{
    reexports::{
        calloop::{EventLoop, Interest, Mode, PostAction},
        wayland_server::Display,
    },
    wayland::socket::ListeningSocketSource,
};

mod config;

fn main() {
    println!("Hello, world!");
}
