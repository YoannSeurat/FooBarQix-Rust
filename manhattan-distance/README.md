# Manhattan Distance

The Manhattan distance (or taxicab distance) is the distance between two points in a grid (like the grid-like street geography of the New York borough of Manhattan) calculated by only taking a vertical and/or horizontal path.

This is a small Rust command-line program that outputs the Manhattan distance between two points of coordinates you will have entered.

## How it works

The Manhattan distance, between two points $p = (p_1, p_2, ..., p_n)$ and $q = (q_1, q_2, ..., q_n)$, in an n-dimensional real coordinate space is : $\sum_{i=1}^n |p_i - q_i|$

## Run

```bash
cargo run
```

Then enter the coordinates for the first and second point when prompted.

## Example

```text
Please enter the first point's coordinates in <x, y> format :
1, 1
Please enter the second point's coordinates :
2, 4
  >  The distance between the two points is 4.

Please enter the first point's coordinates in <x, y> format :
-3, 2  
Please enter the second point's coordinates :
12, -9
  >  The distance between the two points is 26.

Please enter the first point's coordinates in <x, y> format :
1
Too few arguments !

Please enter the first point's coordinates in <x, y> format :
1, 1, 1
Too many arguments !

Please enter the first point's coordinates in <x, y> format :
hello
Wrong number format ! invalid digit found in string
```