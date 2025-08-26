use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn is_valid_number(n: u8) -> bool {
    n >= 1 && n <= 90
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} input.txt", args[0]);
        std::process::exit(1);
    }

    let preprocess_start = Instant::now();

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut tickets: Vec<[u8; 5]> = Vec::new();
    let mut line_count = 0;
    let mut valid_tickets = 0;

    // Read tickets from file into memory
    for line in reader.lines() {
        line_count += 1;
        let line = line?;
        let nums: Vec<u8> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect();
        if nums.len() == 5 {
            // Validate that all numbers are in range 1-90
            if nums.iter().all(|&n| is_valid_number(n)) {
                tickets.push([nums[0], nums[1], nums[2], nums[3], nums[4]]);
                valid_tickets += 1;
            } else {
                eprintln!(
                    "Warning: Skipping line {} with out-of-range numbers: '{}' (numbers must be 1-90)",
                    line_count, line
                );
            }
        } else {
            eprintln!(
                "Warning: Skipping malformed line {}: '{}' (expected 5 numbers, got {})",
                line_count, line, nums.len()
            );
        }
    }

    let preprocess_elapsed = preprocess_start.elapsed();
    eprintln!(
        "Preprocessing: processed {} lines, {} valid tickets in {:.3} s",
        line_count,
        valid_tickets,
        preprocess_elapsed.as_secs_f64()
    );

    println!("READY");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        
        let query_start = Instant::now();
        
        let draw: Vec<u8> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect();

        if draw.len() != 5 {
            eprintln!(
                "Warning: Invalid draw input: '{}' (expected 5 numbers, got {})",
                line, draw.len()
            );
            continue;
        }

        // Validate that all draw numbers are in range 1-90
        if !draw.iter().all(|&n| is_valid_number(n)) {
            eprintln!(
                "Warning: Invalid draw input: '{}' (numbers must be 1-90)",
                line
            );
            continue;
        }

        let mut matches2 = 0;
        let mut matches3 = 0;
        let mut matches4 = 0;
        let mut matches5 = 0;

        // Naive matching for now (O(N))
        for ticket in &tickets {
            let count = ticket.iter().filter(|n| draw.contains(n)).count();
            match count {
                2 => matches2 += 1,
                3 => matches3 += 1,
                4 => matches4 += 1,
                5 => matches5 += 1,
                _ => {}
            }
        }

        println!("{matches2} {matches3} {matches4} {matches5}");
        
        let query_elapsed = query_start.elapsed();
        eprintln!(
            "Query {:?} took {:.3} ms",
            draw,
            query_elapsed.as_secs_f64() * 1000.0
        );
    }

    Ok(())
}
