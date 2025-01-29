mod fib;

// time linear_fib(40)
// time recursive_fib(40)
// time recursive_memoized_fib(40)

use fib::*;

use std::time::Instant;

fn time_function<F, A>(f: F, arg: A) -> u128
where
    F: Fn(A) -> usize,
{
    let start = Instant::now();
    f(arg);
    let duration = start.elapsed();
    duration.as_micros()
}

fn main() {
    let n = 45;
    let linear_fib_time = time_function(linear_fib, n);
    let recursive_fib_time = time_function(recursive_fib, n);
    let recursive_memoized_fib_time = time_function(recursive_memoized_fib, n);

    println!("linear_fib({}) took {} microseconds", n, linear_fib_time);
    println!("recursive_fib({}) took {} microseconds", n, recursive_fib_time);
    println!("recursive_memoized_fib({}) took {} microseconds", n, recursive_memoized_fib_time);
}
