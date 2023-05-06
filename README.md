# number-benchmark
Benchmark comparing rust data types. 
## Overview
This project compares the execution times of different data type additions.  
It does this by initializing a scratchpad of random values and adding each even value to the one following it.  
Each data type has its own set of functions to reduce compiler optimizations skewing the results.
Please keep in mind that this models a very specific use case.  
## Example Results
Performed on AMD Ryzen 7 2700X @ 3.8 GHz
### Release build

```
Number Benchmark v0.1.1
Scratchpad length:65536 Rounds:1024
Performing wrapping integer addition benchmark.
u8   Average: 63.91µs Std.dev: 663.00ns
u16  Average: 67.25µs Std.dev: 193.00ns
u32  Average: 68.79µs Std.dev: 922.00ns
u64  Average: 71.02µs Std.dev: 257.00ns
u128 Average: 103.31µs Std.dev: 412.00ns
Performing float addition benchmark on values between 0 and 1.
f32  Average: 67.37µs Std.dev: 652.00ns
f64  Average: 84.11µs Std.dev: 448.00ns
Performing float addition benchmark on any float value.
f32  Average: 84.09µs Std.dev: 334.00ns
f64  Average: 67.56µs Std.dev: 1.33µs
Total runtime: 4.05s
```

### Debug build

```
Number Benchmark v0.1.1
Scratchpad length:65536 Rounds:1024
Performing wrapping integer addition benchmark.  
u8   Average: 1.85ms Std.dev: 4.02µs  
u16  Average: 1.85ms Std.dev: 10.34µs  
u32  Average: 1.93ms Std.dev: 3.93µs  
u64  Average: 1.82ms Std.dev: 3.98µs  
u128 Average: 1.91ms Std.dev: 19.14µs  
Performing float addition benchmark on values between 0 and 1.  
f32  Average: 2.01ms Std.dev: 85.60µs  
f64  Average: 1.82ms Std.dev: 4.13µs  
Performing float addition benchmark on any float value.  
f32  Average: 1.99ms Std.dev: 17.21µs  
f64  Average: 1.82ms Std.dev: 5.10µs  
Total runtime: 93.30s
```

## Other
This work uses the [fastrand](https://github.com/smol-rs/fastrand) crate licensed under Apache License 2.0.  
Published under Apache License 2.0.   