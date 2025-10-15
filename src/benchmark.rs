use crate::alpha_beta_algorithm::{AlgorithmTraits, AlphaBetaAlgorithm, SearchStats};
use crate::chromosome::Chromosome;
use chess::Board;
use std::str::FromStr;
use std::time::Instant;

/// Standard chess positions for benchmarking
pub struct BenchmarkPositions;

impl BenchmarkPositions {
    /// Returns a collection of standard chess positions for benchmarking
    pub fn get_standard_positions() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Starting Position", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            ("Middlegame Position", "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 4"),
            ("Tactical Position", "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 4"),
            ("Endgame Position", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"),
            ("Complex Middlegame", "r1bq1rk1/ppp1nppp/3p1n2/3Pp3/1bP1P3/2N2N2/PP2BPPP/R1BQK2R w KQ - 0 8"),
            ("Queen's Gambit", "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 4"),
            ("Sicilian Defense", "rnbqkb1r/pp1ppppp/5n2/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3"),
            ("King's Indian Attack", "rnbqkb1r/pppppp1p/5np1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R w KQkq - 0 4"),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub position_name: String,
    pub depth: i32,
    pub nodes_searched: u64,
    pub evaluations: u64,
    pub cutoffs: u64,
    pub terminal_nodes: u64,
    pub tt_hits: u64,
    pub tt_misses: u64,
    pub tt_collisions: u64,
    pub time_ms: u128,
    pub nodes_per_second: f64,
}

impl BenchmarkResult {
    pub fn new(position_name: String, depth: i32, stats: SearchStats, time_ms: u128) -> Self {
        let nodes_per_second = if time_ms > 0 {
            (stats.nodes_searched as f64 * 1000.0) / time_ms as f64
        } else {
            0.0
        };
        
        Self {
            position_name,
            depth,
            nodes_searched: stats.nodes_searched,
            evaluations: stats.evaluations,
            cutoffs: stats.cutoffs,
            terminal_nodes: stats.terminal_nodes,
            tt_hits: stats.tt_hits,
            tt_misses: stats.tt_misses,
            tt_collisions: stats.tt_collisions,
            time_ms,
            nodes_per_second,
        }
    }
}

pub struct BenchmarkRunner {
    pub results: Vec<BenchmarkResult>,
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Run benchmarks on standard positions
    pub fn run_standard_benchmark(&mut self, depth: i32) -> Result<(), String> {
        println!("Running benchmark suite at depth {}", depth);
        println!("======================================");
        
        let positions = BenchmarkPositions::get_standard_positions();
        
        for (name, fen) in positions {
            match self.benchmark_position(name, fen, depth) {
                Ok(result) => {
                    println!("{}: {} nodes/sec ({} nodes in {} ms)", 
                            result.position_name, 
                            result.nodes_per_second as u64, 
                            result.nodes_searched, 
                            result.time_ms);
                    self.results.push(result);
                }
                Err(e) => {
                    eprintln!("Error benchmarking {}: {}", name, e);
                }
            }
        }
        
        self.print_summary();
        Ok(())
    }

    /// Run benchmark on a specific position
    pub fn benchmark_position(&self, name: &str, fen: &str, depth: i32) -> Result<BenchmarkResult, String> {
        let board = Board::from_str(fen).map_err(|e| format!("Invalid FEN: {}", e))?;
        let mut algorithm = AlphaBetaAlgorithm::new();
        
        let start_time = Instant::now();
        let (_best_move, stats) = algorithm.get_best_move_with_stats(board, depth);
        let elapsed = start_time.elapsed();
        
        Ok(BenchmarkResult::new(
            name.to_string(),
            depth,
            stats,
            elapsed.as_millis(),
        ))
    }

    /// Run benchmark with chromosome
    pub fn benchmark_position_with_chromosome(&self, name: &str, fen: &str, depth: i32, chromosome: &Chromosome) -> Result<BenchmarkResult, String> {
        let board = Board::from_str(fen).map_err(|e| format!("Invalid FEN: {}", e))?;
        let mut algorithm = AlphaBetaAlgorithm::new();
        
        let start_time = Instant::now();
        let (_best_move, stats) = algorithm.get_best_move_with_chromosome_and_stats(board, depth, chromosome);
        let elapsed = start_time.elapsed();
        
        Ok(BenchmarkResult::new(
            name.to_string(),
            depth,
            stats,
            elapsed.as_millis(),
        ))
    }

