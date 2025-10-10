Point of this project was to learn about Rust programming language. 

This is initially trying to duplicate my earlier work on python and trying to have good performance. https://github.com/TopiasJ/ChessEngine 

And maybe later it will be used to test different AI stuffs, especially Deep Learning. And could join to AI chess tournaments.

### Commands
```sh
cargo clippy
```
```sh
cargo clippy --fix
```
```sh
cargo fmt
```
```sh
cargo upgrade
```
```sh
cargo build
```
```sh
cargo build --profile release-lto
```
```sh
cargo test
```
```sh
cargo run --profile=release-lto -- single
```
```sh
cargo run --profile=release-lto -- single --depth 2 --max-moves 30
```
```sh
cargo run --profile=release-lto -- tournament
```
```sh
// cargo bench
```

## 🖥️ Command-Line Usage

The engine now supports two main modes via subcommands:

### Single Game Mode

Run a single chess game between two AIs:
```sh
cargo run -- single --depth 2 --max-moves 30
```

#### Parameters:
- `--depth` / `-d`: AI calculation depth in half-moves (default: 5, range: 1-10)
- `--fen` / `-f`: Starting position in FEN notation (optional, default: standard starting position)
- `--max-moves` / `-m`: Maximum number of moves before forcing a draw (default: 200)
- `--show-eval` / `-e`: Show evaluation scores after each move (default: true)
- `--show-board` / `-b`: Show board visualization after each move (default: true)
- `--delay` / `-l`: Delay between moves in milliseconds (default: 0)

#### Examples:
```sh
# Quick test game with low depth
cargo run -- single --depth 2 --max-moves 30

# Standard game with default settings
cargo run -- single

# Full strength game
cargo run --profile=release-lto -- single --depth 5

# Custom starting position
cargo run -- single --fen "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 2 4"

# Silent mode (no visualization)
cargo run -- single --show-board false --show-eval false

# With visualization delay (500ms between moves)
cargo run -- single --delay 500

# Limited game length
cargo run -- single --max-moves 50

# Combined options
cargo run -- single --depth 4 --fen "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 2 4" --max-moves 60 --delay 200
```

### Tournament Mode

Run a tournament between chromosomes (for evolutionary algorithm experiments):
```sh
cargo run -- tournament --wanted-chromosome-count 4 --depth 2
```

#### Parameters:
- `--wanted-chromosome-count` / `-n`: Number of chromosomes in tournament (default: 10)
- `--depth` / `-d`: AI calculation depth (default: 5)
- `--file-path` / `-f`: Path for persistent chromosome storage (optional)
- `--tournament-count` / `-t`: Number of tournaments to run (default: 0 = infinite)

#### Examples:
```sh
# Run infinite tournaments with 4 chromosomes (memory only)
cargo run -- tournament --wanted-chromosome-count 4 --depth 2

# Run with persistent evolution
cargo run -- tournament --wanted-chromosome-count 4 --depth 2 --file-path evolution.json

# Run exactly 5 tournaments
cargo run -- tournament --wanted-chromosome-count 4 --depth 2 --file-path evolution.json --tournament-count 5
```
