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