    /// Print summary of benchmark results
    pub fn print_summary(&self) {
        if self.results.is_empty() {
            println!("No benchmark results to display.");
            return;
        }

        println!("\n======================================");
        println!("BENCHMARK SUMMARY");
        println!("======================================");
        
        let total_nodes: u64 = self.results.iter().map(|r| r.nodes_searched).sum();
        let total_time: u128 = self.results.iter().map(|r| r.time_ms).sum();
        let total_evaluations: u64 = self.results.iter().map(|r| r.evaluations).sum();
        let total_cutoffs: u64 = self.results.iter().map(|r| r.cutoffs).sum();
        let total_terminal_nodes: u64 = self.results.iter().map(|r| r.terminal_nodes).sum();
        
        let avg_nodes_per_second: f64 = self.results.iter().map(|r| r.nodes_per_second).sum::<f64>() / self.results.len() as f64;
        let overall_nodes_per_second = if total_time > 0 {
            (total_nodes as f64 * 1000.0) / total_time as f64
        } else {
            0.0
        };
        
        println!("Total nodes searched: {}", total_nodes);
        println!("Total evaluations: {}", total_evaluations);
        println!("Total cutoffs: {}", total_cutoffs);
        println!("Total terminal nodes: {}", total_terminal_nodes);
        println!("Total time: {} ms", total_time);
        println!("Average nodes/sec per position: {:.0}", avg_nodes_per_second);
        println!("Overall nodes/sec: {:.0}", overall_nodes_per_second);
        
        if total_nodes > 0 {
            let cutoff_rate = (total_cutoffs as f64 / total_nodes as f64) * 100.0;
            println!("Cutoff rate: {:.1}%", cutoff_rate);
        }
        
        // Show transposition table statistics
        let total_tt_hits: u64 = self.results.iter().map(|r| r.tt_hits).sum();
        let total_tt_misses: u64 = self.results.iter().map(|r| r.tt_misses).sum();
        let total_tt_collisions: u64 = self.results.iter().map(|r| r.tt_collisions).sum();
        let total_tt_probes = total_tt_hits + total_tt_misses;
        
        if total_tt_probes > 0 {
            let tt_hit_rate = (total_tt_hits as f64 / total_tt_probes as f64) * 100.0;
            println!("TT hit rate: {:.1}% ({} hits, {} misses, {} collisions)", 
                    tt_hit_rate, total_tt_hits, total_tt_misses, total_tt_collisions);
        }
        
        println!("\nDetailed Results:");
        println!("{:<25} {:<8} {:<12} {:<12} {:<8} {:<12}", "Position", "Depth", "Nodes", "Evals", "Cutoffs", "NPS");
        println!("{:-<80}", "");
        
        for result in &self.results {
            println!("{:<25} {:<8} {:<12} {:<12} {:<8} {:<12.0}", 
                    result.position_name, 
                    result.depth, 
                    result.nodes_searched, 
                    result.evaluations, 
                    result.cutoffs, 
                    result.nodes_per_second);
        }
    }

    /// Compare two benchmark runs
    pub fn compare_with(&self, other: &BenchmarkRunner) -> Option<BenchmarkComparison> {
        if self.results.len() != other.results.len() {
            return None;
        }
        
        let mut comparisons = Vec::new();
        
        for (_i, (result1, result2)) in self.results.iter().zip(other.results.iter()).enumerate() {
            if result1.position_name != result2.position_name || result1.depth != result2.depth {
                return None;
            }
            
            let speedup = result2.nodes_per_second / result1.nodes_per_second;
            let node_reduction = if result1.nodes_searched > 0 {
                ((result1.nodes_searched as f64 - result2.nodes_searched as f64) / result1.nodes_searched as f64) * 100.0
            } else {
                0.0
            };
            
            comparisons.push(PositionComparison {
                position_name: result1.position_name.clone(),
                depth: result1.depth,
                baseline_nps: result1.nodes_per_second,
                optimized_nps: result2.nodes_per_second,
                speedup,
                baseline_nodes: result1.nodes_searched,
                optimized_nodes: result2.nodes_searched,
                node_reduction,
            });
        }
        
        Some(BenchmarkComparison {
            comparisons,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PositionComparison {
    pub position_name: String,
    pub depth: i32,
    pub baseline_nps: f64,
    pub optimized_nps: f64,
    pub speedup: f64,
    pub baseline_nodes: u64,
    pub optimized_nodes: u64,
    pub node_reduction: f64,
}

#[derive(Debug, Clone)]
pub struct BenchmarkComparison {
    pub comparisons: Vec<PositionComparison>,
}

impl BenchmarkComparison {
    pub fn print_comparison(&self) {
        println!("\n======================================");
        println!("BENCHMARK COMPARISON");
        println!("======================================");
        
        let avg_speedup: f64 = self.comparisons.iter().map(|c| c.speedup).sum::<f64>() / self.comparisons.len() as f64;
        let avg_node_reduction: f64 = self.comparisons.iter().map(|c| c.node_reduction).sum::<f64>() / self.comparisons.len() as f64;
        
        println!("Average speedup: {:.2}x", avg_speedup);
        println!("Average node reduction: {:.1}%", avg_node_reduction);
        
        println!("\nDetailed Comparison:");
        println!("{:<25} {:<8} {:<12} {:<12} {:<8} {:<12}", "Position", "Depth", "Baseline NPS", "Optimized NPS", "Speedup", "Node Reduction");
        println!("{:-<90}", "");
        
        for comp in &self.comparisons {
            println!("{:<25} {:<8} {:<12.0} {:<12.0} {:<8.2}x {:<12.1}%", 
                    comp.position_name, 
                    comp.depth, 
                    comp.baseline_nps, 
                    comp.optimized_nps, 
                    comp.speedup, 
                    comp.node_reduction);
        }
    }
}