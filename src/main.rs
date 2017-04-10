#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};
use std::env;

const DEF_ADDR: &'static str = "0.0.0.0";
const PORT_KEY: &'static str = "PORT";
const ALT_PORT: &'static str = "1705";

fn main() {
    // Setting Up

    let mut server = Nickel::new();

    // Get, Posts, ...

    server.get("/", middleware!("Hello, World!"));

    // Starting The Server

    let addr: &str = &format!("{}:{}",
        DEF_ADDR, env::var(PORT_KEY).unwrap_or(ALT_PORT.to_string()));

    server.listen(addr);
}
