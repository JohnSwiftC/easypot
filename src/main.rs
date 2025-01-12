use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut ports: Vec<i32> = Vec::new();

    if args.len() == 1 {
        panic!("Specify ports or port ranges!");
    }

    for arg in args {
        let port = arg.trim().parse();

        match port {
            Ok(p) => {
                ports.push(p);
                continue;
            },
            Err(_) => (),
        }

    }

    let (left, right) = get_ports_from_range(&String::from("829-62911"));

    println!("{} {}", left, right);
}

fn get_ports_from_range(s: &str) -> (i32, i32) {
    let mut left = 0;
    let mut right = 0;

    let mut port1: i32 = -1;
    let mut port2: i32 = -1;

    for c in s.chars() {
        if c == '-' {
            port1 = s[left..right].parse().expect("Bad port range.");
            right += 1;
            left = right;
        } else {
            right += 1;
        }
    }

    port2 = s[left..right].parse().expect("Bad port range.");

    (port1, port2)

}