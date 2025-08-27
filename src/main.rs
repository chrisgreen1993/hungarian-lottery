use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use rayon::prelude::*; 

struct MatchCounts {
    two: u32,
    three: u32,
    four: u32,
    five: u32,
}

fn is_valid_number(n: u8) -> bool {
    n >= 1 && n <= 90
}

// Create a bitmask from the picks (1-90)
// This allows us to use bitwise operations to check for matches
// This is much faster than iterating over the ticket and checking each number
fn create_bitmask(numbers: &[u8]) -> u128 {
    numbers.iter().fold(0u128, |mask, &num| {
        mask | (1u128 << (num - 1))
    })
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

    let mut tickets: Vec<u128> = Vec::new();
    let mut line_count = 0;
    let mut valid_tickets = 0;

    // Read tickets from file and convert to bitmasks
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
                // Create the bitmask from the ticket
                let bitmask = create_bitmask(&nums);
                tickets.push(bitmask);
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

        // Parallel bitmask-based matching using Rayon
        let draw_bitmask = create_bitmask(&draw);
        let matches = tickets
            .par_iter() // Parallel iteration over tickets
            .map(|ticket_bitmask| {
                // Check the number of matches
                let match_count = (ticket_bitmask & draw_bitmask).count_ones() as u8;
                match match_count {
                    2 => MatchCounts { two: 1, three: 0, four: 0, five: 0 },
                    3 => MatchCounts { two: 0, three: 1, four: 0, five: 0 },
                    4 => MatchCounts { two: 0, three: 0, four: 1, five: 0 },
                    5 => MatchCounts { two: 0, three: 0, four: 0, five: 1 },
                    _ => MatchCounts { two: 0, three: 0, four: 0, five: 0 },
                }
            })
            // Combine the matches from each thread into a total
            .reduce(
                || MatchCounts { two: 0, three: 0, four: 0, five: 0 },
                |a, b| MatchCounts {
                    two: a.two + b.two,
                    three: a.three + b.three,
                    four: a.four + b.four,
                    five: a.five + b.five,
                },
            );

        println!("{} {} {} {}", 
            matches.two, 
            matches.three, 
            matches.four, 
            matches.five
        );
        
        let query_elapsed = query_start.elapsed();
        eprintln!(
            "Query {:?} took {:.3} ms",
            draw,
            query_elapsed.as_secs_f64() * 1000.0
        );
    }

    Ok(())
}
