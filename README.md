Ray tracing in Rust.

I followed the C++ book [Ray Tracing in One Weekend](https://raytracing.github.io) by Peter Shirley.

There are a couple differences:
- a cli is available, you can emit a png file or display the image in a window
- the tracing is done in parallel (thanks to Rust and [rayon](https://github.com/rayon-rs/rayon) it was easy enough)

It was fun and a good learning experience.

![example.png](example-1600x800.png?raw=true)

# How to build

```
# You really want the release build to try it out
cargo build --release
```

# How to run

```
./target/release/raytracer -h
```


```
raytracer 0.1.0
Toy raytracer

USAGE:
    raytracer [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --window     Display the result in a floating window

OPTIONS:
    -f, --file <file>    Store the resulting image to a file

ARGS:
    <width>     Width of the scene [default: 200]
    <height>    Height of the scene [default: 100]
```
