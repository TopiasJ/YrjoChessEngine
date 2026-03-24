use chess::{Board, MoveGen};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::str::FromStr;
use yrjo_chess_engine::alpha_beta_algorithm::{AlgorithmTraits, AlphaBetaAlgorithm};
use yrjo_chess_engine::chromosome::Chromosome;
use yrjo_chess_engine::evaluator::Evaluator;

// Standard chess positions for micro-benchmarking
struct BenchPositions;

impl BenchPositions {
    fn starting_position() -> Board {
        Board::default()
    }

    fn middlegame_position() -> Board {
        Board::from_str("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 4").unwrap()
    }

    fn tactical_position() -> Board {
        Board::from_str("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 4").unwrap()
    }

    fn endgame_position() -> Board {
        Board::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap()
    }
}

// Micro-benchmark: Board evaluation function
fn bench_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluation");

    let positions = [
        ("starting", BenchPositions::starting_position()),
        ("middlegame", BenchPositions::middlegame_position()),
        ("tactical", BenchPositions::tactical_position()),
        ("endgame", BenchPositions::endgame_position()),
    ];

    for (name, board) in positions.iter() {
        group.bench_with_input(BenchmarkId::new("default_eval", name), board, |b, board| b.iter(|| Evaluator::evaluate(black_box(*board))));

        let chromosome = Chromosome::new_random(0.2);
        group.bench_with_input(BenchmarkId::new("chromosome_eval", name), board, |b, board| {
            b.iter(|| Evaluator::evaluate_with_chromosome(black_box(*board), black_box(&chromosome)))
        });
    }

    group.finish();
}

// Micro-benchmark: Move generation and ordering
fn bench_move_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_generation");

    let positions = [
        ("starting", BenchPositions::starting_position()),
        ("middlegame", BenchPositions::middlegame_position()),
        ("tactical", BenchPositions::tactical_position()),
    ];

    for (name, board) in positions.iter() {
        group.bench_with_input(BenchmarkId::new("legal_moves", name), board, |b, board| {
            b.iter(|| {
                let moves: Vec<_> = MoveGen::new_legal(black_box(board)).collect();
                black_box(moves)
            })
        });

        let alg = AlphaBetaAlgorithm::new();
        group.bench_with_input(BenchmarkId::new("ordered_moves", name), board, |b, board| {
            b.iter(|| {
                let moves = alg.get_ordered_moves(black_box(board));
                black_box(moves)
            })
        });
    }

    group.finish();
}

// Micro-benchmark: Alpha-beta search at different depths
fn bench_alpha_beta_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("alpha_beta_search");
    group.sample_size(10); // Reduce sample size for longer-running benchmarks

    let positions = [
        ("starting", BenchPositions::starting_position()),
        ("middlegame", BenchPositions::middlegame_position()),
        ("endgame", BenchPositions::endgame_position()),
    ];

    let depths = [2, 3, 4];

    for depth in depths.iter() {
        for (pos_name, board) in positions.iter() {
            let bench_name = format!("{}_depth_{}", pos_name, depth);

            group.bench_with_input(BenchmarkId::new("search", &bench_name), &(board, depth), |b, (board, depth)| {
                b.iter(|| {
                    let mut alg = AlphaBetaAlgorithm::new();
                    let (_move, stats) = alg.get_best_move_with_stats(black_box(**board), black_box(**depth));
                    black_box((stats.nodes_searched, stats.evaluations))
                })
            });
        }
    }

    group.finish();
}

// Micro-benchmark: Single alpha-beta function calls
fn bench_alpha_beta_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("alpha_beta_functions");

    let board = BenchPositions::middlegame_position();

    group.bench_function("alpha_beta_max_depth_2", |b| {
        b.iter(|| {
            let mut alg = AlphaBetaAlgorithm::new();
            alg.alpha_beta_max(black_box(board), black_box(-999999), black_box(999999), black_box(2), black_box(None))
        })
    });

    group.bench_function("alpha_beta_min_depth_2", |b| {
        b.iter(|| {
            let mut alg = AlphaBetaAlgorithm::new();
            alg.alpha_beta_min(black_box(board), black_box(-999999), black_box(999999), black_box(2), black_box(None))
        })
    });

    group.finish();
}

// Benchmark: Complete position analysis (end-to-end)
fn bench_complete_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("complete_analysis");
    group.sample_size(10);

    let positions = [("tactical_position", BenchPositions::tactical_position()), ("endgame_position", BenchPositions::endgame_position())];

    for (name, board) in positions.iter() {
        group.bench_with_input(BenchmarkId::new("depth_3", name), board, |b, board| {
            b.iter(|| {
                let mut alg = AlphaBetaAlgorithm::new();
                let best_move = alg.get_best_move(black_box(*board), black_box(3));
                black_box(best_move)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_evaluation,
    bench_move_generation,
    bench_alpha_beta_search,
    bench_alpha_beta_functions,
    bench_complete_analysis
);
criterion_main!(benches);
