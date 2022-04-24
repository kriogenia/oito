# Initialization

The next step is having a ready to be executed emulator.
That means, specifying the initial state of the emulator to easily start it.
This is a perfect use case of the `Default` trait, so I applied it to all the modules.

Based on the specification those initial values are pretty easy to set.
All the starting addresses like those on the RAM or the IRegister should be point to 0x0 (NULL).
All the starting bit representations like those on the VRegisters should be also 0.
There's one exception, the Chip8 emulator starts reading the program at the position 512, so the PC should be initialized to 0x200 (512).

To improve the readability of all the components I decided to specify two extra custom types to show what each component works with using words instead of the abstractions. 
So, that's how `Address` (`u16`) and `Byte` (`u8`) were born.
This way we can easily see now how IRegister or the PC are used to work with addresses but the VRegister or the RAM hold bytes of information.
I also merged the two register types into a single generic one creating custom types to differiate them.