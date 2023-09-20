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
    let addr = SocketAddr::new(ip_addr, ping_port);
    let mut min_latency = f64::MAX;
    let mut max_latency = f64::MIN;

    let mut rtts: Vec<f64> = Vec::new();

    for i in 0..ping_count {
        let start_time = Instant::now();
        let conn_some = TcpStream::connect(addr);
        if let Err(_) = conn_some {
            eprintln!("Failed connect to {} !", addr.to_string());
            return;
        }
        let duration = start_time.elapsed();
        let rtt = duration.as_secs_f64() * 1000f64; // ms
        println!("TCP ping elapsed {:.3} ms", rtt);

        if let Some(std::cmp::Ordering::Less) = rtt.partial_cmp(&min_latency) {
            min_latency = rtt;
        }

        if let Some(std::cmp::Ordering::Greater) = rtt.partial_cmp(&max_latency) {
            max_latency = rtt;
        }

        rtts.push(rtt);

        thread::sleep(Duration::from_secs(interval));
    }

    let rtt_sum: f64 = rtts.iter().sum();

    let avg_latency = rtt_sum / (ping_count as f64);

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
