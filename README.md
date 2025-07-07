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

- Run a single chess game between two AIs. Example (fast run):
  ```sh
  cargo run -- single --depth 2 --max-moves 30
  ```

- Set calculation depth to 3:
  ```sh
  cargo run -- single --depth 3
  ```
- Start from a custom FEN position:
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
