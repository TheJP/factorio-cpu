; 01 (addr 0x01 - 0x08)
MOV A, 0x25
MOV B, 0x26
MOV C, 0x27
MOV D, 0x28

; 06 (addr 0x09 - 0x10)
MOV [A], 0x23
MOV [B], 0x24
MOV [C], 0x25
MOV [D], 0x26

; 07 (addr 0x11 - 0x12)
MOV [0x29], A

; 02 (addr 0x13)
MOV A, IP

; 07 (addr 0x14 - 0x15)
MOV [0x2A], A

; 04 (addr 0x16)
MOV A, [D]

; 07 (addr 0x17 - 0x18)
MOV [0x2B], A

; 03 (addr 0x19 - 0x1A)
MOV A, [0x29]

; 05 (addr 0x1B - 0x1D)
MOV [0x2C], 0x2A

; 01 (addr 0x1E - 0x1F)
MOV D, 0x2D

; 08 (addr 0x20)
MOV [D], A

; EE
HALT

; Output starting at address 0x25:
; [0x25] 0x23 (Dec 35)
; [0x26] 0x24 (Dec 36)
; [0x27] 0x25 (Dec 37)
; [0x28] 0x26 (Dec 38)
; [0x29] 0x25 (Dec 37)
; [0x2A] 0x13 (Dec 19)
; [0x2B] 0x26 (Dec 38)
; [0x2C] 0x2A (Dec 42)
; [0x2D] 0x25 (Dec 37)
