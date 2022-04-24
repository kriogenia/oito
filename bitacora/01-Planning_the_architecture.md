# Planning the architecture

I wanted my first step to be the definition of the main structure of the virtual machine.
I also wanted to separate everything into their own thing to represent all the *hardware parts* as their own entity.
I didn't want an emulator made of a series of integer named fields so I just broke everything into tiny parts.
I'm expecting to refactor some of those things as I progress but I'm fine with it.

During this design phase I took some liberties interpreting the Chip8 documentation.
The documentation talks about the Chip8 VM made of memory, registers, keyboard, display and timers.
I didn't like those modules so I modeled everything after more real architectures.

## My architecture

So, what did I did? First, I want a CPU entity only having what real CPUs have, 
I want the memory to be RAM outside of the CPU and the same for the VRAM.

So, overall, we have the `OitoCore` being the whole backend of the emulator and being composed at first of:
* `CPU`, will be in charge of all the logic and processing of the virtual machine.
  * `PC`, the program counter.
  * `V Registers`, the sixteen virtual registers of Chip8.
  * `Stack`, manually made FIFO data structure to keep everything in house and as simple as possible.
  * `I Register`, register to RAM indexing.
  * `Delay Timer`, the cycle counter.
  * `Sound Timer`, another timer to control the sound of the system, this could be moved out of the CPU.
* `RAM` or memory, will be the representation of the virtual machine memory.
* `VRAM` or display, this will manage the frame to draw in the different front-end options.

As you can see I left out the **keyboard**. You can easily guess the reason, it's a periferic.
So, I will take it out of the machine. The `OitoCore` will provide an interface to pass the inputs, but the
keyboard will be simulated in the front-end crates, the same way I will perform there the drawing operations
and the simulation of the **display**, in the core I only leave the VRAM that will be drawn into the display.


This is a representation of this architecture:

```
@startuml
[OitoCore] --> [CPU]
[OitoCore] --> [RAM]
[OitoCore] --> [VRAM]
[CPU] --> [PC]
[CPU] --> [16x VRegister]
[CPU] --> [IRegister]
[CPU] --> [Stack]
[CPU] --> [DelayTimer]
[CPU] --> [SoundTimer]
@enduml
```

![Architecture diagram](https://www.planttext.com/api/plantuml/svg/SoWkIImgAStDuOhspop9TyulIerLqDMrKuXs3WYDnH0CSlJ550o3Y88qG0G2d56uXcOL2c51gUcPnIMfHIWodbX5WqiIaxDHH5nISn8hGp9pqHNcb-QbG8KTKlDIWDu00000)