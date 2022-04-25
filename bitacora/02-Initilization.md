# Initialization

The next step is having a ready to be executed emulator.
That means, specifying the initial state of the emulator to easily start it.
This is a perfect use case of the `Default` trait, so I applied it to all the modules.

Based on the specification those initial values are pretty easy to set.
All the starting addresses like those on the RAM or the IRegister should be point to 0x0 (NULL).
All the starting bit representations like those on the VRegisters should be also 0.
There's one exception, the Chip8 emulator starts reading the program at the position 512, so the PC should be initialized to 0x200 (512).

+---------------+= 0xFFF (4095) End of Chip-8 RAM
|               |
|               |
|               |
|               |
|               |
| 0x200 to 0xFFF|
|     Chip-8    |
| Program / Data|
|     Space     |
|               |
|               |
|               |
+- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
|               |
|               |
|               |
+---------------+= 0x200 (512) Start of most Chip-8 programs <----
| 0x000 to 0x1FF|
| Reserved for  |
|  interpreter  |
+---------------+= 0x000 (0) Start of Chip-8 RAM