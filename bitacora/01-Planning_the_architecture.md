# Planning the architecture

I wanted my first step to be the definition of the main structure of the virtual machine.
I also wanted to separate everything into their own thing to represent all the *hardware parts* as their own entity.
I didn't want an emulator made of a series of integer named fields so I just broke everything into tiny parts.
I'm expecting to refactor some of those things as I progress but I'm fine with it.

During this design phase I took some liberties interpreting the Chip8 documentation.
The documentation talks about the Chip8 VM made of memory, registers, keyboard, display and timers.
I didn't like those modules so I modeled everything after more real architectures.

## My architecture

So, what did I did? First, I want a CPU entity only having what real CPUs have, that's the registers and the program counter. 
I also wanted the memory to be like RAM outside of the CPU and the same for the stack.
The drawing information will be stored into a VRAM module that I'll be reading from the different frontends when the time to draw the frames come.

So, overall, we have the `OitoCore` being the whole backend of the emulator and being composed at first of:
* `CPU`, will be in charge of managing the registers of the virtual machine.
  * `PC`, the program counter, holds the address pointing to next opcode.
  * `V Registers`, the sixteen virtual registers of Chip8.
  * `I Register`, register to RAM indexing.
* `Stack`, manually made FIFO data structure to keep everything in house and as simple as possible.
* `RAM` or memory, will be the representation of the virtual machine memory.
* `VRAM` or display, this will manage the frame to draw in the different front-end options.
* `Delay Timer`, frame counter used in some instructions.
* `Sound Timer`, another counter to control the sound of the system.

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
[OitoCore] --> [Stack]
[OitoCore] --> [DelayTimer]
[OitoCore] --> [SoundTimer]
[CPU] --> [PC]
[CPU] --> [16x VRegister]
[CPU] --> [IRegister]
@enduml
```

![Architecture diagram](https://www.planttext.com/api/plantuml/svg/SoWkIImgAStDuOhspop9TyulIerLqDMrKuXs3WYDnH0CSlJ550p3AXfSafYSZIdiafgJM1cIcPjQX4LBVcbU2amEP3z4EC0PXMjeLg4G5fIQdbbSWgRG4o7ga9gN0lGL0000)

## Types for abstraction
To improve the readability of all the components I decided to specify some extra custom types to show what each component works with using words instead of the abstractions. 
So, that's how `Address` (`u16`), `OpCode` (`u16`) and `Byte` (`u8`) were born.
This way we can easily see now how IRegister or the PC are used to work with addresses while VRegisters and the RAM hold bytes of information.

## Registers and Timers
To reduce the repetition of code I examined the entities that are similar but have different purposes to check if they could just be the same component. They indeed can. 

The difference between the VRegister and IRegister is only the size of the information they hold. The VRegister holds only a single byte of information while the IRegister holds a word to point to an Address. So, they are more like `Register<Byte>` and `Register<Address>` over a generic register. I still made custom types to represent those concrete types of generic to evade being verbose and keep the semantics.

Timers on the other hand are the same structure from start to end, at least with my current approach without a sound system. I was thinking about making the timers generic over any `integer` to be reusable but I'll keep them the most basic as possible for now, so holding a `u8` (in this case I didn't use an alias as the count is indeed an integer value, there's no meaning behind it).