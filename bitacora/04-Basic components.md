# Basic components
I want to build the little components first before jumping into the integration of the parts.
That means, build each component with their functional basic interface that I think that I will need.
So, the idea is to build the stack or the RAM and their unit tests before jumping into the emmulation.

This are the functions that I want working before moving to the next phase:
* `CPU`
  * `increase`, to point the PC to the next opcode.
  * `point_at`, to point the PC to the specified address.
* `RAM`
  * `read`, to return the value stored in the given address.
* `Register`
  * `load` to modify the byte stored by the register.
  * `eq from PartialEq` to ease the comparison between registers. 
* `Stack`
  * `push`, to add a new value at top of the stack.
  * `pop`, to return to the top most value of the stack.
* `Timer`
  * `decrease`, to decrease the count of the timer.
* `VRAM`
  * `clear`, to clear the content of the current frame.