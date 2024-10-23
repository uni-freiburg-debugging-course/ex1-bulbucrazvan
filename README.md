[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/3MmVbb7f)
# Debugging 24/25 Exercise 1

Detailed instructions are in the exercise sheet. Following are your checkpoints:

- [ ] Implement parser and evaluator
- [ ] Implement a fuzzer
- [ ] Generate *lots* of random instances with your fuzzer. Your evaluator and z3 must return the same result on generated instances
- [ ] Provide detailed build instructions for your code so that we can evaluate it

I initially planned on making a proper lexer and parser, but also thought to try out Rust for the first time. What I didn't know is that
Rust isn't like moving from C# to Java or any other C-like language. Reading the book and trying to get used a bit to the borrow checker
took longer than expected, and was left with very little time to scramble and come up with this code that just does the required functionality. I have no choice but to submit this for now, but the whole thing will be scrapped and I will make a proper lexer and parser for the future.

Usage:
    Build using "cargo build".
    Run fuzzer with "cargo run --bin fuzzer [number of simplify lines to generate]"; creates a "test.txt" file with the output
    Run evaluator with "cargo run [path to input file eg test.txt]"; prints the result to console  