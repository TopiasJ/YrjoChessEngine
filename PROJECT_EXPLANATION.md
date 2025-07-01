# YrjoChessEngine - Project Explanation

A comprehensive guide to understanding how the YrjoChessEngine works, its architecture, and implementation details.

## 🏗️ Architecture Overview

The chess engine is built with a modular architecture consisting of four main components:

### 1. **Alpha-Beta Algorithm** (`src/alpha_beta_algorithm.rs`)
The core decision-making engine that implements the Alpha-Beta pruning algorithm for efficient game tree search.

**Key Features:**
- **Alpha-Beta Pruning**: Optimizes the minimax algorithm by eliminating branches that won't affect the final decision
- **Move Ordering**: Prioritizes captures and tactical moves for better pruning efficiency
- **Random Selection**: When multiple moves have equal evaluation, randomly selects one to add variety
- **Depth Control**: Configurable search depth (default: 5 half-moves)

**How it works:**
```rust
// The algorithm evaluates positions by recursively searching the game tree
fn alpha_beta_max(&self, board: Board, alpha: i32, beta: i32, depth: i32) -> i32
fn alpha_beta_min(&self, board: Board, alpha: i32, beta: i32, depth: i32) -> i32
```

### 2. **Position Evaluator** (`src/evaluator.rs`)
Evaluates chess positions using material counting and piece values.

**Piece Values:**
- Pawn: 100 points
- Knight/Bishop: 300 points each
- Rook: 500 points
- Queen: 900 points
- King: 10,000 points

**Evaluation Method:**
- Positive scores favor White
- Negative scores favor Black
- Uses bitboard operations for efficient piece counting

### 3. **Board Visualizer** (`src/visualizer.rs`)
Provides human-readable chess board visualization using Unicode chess pieces.

**Features:**
- Unicode chess piece symbols (♔♕♖♗♘♙ for White, ♚♛♜♝♞♟ for Black)
- Standard chess notation (a-h, 1-8)
- Clear board representation

### 4. **Main Game Loop** (`src/main.rs`)
Orchestrates the chess game, handling move generation, board updates, and game termination.

## 🚀 Getting Started

### Prerequisites
- Rust (1.87.0 or later)
- Visual Studio Build Tools (Windows) or equivalent C++ compiler

### Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd YrjoChessEngine
   ```

2. **Install Visual Studio Build Tools (Windows only):**
   ```bash
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```
   Then install the C++ workload through the Visual Studio Installer.

3. **Run the chess engine:**
   ```bash
   cargo run
   ```

## 🎮 How to Use

### Running the Engine
```bash
cargo run
```

The engine will:
1. Start with a standard chess position
2. Play moves automatically using the Alpha-Beta algorithm
3. Display the board after each move
4. Show evaluation scores
5. Continue until checkmate, stalemate, or draw

### Configuration
You can modify the search depth in `src/main.rs`:
```rust
const CALCULATION_HALF_DEPTH: i32 = 5; // Adjust this value
```

### Testing
Run the test suite:
```bash
cargo test
```

**Available Tests:**
- `checkmate1`: Tests checkmate in 1 move
- `checkmate2`: Tests checkmate in 2 moves  
- `checkmate3`: Tests checkmate in 3 moves
- `checkmate4-8`: Ignored due to long execution time

**Run ignored tests individually:**
```bash
cargo test checkmate4 -- --ignored
```

## 🖥️ Command-Line Usage

The chess engine now uses subcommands for flexible control:

### Single Game Mode

Run a single chess game between two AIs. Example (fast run):
```sh
cargo run -- single --depth 2 --max-moves 30
```

You can use all the previous options (depth, fen, max-moves, show-board, show-eval, delay) with the `single` subcommand:
- Set calculation depth to 3:
  ```sh
  cargo run -- single --depth 3
  ```
- Start from a custom FEN position (example: Italian Game, Giuoco Piano):
  ```sh
  cargo run -- single --fen "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 2 4"
  ```
- Limit the number of moves to 50:
  ```sh
  cargo run -- single --max-moves 50
  ```
- Hide board visualization and evaluation:
  ```sh
  cargo run -- single --show-board false --show-eval false
  ```
- Add a delay between moves (e.g., 500 ms):
  ```sh
  cargo run -- single --delay 500
  ```

You can combine these options as needed. For example:
```sh
cargo run -- single --depth 4 --fen "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 2 4" --max-moves 60 --show-board true --show-eval true --delay 200
```

### Tournament Mode

Run a tournament between chromosomes (for evolutionary algorithm experiments):
```sh
cargo run -- tournament --wanted-chromosome-count 4 --depth 2
```
- `--wanted-chromosome-count` sets the number of chromosomes (default: 10)
- `--depth` sets the calculation depth (default: 5)

## 🔧 Technical Details

### Algorithm Implementation

**Alpha-Beta Pruning:**
- **Alpha**: Best score for the maximizing player (White)
- **Beta**: Best score for the minimizing player (Black)
- **Pruning**: Eliminates branches that won't affect the final decision

**Move Generation:**
- Uses the `chess` crate for legal move generation
- Prioritizes captures for better move ordering
- Handles special chess rules (en passant, castling, etc.)

**Performance Optimizations:**
- Bitboard operations for efficient piece counting
- Inline functions for critical evaluation code
- Move ordering to improve pruning efficiency

### Alpha-Beta Algorithm Improvements

The current implementation can be enhanced with several optimizations:

#### 1. **Enhanced Move Ordering**
**Current State:** Basic capture prioritization
**Improvement:** Implement comprehensive move scoring
```rust
fn move_score(mov: ChessMove, board: &Board) -> i32 {
    let mut score = 0;
    
    // Captures (MVV-LVA: Most Valuable Victim - Least Valuable Attacker)
    if let Some(captured_piece) = board.piece_on(mov.get_dest()) {
        score += piece_value(captured_piece) * 10;
        score -= piece_value(board.piece_on(mov.get_source()).unwrap());
    }
    
    // Promotions
    if mov.get_promotion().is_some() {
        score += 900; // Queen promotion value
    }
    
    // Checks
    let new_board = board.make_move_new(mov);
    if new_board.checkers() != &EMPTY {
        score += 50;
    }
    
    score
}
```

#### 2. **Transposition Tables**
**Purpose:** Cache evaluated positions to avoid re-computation
**Implementation:**
```rust
use std::collections::HashMap;

