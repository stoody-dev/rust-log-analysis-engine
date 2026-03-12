use std::fs::File;
use std::io::Write;

fn main() {

    let mut file = File::create("logs/big.log")
        .expect("Unable to create file");

    let levels = ["INFO", "WARNING", "ERROR"];
    let ips = [
        "192.168.1.1",
        "192.168.1.2",
        "192.168.1.3",
        "192.168.1.4",
        "192.168.1.5"
    ];

    let messages = [
        "Server started",
        "Request received",
        "Database connection failed",
        "Disk usage high",
        "Timeout occurred"
    ];

    for i in 0..1_000_000 {

        let level = levels[i % levels.len()];
        let ip = ips[i % ips.len()];
        let message = messages[i % messages.len()];

        let timestamp = format!("2025-03-07 10:{:02}:{:02}", (i / 60) % 60, i % 60);

        let log_line = format!("{} {} {} {}\n", timestamp, level, ip, message);

        file.write_all(log_line.as_bytes()).unwrap();
    }

    println!("Generated 1000000 logs in logs/big.log");
}