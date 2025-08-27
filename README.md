# Hungarian Lottery

## Prerequisites

Install Rust via [rustup.rs](https://rustup.rs)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build and run
```bash
# Build in release mode
cargo build --release

# Or with extra benchmarking logs
cargo build --release --features bench

# Run the built binary
./target/release/hungarian-lottery input.txt

```

After preprocessing, it prints `READY` and then reads draws from stdin, one per line, printing counts (2, 3, 4, 5 matches) per draw.

## Solution Overview

- Preprocessing phase (file input):
  - Read each line (one ticket of 5 numbers), perform validation, and build a 128 bit bitmask where bit (n-1) encodes presence of number n, with one bitmask per ticket.
- Reporting phase (stdin):
  - Read the users picks, validate, generate its bitmask and add to a vector of tickets.
  - Use the [Rayon](https://docs.rs/rayon/latest/rayon/index.html) library to split the ticket bitmask vector processing across many threads.
  - For each ticket bitmask, use a bitwise AND to check if it matches the draw bitmask, setting the appropriate field in the MatchCounts struct.
  - Finally, use reduce to combine the MatchCounts from each thread into a single struct and print the results.

## Performance Analysis

This is a modified version of a naive solution, with parallel processing used to reduce the time complexity to O(N/P), as well as using bitmasks to further increase performance at the cost of a small memory overhead (16 bytes per ticket rather than 5 bytes).

Test machine: Apple M1 Pro CPU / 16GB Memory

N = Number of tickets (10 million)
P = Number of threads (8)
M = number range (1-90)
K = Number of picks (5)

| Operation | Time Complexity | Space Complexity | Measured (N=10 million) |
|---|---|---|---|
| Preprocessing | O(N*K) | O(N*M) | ~1.5s, ~160MB |
| Reporting | O(N/P) | O(N*K) | ~10ms, ~160MB |


### (Very) Rough Estimates

**Number of Tickets (N)**

| Metric | 2x (N=20 million) | 10x (N=100 million) |
|---|---|---|
| Preprocessing Time | ~3s | ~15s |
| Preprocessing Space | ~320MB | ~1.6GB |
| Reporting Time | ~20ms | ~100ms |
| Reporting Space | ~320MB | ~1.6GB |

**Number Range (M)**

| Metric | 2x (M=180) | 10x (M=900) |
|---|---|---|
| Preprocessing Time | ~1.5s | ~1.5s |
| Preprocessing Space | ~320MB | ~1.6GB |
| Reporting Time | ~10ms | ~10ms |
| Reporting Space | ~160MB | ~160MB |

**Number of Picks (K)**

| Metric | 2x (K=10) | 10x (K=50) |
|---|---|---|
| Preprocessing Time | ~3s | ~15s |
| Preprocessing Space | ~160MB | ~160MB |
| Reporting Time | ~10ms | ~10ms |
| Reporting Space | ~320MB | ~1.6GB |


I also explored solutions using binomial coefficients and pre-calculating lookup tables, and although they had sub 1ms, O(1) reporting speed, the lookup tables built during pre-processing grew exponentially as K or M changed, making these solutions very sensitive to changes in K or M space-wise, and would be unworkable if the format of the lottery changed. The implementation is also complex and harder to maintain than other solutions.

Therefore, I settled on this solution as it provides fast enough reporting speeds for the current parameters, scales linearly if either N, M or K changes, and the implementation is simple and easy to maintain.

## Future Improvements

- Adding tests, monitoring etc etc
- Batch multiple draws
  - We could allow the user to enter multiple draws after READY so we can process them all at the same, reducing the need to scan over the ticket bitmask vector for each draw.
- Horizontal scaling across nodes
    - If parameters increase significantly and we reach the limits of processing on a single machine, we could consider scaling horizontally by splitting the workload across multiple machines.



