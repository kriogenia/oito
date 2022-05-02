# OitoCore for SDL2

Implementation of a desktop front-end to run the **OitoCore** and complete the emmulator.
This implementation uses SDL2 and is configurable.

![Emmulator running Tetris](../res/tetris.jpg)

To run the emulator you only need to use the following command:

```sh
cargo run --release path/to/rom
```

In case that you want to customize the window running the emmulator you can use the following flags:
* `--scale` (`-s`). Accepts and integer and it serves to amplify the original 64x48px window size. By default it's set to 20.
* `--bg` (`-b`). Accepts a color in hex RGB format, that color will be used to draw the background. By default, will be black.
* `--fg` (`-f`). Accepts a color in hex RGB format, that color will be used to draw the foreground. By default, will be white.

The following example would run the emmulator with red tones and a bit smaller window than the default one.

```sh
cargo run path/to/rom --scale 15 --bg "#220000" --fg "#FFDDDD"
```