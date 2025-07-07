# Hybrid Benchmarking Strategy

This document explains YrjoChessEngine's dual approach to performance measurement, combining custom chess-specific benchmarks with Criterion.rs micro-benchmarks.

## Overview

The engine employs two complementary benchmarking systems:

1. **Custom Engine Benchmarks** - Chess-specific end-to-end performance measurement
2. **Criterion Micro-Benchmarks** - Statistical analysis of individual functions

## When to Use Each Approach

### Use Custom Benchmarks (`cargo run -- benchmark`) For:
- **End-to-end performance measurement** - Full search performance with real chess positions
- **Chess-specific metrics** - Nodes per second, cutoff rates, search tree analysis
- **Before/after optimization comparison** - Measuring overall engine improvements
- **User-facing performance testing** - What players actually experience
- **Integration testing** - How optimizations affect the complete system

### Use Criterion Benchmarks (`cargo bench`) For:
- **Function-level optimization** - Individual algorithm components
- **Statistical rigor** - Confidence intervals, outlier detection, regression analysis
- **Micro-optimizations** - Small performance improvements in hot code paths
- **Development-time testing** - Quick feedback during optimization work
- **Academic/research analysis** - Detailed statistical performance analysis

## Available Benchmarks

### Custom Engine Benchmarks

```bash
# Full benchmark suite at depth 4
cargo run --release -- benchmark --depth 4

# Custom position testing
cargo run -- benchmark --depth 3 --fen "custom_position"

# Multiple iterations for accuracy
cargo run -- benchmark --depth 3 --iterations 5
```

**Provides:**
- Nodes per second (primary metric)
- Cutoff rates (pruning efficiency)
- Search tree structure analysis
- Position-specific performance
- Real-world chess scenarios

### Criterion Micro-Benchmarks

```bash
# All micro-benchmarks
cargo bench

# Specific benchmark groups
cargo bench evaluation
cargo bench move_generation
cargo bench alpha_beta_search
cargo bench alpha_beta_functions
cargo bench complete_analysis

# Single benchmark
cargo bench simple_evaluation
```

**Benchmark Groups:**

1. **Evaluation Benchmarks**
   - Default piece value evaluation
   - Chromosome-based evaluation
   - Different position types (opening, middlegame, endgame)

2. **Move Generation Benchmarks**
   - Legal move generation
   - Move ordering (captures first)
   - Different position complexities

3. **Alpha-Beta Search Benchmarks**
   - Full search at depths 2, 3, 4
   - Different position types
   - Search statistics tracking

4. **Alpha-Beta Function Benchmarks**
   - Individual `alpha_beta_max` calls
   - Individual `alpha_beta_min` calls
   - Direct function performance

5. **Complete Analysis Benchmarks**
   - End-to-end best move calculation
   - Real game scenarios

## Hybrid Workflow Example

### Scenario: Implementing Transposition Tables

#### 1. Establish Baseline with Both Systems

```bash
# Custom benchmark baseline
cargo run --release -- benchmark --depth 4 > baseline_custom.txt

# Criterion benchmark baseline
cargo bench > baseline_criterion.txt
```

#### 2. Implement Optimization

Add transposition table to `alpha_beta_algorithm.rs`

#### 3. Measure Improvement

```bash
# Custom benchmark after optimization
cargo run --release -- benchmark --depth 4 > optimized_custom.txt

# Criterion benchmark after optimization  
cargo bench > optimized_criterion.txt
```

#### 4. Analysis

**Custom Benchmarks Show:**
- Overall nodes/second improvement: 2.5x speedup
- Node reduction: 60% fewer nodes searched
- Improved cutoff rates: 25% better pruning

**Criterion Benchmarks Show:**
- `alpha_beta_max` function: 15% performance overhead (hash lookup cost)
- Evaluation function: No change (as expected)
- Move generation: No change (as expected)

#### 5. Conclusion

- Net positive: 2.5x overall speedup despite 15% function overhead
- Transposition table saves much more work than it costs
- Both metrics confirm successful optimization

## Best Practices

### For Development Workflow

1. **Start with Criterion** - Quick function-level feedback during development
2. **Validate with Custom** - Confirm end-to-end improvements
3. **Use Both for Analysis** - Understand both micro and macro effects

### For Performance Tuning

1. **Custom benchmarks** for identifying bottlenecks
2. **Criterion benchmarks** for optimizing hot functions
3. **Custom benchmarks** for validating overall improvement

### For Research and Analysis

1. **Criterion** provides statistical confidence
2. **Custom** provides chess-domain insights
3. **Combined** gives complete performance picture

## Metrics Comparison

| Metric | Custom Benchmarks | Criterion Benchmarks |
|--------|------------------|---------------------|
| Nodes per second | ✅ Primary metric | ❌ Not directly measured |
| Statistical rigor | ❌ Basic timing | ✅ Confidence intervals |
| Chess domain insights | ✅ Cutoff rates, tree analysis | ❌ Function-level only |
| Function overhead | ❌ Hidden in overall | ✅ Clearly visible |
| Real-world performance | ✅ Actual game scenarios | ❌ Artificial micro-tests |
| Development speed | ❌ Slower full runs | ✅ Quick feedback |
| Regression detection | ❌ Manual comparison | ✅ Automatic baselines |

## Implementation Details

### Custom Benchmark Architecture

- **SearchStats tracking** - Nodes, evaluations, cutoffs, terminal nodes
- **Standard test positions** - 8 carefully selected chess positions
- **Timing measurement** - High-resolution timing with statistics
- **Chess-specific analysis** - Position type performance differences

### Criterion Integration

- **Public API exposure** - Key functions made public for testing
- **Multiple benchmark groups** - Organized by component type
- **Position variety** - Different chess scenarios for each test
- **Sample size control** - Reduced samples for expensive operations

## Future Enhancements

### Planned Improvements

1. **Baseline Management** - Automatic comparison with previous runs
2. **CI Integration** - Continuous performance monitoring
3. **HTML Reports** - Visual performance analysis
4. **Profile Integration** - Link with profiling tools

### Advanced Usage

1. **Memory Benchmarks** - Track memory usage patterns
2. **Parallel Benchmarks** - Multi-threaded performance analysis
3. **Regression Testing** - Automated performance validation

This hybrid approach provides comprehensive performance measurement capabilities, supporting both development-time optimization and research-grade analysis.