struct TranspositionTable {
    table: HashMap<u64, TranspositionEntry>,
}

struct TranspositionEntry {
    depth: i32,
    score: i32,
    best_move: Option<ChessMove>,
    node_type: NodeType, // EXACT, LOWER_BOUND, UPPER_BOUND
}
```

#### 3. **Quiescence Search**
**Purpose:** Continue searching captures at leaf nodes to avoid horizon effect
**Implementation:**
```rust
fn quiescence_search(board: Board, alpha: i32, beta: i32) -> i32 {
    let stand_pat = Evaluator::evaluate(board);
    if stand_pat >= beta {
        return beta;
    }
    
    let mut alpha = alpha.max(stand_pat);
    
    // Only search captures
    let captures = get_capture_moves(&board);
    for mov in captures {
        let new_board = board.make_move_new(mov);
        let score = -quiescence_search(new_board, -beta, -alpha);
        
        if score >= beta {
            return beta;
        }
        alpha = alpha.max(score);
    }
    
    alpha
}
```

#### 4. **Principal Variation Search (PVS)**
**Purpose:** Optimize search by assuming the first move is the best
**Implementation:**
```rust
fn pvs_search(board: Board, depth: i32, alpha: i32, beta: i32, is_pv: bool) -> i32 {
    if depth == 0 {
        return quiescence_search(board, alpha, beta);
    }
    
    let moves = get_ordered_moves(&board);
    let mut alpha = alpha;
    
    for (i, mov) in moves.iter().enumerate() {
        let new_board = board.make_move_new(*mov);
        let score = if i == 0 || is_pv {
            // Full window search for first move or PV nodes
            -pvs_search(new_board, depth - 1, -beta, -alpha, true)
        } else {
            // Null window search for other moves
            let score = -pvs_search(new_board, depth - 1, -alpha - 1, -alpha, false);
            if score > alpha && score < beta {
                // Re-search with full window
                -pvs_search(new_board, depth - 1, -beta, -alpha, true)
            } else {
                score
            }
        };
        
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    
    alpha
}
```

#### 5. **Iterative Deepening**
**Purpose:** Search incrementally to find the best move quickly
**Implementation:**
```rust
fn iterative_deepening(board: Board, max_depth: i32) -> ChessMove {
    let mut best_move = None;
    
    for depth in 1..=max_depth {
        let result = search_with_depth(board, depth);
        best_move = Some(result.best_move);
        
        // Check if we have enough time for next iteration
        if !has_time_remaining() {
            break;
        }
    }
    
    best_move.unwrap()
}
```

#### 6. **Null Move Pruning**
**Purpose:** Skip moves that are clearly bad by testing if opponent can win without moving
**Implementation:**
```rust
fn null_move_pruning(board: Board, depth: i32, alpha: i32, beta: i32) -> i32 {
    if depth <= 3 || has_pawn_advance(board) {
        return regular_search(board, depth, alpha, beta);
    }
    
    // Make null move
    let null_board = board.null_move();
    let score = -search(null_board, depth - 3, -beta, -beta + 1);
    
    if score >= beta {
        return beta; // Null move pruning successful
    }
    
    // Continue with regular search
    regular_search(board, depth, alpha, beta)
}
```

### Dependencies
- **chess**: Chess rules and move generation
- **rand**: Random number generation for move selection
- **criterion**: Benchmarking (dev dependency)

## 📊 Performance

The engine's performance depends on:
- **Search Depth**: Deeper searches take exponentially longer
- **Position Complexity**: Tactical positions require more analysis
- **Hardware**: CPU speed affects search speed

**Typical Performance:**
- Depth 5: ~1-5 seconds per move
- Depth 6: ~10-30 seconds per move
- Depth 7+: Several minutes per move

## 🧪 Testing Strategy

The test suite validates the engine's ability to find checkmates:
- **Fast Tests**: checkmate1-3 (runs in seconds)
- **Slow Tests**: checkmate4-8 (ignored by default)
- **Position Validation**: Ensures correct move counts for known positions

## 🔮 Future Enhancements

Potential improvements:
- **Opening Book**: Pre-computed opening moves
- **Endgame Tablebases**: Perfect play in endgames
- **Transposition Tables**: Cache evaluated positions
- **Parallel Search**: Multi-threaded move evaluation
- **UCI Protocol**: Interface with chess GUIs
- **Machine Learning**: Neural network evaluation

## 📝 License

This project is open source. Feel free to contribute improvements or use it as a learning resource for chess engine development.

## 🤝 Contributing

Contributions are welcome! Areas for improvement:
- Performance optimizations
- Additional evaluation features
- Better move ordering
- UCI protocol implementation
- Documentation improvements

---

**Note**: This chess engine is designed for educational purposes and demonstrates fundamental concepts in game AI and chess programming. 