# Lights and Colors Out

Solvers for BatMUD gala 2025 Burglefloogah quest parts Lights Out and Colors Out.

## Installation

``` sh
cargo install
```

## Running

You must give the starting board state as a parameter so that all cell statuses
(1s and 0s) are combined starting from top left:

```
$ cargo build --release
$ ./target/release/lights-colors-out --lights-out 0010001001100000000100111

Starting state:
   +---+---+---+---+---+
 1 | 0 | 0 | 1 | 0 | 0 |
   +---+---+---+---+---+
 2 | 0 | 1 | 0 | 0 | 1 |
   +---+---+---+---+---+
 3 | 1 | 0 | 0 | 0 | 0 |
   +---+---+---+---+---+
 4 | 0 | 0 | 0 | 0 | 1 |
   +---+---+---+---+---+
 5 | 0 | 0 | 1 | 1 | 1 |
   +---+---+---+---+---+
     A   B   C   D   E

Processed 100000 moves, queue size: 925392
Processed 200000 moves, queue size: 1844768
Processed 300000 moves, queue size: 2755323
Processed 400000 moves, queue size: 3663857
Processed 500000 moves, queue size: 4574484
Processed 600000 moves, queue size: 5474915
Processed 700000 moves, queue size: 6403923
Moves: <omitted!>
```

And colors out respectively:

```
$ ./target/release/lights-colors-out --colors-out 0001110010101010101010111

Starting state:
   +---+---+---+---+---+
 1 | 0 | 0 | 0 | 1 | 1 |
   +---+---+---+---+---+
 2 | 1 | 0 | 0 | 1 | 0 |
   +---+---+---+---+---+
 3 | 1 | 0 | 1 | 0 | 1 |
   +---+---+---+---+---+
 4 | 0 | 1 | 0 | 1 | 0 |
   +---+---+---+---+---+
 5 | 1 | 0 | 1 | 1 | 1 |
   +---+---+---+---+---+
     A   B   C   D   E

Processed 100000 moves, queue size: 994625
Processed 200000 moves, queue size: 1943993
Processed 300000 moves, queue size: 2894752
Processed 400000 moves, queue size: 3819506
Processed 500000 moves, queue size: 4811605
Processed 600000 moves, queue size: 5765766
Processed 700000 moves, queue size: 6717831
Processed 800000 moves, queue size: 7696713
Processed 900000 moves, queue size: 8615618
Processed 1000000 moves, queue size: 9532706
Processed 1100000 moves, queue size: 10420538
Processed 1200000 moves, queue size: 11329814
Processed 1300000 moves, queue size: 12203494
Processed 1400000 moves, queue size: 13116323
Processed 1500000 moves, queue size: 14046555
Processed 1600000 moves, queue size: 14964300
Processed 1700000 moves, queue size: 15855294
Processed 1800000 moves, queue size: 16754226
Processed 1900000 moves, queue size: 17669436
Processed 2000000 moves, queue size: 18591937
Processed 2100000 moves, queue size: 19523909
Processed 2200000 moves, queue size: 20408326
Processed 2300000 moves, queue size: 21288937
Processed 2400000 moves, queue size: 22207868
Processed 2500000 moves, queue size: 23106098
Moves: <omitted!>
```
