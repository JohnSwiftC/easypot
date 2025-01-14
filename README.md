# EasyPot

A simple port-scanner / scraper honeypot built with Rust!
Outputs all findings and any related data to the terminal and a log file.

# Build

Ensure that you have Rust installed!

> You may also need to update your rust with `rustup update`

Run the following commands (or don't, you probably know what you're doing.)

`git clone https://github.com/JohnSwiftC/easypot.git`
`cd easypot`
`cargo build --release`
`cd target/release`

You will see the executable in that file. Alternatively, do `cargo run -- *args*` to run without building.

# Usage

Syntax is simple! Specify ports or port ranges (`4300`, `10000-10032`) in the command line. Works from there!

# Logging

All findings will be output to a log file in the same directory, numbered in order from 1.