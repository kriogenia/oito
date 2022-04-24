# CPU tick
This is the key point of my emulation. Simmulating the execution of the CPU.
The Chip8 CPU works in a pretty straightforward way:
* **Fetch**` the next opcode from the RAM.
* **Increase** the program counter to point to next opcode.
* **Decode** the operation to perform.
* **Execute** the instruction.

This should happen in each tick and that tick should be the interface of the CPU.
This way we can run the CPU just looping and calling its ticks.

## Memory access
There's an interesting point in consideration about this.
The way I modeled the emulator I took the memory out of the CPU.
I decided that I wanted to simulate a RAM instead of a cache so the memory is out of the CPU.
How do we access it to perform the memory reading?
I considered three options.
* Creating a buffer to simulate real architectures communication between CPU and RAM.
* Giving a reference to the RAM to the CPU.
* Providing the access through a closure.

So, the first option I decided to dismiss was the first one as that involved inventing a component outside of the Chip8 specification. I don't want that, as it would be a) a bad replication of a buffer or b) a a deviation from the work of the Chip8 system.

Finally, between the last two options I decided to pick the functional approach.
The main reason is that this way it looks like the emulation is providing access to the memory and that memory could be anything. I could create a different approach trying to replicate some other component and it would work the same.
In fact, this way I was able to fetch the `fetch` operation without the need to involve the RAM module and keeping the operation in a real unit scenario using only the CPU and a mocked closure.