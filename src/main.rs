use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    // let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7890);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(39, 156, 66, 10)), 22);
    let mut min_latency: f32 = f32::MAX;
    let mut max_latency: f32 = f32::MIN;
    let mut avg_latency: f32 = 0.0f32;
    loop {
        {
            // TCP connect
            let start_time = Instant::now();
            let conn_some = TcpStream::connect(addr);
            if let Err(_) = conn_some {
                eprintln!("Failed connect to {}!", addr.to_string());
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

        count += 1;
        if count >= 10 {
            break;
        }
        thread::sleep(Duration::from_millis(1000));
    }
    avg_latency = avg_latency / (count as f32);

    println!(
        "tcp ping min/avg/max = {:.3}/{:.3}/{:.3} ms",
        min_latency, avg_latency, max_latency
    );
}
