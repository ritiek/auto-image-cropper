# Auto-Image-Cropper

- Removes extra white boundaries from images to correctly resize canvas.

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

## Benchmarks

- This tool was re-written in [Python](python/) to compare with Rust just for fun.

| Image      | Python | Rust | Times faster |
|------------|-------:|-----:|
|[face.jpg](benchmarking/face.jpg)|0.867s|0.155s|5.59|

- Python struggles to find the optimal coordinates but is quick (quicker than Rust) when saving the cropped image back to disk. Rust really outperforms while finding the optimal coordinates.

## Contributing

- Found a bug/ have an idea? Feel free to open your ticket in the [issues section](../../issues).

- Even better, send a pull request. :)

## License

`The MIT License`
