# gifscii

A CLI to animate GIFs in the terminal

![demo](./demo.gif)

# Usage

```
Usage: gifscii [OPTIONS] <INPUT>

Arguments:
  <INPUT>  URL/stdin/path to the GIF file

Options:
  -n, --no-loop          Loop the animation
  -f, --filter <FILTER>  Scaling filter [default: lanczos3] [possible values: lanczos3, gaussian, catmull-rom, triangle, nearest, none]
  -d, --debug            Print debug info - progress, frame time/rate, delays
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## Installation

### crates.io

```sh
cargo install gifscii
```

### From source

```sh
cargo install --git https://github.com/taep96/gifscii
```
