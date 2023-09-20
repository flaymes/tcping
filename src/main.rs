extern crate clap;

use clap::Parser;
use tcping::Cli;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let cli = Cli::parse();
    let interval = cli.interval;
    let count = cli.count;

    let parse_result = tcping::parse_args(cli);
    if let Err(err) = parse_result {
        eprintln!("Parse arguments failed.{}", err);
        return;
    }

    let (ip_addr, ping_port) = parse_result.unwrap();

    tcping::do_tcp_ping(ip_addr, ping_port, interval, count);
}
