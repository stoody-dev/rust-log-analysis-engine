use std::collections::HashMap;
use rayon::prelude::*;
use crate::models::LogEntry;

pub struct LogStats {
    pub total: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub ip_counts: HashMap<String, usize>,
}

impl LogStats {

    pub fn analyze_parallel(entries: Vec<LogEntry>) -> Self {

        let total = entries.len();

        let (error_count, warning_count, info_count) =
            entries.par_iter().fold(
                || (0, 0, 0),
                |mut acc, entry| {

                    match entry.level.as_str() {
                        "ERROR" => acc.0 += 1,
                        "WARNING" => acc.1 += 1,
                        "INFO" => acc.2 += 1,
                        _ => {}
                    }

                    acc
                }
            )
            .reduce(
                || (0, 0, 0),
                |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2)
            );

        let mut ip_counts = HashMap::new();

        for entry in entries {
            *ip_counts.entry(entry.ip).or_insert(0) += 1;
        }

        LogStats {
            total,
            error_count,
            warning_count,
            info_count,
            ip_counts,
        }
    }

    pub fn top_ip(&self) -> Option<(&String, &usize)> {
        self.ip_counts.iter().max_by_key(|(_, count)| *count)
    }
}