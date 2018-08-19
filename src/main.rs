#![feature(rust_2018_preview, uniform_paths)]
//#![deny(warnings)]

mod common;
mod database;
mod errors;
mod forms;
mod server;

fn main() {
    server::serve();
}
