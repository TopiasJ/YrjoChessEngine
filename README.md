Point of this project was to learn about Rust programming language. 

This is initially trying to duplicate my earlier work on python and trying to have good performance. https://github.com/TopiasJ/ChessEngine (Still missing all evolutionary algrorithm stuffs). 

And maybe later it will be used to test different AI stuffs, especially Deep Learning. And could join to AI chess tournaments.

### commands
cargo clippy

cargo clippy --fix

cargo fmt

cargo upgrade

cargo build

cargo build --profile release-lto

// cargo bench

## 🖥️ Command-Line Usage

The engine now supports two main modes via subcommands:

### Single Game Mode

Run a single chess game between two AIs. Example (fast run):
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
- `--wanted-chromosome-count` sets the number of chromosomes (default: 10)
- `--depth` sets the calculation depth (default: 5)
