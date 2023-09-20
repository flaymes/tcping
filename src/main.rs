use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::{env, thread};
use std::str::FromStr;
use std::time::{Duration, Instant};

fn main() {

    let args: Vec<String> = env::args().collect();

    let parse_result = parse_args(&args);
    if let Err(err) = parse_result {
        eprintln!("Parse arguments failed.{}",err);
        return;
    }

    let (ip_addr, ping_port) = parse_result.unwrap();

    do_tcp_ping(ip_addr, ping_port);
}

fn do_tcp_ping(ip_addr: IpAddr, ping_port: u16) {

    let mut ping_itr_num = 0;

    let addr = SocketAddr::new(ip_addr, ping_port);
    let mut min_latency: f32 = f32::MAX;
    let mut max_latency: f32 = f32::MIN;
    let mut avg_latency: f32 = 0.0f32;
    loop {
        {
            // TCP connect
            let start_time = Instant::now();
            let conn_some = TcpStream::connect(addr);
            if let Err(_) = conn_some {
                eprintln!("Failed connect to {} !", addr.to_string());
                return;
            }

            let end_time = Instant::now();
            let duration = end_time - start_time;
            let latency = duration.as_secs_f32() * 1000f32; // ms

            let min_cmp_some = latency.partial_cmp(&min_latency);

            if let Some(std::cmp::Ordering::Less) = min_cmp_some {
                min_latency = latency;
            }

            let max_cmp_some = latency.partial_cmp(&max_latency);
            if let Some(std::cmp::Ordering::Greater) = max_cmp_some {
                max_latency = latency;
            }

            avg_latency += latency;

            println!("TCP ping elapsed {:.3} ms", latency);
        } // auto close stream

        ping_itr_num += 1;
        if ping_itr_num >= 10 {
            break;
        }
        thread::sleep(Duration::from_millis(1000));
    }
    avg_latency = avg_latency / (ping_itr_num as f32);

    println!(
        "tcp ping min/avg/max = {:.3}/{:.3}/{:.3} ms",
        min_latency, avg_latency, max_latency
    );
}

fn parse_args(args: &Vec<String>) -> Result<(IpAddr, u16), &'static str> {
    if args.len() < 5 {
        eprintln!("too few arguments.{:?}", args);
        return Err("too few arguments");
    }

    let mut itr = args.iter();
    itr.next();

    let mut ipAddr = IpAddr::from_str("127.0.0.1").unwrap();
    let mut ping_port: u16 = 80;
    while let Some(arg) = itr.next() {
        match arg.as_str() {
            "-4" => {
                // IP v4
                let ip_str = itr.next().unwrap().as_str();

                let ipv4_result = Ipv4Addr::from_str(ip_str);

                if let Err(err) = ipv4_result {
                    eprintln!("Parse IPv4 address {} failed.{}", ip_str, err);
                    return Err("Parse IPv4 address failed.");
                }
                ipAddr = IpAddr::from(ipv4_result.unwrap());
            }
            "-6" => {
                // IP v6
                let ip_str = itr.next().unwrap().as_str();
                let ipv6_resolve_result = Ipv6Addr::from_str(ip_str);
                if let Err(err) = ipv6_resolve_result {
                    eprintln!("Parse IPv6 address {} failed.{}", ip_str, err);
                    return Err("Parse IPv6 address failed.");
                }
                ipAddr = IpAddr::from(ipv6_resolve_result.unwrap());
            }
            "-p" => {
                // destination port
                let port_str = itr.next().unwrap().as_str();
                let port_result = port_str.parse::<u16>();
                if let Err(err) = port_result {
                    eprintln!("Parse port {} failed. err = {}", port_str, err);
                    return Err("Parse port failed.");
                }
                ping_port = port_result.unwrap();
            }
            _ => {}
        }
    }

    println!("IP:{}, port:{}", ipAddr.to_string(), ping_port);

    Ok((ipAddr,ping_port))
}