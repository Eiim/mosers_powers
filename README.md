# Mosers Powers

A program to test the sequence A000127 (the solutions to [Mosers' circle problem](https://en.wikipedia.org/wiki/Dividing_a_circle_into_areas)) for powers of 2.

Currently tested up to x=2,000,000. That means we have determined that there are no non-trivial powers of 2 that appear in A000127 with exponents less than 2 million. That roughly corresponds to locations in the sequence with 150,000 decimal digits. Testing is currently being run continously on a small server so this will be slightly out of date.

It's also my first Rust program. I decided a low-level approach would be necessary for the greatest speed, and I took that as an excuse to learn a bit of Rust.

Currently uses [rug](https://crates.io/crates/rug) for big integer parsing, which wraps [GMP](https://gmplib.org/).

## Technical details

The program relies on the fact that A000127 is a quartic, and thus grows much slower than the powers of 2. The goal is to find the two consecutive locations in the sequence which produce values less than and greater than our target power of 2. Because the sequence is a quartic, to double the value, we multiply the location in the sequence by 2^(1/4). This gets us extremely close to the target, usually just one or two spots away, even for extremely large indicies.

However, for this to work, we need extremely precise approximations of 2^(1/4), or we'll quickly drift far away from our intended target. To generate these we iteratively extend the binary fraction corresponding to 2^(1/4), which allows for fast binary arithmetic rather than slow integer division.

Values of A000127 are calculated using [Horner's Method](https://en.wikipedia.org/wiki/Horner%27s_method), using just three multiplications, three small additions/subtractions, one left-shift, and a divide by three.

# Future work

I expect to put work into this only occasionally. But possible future directions include:
* Improved polynomial calculation algorithms
* Parallelization
* GPU code
