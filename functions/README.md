# Functions

This is writing basic functions to work with numbers, vectors, integers, etc.

```bash
$ cargo build
```

And then to run:

```bash
$ ./target/debug/functions
The sum is 20
Starting vector is [1, 2, 3, 4, 4, 5, 1]
Deduped vector is [1, 2, 3, 4, 5]
Before filtering [1, 2, 3, 4, 4, 5, 1]
After filtering [2, 4, 4]
Matrix 1 [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
Matrix 2 [[5.0, 5.0, 5.0], [5.0, 5.0, 5.0], [5.0, 5.0, 5.0]]
Empty Matrix to fill [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
Finished matrix: [[15.0, 15.0, 15.0], [15.0, 15.0, 15.0], [15.0, 15.0, 15.0]]
Starting number for sieve is 7
Final list of primes is [2, 3, 5]
Final moves are [(A, C), (A, B), (C, A), (A, C), (B, C), (B, A), (C, B)]
```

To test:

```bash
$ cargo test
   Compiling functions v0.1.0 (/home/vanessa/Documents/Dropbox/Code/rust/learning-rust/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running target/debug/deps/functions-172f13adfafdc399

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/functions-c6dc6866e8eb94e8

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/tests-6c45e746b22db32b

running 6 tests
test test_filter_small ... ok
test test_hanoi_1_disks ... ok
test test_dedup_small ... ok
test test_mat_mult_identity ... ok
test test_sieve_basic ... ok
test test_sum_small ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests functions

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
