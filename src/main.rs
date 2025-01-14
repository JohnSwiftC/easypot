use std::{
    env,
    thread,
    sync::mpsc,
    ops::RangeInclusive,
    net::TcpListener,
    io::{BufReader, BufRead, Write, ErrorKind, Error},
    fs::File,
    fmt::Display,
};

fn main() {
    
    let ports: Vec<i32> = get_ports_from_input();
    let listeners: Vec<TcpListener> = bind_to_ports(ports);
    let (tx, rx) = mpsc::channel::<String>();

    create_threads(listeners, tx);

    let file = make_log_file();

    match file {
        Ok(f) => {
            read_rx_file(rx, f);
        },
        Err(_) => {
            println!("Failed to create logging file, logging disabled.");
            read_rx_no_file(rx);
        },
    }

}

fn make_log_file() -> Result<File, Error> {
    let mut log_iter = 1;
    loop {
        match File::create_new(format!("easypotlog-{}.txt", log_iter)) {
            Ok(f) => {
                return Ok(f);
            },
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    log_iter += 1;
                    continue;
                } else {
                    return Err(e);
                }
            },
        } 
    }
}

fn create_threads(listeners: Vec<TcpListener>, tx: mpsc::Sender<String>) {
    for listener in listeners {
        let txclone = tx.clone();
        thread::spawn(move || {

            let port = listener.local_addr().unwrap().port();

            for stream in listener.incoming() {

                let stream = match stream {
                    Ok(s) => s,
                    Err(_) => {
                        continue;
                    },
                };

                let buf_reader = BufReader::new(&stream);

                let data: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap_or_else(|_err| String::from("Bad data, likely a port scanner.")))
                    .take_while(|line| !line.is_empty())
                    .collect();

                let _ = txclone.send(format!("Port: {}\n{data:#?}", port));

            }

        });
    }
}

fn read_rx_no_file(rx: mpsc::Receiver<String>) {
    for message in rx {
        println!("{}", message);
    }
}

fn read_rx_file<T: Display>(rx: mpsc::Receiver<T>, mut file: File) {
    for message in rx {
        println!("{}", message);

        let res = file.write_all(
            format!("{}\n\n", message).as_bytes()
        );

        if let Err(_) = res {
            println!("Failed to write a line to file!");
        }
    }
}

fn get_ports_from_input() -> Vec<i32> {

    let args: Vec<String> = env::args().skip(1).collect();
    let mut ports: Vec<i32> = Vec::new();

    if args.len() == 0 {
        panic!("Specify ports or port ranges!");
    }

    for arg in args {

        let port = arg.trim().parse();

        match port {
            Ok(p) => {
                ports.push(p);
                continue;
            },
            Err(_) => {
                for port in get_ports_from_range(&arg) {
                    ports.push(port);
                }
            },
        }
    }

    ports
}

fn get_ports_from_range(s: &str) -> RangeInclusive<i32> {
    let mut left = 0;
    let mut right = 0;

    let mut port1: i32 = -1;
    let port2: i32;

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

    port1..=port2

}

fn bind_to_ports(port_list: Vec<i32>) -> Vec<TcpListener> {
    let mut ret: Vec<TcpListener> = Vec::new();

    for port in port_list {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port));
        
        match listener {
            Ok(i) => {
                ret.push(i);
                println!("Bound to port {}", port);
            },
            Err(_) => {
                println!("Could not bind to port {}", port);
            },
        }
    }

    ret
}