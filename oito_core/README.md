# OitoCore

OitoCore is the API of the emulator developed in a way that it can be used with any front-end. 
It exposes the `OitoCore` struct with all the functionality, the constants with the screen sizes and the `Key` enum
to use with the key event functions.

## API
OitoCore offers the following interface to use it and it's what's implemented in both front-ends:
* `new`, returns a new instance of the core with the preloaded sprites ready to be used.
* `load`, loads the bytes of the ROM to execute.
* `tick`, simulates a CPU tick. The first call to this function is the start of the execution of the loaded ROM. This should be used 10 times per frame rendering for max efficiency.
* `frame_tick`, simulates a frame tick. This should be called with each frame render.
* `frame_buffer`, returns the buffer representing the next frame to draw. It's made of booleans indicating if the pixel should be drawn or not (Chip8 was black and white).
* `key_press`, to execute a key press event.
* `key_release`, to execute a key release event.
* `sound`, to know when to play the beep sound. It should be called after each frame tick.
* `default`, returns a new instance without the preloaded sprites. This can't work with the common ROMs but it's useful for testing.
