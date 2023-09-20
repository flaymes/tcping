use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = '6', default_value = "false", help = "ping IPV6 address")]
    pub v6: bool,
    #[arg(short = 'H', long, help = "remote host ip address")]
    pub host: String,
    #[arg(short, long, help = "remote host ping port")]
    pub port: u16,
    #[arg(
        short,
        long,
        default_value = "1",
        help = "ping interval,second. default=1s"
    )]
    pub interval: u64,
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        help = "Number of ping iterations.default=10"
    )]
    pub count: u32,
}

pub fn do_tcp_ping(ip_addr: IpAddr, ping_port: u16, interval: u64, ping_count: u32) {
    let mut ping_itr_num: u32 = 0;

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
            println!("TCP ping elapsed {:.3} ms", latency);
            let min_cmp_some = latency.partial_cmp(&min_latency);

            if let Some(std::cmp::Ordering::Less) = min_cmp_some {
                min_latency = latency;
            }

            let max_cmp_some = latency.partial_cmp(&max_latency);
            if let Some(std::cmp::Ordering::Greater) = max_cmp_some {
                max_latency = latency;
            }

            avg_latency += latency;
        } // auto close stream

        ping_itr_num += 1;
        if ping_itr_num >= ping_count {
            break;
        }
        thread::sleep(Duration::from_secs(interval));
    }
    avg_latency = avg_latency / (ping_itr_num as f32);

    println!(
        "tcp ping min/avg/max = {:.3}/{:.3}/{:.3} ms",
        min_latency, avg_latency, max_latency
    );
}

pub fn parse_args(cli: Cli) -> Result<(IpAddr, u16), &'static str> {
    let host = cli.host;
    let port = cli.port;

    if cli.v6 {
        let ipv6_resolve_result = Ipv6Addr::from_str(host.as_str());
        if let Err(err) = ipv6_resolve_result {
            eprintln!("Parse IPv6 address {} failed.{}", host, err);
            return Err("Parse IPv6 address failed.");
        }
        return Ok((IpAddr::from(ipv6_resolve_result.unwrap()), port));
    } else {
        let ipv4_result = Ipv4Addr::from_str(host.as_str());

        if let Err(err) = ipv4_result {
            eprintln!("Parse IPv4 address {} failed.{}", host, err);
            return Err("Parse IPv4 address failed.");
        }
        return Ok((IpAddr::from(ipv4_result.unwrap()), port));
    }
}
