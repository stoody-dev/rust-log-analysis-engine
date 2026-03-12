mod parser;
mod models;
mod analyzer;

use clap::Parser;
use parser::{stream_logs, parse_structured_log};
use analyzer::LogStats;
use regex::Regex;


#[derive(Parser)]
struct Args {
    file: String,
    pattern: Option<String>,
}

fn main() {

    let args = Args::parse();

    println!("Processing log file: {}\n", args.file);

    let regex = args.pattern.as_ref().map(|p| Regex::new(p).unwrap());

    let entries: Vec<_> = stream_logs(&args.file)
        .filter(|line| {
            if let Some(ref r) = regex {
                r.is_match(line)
            } else {
                true
            }
    })
        .filter_map(|line| parse_structured_log(&line))
        .collect();

    let stats = LogStats::analyze_parallel(entries);

    println!("====== Log Statistics ======");
    println!("Total logs: {}", stats.total);
    println!("INFO logs: {}", stats.info_count);
    println!("WARNING logs: {}", stats.warning_count);
    println!("ERROR logs: {}", stats.error_count);

    if let Some((ip, count)) = stats.top_ip() {
        println!("Top IP: {} ({} requests)", ip, count);
    }
}