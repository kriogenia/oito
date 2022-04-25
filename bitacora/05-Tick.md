# CPU tick
This is the key point of my emulation. Simmulating the execution of the emmulator.
The Chip8 VM works in a pretty straightforward way:
* **Fetch**` the next opcode from the RAM.
* **Decode** the operation to perform.
* **Execute** the instruction.
* **Advance** the program counter to point to the next opcode.

This should happen in each tick and that tick should be the interface of the emmulator.
This way we can run it just looping and calling its ticks.