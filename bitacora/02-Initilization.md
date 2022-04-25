# Initialization

The next step is having a ready to be executed emulator.
That means, specifying the initial state of the emulator to easily start it.
This is a perfect use case of the `Default` trait, so I applied it to all the modules.

Based on the specification those initial values are pretty easy to set.
All the starting addresses like those on the RAM or the IRegister should be point to 0x0 (NULL).
All the starting bit representations like those on the VRegisters should be also 0.
There's one exception, the Chip8 emulator starts reading the program at the position 512, so the PC should be initialized to 0x200 (512).