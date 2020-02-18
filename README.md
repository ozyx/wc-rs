# wc-rs

It's the UNIX command `wc`, except it's written in Rust. The goal is to perform better than `wc`, which is a goal that has been achieved by many before me. I'm trying to accomplish this without peeking at my predecessors' code, as an exercise.

## TODO

- Make args / usage match `wc`
- Make this thing faster than `wc`... it's slightly slower at the moment, and it shouldn't be.
- Support for multiple files