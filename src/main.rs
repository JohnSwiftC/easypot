use std::{
    env,
    thread,
    sync::{Arc, Mutex},
    ops::RangeInclusive,
    net::TcpListener,
    io::{BufReader, BufRead, Write, ErrorKind, Error},
    fs::File,
    collections::HashMap,
};

fn main() {
    
    let ports: Vec<i32> = get_ports_from_input();
    let listeners: Vec<TcpListener> = bind_to_ports(ports);
    let message_stack: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let ip_addr_freq: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));

    create_threads(listeners, Arc::clone(&message_stack), Arc::clone(&ip_addr_freq));

    let file = make_log_file("easypotlog");
    let ipfile = make_log_file("easypotfreq");

    match file {
        Ok(f) => {
            read_file(f, ipfile, Arc::clone(&message_stack), Arc::clone(&ip_addr_freq));
        },
        Err(_) => {
            println!("Failed to create logging file, logging disabled.");
            read_no_file(Arc::clone(&message_stack));
        },
    }

}

fn make_log_file(name: &str) -> Result<File, Error> {
    let mut log_iter = 1;
    loop {
        match File::create_new(format!("{}-{}.txt", name, log_iter)) {
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

fn create_threads(listeners: Vec<TcpListener>, message_stack: Arc<Mutex<Vec<String>>>, ip_addr_freq: Arc<Mutex<HashMap<String, u32>>>) {
    for listener in listeners {
        let message_stack_arc = Arc::clone(&message_stack);
        let ip_addr_freq_arc = Arc::clone(&ip_addr_freq);
        thread::spawn(move || {

            let port = listener.local_addr().unwrap().port();

            for stream in listener.incoming() {

                let stream = match stream {
                    Ok(s) => s,
                    Err(_) => {
                        continue;
                    },
                };

                let remote_ip = match stream.peer_addr() {
                    Ok(addr) => addr.to_string(),
                    Err(_) => String::from("Unidentified"),
                };

                let buf_reader = BufReader::new(&stream);

                let data: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap_or_else(|_err| String::from("Bad data, likely a port scanner.")))
                    .take_while(|line| !line.is_empty())
                    .collect();

                let mut message_stack = message_stack_arc.lock().unwrap();
                let mut ip_addr_freq = ip_addr_freq_arc.lock().unwrap();

                let ipcount = ip_addr_freq.entry(String::from(
                    get_ip_str(&remote_ip)
                )).or_insert(0);
                *ipcount += 1;

                message_stack.push(format!("Port: {} Remote IP: {}\n{data:#?}", port, remote_ip));

            }

        });
    }
}

fn read_no_file(message_stack_arc: Arc<Mutex<Vec<String>>>) {

    loop {

        let mut message_stack = message_stack_arc.lock().unwrap();
        
        let message = message_stack.pop();

        let message = match message {
            Some(val) => val,
            None => continue,
        };

        println!("{}", message);
    }
}

/// Quickly becoming a little messy so documenting for later use.
/// Takes in a File, and then a Result<File>
/// Someone intelligent might make this function take in two results
/// which would clean up main and reduce overall complexity
/// and remove the need for two differnet reading functions branched in main
/// I will not be doing that right now.
fn read_file(mut file: File, mut ipfile: Result<File, Error>, message_stack_arc: Arc<Mutex<Vec<String>>>, ip_addr_freq_arc: Arc<Mutex<HashMap<String, u32>>>) {
    loop {

        let mut message_stack = message_stack_arc.lock().unwrap();
        let ip_addr_freq = ip_addr_freq_arc.lock().unwrap();

        let message = message_stack.pop();

        let message = match message {
            Some(val) => val,
            None => continue,
        };

        let res = file.write_all(
            format!("{}\n\n", message).as_bytes()
        );

        println!("{}", message);

        if let Err(_) = res {
            println!("Failed to write a line to file!");
        }

        // PLEASE REWRITE THIS LOL

        match ipfile {
            Ok(ref mut f) => {
                let _ = f.set_len(0);
                let _ = f.write_all(format!("\n{ip_addr_freq:#?}").as_bytes());
            },
            Err(_) => (),
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
            },
            Err(_) => {
                println!("Could not bind to port {}", port);
            },
        }
    }

    ret
}

fn get_ip_str(ip: &str) -> &str {
    let mut i = 0;
    for c in ip.chars() {
        if c != ':' {
            i += 1;
        } else {
            return &ip[0..i];
        }
    }

    ip
}