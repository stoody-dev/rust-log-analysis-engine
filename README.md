# Parallel Rust Log Analysis Engine

A high-performance CLI tool written in Rust for analyzing large log files efficiently using streaming parsing and parallel processing.

---

##  Overview

Modern systems generate massive log files that need to be analyzed quickly to debug errors, monitor services, and detect unusual behavior.

This tool processes structured log files and extracts useful insights such as:

- Total logs processed
- Log level statistics (INFO, WARNING, ERROR)
- Most active IP addresses
- Processing performance

The engine is designed to handle **millions of log entries** while remaining memory efficient.

---

##  Features

### Streaming Log Processing
Logs are processed line-by-line using buffered reading to avoid loading entire files into memory.

### Structured Log Parsing

### Parallel Processing
Analytics are computed in parallel using Rayon to utilize multiple CPU cores.

### Regex Filtering
Users can filter logs using regular expressions.

### CLI Interface
Arguments are parsed using Clap to provide a clean command line interface.

