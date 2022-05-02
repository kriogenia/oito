# OitoCore for SDL2

Implementation of a desktop front-end to run the **OitoCore** and complete the emmulator.
This implementation uses SDL2 and is configurable.

![Emmulator running Tetris](../res/tetris.jpg)

## Running the emulator

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

## Using the emulator

Once the emulator starts, the specified ROM will be loaded and automatically started.
To use the emulator follow you will use the following mapping of the Chip-8 keyboard.

```
+---+---+---+---+         +---+---+---+---+
|   |   |   |   |         |   |   |   |   |
| 1 | 2 | 3 | 4 |         | 1 | 2 | 3 | C |
|   |   |   |   |         |   |   |   |   |
+---+---+---+---+         +---+---+---+---+
|   |   |   |   |         |   |   |   |   |
| Q | W | E | R |         | 4 | 5 | 6 | D |
|   |   |   |   |         |   |   |   |   |
+---+---+---+---+   -->   +---+---+---+---+
|   |   |   |   |         |   |   |   |   |
| A | S | D | F |         | 7 | 8 | 9 | E |
|   |   |   |   |         |   |   |   |   |
+---+---+---+---+         +---+---+---+---+
|   |   |   |   |         |   |   |   |   |
| Z | X | C | V |         | A | 0 | B | F |
|   |   |   |   |         |   |   |   |   |
+---+---+---+---+         +---+---+---+---+
```

### AZERTY and other distributions

Don't worry if you keyboard follows a different distribution than QWERTY. This front-end uses scancodes so what only matters is the key location. Just use the respective keys of your keyboard. For example (with AZERTY), the QWER road would be AZER to use the 4, 5, 6 and D Chip-8 keys.