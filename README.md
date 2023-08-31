# Brainf-Interpreter - Fast Brainfuck Interpreter in Rust

Welcome to the Brainf-Interpreter project! This is a fast interpreter for the Brainfuck programming language, written in Rust. Brainfuck is a minimalist programming language known for its simplicity and challenging nature.

## About Brainfuck

Brainfuck is an esoteric programming language that operates on an array of memory cells, each initially set to zero. The language consists of a small set of commands, each consisting of a single character. These commands manipulate the memory cells, allowing you to perform computations. It's a fun and intriguing language often used for educational and recreational purposes.

## Getting Started

To run the Brainfuck interpreter, follow these steps:

1. Make sure you have Rust installed on your system. You can download and install it from [here](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine:

   ```sh
   git clone https://github.com/rouzbehsbz/brainf-interpreter.git
3. Navigate to the cloned directory:

    ```sh
    cd brainf-interpreter
4. Run the interpreter with a Brainfuck source file:
    ```sh
    cargo run <source_path>
Replace <source_path> with the path to your Brainfuck source file.

## Example
In this repository, you'll find a sample Brainfuck source file named source.bf. It contains the classic "Hello World" program written in Brainfuck. To execute it, simply run:

    ```sh
    cargo run source.bf