# Oito
Oito is a personal project that I wanted to make in my free time.
I always had interest on emulation so I decided to make my own **Chip8 emulator**, 
the first step to anyone wanting to join the emulation world.

The idea behind Oito was to build the core of the emulator using Rust and a very component oriented approach. This means that the goal of this emulator is not searching the best performance as it is in a lot of cases, but to model the emulator in a way that it's easy to understand how the "physical" components would work and communicate. This doesn't mean that I didn't put some though into performance, I'm using Rust after all, but it's not the main objective.

Over that core there are two front-ends: one for desktop using SDL2 and the other for web using HTML5 and compiling Rust to WASM. So, you can run it as you want. The web version is also published with GitHub Pages using Actions. 

**Hope you like it**.

## Architecture
Oito is not modeled after the Chip8 virtual machine description.
Of course, it follows it to perform the emulation, but the entities of it are not what was implemented.
Oito architecture is based on real-life world architectures and components and it's like the following:

![Oito architecture](https://www.planttext.com/api/plantuml/svg/SoWkIImgAStDuOhspop9TyulIerLqDMrKuXs3WYDnH0CSlJ550p3AXfSafYSZIdiafgJM1cIcPjQX4LBVcbU2amEP3z4EC0PXMjeLg4G5fIQdbbSWgRG4o7ga9gN0lGL0000)

## API
OitoCore offers the following interface to use it and it's what's implemented in both front-ends:
* `new`, returns a new instance of the core with the preloaded sprites ready to be used.
* `load`, loads the bytes of the ROM to execute.
* `tick`, simulates a CPU tick. The first call to this function is the start of the execution of the loaded ROM. This should be used 10 times per frame rendering for max efficiency.
* `frame_tick`, simulates a frame tick. This should be called with each frame render.
* `frame_buffer`, returns the buffer representing the next frame to draw. It's made of booleans indicating if the pixel should be drawn or not (Chip8 was black and white).
* `key_press`, to execute a key press event.
* `key_release`, to execute a key release event.
* `default`, returns a new instance without the preloaded sprites. This can't work with the common ROMs but it's useful for testing.

Any front-end should be able to make `OitoCore` run with this functions.

## Usage
Refer to the instructions of each front-end:
* [SDL2](./sdl2/README.md)
* [Web](./wasm/README.md)

## Next

I'm not finished with Oito. First, because I'm still lacking the sounds and that's not good. And second, because I want to build some insights out of this experience. I always wanted to make a MdBook and I think that some kind of tutorial to make this could be a good option. I would also like to make some post over DEV or Medium, but I don't know yet.