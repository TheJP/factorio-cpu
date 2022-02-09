# CPU Architecture

All values are 32 bit signed integers.

Smallest addressable: 32 bit = 4 byte

Big-endian byte ordering

## Registers

All registers are 32 bit wide.

* Multi Purpose: A, B, C, D

* Instruction Pointer: IP

* Stack Pointer: SP

### Register Encoding

Enncoding of the registers as it is used in the [Instructions](#instructions) section below.

| reg | Encoding |
|-----|----------|
| A   | 01       |
| B   | 02       |
| C   | 03       |
| D   | 04       |
| IP  | 05       |
| SP  | 06       |

## Flags

* Z - Is set, if the result of the last arithmetic or logic operation was 0.
* S - Is set, if the result of the last arithmetic or logic operation was signed.

See [Jump Instructions](#jump) to see how the flags can be used to make conditional jumps.

## In / Out

Input and output from/to external components is done by reads and writes to specific memory addresses. We choose addresses that are outside of the actual memory and connect the components there. Memory size is not fixed or capped and depends on the micro architecture.

## Instructions

### Parameters

* reg   - Any Register
* imm   - Immediate (Static Constant Value)
* label - Jump Label (Target for Jumps)
* [reg] - Memory Address Pointed to by Register
* [imm] - Memory Address Pointed to by Immediate

### Instruction Encoding

The type of every instruction is encoded into the right most byte of the first value. The remaining 3 bytes on the left side of the type may be used for parameters. The first value is padded with 0s (bytes with value 00) on the left, if not all 3 bytes are used.

An instruction may use additional values for more parameters. The additional values do not have to specify the instruction type. The instruction type conclusively determines the amount of additional values used.

All numbers for instruction types or parameters in the encoding table are given in hexadecimal notation in this document. In the instruction tables there is additionally a helper column named "Dec" listing the instruction type in decimal notation.

Reminder: Big-endian byte ordering is used in the architecture.

Example Encoding (see tables for [Register Encoding](#register-encoding) and [MOV Instructions](#mov) for details):

```assembly
MOV D, 42
```

```text
00 00 04 01 00 00 00 2A
            ^^ ^^ ^^ ^^ Immediate 42
         ^^ Instruction: MOV (with argument types: reg, imm)
      ^^ Register D
^^ ^^ Padding 0s
```

The encoded instruction above has instruction type 01. Looking at the instruction encoding tables we see:

* Instruction tpye 01 is the instruction `MOV reg, imm`.
* The instruction (of type 01) is always encoded using 2 values.
* The byte to the left of the type is the target register and decodes to register D.
* The 2nd value is the source value immediate and decodes to 42.

### MOV

```assembly
MOV A, 42
MOV B, C
MOV A, [5]
MOV A, [B]
MOV [5], 42
MOV [A], -5
MOV [5], B
MOV [A], B
```

| Instruction        | Encoding                 ||| Dec | Explanation
|:-------------------|-------------:|:-----|:-----|----:|:------------
| `MOV reg, imm`     |       reg 01 | imm  |      |   1 | Copy imm to reg.
| `MOV reg₁, reg₂`   | reg₂ reg₁ 02 |      |      |   2 | Copy reg₂ to reg₁.
| `MOV reg, [imm]`   |       reg 03 | imm  |      |   3 | Copy memory at imm to reg.
| `MOV reg₁, [reg₂]` | reg₂ reg₁ 04 |      |      |   4 | Copy memory at reg₂ to reg₁.
| `MOV [imm₁], imm₂` |           05 | imm₁ | imm₂ |   5 | Copy imm₂ to memory at imm₁.
| `MOV [reg], imm`   |       reg 06 | imm  |      |   6 | Copy imm to memory at reg.
| `MOV [imm], reg`   |       reg 07 | imm  |      |   7 | Copy reg to memory at imm.
| `MOV [reg₁], reg₂` | reg₂ reg₁ 08 |      |      |   8 | Copy reg₂ to memory at reg₁.

### Arithmetic

```assembly
ADD A, 42
ADD A, B
SUB A, 5
SUB B, C
MUL A, 42
MUL A, B
DIV A, 42
DIV A, B
MOD A, 42
MOD A, B
POW A, 42
POW A, B
INC A
DEC B
```

| Instruction      | Encoding          || Dec | Explanation
|:-----------------|-------------:|:----|----:|:------------
| `ADD reg, imm`   |      reg  10 | imm |  16 | reg  += imm
| `ADD reg₁, reg₂` | reg₂ reg₁ 20 |     |  32 | reg₁ += reg₂
| `SUB reg, imm`   |      reg  11 | imm |  17 | reg  -= imm
| `SUB reg₁, reg₂` | reg₂ reg₁ 21 |     |  33 | reg₁ -= reg₂
| `MUL reg, imm`   |      reg  12 | imm |  18 | reg  *= imm
| `MUL reg₁, reg₂` | reg₂ reg₁ 22 |     |  34 | reg₁ *= reg₂
| `DIV reg, imm`   |      reg  13 | imm |  19 | reg  /= imm
| `DIV reg₁, reg₂` | reg₂ reg₁ 23 |     |  35 | reg₁ /= reg₂
| `MOD reg, imm`   |      reg  14 | imm |  20 | reg  %= imm
| `MOD reg₁, reg₂` | reg₂ reg₁ 24 |     |  36 | reg₁ %= reg₂
| `POW reg, imm`   |      reg  15 | imm |  21 | reg = pow(reg, imm)
| `POW reg₁, reg₂` | reg₂ reg₁ 25 |     |  37 | reg₁ = pow(reg₁, reg₂)
| `INC reg`        |      reg  17 |     |  23 | reg++
| `DEC reg`        |      reg  18 |     |  24 | reg--

Each arithmetic instruction sets the Z and S flags after the computation.

### Bit Operations

```assembly
AND A, 42
AND A, B
OR  A, 42
OR  A, B
XOR A, 42
XOR A, B
SHL A, 1
SHL A, B
SHR A, 2
SHR A, B
NOT A
```

| Instruction      | Encoding          || Dec | Explanation
|:-----------------|-------------:|:----|----:|:------------
| `AND reg, imm`   |      reg  1A | imm |  26 | reg  &= imm  (bitwise and)
| `AND reg₁, reg₂` | reg₂ reg₁ 2A |     |  42 | reg₁ &= reg₂ (bitwise and)
| `OR  reg, imm`   |      reg  1B | imm |  27 | reg  ¦= imm  (bitwise or)
| `OR  reg₁, reg₂` | reg₂ reg₁ 2B |     |  43 | reg₁ ¦= reg₂ (bitwise or)
| `XOR reg, imm`   |      reg  1C | imm |  28 | reg  ^= imm  (bitwise xor)
| `XOR reg₁, reg₂` | reg₂ reg₁ 2C |     |  44 | reg₁ ^= reg₂ (bitwise xor)
| `SHL reg, imm`   |      reg  1D | imm |  29 | reg  <<= imm
| `SHL reg₁, reg₂` | reg₂ reg₁ 2D |     |  45 | reg₁ <<= reg₂
| `SHR reg, imm`   |      reg  1E | imm |  30 | reg  >>= imm
| `SHR reg₁, reg₂` | reg₂ reg₁ 2E |     |  46 | reg₁ >>= reg₂
| `NOT reg`        |      reg  1F |     |  31 | reg = ~reg (bitwise not)

Each bit operation instruction sets the Z and S flags after the computation.

### CMP

```assembly
CMP A, 5
CMP B, C
```

| Instruction      | Encoding           || Dec | Explanation
|:-----------------|-------------:|:-----|----:|:------------
| `CMP reg, imm`   |      reg  16 | imm  |  22 | reg  - imm  (only set flags)
| `CMP reg₁, reg₂` | reg₂ reg₁ 26 |      |  38 | reg₁ - reg₂ (only set flags)

The compare instruction sets the Z and S flags after the computation.

### Jump

```assembly
JMP labelx
JZ  labely
JNZ labelz
JS  labely
JNS labelz
```

Jumps jump a relative distance, where 0 is the current location. A jump with 0 as argument is an endless loop.

| Instruction | Encoding    | Dec | Explanation
|:------------|------------:|----:|:------------
| `JMP label` | location 50 |  80 | Jump to label. (unconditional)
| `JZ  label` | location 51 |  81 | Jump to label if Z flag is set.
| `JNZ label` | location 52 |  82 | Jump to label if Z flag is not set.
| `JS  label` | location 53 |  83 | Jump to label if S flag is set.
| `JNS label` | location 54 |  84 | Jump to label if S flag is not set.
| `JE  label` | location 51 |  81 | Alias for JZ. (Used with CMP for "jump if equal".)
| `JNE label` | location 52 |  82 | Alias for JNZ. (Used with CMP for "jump if not equal".)
| `JLT label` | location 53 |  83 | Alias for JS. (Used with CMP for "jump if less than".)
| `JGE label` | location 54 |  84 | Alias for JNS. (Used with CMP for "jump if equal or greater than".)
| `JLE label` | location 55 |  85 | Jump to label if S or Z flag is set. (Used with CMP for "jump if equal or less than".)
| `JGT label` | location 56 |  86 | Jump to label if both S and Z flags are not set. (Used with CMP for "jump if greater than".)

`location` in the table above is an i24 (24 bit signed integer). If the jump condition is true, `location` is added to IP (instruction pointer register).

Note: The smallest addressable space is 32 bits. This is other than most machines, which can address down to the byte space (smallest addressable space equal to 8 bit). When computing correct locations for jump instructions, this has to be considered too: an increase of 1 in location equals the skip of 32 bit in memory.

### Stack

```assembly
PUSH 42
PUSH A
POP  A
```

| Instruction      | Encoding      || Dec | Explanation
|:-----------------|---------:|:----|----:|:------------
| `PUSH imm`       |       60 | imm |  96 | Push imm onto the stack: `[SP] = imm; SP--`
| `PUSH reg`       |   reg 61 |     |  97 | Push reg onto the stack: `[SP] = reg; SP--`
| `POP  reg`       |   reg 62 |     |  98 | Pop from the stack into reg: `SP++; reg = [SP]`

Note: The stack grows upwards (negative address). So decreasing the stack pointer (SP--) grows the stack, while increasing the stack pointer (SP++) shrinks the stack.

### Call

```assembly
CALL labelx
INT  A
labelx:
  RET
```

| Instruction  | Encoding    | Dec | Explanation
|:-------------|------------:|----:|:------------
| `CALL label` | location 70 | 112 | Save IP on the stack and jump to label: `PUSH IP; IP += location`
| `INT  reg`   |      reg 72 | 114 | Save IP on the stack and jump to reg: `PUSH IP; IP += reg`
| `RET`        |          71 | 113 | Reutrn from call by popping IP from the stack: `POP IP`

### Miscellaneous

```assembly
HALT
NOP
```

| Instruction | Encoding | Dec | Explanation
|:------------|---------:|----:|:------------
| `HALT`      |       EE | 238 | Halts machine execution.
| `NOP`       |       FF | 255 | Does nothing.

### TBD

* Interrupts
* Flag Register
