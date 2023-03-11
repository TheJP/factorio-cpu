# Micro Architecture

## Used Signals

### Internal

* **C:** Clock counter for each instruction. Can be used to pause/halt execution.
* **Pink:** General purpose internal signal. Should never be present on any memory or register wires.

### Memory

* **Green:** Memory

### Registers

* **Yellow:** Internal Command Signal for Instructions
* **Red:** Reset Registers
* **0, 1, 2, 3, 4:** General Purpose Registers
* **N:** Instruction Pointer (IP)
* **I:** Current Instruction (`[IP] & 0xFF`)
* **V:** Current Value (`[IP] >> 8`) (sign retaining shift)
* **P:** Stack Pointer (SP)
* **Z:** Zero Flag
* **S:** Signed Flag
* **A, B:** Decoded Register Arguments
* **X:** Write to Register Argument
* **Cyan:** Write to Register Argument
