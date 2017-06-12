# Auto-Image-Cropper

- Removes extra white boundaries from images to correctly resize canvas.

- Thanks **[@matthewkmayer](https://github.com/matthewkmayer)** for bringing down cropping times!

## Screenshots

The borders are just to represent the actual images tested with.
(click to zoom)

<img src="http://i.imgur.com/3pc600q.jpg" width="240">            <img src="http://i.imgur.com/nMR1ZuV.jpg" width="200">

<img src="http://i.imgur.com/QIXGDCk.jpg" width="270">            <img src="http://i.imgur.com/NTfeN3e.jpg" width="200">

## Installation and Usage

```
cargo install auto-image-cropper
```

or if you like to live on the bleeding edge

```
git clone https://github.com/Ritiek/auto-image-cropper
cd auto-image-cropper
cargo install
```

Use `autocrop` to start using the tool.

```
USAGE:
    autocrop [OPTIONS] --input <LOCATION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <LOCATION>     Location of input image/directory
    -o, --output <LOCATION>    Location of output image/directory
```

## Benchmarks

- This tool was re-written in [Python](python/) to compare with Rust just for fun.

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

## Contributing

- Found a bug/ have an idea? Feel free to open your ticket in the [issues section](../../issues).

- Even better, send a pull request. :)

## License

`The MIT License`
