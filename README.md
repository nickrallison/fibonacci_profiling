# Fibonacci Profiling


## Description

This project is a small demo to profile the performance of several different Fibonacci functions.

## Results

The results of the profiling are as follows (On a Ryzen 9 7950X3D):

| Function | Time (microseconds) |
| --- | --- |
| linear_fib(45) | 3 |
| recursive_fib(45) | 1829108 |
| recursive_memoized_fib(45) | 24 |

## Usage

All that is needed to run is cargo

Run the following command: `cargo run --release`

![image](https://github.com/user-attachments/assets/29f64e45-e595-4af6-9e10-bc5029d5ca40)
