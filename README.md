# Mosers Powers

A program to test the sequence A000127 (the solutions to [Mosers' circle problem](https://en.wikipedia.org/wiki/Dividing_a_circle_into_areas)) for powers of 2.

Currently tested up to x=300000. That means we have determined that there are no non-trivial powers of 2 that appear in A000127 with exponents less than 300000. That roughly corresponds to locations in the sequence with 15,000 decimal digits.

It's also my first Rust program. I decided a low-level approach would be necessary for the greatest speed, and I took that as an excuse to learn a bit of Rust.

Currently uses [rug](https://crates.io/crates/rug) for big integer parsing, which wraps [GMP](https://gmplib.org/).

## Technical details

The program relies on the fact that A000127 is a quartic, and thus grows much slower than the powers of 2. The goal is to find the two consecutive locations in the sequence which produce values less than and greater than our target power of 2. Because the sequence is a quartic, to double the value, we multiply the location in the sequence by 2^(1/4). This gets us extremely close to the target, usually just one or two spots away, even when we have indicies in the range of 10^15000.

However, for this to work, we need extremely precise approximations of 2^(1/4), or we'll quickly drift far away from our intended target. To generate these we use a generalized continous fraction for 2^(1/4) which follows a [very nice pattern](https://en.wikipedia.org/wiki/Generalized_continued_fraction#Roots_of_positive_numbers). We can then turn this into a large fraction quite easily, and from there it is easy to use for our approximation.

Values of A000127 are calculated using [Horner's Method](https://en.wikipedia.org/wiki/Horner%27s_method), using just three multiplications, three small additions/subtractions, one left-shift, and a divide by three. The divide by three in particular may be a target for future improvement, but it is currently a small proportion of overall time.

# Future work

I expect to put work into this only occasionally. But possible future directions include:

* Improvements to 2^(1/4) calculation
  * Remember previous state to iterate from there, reducing wasted work
  * Use more "precise" algorithm to make smaller numerator and denominator lengths, thus making scaling faster
* Load and save state
* Parallelization
* GPU code
