# FooBarQix

A small Rust command-line program that prints FooBarQix labels for numbers from `0` up to the maximum value you enter.

## How it works

For each number :
- `Foo` for multiples of `3`
- `Bar` for multiples of `5`
- `Qix` for multiples of `7`
- adds `Foo`, `Bar`, and/or `Qix` if the digits `3`, `5`, and `7` are found in the number itself 
- prints the number if none of the rules apply

## Run

```bash
cargo run
```

Then enter a maximum number when prompted.

## Example

```text
Enter max number :
15
0 => FooBarQix
1 => 1
2 => 2
3 => FooFoo
4 => 4
5 => BarBar
6 => Foo
7 => QixQix
8 => 8
9 => Foo
10 => Bar
11 => 11
12 => Foo
13 => Foo
14 => Qix
```