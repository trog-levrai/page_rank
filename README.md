# Page Rank

A simple project implementing the Page Rank algorithm in Rust. It's a small
homework intended to learn the language.

## Input

The input is a CSV with 2 columns: `src`, `dst`.
This is describing the graph we want to compute PR for.

## Output

The output is a CSV with 2 columns: `node`, `score`

## Usage

To compute Page Rank, an argument has to be provided:

- Damping factor as a floating point number between 0 and 1.

The program reads the input graph from STDIN and outputs result in STDOUT.
