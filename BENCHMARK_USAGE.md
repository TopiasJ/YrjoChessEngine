# Benchmark Usage Guide

This guide explains how to use the performance benchmarking system in YrjoChessEngine to measure nodes per second before and after optimizations.

## Overview

The benchmarking system provides comprehensive performance measurement for the alpha-beta search algorithm, including:
- **Nodes per second (NPS)** - Primary performance metric
- **Node counts** - Total nodes searched during evaluation
- **Evaluation counts** - Leaf node evaluations performed
- **Cutoff counts** - Alpha-beta pruning efficiency
- **Terminal node counts** - Game-ending positions found

## Quick Start

### Basic Benchmark
Run the standard benchmark suite at depth 3:
```bash
cargo run -- benchmark --depth 3
```

### Custom Position Benchmark
Test a specific chess position:
```bash
cargo run -- benchmark --depth 4 --fen "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 4"
```

### Multiple Iterations
Run multiple iterations for more accurate measurements:
```bash
cargo run -- benchmark --depth 3 --iterations 3
```

## Command Line Options

```
cargo run -- benchmark [OPTIONS]

Options:
  -d, --depth <DEPTH>        Calculation depth (half-moves) [default: 4]
  -f, --fen <FEN>           Custom FEN position to benchmark
  -i, --iterations <COUNT>   Number of benchmark iterations [default: 1]
  -h, --help                Print help information
```

## Standard Test Positions

The benchmark suite includes 8 carefully selected positions representing different game phases:

1. **Starting Position** - Standard opening setup
2. **Middlegame Position** - Complex piece development
3. **Tactical Position** - Sharp tactical complications
4. **Endgame Position** - King and pawn endgame
5. **Complex Middlegame** - Multiple piece exchanges
6. **Queen's Gambit** - Popular opening variation
7. **Sicilian Defense** - Dynamic pawn structure
8. **King's Indian Attack** - Positional setup

## Performance Metrics Explained

### Nodes Per Second (NPS)
- **What it measures**: Total search nodes processed per second
- **Why it matters**: Primary indicator of search speed
- **Typical values**: 150,000-300,000 NPS at depth 3-4

### Cutoff Rate
- **What it measures**: Percentage of nodes that caused alpha-beta pruning
- **Why it matters**: Indicates pruning efficiency
- **Good values**: 8-15% for well-ordered moves

### Evaluation Ratio
- **What it measures**: Leaf evaluations vs total nodes
- **Why it matters**: Shows search tree structure
- **Typical values**: 80-90% of nodes should be evaluations

## Measuring Performance Improvements

### Step 1: Establish Baseline
Before making any optimizations, run a baseline benchmark:
```bash
# Create baseline measurements
cargo run -- benchmark --depth 4 --iterations 3 > baseline_results.txt
```

### Step 2: Implement Optimization
Make your performance improvements to the alpha-beta algorithm.

### Step 3: Measure Improvement
Run the same benchmark after optimization:
```bash
# Measure optimized performance
cargo run -- benchmark --depth 4 --iterations 3 > optimized_results.txt
```

### Step 4: Compare Results
Compare the key metrics:
- **Speedup**: `optimized_NPS / baseline_NPS`
- **Node reduction**: `(baseline_nodes - optimized_nodes) / baseline_nodes * 100%`
- **Cutoff improvement**: Compare cutoff rates

## Example: Measuring Transposition Table Impact

```bash
# Before adding transposition table
cargo run -- benchmark --depth 5
# Result: 180,000 NPS, 2.5M nodes

# After adding transposition table
cargo run -- benchmark --depth 5  
# Expected: 300,000+ NPS, 1.5M nodes (40% reduction)
# Speedup: 1.67x, Node reduction: 40%
```

## Benchmarking Best Practices

### 1. Consistent Environment
- Run benchmarks on the same machine
- Close other applications to reduce interference
- Use release builds for accurate measurements: `cargo run --release -- benchmark`

### 2. Multiple Iterations
- Always run multiple iterations to account for variance
- Use `--iterations 3` or higher for reliable results
- Consider system warm-up effects

### 3. Appropriate Depth
- Start with depth 3-4 for quick feedback
- Use depth 5-6 for more comprehensive testing
- Deeper searches magnify optimization effects

### 4. Specific Position Testing
- Test optimizations on positions they're designed to improve
- Use tactical positions for pruning improvements
- Use endgame positions for evaluation optimizations

## Integration with Development Workflow

### Performance Testing Pipeline
```bash
#!/bin/bash
# performance_test.sh

echo "Running baseline benchmark..."
cargo run --release -- benchmark --depth 4 --iterations 3 > baseline.txt

echo "Implementing optimization..."
# (make your changes)

echo "Running optimized benchmark..."
cargo run --release -- benchmark --depth 4 --iterations 3 > optimized.txt

echo "Comparing results..."
# (manually compare or use diff tools)
```

### Continuous Performance Monitoring
- Run benchmarks before major changes
- Track performance regressions
- Maintain performance baselines for different optimization levels

## Troubleshooting

### Low Performance
- Ensure using release build: `cargo run --release`
- Check for debug prints in search algorithm
- Verify compiler optimizations are enabled

### Inconsistent Results
- Run more iterations: `--iterations 5`
- Check for background processes
- Use consistent test conditions

### Memory Issues
- Monitor memory usage during deep searches
- Consider reducing depth for memory-constrained systems

## Advanced Usage

### Custom Benchmark Positions
Create your own test suite by modifying `src/benchmark.rs`:
```rust
// Add to BenchmarkPositions::get_standard_positions()
("Your Position", "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"),
```

### Profiling Integration
Use with profiling tools:
```bash
# Profile with perf
perf record cargo run --release -- benchmark --depth 5
perf report

# Profile with Rust's built-in profiler
RUSTFLAGS="-C target-cpu=native" cargo run --release -- benchmark --depth 5
```

This benchmarking system provides the foundation for systematic performance optimization of the chess engine's search algorithm.