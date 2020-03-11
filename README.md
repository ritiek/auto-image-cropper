# auto-image-cropper

[![Crates.io](https://img.shields.io/crates/v/auto-image-cropper.svg)](https://crates.io/crates/auto-image-cropper/) [![Build Status](https://travis-ci.org/ritiek/auto-image-cropper.svg?branch=master)](https://travis-ci.org/ritiek/auto-image-cropper/)

- Removes extra white boundaries from images to correctly resize canvas.

- Thanks **[@matthewkmayer](https://github.com/matthewkmayer)** for bringing down cropping times
  (see [#1](https://github.com/ritiek/auto-image-cropper/pull/1) and [#2](https://github.com/ritiek/auto-image-cropper/pull/2))!

## Screenshots

The borders are just to represent the actual images tested with.
(click to zoom)

<img src="http://i.imgur.com/3pc600q.jpg" width="240">            <img src="http://i.imgur.com/nMR1ZuV.jpg" width="200">

<img src="http://i.imgur.com/QIXGDCk.jpg" width="270">            <img src="http://i.imgur.com/NTfeN3e.jpg" width="200">

## Installation and Usage

```shell
$ cargo install auto-image-cropper
```

or if you like to live on the bleeding edge

```shell
$ git clone https://github.com/Ritiek/auto-image-cropper
$ cd auto-image-cropper
$ cargo build --release
```

Use `./target/release/autocrop` to start using the tool.

```shell
USAGE:
    autocrop [OPTIONS] --input <LOCATION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <LOCATION>     Location of input image/directory
    -o, --output <LOCATION>    Location of output image/directory
```

## Python Bindings

This tool also provides Python bindings via PyO3 (using Rust FFI). This feature can be
enabled by passing `--features "python-binding"` to cargo when compiling.

For example:
```shell
$ cargo build --release --features "python-binding"
```

This will generate a dynamic library (\*.so) on Linux machines with the name
`./target/release/libauto_image_cropper.so`.

By moving this dynamic library into your current working directory, it can be imported via Python
scripts using:
```python
>>> import libauto_image_cropper
# Returns the optimal top-left and bottom-right corner
# coordinates for a given image to be cropped
>>> (x1, y1), (x2, y2) = libauto_image_cropper.calculate_corners("/path/to/imgfile")
```
(I haven't checked this out on Windows or OSX, but should follow a similar procedure)

## Benchmarks

- This tool was hackishly re-written in [Python](python/) to compare with Rust - just for fun.

- The benchmarks were done on a MacBook Air running macOS Sierra 10.12.2.

|                  Image                  |  Python |  Rust  | Times Faster |
|:---------------------------------------:|:-------:|:------:|:------------:|
| [face.jpg](benchmarking/face.jpg)       |  0.867s | 0.155s |         5.59 |
| [square.png](benchmarking/square.png)   |  1.682s | 0.142s |        11.84 |
| [flowers.jpg](benchmarking/flowers.jpg) |  2.222s | 0.476s |         4.66 |
| [human.jpg](benchmarking/human.jpg)     |  2.362s | 0.294s |         8.02 |
| [pets.jpg](benchmarking/pets.jpg)       |  5.366s | 1.704s |         3.14 |
| [agent47.jpg](benchmarking/agent47.jpg) | 51.559s | 7.519s |         6.85 |

- Python struggles to find the optimal coordinates but is quick (quicker than Rust) when saving the cropped image back to disk. Rust really outperforms while finding the optimal coordinates.

**[2020 / 03] UPDATE:** These benchmarks were done in 2017. Rust and its image libraries have a come long way ahead and I
believe should now offer even better performance! I'll update these benchmarks when I'm able to.

## License

`The MIT License`
