# Instructions
The instructions are the big logic of the emulator.
Each program code consist of a series of two bytes aka operation codes (OpCodes).
Those OpCodes represent the different instructions with the parameters for each one.
There are 35 of those.

Again, one of my main goals in this project is for it to be easily comprehensible to anyone that could look at it without already knowing about Chip8. So, I wanted to modelate the instructions, two possible approaches can be made here.

## Enumerate

So, the idea for the instructions is to build the `Instruction` from the `OpCode` using the `TryFrom` trait.
If the code doesn't match any instruction then an exception should be thrown.

The instructions would then be executed in the `OitoCore` matching against the built enum value.
That will generate an extremely populated function with all those instructions but this would leave all the logic in just two files and work directly on the stack without heap allocation.

## Trait

The other possibility is to follow the Command pattern and create a instruction trait making new functions or structs for each instruction.
This will generate a big number of new entities and all those would have to leave in the heap as their size would be unknown (or we could try to implement the `Sized` trait based on the size of the bigger parameter).

A more complex parsing option will be needed and we will have a big number of new files, but all the logic would be scattered instead of filling a big function. 

## Benchmarking

I think that this could be a nice opportunity to perform some benchmarking to meassure the optimal option. I think that it will be the enumerate as it doesn't need any heap allocation, but let's see.

## Features

If I build everything to check what is fastest it would a bad idea to just delete the slowest code and lose that example.
I'll probably define the different implementations as features so we could run any of them whenever we want.