# EasyPot

A simple port-scanner / scraper honeypot built with rust!
Outputs all findings and any related data to the terminal and a log file.

# Build

Ensure that you have Rust installed, then build with `cargo build --release` after cloning. Then navigate to `/target/release` to run.

# Usage

Syntax is simple! Specify ports or port ranges (`4300`, `10000-10032`) in the command line. Works from there!

# Logging

All finding will be output to a log file in the same directory, numbered in order from 1.