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

You can now run the chess engine with various command-line options:

- Run with default settings:
  ```sh
  cargo run
  ```

- Set calculation depth to 3:
  ```sh
  cargo run -- --depth 3
  ```

- Start from a custom FEN position:
  ```sh
  cargo run -- --fen "7k/8/5K2/8/8/8/8/5R2 w - - 0 1"
  ```

- Limit the number of moves to 50:
  ```sh
  cargo run -- --max-moves 50
  ```

- Hide board visualization and evaluation:
  ```sh
  cargo run -- --show-board false --show-eval false
  ```

- Add a delay between moves (e.g., 500 ms):
  ```sh
  cargo run -- --delay 500
  ```

You can combine these options as needed. For example:
```sh
cargo run -- --depth 4 --fen "7k/8/5K2/8/8/8/8/5R2 w - - 0 1" --max-moves 60 --show-board true --show-eval true --delay 200
```
