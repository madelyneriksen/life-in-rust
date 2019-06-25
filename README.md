# Conway's Game of Life in Rust

Wow, compiled code is _really fast_. This implementation of Life uses a `HashSet` for speedy checks over a sparse "array".

## Getting Started 🚀

There are no dependencies beyond the standard library, a stable rust compiler, and `cargo`. Simply clone and compile!

```bash
git clone https://github.com/madelyneriksen/life-in-rust .
cargo build --release
```

## Life With Hashmaps (or sets)

I originally talked about this in a [blog post](https://www.madelyneriksen.com/python-game-of-life) where I used a Python dictionary to create a very efficient version of Life. The same algorithm is used in the Python and Rust implementations, so if you're curious check it out!

Of course, Rust is way faster. 🔥

## License

* MIT
