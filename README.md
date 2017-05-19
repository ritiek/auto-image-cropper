# Auto-Image-Cropper

- Removes extra white boundaries from images to correctly resize canvas.

## Screenshots

The borders are just to represent the actual images tested with.
(click to zoom)

<img src="http://i.imgur.com/3pc600q.jpg" width="200">            <img src="http://i.imgur.com/nMR1ZuV.jpg" width="200">

<img src="http://i.imgur.com/QIXGDCk.jpg" width="230">            <img src="http://i.imgur.com/NTfeN3e.jpg" width="200">

## Installation and Usage

`git clone https://github.com/Ritiek/auto-image-cropper`

`cd auto-image-cropper`

`cargo build --release`

`cd target/release`

`./auto-image-cropper`

## Benchmarks

- This tool was re-written in [Python](python/) to compare with Rust just for fun.

- [table]

- Python struggles to find the optimal coordinates but is quick (quicker than Rust) when saving the cropped image back to disk. Rust really outperforms when finding the optimal coordinates.

## Contributing

- Found a bug/ have an idea? Feel free to open your ticket in the [issues section](../../issues).

- Even better, send a pull request. :)

## License

`The MIT License`
