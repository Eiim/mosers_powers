# Mosers Powers

A program to test the sequence A000127 (the solutions to [Mosers' circle problem](https://en.wikipedia.org/wiki/Dividing_a_circle_into_areas)) for powers of 2.

Currently tested up to x=69884. That means we have determined that there are no non-trivial powers of 2 that appear in A000127 with exponents less than 69884. That roughly corresponds to locations in the sequence with 5,200 decimal digits.

It's also my first Rust program. I decided a low-level approach would be necessary for the greatest speed, and I took that as an excuse to learn a bit of Rust.

Currently uses [num_bigint](https://github.com/rust-num/num-bigint) for big integer parsing.

## Technical details

The program relies on the fact that A000127 is a quartic, and thus grows much slower than the powers of 2. The goal is to find the two consecutive locations in the sequence which produce values less than and greater than our target power of 2. Because the sequence is a quartic, to double the value, we multiply the location in the sequence by 2^(1/4). This gets us extremely close to the target, usually just one or two spots away, even when we have indicies in the range of 10^5000.

However, for this to work, we need extremely precise approximations of 2^(1/4), or we'll quickly drift far away from our intended target. To generate these we use a generalized continous fraction for 2^(1/4) which follows a [very nice pattern](https://en.wikipedia.org/wiki/Generalized_continued_fraction#Roots_of_positive_numbers). We can then turn this into a large fraction quite easily, and from there it is easy to use for our approximation.

Values of A000127 are calculated directly from the binomial coefficient representation. There's quite possibly room for improvement in this area.

# Future work

I'm continously improving this program for now, so this document may be out of date. But possible future directions include:

* Parallelization
* GPU code
* Changing to a better method to multiply by 2^(1/4) that doesn't rely on large integer division
* Switching big int libraries to something more preformant